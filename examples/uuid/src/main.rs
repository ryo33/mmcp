use std::time::Duration;

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

/// Generate a UUID in async fn
#[tool]
async fn generate_uuid_in_async() -> String {
    tokio::time::sleep(Duration::from_secs(1)).await;
    Uuid::new_v4().to_string()
}

#[tokio::main]
async fn main() {
    MCPServer::new("mmcp-uuid", env!("CARGO_PKG_VERSION"))
        .with_tools_from_inventory()
        .start(stdio_server_rpc())
        .await
        .unwrap();
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde_json::{Value, json};
    use std::io::{BufRead, BufReader, Write};
    use std::process::{Child, Command, Stdio};

    fn run_server() -> Child {
        let mut cmd = Command::new("cargo");
        cmd.args(["run", "--bin", "mmcp-uuid"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
    }

    /// Performs MCP initialization and returns the stdin/stdout handles
    /// along with the server info from the initialization response
    fn initialize_mcp(
        child: &mut Child,
    ) -> (
        std::process::ChildStdin,
        BufReader<std::process::ChildStdout>,
        Value,
    ) {
        let mut stdin = child.stdin.take().unwrap();
        let mut stdout = BufReader::new(child.stdout.take().unwrap());

        // Initialize the MCP connection - this must be the first exchange
        let initialize_request = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "capabilities": {
                    "tools": {
                        "execute": true
                    }
                },
                "clientInfo": {
                    "name": "test-client",
                    "version": "0.1.0"
                },
                "protocolVersion": "2025-03-26"
            }
        });

        // Send initialize request
        writeln!(stdin, "{}", initialize_request).unwrap();

        // Read initialize response
        let mut response_str = String::new();
        stdout.read_line(&mut response_str).unwrap();
        let response: Value = serde_json::from_str(&response_str).unwrap();

        // Verify initialize response using a single assertion
        // Note: We only verify the critical fields, as some like "protocolVersion" may change
        assert_eq!(
            json!({
                "id": 1,
                "jsonrpc": "2.0",
                "result": {
                    "capabilities": {
                        "prompts": {
                            "listChanged": true
                        },
                        "resources": {
                            "listChanged": true,
                            "subscribe": false,
                        },
                        "tools": {
                            "listChanged": true,
                        }
                    },
                    "protocolVersion": "2025-03-26",
                    "serverInfo": {
                        "name": "mmcp-uuid",
                        "version": env!("CARGO_PKG_VERSION")
                    },
                }
            }),
            response,
            "Initialize response should match expected format"
        );

        // Send initialized notification (required by the protocol to complete initialization)
        let initialized_notification = json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });

        writeln!(stdin, "{}", initialized_notification).unwrap();

        // Return stdin/stdout and server info for further communication
        (stdin, stdout, response["result"].clone())
    }

    #[test]
    fn test_tools_list() {
        // Start the MCP server
        let mut child = run_server();

        // Initialize MCP protocol
        let (mut stdin, mut stdout, _) = initialize_mcp(&mut child);

        // Request tools list
        let list_request = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "method": "tools/list"
        });

        writeln!(stdin, "{}", list_request).unwrap();

        // Read list response
        let mut response_str = String::new();
        stdout.read_line(&mut response_str).unwrap();
        let response: Value = serde_json::from_str(&response_str).unwrap();

        // Verify the response in a single assertion
        assert_eq!(
            json!({
                "id": 2,
                "jsonrpc": "2.0",
                "result": {
                    "tools": [
                        {
                            "name": "generate_uuid",
                            "description": "Generate a UUID",
                            "inputSchema": {"type": "object"},
                            "annotations": {},
                        },
                        {
                            "name": "generate_uuid_in_async",
                            "description": "Generate a UUID in async fn",
                            "inputSchema": {"type": "object"},
                            "annotations": {},
                        },
                        {
                            "name": "generate_uuid_many",
                            "description": "Generate multiple UUIDs",
                            "inputSchema": {
                                "$schema": "https://json-schema.org/draft/2020-12/schema",
                                "title": "GenerateUuidManyInputSchema",
                                "type": "object",
                                "properties": {
                                    "count": {
                                        "type": "integer",
                                        "format": "uint",
                                        "minimum": 0,
                                    },
                                },
                                "required": ["count"]
                            },
                            "annotations": {},
                        }
                    ]
                }
            }),
            response,
            "Response should match expected JSON-RPC format"
        );

        // After verifying structure, check tool names separately
        let tool_names: Vec<&str> = response["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();

        assert!(
            tool_names.contains(&"generate_uuid"),
            "Should have generate_uuid tool"
        );
        assert!(
            tool_names.contains(&"generate_uuid_many"),
            "Should have generate_uuid_many tool"
        );

        // Clean up
        child.kill().unwrap();
    }

    #[test]
    fn test_generate_uuid() {
        // Start the MCP server
        let mut child = run_server();

        // Initialize MCP protocol
        let (mut stdin, mut stdout, _) = initialize_mcp(&mut child);

        // Execute the tool
        let execute_request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": "generate_uuid"
            }
        });

        // Send execute request
        writeln!(stdin, "{}", execute_request).unwrap();

        // Read execute response
        let mut response_str = String::new();
        stdout.read_line(&mut response_str).unwrap();

        // Parse response
        let response: Value = serde_json::from_str(&response_str).unwrap();

        // Verify the response using a single assertion for the structure
        assert_eq!(
            json!({
                "id": 2,
                "jsonrpc": "2.0",
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": response["result"]["content"][0]["text"],
                        }
                    ]
                }
            }),
            response,
            "Response should have correct structure"
        );

        // Clean up
        child.kill().unwrap();
    }

    #[test]
    fn test_generate_uuid_many() {
        // Start the MCP server
        let mut child = run_server();

        // Initialize MCP protocol
        let (mut stdin, mut stdout, _) = initialize_mcp(&mut child);

        // Execute the tool
        let execute_request = json!({
            "id": 2,
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": "generate_uuid_many",
                "arguments": {
                    "count": 3
                }
            }
        });

        // Send execute request
        writeln!(stdin, "{}", execute_request).unwrap();

        // Read execute response
        let mut response_str = String::new();
        stdout.read_line(&mut response_str).unwrap();

        // Parse response
        let response: Value = serde_json::from_str(&response_str).unwrap();

        // First, validate the response structure with a single assertion
        assert_eq!(
            json!({
                "id": 2,
                "jsonrpc": "2.0",
                "result": {
                    "content": [
                        {"type": "text", "text": response["result"]["content"][0]["text"]},
                        {"type": "text", "text": response["result"]["content"][1]["text"]},
                        {"type": "text", "text": response["result"]["content"][2]["text"]}
                    ]
                }
            }),
            response,
            "Response should have correct structure with 3 text items"
        );

        // Clean up
        child.kill().unwrap();
    }

    #[test]
    fn test_generate_uuid_in_async() {
        // Start the MCP server
        let mut child = run_server();

        // Initialize MCP protocol
        let (mut stdin, mut stdout, _) = initialize_mcp(&mut child);

        // Execute the tool
        let execute_request = json!({
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/call",
            "params": {
                "name": "generate_uuid_in_async"
            }
        });

        // Send execute request
        writeln!(stdin, "{}", execute_request).unwrap();

        // Read execute response
        let mut response_str = String::new();
        stdout.read_line(&mut response_str).unwrap();

        // Parse response
        let response: Value = serde_json::from_str(&response_str).unwrap();

        // Verify the response structure
        assert_eq!(
            json!({
                "id": 2,
                "jsonrpc": "2.0",
                "result": {
                    "content": [
                        {
                            "type": "text",
                            "text": response["result"]["content"][0]["text"],
                        }
                    ]
                }
            }),
            response,
            "Response should have correct structure"
        );

        // Clean up
        child.kill().unwrap();
    }
}
