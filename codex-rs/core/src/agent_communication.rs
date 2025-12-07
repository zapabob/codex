//! Agent Communication Protocol
//!
//! Inter-agent messaging system for centralized and parallel development modes

use crate::agents::secure_message::SecureAgentCommunicator;
use crate::agents::secure_message::SecureAgentMessage;
use crate::agents::secure_message::SecureMetadata;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Agent communication manager
pub struct AgentCommunicationManager {
    /// Communicator instance
    communicator: Arc<SecureAgentCommunicator>,
    /// Message channels for each agent
    agent_channels: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<InterAgentMessage>>>>,
    /// Development mode context
    development_mode: DevelopmentMode,
}

/// Inter-agent message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterAgentMessage {
    /// Task assignment
    TaskAssigned {
        task_id: String,
        agent_name: String,
        task_description: String,
        dependencies: Vec<String>,
    },
    /// Task completion notification
    TaskCompleted {
        task_id: String,
        agent_name: String,
        result: TaskResult,
        artifacts: Vec<String>,
    },
    /// Task status update
    TaskStatusUpdate {
        task_id: String,
        agent_name: String,
        status: TaskStatus,
        progress: f64,
    },
    /// Coordination request
    CoordinationRequest {
        from_agent: String,
        to_agent: String,
        request_type: CoordinationType,
        payload: serde_json::Value,
    },
    /// Coordination response
    CoordinationResponse {
        request_id: String,
        from_agent: String,
        to_agent: String,
        response: CoordinationResult,
    },
    /// Quality control feedback
    QcFeedback {
        task_id: String,
        agent_name: String,
        feedback_type: QcFeedbackType,
        suggestions: Vec<String>,
        metrics: HashMap<String, f64>,
    },
    /// MCP server status
    McpServerStatus {
        server_name: String,
        status: ServerStatus,
        capabilities: Vec<String>,
    },
}

/// Task result types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskResult {
    Success {
        output: String,
        metrics: HashMap<String, f64>,
    },
    Failure {
        error: String,
        suggestions: Vec<String>,
    },
    Partial {
        completed_parts: Vec<String>,
        remaining_parts: Vec<String>,
    },
}

/// Task status types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked {
        reason: String,
        dependencies: Vec<String>,
    },
    Completed,
    Failed {
        error: String,
    },
}

/// Coordination request types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationType {
    CodeReview {
        code: String,
        language: String,
    },
    DependencyAnalysis {
        files: Vec<String>,
    },
    Testing {
        test_type: String,
        scope: Vec<String>,
    },
    Documentation {
        docs_type: String,
        content: String,
    },
    Integration {
        components: Vec<String>,
    },
}

/// Coordination result types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationResult {
    Approved {
        feedback: String,
    },
    Rejected {
        reason: String,
        suggestions: Vec<String>,
    },
    Modified {
        changes: Vec<String>,
    },
    Deferred {
        reason: String,
        deadline: String,
    },
}

/// QC feedback types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QcFeedbackType {
    CodeQuality {
        score: f64,
        issues: Vec<String>,
    },
    Performance {
        improvements: Vec<String>,
        metrics: HashMap<String, f64>,
    },
    Security {
        vulnerabilities: Vec<String>,
        severity: String,
    },
    Architecture {
        suggestions: Vec<String>,
        compliance: f64,
    },
}

/// Server status types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error { message: String },
}

/// Development mode context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevelopmentMode {
    Centralized,
    Parallel { worktree_base: String },
}

impl AgentCommunicationManager {
    /// Create new communication manager
    pub fn new(development_mode: DevelopmentMode) -> Self {
        Self {
            communicator: Arc::new(SecureAgentCommunicator::new()),
            agent_channels: Arc::new(Mutex::new(HashMap::new())),
            development_mode,
        }
    }

    /// Register agent for communication
    pub async fn register_agent(
        &self,
        agent_name: String,
        message_handler: mpsc::UnboundedSender<InterAgentMessage>,
    ) -> Result<(), String> {
        let mut channels = self.agent_channels.lock().await;
        channels.insert(agent_name, message_handler);
        Ok(())
    }

    /// Send message to specific agent
    pub async fn send_to_agent(
        &self,
        to_agent: &str,
        message: InterAgentMessage,
    ) -> Result<(), String> {
        let channels = self.agent_channels.lock().await;

        if let Some(channel) = channels.get(to_agent) {
            channel
                .send(message)
                .map_err(|e| format!("Failed to send message to {}: {}", to_agent, e))?;
            Ok(())
        } else {
            Err(format!("Agent {} not registered", to_agent))
        }
    }

    /// Broadcast message to all agents
    pub async fn broadcast_to_agents(&self, message: InterAgentMessage) -> Result<(), String> {
        let channels = self.agent_channels.lock().await;

        for (agent_name, channel) in channels.iter() {
            if let Err(e) = channel.send(message.clone()) {
                eprintln!("Failed to send broadcast to {}: {}", agent_name, e);
            }
        }

        Ok(())
    }

