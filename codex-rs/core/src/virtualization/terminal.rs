//! Virtual OS Terminal
//!
//! Provides terminal interface for executing CLI commands in virtual OS environment
//! with safety checks to prevent dangerous commands.

use super::network::VirtualNetwork;
use crate::command_safety::is_dangerous_command;
use crate::command_safety::is_safe_command;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use tokio::process::Command as TokioCommand;
use tracing::{info, warn};

/// Terminal command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub is_blocked: bool,
    pub block_reason: Option<String>,
}

/// Terminal session
pub struct TerminalSession {
    session_id: String,
    working_directory: PathBuf,
    environment: HashMap<String, String>,
    history: Vec<TerminalCommand>,
    network: Option<VirtualNetwork>,
}

/// Terminal command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalCommand {
    pub command: Vec<String>,
    pub working_directory: PathBuf,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub result: Option<TerminalResult>,
}

impl TerminalSession {
    pub fn new(working_directory: PathBuf, network: Option<VirtualNetwork>) -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            working_directory,
            environment: Self::default_environment(),
            history: Vec::new(),
            network,
        }
    }

    /// Execute a command
    pub async fn execute_command(&mut self, command: Vec<String>) -> Result<TerminalResult> {
        // YOLOモードでも危険なコマンドは完全にブロック
        if command_might_be_dangerous(&command) {
            warn!(
                "Dangerous command blocked (even in YOLO mode): {:?}",
                command
            );
            let result = TerminalResult {
                exit_code: 1,
                stdout: String::new(),
                stderr: format!(
                    "Error: Dangerous command blocked for security reasons (even in YOLO mode): {:?}. Dangerous commands cannot be executed even in YOLO mode.",
                    command
                ),
                is_blocked: true,
                block_reason: Some(
                    "Dangerous commands cannot be executed even in YOLO mode".to_string(),
                ),
            };

            // Save to history
            self.history.push(TerminalCommand {
                command: command.clone(),
                working_directory: self.working_directory.clone(),
                timestamp: chrono::Utc::now(),
                result: Some(result.clone()),
            });

            return Ok(result);
        }

        // Check if command is safe (can be auto-approved)
        let _is_safe = is_known_safe_command(&command);

        // Check network access if needed
        if let Some(ref network) = self.network {
            // Extract URL from command if it's a network command
            if let Some(url) = Self::extract_url_from_command(&command) {
                if !network.is_allowed(&url) {
                    warn!("Network access blocked: {}", url);
                    let result = TerminalResult {
                        exit_code: 1,
                        stdout: String::new(),
                        stderr: format!(
                            "Error: Network access blocked by security policy: {}",
                            url
                        ),
                        is_blocked: true,
                        block_reason: Some("Network access denied".to_string()),
                    };

                    self.history.push(TerminalCommand {
                        command: command.clone(),
                        working_directory: self.working_directory.clone(),
                        timestamp: chrono::Utc::now(),
                        result: Some(result.clone()),
                    });

                    return Ok(result);
                }
            }
        }

        // Execute the command
        info!("Executing command: {:?}", command);

        let cmd_name = command.first().context("Empty command")?;
        let args = &command[1..];

        let mut cmd = TokioCommand::new(cmd_name);
        cmd.args(args)
            .current_dir(&self.working_directory)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Set environment variables
        for (key, value) in &self.environment {
            cmd.env(key, value);
        }

        let output = cmd.output().await.context("Failed to execute command")?;

        let result = TerminalResult {
            exit_code: output.status.code().unwrap_or(1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            is_blocked: false,
            block_reason: None,
        };

        // Save to history
        self.history.push(TerminalCommand {
            command: command.clone(),
            working_directory: self.working_directory.clone(),
            timestamp: chrono::Utc::now(),
            result: Some(result.clone()),
        });

        Ok(result)
    }

    /// Get command history
    pub fn get_history(&self) -> &[TerminalCommand] {
        &self.history
    }

    /// Change working directory
    pub fn change_directory(&mut self, path: PathBuf) -> Result<()> {
        if path.is_absolute() {
            self.working_directory = path;
        } else {
            self.working_directory = self.working_directory.join(path);
        }

        // Normalize path
        self.working_directory = self
            .working_directory
            .canonicalize()
            .context("Failed to canonicalize path")?;

        Ok(())
    }

    /// Get current working directory
    pub fn get_working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    /// Set environment variable
    pub fn set_env(&mut self, key: String, value: String) {
        self.environment.insert(key, value);
    }

    /// Get environment variable
    pub fn get_env(&self, key: &str) -> Option<&String> {
        self.environment.get(key)
    }

    /// List available CLI commands
    pub async fn list_available_commands(&self) -> Vec<String> {
        let mut commands = Vec::new();

        // Common CLI commands
        let common_commands = vec![
            "codex", "gemini", "claude", "git", "cargo", "rustc", "rustup", "node", "npm", "npx",
            "python", "python3", "pip", "go", "golang", "docker", "kubectl", "curl", "wget", "ls",
            "cd", "pwd", "cat", "grep", "find",
        ];

        for cmd in common_commands {
            if Self::command_exists(cmd).await {
                commands.push(cmd.to_string());
            }
        }

        commands
    }

    /// Check if a command exists
    async fn command_exists(cmd: &str) -> bool {
        #[cfg(windows)]
        {
            let output = Command::new("where").arg(cmd).output().ok();
            output.map(|o| o.status.success()).unwrap_or(false)
        }

        #[cfg(not(windows))]
        {
            let output = Command::new("which").arg(cmd).output().ok();
            output.map(|o| o.status.success()).unwrap_or(false)
        }
    }

    /// Extract URL from command (for network access check)
    fn extract_url_from_command(command: &[String]) -> Option<String> {
        // Check for curl, wget, or similar commands
        for arg in command.iter() {
            if arg.starts_with("http://") || arg.starts_with("https://") {
                return Some(arg.clone());
            }
        }
        None
    }

    /// Default environment variables
    fn default_environment() -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert(
            "PATH".to_string(),
            std::env::var("PATH").unwrap_or_default(),
        );
        env.insert(
            "HOME".to_string(),
            std::env::var("HOME").unwrap_or_default(),
        );
        env.insert(
            "USER".to_string(),
            std::env::var("USER").unwrap_or_default(),
        );
        env
    }
}

/// Terminal Manager
pub struct TerminalManager {
    sessions: HashMap<String, TerminalSession>,
}

impl TerminalManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Create a new terminal session
    pub fn create_session(
        &mut self,
        working_directory: PathBuf,
        network: Option<VirtualNetwork>,
    ) -> String {
        let session = TerminalSession::new(working_directory, network);
        let session_id = session.session_id.clone();
        self.sessions.insert(session_id.clone(), session);
        session_id
    }

    /// Get a session
    pub fn get_session(&self, session_id: &str) -> Option<&TerminalSession> {
        self.sessions.get(session_id)
    }

    /// Get a mutable session
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut TerminalSession> {
        self.sessions.get_mut(session_id)
    }

    /// Remove a session
    pub fn remove_session(&mut self, session_id: &str) {
        self.sessions.remove(session_id);
    }

    /// List all sessions
    pub fn list_sessions(&self) -> Vec<&TerminalSession> {
        self.sessions.values().collect()
    }
}

impl Default for TerminalManager {
    fn default() -> Self {
        Self::new()
    }
}
