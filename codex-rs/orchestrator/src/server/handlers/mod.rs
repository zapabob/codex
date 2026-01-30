use super::types::{AgentState, SessionInfo, TaskInfo, TokenBudget};
use crate::auth::AuthManager;
use crate::rpc::{RpcRequest, RpcResponse};
use crate::server::OrchestratorServer;
use crate::server::config::OrchestratorConfig;
use codex_core::plan::manager::PlanManager;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

pub mod agent;
pub mod blueprint;
pub mod fs;
pub mod lock;
pub mod task;
pub mod tokens;
pub mod vcs;

impl OrchestratorServer {
    /// Process read-only request
    pub(crate) async fn process_read_request(
        request: &RpcRequest,
        start_time: SystemTime,
        active_agents: &Arc<RwLock<HashMap<String, AgentState>>>,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
        token_budget: &Arc<RwLock<TokenBudget>>,
        _active_sessions: &Arc<RwLock<HashMap<String, SessionInfo>>>,
        queue_size: &Arc<RwLock<usize>>,
        config: &OrchestratorConfig,
        _auth_manager: &Arc<AuthManager>,
    ) -> RpcResponse {
        match request.method.as_str() {
            "status.get" => {
                Self::handle_status_get(
                    request,
                    start_time,
                    active_agents,
                    active_tasks,
                    token_budget,
                    queue_size,
                    config,
                )
                .await
            }
            "lock.status" => Self::handle_lock_status(request, config).await,
            "agent.list" => Self::handle_agent_list(request, active_agents).await,
            "tokens.getBudget" => Self::handle_tokens_get_budget(request, token_budget).await,
            "fs.read" => Self::handle_fs_read(request, config).await,
            "vcs.diff" => Self::handle_vcs_diff(request, config).await,
            "blueprint.get" => Self::handle_blueprint_get(request).await,
            _ => RpcResponse::method_not_found(request.id.clone(), &request.method),
        }
    }

    /// Process write request (in single-writer queue)
    pub(crate) async fn process_write_request(
        request: &RpcRequest,
        config: &OrchestratorConfig,
        _auth_manager: &Arc<AuthManager>,
        active_agents: &Arc<RwLock<HashMap<String, AgentState>>>,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
        _token_budget: &Arc<RwLock<TokenBudget>>,
        _active_sessions: &Arc<RwLock<HashMap<String, SessionInfo>>>,
        subscribers: &Arc<RwLock<HashMap<String, Vec<String>>>>,
        _plan_manager: &Arc<PlanManager>,
    ) -> RpcResponse {
        match request.method.as_str() {
            "lock.acquire" => Self::handle_lock_acquire(request, subscribers).await,
            "lock.release" => Self::handle_lock_release(request, subscribers).await,
            "fs.write" => Self::handle_fs_write(request, config).await,
            "fs.patch" => Self::handle_fs_patch(request).await,
            "agent.register" => Self::handle_agent_register(request, active_agents).await,
            "agent.heartbeat" => Self::handle_agent_heartbeat(request, active_agents).await,
            "task.submit" => Self::handle_task_submit(request, active_tasks).await,
            "task.cancel" => Self::handle_task_cancel(request, active_tasks).await,
            _ => RpcResponse::method_not_found(request.id.clone(), &request.method),
        }
    }

    async fn handle_status_get(
        request: &RpcRequest,
        start_time: SystemTime,
        active_agents: &Arc<RwLock<HashMap<String, AgentState>>>,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
        token_budget: &Arc<RwLock<TokenBudget>>,
        queue_size: &Arc<RwLock<usize>>,
        config: &OrchestratorConfig,
    ) -> RpcResponse {
        let agents = active_agents.read().await;
        let tasks = active_tasks.read().await;
        let budget = token_budget.read().await;
        let size = queue_size.read().await;
        let uptime = SystemTime::now()
            .duration_since(start_time)
            .unwrap_or_default()
            .as_secs();

        RpcResponse {
            id: request.id.clone(),
            result: Some(serde_json::json!({
                "server_version": env!("CARGO_PKG_VERSION"),
                "uptime_seconds": uptime,
                "queue_size": *size,
                "queue_capacity": config.queue_capacity,
                "active_agents": agents.len(),
                "active_tasks": tasks.len(),
                "total_tokens_used": budget.used,
                "total_tokens_budget": budget.total_budget,
            })),
            error: None,
        }
    }
}
