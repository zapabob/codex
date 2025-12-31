//! Microsoft 365 MCP tool handler
//!
//! Provides Word, Excel, PowerPoint, and Outlook operations as MCP tools

use anyhow::Context;
use anyhow::Result;
use codex_microsoft365::Microsoft365AuthManager;
use codex_microsoft365::Microsoft365Client;
use mcp_types::CallToolRequestParams;
use mcp_types::CallToolResult;
use mcp_types::ContentBlock;
use mcp_types::ListToolsResult;
use mcp_types::TextContent;
use mcp_types::Tool;
use mcp_types::ToolInputSchema;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

/// Microsoft 365 tool handler
pub struct Microsoft365ToolHandler {
    /// Microsoft 365 client
    client: Option<Arc<Microsoft365Client>>,
    /// Authentication manager
    auth_manager: Option<Arc<Microsoft365AuthManager>>,
    /// Codex home directory
    codex_home: PathBuf,
}

impl Microsoft365ToolHandler {
    /// Create a new Microsoft 365 tool handler
    pub fn new(codex_home: PathBuf) -> Self {
        Self {
            client: None,
            auth_manager: None,
            codex_home,
        }
    }

    /// Initialize authentication (if configured)
    pub fn initialize_auth(
        &mut self,
        client_id: String,
        tenant_id: String,
        redirect_url: String,
    ) -> Result<()> {
        let auth_manager = Arc::new(
            Microsoft365AuthManager::new(
                client_id,
                tenant_id,
                redirect_url,
                self.codex_home.clone(),
            )
            .context("Failed to create auth manager")?,
        );

        let client = Arc::new(Microsoft365Client::new(auth_manager.clone()));

        self.auth_manager = Some(auth_manager);
        self.client = Some(client);

        info!("Microsoft 365 authentication initialized");
        Ok(())
    }

