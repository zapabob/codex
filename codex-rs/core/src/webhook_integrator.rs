//! Webhook Integrator for External Services
//!
//! Integrates with GitHub, Slack, LINE via environment variables
//! Supports automated notifications and bidirectional communication

use crate::Result;
use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use tracing;

/// Webhook service types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WebhookService {
    GitHub,
    Slack,
    Line,
}

/// Webhook event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEvent {
    CommitPushed {
        repo: String,
        branch: String,
        commits: Vec<GitCommit>,
    },
    PullRequestCreated {
        repo: String,
        pr_number: u32,
        title: String,
        author: String,
    },
    PullRequestMerged {
        repo: String,
        pr_number: u32,
        merged_by: String,
    },
    IssueCreated {
        repo: String,
        issue_number: u32,
        title: String,
        author: String,
    },
    TestResults {
        passed: u32,
        failed: u32,
        coverage: f64,
    },
    DeploymentCompleted {
        app_name: String,
        environment: String,
        status: String,
    },
    SecurityAlert {
        alert_type: String,
        severity: String,
        description: String,
    },
}

/// Git commit information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub sha: String,
    pub message: String,
    pub author: String,
    pub timestamp: String,
}

/// Webhook configuration
#[derive(Debug, Clone)]
pub struct WebhookConfig {
    pub service: WebhookService,
    pub url: String,
    pub secret: Option<String>,
    pub enabled: bool,
}

/// Webhook Integrator
pub struct WebhookIntegrator {
    configs: Arc<Mutex<HashMap<WebhookService, WebhookConfig>>>,
    client: Client,
    command_tx: mpsc::UnboundedSender<WebhookCommand>,
    command_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<WebhookCommand>>>>,
}

#[derive(Debug)]
enum WebhookCommand {
    SendEvent {
        service: WebhookService,
        event: WebhookEvent,
        response: oneshot::Sender<Result<()>>,
    },
    ConfigureWebhook {
        config: WebhookConfig,
        response: oneshot::Sender<Result<()>>,
    },
    LoadFromEnvironment {
        response: oneshot::Sender<Result<Vec<WebhookConfig>>>,
    },
    TestWebhook {
        service: WebhookService,
        response: oneshot::Sender<Result<bool>>,
    },
}

