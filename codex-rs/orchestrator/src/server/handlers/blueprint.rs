use crate::rpc::*;
use crate::server::OrchestratorServer;

impl OrchestratorServer {
    pub(crate) async fn handle_blueprint_get(request: &RpcRequest) -> RpcResponse {
        let params: Result<BlueprintGetRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(_params) => {
                // TODO: Implement with BlueprintManager
                RpcResponse {
                    id: request.id.clone(),
                    result: Some(serde_json::json!({
                        "blueprint": {
                            "id": "example-bp",
                            "title": "Example Blueprint",
                            "goal": "Demonstrate blueprint structure",
                            "state": "drafting",
                        }
                    })),
                    error: None,
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }
}
