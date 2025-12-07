//! External MCP Tool Handler
//!
//! Handles MCP tool calls for external MCP server management

use super::external_mcp_manager::{ExternalMcpManager, ServerStatus};
use super::external_mcp_tool::*;
use crate::message_processor::MessageProcessor;
use crate::outgoing_message::{OutgoingMessage, OutgoingMessageSender};
use mcp_types::ToolResponseContent;
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};

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
        name: &str,
        arguments: Value,
        sender: OutgoingMessageSender,
    ) -> Result<(), String> {
        match name {
            "external_mcp_list_servers" => {
                self.handle_list_servers(arguments, sender).await
            }
            "external_mcp_get_server_status" => {
                self.handle_get_server_status(arguments, sender).await
            }
            "external_mcp_start_server" => {
                self.handle_start_server(arguments, sender).await
            }
            "external_mcp_stop_server" => {
                self.handle_stop_server(arguments, sender).await
            }
            "external_mcp_send_request" => {
                self.handle_send_request(arguments, sender).await
            }
            "external_mcp_get_server_config" => {
                self.handle_get_server_config(arguments, sender).await
            }
            _ => Err(format!("Unknown external MCP tool: {}", name)),
        }
    }

    /// Handle list servers request
    async fn handle_list_servers(
        &self,
        _arguments: Value,
        sender: OutgoingMessageSender,
    ) -> Result<(), String> {
        // Validate parameters
        let _params: ListExternalServersParam = serde_json::from_value(_arguments)
            .map_err(|e| format!("Invalid parameters for list_servers: {}", e))?;

        // Get server list
        let servers = self.manager.list_servers().await;

        let response = ServerListResponse { servers };

        let content = ToolResponseContent::Text {
            text: serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize response".to_string()),
        };

        let message = OutgoingMessage::ToolResponse { content };
        sender.send(message).await
            .map_err(|e| format!("Failed to send response: {}", e))?;

        Ok(())
    }

    /// Handle get server status request
    async fn handle_get_server_status(
        &self,
        arguments: Value,
        sender: OutgoingMessageSender,
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

        let content = ToolResponseContent::Text {
            text: serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize response".to_string()),
        };

        let message = OutgoingMessage::ToolResponse { content };
        sender.send(message).await
            .map_err(|e| format!("Failed to send response: {}", e))?;

        Ok(())
    }

    /// Handle start server request
    async fn handle_start_server(
        &self,
        arguments: Value,
        sender: OutgoingMessageSender,
    ) -> Result<(), String> {
        // Validate parameters
        let params: StartExternalServerParam = serde_json::from_value(arguments)
            .map_err(|e| format!("Invalid parameters for start_server: {}", e))?;

        // Start server
        let result = self.manager.start_server(&params.server_name).await;

        let response = match result {
            Ok(()) => SuccessResponse {
                message: format!("Successfully started server: {}", params.server_name),
            },
            Err(e) => ErrorResponse {
                error: format!("Failed to start server '{}': {}", params.server_name, e),
            },
        };

        let content = ToolResponseContent::Text {
            text: serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize response".to_string()),
        };

        let message = OutgoingMessage::ToolResponse { content };
        sender.send(message).await
            .map_err(|e| format!("Failed to send response: {}", e))?;

        Ok(())
    }

    /// Handle stop server request
    async fn handle_stop_server(
        &self,
        arguments: Value,
        sender: OutgoingMessageSender,
    ) -> Result<(), String> {
        // Validate parameters
        let params: StopExternalServerParam = serde_json::from_value(arguments)
            .map_err(|e| format!("Invalid parameters for stop_server: {}", e))?;

        // Stop server
        let result = self.manager.stop_server(&params.server_name).await;

        let response = match result {
            Ok(()) => SuccessResponse {
                message: format!("Successfully stopped server: {}", params.server_name),
            },
            Err(e) => ErrorResponse {
                error: format!("Failed to stop server '{}': {}", params.server_name, e),
            },
        };

        let content = ToolResponseContent::Text {
            text: serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize response".to_string()),
        };

        let message = OutgoingMessage::ToolResponse { content };
        sender.send(message).await
            .map_err(|e| format!("Failed to send response: {}", e))?;

        Ok(())
    }

    /// Handle send request to external server
    async fn handle_send_request(
        &self,
        arguments: Value,
        sender: OutgoingMessageSender,
    ) -> Result<(), String> {
        // Validate parameters
        let params: SendExternalRequestParam = serde_json::from_value(arguments)
            .map_err(|e| format!("Invalid parameters for send_request: {}", e))?;

        // Send request to external server
        let result = self.manager.send_request(&params.server_name, arguments).await;

        let response = match result {
            Ok(response_data) => serde_json::to_string_pretty(&response_data)
                .unwrap_or_else(|_| "Success".to_string()),
            Err(e) => format!("Error: {}", e),
        };

        let content = ToolResponseContent::Text { text: response };

        let message = OutgoingMessage::ToolResponse { content };
        sender.send(message).await
            .map_err(|e| format!("Failed to send response: {}", e))?;

        Ok(())
    }

    /// Handle get server config request
    async fn handle_get_server_config(
        &self,
        arguments: Value,
        sender: OutgoingMessageSender,
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

        let content = ToolResponseContent::Text {
            text: serde_json::to_string_pretty(&response)
                .unwrap_or_else(|_| "Failed to serialize response".to_string()),
        };

        let message = OutgoingMessage::ToolResponse { content };
        sender.send(message).await
            .map_err(|e| format!("Failed to send response: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
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
