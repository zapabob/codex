use crate::rpc::*;
use crate::server::OrchestratorServer;
use crate::server::config::OrchestratorConfig;
use codex_core::lock::RepositoryLock;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

impl OrchestratorServer {
    pub(crate) async fn handle_lock_status(
        request: &RpcRequest,
        config: &OrchestratorConfig,
    ) -> RpcResponse {
        let params: Result<LockStatusRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => {
                let repo_path = params.path.unwrap_or_else(|| {
                    config
                        .codex_dir
                        .parent()
                        .unwrap_or_else(|| Path::new("."))
                        .to_path_buf()
                });

                match RepositoryLock::new(&repo_path) {
                    Ok(lock) => match lock.status() {
                        Ok(Some(metadata)) => RpcResponse {
                            id: request.id.clone(),
                            result: Some(serde_json::json!({
                                "locked": true,
                                "holder": format!("PID {}", metadata.pid),
                                "acquired_at": metadata.started_at.to_string(),
                            })),
                            error: None,
                        },
                        Ok(None) => RpcResponse {
                            id: request.id.clone(),
                            result: Some(serde_json::json!({ "locked": false })),
                            error: None,
                        },
                        Err(e) => RpcResponse::internal_error(
                            request.id.clone(),
                            &format!("Failed to check lock status: {e}"),
                        ),
                    },
                    Err(e) => RpcResponse::internal_error(
                        request.id.clone(),
                        &format!("Failed to create lock manager: {e}"),
                    ),
                }
            }
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }

    pub(crate) async fn handle_lock_acquire(
        request: &RpcRequest,
        subscribers: &Arc<RwLock<HashMap<String, Vec<String>>>>,
    ) -> RpcResponse {
        let params: Result<LockAcquireRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => match RepositoryLock::new(params.path.as_path()) {
                Ok(lock) => {
                    if params.force {
                        if let Err(e) = lock.force_remove() {
                            return RpcResponse::internal_error(
                                request.id.clone(),
                                &format!("Failed to force remove lock: {e}"),
                            );
                        }
                    }

                    match lock.acquire(None) {
                        Ok(metadata) => {
                            Self::publish_event(
                                EVENT_LOCK_CHANGED,
                                serde_json::json!({
                                    "locked": true,
                                    "holder": format!("PID {}", metadata.pid),
                                    "path": params.path.to_string_lossy().to_string(),
                                }),
                                subscribers,
                            )
                            .await;

                            RpcResponse {
                                id: request.id.clone(),
                                result: Some(serde_json::json!({
                                    "success": true,
                                    "message": format!("Lock acquired by PID {}", metadata.pid),
                                })),
                                error: None,
                            }
                        }
                        Err(e) => RpcResponse::error(
                            request.id.clone(),
                            ERROR_CONFLICT,
                            &format!("Lock conflict: {e}"),
                        ),
                    }
                }
                Err(e) => RpcResponse::internal_error(
                    request.id.clone(),
                    &format!("Failed to create lock manager: {e}"),
                ),
            },
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }

    pub(crate) async fn handle_lock_release(
        request: &RpcRequest,
        subscribers: &Arc<RwLock<HashMap<String, Vec<String>>>>,
    ) -> RpcResponse {
        let params: Result<LockReleaseRequest, _> = serde_json::from_value(request.params.clone());
        match params {
            Ok(params) => match RepositoryLock::new(params.path.as_path()) {
                Ok(lock) => match lock.release() {
                    Ok(_) => {
                        Self::publish_event(
                            EVENT_LOCK_CHANGED,
                            serde_json::json!({
                                "locked": false,
                                "path": params.path.to_string_lossy().to_string(),
                            }),
                            subscribers,
                        )
                        .await;

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(serde_json::json!({ "success": true })),
                            error: None,
                        }
                    }
                    Err(e) => {
                        let error_msg = e.to_string();
                        let code = if error_msg.contains("Cannot release lock owned by") {
                            ERROR_CONFLICT
                        } else {
                            ERROR_INTERNAL
                        };
                        RpcResponse::error(
                            request.id.clone(),
                            code,
                            &format!("Failed to release lock: {e}"),
                        )
                    }
                },
                Err(e) => RpcResponse::internal_error(
                    request.id.clone(),
                    &format!("Failed to create lock manager: {e}"),
                ),
            },
            Err(e) => {
                RpcResponse::invalid_params(request.id.clone(), &format!("Invalid params: {e}"))
            }
        }
    }
}
