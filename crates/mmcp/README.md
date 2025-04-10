# MMCP

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/mmcp-222222)](https://github.com/ryo33/mmcp)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/mmcp)](https://crates.io/crates/mmcp)
[![docs.rs](https://img.shields.io/docsrs/mmcp)](https://docs.rs/mmcp)
![GitHub Repo stars](https://img.shields.io/github/stars/ryo33/mmcp?style=social)

A modular, minimalistic and macro-powered MCP (Model Context Protocol) framework for Rust.

This is the main crate for the MMCP framework, providing core functionality and interfaces.

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

## Example

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

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
