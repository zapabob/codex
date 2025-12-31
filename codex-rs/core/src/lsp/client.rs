//! LSP client implementation for connecting to language servers
//!
//! Supports rust-analyzer, TypeScript Server, Python Language Server, etc.

use anyhow::{Context, Result};
use lsp_types::{
    notification::{DidChangeTextDocument, DidOpenTextDocument, Initialized, Notification},
    request::{Completion, HoverRequest, References, Request},
    CompletionParams, CompletionResponse, Hover, HoverParams,
    InitializeParams, InitializeResult, InitializedParams, ReferenceParams, ServerCapabilities,
    TextDocumentIdentifier, TextDocumentItem, TextDocumentPositionParams, Url, VersionedTextDocumentIdentifier,
};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Command, ChildStdin, ChildStdout};
use tokio::sync::Mutex;
use tracing::{debug, error, info};

/// LSP client for connecting to language servers
pub struct LspClient {
    /// Language server process
    process: Option<tokio::process::Child>,
    /// Standard input for the language server
    stdin: Option<Arc<Mutex<ChildStdin>>>,
    /// Standard output reader for the language server
    stdout_reader: Option<Arc<Mutex<BufReader<ChildStdout>>>>,
    /// Server capabilities
    capabilities: Option<ServerCapabilities>,
    /// Server name
    server_name: String,
    /// Root URI for the workspace
    root_uri: Option<Url>,
    /// Next request ID
    next_request_id: Arc<Mutex<u64>>,
    /// Pending requests
    pending_requests: Arc<Mutex<HashMap<u64, tokio::sync::oneshot::Sender<Value>>>>,
}

