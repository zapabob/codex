//! External MCP Tool Handler
//!
//! Handles MCP tool calls for external MCP server management

use super::external_mcp_manager::ExternalMcpManager;
use super::external_mcp_tool::*;
use crate::outgoing_message::OutgoingMessageSender;
use mcp_types::{CallToolResult, ContentBlock, TextContent};
use rmcp::model::RequestId;
use serde_json::Value;
use std::sync::Arc;

/// Handler for external MCP tool calls
pub struct ExternalMcpToolHandler {
    manager: Arc<ExternalMcpManager>,
}

impl ExternalMcpToolHandler {
    /// Create new external MCP tool handler
    pub fn new(manager: Arc<ExternalMcpManager>) -> Self {
        Self { manager }
    }

    /// Handle external MCP tool calls
    pub async fn handle_tool_call(
        &self,
        id: RequestId,
        name: &str,
        arguments: Value,
        sender: Arc<OutgoingMessageSender>,
    ) -> Result<(), String> {
        match name {
            "external_mcp_list_servers" => self.handle_list_servers(id, arguments, sender).await,
            "external_mcp_get_server_status" => {
                self.handle_get_server_status(id, arguments, sender).await
            }
            "external_mcp_start_server" => self.handle_start_server(id, arguments, sender).await,
            "external_mcp_stop_server" => self.handle_stop_server(id, arguments, sender).await,
            "external_mcp_send_request" => self.handle_send_request(id, arguments, sender).await,
            "external_mcp_get_server_config" => {
                self.handle_get_server_config(id, arguments, sender).await
            }
            _ => Err(format!("Unknown external MCP tool: {}", name)),
        }
    }

    /// Handle list servers request
    async fn handle_list_servers(
        &self,
        id: RequestId,
        _arguments: Value,
        sender: Arc<OutgoingMessageSender>,
    ) -> Result<(), String> {
        // Validate parameters
        let _params: ListExternalServersParam = serde_json::from_value(_arguments)
            .map_err(|e| format!("Invalid parameters for list_servers: {}", e))?;

        // Get server list
        let servers = self.manager.list_servers().await;

        let response = ServerListResponse { servers };

        let content = vec![ContentBlock::TextContent(TextContent {
            r#type: "text".to_string(),
            text: serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize response".to_string()),
            annotations: None,
        })];

        let result = CallToolResult {
            content,
            is_error: Some(false),
            structured_content: None,
        };

        sender.send_response(id, result).await;

        Ok(())
    }

    /// Handle get server status request
    async fn handle_get_server_status(
        &self,
        id: RequestId,
        arguments: Value,
        sender: Arc<OutgoingMessageSender>,
    ) -> Result<(), String> {
        // Validate parameters
        let params: GetServerStatusParam = serde_json::from_value(arguments)
            .map_err(|e| format!("Invalid parameters for get_server_status: {}", e))?;

        // Get server status
        let status = self.manager.get_server_status(&params.server_name).await;

        let response = match status {
            Ok(status) => ServerStatusResponse {
                server_name: params.server_name,
                status: status.to_string(),
                details: None,
            },
            Err(e) => ServerStatusResponse {
                server_name: params.server_name,
                status: "error".to_string(),
                details: Some(e),
            },
        };

        let content = vec![ContentBlock::TextContent(TextContent {
            r#type: "text".to_string(),
            text: serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize response".to_string()),
            annotations: None,
        })];

        let result = CallToolResult {
            content,
            is_error: Some(false),
            structured_content: None,
        };

        sender.send_response(id, result).await;

        Ok(())
    }

