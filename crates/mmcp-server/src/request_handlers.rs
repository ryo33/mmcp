use mmcp_protocol::{
    consts::error_codes,
    mcp::{
        self, CallToolRequest, ClientRequest, CompleteRequest, GetPromptRequest, JSONRPCError,
        JSONRPCRequest, JSONRPCResponse, JsonrpcBatchResponseItem, JsonrpcErrorError,
        ListPromptsRequest, ListResourcesRequest, ListToolsRequest, PingRequest,
        ReadResourceRequest, RequestId, SetLevelRequest, SubscribeRequest, UnsubscribeRequest,
    },
};

use crate::{MCPServer, serialize_tool_call_result};

impl MCPServer {
    pub async fn handle_request(
        &self,
        request: JSONRPCRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let request_id = request.id;
        let client_request = serde_json::from_value::<ClientRequest>(serde_json::json!({
            "method": request.method,
            "params": request.params,
        }))?;

        match client_request {
            ClientRequest::InitializeRequest(_) => Err(anyhow::anyhow!(
                "Unexpected initialize request after initialization"
            )),
            ClientRequest::PingRequest(ping_request) => {
                self.handle_ping_request(request_id, ping_request).await
            }
            ClientRequest::ListResourcesRequest(list_resources_request) => {
                self.handle_list_resources_request(request_id, list_resources_request)
                    .await
            }
            ClientRequest::ReadResourceRequest(read_resource_request) => {
                self.handle_read_resource_request(request_id, read_resource_request)
                    .await
            }
            ClientRequest::SubscribeRequest(subscribe_request) => {
                self.handle_subscribe_request(request_id, subscribe_request)
                    .await
            }
            ClientRequest::UnsubscribeRequest(unsubscribe_request) => {
                self.handle_unsubscribe_request(request_id, unsubscribe_request)
                    .await
            }
            ClientRequest::ListPromptsRequest(list_prompts_request) => {
                self.handle_list_prompts_request(request_id, list_prompts_request)
                    .await
            }
            ClientRequest::GetPromptRequest(get_prompt_request) => {
                self.handle_get_prompt_request(request_id, get_prompt_request)
                    .await
            }
            ClientRequest::ListToolsRequest(list_tools_request) => {
                self.handle_list_tools_request(request_id, list_tools_request)
                    .await
            }
            ClientRequest::CallToolRequest(call_tool_request) => {
                self.handle_call_tool_request(request_id, call_tool_request)
                    .await
            }
            ClientRequest::SetLevelRequest(set_level_request) => {
                self.handle_set_level_request(request_id, set_level_request)
                    .await
            }
            ClientRequest::CompleteRequest(complete_request) => {
                self.handle_complete_request(request_id, complete_request)
                    .await
            }
        }
    }

    async fn handle_ping_request(
        &self,
        request_id: RequestId,
        _request: PingRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let extra = serde_json::json!({
            "message": "pong"
        })
        .as_object()
        .unwrap()
        .clone();

        Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
            id: request_id,
            jsonrpc: Default::default(),
            result: mcp::Result {
                meta: Default::default(),
                extra,
            },
            extra: Default::default(),
        }))
    }

    async fn handle_list_resources_request(
        &self,
        request_id: RequestId,
        _request: ListResourcesRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let resources = self
            .list_resources()
            .map(|resource| Ok(serde_json::to_value(resource)?))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
            id: request_id,
            jsonrpc: Default::default(),
            result: mcp::Result {
                meta: Default::default(),
                extra: serde_json::json!({"resources": resources})
                    .as_object()
                    .unwrap()
                    .clone(),
            },
            extra: Default::default(),
        }))
    }

    async fn handle_read_resource_request(
        &self,
        _request_id: RequestId,
        _request: ReadResourceRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_subscribe_request(
        &self,
        _request_id: RequestId,
        _request: SubscribeRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_unsubscribe_request(
        &self,
        _request_id: RequestId,
        _request: UnsubscribeRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_list_prompts_request(
        &self,
        request_id: RequestId,
        _request: ListPromptsRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let prompts = self
            .list_prompts()
            .map(|prompt| Ok(serde_json::to_value(prompt)?))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
            id: request_id,
            jsonrpc: Default::default(),
            result: mcp::Result {
                meta: Default::default(),
                extra: serde_json::json!({"prompts": prompts})
                    .as_object()
                    .unwrap()
                    .clone(),
            },
            extra: Default::default(),
        }))
    }

    async fn handle_get_prompt_request(
        &self,
        _request_id: RequestId,
        _request: GetPromptRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_list_tools_request(
        &self,
        request_id: RequestId,
        _request: ListToolsRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let tools = self
            .list_tools()
            .map(|tool| {
                Ok(serde_json::json!({
                    "name": tool.name(),
                    "description": tool.description(),
                    "inputSchema": serde_json::from_str::<serde_json::Value>(tool.input_schema().as_ref())?,
                    "annotations": tool.annotations()
                }))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
            id: request_id,
            jsonrpc: Default::default(),
            result: mcp::Result {
                meta: Default::default(),
                extra: serde_json::json!({"tools": tools})
                    .as_object()
                    .unwrap()
                    .clone(),
            },
            extra: Default::default(),
        }))
    }

    async fn handle_call_tool_request(
        &self,
        request_id: RequestId,
        request: CallToolRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        let tool_name = request.params.name.as_str();

        if let Some(tool) = self.get_tool(tool_name) {
            let result = tool.execute(request).await;
            Ok(JsonrpcBatchResponseItem::JSONRPCResponse(JSONRPCResponse {
                id: request_id,
                jsonrpc: Default::default(),
                result: serialize_tool_call_result(result)?,
                extra: Default::default(),
            }))
        } else {
            Ok(JsonrpcBatchResponseItem::JSONRPCError(JSONRPCError {
                error: JsonrpcErrorError {
                    message: format!("Tool not found: {}", tool_name),
                    code: error_codes::INVALID_PARAMS,
                    data: None,
                    extra: Default::default(),
                },
                id: request_id,
                jsonrpc: Default::default(),
                extra: Default::default(),
            }))
        }
    }

    async fn handle_set_level_request(
        &self,
        _request_id: RequestId,
        _request: SetLevelRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }

    async fn handle_complete_request(
        &self,
        _request_id: RequestId,
        _request: CompleteRequest,
    ) -> anyhow::Result<JsonrpcBatchResponseItem> {
        todo!()
    }
}
