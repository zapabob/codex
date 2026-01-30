use crate::rpc::{RpcRequest, RpcResponse};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct IdempotencyEntry {
    pub response: RpcResponse,
    pub expires_at: SystemTime,
}

#[derive(Debug)]
pub struct WriteRequest {
    pub request: RpcRequest,
    pub response_tx: tokio::sync::oneshot::Sender<RpcResponse>,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub task_id: String,
    pub agent_type: String,
    pub status: String,
    pub submitted_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub cwd: PathBuf,
    pub started_at: SystemTime,
}

#[derive(Debug, Clone)]
pub struct TokenBudget {
    pub total_budget: u64,
    pub used: u64,
    pub warning_threshold: u64,
    pub per_agent_usage: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct AgentState {
    pub agent_id: String,
    pub agent_type: String,
    pub status: String,
    pub last_heartbeat: String, // String ISO 8601
}
