use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::token_tracker::{PairSession, TokenTracker, TokenUsage};
use crate::lock::{LockInfo, RepoLock};

/// Orchestrator status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorStatus {
    pub lock: Option<LockStatus>,
    pub agents: Vec<AgentStatus>,
    pub tokens: TokenStatus,
    pub sessions: Vec<PairSession>,
}

/// Lock status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockStatus {
    pub holder: String,
    pub pid: u32,
    pub hostname: String,
    pub since: u64,
    pub stale: bool,
}

impl From<&LockInfo> for LockStatus {
    fn from(info: &LockInfo) -> Self {
        Self {
            holder: format!("{}@{}", info.pid, info.hostname),
            pid: info.pid,
            hostname: info.hostname.clone(),
            since: info.started_at,
            stale: info.is_stale(),
        }
    }
}

/// Agent status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStatus {
    pub id: String,
    pub name: String,
    pub status: String,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
}

/// Token status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStatus {
    pub used: u64,
    pub budget: u64,
    pub by_agent: Vec<(String, TokenUsage)>,
}

/// Status provider for orchestrator
pub struct StatusProvider {
    token_tracker: Arc<TokenTracker>,
    agents: Arc<RwLock<Vec<AgentStatus>>>,
}

impl StatusProvider {
    pub fn new(token_tracker: Arc<TokenTracker>) -> Self {
        Self {
            token_tracker,
            agents: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get current orchestrator status
    pub async fn get_status(&self, repo_path: &std::path::Path) -> Result<OrchestratorStatus> {
        // Get lock status
        let lock = RepoLock::get_current(repo_path)?
            .map(|info| LockStatus::from(&info));

        // Get agent statuses
        let agents = self.agents.read().await.clone();

        // Get token status
        let tokens = TokenStatus {
            used: self.token_tracker.get_total_usage(),
            budget: self.token_tracker.get_budget().total_budget,
            by_agent: self.token_tracker.get_all_usages(),
        };

        // Get sessions
        let sessions = self.token_tracker.get_sessions();

        Ok(OrchestratorStatus {
            lock,
            agents,
            tokens,
            sessions,
        })
    }

    /// Update agent status
    pub async fn update_agent(&self, agent: AgentStatus) {
        let mut agents = self.agents.write().await;
        if let Some(existing) = agents.iter_mut().find(|a| a.id == agent.id) {
            *existing = agent;
        } else {
            agents.push(agent);
        }
    }

    /// Remove agent
    pub async fn remove_agent(&self, agent_id: &str) {
        let mut agents = self.agents.write().await;
        agents.retain(|a| a.id != agent_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_status_provider() {
        let temp_dir = TempDir::new().unwrap();
        let budget = TokenBudget::default();
        let tracker = Arc::new(TokenTracker::new(budget));
        let provider = StatusProvider::new(tracker.clone());

        // Update agent
        provider
            .update_agent(AgentStatus {
                id: "agent1".to_string(),
                name: "Test Agent".to_string(),
                status: "running".to_string(),
                tasks_completed: 5,
                tasks_failed: 1,
            })
            .await;

        // Get status
        let status = provider.get_status(temp_dir.path()).await.unwrap();
        assert_eq!(status.agents.len(), 1);
        assert_eq!(status.agents[0].id, "agent1");
    }
}
