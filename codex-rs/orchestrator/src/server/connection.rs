use crate::audit::AuditLogger;
use crate::auth::AuthManager;
use crate::error_handler::{create_secure_rpc_error, messages};
use crate::input_validation::validate_json_value;
use crate::rate_limit::RateLimiter;
use crate::replay_protection::ReplayProtection;
use crate::rpc::*;
use crate::server::OrchestratorServer;
use crate::server::config::OrchestratorConfig;
use crate::server::types::{
    AgentState, IdempotencyEntry, SessionInfo, TaskInfo, TokenBudget, WriteRequest,
};
use crate::session::SessionManager;
use crate::transport::Connection;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{RwLock, mpsc};

impl OrchestratorServer {
    /// Handle a client connection
    pub(crate) async fn handle_connection(
        conn: &mut dyn Connection,
        auth_manager: &Arc<AuthManager>,
        idempotency_cache: &Arc<RwLock<HashMap<String, IdempotencyEntry>>>,
        write_queue: &mpsc::Sender<WriteRequest>,
        start_time: SystemTime,
        active_agents: &Arc<RwLock<HashMap<String, AgentState>>>,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
        token_budget: &Arc<RwLock<TokenBudget>>,
        active_sessions: &Arc<RwLock<HashMap<String, SessionInfo>>>,
        queue_size: &Arc<RwLock<usize>>,
        config: &OrchestratorConfig,
        rate_limiter: &Arc<RateLimiter>,
        replay_protection: &Arc<ReplayProtection>,
        audit_logger: &Arc<Option<AuditLogger>>,
        session_manager: &Arc<SessionManager>,
    ) -> Result<()> {
        loop {
            // Read request
            let data = conn.read_message().await?;

            // Parse request
            let request: RpcRequest = match serde_json::from_slice(&data) {
                Ok(req) => {
                    // Validate JSON structure to prevent injection attacks
                    if let Err(validation_err) =
                        validate_json_value(&serde_json::to_value(&req).unwrap_or_default())
                    {
                        let error_response = RpcResponse {
                            id: "".to_string(),
                            result: None,
                            error: Some(create_secure_rpc_error(
                                ERROR_INVALID_REQUEST,
                                messages::INVALID_REQUEST,
                                Some(&validation_err),
                            )),
                        };
                        let response_data = serde_json::to_vec(&error_response)?;
                        conn.write_message(&response_data).await?;
                        continue;
                    }
                    req
                }
                Err(e) => {
                    let error_response = RpcResponse {
                        id: "".to_string(),
                        result: None,
                        error: Some(create_secure_rpc_error(
                            ERROR_PARSE,
                            messages::INVALID_REQUEST,
                            Some(&e),
                        )),
                    };
                    let response_data = serde_json::to_vec(&error_response)?;
                    conn.write_message(&response_data).await?;
                    continue;
                }
            };

            // Get client ID for rate limiting
            let client_id = request
                .auth_token
                .as_ref()
                .or(request.api_key.as_ref())
                .map(|s| s.as_str())
                .unwrap_or("anonymous");

            // Check rate limiting
            if let Err(rate_limit_error) = rate_limiter.check(client_id).await {
                if let Some(ref logger) = **audit_logger {
                    let _ = logger
                        .log_security_event(
                            "rate_limit_exceeded".to_string(),
                            serde_json::json!({
                                "client_id": client_id,
                                "error": format!("{}", rate_limit_error)
                            }),
                            None,
                            None,
                        )
                        .await;
                }

                let error_response = RpcResponse {
                    id: request.id.clone(),
                    result: None,
                    error: Some(RpcError {
                        code: 429,
                        message: format!("Rate limit exceeded: {}", rate_limit_error),
                        data: Some(serde_json::json!({ "retry_after": 1 })),
                    }),
                };
                let response_data = serde_json::to_vec(&error_response)?;
                conn.write_message(&response_data).await?;
                continue;
            }

            // Check replay protection
            if let Some(nonce) = request.params.get("nonce").and_then(|v| v.as_str()) {
                let timestamp = SystemTime::now();
                if let Err(replay_error) = replay_protection.verify(nonce, timestamp).await {
                    if let Some(ref logger) = **audit_logger {
                        let _ = logger
                            .log_security_event(
                                "replay_attack".to_string(),
                                serde_json::json!({
                                    "nonce": nonce,
                                    "error": format!("{}", replay_error)
                                }),
                                None,
                                None,
                            )
                            .await;
                    }

                    let error_response = RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: 400,
                            message: format!("Replay attack detected: {}", replay_error),
                            data: None,
                        }),
                    };
                    let response_data = serde_json::to_vec(&error_response)?;
                    conn.write_message(&response_data).await?;
                    continue;
                }
            }

            // Check idempotency cache
            if let Some(idem_key) = &request.idem_key {
                let cache = idempotency_cache.read().await;
                if let Some(entry) = cache.get(idem_key) {
                    if entry.expires_at > SystemTime::now() {
                        let response_data = serde_json::to_vec(&entry.response)?;
                        conn.write_message(&response_data).await?;
                        continue;
                    }
                }
            }

            // Log RPC request
            if let Some(ref logger) = **audit_logger {
                let _ = logger
                    .log_rpc_request(
                        request.id.clone(),
                        request.method.clone(),
                        request
                            .auth_token
                            .as_ref()
                            .or(request.api_key.as_ref())
                            .map(|_| "authenticated".to_string()),
                        request.params.clone(),
                        None,
                    )
                    .await;
            }

            // Verify authentication
            if Self::requires_auth(&request.method) {
                if let Err(auth_error) = Self::verify_auth(auth_manager, &request, None).await {
                    let error_response = RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(auth_error),
                    };
                    let response_data = serde_json::to_vec(&error_response)?;
                    conn.write_message(&response_data).await?;
                    continue;
                }
            }

            // Dispatch request
            let response = if Self::is_write_method(&request.method) {
                Self::dispatch_write_request(request, write_queue, queue_size).await
            } else {
                Self::process_read_request(
                    &request,
                    start_time,
                    active_agents,
                    active_tasks,
                    token_budget,
                    active_sessions,
                    queue_size,
                    config,
                    auth_manager,
                )
                .await
            };

            // Cache response
            if let Some(idem_key) = &request.idem_key {
                let mut cache = idempotency_cache.write().await;
                cache.insert(
                    idem_key.clone(),
                    IdempotencyEntry {
                        response: response.clone(),
                        expires_at: SystemTime::now() + std::time::Duration::from_secs(600),
                    },
                );
            }

            // Send response
            let response_data = serde_json::to_vec(&response)?;
            conn.write_message(&response_data).await?;
        }
    }

    async fn dispatch_write_request(
        request: RpcRequest,
        write_queue: &mpsc::Sender<WriteRequest>,
        queue_size: &Arc<RwLock<usize>>,
    ) -> RpcResponse {
        let (response_tx, response_rx) = tokio::sync::oneshot::channel();
        let write_req = WriteRequest {
            request: request.clone(),
            response_tx,
        };

        match write_queue.try_send(write_req) {
            Ok(_) => {
                {
                    let mut size = queue_size.write().await;
                    *size += 1;
                }
                match response_rx.await {
                    Ok(resp) => {
                        let mut size = queue_size.write().await;
                        *size = size.saturating_sub(1);
                        resp
                    }
                    Err(_) => {
                        let mut size = queue_size.write().await;
                        *size = size.saturating_sub(1);
                        RpcResponse {
                            id: request.id.clone(),
                            result: None,
                            error: Some(RpcError {
                                code: ERROR_INTERNAL,
                                message: "Write queue processing failed".to_string(),
                                data: None,
                            }),
                        }
                    }
                }
            }
            Err(_) => RpcResponse {
                id: request.id.clone(),
                result: None,
                error: Some(RpcError {
                    code: ERROR_BACKPRESSURE,
                    message: "Write queue full".to_string(),
                    data: None,
                }),
            },
        }
    }

    pub(crate) fn is_write_method(method: &str) -> bool {
        matches!(
            method,
            "lock.acquire"
                | "lock.release"
                | "fs.write"
                | "fs.patch"
                | "vcs.commit"
                | "vcs.push"
                | "agent.register"
                | "task.submit"
                | "task.cancel"
                | "tokens.reportUsage"
                | "session.start"
                | "session.end"
                | "blueprint.create"
                | "blueprint.update"
                | "blueprint.approve"
                | "blueprint.reject"
                | "blueprint.export"
                | "blueprint.setMode"
                | "blueprint.addResearch"
        )
    }
}
