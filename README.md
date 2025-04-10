# MMCP

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/mmcp-222222)](https://github.com/ryo33/mmcp)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/mmcp)](https://crates.io/crates/mmcp)
[![docs.rs](https://img.shields.io/docsrs/mmcp)](https://docs.rs/mmcp)
![GitHub Repo stars](https://img.shields.io/github/stars/ryo33/mmcp?style=social)

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
    MCPServer::new("mmcp-uuid", env!("CARGO_PKG_VERSION"))
        .with_tools_from_inventory()
        .start(stdio_server_rpc())
        .await
        .unwrap();
}
```

## Implementation Status

- [x] STDIO server implementation
- [x] Protocol definitions
- [x] Tool macros
- [x] RPC implementation
- [ ] HTTP transport with Axum
- [ ] Client implementation
- [ ] Authentication
- [ ] Logging
- [ ] Schemars 0.8 support
- [ ] Support more MCP functionalities

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