impl WebhookIntegrator {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        Self {
            configs: Arc::new(Mutex::new(HashMap::new())),
            client: Client::new(),
            command_tx: tx,
            command_rx: Arc::new(Mutex::new(Some(rx))),
        }
    }

    /// Send webhook event
    pub async fn send_event(&self, service: WebhookService, event: WebhookEvent) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(WebhookCommand::SendEvent {
            service,
            event,
            response: tx,
        })?;

        rx.await?
    }

    /// Configure webhook
    pub async fn configure_webhook(&self, config: WebhookConfig) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(WebhookCommand::ConfigureWebhook {
            config,
            response: tx,
        })?;

        rx.await?
    }

    /// Load webhook configurations from environment variables
    pub async fn load_from_environment(&self) -> Result<Vec<WebhookConfig>> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(WebhookCommand::LoadFromEnvironment { response: tx })?;

        rx.await?
    }

    /// Test webhook connectivity
    pub async fn test_webhook(&self, service: WebhookService) -> Result<bool> {
        let (tx, rx) = oneshot::channel();

        self.command_tx.send(WebhookCommand::TestWebhook {
            service,
            response: tx,
        })?;

        rx.await?
    }

    /// Convenience method for sending commit notifications
    pub async fn notify_commit(
        &self,
        repo: &str,
        branch: &str,
        commits: Vec<GitCommit>,
    ) -> Result<()> {
        let event = WebhookEvent::CommitPushed {
            repo: repo.to_string(),
            branch: branch.to_string(),
            commits,
        };

        // Send to all configured services
        let configs = self.configs.lock()
            .unwrap_or_else(|e| {
                tracing::error!("Failed to acquire configs lock in notify_commit_pushed: {}", e);
                e.into_inner()
            });
        for (service, config) in configs.iter() {
            if config.enabled {
                let _ = self.send_event(*service, event.clone()).await;
            }
        }

        Ok(())
    }

    /// Convenience method for sending PR notifications
    pub async fn notify_pr_created(
        &self,
        repo: &str,
        pr_number: u32,
        title: &str,
        author: &str,
    ) -> Result<()> {
        let event = WebhookEvent::PullRequestCreated {
            repo: repo.to_string(),
            pr_number,
            title: title.to_string(),
            author: author.to_string(),
        };

        let configs = self.configs.lock()
            .unwrap_or_else(|e| {
                tracing::error!("Failed to acquire configs lock in notify_pr_created: {}", e);
                e.into_inner()
            });
        for (service, config) in configs.iter() {
            if config.enabled {
                let _ = self.send_event(*service, event.clone()).await;
            }
        }

        Ok(())
    }

    /// Run the webhook integrator
    pub async fn run(self) -> Result<()> {
        let mut rx = self.command_rx.lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire command_rx lock: {}", e))?
            .take()
            .ok_or_else(|| anyhow::anyhow!("Command receiver already taken"))?;

        while let Some(cmd) = rx.recv().await {
            match cmd {
                WebhookCommand::SendEvent {
                    service,
                    event,
                    response,
                } => {
                    let result = self.send_event_internal(service, event).await;
                    let _ = response.send(result);
                }
                WebhookCommand::ConfigureWebhook { config, response } => {
                    self.configure_webhook_internal(config);
                    let _ = response.send(Ok(()));
                }
                WebhookCommand::LoadFromEnvironment { response } => {
                    let result = self.load_from_environment_internal();
                    let _ = response.send(result);
                }
                WebhookCommand::TestWebhook { service, response } => {
                    let result = self.test_webhook_internal(service).await;
                    let _ = response.send(result);
                }
            }
        }

        Ok(())
    }

    async fn send_event_internal(
        &self,
        service: WebhookService,
        event: WebhookEvent,
    ) -> Result<()> {
        let configs = self.configs.lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire configs lock: {}", e))?;
        let config = configs.get(&service).ok_or("Webhook not configured")?;

        if !config.enabled {
            return Ok(());
        }

        let payload = self.format_payload(service, &event)?;
        let signature = self.generate_signature(&payload, &config.secret)?;

        let response = self
            .client
            .post(&config.url)
            .header("Content-Type", "application/json")
            .header("X-Hub-Signature-256", signature)
            .header("X-Codex-Event", self.get_event_type(&event))
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Webhook failed: {}", response.status()).into());
        }

        Ok(())
    }

    fn configure_webhook_internal(&self, config: WebhookConfig) {
        let mut configs = self.configs.lock()
            .unwrap_or_else(|e| {
                tracing::error!("Failed to acquire configs lock in configure_webhook_internal: {}", e);
                e.into_inner()
            });
        configs.insert(config.service, config);
    }

    fn load_from_environment_internal(&self) -> Result<Vec<WebhookConfig>> {
        let mut configs = Vec::new();

        // GitHub webhook
        if let Ok(url) = env::var("CODEX_GITHUB_WEBHOOK_URL") {
            let secret = env::var("CODEX_GITHUB_WEBHOOK_SECRET").ok();
            let enabled = env::var("CODEX_GITHUB_WEBHOOK_ENABLED")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(true);

            configs.push(WebhookConfig {
                service: WebhookService::GitHub,
                url,
                secret,
                enabled,
            });
        }

        // Slack webhook
        if let Ok(url) = env::var("CODEX_SLACK_WEBHOOK_URL") {
            let enabled = env::var("CODEX_SLACK_WEBHOOK_ENABLED")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(true);

            configs.push(WebhookConfig {
                service: WebhookService::Slack,
                url,
                secret: None,
                enabled,
            });
        }

        // LINE webhook
        if let Ok(url) = env::var("CODEX_LINE_WEBHOOK_URL") {
            let enabled = env::var("CODEX_LINE_WEBHOOK_ENABLED")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(true);

            configs.push(WebhookConfig {
                service: WebhookService::Line,
                url,
                secret: None,
                enabled,
            });
        }

        // Configure all loaded configs
        for config in &configs {
            self.configure_webhook_internal(config.clone());
        }

        Ok(configs)
    }

    async fn test_webhook_internal(&self, service: WebhookService) -> Result<bool> {
        let test_event = WebhookEvent::TestResults {
            passed: 1,
            failed: 0,
            coverage: 1.0,
        };

        match self.send_event_internal(service, test_event).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn format_payload(
        &self,
        service: WebhookService,
        event: &WebhookEvent,
    ) -> Result<serde_json::Value> {
        match service {
            WebhookService::GitHub => self.format_github_payload(event),
            WebhookService::Slack => self.format_slack_payload(event),
            WebhookService::Line => self.format_line_payload(event),
        }
    }

    fn format_github_payload(&self, event: &WebhookEvent) -> Result<serde_json::Value> {
        let payload = match event {
            WebhookEvent::CommitPushed {
                repo,
                branch,
                commits,
            } => {
                serde_json::json!({
                    "event": "push",
                    "repository": {
                        "name": repo,
                        "full_name": repo
                    },
                    "ref": format!("refs/heads/{}", branch),
                    "commits": commits.iter().map(|c| {
                        serde_json::json!({
                            "id": c.sha,
                            "message": c.message,
                            "author": {
                                "name": c.author
                            },
                            "timestamp": c.timestamp
                        })
                    }).collect::<Vec<_>>()
                })
            }
            WebhookEvent::PullRequestCreated {
                repo,
                pr_number,
                title,
                author,
            } => {
                serde_json::json!({
                    "action": "opened",
                    "pull_request": {
                        "number": pr_number,
                        "title": title,
                        "user": {
                            "login": author
                        }
                    },
                    "repository": {
                        "name": repo
                    }
                })
            }
            _ => serde_json::json!({
                "event": "custom",
                "data": event
            }),
        };

        Ok(payload)
    }

    fn format_slack_payload(&self, event: &WebhookEvent) -> Result<serde_json::Value> {
        let (text, color) = match event {
            WebhookEvent::CommitPushed {
                repo,
                branch,
                commits,
            } => (
                format!("🚀 {} commits pushed to {}/{}", commits.len(), repo, branch),
                "good",
            ),
            WebhookEvent::TestResults { passed, failed, .. } => {
                if *failed == 0 {
                    (
                        format!("✅ All tests passed! {} passed, {} failed", passed, failed),
                        "good",
                    )
                } else {
                    (
                        format!("❌ Tests failed: {} passed, {} failed", passed, failed),
                        "danger",
                    )
                }
            }
            WebhookEvent::SecurityAlert {
                severity,
                description,
                ..
            } => (
                format!("🚨 Security Alert [{}]: {}", severity, description),
                "danger",
            ),
            _ => (format!("📢 Codex Event: {:?}", event), "good"),
        };

        let payload = serde_json::json!({
            "attachments": [{
                "color": color,
                "text": text,
                "footer": "Codex AI Development Platform"
            }]
        });

        Ok(payload)
    }

    fn format_line_payload(&self, event: &WebhookEvent) -> Result<serde_json::Value> {
        let message = match event {
            WebhookEvent::CommitPushed {
                repo,
                branch,
                commits,
            } => {
                format!("🚀 {} commits pushed to {}/{}", commits.len(), repo, branch)
            }
            WebhookEvent::TestResults { passed, failed, .. } => {
                if *failed == 0 {
                    format!("✅ All tests passed! {} passed", passed)
                } else {
                    format!("❌ Tests failed: {} failed", failed)
                }
            }
            _ => "📢 Codex Event Notification".to_string(),
        };

        let payload = serde_json::json!({
            "to": "default_user", // Would be set per user
            "messages": [{
                "type": "text",
                "text": message
            }]
        });

        Ok(payload)
    }

    fn generate_signature(
        &self,
        payload: &serde_json::Value,
        secret: &Option<String>,
    ) -> Result<String> {
        match secret {
            Some(secret) => {
                use hmac::Hmac;
                use hmac::Mac;
                use sha2::Sha256;

                let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
                    .map_err(|_| "Invalid secret length")?;

                mac.update(payload.to_string().as_bytes());
                let result = mac.finalize();
                let signature = hex::encode(result.into_bytes());

                Ok(format!("sha256={}", signature))
            }
            None => Ok("".to_string()),
        }
    }

    fn get_event_type(&self, event: &WebhookEvent) -> String {
        match event {
            WebhookEvent::CommitPushed { .. } => "push",
            WebhookEvent::PullRequestCreated { .. } => "pull_request",
            WebhookEvent::PullRequestMerged { .. } => "pull_request",
            WebhookEvent::IssueCreated { .. } => "issues",
            WebhookEvent::TestResults { .. } => "test_results",
            WebhookEvent::DeploymentCompleted { .. } => "deployment",
            WebhookEvent::SecurityAlert { .. } => "security",
        }
        .to_string()
    }
}

impl Default for WebhookIntegrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_webhook_configuration() {
        let integrator = WebhookIntegrator::new();

        let config = WebhookConfig {
            service: WebhookService::Slack,
            url: "https://hooks.slack.com/test".to_string(),
            secret: None,
            enabled: true,
        };

        let result = integrator.configure_webhook(config).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_payload_formatting() {
        let integrator = WebhookIntegrator::new();

        let event = WebhookEvent::TestResults {
            passed: 10,
            failed: 2,
            coverage: 0.85,
        };

        // Test Slack formatting
        let slack_payload = integrator.format_slack_payload(&event).unwrap();
        assert!(slack_payload["attachments"].is_array());

        // Test GitHub formatting
        let github_payload = integrator.format_github_payload(&event).unwrap();
        assert!(github_payload["event"].is_string());
    }
}
