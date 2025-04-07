use futures::{StreamExt, TryStreamExt};
use mmcp_protocol::mcp::{JSONRPCMessage, RPCPort, RPCPortError};
use mmcp_server::MCPServer;
use serde::{Serialize, de::DeserializeOwned};
use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;

async fn start_server(mut server: MCPServer) -> anyhow::Result<()> {
    let (tx, rx) = futures::channel::mpsc::channel::<anyhow::Result<JSONRPCMessage>>(100);
    let stream = LinesStream::new(tokio::io::BufReader::new(tokio::io::stdin()).lines())
        .map_err(anyhow::Error::from)
        .and_then(async |line| serde_json::from_str::<JSONRPCMessage>(&line).map_err(From::from));
    let writer = tokio::io::BufWriter::new(tokio::io::stdout());

    // forward the stream to the channel and enforce tx close on stream close.
    tokio::spawn(stream.map(Ok).forward(tx.clone()));

    let mut adapter = StdioAdapter { tx, rx, writer };
    server.start(&mut adapter).await?;
    Ok(())
}

pub struct StdioAdapter {
    tx: futures::channel::mpsc::Sender<anyhow::Result<JSONRPCMessage>>,
    rx: futures::channel::mpsc::Receiver<anyhow::Result<JSONRPCMessage>>,
    writer: tokio::io::BufWriter<tokio::io::Stdout>,
}

impl RPCPort for StdioAdapter {
    async fn wait_for_request<T: DeserializeOwned + Send>(
        &mut self,
    ) -> anyhow::Result<mmcp_rpc::port::TypedRequest<T>> {
        while let Some(message) = self.rx.next().await {
            match message {
                Ok(message) => {
                    return Ok(message);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(RPCPortError::StreamClosed.into())
    }

    async fn wait_for_notification<T: DeserializeOwned + Send>(
        &mut self,
    ) -> anyhow::Result<mmcp::port::TypedNotification<T>> {
        todo!()
    }

    async fn respond<T: Serialize + Send>(
        &self,
        response: mmcp_rpc::port::TypedResponse<T>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn send_notification<T: Serialize + Send>(
        &self,
        notification: mmcp_rpc::port::TypedNotification<T>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn request<T: Serialize + Send, R: DeserializeOwned + Send>(
        &self,
        request: mmcp_rpc::port::TypedRequest<T>,
    ) -> anyhow::Result<
        Result<mmcp_rpc::port::TypedResponse<R>, mmcp_rpc::protocol::mcp::JSONRPCError>,
    > {
        todo!()
    }

    async fn receive_any_message(&mut self) -> anyhow::Result<mmcp::protocol::mcp::JSONRPCMessage> {
        if let Some(message) = self.rx.next().await {
            message
        } else {
            Err(RPCPortError::StreamClosed.into())
        }
    }

    async fn send_message(
        &self,
        message: mmcp_rpc::protocol::mcp::JSONRPCMessage,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
