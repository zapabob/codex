//! OpenCode CLI Integration Bridge for Tauri
//!
//! Provides JSON-RPC communication with OpenCode CLI over stdio.
//! Implements proper error handling, timeout management, and type-safe request/response handling.

#![deny(warnings)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

/// Error types for OpenCode bridge operations
#[derive(Debug, Error)]
pub enum OpenCodeError {
    /// Process spawn failure
    #[error("Failed to spawn OpenCode process: {0}")]
    SpawnError(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// I/O error during communication
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Timeout error
    #[error("Operation timed out after {0:?}")]
    Timeout(Duration),

    /// OpenCode returned an error
    #[error("OpenCode error: {message} (code: {code:?})")]
    OpenCodeError {
        /// Error message
        message: String,
        /// Optional error code
        code: Option<i64>,
    },

    /// Bridge not initialized
    #[error("OpenCode bridge not initialized")]
    NotInitialized,

    /// Process terminated unexpectedly
    #[error("OpenCode process terminated unexpectedly")]
    ProcessTerminated,

    /// Binary not found in PATH
    #[error("OpenCode binary not found: {0}")]
    BinaryNotFound(String),
}

/// Result type alias for OpenCode operations
pub type Result<T> = std::result::Result<T, OpenCodeError>;

/// JSON-RPC request structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenCodeRequest {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Method name to invoke
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    /// Request ID
    pub id: u64,
}

impl OpenCodeRequest {
    /// Create a new JSON-RPC request
    pub fn new(method: impl Into<String>, params: Option<serde_json::Value>, id: u64) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.into(),
            params,
            id,
        }
    }
}

/// JSON-RPC error structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    /// Error code
    pub code: i64,
    /// Error message
    pub message: String,
    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC response structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenCodeResponse {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Result data (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// Error details (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID
    pub id: u64,
}

impl OpenCodeResponse {
    /// Check if the response indicates success
    pub const fn is_success(&self) -> bool {
        self.error.is_none() && self.result.is_some()
    }

    /// Extract result or return error
    pub fn into_result(self) -> Result<serde_json::Value> {
        if let Some(err) = self.error {
            Err(OpenCodeError::OpenCodeError {
                message: err.message,
                code: Some(err.code),
            })
        } else if let Some(result) = self.result {
            Ok(result)
        } else {
            Err(OpenCodeError::OpenCodeError {
                message: "Empty response".to_string(),
                code: None,
            })
        }
    }
}

/// OpenCode agent information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenCodeAgent {
    /// Agent ID
    pub id: String,
    /// Agent name
    pub name: String,
    /// Agent description
    pub description: Option<String>,
    /// Agent capabilities
    pub capabilities: Vec<String>,
    /// Whether agent is available
    pub available: bool,
}

/// OpenCode authentication status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthStatus {
    /// Whether authenticated
    pub authenticated: bool,
    /// Provider name (e.g., "openai", "anthropic")
    pub provider: Option<String>,
    /// Model being used
    pub model: Option<String>,
}

/// Execution result from OpenCode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecutionResult {
    /// Whether execution succeeded
    pub success: bool,
    /// Output content
    pub output: String,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// Files modified
    #[serde(default)]
    pub files_modified: Vec<String>,
}

/// Configuration for OpenCode bridge
#[derive(Debug, Clone)]
pub struct OpenCodeConfig {
    /// Path to OpenCode binary (None for auto-detect from PATH)
    pub binary_path: Option<String>,
    /// Default timeout for operations
    pub default_timeout: Duration,
    /// Working directory for OpenCode
    pub working_directory: Option<String>,
    /// Environment variables to pass
    pub env_vars: Vec<(String, String)>,
}

impl Default for OpenCodeConfig {
    fn default() -> Self {
        Self {
            binary_path: None,
            default_timeout: Duration::from_secs(120),
            working_directory: None,
            env_vars: Vec::new(),
        }
    }
}

