//! External MCP Server Manager
//!
//! Manages connections to external MCP servers defined in .cursor/mcp.json

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::Command as TokioCommand;
use tokio::sync::Mutex;

/// External MCP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalMcpServer {
    /// Server command
    pub command: String,
    /// Command arguments
    pub args: Vec<String>,
    /// Environment variables
    pub env: HashMap<String, String>,
}

/// MCP configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// MCP servers configuration
    pub mcp_servers: HashMap<String, ExternalMcpServer>,
}

/// External MCP server connection
#[derive(Debug)]
pub struct McpServerConnection {
    /// Server name
    pub name: String,
    /// Server configuration
    pub config: ExternalMcpServer,
    /// Running process (if any)
    pub process: Option<tokio::process::Child>,
    /// Connection status
    pub connected: bool,
}

/// External MCP server manager
pub struct ExternalMcpManager {
    /// Configuration file path
    config_path: PathBuf,
    /// Server connections
    connections: Arc<Mutex<HashMap<String, McpServerConnection>>>,
}

impl ExternalMcpManager {
    /// Create new external MCP manager
    pub fn new(config_path: Option<PathBuf>) -> Self {
        let config_path = config_path.unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".cursor")
                .join("mcp.json")
        });

        Self {
            config_path,
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Load MCP configuration from file
    pub async fn load_config(&self) -> Result<McpConfig, String> {
        if !self.config_path.exists() {
            return Err(format!(
                "MCP config file not found: {}",
                self.config_path.display()
            ));
        }

        let content = tokio::fs::read_to_string(&self.config_path)
            .await
            .map_err(|e| format!("Failed to read MCP config: {}", e))?;

        let config: McpConfig = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse MCP config: {}", e))?;

        Ok(config)
    }

    /// Initialize all external MCP servers
    pub async fn initialize_servers(&self) -> Result<(), String> {
        let config = self.load_config().await?;

        let mut connections = self.connections.lock().await;

        for (name, server_config) in config.mcp_servers {
            let connection = McpServerConnection {
                name: name.clone(),
                config: server_config,
                process: None,
                connected: false,
            };

            connections.insert(name, connection);
        }

        Ok(())
    }

    /// Start an external MCP server
    pub async fn start_server(&self, name: &str) -> Result<(), String> {
        let mut connections = self.connections.lock().await;

        let connection = connections
            .get_mut(name)
            .ok_or_else(|| format!("Server '{}' not found", name))?;

        if connection.connected {
            return Ok(()); // Already running
        }

        // Start the external MCP server process
        let mut command = TokioCommand::new(&connection.config.command);

        // Add arguments
        for arg in &connection.config.args {
            command.arg(arg);
        }

        // Set environment variables
        command.envs(&connection.config.env);

        // Configure stdio for MCP communication
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        command.stdin(Stdio::piped());

        let child = command
            .spawn()
            .map_err(|e| format!("Failed to start MCP server '{}': {}", name, e))?;

        connection.process = Some(child);
        connection.connected = true;

        info!("Started external MCP server: {}", name);

        Ok(())
    }

    /// Stop an external MCP server
    pub async fn stop_server(&self, name: &str) -> Result<(), String> {
        let mut connections = self.connections.lock().await;

        let connection = connections
            .get_mut(name)
            .ok_or_else(|| format!("Server '{}' not found", name))?;

        if let Some(mut child) = connection.process.take() {
            child
                .kill()
                .await
                .map_err(|e| format!("Failed to stop MCP server '{}': {}", name, e))?;

            connection.connected = false;
            info!("Stopped external MCP server: {}", name);
        }

        Ok(())
    }

    /// Get list of available servers
    pub async fn list_servers(&self) -> Vec<String> {
        let connections = self.connections.lock().await;
        connections.keys().cloned().collect()
    }

    /// Get server status
    pub async fn get_server_status(&self, name: &str) -> Result<ServerStatus, String> {
        let connections = self.connections.lock().await;

        let connection = connections
            .get(name)
            .ok_or_else(|| format!("Server '{}' not found", name))?;

        let status = if connection.connected {
            if let Some(ref child) = connection.process {
                match child.try_wait() {
                    Ok(Some(exit_status)) => {
                        if exit_status.success() {
                            ServerStatus::Stopped
                        } else {
                            ServerStatus::Error(format!("Exit code: {}", exit_status))
                        }
                    }
                    Ok(None) => ServerStatus::Running,
                    Err(e) => ServerStatus::Error(format!("Status check failed: {}", e)),
                }
            } else {
                ServerStatus::Disconnected
            }
        } else {
            ServerStatus::Stopped
        };

        Ok(status)
    }

    /// Send request to external MCP server
    pub async fn send_request(
        &self,
        server_name: &str,
        request: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let connections = self.connections.lock().await;

        let connection = connections
            .get(server_name)
            .ok_or_else(|| format!("Server '{}' not found", server_name))?;

        if !connection.connected {
            return Err(format!("Server '{}' is not running", server_name));
        }

        // This is a simplified implementation
        // In a real implementation, you would:
        // 1. Get the stdin/stdout handles from the child process
        // 2. Send JSON-RPC requests
        // 3. Receive and parse responses
        // 4. Handle the MCP protocol properly

        Err("External MCP server communication not yet implemented".to_string())
    }

    /// Get server configuration
    pub async fn get_server_config(&self, name: &str) -> Result<ExternalMcpServer, String> {
        let connections = self.connections.lock().await;

        let connection = connections
            .get(name)
            .ok_or_else(|| format!("Server '{}' not found", name))?;

        Ok(connection.config.clone())
    }
}

/// Server status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerStatus {
    /// Server is running
    Running,
    /// Server is stopped
    Stopped,
    /// Server is disconnected
    Disconnected,
    /// Server has an error
    Error(String),
}

impl std::fmt::Display for ServerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerStatus::Running => write!(f, "running"),
            ServerStatus::Stopped => write!(f, "stopped"),
            ServerStatus::Disconnected => write!(f, "disconnected"),
            ServerStatus::Error(msg) => write!(f, "error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_config_loading() {
        let config_content = r#"
        {
            "mcpServers": {
                "test-server": {
                    "command": "echo",
                    "args": ["hello"],
                    "env": {
                        "TEST_VAR": "test_value"
                    }
                }
            }
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(config_content.as_bytes()).unwrap();

        let manager = ExternalMcpManager::new(Some(temp_file.path().to_path_buf()));
        let config = manager.load_config().await.unwrap();

        assert!(config.mcp_servers.contains_key("test-server"));
        let server = &config.mcp_servers["test-server"];
        assert_eq!(server.command, "echo");
        assert_eq!(server.args, vec!["hello"]);
        assert_eq!(server.env.get("TEST_VAR"), Some(&"test_value".to_string()));
    }

    #[tokio::test]
    async fn test_server_initialization() {
        let config_content = r#"
        {
            "mcpServers": {
                "test-server": {
                    "command": "echo",
                    "args": ["hello"],
                    "env": {}
                }
            }
        }
        "#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(config_content.as_bytes()).unwrap();

        let manager = ExternalMcpManager::new(Some(temp_file.path().to_path_buf()));
        manager.initialize_servers().await.unwrap();

        let servers = manager.list_servers().await;
        assert!(servers.contains(&"test-server".to_string()));
    }
}
