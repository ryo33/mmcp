use futures::{StreamExt, TryStreamExt};
use mmcp::{
    port::{RPCPort, RPCPortError},
    protocol::mcp::JSONRPCMessage,
    serde_json,
    server::MCPServer,
};
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
    async fn wait_for_request<T: mmcp::serde::de::DeserializeOwned + Send>(
        &mut self,
    ) -> anyhow::Result<mmcp::port::TypedRequest<T>> {
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

    async fn wait_for_notification<T: mmcp::serde::de::DeserializeOwned + Send>(
        &mut self,
    ) -> anyhow::Result<mmcp::port::TypedNotification<T>> {
        todo!()
    }

    async fn send_response<T: mmcp::serde::Serialize + Send>(
        &self,
        response: mmcp::port::TypedResponse<T>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn send_notification<T: mmcp::serde::Serialize + Send>(
        &self,
        notification: mmcp::port::TypedNotification<T>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn request<
        T: mmcp::serde::Serialize + Send,
        R: mmcp::serde::de::DeserializeOwned + Send,
    >(
        &self,
        request: mmcp::port::TypedRequest<T>,
    ) -> anyhow::Result<Result<mmcp::port::TypedResponse<R>, mmcp::protocol::mcp::JSONRPCError>>
    {
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
        message: mmcp::protocol::mcp::JSONRPCMessage,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
