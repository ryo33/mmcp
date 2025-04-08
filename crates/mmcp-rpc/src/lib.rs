use std::collections::HashMap;

use futures::{
    Sink, SinkExt, Stream, StreamExt,
    channel::{mpsc, oneshot},
};
use mmcp_protocol::{
    mcp::{
        JSONRPCError, JSONRPCMessage, JSONRPCNotification, JSONRPCRequest, JSONRPCResponse,
        JsonrpcBatchResponseItem, JsonrpcNotificationParams, JsonrpcRequestParams, RequestId,
        Result as JsonRpcResult,
    },
    port::{RPCPort, RPCPortError, RPCSink},
};
use serde_json::Value;

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
    response_subscriptions: HashMap<RequestId, ResponseSubscriber>,
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
                    self.response_subscriptions.insert(request_id, response);
                }
            }
        }

        // 2. Try to get a message from the stream, returning None if the stream is closed
        if let Some(message) = self.rpc_rx.next().await {
            // 3. If it's a response, attempt to forward it to subscribers
            match &message {
                JSONRPCMessage::JSONRPCResponse(response) => {
                    self.handle_response(response);
                }
                JSONRPCMessage::JSONRPCError(error) => {
                    self.handle_error(error);
                }
                JSONRPCMessage::JSONRPCBatchResponse(batch) => {
                    for item in batch.0.iter() {
                        match item {
                            JsonrpcBatchResponseItem::JSONRPCResponse(response) => {
                                self.handle_response(response);
                            }
                            JsonrpcBatchResponseItem::JSONRPCError(error) => {
                                self.handle_error(error);
                            }
                        }
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

impl<S, R> RPCRuntime<S, R>
where
    S: Sink<JSONRPCMessage> + Unpin + Clone + Send + Sync + 'static,
    R: Stream<Item = JSONRPCMessage> + Unpin + Send + Sync + 'static,
{
    fn handle_response(&mut self, response: &JSONRPCResponse) {
        if let Some(subscriber) = self.response_subscriptions.remove(&response.id) {
            // Ignore errors if the subscriber dropped their receiver
            let _ = subscriber.send(Ok(response.clone()));
            // Return the message anyway so callers can process it if needed
        }
    }

    fn handle_error(&mut self, error: &JSONRPCError) {
        if let Some(subscriber) = self.response_subscriptions.remove(&error.id) {
            // Ignore errors if the subscriber dropped their receiver
            let _ = subscriber.send(Err(error.clone()));
            // Return the message anyway so callers can process it if needed
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
        // Serialize notification to JSON
        let notification_value = serde_json::to_value(notification)
            .map_err(|e| anyhow::anyhow!("failed to serialize notification: {}", e))?;

        // If the notification is already a JSON object or null for optional params, extract it
        let params = match notification_value {
            Value::Object(obj) => Some(JsonrpcNotificationParams {
                meta: None,
                extra: obj,
            }),
            Value::Null => None, // Allow null for optional params
            _ => return Err(RPCPortError::SerializeNotObject(notification_value).into()),
        };

        // Create notification message
        let rpc_notification = JSONRPCNotification {
            jsonrpc: Default::default(),
            method: method.to_string(),
            params,
            extra: Default::default(),
        };

        // Send notification
        self.send_message(JSONRPCMessage::JSONRPCNotification(rpc_notification))
            .await
    }

    async fn send_response<T: serde::Serialize + Send>(
        &mut self,
        request_id: RequestId,
        response: T,
    ) -> anyhow::Result<()> {
        // Serialize response
        let response_value = serde_json::to_value(response)
            .map_err(|e| anyhow::anyhow!("failed to serialize response: {}", e))?;

        // Create JSON-RPC response with Result
        let result = JsonRpcResult {
            meta: None,
            extra: match response_value {
                Value::Object(obj) => obj,
                _ => return Err(RPCPortError::SerializeNotObject(response_value).into()),
            },
        };

        let rpc_response = JSONRPCResponse {
            id: request_id,
            jsonrpc: Default::default(),
            result,
            extra: Default::default(),
        };

        // Send response
        self.send_message(JSONRPCMessage::JSONRPCResponse(rpc_response))
            .await
    }

    async fn request<T: serde::Serialize + Send, R: serde::de::DeserializeOwned + Send>(
        &mut self,
        request_id: RequestId,
        method: &str,
        request: T,
    ) -> anyhow::Result<Result<R, JSONRPCError>> {
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
        let params_value = serde_json::to_value(request)
            .map_err(|e| anyhow::anyhow!("failed to serialize request params: {}", e))?;

        // If the params is already a JSON object or null for optional params, extract it
        let params = match params_value {
            Value::Object(obj) => Some(JsonrpcRequestParams {
                meta: None,
                extra: obj,
            }),
            Value::Null => None, // Allow null for optional params
            _ => return Err(RPCPortError::SerializeNotObject(params_value).into()),
        };

        // Create JSON-RPC request
        let rpc_request = JSONRPCRequest {
            id: request_id,
            jsonrpc: Default::default(),
            method: method.to_string(),
            params,
            extra: Default::default(),
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
                // Directly use the extra field as the Value object
                let result_value = Value::Object(response.result.extra);

                // Deserialize into the expected type
                let result = serde_json::from_value(result_value)
                    .map_err(|e| anyhow::anyhow!("failed to deserialize response: {}", e))?;

                Ok(Ok(result))
            }
            Err(error) => Ok(Err(error)),
        }
    }
}
