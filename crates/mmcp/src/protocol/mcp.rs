///Optional annotations for the client. The client can use annotations to inform how objects are used or displayed
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Annotations {
    /**Describes who the intended customer of this object or data is.

It can include multiple entries to indicate content useful for multiple audiences (e.g., `["user", "assistant"]`).*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<Role>>,
    /**Describes how important this data is for operating the server.

A value of 1 means "most important," and indicates that the data is
effectively required, while 0 means "least important," and indicates that
the data is entirely optional.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Audio provided to or from an LLM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde_with::serde_as]
pub struct AudioContent {
    ///Optional annotations for the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    ///The base64-encoded audio data.
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
    ///The MIME type of the audio. Different providers may support different audio types.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub r#type: monostate::MustBe!("audio"),
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for BlobResourceContents
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde_with::serde_as]
pub struct BlobResourceContents {
    ///A base64-encoded string representing the binary data of the item.
    #[serde_as(as = "Base64")]
    pub blob: Vec<u8>,
    ///The MIME type of this resource, if known.
    #[serde(rename = "mimeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///The URI of this resource.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for CallToolRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CallToolRequestParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Map<String, serde_json::Value>>,
    pub name: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Used by the client to invoke a tool provided by the server.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CallToolRequest {
    pub method: monostate::MustBe!("tools/call"),
    pub params: CallToolRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for CallToolResultContent
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CallToolResultContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
    EmbeddedResource(EmbeddedResource),
}
/**The server's response to a tool call.

Any errors that originate from the tool SHOULD be reported inside the result
object, with `isError` set to true, _not_ as an MCP protocol-level error
response. Otherwise, the LLM would not be able to see that an error occurred
and self-correct.

However, any errors in _finding_ the tool, an error indicating that the
server does not support tool calls, or any other exceptional conditions,
should be reported as an MCP error response.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CallToolResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    pub content: Vec<CallToolResultContent>,
    /**Whether the tool call ended in an error.

If not set, this is assumed to be false (the call was successful).*/
    #[serde(rename = "isError")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for CancelledNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelledNotificationParams {
    ///An optional string describing the reason for the cancellation. This MAY be logged or presented to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /**The ID of the request to cancel.

This MUST correspond to the ID of a request previously issued in the same direction.*/
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/**This notification can be sent by either side to indicate that it is cancelling a previously-issued request.

The request SHOULD still be in-flight, but due to communication latency, it is always possible that this notification MAY arrive after the request has already finished.

This notification indicates that the result will be unused, so any associated processing SHOULD cease.

A client MUST NOT attempt to cancel its `initialize` request.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelledNotification {
    pub method: monostate::MustBe!("notifications/cancelled"),
    pub params: CancelledNotificationParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Present if the client supports listing roots.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClientCapabilitiesRoots {
    ///Whether the client supports notifications for changes to the roots list.
    #[serde(rename = "listChanged")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Capabilities a client may support. Known capabilities are defined here, in this schema, but this is not a closed set: any client can define its own, additional capabilities.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClientCapabilities {
    ///Experimental, non-standard capabilities that the client supports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental: Option<
        indexmap::IndexMap<String, serde_json::Map<String, serde_json::Value>>,
    >,
    ///Present if the client supports listing roots.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roots: Option<ClientCapabilitiesRoots>,
    ///Present if the client supports sampling from an LLM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampling: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ClientNotification
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ClientNotification {
    CancelledNotification(CancelledNotification),
    InitializedNotification(InitializedNotification),
    ProgressNotification(ProgressNotification),
    RootsListChangedNotification(RootsListChangedNotification),
}
///Generated from JSON schema definition for ClientRequest
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ClientRequest {
    InitializeRequest(InitializeRequest),
    PingRequest(PingRequest),
    ListResourcesRequest(ListResourcesRequest),
    ReadResourceRequest(ReadResourceRequest),
    SubscribeRequest(SubscribeRequest),
    UnsubscribeRequest(UnsubscribeRequest),
    ListPromptsRequest(ListPromptsRequest),
    GetPromptRequest(GetPromptRequest),
    ListToolsRequest(ListToolsRequest),
    CallToolRequest(CallToolRequest),
    SetLevelRequest(SetLevelRequest),
    CompleteRequest(CompleteRequest),
}
///Generated from JSON schema definition for ClientResult
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ClientResult {
    Result(Result),
    CreateMessageResult(CreateMessageResult),
    ListRootsResult(ListRootsResult),
}
///The argument's information
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompleteRequestParamsArgument {
    ///The name of the argument
    pub name: String,
    ///The value of the argument to use for completion matching.
    pub value: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for CompleteRequestParamsRef
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CompleteRequestParamsRef {
    PromptReference(PromptReference),
    ResourceReference(ResourceReference),
}
///Generated from JSON schema definition for CompleteRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompleteRequestParams {
    ///The argument's information
    pub argument: CompleteRequestParamsArgument,
    pub r#ref: CompleteRequestParamsRef,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A request from the client to the server, to ask for completion options.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompleteRequest {
    pub method: monostate::MustBe!("completion/complete"),
    pub params: CompleteRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for CompleteResultCompletion
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompleteResultCompletion {
    ///Indicates whether there are additional completion options beyond those provided in the current response, even if the exact total is unknown.
    #[serde(rename = "hasMore")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    ///The total number of completion options available. This can exceed the number of values actually sent in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    ///An array of completion values. Must not exceed 100 items.
    pub values: Vec<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///The server's response to a completion/complete request
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompleteResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    pub completion: CompleteResultCompletion,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CreateMessageRequestParamsIncludeContext {
    #[serde(rename = "allServers")]
    AllServers,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "thisServer")]
    ThisServer,
}
///Generated from JSON schema definition for CreateMessageRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMessageRequestParams {
    ///A request to include context from one or more MCP servers (including the caller), to be attached to the prompt. The client MAY ignore this request.
    #[serde(rename = "includeContext")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_context: Option<CreateMessageRequestParamsIncludeContext>,
    ///The maximum number of tokens to sample, as requested by the server. The client MAY choose to sample fewer tokens than requested.
    #[serde(rename = "maxTokens")]
    pub max_tokens: i64,
    pub messages: Vec<SamplingMessage>,
    ///Optional metadata to pass through to the LLM provider. The format of this metadata is provider-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
    ///The server's preferences for which model to select. The client MAY ignore these preferences.
    #[serde(rename = "modelPreferences")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_preferences: Option<ModelPreferences>,
    #[serde(rename = "stopSequences")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    ///An optional system prompt the server wants to use for sampling. The client MAY modify or omit this prompt.
    #[serde(rename = "systemPrompt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A request from the server to sample an LLM via the client. The client has full discretion over which model to select. The client should also inform the user before beginning sampling, to allow them to inspect the request (human in the loop) and decide whether to approve it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMessageRequest {
    pub method: monostate::MustBe!("sampling/createMessage"),
    pub params: CreateMessageRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for CreateMessageResultContent
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum CreateMessageResultContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
}
///The client's response to a sampling/create_message request from the server. The client should inform the user before returning the sampled message, to allow them to inspect the response (human in the loop) and decide whether to allow the server to see it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMessageResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    pub content: CreateMessageResultContent,
    ///The name of the model that generated the message.
    pub model: String,
    pub role: Role,
    ///The reason why sampling stopped, if known.
    #[serde(rename = "stopReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///An opaque token used to represent a cursor for pagination.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Cursor(pub String);
///Generated from JSON schema definition for EmbeddedResourceResource
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum EmbeddedResourceResource {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
/**The contents of a resource, embedded into a prompt or tool call result.

It is up to the client how best to render embedded resources for the benefit
of the LLM and/or the user.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EmbeddedResource {
    ///Optional annotations for the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    pub resource: EmbeddedResourceResource,
    pub r#type: monostate::MustBe!("resource"),
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for EmptyResult
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct EmptyResult(pub Result);
///Generated from JSON schema definition for GetPromptRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPromptRequestParams {
    ///Arguments to use for templating the prompt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<indexmap::IndexMap<String, String>>,
    ///The name of the prompt or prompt template.
    pub name: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Used by the client to get a prompt provided by the server.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPromptRequest {
    pub method: monostate::MustBe!("prompts/get"),
    pub params: GetPromptRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///The server's response to a prompts/get request from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPromptResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    ///An optional description for the prompt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub messages: Vec<PromptMessage>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///An image provided to or from an LLM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde_with::serde_as]
pub struct ImageContent {
    ///Optional annotations for the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    ///The base64-encoded image data.
    #[serde_as(as = "Base64")]
    pub data: Vec<u8>,
    ///The MIME type of the image. Different providers may support different image types.
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub r#type: monostate::MustBe!("image"),
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Describes the name and version of an MCP implementation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Implementation {
    pub name: String,
    pub version: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for InitializeRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeRequestParams {
    pub capabilities: ClientCapabilities,
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
    ///The latest version of the Model Context Protocol that the client supports. The client MAY decide to support older versions as well.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///This request is sent from the client to the server when it first connects, asking it to begin initialization.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeRequest {
    pub method: monostate::MustBe!("initialize"),
    pub params: InitializeRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///After receiving an initialize request from the client, the server sends this response.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializeResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    pub capabilities: ServerCapabilities,
    /**Instructions describing how to use the server and its features.

This can be used by clients to improve the LLM's understanding of available tools, resources, etc. It can be thought of like a "hint" to the model. For example, this information MAY be added to the system prompt.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    ///The version of the Model Context Protocol that the server wants to use. This may not match the version that the client requested. If the client cannot support this version, it MUST disconnect.
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(rename = "serverInfo")]
    pub server_info: Implementation,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for InitializedNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializedNotificationParams {
    ///This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///This notification is sent from the client to the server after initialization has finished.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InitializedNotification {
    pub method: monostate::MustBe!("notifications/initialized"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<InitializedNotificationParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for JsonrpcBatchRequestItem
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum JsonrpcBatchRequestItem {
    JSONRPCRequest(JSONRPCRequest),
    JSONRPCNotification(JSONRPCNotification),
}
///A JSON-RPC batch request, as described in https://www.jsonrpc.org/specification#batch.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct JSONRPCBatchRequest(pub Vec<JsonrpcBatchRequestItem>);
///Generated from JSON schema definition for JsonrpcBatchResponseItem
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum JsonrpcBatchResponseItem {
    JSONRPCResponse(JSONRPCResponse),
    JSONRPCError(JSONRPCError),
}
///A JSON-RPC batch response, as described in https://www.jsonrpc.org/specification#batch.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct JSONRPCBatchResponse(pub Vec<JsonrpcBatchResponseItem>);
///Generated from JSON schema definition for JsonrpcErrorError
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonrpcErrorError {
    ///The error type that occurred.
    pub code: i64,
    ///Additional information about the error. The value of this member is defined by the sender (e.g. detailed error information, nested errors etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    ///A short description of the error. The message SHOULD be limited to a concise single sentence.
    pub message: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A response to a request that indicates an error occurred.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JSONRPCError {
    pub error: JsonrpcErrorError,
    pub id: RequestId,
    pub jsonrpc: monostate::MustBe!("2.0"),
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Refers to any valid JSON-RPC object that can be decoded off the wire, or encoded to be sent.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum JSONRPCMessage {
    JSONRPCRequest(JSONRPCRequest),
    JSONRPCNotification(JSONRPCNotification),
    JSONRPCBatchRequest(JSONRPCBatchRequest),
    JSONRPCResponse(JSONRPCResponse),
    JSONRPCError(JSONRPCError),
    JSONRPCBatchResponse(JSONRPCBatchResponse),
}
///Generated from JSON schema definition for JsonrpcNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonrpcNotificationParams {
    ///This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A notification which does not expect a response.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JSONRPCNotification {
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<JsonrpcNotificationParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for JsonrpcRequestParamsMeta
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonrpcRequestParamsMeta {
    ///If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.
    #[serde(rename = "progressToken")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<ProgressToken>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for JsonrpcRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JsonrpcRequestParams {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<JsonrpcRequestParamsMeta>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A request that expects a response.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JSONRPCRequest {
    pub id: RequestId,
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<JsonrpcRequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A successful (non-error) response to a request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct JSONRPCResponse {
    pub id: RequestId,
    pub jsonrpc: monostate::MustBe!("2.0"),
    pub result: Result,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ListPromptsRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListPromptsRequestParams {
    /**An opaque token representing the current pagination position.
If provided, the server should return results starting after this cursor.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Sent from the client to request a list of prompts and prompt templates the server has.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListPromptsRequest {
    pub method: monostate::MustBe!("prompts/list"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<ListPromptsRequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///The server's response to a prompts/list request from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListPromptsResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /**An opaque token representing the pagination position after the last returned result.
If present, there may be more results available.*/
    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub prompts: Vec<Prompt>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ListResourceTemplatesRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListResourceTemplatesRequestParams {
    /**An opaque token representing the current pagination position.
If provided, the server should return results starting after this cursor.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Sent from the client to request a list of resource templates the server has.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListResourceTemplatesRequest {
    pub method: monostate::MustBe!("resources/templates/list"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<ListResourceTemplatesRequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///The server's response to a resources/templates/list request from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListResourceTemplatesResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /**An opaque token representing the pagination position after the last returned result.
If present, there may be more results available.*/
    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: Vec<ResourceTemplate>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ListResourcesRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListResourcesRequestParams {
    /**An opaque token representing the current pagination position.
If provided, the server should return results starting after this cursor.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Sent from the client to request a list of resources the server has.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListResourcesRequest {
    pub method: monostate::MustBe!("resources/list"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<ListResourcesRequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///The server's response to a resources/list request from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListResourcesResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /**An opaque token representing the pagination position after the last returned result.
If present, there may be more results available.*/
    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub resources: Vec<Resource>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ListRootsRequestParamsMeta
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListRootsRequestParamsMeta {
    ///If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.
    #[serde(rename = "progressToken")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<ProgressToken>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ListRootsRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListRootsRequestParams {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<ListRootsRequestParamsMeta>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/**Sent from the server to request a list of root URIs from the client. Roots allow
servers to ask for specific directories or files to operate on. A common example
for roots is providing a set of repositories or directories a server should operate
on.

This request is typically used when the server needs to understand the file system
structure or access specific locations that the client has permission to read from.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListRootsRequest {
    pub method: monostate::MustBe!("roots/list"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<ListRootsRequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/**The client's response to a roots/list request from the server.
This result contains an array of Root objects, each representing a root directory
or file that the server can operate on.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListRootsResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    pub roots: Vec<Root>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ListToolsRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListToolsRequestParams {
    /**An opaque token representing the current pagination position.
If provided, the server should return results starting after this cursor.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Sent from the client to request a list of tools the server has.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListToolsRequest {
    pub method: monostate::MustBe!("tools/list"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<ListToolsRequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///The server's response to a tools/list request from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ListToolsResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /**An opaque token representing the pagination position after the last returned result.
If present, there may be more results available.*/
    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub tools: Vec<Tool>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/**The severity of a log message.

These map to syslog message severities, as specified in RFC-5424:
https://datatracker.ietf.org/doc/html/rfc5424#section-6.2.1*/
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LoggingLevel {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emergency")]
    Emergency,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
///Generated from JSON schema definition for LoggingMessageNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LoggingMessageNotificationParams {
    ///The data to be logged, such as a string message or an object. Any JSON serializable type is allowed here.
    pub data: serde_json::Value,
    ///The severity of this log message.
    pub level: LoggingLevel,
    ///An optional name of the logger issuing this message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Notification of a log message passed from server to client. If no logging/setLevel request has been sent from the client, the server MAY decide which messages to send automatically.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LoggingMessageNotification {
    pub method: monostate::MustBe!("notifications/message"),
    pub params: LoggingMessageNotificationParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/**Hints to use for model selection.

Keys not declared here are currently left unspecified by the spec and are up
to the client to interpret.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelHint {
    /**A hint for a model name.

The client SHOULD treat this as a substring of a model name; for example:
 - `claude-3-5-sonnet` should match `claude-3-5-sonnet-20241022`
 - `sonnet` should match `claude-3-5-sonnet-20241022`, `claude-3-sonnet-20240229`, etc.
 - `claude` should match any Claude model

The client MAY also map the string to a different provider's model name or a different model family, as long as it fills a similar niche; for example:
 - `gemini-1.5-flash` could match `claude-3-haiku-20240307`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/**The server's preferences for model selection, requested of the client during sampling.

Because LLMs can vary along multiple dimensions, choosing the "best" model is
rarely straightforward.  Different models excel in different areas—some are
faster but less capable, others are more capable but more expensive, and so
on. This interface allows servers to express their priorities across multiple
dimensions to help clients make an appropriate selection for their use case.

These preferences are always advisory. The client MAY ignore them. It is also
up to the client to decide how to interpret these preferences and how to
balance them against other considerations.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelPreferences {
    /**How much to prioritize cost when selecting a model. A value of 0 means cost
is not important, while a value of 1 means cost is the most important
factor.*/
    #[serde(rename = "costPriority")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_priority: Option<f64>,
    /**Optional hints to use for model selection.

If multiple hints are specified, the client MUST evaluate them in order
(such that the first match is taken).

The client SHOULD prioritize these hints over the numeric priorities, but
MAY still use the priorities to select from ambiguous matches.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<ModelHint>>,
    /**How much to prioritize intelligence and capabilities when selecting a
model. A value of 0 means intelligence is not important, while a value of 1
means intelligence is the most important factor.*/
    #[serde(rename = "intelligencePriority")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intelligence_priority: Option<f64>,
    /**How much to prioritize sampling speed (latency) when selecting a model. A
value of 0 means speed is not important, while a value of 1 means speed is
the most important factor.*/
    #[serde(rename = "speedPriority")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed_priority: Option<f64>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for NotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotificationParams {
    ///This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for Notification
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Notification {
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<NotificationParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for PaginatedRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaginatedRequestParams {
    /**An opaque token representing the current pagination position.
If provided, the server should return results starting after this cursor.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for PaginatedRequest
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaginatedRequest {
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<PaginatedRequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for PaginatedResult
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaginatedResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /**An opaque token representing the pagination position after the last returned result.
If present, there may be more results available.*/
    #[serde(rename = "nextCursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for PingRequestParamsMeta
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PingRequestParamsMeta {
    ///If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.
    #[serde(rename = "progressToken")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<ProgressToken>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for PingRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PingRequestParams {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<PingRequestParamsMeta>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A ping, issued by either the server or the client, to check that the other party is still alive. The receiver must promptly respond, or else may be disconnected.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PingRequest {
    pub method: monostate::MustBe!("ping"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<PingRequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ProgressNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProgressNotificationParams {
    ///An optional message describing the current progress.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    ///The progress thus far. This should increase every time progress is made, even if the total is unknown.
    pub progress: f64,
    ///The progress token which was given in the initial request, used to associate this notification with the request that is proceeding.
    #[serde(rename = "progressToken")]
    pub progress_token: ProgressToken,
    ///Total number of items to process (or total progress required), if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///An out-of-band notification used to inform the receiver of a progress update for a long-running request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProgressNotification {
    pub method: monostate::MustBe!("notifications/progress"),
    pub params: ProgressNotificationParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A progress token, used to associate progress notifications with the original request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ProgressToken {
    String(String),
    Integer(i64),
}
///A prompt or prompt template that the server offers.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Prompt {
    ///A list of arguments to use for templating the prompt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
    ///An optional description of what this prompt provides
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The name of the prompt or prompt template.
    pub name: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Describes an argument that a prompt can accept.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromptArgument {
    ///A human-readable description of the argument.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The name of the argument.
    pub name: String,
    ///Whether this argument must be provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for PromptListChangedNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromptListChangedNotificationParams {
    ///This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///An optional notification from the server to the client, informing it that the list of prompts it offers has changed. This may be issued by servers without any previous subscription from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromptListChangedNotification {
    pub method: monostate::MustBe!("notifications/prompts/list_changed"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<PromptListChangedNotificationParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for PromptMessageContent
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum PromptMessageContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
    EmbeddedResource(EmbeddedResource),
}
/**Describes a message returned as part of a prompt.

This is similar to `SamplingMessage`, but also supports the embedding of
resources from the MCP server.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromptMessage {
    pub content: PromptMessageContent,
    pub role: Role,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Identifies a prompt.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromptReference {
    ///The name of the prompt or prompt template
    pub name: String,
    pub r#type: monostate::MustBe!("ref/prompt"),
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ReadResourceRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReadResourceRequestParams {
    ///The URI of the resource to read. The URI can use any protocol; it is up to the server how to interpret it.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Sent from the client to the server, to read a specific resource URI.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReadResourceRequest {
    pub method: monostate::MustBe!("resources/read"),
    pub params: ReadResourceRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ReadResourceResultContents
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ReadResourceResultContents {
    TextResourceContents(TextResourceContents),
    BlobResourceContents(BlobResourceContents),
}
///The server's response to a resources/read request from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReadResourceResult {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    pub contents: Vec<ReadResourceResultContents>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for RequestParamsMeta
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RequestParamsMeta {
    ///If specified, the caller is requesting out-of-band progress notifications for this request (as represented by notifications/progress). The value of this parameter is an opaque token that will be attached to any subsequent notifications. The receiver is not obligated to provide these notifications.
    #[serde(rename = "progressToken")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<ProgressToken>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for RequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RequestParams {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestParamsMeta>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for Request
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Request {
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<RequestParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A uniquely identifying ID for a request in JSON-RPC.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    String(String),
    Integer(i64),
}
///A known resource that the server is capable of reading.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Resource {
    ///Optional annotations for the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    /**A description of what this resource represents.

This can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a "hint" to the model.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The MIME type of this resource, if known.
    #[serde(rename = "mimeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /**A human-readable name for this resource.

This can be used by clients to populate UI elements.*/
    pub name: String,
    ///The URI of this resource.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///The contents of a specific resource or sub-resource.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceContents {
    ///The MIME type of this resource, if known.
    #[serde(rename = "mimeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///The URI of this resource.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ResourceListChangedNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceListChangedNotificationParams {
    ///This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///An optional notification from the server to the client, informing it that the list of resources it can read from has changed. This may be issued by servers without any previous subscription from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceListChangedNotification {
    pub method: monostate::MustBe!("notifications/resources/list_changed"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<ResourceListChangedNotificationParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A reference to a resource or resource template definition.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceReference {
    pub r#type: monostate::MustBe!("ref/resource"),
    ///The URI or URI template of the resource.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A template description for resources available on the server.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceTemplate {
    ///Optional annotations for the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    /**A description of what this template is for.

This can be used by clients to improve the LLM's understanding of available resources. It can be thought of like a "hint" to the model.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The MIME type for all resources that match this template. This should only be included if all resources matching this template have the same type.
    #[serde(rename = "mimeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /**A human-readable name for the type of resource this template refers to.

This can be used by clients to populate UI elements.*/
    pub name: String,
    ///A URI template (according to RFC 6570) that can be used to construct resource URIs.
    #[serde(rename = "uriTemplate")]
    pub uri_template: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ResourceUpdatedNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceUpdatedNotificationParams {
    ///The URI of the resource that has been updated. This might be a sub-resource of the one that the client actually subscribed to.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A notification from the server to the client, informing it that a resource has changed and may need to be read again. This should only be sent if the client previously sent a resources/subscribe request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourceUpdatedNotification {
    pub method: monostate::MustBe!("notifications/resources/updated"),
    pub params: ResourceUpdatedNotificationParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for Result
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Result {
    ///This result property is reserved by the protocol to allow clients and servers to attach additional metadata to their responses.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///The sender or recipient of messages and data in a conversation.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}
///Represents a root directory or file that the server can operate on.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Root {
    /**An optional name for the root. This can be used to provide a human-readable
identifier for the root, which may be useful for display purposes or for
referencing the root in other parts of the application.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /**The URI identifying the root. This *must* start with file:// for now.
This restriction may be relaxed in future versions of the protocol to allow
other URI schemes.*/
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for RootsListChangedNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RootsListChangedNotificationParams {
    ///This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/**A notification from the client to the server, informing it that the list of roots has changed.
This notification should be sent whenever the client adds, removes, or modifies any root.
The server should then request an updated list of roots using the ListRootsRequest.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RootsListChangedNotification {
    pub method: monostate::MustBe!("notifications/roots/list_changed"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<RootsListChangedNotificationParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for SamplingMessageContent
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum SamplingMessageContent {
    TextContent(TextContent),
    ImageContent(ImageContent),
    AudioContent(AudioContent),
}
///Describes a message issued to or received from an LLM API.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SamplingMessage {
    pub content: SamplingMessageContent,
    pub role: Role,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Present if the server offers any prompt templates.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerCapabilitiesPrompts {
    ///Whether this server supports notifications for changes to the prompt list.
    #[serde(rename = "listChanged")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Present if the server offers any resources to read.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerCapabilitiesResources {
    ///Whether this server supports notifications for changes to the resource list.
    #[serde(rename = "listChanged")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    ///Whether this server supports subscribing to resource updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Present if the server offers any tools to call.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerCapabilitiesTools {
    ///Whether this server supports notifications for changes to the tool list.
    #[serde(rename = "listChanged")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Capabilities that a server may support. Known capabilities are defined here, in this schema, but this is not a closed set: any server can define its own, additional capabilities.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServerCapabilities {
    ///Present if the server supports argument autocompletion suggestions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completions: Option<serde_json::Map<String, serde_json::Value>>,
    ///Experimental, non-standard capabilities that the server supports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experimental: Option<
        indexmap::IndexMap<String, serde_json::Map<String, serde_json::Value>>,
    >,
    ///Present if the server supports sending log messages to the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logging: Option<serde_json::Map<String, serde_json::Value>>,
    ///Present if the server offers any prompt templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompts: Option<ServerCapabilitiesPrompts>,
    ///Present if the server offers any resources to read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ServerCapabilitiesResources>,
    ///Present if the server offers any tools to call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<ServerCapabilitiesTools>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ServerNotification
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ServerNotification {
    CancelledNotification(CancelledNotification),
    ProgressNotification(ProgressNotification),
    ResourceListChangedNotification(ResourceListChangedNotification),
    ResourceUpdatedNotification(ResourceUpdatedNotification),
    PromptListChangedNotification(PromptListChangedNotification),
    ToolListChangedNotification(ToolListChangedNotification),
    LoggingMessageNotification(LoggingMessageNotification),
}
///Generated from JSON schema definition for ServerRequest
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ServerRequest {
    PingRequest(PingRequest),
    CreateMessageRequest(CreateMessageRequest),
    ListRootsRequest(ListRootsRequest),
}
///Generated from JSON schema definition for ServerResult
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ServerResult {
    Result(Result),
    InitializeResult(InitializeResult),
    ListResourcesResult(ListResourcesResult),
    ReadResourceResult(ReadResourceResult),
    ListPromptsResult(ListPromptsResult),
    GetPromptResult(GetPromptResult),
    ListToolsResult(ListToolsResult),
    CallToolResult(CallToolResult),
    CompleteResult(CompleteResult),
}
///Generated from JSON schema definition for SetLevelRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLevelRequestParams {
    ///The level of logging that the client wants to receive from the server. The server should send all logs at this level and higher (i.e., more severe) to the client as notifications/message.
    pub level: LoggingLevel,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A request from the client to the server, to enable or adjust logging.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SetLevelRequest {
    pub method: monostate::MustBe!("logging/setLevel"),
    pub params: SetLevelRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for SubscribeRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubscribeRequestParams {
    ///The URI of the resource to subscribe to. The URI can use any protocol; it is up to the server how to interpret it.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Sent from the client to request resources/updated notifications from the server whenever a particular resource changes.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubscribeRequest {
    pub method: monostate::MustBe!("resources/subscribe"),
    pub params: SubscribeRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Text provided to or from an LLM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextContent {
    ///Optional annotations for the client.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    ///The text content of the message.
    pub text: String,
    pub r#type: monostate::MustBe!("text"),
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for TextResourceContents
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextResourceContents {
    ///The MIME type of this resource, if known.
    #[serde(rename = "mimeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    ///The text of the item. This must only be set if the item can actually be represented as text (not binary data).
    pub text: String,
    ///The URI of this resource.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A JSON Schema object defining the expected parameters for the tool.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolInputSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<
        indexmap::IndexMap<String, serde_json::Map<String, serde_json::Value>>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    pub r#type: monostate::MustBe!("object"),
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Definition for a tool the client can call.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tool {
    ///Optional additional tool information.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ToolAnnotations>,
    /**A human-readable description of the tool.

This can be used by clients to improve the LLM's understanding of available tools. It can be thought of like a "hint" to the model.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///A JSON Schema object defining the expected parameters for the tool.
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
    ///The name of the tool.
    pub name: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/**Additional properties describing a Tool to clients.

NOTE: all properties in ToolAnnotations are **hints**.
They are not guaranteed to provide a faithful description of
tool behavior (including descriptive properties like `title`).

Clients should never make tool use decisions based on ToolAnnotations
received from untrusted servers.*/
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolAnnotations {
    /**If true, the tool may perform destructive updates to its environment.
If false, the tool performs only additive updates.

(This property is meaningful only when `readOnlyHint == false`)

Default: true*/
    #[serde(rename = "destructiveHint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destructive_hint: Option<bool>,
    /**If true, calling the tool repeatedly with the same arguments
will have no additional effect on the its environment.

(This property is meaningful only when `readOnlyHint == false`)

Default: false*/
    #[serde(rename = "idempotentHint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_hint: Option<bool>,
    /**If true, this tool may interact with an "open world" of external
entities. If false, the tool's domain of interaction is closed.
For example, the world of a web search tool is open, whereas that
of a memory tool is not.

Default: true*/
    #[serde(rename = "openWorldHint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_world_hint: Option<bool>,
    /**If true, the tool does not modify its environment.

Default: false*/
    #[serde(rename = "readOnlyHint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_hint: Option<bool>,
    ///A human-readable title for the tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for ToolListChangedNotificationParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolListChangedNotificationParams {
    ///This parameter name is reserved by MCP to allow clients and servers to attach additional metadata to their notifications.
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///An optional notification from the server to the client, informing it that the list of tools it offers has changed. This may be issued by servers without any previous subscription from the client.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ToolListChangedNotification {
    pub method: monostate::MustBe!("notifications/tools/list_changed"),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<ToolListChangedNotificationParams>,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Generated from JSON schema definition for UnsubscribeRequestParams
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnsubscribeRequestParams {
    ///The URI of the resource to unsubscribe from.
    pub uri: String,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///Sent from the client to request cancellation of resources/updated notifications from the server. This should follow a previous resources/subscribe request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UnsubscribeRequest {
    pub method: monostate::MustBe!("resources/unsubscribe"),
    pub params: UnsubscribeRequestParams,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