    /// Handle start server request
    async fn handle_start_server(
        &self,
        id: RequestId,
        arguments: Value,
        sender: Arc<OutgoingMessageSender>,
    ) -> Result<(), String> {
        // Validate parameters
        let params: StartExternalServerParam = serde_json::from_value(arguments)
            .map_err(|e| format!("Invalid parameters for start_server: {}", e))?;

        // Start server
        let result = self.manager.start_server(&params.server_name).await;
        let is_error = result.is_err();

        let response_json = match result {
            Ok(()) => serde_json::to_string_pretty(&SuccessResponse {
                message: format!("Successfully started server: {}", params.server_name),
            }),
            Err(e) => serde_json::to_string_pretty(&ErrorResponse {
                error: format!("Failed to start server '{}': {}", params.server_name, e),
            }),
        }
        .unwrap_or_else(|_| "Failed to serialize response".to_string());

        let content = vec![ContentBlock::TextContent(TextContent {
            r#type: "text".to_string(),
            text: response_json,
            annotations: None,
        })];

        let call_result = CallToolResult {
            content,
            is_error: Some(is_error),
            structured_content: None,
        };

        sender.send_response(id, call_result).await;

        Ok(())
    }

    /// Handle stop server request
    async fn handle_stop_server(
        &self,
        id: RequestId,
        arguments: Value,
        sender: Arc<OutgoingMessageSender>,
    ) -> Result<(), String> {
        // Validate parameters
        let params: StopExternalServerParam = serde_json::from_value(arguments)
            .map_err(|e| format!("Invalid parameters for stop_server: {}", e))?;

        // Stop server
        let result = self.manager.stop_server(&params.server_name).await;
        let is_error = result.is_err();

        let response_json = match result {
            Ok(()) => serde_json::to_string_pretty(&SuccessResponse {
                message: format!("Successfully stopped server: {}", params.server_name),
            }),
            Err(e) => serde_json::to_string_pretty(&ErrorResponse {
                error: format!("Failed to stop server '{}': {}", params.server_name, e),
            }),
        }
        .unwrap_or_else(|_| "Failed to serialize response".to_string());

        let content = vec![ContentBlock::TextContent(TextContent {
            r#type: "text".to_string(),
            text: response_json,
            annotations: None,
        })];

        let call_result = CallToolResult {
            content,
            is_error: Some(is_error),
            structured_content: None,
        };

        sender.send_response(id, call_result).await;

        Ok(())
    }

    /// Handle send request to external server
    async fn handle_send_request(
        &self,
        id: RequestId,
        arguments: Value,
        sender: Arc<OutgoingMessageSender>,
    ) -> Result<(), String> {
        // Validate parameters
        let params: SendExternalRequestParam = serde_json::from_value(arguments.clone())
            .map_err(|e| format!("Invalid parameters for send_request: {}", e))?;

        // Send request to external server
        let result = self
            .manager
            .send_request(&params.server_name, arguments)
            .await;

        let response = match result {
            Ok(response_data) => serde_json::to_string_pretty(&response_data)
                .unwrap_or_else(|_| "Success".to_string()),
            Err(e) => format!("Error: {}", e),
        };

        let content = vec![ContentBlock::TextContent(TextContent {
            r#type: "text".to_string(),
            text: response,
            annotations: None,
        })];

        let call_result = CallToolResult {
            content,
            is_error: None,
            structured_content: None,
        };

        sender.send_response(id, call_result).await;

        Ok(())
    }

    /// Handle get server config request
    async fn handle_get_server_config(
        &self,
        id: RequestId,
        arguments: Value,
        sender: Arc<OutgoingMessageSender>,
    ) -> Result<(), String> {
        // Validate parameters
        let params: GetServerConfigParam = serde_json::from_value(arguments)
            .map_err(|e| format!("Invalid parameters for get_server_config: {}", e))?;

        // Get server config
        let config = self.manager.get_server_config(&params.server_name).await;

        let response = match config {
            Ok(config) => ServerConfigResponse {
                server_name: params.server_name,
                command: config.command,
                args: config.args,
                env: config.env,
            },
            Err(e) => return Err(format!("Failed to get server config: {}", e)),
        };

        let content = vec![ContentBlock::TextContent(TextContent {
            r#type: "text".to_string(),
            text: serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize response".to_string()),
            annotations: None,
        })];

        let result = CallToolResult {
            content,
            is_error: Some(false),
            structured_content: None,
        };

        sender.send_response(id, result).await;

        Ok(())
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_tool_handler_creation() {
        let manager = Arc::new(ExternalMcpManager::new(None));
        let handler = ExternalMcpToolHandler::new(manager);

        // Handler should be created successfully
        assert!(true); // If we reach here, creation was successful
    }
}
