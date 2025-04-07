use std::{borrow::Cow, future::Future, pin::Pin};

use futures::FutureExt as _;
use serde::Serialize;

use mmcp_protocol::mcp::{
    AudioContent, CallToolRequest, CallToolResult, CallToolResultContent, EmbeddedResource,
    ImageContent, TextContent, ToolAnnotations,
};

pub type BoxedTool = Box<dyn Tool + Send + Sync + 'static>;

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
    fn execute(
        &self,
        request: CallToolRequest,
    ) -> Pin<Box<dyn Future<Output = CallToolResult> + Send + '_>>;
}

pub trait TypedTool {
    type Input;
    type Output;

    /// The name of the tool
    fn name(&self) -> Cow<'static, str>;

    /// The description of the tool
    fn description(&self) -> Cow<'static, str>;

    /// The parameters of the tool
    fn input_schema(&self) -> Cow<'static, str>;

    /// The annotations of the tool
    fn annotations(&self) -> ToolAnnotations;

    /// Execute the tool
    fn execute(&self, arguments: Self::Input) -> impl Future<Output = Self::Output> + Send;
}

impl<T, I, O> Tool for T
where
    T: TypedTool<Input = I, Output = O> + Sync,
    I: serde::de::DeserializeOwned,
    O: IntoToolResult,
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

    fn execute(
        &self,
        request: CallToolRequest,
    ) -> Pin<Box<dyn Future<Output = CallToolResult> + Send + '_>> {
        let input = match serde_json::from_value(serde_json::Value::Object(
            request.params.arguments.clone().unwrap_or_default(),
        )) {
            Ok(input) => input,
            Err(e) => {
                return Box::pin(async move {
                    CallToolResult {
                        content: vec![CallToolResultContent::TextContent(TextContent {
                            text: format!("Error: parsing input: {}", e),
                            annotations: None,
                            r#type: Default::default(),
                            extra: Default::default(),
                        })],
                        is_error: Some(true),
                        meta: None,
                        extra: Default::default(),
                    }
                });
            }
        };
        Box::pin(self.execute(input).map(|output| output.into_tool_result()))
    }
}

#[diagnostic::on_unimplemented(
    message = "`{Self}` must implement `IntoToolResult`",
    note = "Wrap your type in `Text<{Self}>` or `Json<{Self}>` if it implements `Display` or `Serialize`."
)]
pub trait IntoToolResult {
    fn into_tool_result(self) -> CallToolResult;
}

impl<T, E> IntoToolResult for Result<T, E>
where
    T: IntoToolResult,
    E: IntoToolResult,
{
    fn into_tool_result(self) -> CallToolResult {
        match self {
            Ok(output) => output.into_tool_result(),
            Err(error) => error.into_tool_result(),
        }
    }
}

impl<T> IntoToolResult for Option<T>
where
    T: IntoToolResult,
{
    fn into_tool_result(self) -> CallToolResult {
        match self {
            Some(output) => output.into_tool_result(),
            None => CallToolResult {
                content: vec![],
                is_error: None,
                meta: None,
                extra: Default::default(),
            },
        }
    }
}

impl<T> IntoToolResult for Vec<T>
where
    T: IntoToolResult,
{
    fn into_tool_result(self) -> CallToolResult {
        CallToolResult {
            content: self
                .into_iter()
                .flat_map(|item| item.into_tool_result().content)
                .collect(),
            is_error: None,
            meta: None,
            extra: Default::default(),
        }
    }
}

impl IntoToolResult for String {
    fn into_tool_result(self) -> CallToolResult {
        CallToolResult {
            content: vec![CallToolResultContent::TextContent(TextContent {
                text: self,
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

impl IntoToolResult for &str {
    fn into_tool_result(self) -> CallToolResult {
        self.to_string().into_tool_result()
    }
}

pub struct Json<T>(pub T);

impl<T> IntoToolResult for Json<T>
where
    T: Serialize,
{
    fn into_tool_result(self) -> CallToolResult {
        CallToolResult {
            content: vec![CallToolResultContent::TextContent(TextContent {
                text: serde_json::to_string(&self.0).unwrap(),
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

impl IntoToolResult for CallToolResultContent {
    fn into_tool_result(self) -> CallToolResult {
        CallToolResult {
            content: vec![self],
            is_error: None,
            meta: None,
            extra: Default::default(),
        }
    }
}

impl IntoToolResult for TextContent {
    fn into_tool_result(self) -> CallToolResult {
        CallToolResult {
            content: vec![CallToolResultContent::TextContent(self)],
            is_error: None,
            meta: None,
            extra: Default::default(),
        }
    }
}

impl IntoToolResult for ImageContent {
    fn into_tool_result(self) -> CallToolResult {
        CallToolResult {
            content: vec![CallToolResultContent::ImageContent(self)],
            is_error: None,
            meta: None,
            extra: Default::default(),
        }
    }
}

impl IntoToolResult for AudioContent {
    fn into_tool_result(self) -> CallToolResult {
        CallToolResult {
            content: vec![CallToolResultContent::AudioContent(self)],
            is_error: None,
            meta: None,
            extra: Default::default(),
        }
    }
}

impl IntoToolResult for EmbeddedResource {
    fn into_tool_result(self) -> CallToolResult {
        CallToolResult {
            content: vec![CallToolResultContent::EmbeddedResource(self)],
            is_error: None,
            meta: None,
            extra: Default::default(),
        }
    }
}

pub struct Text<T>(pub T);

impl<T> IntoToolResult for Text<T>
where
    T: std::fmt::Display,
{
    fn into_tool_result(self) -> CallToolResult {
        self.0.to_string().into_tool_result()
    }
}
