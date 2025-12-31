//! LSP tool handler for MCP server
//!
//! Provides LSP diagnostics as MCP tools

use anyhow::Context;
use anyhow::Result;
use codex_core::lsp::DiagnosticsManager;
use codex_core::lsp::LspClient;
use codex_core::lsp::Url;
use mcp_types::CallToolRequestParams;
use mcp_types::CallToolResult;
use mcp_types::ContentBlock;
use mcp_types::ListToolsResult;
use mcp_types::TextContent;
use mcp_types::Tool;
use mcp_types::ToolInputSchema;
use serde_json::Value;
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
                    title: None,
                    description: Some("Get LSP diagnostics for a document or all documents".to_string()),
                    input_schema: tool_input_schema(
                        serde_json::json!({
                            "uri": {
                                "type": "string",
                                "description": "Document URI (optional, if not provided returns all diagnostics)"
                            }
                        }),
                        None,
                    ),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "lsp_start_server".to_string(),
                    title: None,
                    description: Some("Start an LSP server for a language".to_string()),
                    input_schema: tool_input_schema(
                        serde_json::json!({
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
                        }),
                        Some(&["server_name", "command", "root_path"]),
                    ),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "lsp_stop_server".to_string(),
                    title: None,
                    description: Some("Stop an LSP server".to_string()),
                    input_schema: tool_input_schema(
                        serde_json::json!({
                            "server_name": {
                                "type": "string",
                                "description": "Name of the language server to stop"
                            }
                        }),
                        Some(&["server_name"]),
                    ),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "lsp_get_completions".to_string(),
                    title: None,
                    description: Some("Get code completions at a position".to_string()),
                    input_schema: tool_input_schema(
                        serde_json::json!({
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
                        }),
                        Some(&["server_name", "uri", "line", "character"]),
                    ),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "lsp_get_hover".to_string(),
                    title: None,
                    description: Some("Get hover information at a position".to_string()),
                    input_schema: tool_input_schema(
                        serde_json::json!({
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
                        }),
                        Some(&["server_name", "uri", "line", "character"]),
                    ),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "lsp_get_statistics".to_string(),
                    title: None,
                    description: Some("Get LSP diagnostics statistics".to_string()),
                    input_schema: tool_input_schema(serde_json::json!({}), None),
                    output_schema: None,
                    annotations: None,
                },
            ],
            next_cursor: None,
        }
    }

    /// Handle a tool call
    pub async fn handle_tool_call(
        &self,
        tool_call: CallToolRequestParams,
    ) -> Result<CallToolResult> {
        match tool_call.name.as_str() {
            "lsp_get_diagnostics" => {
                self.handle_get_diagnostics(tool_call.arguments.unwrap_or(Value::Null))
                    .await
            }
            "lsp_start_server" => {
                self.handle_start_server(tool_call.arguments.unwrap_or(Value::Null))
                    .await
            }
            "lsp_stop_server" => {
                self.handle_stop_server(tool_call.arguments.unwrap_or(Value::Null))
                    .await
            }
            "lsp_get_completions" => {
                self.handle_get_completions(tool_call.arguments.unwrap_or(Value::Null))
                    .await
            }
            "lsp_get_hover" => {
                self.handle_get_hover(tool_call.arguments.unwrap_or(Value::Null))
                    .await
            }
            "lsp_get_statistics" => {
                self.handle_get_statistics(tool_call.arguments.unwrap_or(Value::Null))
                    .await
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_call.name)),
        }
    }

    async fn handle_get_diagnostics(&self, arguments: serde_json::Value) -> Result<CallToolResult> {
        let uri: Option<String> = arguments
            .get("uri")
            .and_then(|v| v.as_str())
            .map(std::string::ToString::to_string);

        if let Some(uri_str) = uri {
            let uri = Url::parse(&uri_str).context("Invalid URI")?;
            let diagnostics = self
                .diagnostics_manager
                .get_combined_diagnostics(&uri)
                .await;

            Ok(CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: serde_json::to_string_pretty(&diagnostics)?,
                    annotations: None,
                })],
                is_error: Some(false),
                structured_content: None,
            })
        } else {
            let all_diagnostics = self.diagnostics_manager.get_all_diagnostics().await;
            Ok(CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: serde_json::to_string_pretty(&all_diagnostics)?,
                    annotations: None,
                })],
                is_error: Some(false),
                structured_content: None,
            })
        }
    }

    async fn handle_start_server(&self, arguments: serde_json::Value) -> Result<CallToolResult> {
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
            .filter_map(|v| v.as_str().map(std::string::ToString::to_string))
            .collect();

        let root_path = arguments
            .get("root_path")
            .and_then(|v| v.as_str())
            .context("Missing root_path")?
            .to_string();

        let root_path = PathBuf::from(root_path);
        let mut client = LspClient::new(server_name.clone(), command.clone(), root_path.clone());
        client
            .start(command)
            .await
            .context("Failed to start LSP server")?;

        let mut clients = self.clients.write().await;
        clients.insert(server_name.clone(), client);

        info!("Started LSP server: {}", server_name);

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: format!("Started LSP server: {server_name}"),
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_stop_server(&self, arguments: serde_json::Value) -> Result<CallToolResult> {
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
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: format!("Stopped LSP server: {server_name}"),
                    annotations: None,
                })],
                is_error: Some(false),
                structured_content: None,
            })
        } else {
            Err(anyhow::anyhow!("LSP server not found: {server_name}"))
        }
    }

    async fn handle_get_completions(&self, arguments: serde_json::Value) -> Result<CallToolResult> {
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
            .and_then(serde_json::Value::as_u64)
            .context("Missing line")? as u32;

        let character = arguments
            .get("character")
            .and_then(serde_json::Value::as_u64)
            .context("Missing character")? as u32;

        let clients = self.clients.read().await;
        let client = clients
            .get(&server_name)
            .context(format!("LSP server not found: {server_name}"))?;

        let uri = Url::parse(&uri).context("Invalid URI")?;
        let completions = client
            .get_completions(uri, line, character)
            .await
            .context("Failed to get completions")?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: serde_json::to_string_pretty(&completions)?,
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_get_hover(&self, arguments: serde_json::Value) -> Result<CallToolResult> {
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
            .and_then(serde_json::Value::as_u64)
            .context("Missing line")? as u32;

        let character = arguments
            .get("character")
            .and_then(serde_json::Value::as_u64)
            .context("Missing character")? as u32;

        let clients = self.clients.read().await;
        let client = clients
            .get(&server_name)
            .context(format!("LSP server not found: {server_name}"))?;

        let uri = Url::parse(&uri).context("Invalid URI")?;
        let hover = client
            .get_hover(uri, line, character)
            .await
            .context("Failed to get hover")?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: serde_json::to_string_pretty(&hover)?,
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_get_statistics(&self, _arguments: serde_json::Value) -> Result<CallToolResult> {
        let stats = self.diagnostics_manager.get_statistics().await;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: serde_json::to_string_pretty(&stats)?,
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }
}

fn tool_input_schema(
    properties: serde_json::Value,
    required: Option<&[&str]>,
) -> ToolInputSchema {
    ToolInputSchema {
        properties: Some(properties),
        required: required
            .map(|items| items.iter().map(std::string::ToString::to_string).collect()),
        r#type: "object".to_string(),
    }
}

impl Default for LspToolHandler {
    fn default() -> Self {
        Self::new()
    }
}
