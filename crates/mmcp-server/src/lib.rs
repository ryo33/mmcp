pub mod inventory;
pub mod primitives;

use std::{borrow::Cow, collections::HashMap};

use crate::{
    inventory::ToolRegistration,
    primitives::tool::{BoxedTool, Tool},
};
use anyhow::anyhow;
use mmcp_protocol::{
    consts::{PROTOCOL_VERSION, error_codes},
    mcp::{
        self, CallToolRequest, CallToolResult, CallToolResultContent, Implementation,
        InitializeRequest, InitializeResult, JSONRPCBatchRequest, JSONRPCError, JSONRPCMessage,
        JSONRPCRequest, JSONRPCResponse, JsonrpcBatchResponseItem, JsonrpcErrorError, RequestId,
        ServerCapabilities, ServerCapabilitiesTools, TextContent,
    },
    port::{RPCPort, RPCSink},
};

pub struct MCPServer {
    name: String,
    version: String,
    tools: HashMap<Cow<'static, str>, BoxedTool>,
    instructions: Option<String>,
}

impl MCPServer {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            tools: HashMap::new(),
            instructions: None,
        }
    }

    pub fn with_tools_from_inventory(mut self) -> Self {
        for tool in inventory::iter::<ToolRegistration> {
            let tool = tool.tool();
            self.tools.insert(tool.name(), tool);
        }
        self
    }

    pub fn add_tool(&mut self, tool: impl Tool + Send + Sync + 'static) {
        self.tools.insert(tool.name(), Box::new(tool));
    }

    pub fn get_tool(&self, name: &str) -> Option<&BoxedTool> {
        self.tools.get(name)
    }

    pub fn list_tools(&self) -> impl Iterator<Item = &BoxedTool> {
        self.tools.values()
    }

    /// Set the instructions for the server which will be sent to the client on initialize.
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Start the server by initializing and then processing requests
    pub async fn start<P: RPCPort>(&self, mut port: P) -> anyhow::Result<()> {
        // Create a single sink to use throughout the lifecycle
        let mut sink = port.sink();

        // First handle initialization
        let queued_messages = self.initialize(&mut port, &mut sink).await?;

        // Process any messages queued during initialization
        for message in queued_messages {
            self.handle_message(&mut sink, message).await?;
        }

        // Main message processing loop
        while let Ok(Some(message)) = port.progress().await {
            self.handle_message(&mut sink, message).await?;
        }

        Ok(())
    }

    /// Handle the initialization process
    async fn initialize<P: RPCPort, S: RPCSink>(
        &self,
        port: &mut P,
        sink: &mut S,
    ) -> anyhow::Result<Vec<JSONRPCMessage>> {
        let mut queued_messages = Vec::new();

        // Step 1: Wait for initialize request
        let (init_request_id, _init_request) = loop {
            let message = port
                .progress()
                .await?
                .ok_or_else(|| anyhow!("connection closed during initialization"))?;

            match message {
                JSONRPCMessage::JSONRPCRequest(request) if request.method == "initialize" => {
                    // Parse the initialize request
                    let request_value = serde_json::to_value(&request)
                        .map_err(|e| anyhow!("failed to serialize request: {}", e))?;
                    let initialize_request: InitializeRequest =
                        serde_json::from_value(request_value)
                            .map_err(|e| anyhow!("failed to parse initialize request: {}", e))?;

                    // Return the request ID and request
                    break (request.id.clone(), initialize_request);
                }
                // Queue any other messages to be processed after initialization
                _ => queued_messages.push(message),
            }
        };

        // Step 2: Respond to initialize request
        self.send_initialize_response(sink, init_request_id).await?;

        // Step 3: Wait for initialized notification
        loop {
            let message = port
                .progress()
                .await?
                .ok_or_else(|| anyhow!("connection closed during initialization"))?;

            match message {
                JSONRPCMessage::JSONRPCNotification(notification)
                    if notification.method == "notifications/initialized" =>
                {
                    // Initialized notification received, initialization is complete
                    break;
                }
                // Queue any other messages to be processed after initialization
                _ => queued_messages.push(message),
            }
        }

        Ok(queued_messages)
    }

    /// Send the initialize response with server information and capabilities
    async fn send_initialize_response<S: RPCSink>(
        &self,
        sink: &mut S,
        id: RequestId,
    ) -> anyhow::Result<()> {
        let response = InitializeResult {
            meta: None,
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools {
                    list_changed: Some(true),
                    extra: Default::default(),
                }),
                ..Default::default()
            },
            instructions: self.instructions.clone(),
            protocol_version: PROTOCOL_VERSION.to_string(),
            server_info: Implementation {
                name: self.name.clone(),
                version: self.version.clone(),
                extra: Default::default(),
            },
            extra: Default::default(),
        };

        // Send response
        sink.send_response(id, response).await?;

        Ok(())
    }

    /// Handle a single message from the client
    async fn handle_message<S: RPCSink>(
        &self,
        sink: &mut S,
        message: JSONRPCMessage,
    ) -> anyhow::Result<()> {
        match message {
            JSONRPCMessage::JSONRPCRequest(request) => {
                let response = self.handle_request(request).await?;
                match response {
                    JsonrpcBatchResponseItem::JSONRPCResponse(response) => {
                        sink.send_message(JSONRPCMessage::JSONRPCResponse(response))
                            .await?;
                    }
                    JsonrpcBatchResponseItem::JSONRPCError(error) => {
                        sink.send_message(JSONRPCMessage::JSONRPCError(error))
                            .await?;
                    }
                }
            }
            JSONRPCMessage::JSONRPCNotification(_notification) => {
                // TODO
            }
            JSONRPCMessage::JSONRPCBatchRequest(batch) => {
                self.handle_batch_request(sink, batch).await?;
            }
            _ => {}
        }

        Ok(())
    }

    async fn handle_request(
        &self,
        request: JSONRPCRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        match request.method.as_str() {
            "tools/call" => {
                let Some(params) = request.params else {
                    return Ok(JsonrpcBatchResponseItem::JSONRPCError(JSONRPCError {
                        error: JsonrpcErrorError {
                            message: "No parameters provided".to_string(),
                            code: error_codes::INVALID_PARAMS,
                            data: None,
                            extra: Default::default(),
                        },
                        id: request.id,
                        jsonrpc: Default::default(),
                        extra: Default::default(),
                    }));
                };

                let tool_request: CallToolRequest =
                    match serde_json::from_value(serde_json::Value::Object(params.extra)) {
                        Ok(req) => req,
                        Err(e) => {
                            let result = CallToolResult {
                                extra: Default::default(),
                                meta: Default::default(),
                                content: vec![CallToolResultContent::TextContent(TextContent {
                                    text: format!("Failed to parse tool call request: {}", e),
                                    r#type: Default::default(),
                                    annotations: Default::default(),
                                    extra: Default::default(),
                                })],
                                is_error: Some(true),
                            };
                            return Ok(JsonrpcBatchResponseItem::JSONRPCResponse(
                                JSONRPCResponse {
                                    id: request.id,
                                    jsonrpc: Default::default(),
                                    result: serialize_tool_call_result(result)?,
                                    extra: Default::default(),
                                },
                            ));
                        }
                    };

                // Extract tool name from params
                let tool_name = tool_request.params.name.as_str();

                // Find the tool and execute it
                if let Some(tool) = self.get_tool(tool_name) {
                    // Execute the tool using the tool trait's execute method
                    let result = tool.execute(tool_request).await;
                    Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
                        id: request.id,
                        jsonrpc: Default::default(),
                        result: serialize_tool_call_result(result)?,
                        extra: Default::default(),
                    }))
                } else {
                    // Tool not found, send error response
                    Ok(JsonrpcBatchResponseItem::JSONRPCError(JSONRPCError {
                        error: JsonrpcErrorError {
                            message: format!("Tool not found: {}", tool_name),
                            code: error_codes::INVALID_PARAMS,
                            data: None,
                            extra: Default::default(),
                        },
                        id: request.id,
                        jsonrpc: Default::default(),
                        extra: Default::default(),
                    }))
                }
            }
            "tools/list" => {
                // Handle tool listing request
                let tools = self
                    .list_tools()
                    .map(|tool| {
                        Ok(serde_json::json!({
                            "name": tool.name(),
                            "description": tool.description(),
                            "inputSchema": serde_json::from_str::<serde_json::Value>(tool.input_schema().as_ref())?,
                            "annotations": tool.annotations()
                        }))
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?;

                Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
                    id: request.id,
                    jsonrpc: Default::default(),
                    result: mcp::Result {
                        meta: Default::default(),
                        extra: serde_json::json!({"tools": tools})
                            .as_object()
                            .unwrap()
                            .clone(),
                    },
                    extra: Default::default(),
                }))
            }
            // Handle other request types
            _ => {
                // Return method not found error
                Ok(JsonrpcBatchResponseItem::JSONRPCError(JSONRPCError {
                    error: JsonrpcErrorError {
                        message: format!("Method not supported: {}", request.method),
                        code: error_codes::METHOD_NOT_FOUND,
                        data: None,
                        extra: Default::default(),
                    },
                    id: request.id,
                    jsonrpc: Default::default(),
                    extra: Default::default(),
                }))
            }
        }
    }

    async fn handle_batch_request<S: RPCSink>(
        &self,
        _sink: &mut S,
        _batch: JSONRPCBatchRequest,
    ) -> anyhow::Result<()> {
        // TODO: Implement batch request handling
        Ok(())
    }
}

fn serialize_tool_call_result(result: CallToolResult) -> anyhow::Result<mcp::Result> {
    let serde_json::Value::Object(result) = serde_json::to_value(&result)? else {
        panic!("CallToolResult should be serialized to an object");
    };
    Ok(mcp::Result {
        meta: Default::default(),
        extra: result,
    })
}
