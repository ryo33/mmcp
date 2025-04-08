pub mod schemars;

#[cfg(feature = "macros")]
pub use mmcp_macros::tool;

#[cfg(feature = "macros")]
pub use serde;
#[cfg(feature = "macros")]
pub use serde_json;

#[cfg(feature = "server")]
pub mod server {
    pub use mmcp_server::*;

    #[cfg(feature = "server-stdio")]
    pub use mmcp_server_stdio::stdio_server_rpc;
}

pub use mmcp_protocol as protocol;
