use futures::{StreamExt, TryStreamExt};
use mmcp_protocol::{mcp::JSONRPCMessage, port::RPCPort};
use mmcp_rpc::RPCRuntime;
use std::io::Write;
use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;

pub fn stdio_server_rpc() -> impl RPCPort {
    let (tx, mut rx) = futures::channel::mpsc::channel::<JSONRPCMessage>(100);
    let stream = LinesStream::new(tokio::io::BufReader::new(tokio::io::stdin()).lines())
        .map_err(anyhow::Error::from)
        .map_ok(|line| {
            Box::pin(async move {
                match serde_json::from_str::<JSONRPCMessage>(&line) {
                    Ok(message) => Some(message),
                    Err(e) => {
                        eprintln!("Error parsing JSON: {}: {}", e, line);
                        None
                    }
                }
            })
        })
        .try_filter_map(|message| Box::pin(async move { Ok(message.await) }));
    let mut writer = std::io::BufWriter::new(std::io::stdout());

    let rpc = RPCRuntime::new(tx, stream);

    // forward the stream to the channel and enforce tx close on stream close.
    tokio::spawn(async move {
        while let Some(message) = rx.next().await {
            let json = serde_json::to_string(&message).unwrap();
            writeln!(&mut writer, "{}", json).unwrap();
            writer.flush().unwrap();
        }
    });

    rpc
}
