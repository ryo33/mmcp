use mmcp::{primitives::tool::Text, server::MCPServer, tool};
use uuid::Uuid;

#[tool]
fn generate_uuid() -> Text<Uuid> {
    Text(Uuid::new_v4())
}

#[tool]
fn generate_uuid_many(count: usize) -> Vec<Text<Uuid>> {
    (0..count).map(|_| generate_uuid()).collect()
}

fn main() {
    let server = MCPServer::new("mmcp-uuid", env!("CARGO_PKG_VERSION")).with_tools_from_inventory();

    for tool in server.list_tools() {
        println!("Tool: {}", tool.name());
    }
}
