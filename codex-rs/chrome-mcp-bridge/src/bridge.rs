use anyhow::{Context, Result};
use mcp_types::{
    CallToolRequest, CallToolRequestParams, CallToolResult, ContentBlock, InitializeRequest,
    InitializeRequestParams, InitializeResult, JSONRPCMessage, ListToolsRequest, ListToolsResult,
    ModelContextProtocolRequest, RequestId, ServerCapabilities, ServerCapabilitiesTools,
    TextContent,
};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, stdout};
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tracing::{error, info};

use crate::tools::get_chrome_tools;

/// Bridge server that connects CLI and Chrome extension via MCP
pub struct BridgeServer {
    extension_connections: Arc<Mutex<HashMap<String, ExtensionConnection>>>,
    cli_connections: Arc<Mutex<HashMap<String, CliConnection>>>,
    tools: Vec<mcp_types::Tool>,
}

struct ExtensionConnection {
    // Extension connection state
    // In a full implementation, this would hold the actual connection
}

struct CliConnection {
    // CLI connection state
    // In a full implementation, this would hold the actual connection
}

impl BridgeServer {
    pub fn new() -> Self {
        Self {
            extension_connections: Arc::new(Mutex::new(HashMap::new())),
            cli_connections: Arc::new(Mutex::new(HashMap::new())),
            tools: get_chrome_tools(),
        }
    }

