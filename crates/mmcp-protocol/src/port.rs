use serde::{Serialize, de::DeserializeOwned};

use crate::mcp::{JSONRPCError, JSONRPCMessage, RequestId};

#[derive(thiserror::Error, Debug)]
pub enum RPCPortError {
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("failed to serialize result to object got {0}")]
    SerializeNotObject(serde_json::Value),
}

pub trait RPCSink {
    fn send_message(
        &mut self,
        message: JSONRPCMessage,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;

    /// Send a notification to the peer.
    fn send_notification<T: Serialize + Send>(
        &mut self,
        method: &str,
        notification: T,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;

    /// Send a response to the peer.
    fn send_response<T: Serialize + Send>(
        &mut self,
        request_id: RequestId,
        response: T,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;

    /// Send a request to the peer and wait for a response.
    fn request<T: Serialize + Send, R: DeserializeOwned + Send>(
        &mut self,
        request_id: RequestId,
        method: &str,
        request: T,
    ) -> impl Future<Output = anyhow::Result<Result<R, JSONRPCError>>> + Send;
}

pub trait RPCPort {
    /// Get the sink for sending messages to the peer.
    fn sink(&self) -> impl RPCSink + Clone + Send + 'static;
    /// Fetch a message from the peer. Handling commands in the background which sent by the `RPCSink`.
    fn progress(
        &mut self,
    ) -> impl std::future::Future<Output = anyhow::Result<Option<JSONRPCMessage>>> + Send;
}
