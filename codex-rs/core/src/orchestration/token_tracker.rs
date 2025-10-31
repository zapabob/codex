use anyhow::Result;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// Token usage information for a specific agent or session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    pub last_updated: u64,
}

impl TokenUsage {
    pub fn new() -> Self {
        Self {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
            last_updated: current_timestamp(),
        }
    }

    pub fn add(&mut self, prompt: u64, completion: u64) {
        self.prompt_tokens += prompt;
        self.completion_tokens += completion;
        self.total_tokens = self.prompt_tokens + self.completion_tokens;
        self.last_updated = current_timestamp();
    }
}

impl Default for TokenUsage {
    fn default() -> Self {
        Self::new()
    }
}

/// Token budget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBudget {
    pub total_budget: u64,
    pub warning_threshold: u64,
    pub per_agent_limit: Option<u64>,
}

impl Default for TokenBudget {
    fn default() -> Self {
        Self {
            total_budget: 100_000,
            warning_threshold: 80_000,
            per_agent_limit: Some(20_000),
        }
    }
}

/// Session information for pair programming or collaborative work
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairSession {
    pub session_id: String,
    pub participants: Vec<String>,
    pub roles: Vec<String>,
    pub started_at: u64,
    pub ended_at: Option<u64>,
    pub notes: String,
}

impl PairSession {
    pub fn new(session_id: String, participants: Vec<String>, roles: Vec<String>) -> Self {
        Self {
            session_id,
            participants,
            roles,
            started_at: current_timestamp(),
            ended_at: None,
            notes: String::new(),
        }
    }

    pub fn end(&mut self) {
        self.ended_at = Some(current_timestamp());
    }
}

/// Centralized token tracker for orchestration
#[derive(Debug, Clone)]
pub struct TokenTracker {
    budget: TokenBudget,
    usage_by_agent: Arc<DashMap<String, TokenUsage>>,
    sessions: Arc<DashMap<String, PairSession>>,
}

impl TokenTracker {
    pub fn new(budget: TokenBudget) -> Self {
        Self {
            budget,
            usage_by_agent: Arc::new(DashMap::new()),
            sessions: Arc::new(DashMap::new()),
        }
    }

    /// Record token usage for an agent
    pub fn record_usage(&self, agent_id: &str, prompt_tokens: u64, completion_tokens: u64) -> Result<()> {
        let mut usage = self.usage_by_agent
            .entry(agent_id.to_string())
            .or_insert_with(TokenUsage::new);
        
        usage.add(prompt_tokens, completion_tokens);
        
        // Check if budget exceeded
        let total_used = self.get_total_usage();
        if total_used > self.budget.total_budget {
            return Err(anyhow::anyhow!(
                "Token budget exceeded: used {} / {}",
                total_used,
                self.budget.total_budget
            ));
        }
        
        // Check warning threshold
        if total_used > self.budget.warning_threshold {
            tracing::warn!(
                "Token usage warning: used {} / {} ({}%)",
                total_used,
                self.budget.total_budget,
                (total_used * 100) / self.budget.total_budget
            );
        }
        
        // Check per-agent limit
        if let Some(per_agent_limit) = self.budget.per_agent_limit {
            if usage.total_tokens > per_agent_limit {
                tracing::warn!(
                    "Agent {} exceeded per-agent limit: used {} / {}",
                    agent_id,
                    usage.total_tokens,
                    per_agent_limit
                );
            }
        }
        
        Ok(())
    }

    /// Get total token usage across all agents
    pub fn get_total_usage(&self) -> u64 {
        self.usage_by_agent
            .iter()
            .map(|entry| entry.value().total_tokens)
            .sum()
    }

    /// Get token usage for a specific agent
    pub fn get_agent_usage(&self, agent_id: &str) -> Option<TokenUsage> {
        self.usage_by_agent.get(agent_id).map(|usage| usage.clone())
    }

    /// Get all agent usages
    pub fn get_all_usages(&self) -> Vec<(String, TokenUsage)> {
        self.usage_by_agent
            .iter()
            .map(|entry| (entry.key().clone(), entry.value().clone()))
            .collect()
    }

    /// Start a new pair programming session
    pub fn start_session(&self, session_id: String, participants: Vec<String>, roles: Vec<String>) {
        let session = PairSession::new(session_id.clone(), participants, roles);
        self.sessions.insert(session_id, session);
    }

    /// End a pair programming session
    pub fn end_session(&self, session_id: &str) -> Result<()> {
        if let Some(mut session) = self.sessions.get_mut(session_id) {
            session.end();
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found: {}", session_id))
        }
    }

    /// Add notes to a session
    pub fn add_session_notes(&self, session_id: &str, notes: &str) -> Result<()> {
        if let Some(mut session) = self.sessions.get_mut(session_id) {
            session.notes.push_str(notes);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found: {}", session_id))
        }
    }

    /// Get all sessions
    pub fn get_sessions(&self) -> Vec<PairSession> {
        self.sessions
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get budget information
    pub fn get_budget(&self) -> &TokenBudget {
        &self.budget
    }

    /// Reset all usage counters
    pub fn reset(&self) {
        self.usage_by_agent.clear();
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_usage() {
        let mut usage = TokenUsage::new();
        usage.add(100, 50);
        assert_eq!(usage.total_tokens, 150);
        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.completion_tokens, 50);
    }

    #[test]
    fn test_token_tracker() {
        let budget = TokenBudget::default();
        let tracker = TokenTracker::new(budget);
        
        tracker.record_usage("agent1", 100, 50).unwrap();
        tracker.record_usage("agent2", 200, 100).unwrap();
        
        assert_eq!(tracker.get_total_usage(), 450);
        
        let usage = tracker.get_agent_usage("agent1").unwrap();
        assert_eq!(usage.total_tokens, 150);
    }

    #[test]
    fn test_budget_exceeded() {
        let budget = TokenBudget {
            total_budget: 500,
            warning_threshold: 400,
            per_agent_limit: None,
        };
        let tracker = TokenTracker::new(budget);
        
        tracker.record_usage("agent1", 300, 100).unwrap();
        let result = tracker.record_usage("agent2", 100, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_pair_session() {
        let budget = TokenBudget::default();
        let tracker = TokenTracker::new(budget);
        
        tracker.start_session(
            "session1".to_string(),
            vec!["user1".to_string(), "user2".to_string()],
            vec!["driver".to_string(), "navigator".to_string()],
        );
        
        tracker.add_session_notes("session1", "Working on feature X").unwrap();
        tracker.end_session("session1").unwrap();
        
        let sessions = tracker.get_sessions();
        assert_eq!(sessions.len(), 1);
        assert!(sessions[0].ended_at.is_some());
    }
}
