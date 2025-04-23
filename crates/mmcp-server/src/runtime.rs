pub mod notification_handlers;
pub mod request_handlers;

use crate::MCPServer;
use mmcp_protocol::{
    mcp::{
        JSONRPCBatchResponse, JSONRPCMessage, JsonrpcBatchRequestItem, JsonrpcBatchResponseItem,
    },
    port::{RPCPort, RPCSink},
};
use std::sync::Arc;
use tokio::spawn;

/// Runtime for handling messages concurrently after initialization
#[derive(Clone)]
pub struct MCPServerRuntime<S: RPCSink + Clone + Send + 'static> {
    server: Arc<MCPServer>,
    sink: S,
}

impl<S: RPCSink + Clone + Send + 'static> MCPServerRuntime<S> {
    /// Construct runtime from server and sink
    pub fn new(server: MCPServer, sink: S) -> Self {
        MCPServerRuntime {
            server: Arc::new(server),
            sink,
        }
    }

    /// Run the server: initialization + message loop
    pub async fn run<P: RPCPort>(self, mut port: P) -> anyhow::Result<()> {
        // Initialization phase
        let mut sink = self.sink.clone();
        let queued = self.server.initialize(&mut port, &mut sink).await?;
        // Handle any queued messages
        for msg in queued {
            self.dispatch(msg).await?;
        }
        // Main loop: process messages as they arrive
        while let Ok(Some(msg)) = port.progress().await {
            self.dispatch(msg).await?;
        }
        Ok(())
    }

    async fn dispatch(&self, message: JSONRPCMessage) -> anyhow::Result<()> {
        match message {
            JSONRPCMessage::JSONRPCRequest(request) => {
                let server = Arc::clone(&self.server);
                let mut sink = self.sink.clone();
                spawn(async move {
                    let msg = match server.handle_request(request).await {
                        Ok(JsonrpcBatchResponseItem::JSONRPCResponse(resp)) => {
                            JSONRPCMessage::JSONRPCResponse(resp)
                        }
                        Ok(JsonrpcBatchResponseItem::JSONRPCError(err)) => {
                            JSONRPCMessage::JSONRPCError(err)
                        }
                        Err(e) => {
                            eprintln!("Request handler error: {e}");
                            return;
                        }
                    };
                    let _ = sink.send_message(msg).await;
                });
            }
            JSONRPCMessage::JSONRPCNotification(notification) => {
                let server = Arc::clone(&self.server);
                spawn(async move {
                    let _ = server.handle_notification(notification).await;
                });
            }
            JSONRPCMessage::JSONRPCBatchRequest(batch) => {
                let server = Arc::clone(&self.server);
                let mut sink = self.sink.clone();
                spawn(async move {
                    let mut handles = Vec::new();
                    for item in batch.0 {
                        if let JsonrpcBatchRequestItem::JSONRPCRequest(req) = item {
                            let server = Arc::clone(&server);
                            handles.push(spawn(async move { server.handle_request(req).await }));
                        }
                    }
                    let mut responses = Vec::new();
                    for handle in handles {
                        if let Ok(Ok(resp)) = handle.await {
                            responses.push(resp);
                        }
                    }
                    let _ = sink
                        .send_message(JSONRPCMessage::JSONRPCBatchResponse(JSONRPCBatchResponse(
                            responses,
                        )))
                        .await;
                });
            }
            _ => {}
        }
        Ok(())
    }
}