    /// List available Microsoft 365 tools
    pub fn list_tools(&self) -> ListToolsResult {
        ListToolsResult {
            tools: vec![
                Tool {
                    name: "m365_word_read".to_string(),
                    title: None,
                    description: Some("Read a Word document from OneDrive/SharePoint".to_string()),
                    input_schema: serde_json::from_value(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "drive_id": { "type": "string", "description": "Drive ID" },
                            "item_id": { "type": "string", "description": "Item ID" }
                        },
                        "required": ["drive_id", "item_id"]
                    }))
                    .unwrap(),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "m365_word_create".to_string(),
                    title: None,
                    description: Some("Create a new Word document".to_string()),
                    input_schema: serde_json::from_value(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "name": { "type": "string", "description": "Document name" },
                            "content": { "type": "string", "description": "Document content" }
                        },
                        "required": ["name", "content"]
                    }))
                    .unwrap(),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "m365_excel_read".to_string(),
                    title: None,
                    description: Some("Read an Excel spreadsheet".to_string()),
                    input_schema: serde_json::from_value(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "drive_id": { "type": "string" },
                            "item_id": { "type": "string" }
                        },
                        "required": ["drive_id", "item_id"]
                    }))
                    .unwrap(),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "m365_excel_update_cell".to_string(),
                    title: None,
                    description: Some("Update a cell in an Excel spreadsheet".to_string()),
                    input_schema: serde_json::from_value(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "drive_id": { "type": "string" },
                            "item_id": { "type": "string" },
                            "worksheet": { "type": "string" },
                            "cell": { "type": "string" },
                            "value": { "type": "string" }
                        },
                        "required": ["drive_id", "item_id", "worksheet", "cell", "value"]
                    }))
                    .unwrap(),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "m365_powerpoint_read".to_string(),
                    title: None,
                    description: Some("Read a PowerPoint presentation".to_string()),
                    input_schema: serde_json::from_value(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "drive_id": { "type": "string" },
                            "item_id": { "type": "string" }
                        },
                        "required": ["drive_id", "item_id"]
                    }))
                    .unwrap(),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "m365_outlook_send_email".to_string(),
                    title: None,
                    description: Some("Send an email via Outlook".to_string()),
                    input_schema: serde_json::from_value(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "to": { "type": "array", "items": { "type": "string" } },
                            "subject": { "type": "string" },
                            "body": { "type": "string" }
                        },
                        "required": ["to", "subject", "body"]
                    }))
                    .unwrap(),
                    output_schema: None,
                    annotations: None,
                },
                Tool {
                    name: "m365_outlook_get_calendar".to_string(),
                    title: None,
                    description: Some("Get calendar events".to_string()),
                    input_schema: serde_json::from_value(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "start": { "type": "string", "description": "Start date (ISO 8601)" },
                            "end": { "type": "string", "description": "End date (ISO 8601)" }
                        },
                        "required": ["start", "end"]
                    }))
                    .unwrap(),
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
        let client = self
            .client
            .as_ref()
            .context("Microsoft 365 not authenticated. Please configure client_id and tenant_id in config.toml")?;

        match tool_call.name.as_str() {
            "m365_word_read" => self.handle_word_read(client, tool_call.arguments).await,
            "m365_word_create" => self.handle_word_create(client, tool_call.arguments).await,
            "m365_excel_read" => self.handle_excel_read(client, tool_call.arguments).await,
            "m365_excel_update_cell" => {
                self.handle_excel_update_cell(client, tool_call.arguments)
                    .await
            }
            "m365_powerpoint_read" => {
                self.handle_powerpoint_read(client, tool_call.arguments)
                    .await
            }
            "m365_outlook_send_email" => {
                self.handle_outlook_send_email(client, tool_call.arguments)
                    .await
            }
            "m365_outlook_get_calendar" => {
                self.handle_outlook_get_calendar(client, tool_call.arguments)
                    .await
            }
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_call.name)),
        }
    }

    async fn handle_word_read(
        &self,
        client: &Microsoft365Client,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let drive_id = arguments
            .get("drive_id")
            .and_then(|v| v.as_str())
            .context("Missing drive_id")?;
        let item_id = arguments
            .get("item_id")
            .and_then(|v| v.as_str())
            .context("Missing item_id")?;

        let content = client.word_read_document(drive_id, item_id).await?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: content,
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_word_create(
        &self,
        client: &Microsoft365Client,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let name = arguments
            .get("name")
            .and_then(|v| v.as_str())
            .context("Missing name")?
            .to_string();
        let content = arguments
            .get("content")
            .and_then(|v| v.as_str())
            .context("Missing content")?
            .to_string();

        let result = client.word_create_document(name.clone(), content).await?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: result,
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_excel_read(
        &self,
        client: &Microsoft365Client,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let drive_id = arguments
            .get("drive_id")
            .and_then(|v| v.as_str())
            .context("Missing drive_id")?;
        let item_id = arguments
            .get("item_id")
            .and_then(|v| v.as_str())
            .context("Missing item_id")?;

        let data = client.excel_read_spreadsheet(drive_id, item_id).await?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: serde_json::to_string_pretty(&data)?,
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_excel_update_cell(
        &self,
        client: &Microsoft365Client,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let drive_id = arguments
            .get("drive_id")
            .and_then(|v| v.as_str())
            .context("Missing drive_id")?;
        let item_id = arguments
            .get("item_id")
            .and_then(|v| v.as_str())
            .context("Missing item_id")?;
        let worksheet = arguments
            .get("worksheet")
            .and_then(|v| v.as_str())
            .context("Missing worksheet")?;
        let cell = arguments
            .get("cell")
            .and_then(|v| v.as_str())
            .context("Missing cell")?;
        let value = arguments.get("value").context("Missing value")?.clone();

        client
            .excel_update_cell(drive_id, item_id, worksheet, cell, value)
            .await?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: format!("Cell {} updated successfully", cell),
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_powerpoint_read(
        &self,
        client: &Microsoft365Client,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let drive_id = arguments
            .get("drive_id")
            .and_then(|v| v.as_str())
            .context("Missing drive_id")?;
        let item_id = arguments
            .get("item_id")
            .and_then(|v| v.as_str())
            .context("Missing item_id")?;

        let data = client
            .powerpoint_read_presentation(drive_id, item_id)
            .await?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: serde_json::to_string_pretty(&data)?,
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_outlook_send_email(
        &self,
        client: &Microsoft365Client,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let to: Vec<String> = arguments
            .get("to")
            .and_then(|v| v.as_array())
            .context("Missing to")?
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        let subject = arguments
            .get("subject")
            .and_then(|v| v.as_str())
            .context("Missing subject")?
            .to_string();
        let body = arguments
            .get("body")
            .and_then(|v| v.as_str())
            .context("Missing body")?
            .to_string();

        client.outlook_send_email(to, subject, body).await?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: "Email sent successfully".to_string(),
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }

    async fn handle_outlook_get_calendar(
        &self,
        client: &Microsoft365Client,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let start_str = arguments
            .get("start")
            .and_then(|v| v.as_str())
            .context("Missing start")?;
        let end_str = arguments
            .get("end")
            .and_then(|v| v.as_str())
            .context("Missing end")?;

        let start = chrono::DateTime::parse_from_rfc3339(start_str)
            .context("Invalid start date")?
            .with_timezone(&chrono::Utc);
        let end = chrono::DateTime::parse_from_rfc3339(end_str)
            .context("Invalid end date")?
            .with_timezone(&chrono::Utc);

        let events = client.outlook_get_calendar_events(start, end).await?;

        Ok(CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: serde_json::to_string_pretty(&events)?,
                annotations: None,
            })],
            is_error: Some(false),
            structured_content: None,
        })
    }
}