/// OpenCode bridge for managing communication with OpenCode CLI
pub struct OpenCodeBridge {
    /// Bridge configuration
    config: OpenCodeConfig,
    /// Current child process
    process: Option<Arc<Mutex<Child>>>,
    /// stdin handle for sending requests
    stdin: Option<Arc<Mutex<ChildStdin>>>,
    /// stdout reader for receiving responses
    stdout_reader: Option<Arc<Mutex<BufReader<ChildStdout>>>>,
    /// Request ID counter
    request_id: Arc<AtomicU64>,
    /// Whether bridge is initialized
    initialized: Arc<std::sync::atomic::AtomicBool>,
}

impl OpenCodeBridge {
    /// Create a new OpenCode bridge with configuration
    pub fn new(config: OpenCodeConfig) -> Self {
        Self {
            config,
            process: None,
            stdin: None,
            stdout_reader: None,
            request_id: Arc::new(AtomicU64::new(1)),
            initialized: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Find OpenCode binary in PATH or use configured path
    fn find_binary(&self) -> Result<String> {
        if let Some(ref path) = self.config.binary_path {
            if std::path::Path::new(path).exists() {
                return Ok(path.clone());
            }
        }

        // Try to find in PATH
        let binary_name = if cfg!(windows) {
            "opencode.exe"
        } else {
            "opencode"
        };

        match which::which(binary_name) {
            Ok(path) => Ok(path.to_string_lossy().to_string()),
            Err(_) => Err(OpenCodeError::BinaryNotFound(
                "OpenCode not found in PATH. Please install OpenCode or specify binary_path".into(),
            )),
        }
    }

    /// Initialize the bridge by spawning OpenCode process
    pub async fn initialize(&mut self) -> Result<()> {
        if self.initialized.load(Ordering::SeqCst) {
            warn!("OpenCode bridge already initialized");
            return Ok(());
        }

        let binary_path = self.find_binary()?;
        info!("Starting OpenCode process: {}", binary_path);

        let mut cmd = Command::new(&binary_path);
        cmd.arg("--jsonrpc") // Enable JSON-RPC mode
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null()); // Suppress stderr for cleaner JSON

        // Set working directory if specified
        if let Some(ref wd) = self.config.working_directory {
            cmd.current_dir(wd);
        }

        // Set environment variables
        for (key, value) in &self.config.env_vars {
            cmd.env(key, value);
        }

        let mut child = cmd
            .spawn()
            .map_err(|e| OpenCodeError::SpawnError(e.to_string()))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| OpenCodeError::SpawnError("Failed to capture stdin".to_string()))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| OpenCodeError::SpawnError("Failed to capture stdout".to_string()))?;

        self.process = Some(Arc::new(Mutex::new(child)));
        self.stdin = Some(Arc::new(Mutex::new(stdin)));
        self.stdout_reader = Some(Arc::new(Mutex::new(BufReader::new(stdout))));
        self.initialized.store(true, Ordering::SeqCst);