    /// Run bridge server in stdio mode
    pub async fn run_stdio() -> Result<()> {
        let bridge = Arc::new(BridgeServer::new());
        let (incoming_tx, mut incoming_rx) = mpsc::channel::<JSONRPCMessage>(128);
        let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<JSONRPCMessage>();

        // Task: read from stdin
        let stdin_handle = tokio::spawn({
            let incoming_tx = incoming_tx.clone();
            async move {
                let stdin = tokio::io::stdin();
                let reader = BufReader::new(stdin);
                let mut lines = reader.lines();

                loop {
                    match lines.next_line().await {
                        Ok(Some(line)) => match serde_json::from_str::<JSONRPCMessage>(&line) {
                            Ok(msg) => {
                                if incoming_tx.send(msg).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => error!("Failed to deserialize JSONRPCMessage: {e}"),
                        },
                        Ok(None) => break,
                        Err(e) => {
                            error!("Failed to read line from stdin: {e}");
                            break;
                        }
                    }
                }
            }
        });

        // Task: process messages
        let processor_handle = tokio::spawn({
            let bridge = bridge.clone();
            let outgoing_tx = outgoing_tx.clone();
            async move {
                while let Some(msg) = incoming_rx.recv().await {
                    if let Err(e) = bridge.process_message(msg, outgoing_tx.clone()).await {
                        error!("Error processing message: {e}");
                    }
                }
            }
        });

        // Task: write to stdout
        let stdout_handle = tokio::spawn(async move {
            let mut stdout = stdout();
            while let Some(msg) = outgoing_rx.recv().await {
                match serde_json::to_string(&msg) {
                    Ok(json) => {
                        if let Err(e) = stdout.write_all(json.as_bytes()).await {
                            error!("Failed to write to stdout: {e}");
                            break;
                        }
                        if let Err(e) = stdout.write_all(b"\n").await {
                            error!("Failed to write newline to stdout: {e}");
                            break;
                        }
                        if let Err(e) = stdout.flush().await {
                            error!("Failed to flush stdout: {e}");
                            break;
                        }
                    }
                    Err(e) => error!("Failed to serialize JSONRPCMessage: {e}"),
                }
            }
        });

        let _ = tokio::join!(stdin_handle, processor_handle, stdout_handle);
        Ok(())
    }

    /// Run bridge server in HTTP mode
    pub async fn run_http(port: u16) -> Result<()> {
        info!("Starting MCP bridge server on port {}", port);

        // For now, we'll implement a basic HTTP server
        // In a full implementation, this would use streamable HTTP MCP transport
        use tokio::net::TcpListener;

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .await
            .context("Failed to bind to port")?;

        info!("MCP bridge server listening on http://127.0.0.1:{}", port);

        let bridge = Arc::new(BridgeServer::new());

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("New connection from {}", addr);
                    let bridge_clone = bridge.clone();
                    tokio::spawn(async move {
                        if let Err(e) = bridge_clone.handle_http_connection(stream).await {
                            error!("Error handling HTTP connection: {e}");
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {e}");
                }
            }
        }
    }

    async fn handle_http_connection(&self, _stream: tokio::net::TcpStream) -> Result<()> {
        // HTTP connection handling would be implemented here
        // For now, this is a placeholder
        Ok(())
    }

    async fn process_message(
        &self,
        msg: JSONRPCMessage,
        outgoing_tx: mpsc::UnboundedSender<JSONRPCMessage>,
    ) -> Result<()> {
        match msg {
            JSONRPCMessage::Request(req) => {
                self.handle_request(req, outgoing_tx).await?;
            }
            JSONRPCMessage::Response(_) => {
                // Handle responses from extension
            }
            JSONRPCMessage::Notification(_) => {
                // Handle notifications
            }
            JSONRPCMessage::Error(_) => {
                // Handle errors
            }
        }
        Ok(())
    }

    async fn handle_request(
        &self,
        request: mcp_types::JSONRPCRequest,
        outgoing_tx: mpsc::UnboundedSender<JSONRPCMessage>,
    ) -> Result<()> {
        let request_id = request.id.clone();
        let method = request.method.clone();

        match method.as_str() {
            "initialize" => {
                let params: InitializeRequestParams = serde_json::from_value(
                    request
                        .params
                        .ok_or_else(|| anyhow::anyhow!("Missing params"))?,
                )?;
                let result = InitializeResult {
                    capabilities: ServerCapabilities {
                        tools: Some(ServerCapabilitiesTools {
                            list_changed: Some(true),
                        }),
                        completions: None,
                        experimental: None,
                        logging: None,
                        prompts: None,
                        resources: None,
                    },
                    instructions: None,
                    protocol_version: params.protocol_version.clone(),
                    server_info: mcp_types::Implementation {
                        name: "codex-chrome-mcp-bridge".to_string(),
                        version: env!("CARGO_PKG_VERSION").to_string(),
                        title: Some("Codex Chrome MCP Bridge".to_string()),
                        user_agent: None,
                    },
                };
                self.send_response::<InitializeRequest>(request_id, result, outgoing_tx)
                    .await?;
            }
            "tools/list" => {
                let result = ListToolsResult {
                    tools: self.tools.clone(),
                    next_cursor: None,
                };
                self.send_response::<ListToolsRequest>(request_id, result, outgoing_tx)
                    .await?;
            }
            "tools/call" => {
                let params: CallToolRequestParams = serde_json::from_value(
                    request
                        .params
                        .ok_or_else(|| anyhow::anyhow!("Missing params"))?,
                )?;
                self.handle_tool_call(request_id, params, outgoing_tx)
                    .await?;
            }
            _ => {
                error!("Unknown method: {}", method);
            }
        }

        Ok(())
    }

    async fn handle_tool_call(
        &self,
        request_id: RequestId,
        params: CallToolRequestParams,
        outgoing_tx: mpsc::UnboundedSender<JSONRPCMessage>,
    ) -> Result<()> {
        let CallToolRequestParams { name, arguments } = params;

        // For now, return a message indicating that the extension needs to process this
        // In a full implementation, this would forward the request to the extension
        let result = match name.as_str() {
            "dom_read" => CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: format!(
                        "DOM read request received. This requires the Chrome extension to be connected. Arguments: {}",
                        serde_json::to_string(&arguments.unwrap_or_default())?
                    ),
                    annotations: None,
                })],
                is_error: Some(false),
                structured_content: None,
            },
            "console_get_logs" => CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: format!(
                        "Console logs request received. This requires the Chrome extension to be connected. Arguments: {}",
                        serde_json::to_string(&arguments.unwrap_or_default())?
                    ),
                    annotations: None,
                })],
                is_error: Some(false),
                structured_content: None,
            },
            "network_get_logs" => CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: format!(
                        "Network logs request received. This requires the Chrome extension to be connected. Arguments: {}",
                        serde_json::to_string(&arguments.unwrap_or_default())?
                    ),
                    annotations: None,
                })],
                is_error: Some(false),
                structured_content: None,
            },
            _ => CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: format!("Unknown tool: {}", name),
                    annotations: None,
                })],
                is_error: Some(true),
                structured_content: None,
            },
        };

        self.send_response::<CallToolRequest>(request_id, result, outgoing_tx)
            .await?;

        Ok(())
    }

    async fn send_response<T>(
        &self,
        id: RequestId,
        result: T::Result,
        outgoing_tx: mpsc::UnboundedSender<JSONRPCMessage>,
    ) -> Result<()>
    where
        T: ModelContextProtocolRequest,
    {
        let response = mcp_types::JSONRPCResponse {
            jsonrpc: mcp_types::JSONRPC_VERSION.to_string(),
            id,
            result: serde_json::to_value(result)?,
        };

        outgoing_tx
            .send(JSONRPCMessage::Response(response))
            .map_err(|e| anyhow::anyhow!("Failed to send response: {e}"))?;

        Ok(())
    }
}
