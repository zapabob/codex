use crate::rpc::*;
use crate::server::OrchestratorServer;
use crate::server::types::TokenBudget;
use std::sync::Arc;
use tokio::sync::RwLock;

impl OrchestratorServer {
    pub(crate) async fn handle_tokens_get_budget(
        request: &RpcRequest,
        token_budget: &Arc<RwLock<TokenBudget>>,
    ) -> RpcResponse {
        let budget = token_budget.read().await;
        RpcResponse {
            id: request.id.clone(),
            result: Some(serde_json::json!({
                "total_budget": budget.total_budget,
                "used": budget.used,
                "remaining": budget.total_budget.saturating_sub(budget.used),
                "warning_threshold": budget.warning_threshold,
            })),
            error: None,
        }
    }
}
