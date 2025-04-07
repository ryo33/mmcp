use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::protocol::mcp::{JSONRPCError, JSONRPCMessage, RequestId};

#[derive(thiserror::Error, Debug)]
pub enum RPCPortError {
    #[error("stream has been closed")]
    StreamClosed,
}

pub trait RPCPort {
    /// Wait until a specific request is received. This is convenient for waiting for the initialization request.
    fn wait_for_request<T: DeserializeOwned + Send>(
        &mut self,
    ) -> impl Future<Output = anyhow::Result<TypedRequest<T>>> + Send;

    /// Wait until a specific notification is received. This is convenient for waiting for the initialized notification.
    fn wait_for_notification<T: DeserializeOwned + Send>(
        &mut self,
    ) -> impl Future<Output = anyhow::Result<TypedNotification<T>>> + Send;

    /// Send a response to a request.
    fn send_response<T: Serialize + Send>(
        &self,
        response: TypedResponse<T>,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;

    /// Send a notification to the peer.
    fn send_notification<T: Serialize + Send>(
        &self,
        notification: TypedNotification<T>,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;

    /// Send a request to the peer and wait for a response.
    fn request<T: Serialize + Send, R: DeserializeOwned + Send>(
        &mut self,
        request: TypedRequest<T>,
    ) -> impl Future<Output = anyhow::Result<Result<TypedResponse<R>, JSONRPCError>>> + Send;

    fn receive_any_message(
        &mut self,
    ) -> impl Future<Output = anyhow::Result<JSONRPCMessage>> + Send;

    fn send_message(
        &self,
        message: JSONRPCMessage,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedRequest<T> {
    pub id: RequestId,
    pub method: String,
    pub params: T,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
///A successful (non-error) response to a request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedResponse<T> {
    pub id: RequestId,
    pub result: T,
    /// Additional parameters that are not part of the schema.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypedNotification<T> {
    pub method: String,
    pub params: T,
}
