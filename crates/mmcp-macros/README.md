# MMCP Macros

[![GitHub](https://img.shields.io/badge/GitHub-ryo33/mmcp-222222)](https://github.com/ryo33/mmcp)
![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/mmcp-macros)](https://crates.io/crates/mmcp-macros)
[![docs.rs](https://img.shields.io/docsrs/mmcp-macros)](https://docs.rs/mmcp-macros)
![GitHub Repo stars](https://img.shields.io/github/stars/ryo33/mmcp?style=social)

Procedural macros for the MMCP framework.

This crate provides procedural macros like `#[tool]` to make using the MMCP framework more ergonomic.

## Example

```rust
use mmcp::{tool, server::primitives::tool::Text};
use uuid::Uuid;

/// Generate a UUID
#[tool]
fn generate_uuid() -> Text<Uuid> {
    Text(Uuid::new_v4())
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
