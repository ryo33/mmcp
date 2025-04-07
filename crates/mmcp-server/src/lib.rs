pub mod inventory;
pub mod primitives;

use std::{borrow::Cow, collections::HashMap};

use crate::{
    inventory::ToolRegistration,
    primitives::tool::{BoxedTool, Tool},
};
use mmcp_protocol::{
    mcp::{
        Implementation, InitializeRequest, InitializeResult, InitializedNotification,
        JSONRPCMessage, ServerCapabilities, ServerCapabilitiesTools,
    },
    port::{RPCMessageHandler, RPCPort},
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

    pub fn handler(&self) -> impl RPCMessageHandler {
        self
    }

    pub async fn start(&self, port: impl RPCPort) -> anyhow::Result<()> {
        let (id, _request) = port
            .wait_for_request::<InitializeRequest>("initialize")
            .await?;
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
            protocol_version: "2025-03-26".to_string(),
            server_info: Implementation {
                name: self.name.clone(),
                version: self.version.clone(),
                extra: Default::default(),
            },
            extra: Default::default(),
        };
        port.send_response(id, response).await?;
        port.wait_for_notification::<InitializedNotification>("notifications/initialized")
            .await?;
        Ok(())
    }
}

impl RPCMessageHandler for &MCPServer {
    async fn handle_message(&self, message: JSONRPCMessage) -> anyhow::Result<()> {
        // match message {
        Ok(())
    }
}
