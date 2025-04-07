use std::collections::HashMap;

use futures::{
    Sink, SinkExt, Stream, StreamExt,
    channel::{mpsc, oneshot},
};
use mmcp_protocol::{
    mcp::{JSONRPCError, JSONRPCMessage, JSONRPCResponse, RequestId},
    port::{RPCPort, RPCSink},
};

type ResponseSubscriber = oneshot::Sender<Result<JSONRPCResponse, JSONRPCError>>;

enum Command {
    WaitResponse {
        request_id: RequestId,
        response: ResponseSubscriber,
    },
}

#[derive(Clone)]
pub struct RPCSender<T> {
    rpc_tx: T,
    command_tx: mpsc::Sender<Command>,
}

pub struct RPCRuntime<T, R> {
    rpc_tx: T,
    rpc_rx: R,
    command_tx: mpsc::Sender<Command>,
    command_rx: mpsc::Receiver<Command>,
    response_subscriptions: Vec<(RequestId, ResponseSubscriber)>,
}

impl<T, R> RPCRuntime<T, R> {
    pub fn new(rpc_tx: T, rpc_rx: R) -> Self {
        let (command_tx, command_rx) = mpsc::channel(100);
        Self {
            rpc_tx,
            rpc_rx,
            command_tx,
            command_rx,
            response_subscriptions: Default::default(),
        }
    }

    // Helper method to find and remove a subscription by request ID
    fn find_subscription(&mut self, id: &RequestId) -> Option<ResponseSubscriber> {
        if let Some(idx) = self
            .response_subscriptions
            .iter()
            .position(|(req_id, _)| req_id == id)
        {
            Some(self.response_subscriptions.swap_remove(idx).1)
        } else {
            None
        }
    }
}

impl<S, R> RPCPort for RPCRuntime<S, R>
where
    S: Sink<JSONRPCMessage> + Unpin + Clone + Send + Sync + 'static,
    R: Stream<Item = JSONRPCMessage> + Unpin + Send + Sync + 'static,
{
    fn sink(&self) -> impl RPCSink + Clone + Send + 'static {
        RPCSender {
            rpc_tx: self.rpc_tx.clone(),
            command_tx: self.command_tx.clone(),
        }
    }

    async fn progress(&mut self) -> anyhow::Result<Option<JSONRPCMessage>> {
        // 1. Process all pending commands first with priority
        while let Ok(Some(command)) = self.command_rx.try_next() {
            match command {
                Command::WaitResponse {
                    request_id,
                    response,
                } => {
                    self.response_subscriptions.push((request_id, response));
                }
            }
        }

        // 2. Try to get a message from the stream, returning None if the stream is closed
        if let Some(message) = self.rpc_rx.next().await {
            // 3. If it's a response, attempt to forward it to subscribers
            match &message {
                JSONRPCMessage::JSONRPCResponse(response) => {
                    if let Some(subscriber) = self.find_subscription(&response.id) {
                        // Ignore errors if the subscriber dropped their receiver
                        let _ = subscriber.send(Ok(response.clone()));
                        // Return the message anyway so callers can process it if needed
                    }
                }
                JSONRPCMessage::JSONRPCError(error) => {
                    if let Some(subscriber) = self.find_subscription(&error.id) {
                        // Ignore errors if the subscriber dropped their receiver
                        let _ = subscriber.send(Err(error.clone()));
                        // Return the message anyway so callers can process it if needed
                    }
                }
                _ => {}
            }

            // Return the message even if it's a response without subscribers
            Ok(Some(message))
        } else {
            // Return None only when the stream is closed
            Ok(None)
        }
    }
}

impl<S> RPCSink for RPCSender<S>
where
    S: Sink<JSONRPCMessage> + Unpin + Send + Sync,
{
    async fn send_message(&mut self, message: JSONRPCMessage) -> anyhow::Result<()> {
        self.rpc_tx
            .send(message)
            .await
            .map_err(|_| anyhow::anyhow!("failed to send message to rpc"))?;
        Ok(())
    }

    async fn send_notification<T: serde::Serialize + Send>(
        &mut self,
        method: &str,
        notification: T,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn send_response<T: serde::Serialize + Send>(
        &mut self,
        request_id: RequestId,
        response: T,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn request<T: serde::Serialize + Send, R: serde::de::DeserializeOwned + Send>(
        &mut self,
        request_id: RequestId,
        method: &str,
        request: T,
    ) -> anyhow::Result<Result<R, JSONRPCError>> {
        use mmcp_protocol::mcp::{JSONRPCRequest, JsonrpcRequestParams};

        // Create oneshot channel for receiving the response
        let (response_tx, response_rx) = oneshot::channel();

        // Create command to register response subscriber
        self.command_tx
            .send(Command::WaitResponse {
                request_id: request_id.clone(),
                response: response_tx,
            })
            .await
            .map_err(|_| anyhow::anyhow!("failed to register response subscriber"))?;

        // Serialize request to JSON
        let params = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("failed to serialize request params: {}", e))?;

        // Create JSON-RPC request
        let rpc_request = JSONRPCRequest {
            id: request_id,
            jsonrpc: monostate::MustBe!("2.0"),
            method: method.to_string(),
            params: Some(JsonrpcRequestParams {
                meta: None,
                extra: serde_json::Map::new(),
            }),
            extra: serde_json::Map::new(),
        };

        // Send request
        self.send_message(JSONRPCMessage::JSONRPCRequest(rpc_request))
            .await?;

        // Wait for response
        let response = response_rx
            .await
            .map_err(|_| anyhow::anyhow!("response channel closed"))?;

        // Process result
        match response {
            Ok(response) => {
                let result = serde_json::from_value(response.result)
                    .map_err(|e| anyhow::anyhow!("failed to deserialize response: {}", e))?;
                Ok(Ok(result))
            }
            Err(error) => Ok(Err(error)),
        }
    }
}
