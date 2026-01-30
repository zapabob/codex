use crate::rpc::*;
use crate::server::OrchestratorServer;
use crate::server::types::TaskInfo;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

impl OrchestratorServer {
    pub(crate) async fn handle_task_submit(
        request: &RpcRequest,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
    ) -> RpcResponse {
        let params: Result<TaskSubmitRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => {
                let mut tasks = active_tasks.write().await;
                tasks.insert(
                    params.task_id.clone(),
                    TaskInfo {
                        task_id: params.task_id.clone(),
                        agent_type: params.agent_type.clone(),
                        status: "pending".to_string(),
                        submitted_at: std::time::SystemTime::now(),
                    },
                );

                RpcResponse {
                    id: request.id.clone(),
                    result: Some(serde_json::json!({
                        "success": true,
                        "task_id": params.task_id,
                    })),
                    error: None,
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }

    pub(crate) async fn handle_task_cancel(
        request: &RpcRequest,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
    ) -> RpcResponse {
        let params: Result<TaskCancelRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => {
                let mut tasks = active_tasks.write().await;
                if let Some(task) = tasks.get_mut(&params.task_id) {
                    task.status = "cancelled".to_string();

                    RpcResponse {
                        id: request.id.clone(),
                        result: Some(serde_json::json!({ "success": true })),
                        error: None,
                    }
                } else {
                    RpcResponse::invalid_params(
                        request.id.clone(),
                        &format!("Task {} not found", params.task_id),
                    )
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }
}
