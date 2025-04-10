# MMCP Server

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/mmcp-222222)](https://github.com/ryo33/mmcp)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/mmcp-server)](https://crates.io/crates/mmcp-server)
[![docs.rs](https://img.shields.io/docsrs/mmcp-server)](https://docs.rs/mmcp-server)
![GitHub Repo stars](https://img.shields.io/github/stars/ryo33/mmcp?style=social)

Server implementation for the MMCP framework.

This crate provides the core server implementation for the MMCP framework, including the `MCPServer` type and related functionality.

## Example

```rust
use mmcp::server::{MCPServer, stdio_server_rpc};

#[tokio::main]
async fn main() {
    let server = MCPServer::new("my-server", "1.0.0").with_tools_from_inventory();

    let adapter = stdio_server_rpc();
    server.start(adapter).await.unwrap();
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
