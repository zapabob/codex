use crate::rpc::{
    self, AgentHeartbeatRequest, AgentListResponse, AgentRegisterRequest, RpcRequest, RpcResponse,
};
use crate::server::OrchestratorServer;
use crate::server::types::AgentState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

impl OrchestratorServer {
    pub(crate) async fn handle_agent_list(
        request: &RpcRequest,
        active_agents: &Arc<RwLock<HashMap<String, AgentState>>>,
    ) -> RpcResponse {
        let agents = active_agents.read().await;
        let agent_list: Vec<rpc::AgentInfo> = agents
            .values()
            .map(|s| rpc::AgentInfo {
                agent_id: s.agent_id.clone(),
                agent_type: s.agent_type.clone(),
                status: s.status.clone(),
                last_heartbeat: s.last_heartbeat.clone(),
            })
            .collect();

        RpcResponse {
            id: request.id.clone(),
            result: Some(
                serde_json::to_value(AgentListResponse { agents: agent_list }).unwrap_or_else(
                    |e| serde_json::json!({ "error": format!("Serialization failed: {e}") }),
                ),
            ),
            error: None,
        }
    }

    pub(crate) async fn handle_agent_register(
        request: &RpcRequest,
        active_agents: &Arc<RwLock<HashMap<String, AgentState>>>,
    ) -> RpcResponse {
        let params: Result<AgentRegisterRequest, _> =
            serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => {
                let mut agents = active_agents.write().await;
                agents.insert(
                    params.agent_id.clone(),
                    AgentState {
                        agent_id: params.agent_id.clone(),
                        agent_type: params.agent_type.clone(),
                        status: "active".to_string(),
                        last_heartbeat: chrono::Utc::now().to_rfc3339(),
                    },
                );

                RpcResponse {
                    id: request.id.clone(),
                    result: Some(serde_json::json!({ "success": true })),
                    error: None,
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }

    pub(crate) async fn handle_agent_heartbeat(
        request: &RpcRequest,
        active_agents: &Arc<RwLock<HashMap<String, AgentState>>>,
    ) -> RpcResponse {
        let params: Result<AgentHeartbeatRequest, _> =
            serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => {
                let mut agents = active_agents.write().await;
                if let Some(agent) = agents.get_mut(&params.agent_id) {
                    agent.last_heartbeat = chrono::Utc::now().to_rfc3339();
                    agent.status = "active".to_string();

                    RpcResponse {
                        id: request.id.clone(),
                        result: Some(serde_json::json!({ "success": true })),
                        error: None,
                    }
                } else {
                    RpcResponse::invalid_params(
                        request.id.clone(),
                        &format!("Agent {} not found", params.agent_id),
                    )
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }
}
