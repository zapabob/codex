//! LSP tool handler for MCP server
//!
//! Provides LSP diagnostics as MCP tools

use anyhow::{Context, Result};
use codex_core::lsp::{DiagnosticsManager, LspClient};
use mcp_types::{
    CallToolRequestParams, CallToolResult, ListToolsResult, Tool, Url,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// LSP tool handler
pub struct LspToolHandler {
    /// Active LSP clients
    clients: Arc<RwLock<HashMap<String, LspClient>>>,
    /// Diagnostics manager
    diagnostics_manager: Arc<DiagnosticsManager>,
}

impl LspToolHandler {
    /// Create a new LSP tool handler
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            diagnostics_manager: Arc::new(DiagnosticsManager::new(100)),
        }
    }

    /// List available LSP tools
    pub fn list_tools(&self) -> ListToolsResult {
        ListToolsResult {
            tools: vec![
                Tool {
                    name: "lsp_get_diagnostics".to_string(),
                    description: "Get LSP diagnostics for a document or all documents".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "uri": {
                                "type": "string",
                                "description": "Document URI (optional, if not provided returns all diagnostics)"
                            }
                        }
                    }),
                },
                Tool {
                    name: "lsp_start_server".to_string(),
                    description: "Start an LSP server for a language".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "server_name": {
                                "type": "string",
                                "description": "Name of the language server (e.g., 'rust-analyzer', 'typescript')"
                            },
                            "command": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Command to start the language server"
                            },
                            "root_path": {
                                "type": "string",
                                "description": "Root path of the workspace"
                            }
                        },
                        "required": ["server_name", "command", "root_path"]
                    }),
                },
                Tool {
                    name: "lsp_stop_server".to_string(),
                    description: "Stop an LSP server".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "server_name": {
                                "type": "string",
                                "description": "Name of the language server to stop"
                            }
                        },
                        "required": ["server_name"]
                    }),
                },
                Tool {
                    name: "lsp_get_completions".to_string(),
                    description: "Get code completions at a position".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "server_name": {
                                "type": "string",
                                "description": "Name of the language server"
                            },
                            "uri": {
                                "type": "string",
                                "description": "Document URI"
                            },
                            "line": {
                                "type": "number",
                                "description": "Line number (0-based)"
                            },
                            "character": {
                                "type": "number",
                                "description": "Character position (0-based)"
                            }
                        },
                        "required": ["server_name", "uri", "line", "character"]
                    }),
                },
                Tool {
                    name: "lsp_get_hover".to_string(),
                    description: "Get hover information at a position".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "server_name": {
                                "type": "string",
                                "description": "Name of the language server"
                            },
                            "uri": {
                                "type": "string",
                                "description": "Document URI"
                            },
                            "line": {
                                "type": "number",
                                "description": "Line number (0-based)"
                            },
                            "character": {
                                "type": "number",
                                "description": "Character position (0-based)"
                            }
                        },
                        "required": ["server_name", "uri", "line", "character"]
                    }),
                },
                Tool {
                    name: "lsp_get_statistics".to_string(),
                    description: "Get LSP diagnostics statistics".to_string(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {}
                    }),
                },
            ],
        }
    }

    /// Handle a tool call
    pub async fn handle_tool_call(&self, tool_call: CallToolRequestParams) -> Result<CallToolResult> {
        match tool_call.name.as_str() {
            "lsp_get_diagnostics" => self.handle_get_diagnostics(tool_call.arguments).await,
            "lsp_start_server" => self.handle_start_server(tool_call.arguments).await,
            "lsp_stop_server" => self.handle_stop_server(tool_call.arguments).await,
            "lsp_get_completions" => self.handle_get_completions(tool_call.arguments).await,
            "lsp_get_hover" => self.handle_get_hover(tool_call.arguments).await,
            "lsp_get_statistics" => self.handle_get_statistics(tool_call.arguments).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_call.name)),
        }
    }

    async fn handle_get_diagnostics(
        &self,
        arguments: serde_json::Value,
    ) -> Result<CallToolResult> {
        let uri: Option<String> = arguments.get("uri").and_then(|v| v.as_str()).map(|s| s.to_string());

        if let Some(uri_str) = uri {
            let uri = Url::parse(&uri_str).context("Invalid URI")?;
            let diagnostics = self.diagnostics_manager.get_combined_diagnostics(&uri).await;

            Ok(CallToolResult {
                content: vec![CallToolResult::Text {
                    text: serde_json::to_string_pretty(&diagnostics)?,
                }],
                is_error: false,
            })
        } else {
            let all_diagnostics = self.diagnostics_manager.get_all_diagnostics().await;
            Ok(CallToolResult {
                content: vec![CallToolResult::Text {
                    text: serde_json::to_string_pretty(&all_diagnostics)?,
                }],
                is_error: false,
            })
        }
    }

    async fn handle_start_server(
        &self,
        arguments: serde_json::Value,
    ) -> Result<CallToolResult> {
        let server_name = arguments
            .get("server_name")
            .and_then(|v| v.as_str())
            .context("Missing server_name")?
            .to_string();

        let command: Vec<String> = arguments
            .get("command")
            .and_then(|v| v.as_array())
            .context("Missing command")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();

        let root_path = arguments
            .get("root_path")
            .and_then(|v| v.as_str())
            .context("Missing root_path")?
            .to_string();

        let root_path = PathBuf::from(root_path);
        let mut client = LspClient::new(server_name.clone(), command.clone(), root_path.clone());
        client.start(command).await.context("Failed to start LSP server")?;

        let mut clients = self.clients.write().await;
        clients.insert(server_name.clone(), client);

        info!("Started LSP server: {}", server_name);

        Ok(CallToolResult {
            content: vec![CallToolResult::Text {
                text: format!("Started LSP server: {}", server_name),
            }],
            is_error: false,
        })
    }

    async fn handle_stop_server(
        &self,
        arguments: serde_json::Value,
    ) -> Result<CallToolResult> {
        let server_name = arguments
            .get("server_name")
            .and_then(|v| v.as_str())
            .context("Missing server_name")?
            .to_string();

        let mut clients = self.clients.write().await;
        if let Some(mut client) = clients.remove(&server_name) {
            client.stop().await.context("Failed to stop LSP server")?;
            info!("Stopped LSP server: {}", server_name);
            Ok(CallToolResult {
                content: vec![CallToolResult::Text {
                    text: format!("Stopped LSP server: {}", server_name),
                }],
                is_error: false,
            })
        } else {
            Err(anyhow::anyhow!("LSP server not found: {}", server_name))
        }
    }

    async fn handle_get_completions(
        &self,
        arguments: serde_json::Value,
    ) -> Result<CallToolResult> {
        let server_name = arguments
            .get("server_name")
            .and_then(|v| v.as_str())
            .context("Missing server_name")?
            .to_string();

        let uri = arguments
            .get("uri")
            .and_then(|v| v.as_str())
            .context("Missing uri")?
            .to_string();

        let line = arguments
            .get("line")
            .and_then(|v| v.as_u64())
            .context("Missing line")? as u32;

        let character = arguments
            .get("character")
            .and_then(|v| v.as_u64())
            .context("Missing character")? as u32;

        let clients = self.clients.read().await;
        let client = clients
            .get(&server_name)
            .context(format!("LSP server not found: {}", server_name))?;

        let uri = Url::parse(&uri).context("Invalid URI")?;
        let completions = client
            .get_completions(uri, line, character)
            .await
            .context("Failed to get completions")?;

        Ok(CallToolResult {
            content: vec![CallToolResult::Text {
                text: serde_json::to_string_pretty(&completions)?,
            }],
            is_error: false,
        })
    }

    async fn handle_get_hover(
        &self,
        arguments: serde_json::Value,
    ) -> Result<CallToolResult> {
        let server_name = arguments
            .get("server_name")
            .and_then(|v| v.as_str())
            .context("Missing server_name")?
            .to_string();

        let uri = arguments
            .get("uri")
            .and_then(|v| v.as_str())
            .context("Missing uri")?
            .to_string();

        let line = arguments
            .get("line")
            .and_then(|v| v.as_u64())
            .context("Missing line")? as u32;

        let character = arguments
            .get("character")
            .and_then(|v| v.as_u64())
            .context("Missing character")? as u32;

        let clients = self.clients.read().await;
        let client = clients
            .get(&server_name)
            .context(format!("LSP server not found: {}", server_name))?;

        let uri = Url::parse(&uri).context("Invalid URI")?;
        let hover = client
            .get_hover(uri, line, character)
            .await
            .context("Failed to get hover")?;

        Ok(CallToolResult {
            content: vec![CallToolResult::Text {
                text: serde_json::to_string_pretty(&hover)?,
            }],
            is_error: false,
        })
    }

    async fn handle_get_statistics(
        &self,
        _arguments: serde_json::Value,
    ) -> Result<CallToolResult> {
        let stats = self.diagnostics_manager.get_statistics().await;

        Ok(CallToolResult {
            content: vec![CallToolResult::Text {
                text: serde_json::to_string_pretty(&stats)?,
            }],
            is_error: false,
        })
    }
}

impl Default for LspToolHandler {
    fn default() -> Self {
        Self::new()
    }
}