    /// Send secure message through encrypted channel
    pub async fn send_secure_message(
        &self,
        message: InterAgentMessage,
        from_agent: &str,
        to_agent: Option<&str>,
    ) -> Result<String, String> {
        let metadata = SecureMetadata {
            message_id: Uuid::new_v4().to_string(),
            message_type: "inter_agent".to_string(),
            priority: 1,
            ttl: 3600, // 1 hour
            requires_ack: true,
            custom_fields: {
                let mut fields = HashMap::new();
                fields.insert(
                    "development_mode".to_string(),
                    serde_json::to_string(&self.development_mode).unwrap_or_default(),
                );
                fields.insert("from_agent".to_string(), from_agent.to_string());
                if let Some(to) = to_agent {
                    fields.insert("to_agent".to_string(), to.to_string());
                }
                fields
            },
        };

        let secure_message = SecureAgentMessage {
            from: from_agent.to_string(),
            to: to_agent.map(|s| s.to_string()),
            encrypted_content: serde_json::to_vec(&message)
                .map_err(|e| format!("Failed to serialize message: {}", e))?,
            signature: vec![], // Will be filled by SecureAgentCommunicator
            nonce: 0,          // Will be filled by SecureAgentCommunicator
            timestamp: chrono::Utc::now().to_rfc3339(),
            metadata,
        };

        self.communicator.send_secure_message(secure_message).await
    }

    /// Receive secure message
    pub async fn receive_secure_message(&self) -> Result<InterAgentMessage, String> {
        let secure_message = self.communicator.receive_secure_message().await?;

        let message: InterAgentMessage = serde_json::from_slice(&secure_message.encrypted_content)
            .map_err(|e| format!("Failed to deserialize message: {}", e))?;

        Ok(message)
    }

    /// Get communication statistics
    pub async fn get_stats(&self) -> CommunicationStats {
        CommunicationStats {
            registered_agents: self.agent_channels.lock().await.len(),
            development_mode: self.development_mode.clone(),
            // Add more stats as needed
        }
    }

    /// Handle task coordination
    pub async fn coordinate_task(
        &self,
        task_id: &str,
        coordinating_agent: &str,
        target_agents: &[String],
    ) -> Result<(), String> {
        // Send coordination request to target agents
        for agent in target_agents {
            let coord_request = InterAgentMessage::CoordinationRequest {
                from_agent: coordinating_agent.to_string(),
                to_agent: agent.clone(),
                request_type: CoordinationType::CodeReview {
                    code: "// Coordination request for task".to_string(),
                    language: "rust".to_string(),
                },
                payload: serde_json::json!({
                    "task_id": task_id,
                    "coordinating_agent": coordinating_agent
                }),
            };

            self.send_to_agent(agent, coord_request).await?;
        }

        Ok(())
    }

    /// Handle parallel development merge coordination
    pub async fn coordinate_parallel_merge(
        &self,
        task_id: &str,
        worktree_results: HashMap<String, String>,
    ) -> Result<(), String> {
        // Send merge coordination to all agents
        let merge_message = InterAgentMessage::QcFeedback {
            task_id: task_id.to_string(),
            agent_name: "coordinator".to_string(),
            feedback_type: QcFeedbackType::Architecture {
                suggestions: vec![
                    "Review parallel worktree results".to_string(),
                    "Resolve merge conflicts".to_string(),
                    "Validate integrated solution".to_string(),
                ],
                compliance: 0.85,
            },
            suggestions: vec![
                "Use best practices from each worktree".to_string(),
                "Ensure code consistency across worktrees".to_string(),
            ],
            metrics: {
                let mut metrics = HashMap::new();
                metrics.insert("worktrees_count".to_string(), worktree_results.len() as f64);
                metrics.insert(
                    "total_lines".to_string(),
                    worktree_results
                        .values()
                        .map(|s| s.lines().count())
                        .sum::<usize>() as f64,
                );
                metrics
            },
        };

        self.broadcast_to_agents(merge_message).await
    }
}

/// Communication statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStats {
    pub registered_agents: usize,
    pub development_mode: DevelopmentMode,
    // Add more fields as needed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_registration() {
        let manager = AgentCommunicationManager::new(DevelopmentMode::Centralized);
        let (tx, _rx) = mpsc::unbounded_channel();

        assert!(
            manager
                .register_agent("test-agent".to_string(), tx)
                .await
                .is_ok()
        );

        let stats = manager.get_stats().await;
        assert_eq!(stats.registered_agents, 1);
    }

    #[tokio::test]
    async fn test_message_sending() {
        let manager = AgentCommunicationManager::new(DevelopmentMode::Centralized);
        let (tx, rx) = mpsc::unbounded_channel();

        manager
            .register_agent("test-agent".to_string(), tx)
            .await
            .unwrap();

        let message = InterAgentMessage::TaskStatusUpdate {
            task_id: "test-task".to_string(),
            agent_name: "test-agent".to_string(),
            status: TaskStatus::InProgress,
            progress: 0.5,
        };

        manager
            .send_to_agent("test-agent", message.clone())
            .await
            .unwrap();

        // Verify message was received
        if let Ok(received) = rx.try_recv() {
            match received {
                InterAgentMessage::TaskStatusUpdate { task_id, .. } => {
                    assert_eq!(task_id, "test-task");
                }
                _ => panic!("Unexpected message type"),
            }
        } else {
            panic!("Message not received");
        }
    }
}
