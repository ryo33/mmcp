pub mod inventory;
pub mod primitives;

use std::{borrow::Cow, collections::BTreeMap};

use crate::{
    inventory::ToolRegistration,
    primitives::tool::{BoxedTool, Tool},
};
use anyhow::{Context as _, anyhow};
use mmcp_protocol::{
    ProtocolVersion,
    consts::error_codes,
    mcp::{
        self, CallToolRequest, CallToolResult, ClientRequest, CompleteRequest, GetPromptRequest,
        Implementation, InitializeRequest, InitializeResult, JSONRPCBatchRequest, JSONRPCError,
        JSONRPCMessage, JSONRPCRequest, JSONRPCResponse, JsonrpcBatchResponseItem,
        JsonrpcErrorError, ListPromptsRequest, ListResourcesRequest, ListToolsRequest, PingRequest,
        ReadResourceRequest, RequestId, ServerCapabilities, ServerCapabilitiesPrompts,
        ServerCapabilitiesResources, ServerCapabilitiesTools, SetLevelRequest, SubscribeRequest,
        UnsubscribeRequest,
    },
    port::{RPCPort, RPCSink},
};

pub struct MCPServer {
    name: String,
    version: String,
    tools: BTreeMap<Cow<'static, str>, BoxedTool>,
    instructions: Option<String>,
}

impl MCPServer {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            tools: Default::default(),
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

    /// List the resources available on this server.
    pub fn list_resources(&self) -> impl Iterator<Item = mcp::Resource> {
        // Currently, we don't have any resources, so return an empty iterator
        std::iter::empty()
    }

    /// List the prompts available on this server.
    pub fn list_prompts(&self) -> impl Iterator<Item = mcp::Prompt> {
        // Currently, we don't have any prompts, so return an empty iterator
        std::iter::empty()
    }

