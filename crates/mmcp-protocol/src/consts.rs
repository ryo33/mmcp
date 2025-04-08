/// Current protocol version as defined in the Model Context Protocol (MCP) specification
pub const PROTOCOL_VERSION: &str = "2025-03-26";

/// JSON-RPC standard error codes
pub mod error_codes {
    // Standard JSON-RPC 2.0 error codes
    /// Invalid JSON was received by the server.
    pub const PARSE_ERROR: i64 = -32700;
    /// The JSON sent is not a valid Request object.
    pub const INVALID_REQUEST: i64 = -32600;
    /// The method does not exist / is not available.
    pub const METHOD_NOT_FOUND: i64 = -32601;
    /// Invalid method parameter(s).
    pub const INVALID_PARAMS: i64 = -32602;
    /// Internal JSON-RPC error.
    pub const INTERNAL_ERROR: i64 = -32603;
}
