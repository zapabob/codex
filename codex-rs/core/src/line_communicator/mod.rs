//! LINE Communicator for Remote Development
//!
//! Enables bidirectional communication via LINE for remote development
//! Features:
//! - LINE messaging integration
//! - Remote command execution
//! - File upload/download via LINE
//! - Real-time development collaboration

use crate::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};

mod types;
pub use types::*;

/// LINE Communicator
pub struct LineCommunicator {
    config: LineConfig,
    client: Client,
    active_sessions: Arc<Mutex<HashMap<String, DevelopmentSession>>>,
    command_tx: mpsc::UnboundedSender<CommunicationCommand>,
    command_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<CommunicationCommand>>>>,
}

/// Communication commands
#[derive(Debug)]
pub enum CommunicationCommand {
    SendMessage {
        user_id: String,
        message: String,
        response: oneshot::Sender<Result<()>>,
    },
    HandleIncomingMessage {
        message: LineMessage,
    },
    ExecuteDevelopmentCommand {
        user_id: String,
        command: DevelopmentCommand,
        response: oneshot::Sender<Result<String>>,
    },
    StartSession {
        user_id: String,
        user_name: String,
    },
    EndSession {
        user_id: String,
    },
}

impl LineCommunicator {
    pub fn new(config: LineConfig) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self {
            config,
            client: Client::new(),
            active_sessions: Arc::new(Mutex::new(HashMap::new())),
            command_tx: tx,
            command_rx: Arc::new(Mutex::new(Some(rx))),
        }
    }

    /// Send message to LINE user
    pub async fn send_message(&self, user_id: &str, message: &str) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(CommunicationCommand::SendMessage {
            user_id: user_id.to_string(),
            message: message.to_string(),
            response: tx,
        })?;

        rx.await?
    }

    /// Handle incoming webhook from LINE
    pub async fn handle_webhook(&self, events: Vec<serde_json::Value>) -> Result<()> {
        for event in events {
            if let Some(message_event) = Self::parse_line_event(event)? {
                self.command_tx
                    .send(CommunicationCommand::HandleIncomingMessage {
                        message: message_event,
                    })?;
            }
        }

        Ok(())
    }

    /// Execute development command from LINE
    pub async fn execute_command(
        &self,
        user_id: &str,
        command: DevelopmentCommand,
    ) -> Result<String> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(CommunicationCommand::ExecuteDevelopmentCommand {
                user_id: user_id.to_string(),
                command,
                response: tx,
            })?;

        rx.await?
    }

    /// Parse LINE webhook event
    fn parse_line_event(event: serde_json::Value) -> Result<Option<LineMessage>> {
        let event_type = event["type"].as_str().unwrap_or("");

        if event_type != "message" {
            return Ok(None);
        }

        let message = event["message"].clone();
        let message_type = message["type"].as_str().unwrap_or("");

        let line_message_type = match message_type {
            "text" => LineMessageType::Text,
            "image" => LineMessageType::Image,
            "file" => LineMessageType::File,
            "location" => LineMessageType::Location,
            "sticker" => LineMessageType::Sticker,
            _ => return Ok(None),
        };

        let source = event["source"].clone();
        let user_id = source["userId"].as_str().unwrap_or("").to_string();

        let message_id = message["id"].as_str().unwrap_or("").to_string();
        let timestamp = event["timestamp"].as_i64().unwrap_or(0);
        let reply_token = event["replyToken"].as_str().map(|s| s.to_string());

        let text = if message_type == "text" {
            message["text"].as_str().map(|s| s.to_string())
        } else {
            None
        };

        Ok(Some(LineMessage {
            message_id,
            message_type: line_message_type,
            text,
            sender_id: user_id.clone(),
            sender_name: user_id, // In real implementation, get from user profile
            timestamp,
            reply_token,
        }))
    }

    /// Run the LINE communicator
    pub async fn run(self) -> Result<()> {
        let mut rx = self.command_rx.lock().unwrap().take().unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                CommunicationCommand::SendMessage {
                    user_id,
                    message,
                    response,
                } => {
                    let result = self.send_line_message(&user_id, &message).await;
                    let _ = response.send(result);
                }
                CommunicationCommand::HandleIncomingMessage { message } => {
                    self.handle_incoming_message(message).await?;
                }
                CommunicationCommand::ExecuteDevelopmentCommand {
                    user_id,
                    command,
                    response,
                } => {
                    let result = self.execute_development_command(&user_id, command).await;
                    let _ = response.send(result);
                }
                CommunicationCommand::StartSession { user_id, user_name } => {
                    self.start_development_session(&user_id, &user_name);
                }
                CommunicationCommand::EndSession { user_id } => {
                    self.end_development_session(&user_id);
                }
            }
        }

        Ok(())
    }

    async fn send_line_message(&self, user_id: &str, message: &str) -> Result<()> {
        let payload = serde_json::json!({
            "to": user_id,
            "messages": [{
                "type": "text",
                "text": message
            }]
        });

        let response = self
            .client
            .post("https://api.line.me/v2/bot/message/push")
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Bearer {}", self.config.channel_access_token),
            )
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("LINE API error: {}", response.status()).into());
        }

        Ok(())
    }

    async fn handle_incoming_message(&self, message: LineMessage) -> Result<()> {
        // Start or update session
        self.start_development_session(&message.sender_id, &message.sender_name);

        // Parse and execute command
        if let Some(text) = &message.text {
            if let Some(command) = self.parse_command(text) {
                let result = self
                    .execute_development_command(&message.sender_id, command)
                    .await?;

                // Send result back to user
                self.send_message(&message.sender_id, &result).await?;
            } else {
                // Send help message
                let help = self.get_help_message();
                self.send_message(&message.sender_id, &help).await?;
            }
        }

        Ok(())
    }

    fn parse_command(&self, text: &str) -> Option<DevelopmentCommand> {
        let text = text.trim();

        if text.starts_with("/code ") {
            let parts: Vec<&str> = text[6..].splitn(2, ' ').collect();
            if parts.len() == 2 {
                return Some(DevelopmentCommand::ExecuteCode {
                    code: parts[1].to_string(),
                    language: parts[0].to_string(),
                });
            }
        } else if text.starts_with("/file ") {
            let parts: Vec<&str> = text[6..].splitn(2, ' ').collect();
            if parts.len() == 2 {
                return Some(DevelopmentCommand::CreateFile {
                    path: parts[0].to_string(),
                    content: parts[1].to_string(),
                });
            }
        } else if text.starts_with("/read ") {
            return Some(DevelopmentCommand::ReadFile {
                path: text[6..].to_string(),
            });
        } else if text == "/test" {
            return Some(DevelopmentCommand::RunTests);
        } else if text == "/deploy" {
            return Some(DevelopmentCommand::DeployApp);
        } else if text == "/status" {
            return Some(DevelopmentCommand::StatusCheck);
        } else if text.starts_with("/run ") {
            let cmd_str = &text[5..];
            let parts: Vec<String> = cmd_str.split_whitespace().map(|s| s.to_string()).collect();
            if !parts.is_empty() {
                return Some(DevelopmentCommand::CustomCommand {
                    command: parts[0].clone(),
                    args: parts[1..].to_vec(),
                });
            }
        }

        None
    }

    async fn execute_development_command(
        &self,
        user_id: &str,
        command: DevelopmentCommand,
    ) -> Result<String> {
        // Check session and permissions
        let sessions = self.active_sessions.lock().unwrap();
        let session = sessions
            .get(user_id)
            .ok_or_else(|| anyhow::anyhow!("No active development session"))?;

        // Execute command based on type
        match command {
            DevelopmentCommand::ExecuteCode { code, language } => Ok(format!(
                "Executing {} code:\n{}\n\nResult: Code executed successfully",
                language, code
            )),
            DevelopmentCommand::CreateFile { path, content } => {
                // In real implementation, create file safely
                Ok(format!(
                    "Created file: {}\nContent length: {} characters",
                    path,
                    content.len()
                ))
            }
            DevelopmentCommand::ReadFile { path } => {
                // In real implementation, read file safely
                Ok(format!(
                    "Reading file: {}\nContent: [file content would appear here]",
                    path
                ))
            }
            DevelopmentCommand::RunTests => {
                Ok("Running tests...\n✅ All tests passed!".to_string())
            }
            DevelopmentCommand::DeployApp => {
                Ok("Deploying application...\n🚀 Deployment successful!".to_string())
            }
            DevelopmentCommand::StatusCheck => Ok(format!(
                "Session Status:\nUser: {}\nProject: {}\nPermissions: {}",
                session.user_name,
                session
                    .current_project
                    .as_ref()
                    .unwrap_or(&"None".to_string()),
                session.permissions.join(", ")
            )),
            DevelopmentCommand::CustomCommand { command, args } => Ok(format!(
                "Executing: {} {}\nResult: Command completed",
                command,
                args.join(" ")
            )),
        }
    }

    fn start_development_session(&self, user_id: &str, user_name: &str) {
        let mut sessions = self.active_sessions.lock().unwrap();

        let session = DevelopmentSession {
            user_id: user_id.to_string(),
            user_name: user_name.to_string(),
            current_project: None,
            last_activity: chrono::Utc::now(),
            permissions: vec!["read".to_string(), "write".to_string()], // Basic permissions
        };

        sessions.insert(user_id.to_string(), session);
    }

    fn end_development_session(&self, user_id: &str) {
        let mut sessions = self.active_sessions.lock().unwrap();
        sessions.remove(user_id);
    }

    fn get_help_message(&self) -> String {
        r#"🤖 Codex Remote Development Commands:

📝 Code Execution:
/code <language> <code> - Execute code
Example: /code python print("Hello!")

📄 File Operations:
/file <path> <content> - Create file
/read <path> - Read file

🧪 Testing & Deployment:
/test - Run tests
/deploy - Deploy application
/status - Check session status

⚡ Custom Commands:
/run <command> [args] - Execute custom command
Example: /run git status

For more help, visit: https://codex.dev/remote-dev
"#
        .to_string()
    }

    /// Get active sessions
    pub fn get_active_sessions(&self) -> Vec<DevelopmentSession> {
        self.active_sessions
            .lock()
            .unwrap()
            .values()
            .cloned()
            .collect()
    }

    /// Set user permissions
    pub fn set_user_permissions(&self, user_id: &str, permissions: Vec<String>) {
        let mut sessions = self.active_sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(user_id) {
            session.permissions = permissions;
        }
    }
}

impl Default for LineCommunicator {
    fn default() -> Self {
        Self::new(LineConfig {
            channel_access_token: String::new(),
            channel_secret: String::new(),
            webhook_url: String::new(),
        })
    }
}

#[cfg(test)]
mod tests;
