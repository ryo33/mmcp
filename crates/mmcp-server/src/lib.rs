pub mod inventory;
pub mod primitives;
mod runtime;

use anyhow::{Context as _, anyhow};
use mmcp_protocol::{
    ProtocolVersion,
    mcp::{
        self, CallToolResult, Implementation, InitializeRequest, InitializeResult, JSONRPCMessage,
        RequestId, ServerCapabilities, ServerCapabilitiesPrompts, ServerCapabilitiesResources,
        ServerCapabilitiesTools,
    },
    port::{RPCPort, RPCSink},
};
use std::{borrow::Cow, collections::BTreeMap};

use crate::{
    inventory::ToolRegistration,
    primitives::tool::{BoxedTool, Tool},
    runtime::MCPServerRuntime,
};

/// MCP Server implementation with builder pattern for configuration
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

    pub fn add_tool(mut self, tool: impl Tool + Send + Sync + 'static) -> Self {
        self.tools.insert(tool.name(), Box::new(tool));
        self
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

    /// Start the server by transforming into runtime, then run
    pub async fn start<P: RPCPort>(self, port: P) -> anyhow::Result<()> {
        let sink = port.sink();
        let runtime = MCPServerRuntime::new(self, sink.clone());
        runtime.run(port).await
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
        sink.send_response(id, response).await?;
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