impl LspClient {
    /// Create a new LSP client for the specified language server
    pub fn new(server_name: String, _command: Vec<String>, root_path: PathBuf) -> Self {
        Self {
            process: None,
            stdin: None,
            stdout_reader: None,
            capabilities: None,
            server_name,
            root_uri: Some(
                Url::from_file_path(&root_path)
                    .unwrap_or_else(|_| Url::parse("file:///").unwrap()),
            ),
            next_request_id: Arc::new(Mutex::new(1)),
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Start the language server process
    pub async fn start(&mut self, command: Vec<String>) -> Result<()> {
        info!("Starting LSP server: {:?}", command);

        let mut child = Command::new(&command[0])
            .args(&command[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("Failed to spawn language server process")?;

        let stdin = child.stdin.take().context("Failed to get stdin")?;
        let stdout = child.stdout.take().context("Failed to get stdout")?;

        self.stdin = Some(Arc::new(Mutex::new(stdin)));
        let stdout_reader = Arc::new(Mutex::new(BufReader::new(stdout)));
        self.stdout_reader = Some(stdout_reader.clone());
        self.process = Some(child);

        // Start message reader task
        let pending_requests = self.pending_requests.clone();
        tokio::spawn(async move {
            Self::read_messages(stdout_reader, pending_requests).await;
        });

        // Initialize the language server
        self.initialize().await?;

        Ok(())
    }

    /// Read messages from the language server
    async fn read_messages(
        stdout_reader: Arc<Mutex<BufReader<ChildStdout>>>,
        pending_requests: Arc<Mutex<HashMap<u64, tokio::sync::oneshot::Sender<Value>>>>,
    ) {
        loop {
            let mut reader = stdout_reader.lock().await;
            let mut line = String::new();
            
            if let Err(e) = reader.read_line(&mut line).await {
                error!("Failed to read from language server: {}", e);
                break;
            }

            if line.is_empty() {
                break;
            }

            // Parse Content-Length header
            if line.starts_with("Content-Length:") {
                if let Some(len_str) = line.strip_prefix("Content-Length:") {
                    if let Ok(content_length) = len_str.trim().parse::<usize>() {
                        // Read empty line
                        let mut empty_line = String::new();
                        if reader.read_line(&mut empty_line).await.is_err() {
                            break;
                        }

                        // Read message body
                        let mut body = vec![0u8; content_length];
                        if reader.read_exact(&mut body).await.is_ok() {
                            if let Ok(message) = serde_json::from_slice::<Value>(&body) {
                                if let Some(id) = message.get("id").and_then(|v| v.as_u64()) {
                                    if let Some(result) = message.get("result") {
                                        let mut pending = pending_requests.lock().await;
                                        if let Some(tx) = pending.remove(&id) {
                                            let _ = tx.send(result.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Initialize the language server
    async fn initialize(&mut self) -> Result<()> {
        let root_uri = self.root_uri.clone().context("Root URI not set")?;

        let params = InitializeParams {
            process_id: Some(std::process::id()),
            client_info: Some(lsp_types::ClientInfo {
                name: "codex".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            locale: None,
            root_path: None,
            root_uri: Some(root_uri.clone()),
            initialization_options: None,
            capabilities: lsp_types::ClientCapabilities::default(),
            trace: None,
            workspace_folders: None,
            work_done_progress_params: Default::default(),
        };

        let response = self
            .send_request::<lsp_types::request::Initialize>(params)
            .await
            .context("Failed to initialize language server")?;

        self.capabilities = Some(response.capabilities);

        // Send initialized notification
        self.send_notification::<Initialized>(InitializedParams {}).await?;

        info!("Language server initialized: {}", self.server_name);
        Ok(())
    }

    /// Send a request to the language server
    async fn send_request<R>(&self, params: R::Params) -> Result<R::Result>
    where
        R: Request,
        R::Params: serde::Serialize,
        R::Result: serde::de::DeserializeOwned,
    {
        let id = {
            let mut next_id = self.next_request_id.lock().await;
            let id = *next_id;
            *next_id += 1;
            id
        };

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": R::METHOD,
            "params": params,
        });

        let (tx, rx) = tokio::sync::oneshot::channel();
        {
            let mut pending = self.pending_requests.lock().await;
            pending.insert(id, tx);
        }

        self.send_message(request).await?;

        let response = rx.await.context("Request cancelled")?;
        let result: R::Result = serde_json::from_value(response)
            .context("Failed to deserialize response")?;

        Ok(result)
    }

    /// Send a notification to the language server
    async fn send_notification<N>(&self, params: N::Params) -> Result<()>
    where
        N: Notification,
        N::Params: serde::Serialize,
    {
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": N::METHOD,
            "params": params,
        });

        self.send_message(notification).await
    }

    /// Send a JSON-RPC message to the language server
    async fn send_message(&self, message: Value) -> Result<()> {
        let stdin = self.stdin.as_ref().context("Stdin not available")?;
        let mut stdin = stdin.lock().await;

        let content = serde_json::to_string(&message)?;
        let content = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);

        stdin
            .write_all(content.as_bytes())
            .await
            .context("Failed to write to stdin")?;
        stdin.flush().await.context("Failed to flush stdin")?;

        debug!("Sent LSP message: {}", serde_json::to_string_pretty(&message)?);
        Ok(())
    }

    /// Open a text document
    pub async fn open_document(&self, uri: Url, language_id: String, text: String) -> Result<()> {
        let params = lsp_types::DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri,
                language_id,
                version: 0,
                text,
            },
        };

        self.send_notification::<DidOpenTextDocument>(params).await
    }

    /// Update text document content
    pub async fn change_document(
        &self,
        uri: Url,
        version: i32,
        changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
    ) -> Result<()> {
        let params = lsp_types::DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier { uri, version },
            content_changes: changes,
        };

        self.send_notification::<DidChangeTextDocument>(params).await
    }

    /// Get completion items at a position
    pub async fn get_completions(
        &self,
        uri: Url,
        line: u32,
        character: u32,
    ) -> Result<Option<CompletionResponse>> {
        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: lsp_types::Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: None,
        };

        self.send_request::<Completion>(params).await
    }

    /// Get hover information at a position
    pub async fn get_hover(&self, uri: Url, line: u32, character: u32) -> Result<Option<Hover>> {
        let params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: lsp_types::Position { line, character },
            },
            work_done_progress_params: Default::default(),
        };

        self.send_request::<HoverRequest>(params).await
    }

    /// Get references to a symbol
    pub async fn get_references(
        &self,
        uri: Url,
        line: u32,
        character: u32,
        include_declaration: bool,
    ) -> Result<Option<Vec<lsp_types::Location>>> {
        let params = ReferenceParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri },
                position: lsp_types::Position { line, character },
            },
            work_done_progress_params: Default::default(),
            partial_result_params: Default::default(),
            context: lsp_types::ReferenceContext {
                include_declaration,
            },
        };

        self.send_request::<References>(params).await
    }

    /// Get server capabilities
    pub fn capabilities(&self) -> Option<&ServerCapabilities> {
        self.capabilities.as_ref()
    }

    /// Get server name
    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    /// Stop the language server
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill().await;
        }
        Ok(())
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.start_kill();
        }
    }
}

// Helper type aliases for LSP notifications
type DidOpenTextDocumentParams = lsp_types::DidOpenTextDocumentParams;
type DidChangeTextDocumentParams = lsp_types::DidChangeTextDocumentParams;
