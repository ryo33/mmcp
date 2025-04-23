use mmcp::{
    server::{MCPServer, primitives::tool::Text, stdio_server_rpc},
    tool,
};

#[tool]
/// Add two numbers
async fn add(x: i32, y: i32) -> Text<i32> {
    Text(x + y)
}

#[tool]
/// Subtract two numbers
async fn sub(x: i32, y: i32) -> Text<i32> {
    Text(x - y)
}

#[tokio::main]
async fn main() {
    MCPServer::new("mmcp-uuid", env!("CARGO_PKG_VERSION"))
        .with_tools_from_inventory()
        .start(stdio_server_rpc())
        .await
        .unwrap();
}
