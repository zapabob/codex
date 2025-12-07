//! External MCP Tool
//!
//! MCP tool for interacting with external MCP servers

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Parameters for listing external MCP servers
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ListExternalServersParam {}

/// Parameters for getting server status
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetServerStatusParam {
    /// Name of the server to check
    pub server_name: String,
}

/// Parameters for starting an external server
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StartExternalServerParam {
    /// Name of the server to start
    pub server_name: String,
}

/// Parameters for stopping an external server
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StopExternalServerParam {
    /// Name of the server to stop
    pub server_name: String,
}

/// Parameters for sending request to external server
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SendExternalRequestParam {
    /// Name of the target server
    pub server_name: String,
    /// Method to call
    pub method: String,
    /// Parameters for the method call
    pub params: serde_json::Value,
}

/// Parameters for getting server configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GetServerConfigParam {
    /// Name of the server
    pub server_name: String,
}

/// Response for server list
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServerListResponse {
    /// List of available server names
    pub servers: Vec<String>,
}

/// Response for server status
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServerStatusResponse {
    /// Server name
    pub server_name: String,
    /// Server status
    pub status: String,
    /// Additional status information
    pub details: Option<String>,
}

/// Response for server configuration
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServerConfigResponse {
    /// Server name
    pub server_name: String,
    /// Server command
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env: std::collections::HashMap<String, String>,
}

/// Generic success response
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SuccessResponse {
    /// Success message
    pub message: String,
}

/// Generic error response
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

/// Create external MCP list servers tool
pub fn create_external_mcp_list_servers_tool() -> mcp_types::Tool {
    mcp_types::Tool {
        name: "external_mcp_list_servers".to_string(),
        description: Some("List all available external MCP servers configured in .cursor/mcp.json".to_string()),
        input_schema: schemars::schema_for!(ListExternalServersParam),
    }
}

/// Create external MCP get server status tool
pub fn create_external_mcp_get_server_status_tool() -> mcp_types::Tool {
    mcp_types::Tool {
        name: "external_mcp_get_server_status".to_string(),
        description: Some("Get the status of a specific external MCP server".to_string()),
        input_schema: schemars::schema_for!(GetServerStatusParam),
    }
}

/// Create external MCP start server tool
pub fn create_external_mcp_start_server_tool() -> mcp_types::Tool {
    mcp_types::Tool {
        name: "external_mcp_start_server".to_string(),
        description: Some("Start an external MCP server".to_string()),
        input_schema: schemars::schema_for!(StartExternalServerParam),
    }
}

/// Create external MCP stop server tool
pub fn create_external_mcp_stop_server_tool() -> mcp_types::Tool {
    mcp_types::Tool {
        name: "external_mcp_stop_server".to_string(),
        description: Some("Stop an external MCP server".to_string()),
        input_schema: schemars::schema_for!(StopExternalServerParam),
    }
}

/// Create external MCP send request tool
pub fn create_external_mcp_send_request_tool() -> mcp_types::Tool {
    mcp_types::Tool {
        name: "external_mcp_send_request".to_string(),
        description: Some("Send a request to an external MCP server".to_string()),
        input_schema: schemars::schema_for!(SendExternalRequestParam),
    }
}

/// Create external MCP get server config tool
pub fn create_external_mcp_get_server_config_tool() -> mcp_types::Tool {
    mcp_types::Tool {
        name: "external_mcp_get_server_config".to_string(),
        description: Some("Get configuration for an external MCP server".to_string()),
        input_schema: schemars::schema_for!(GetServerConfigParam),
    }
}