        info!("OpenCode bridge initialized successfully");
        Ok(())
    }

    /// Check if bridge is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }

    /// Send a JSON-RPC request and wait for response
    async fn send_request(&self, request: &OpenCodeRequest) -> Result<OpenCodeResponse> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err(OpenCodeError::NotInitialized);
        }

        let stdin = self.stdin.as_ref().ok_or(OpenCodeError::NotInitialized)?;
        let stdout_reader = self
            .stdout_reader
            .as_ref()
            .ok_or(OpenCodeError::NotInitialized)?;

        // Serialize and send request
        let request_json = serde_json::to_string(request)?;
        let request_line = format!("{}\n", request_json);

        debug!("Sending request: {}", request_json);

        {
            let mut stdin_guard = stdin.lock().await;
            stdin_guard.write_all(request_line.as_bytes())?;
            stdin_guard.flush()?;
        }

        // Read response with timeout
        let timeout = self.config.default_timeout;
        let start = Instant::now();

        let mut response_line = String::new();
        loop {
            if start.elapsed() > timeout {
                return Err(OpenCodeError::Timeout(timeout));
            }

            let mut stdout_guard = stdout_reader.lock().await;
            match tokio::time::timeout(Duration::from_millis(100), async {
                stdout_guard.read_line(&mut response_line)
            })
            .await
            {
                Ok(Ok(0)) => {
                    // EOF reached - process might have terminated
                    return Err(OpenCodeError::ProcessTerminated);
                }
                Ok(Ok(_)) => {
                    // Got a line
                    break;
                }
                Ok(Err(e)) => {
                    return Err(OpenCodeError::IoError(e));
                }
                Err(_) => {
                    // Timeout on read, check if process is still alive
                    drop(stdout_guard);
                    if let Some(ref process) = self.process {
                        let proc_guard = process.lock().await;
                        match proc_guard.try_wait() {
                            Ok(Some(_)) => {
                                return Err(OpenCodeError::ProcessTerminated);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        debug!("Received response: {}", response_line.trim());

        // Parse response
        let response: OpenCodeResponse = serde_json::from_str(&response_line)?;

        // Verify ID matches
        if response.id != request.id {
            return Err(OpenCodeError::OpenCodeError {
                message: format!(
                    "Response ID mismatch: expected {}, got {}",
                    request.id, response.id
                ),
                code: None,
            });
        }

        Ok(response)
    }

    /// Execute a prompt using OpenCode
    pub async fn execute_prompt(
        &self,
        prompt: impl Into<String>,
        timeout: Option<Duration>,
    ) -> Result<ExecutionResult> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let params = serde_json::json!({
            "prompt": prompt.into(),
        });

        let request = OpenCodeRequest::new("execute", Some(params), id);

        // Temporarily override timeout if specified
        let original_timeout = self.config.default_timeout;
        let timeout_to_use = timeout.unwrap_or(original_timeout);

        let response = tokio::time::timeout(timeout_to_use, self.send_request(&request)).await;

        match response {
            Ok(Ok(resp)) => {
                let result = resp.into_result()?;
                let execution_result: ExecutionResult = serde_json::from_value(result)?;
                Ok(execution_result)
            }
            Ok(Err(e)) => Err(e),
            Err(_) => Err(OpenCodeError::Timeout(timeout_to_use)),
        }
    }

    /// List available agents
    pub async fn list_agents(&self) -> Result<Vec<OpenCodeAgent>> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let request = OpenCodeRequest::new("list_agents", None, id);

        let response = self.send_request(&request).await?;
        let result = response.into_result()?;

        let agents: Vec<OpenCodeAgent> = serde_json::from_value(result)?;
        Ok(agents)
    }

    /// Get authentication status
    pub async fn get_auth_status(&self) -> Result<AuthStatus> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let request = OpenCodeRequest::new("auth_status", None, id);

        let response = self.send_request(&request).await?;
        let result = response.into_result()?;

        let status: AuthStatus = serde_json::from_value(result)?;
        Ok(status)
    }

    /// Shutdown the bridge and terminate OpenCode process
    pub async fn shutdown(&mut self) -> Result<()> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Ok(());
        }

        info!("Shutting down OpenCode bridge");

        // Try graceful shutdown first
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let request = OpenCodeRequest::new("shutdown", None, id);

        // Send shutdown request but don't wait long
        let _ = tokio::time::timeout(Duration::from_secs(5), self.send_request(&request)).await;

        // Terminate process
        if let Some(ref process) = self.process {
            let mut proc_guard = process.lock().await;
            let _ = proc_guard.kill();
            let _ = proc_guard.wait();
        }

        self.process = None;
        self.stdin = None;
        self.stdout_reader = None;
        self.initialized.store(false, Ordering::SeqCst);

        info!("OpenCode bridge shutdown complete");
        Ok(())
    }
}

impl Drop for OpenCodeBridge {
    fn drop(&mut self) {
        if self.initialized.load(Ordering::SeqCst) {
            // Spawn a blocking cleanup - best effort
            let process = self.process.take();
            if let Some(proc) = process {
                let proc_arc = proc.clone();
                std::thread::spawn(move || {
                    let rt = tokio::runtime::Runtime::new().ok()?;
                    rt.block_on(async {
                        let mut guard = proc_arc.lock().await;
                        let _ = guard.kill();
                        let _ = guard.wait();
                        Some(())
                    })
                });
            }
        }
    }
}