    /// Set the instructions for the server which will be sent to the client on initialize.
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Start the server by initializing and then processing requests
    pub async fn start<P: RPCPort>(self, mut port: P) -> anyhow::Result<()> {
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
        let (init_request_id, init_request) = loop {
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
        self.send_initialize_response(sink, init_request_id, &init_request)
            .await?;

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
        init_request: &InitializeRequest,
    ) -> anyhow::Result<()> {
        let protocol_version = init_request
            .params
            .protocol_version
            .parse::<ProtocolVersion>()
            .context("failed to parse protocol version")?;
        let response = InitializeResult {
            meta: None,
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools {
                    list_changed: Some(true),
                    extra: Default::default(),
                }),
                resources: Some(ServerCapabilitiesResources {
                    list_changed: Some(true),
                    subscribe: Some(false),
                    extra: Default::default(),
                }),
                prompts: Some(ServerCapabilitiesPrompts {
                    list_changed: Some(true),
                    extra: Default::default(),
                }),
                ..Default::default()
            },
            instructions: self.instructions.clone(),
            protocol_version: protocol_version.to_string(),
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
                if let Some(response) = self.handle_request(request).await? {
                    let message = match response {
                        JsonrpcBatchResponseItem::JSONRPCResponse(response) => {
                            JSONRPCMessage::JSONRPCResponse(response)
                        }
                        JsonrpcBatchResponseItem::JSONRPCError(error) => {
                            JSONRPCMessage::JSONRPCError(error)
                        }
                    };
                    sink.send_message(message).await?;
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
    ) -> anyhow::Result<Option<JsonrpcBatchResponseItem>> {
        let request_id = request.id;
        let client_request = serde_json::from_value::<ClientRequest>(serde_json::json!({
            "method": request.method,
            "params": request.params,
        }))?;

        match client_request {
            ClientRequest::InitializeRequest(_) => Ok(None),
            ClientRequest::PingRequest(ping_request) => self
                .handle_ping_request(request_id, ping_request)
                .await
                .map(Some),
            ClientRequest::ListResourcesRequest(list_resources_request) => self
                .handle_list_resources_request(request_id, list_resources_request)
                .await
                .map(Some),
            ClientRequest::ReadResourceRequest(read_resource_request) => self
                .handle_read_resource_request(request_id, read_resource_request)
                .await
                .map(Some),
            ClientRequest::SubscribeRequest(subscribe_request) => self
                .handle_subscribe_request(request_id, subscribe_request)
                .await
                .map(Some),
            ClientRequest::UnsubscribeRequest(unsubscribe_request) => self
                .handle_unsubscribe_request(request_id, unsubscribe_request)
                .await
                .map(Some),
            ClientRequest::ListPromptsRequest(list_prompts_request) => self
                .handle_list_prompts_request(request_id, list_prompts_request)
                .await
                .map(Some),
            ClientRequest::GetPromptRequest(get_prompt_request) => self
                .handle_get_prompt_request(request_id, get_prompt_request)
                .await
                .map(Some),
            ClientRequest::ListToolsRequest(list_tools_request) => self
                .handle_list_tools_request(request_id, list_tools_request)
                .await
                .map(Some),
            ClientRequest::CallToolRequest(call_tool_request) => self
                .handle_call_tool_request(request_id, call_tool_request)
                .await
                .map(Some),
            ClientRequest::SetLevelRequest(set_level_request) => self
                .handle_set_level_request(request_id, set_level_request)
                .await
                .map(Some),
            ClientRequest::CompleteRequest(complete_request) => self
                .handle_complete_request(request_id, complete_request)
                .await
                .map(Some),
        }
    }

    async fn handle_ping_request(
        &self,
        request_id: RequestId,
        _request: PingRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let extra = serde_json::json!({
            "message": "pong"
        })
        .as_object()
        .unwrap()
        .clone();

        Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
            id: request_id,
            jsonrpc: Default::default(),
            result: mcp::Result {
                meta: Default::default(),
                extra,
            },
            extra: Default::default(),
        }))
    }

    async fn handle_list_resources_request(
        &self,
        request_id: RequestId,
        _request: ListResourcesRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let resources = self
            .list_resources()
            .map(|resource| Ok(serde_json::to_value(resource)?))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
            id: request_id,
            jsonrpc: Default::default(),
            result: mcp::Result {
                meta: Default::default(),
                extra: serde_json::json!({"resources": resources})
                    .as_object()
                    .unwrap()
                    .clone(),
            },
            extra: Default::default(),
        }))
    }

    async fn handle_read_resource_request(
        &self,
        _request_id: RequestId,
        _request: ReadResourceRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_subscribe_request(
        &self,
        _request_id: RequestId,
        _request: SubscribeRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_unsubscribe_request(
        &self,
        _request_id: RequestId,
        _request: UnsubscribeRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_list_prompts_request(
        &self,
        request_id: RequestId,
        _request: ListPromptsRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let prompts = self
            .list_prompts()
            .map(|prompt| Ok(serde_json::to_value(prompt)?))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
            id: request_id,
            jsonrpc: Default::default(),
            result: mcp::Result {
                meta: Default::default(),
                extra: serde_json::json!({"prompts": prompts})
                    .as_object()
                    .unwrap()
                    .clone(),
            },
            extra: Default::default(),
        }))
    }

    async fn handle_get_prompt_request(
        &self,
        _request_id: RequestId,
        _request: GetPromptRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_list_tools_request(
        &self,
        request_id: RequestId,
        _request: ListToolsRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
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
            id: request_id,
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

    async fn handle_call_tool_request(
        &self,
        request_id: RequestId,
        request: CallToolRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let tool_name = request.params.name.as_str();

        if let Some(tool) = self.get_tool(tool_name) {
            let result = tool.execute(request).await;
            Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
                id: request_id,
                jsonrpc: Default::default(),
                result: serialize_tool_call_result(result)?,
                extra: Default::default(),
            }))
        } else {
            Ok(JsonrpcBatchResponseItem::JSONRPCError(JSONRPCError {
                error: JsonrpcErrorError {
                    message: format!("Tool not found: {}", tool_name),
                    code: error_codes::INVALID_PARAMS,
                    data: None,
                    extra: Default::default(),
                },
                id: request_id,
                jsonrpc: Default::default(),
                extra: Default::default(),
            }))
        }
    }

    async fn handle_set_level_request(
        &self,
        _request_id: RequestId,
        _request: SetLevelRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_complete_request(
        &self,
        _request_id: RequestId,
        _request: CompleteRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
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
