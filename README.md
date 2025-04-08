# MMCP

A modular, minimalistic and macro-powered MCP (Model Context Protocol) framework for Rust.

## Examples

```toml
mmcp = { version = "0.1", features = ["server-stdio"] }
uuid = { version = "1", features = ["v4"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use mmcp::{
    server::{MCPServer, primitives::tool::Text, stdio_server_rpc},
    tool,
};
use uuid::Uuid;

/// Generate a UUID
#[tool]
fn generate_uuid() -> Text<Uuid> {
    Text(Uuid::new_v4())
}

/// Generate multiple UUIDs
#[tool]
fn generate_uuid_many(count: usize) -> Vec<Text<Uuid>> {
    (0..count).map(|_| generate_uuid()).collect()
}

#[tokio::main]
async fn main() {
    let server = MCPServer::new("mmcp-uuid", env!("CARGO_PKG_VERSION")).with_tools_from_inventory();

    let adapter = stdio_server_rpc();
    server.start(adapter).await.unwrap();
}
```

## Development

```json
{
	"mcpServers": {
		"mmcp-uuid": {
			"command": "cargo",
			"args": [
				"run",
				"--manifest-path",
				"/path/to/mmcp/Cargo.toml",
				"-p",
				"mmcp-uuid"
			]
		}
	}
}
```
