use std::borrow::Cow;

use crate::protocol::mcp::{
    CallToolRequest, CallToolResult, CallToolResultContent, TextContent, ToolAnnotations,
};

pub trait Tool {
    /// The name of the tool
    fn name(&self) -> Cow<'static, str>;

    /// The description of the tool
    fn description(&self) -> Cow<'static, str>;

    /// The parameters of the tool
    fn input_schema(&self) -> Cow<'static, str>;

    /// The annotations of the tool
    fn annotations(&self) -> ToolAnnotations;

    /// Execute the tool
    fn execute(&self, request: &CallToolRequest) -> CallToolResult;
}

pub trait TypedTool {
    type Input;
    type Output;
    type Error;

    /// The name of the tool
    fn name(&self) -> Cow<'static, str>;

    /// The description of the tool
    fn description(&self) -> Cow<'static, str>;

    /// The parameters of the tool
    fn input_schema(&self) -> Cow<'static, str>;

    /// The annotations of the tool
    fn annotations(&self) -> ToolAnnotations;

    /// Execute the tool
    fn execute(&self, arguments: &Self::Input) -> Result<Self::Output, Self::Error>;
}

impl<T, I, O, E> Tool for T
where
    T: TypedTool<Input = I, Output = O, Error = E>,
    I: serde::de::DeserializeOwned,
    O: serde::Serialize,
    E: std::fmt::Display,
{
    fn name(&self) -> Cow<'static, str> {
        self.name()
    }

    fn description(&self) -> Cow<'static, str> {
        self.description()
    }

    fn input_schema(&self) -> Cow<'static, str> {
        self.input_schema()
    }

    fn annotations(&self) -> ToolAnnotations {
        self.annotations()
    }

    fn execute(&self, request: &CallToolRequest) -> CallToolResult {
        let input = match serde_json::from_value(serde_json::Value::Object(
            request.params.arguments.clone().unwrap_or_default(),
        )) {
            Ok(input) => input,
            Err(e) => {
                return CallToolResult {
                    content: vec![CallToolResultContent::TextContent(TextContent {
                        text: format!("Error: parsing input: {}", e),
                        annotations: None,
                        r#type: Default::default(),
                        extra: Default::default(),
                    })],
                    is_error: Some(true),
                    meta: None,
                    extra: Default::default(),
                };
            }
        };
        let output = match self.execute(&input) {
            Ok(output) => output,
            Err(e) => {
                return CallToolResult {
                    content: vec![CallToolResultContent::TextContent(TextContent {
                        text: format!("Error: executing tool: {}", e),
                        annotations: None,
                        r#type: Default::default(),
                        extra: Default::default(),
                    })],
                    is_error: Some(true),
                    meta: None,
                    extra: Default::default(),
                };
            }
        };
        CallToolResult {
            content: vec![CallToolResultContent::TextContent(TextContent {
                text: serde_json::to_string(&output).unwrap(),
                annotations: None,
                r#type: Default::default(),
                extra: Default::default(),
            })],
            is_error: None,
            meta: None,
            extra: Default::default(),
        }
    }
}
