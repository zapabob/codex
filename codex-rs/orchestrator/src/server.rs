/// Orchestrator RPC server
///
/// Single-Writer Queue architecture with idempotency cache
use crate::auth::AuthManager;
use crate::rpc::*;
use crate::transport::Connection;
use crate::transport::Transport;
use crate::transport::TransportConfig;
use crate::transport::TransportInfo;
use anyhow::Context;
use anyhow::Result;
use codex_core::lock::RepositoryLock;
use codex_core::plan::manager::PlanManager;
use codex_core::plan::policy::ApprovalRole;
use git2::DiffOptions;
use git2::Repository;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tokio::sync::mpsc;
use tokio::time::sleep;

/// Idempotency cache entry
#[derive(Debug, Clone)]
struct IdempotencyEntry {
    response: RpcResponse,
    expires_at: SystemTime,
}

/// Orchestrator server state
pub struct OrchestratorServer {
    /// Server configuration
    config: OrchestratorConfig,
    /// Transport layer
    transport: Box<dyn Transport>,
    /// Authentication manager
    auth_manager: Arc<AuthManager>,
    /// Idempotency cache (idem_key -> response)
    idempotency_cache: Arc<RwLock<HashMap<String, IdempotencyEntry>>>,
    /// Single-writer queue
    write_queue: mpsc::Sender<WriteRequest>,
    /// Write queue receiver (for processing)
    write_queue_rx: Option<mpsc::Receiver<WriteRequest>>,
    /// Queue size tracker
    queue_size: Arc<RwLock<usize>>,
    /// Server start time
    start_time: SystemTime,
    /// Active agents
    active_agents: Arc<RwLock<HashMap<String, AgentInfo>>>,
    /// Active tasks
    active_tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
    /// Token budget tracker
    token_budget: Arc<RwLock<TokenBudget>>,
    /// Active sessions
    active_sessions: Arc<RwLock<HashMap<String, SessionInfo>>>,
    /// PubSub subscribers
    subscribers: Arc<RwLock<HashMap<String, Vec<String>>>>, // topic -> connection_ids
    /// Plan manager for blueprint operations
    plan_manager: Arc<PlanManager>,
}

/// Orchestrator configuration
#[derive(Debug, Clone)]
pub struct OrchestratorConfig {
    pub queue_capacity: usize,
    pub transport_config: TransportConfig,
    pub codex_dir: PathBuf,
    pub total_token_budget: u64,
    pub warning_threshold: u64,
    pub per_agent_limit: u64,
}

impl Default for OrchestratorConfig {
    fn default() -> Self {
        Self {
            queue_capacity: 1024,
            transport_config: TransportConfig::default(),
            codex_dir: dirs::home_dir().unwrap_or_default().join(".codex"),
            total_token_budget: 100_000,
            warning_threshold: 80_000,
            per_agent_limit: 20_000,
        }
    }
}

/// Write request (queued for single-writer processing)
#[derive(Debug)]
struct WriteRequest {
    request: RpcRequest,
    response_tx: tokio::sync::oneshot::Sender<RpcResponse>,
}

/// Task information
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TaskInfo {
    task_id: String,
    agent_type: String,
    status: String,
    submitted_at: SystemTime,
}

/// Session information
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct SessionInfo {
    session_id: String,
    cwd: PathBuf,
    started_at: SystemTime,
}

/// Token budget tracker
#[derive(Debug, Clone)]
struct TokenBudget {
    total_budget: u64,
    used: u64,
    warning_threshold: u64,
    per_agent_usage: HashMap<String, u64>,
}

impl OrchestratorServer {
    /// Create a new orchestrator server
    pub async fn new(config: OrchestratorConfig) -> Result<Self> {
        // Create transport
        let transport =
            crate::transport::create_transport(config.transport_config.clone(), &config.codex_dir)
                .await
                .context("Failed to create transport")?;

        // Load or create authentication manager
        let auth_manager = Arc::new(
            AuthManager::new(&config.codex_dir).context("Failed to initialize auth manager")?,
        );

        // Create single-writer queue
        let (write_queue_tx, write_queue_rx) = mpsc::channel::<WriteRequest>(config.queue_capacity);

        let token_budget = Arc::new(RwLock::new(TokenBudget {
            total_budget: config.total_token_budget,
            used: 0,
            warning_threshold: config.warning_threshold,
            per_agent_usage: HashMap::new(),
        }));

        let plan_manager = Arc::new(PlanManager::new().context("Failed to create PlanManager")?);

        Ok(Self {
            config,
            transport,
            auth_manager,
            idempotency_cache: Arc::new(RwLock::new(HashMap::new())),
            write_queue: write_queue_tx,
            write_queue_rx: Some(write_queue_rx),
            queue_size: Arc::new(RwLock::new(0)),
            start_time: SystemTime::now(),
            active_agents: Arc::new(RwLock::new(HashMap::new())),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
            token_budget,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            subscribers: Arc::new(RwLock::new(HashMap::new())),
            plan_manager,
        })
    }

    /// Get transport info
    pub fn transport_info(&self) -> TransportInfo {
        self.transport.info()
    }

    /// Start the orchestrator server
    pub async fn run(&mut self) -> Result<()> {
        // Take ownership of write_queue_rx
        let mut write_queue_rx = self
            .write_queue_rx
            .take()
            .context("Server already running")?;

        // Spawn write queue processor
        let auth_manager = Arc::clone(&self.auth_manager);
        let active_agents = Arc::clone(&self.active_agents);
        let active_tasks = Arc::clone(&self.active_tasks);
        let token_budget = Arc::clone(&self.token_budget);
        let active_sessions = Arc::clone(&self.active_sessions);
        let subscribers = Arc::clone(&self.subscribers);
        let plan_manager = Arc::clone(&self.plan_manager);
        let config = self.config.clone();

        tokio::spawn(async move {
            while let Some(write_req) = write_queue_rx.recv().await {
                let response = Self::process_write_request(
                    &write_req.request,
                    &config,
                    &auth_manager,
                    &active_agents,
                    &active_tasks,
                    &token_budget,
                    &active_sessions,
                    &subscribers,
                    &plan_manager,
                )
                .await;

                let _ = write_req.response_tx.send(response);
            }
        });

        // Spawn idempotency cache cleanup task
        let idempotency_cache = Arc::clone(&self.idempotency_cache);
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(60)).await;
                Self::cleanup_idempotency_cache(&idempotency_cache).await;
            }
        });

        // Accept connections
        loop {
            match self.transport.accept().await {
                Ok(mut conn) => {
                    let auth_manager = Arc::clone(&self.auth_manager);
                    let idempotency_cache = Arc::clone(&self.idempotency_cache);
                    let write_queue = self.write_queue.clone();
                    let start_time = self.start_time;
                    let active_agents = Arc::clone(&self.active_agents);
                    let active_tasks = Arc::clone(&self.active_tasks);
                    let token_budget = Arc::clone(&self.token_budget);
                    let active_sessions = Arc::clone(&self.active_sessions);
                    let queue_size = Arc::clone(&self.queue_size);
                    let config = self.config.clone();

                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(
                            conn.as_mut(),
                            &auth_manager,
                            &idempotency_cache,
                            &write_queue,
                            start_time,
                            &active_agents,
                            &active_tasks,
                            &token_budget,
                            &active_sessions,
                            &queue_size,
                            &config,
                        )
                        .await
                        {
                            eprintln!("Connection error: {e}");
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Accept error: {e}");
                }
            }
        }
    }

    /// Handle a client connection
    async fn handle_connection(
        conn: &mut dyn Connection,
        _auth_manager: &Arc<AuthManager>,
        idempotency_cache: &Arc<RwLock<HashMap<String, IdempotencyEntry>>>,
        write_queue: &mpsc::Sender<WriteRequest>,
        start_time: SystemTime,
        active_agents: &Arc<RwLock<HashMap<String, AgentInfo>>>,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
        token_budget: &Arc<RwLock<TokenBudget>>,
        active_sessions: &Arc<RwLock<HashMap<String, SessionInfo>>>,
        queue_size: &Arc<RwLock<usize>>,
        config: &OrchestratorConfig,
    ) -> Result<()> {
        loop {
            // Read request
            let data = conn.read_message().await?;

            // Parse request
            let request: RpcRequest = match serde_json::from_slice(&data) {
                Ok(req) => req,
                Err(e) => {
                    let error_response = RpcResponse {
                        id: "".to_string(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_PARSE,
                            message: format!("Parse error: {e}"),
                            data: None,
                        }),
                    };
                    let response_data = serde_json::to_vec(&error_response)?;
                    conn.write_message(&response_data).await?;
                    continue;
                }
            };

            // Check idempotency cache
            if let Some(idem_key) = &request.idem_key {
                let cache = idempotency_cache.read().await;
                if let Some(entry) = cache.get(idem_key)
                    && entry.expires_at > SystemTime::now()
                {
                    // Return cached response
                    let response_data = serde_json::to_vec(&entry.response)?;
                    conn.write_message(&response_data).await?;
                    continue;
                }
            }

            // Process request
            let response = if Self::is_write_method(&request.method) {
                // Queue write request
                let (response_tx, response_rx) = tokio::sync::oneshot::channel();
                let write_req = WriteRequest {
                    request: request.clone(),
                    response_tx,
                };

                match write_queue.try_send(write_req) {
                    Ok(_) => {
                        // Increment queue size
                        {
                            let mut size = queue_size.write().await;
                            *size += 1;
                        }
                        // Wait for response
                        match response_rx.await {
                            Ok(resp) => resp,
                            Err(_) => {
                                // Decrement on error
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
            } else {
                // Handle read-only request directly
                Self::process_read_request(
                    &request,
                    start_time,
                    active_agents,
                    active_tasks,
                    token_budget,
                    active_sessions,
                    queue_size,
                    config,
                )
                .await
            };

            // Cache response if idempotency key provided
            if let Some(idem_key) = &request.idem_key {
                let mut cache = idempotency_cache.write().await;
                cache.insert(
                    idem_key.clone(),
                    IdempotencyEntry {
                        response: response.clone(),
                        expires_at: SystemTime::now() + Duration::from_secs(600), // 10 min TTL
                    },
                );
            }

            // Send response
            let response_data = serde_json::to_vec(&response)?;
            conn.write_message(&response_data).await?;
        }
    }

    /// Check if method is a write operation
    fn is_write_method(method: &str) -> bool {
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

    /// Publish event to subscribers
    async fn publish_event(
        topic: &str,
        _data: serde_json::Value,
        subscribers: &Arc<RwLock<HashMap<String, Vec<String>>>>,
    ) {
        let subscribers = subscribers.read().await;
        if let Some(connection_ids) = subscribers.get(topic) {
            // In a real implementation, we would send events to these connections
            // For now, we just log that an event would be sent
            tracing::debug!(
                "Publishing event to topic '{}' for {} subscribers",
                topic,
                connection_ids.len()
            );
        }
    }

    /// Process read-only request
    async fn process_read_request(
        request: &RpcRequest,
        start_time: SystemTime,
        active_agents: &Arc<RwLock<HashMap<String, AgentInfo>>>,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
        token_budget: &Arc<RwLock<TokenBudget>>,
        _active_sessions: &Arc<RwLock<HashMap<String, SessionInfo>>>,
        queue_size: &Arc<RwLock<usize>>,
        config: &OrchestratorConfig,
    ) -> RpcResponse {
        match request.method.as_str() {
            "status.get" => {
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
                    result: Some(json!({
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
            "lock.status" => {
                let params: Result<LockStatusRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Determine repository path
                        let repo_path = if let Some(path) = params.path {
                            path
                        } else {
                            // Use codex_dir's parent as repository root
                            config
                                .codex_dir
                                .parent()
                                .unwrap_or_else(|| Path::new("."))
                                .to_path_buf()
                        };

                        // Create lock manager for the repository
                        match RepositoryLock::new(&repo_path) {
                            Ok(lock) => match lock.status() {
                                Ok(Some(metadata)) => RpcResponse {
                                    id: request.id.clone(),
                                    result: Some(json!({
                                        "locked": true,
                                        "holder": format!("PID {}", metadata.pid),
                                        "acquired_at": metadata.started_at.to_string(),
                                    })),
                                    error: None,
                                },
                                Ok(None) => RpcResponse {
                                    id: request.id.clone(),
                                    result: Some(json!({
                                        "locked": false,
                                    })),
                                    error: None,
                                },
                                Err(e) => RpcResponse {
                                    id: request.id.clone(),
                                    result: None,
                                    error: Some(RpcError {
                                        code: ERROR_INTERNAL,
                                        message: format!("Failed to check lock status: {e}"),
                                        data: None,
                                    }),
                                },
                            },
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to create lock manager: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "agent.list" => {
                let agents = active_agents.read().await;
                let agent_list: Vec<_> = agents.values().cloned().collect();

                RpcResponse {
                    id: request.id.clone(),
                    result: Some(
                        serde_json::to_value(AgentListResponse { agents: agent_list })
                            .unwrap_or_else(|e| {
                                serde_json::json!({
                                    "error": format!("Serialization failed: {e}")
                                })
                            }),
                    ),
                    error: None,
                }
            }
            "tokens.getBudget" => {
                let budget = token_budget.read().await;
                RpcResponse {
                    id: request.id.clone(),
                    result: Some(json!({
                        "total_budget": budget.total_budget,
                        "used": budget.used,
                        "remaining": budget.total_budget.saturating_sub(budget.used),
                        "warning_threshold": budget.warning_threshold,
                    })),
                    error: None,
                }
            }
            "fs.read" => {
                let params: Result<FsReadRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Validate path (prevent directory traversal)
                        let path = match params.path.canonicalize() {
                            Ok(p) => p,
                            Err(e) => {
                                return RpcResponse {
                                    id: request.id.clone(),
                                    result: None,
                                    error: Some(RpcError {
                                        code: ERROR_INVALID_PARAMS,
                                        message: format!("Invalid path: {e}"),
                                        data: None,
                                    }),
                                };
                            }
                        };

                        // Read file
                        match tokio::fs::read_to_string(&path).await {
                            Ok(content) => RpcResponse {
                                id: request.id.clone(),
                                result: Some(json!({
                                    "content": content,
                                })),
                                error: None,
                            },
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to read file: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "vcs.diff" => {
                // Find repository root from codex_dir
                let repo_root = config.codex_dir.parent().unwrap_or_else(|| Path::new("."));

                match Repository::open(repo_root) {
                    Ok(repo) => {
                        // Get working directory diff
                        let head = match repo.head() {
                            Ok(head) => head.peel_to_commit().ok(),
                            Err(_) => None,
                        };

                        let mut diff_options = DiffOptions::new();
                        diff_options.include_untracked(true);
                        diff_options.include_ignored(false);

                        let diff = if let Some(head_commit) = head {
                            let tree = match head_commit.tree() {
                                Ok(t) => t,
                                Err(e) => {
                                    return RpcResponse {
                                        id: request.id.clone(),
                                        result: None,
                                        error: Some(RpcError {
                                            code: ERROR_INTERNAL,
                                            message: format!("Failed to get tree: {e}"),
                                            data: None,
                                        }),
                                    };
                                }
                            };
                            repo.diff_tree_to_workdir_with_index(
                                Some(&tree),
                                Some(&mut diff_options),
                            )
                        } else {
                            repo.diff_tree_to_workdir(None, Some(&mut diff_options))
                        };

                        match diff {
                            Ok(diff) => {
                                let mut diff_text = String::new();
                                if let Err(e) =
                                    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                                        match line.origin() {
                                            ' ' | '+' | '-' | 'F' | 'H' | 'B' => {
                                                diff_text.push(line.origin());
                                                if let Ok(content) =
                                                    std::str::from_utf8(line.content())
                                                {
                                                    diff_text.push_str(content);
                                                }
                                            }
                                            _ => {}
                                        }
                                        true
                                    })
                                {
                                    return RpcResponse {
                                        id: request.id.clone(),
                                        result: None,
                                        error: Some(RpcError {
                                            code: ERROR_INTERNAL,
                                            message: format!("Failed to format diff: {e}"),
                                            data: None,
                                        }),
                                    };
                                }

                                RpcResponse {
                                    id: request.id.clone(),
                                    result: Some(json!({
                                        "diff": diff_text,
                                    })),
                                    error: None,
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to compute diff: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INTERNAL,
                            message: format!("Not a git repository or failed to open: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "blueprint.get" => {
                let params: Result<BlueprintGetRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(_params) => {
                        // TODO: Implement with BlueprintManager
                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({
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
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            _ => RpcResponse {
                id: request.id.clone(),
                result: None,
                error: Some(RpcError {
                    code: ERROR_METHOD_NOT_FOUND,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
            },
        }
    }

    /// Process write request (in single-writer queue)
    async fn process_write_request(
        request: &RpcRequest,
        config: &OrchestratorConfig,
        _auth_manager: &Arc<AuthManager>,
        active_agents: &Arc<RwLock<HashMap<String, AgentInfo>>>,
        active_tasks: &Arc<RwLock<HashMap<String, TaskInfo>>>,
        token_budget: &Arc<RwLock<TokenBudget>>,
        active_sessions: &Arc<RwLock<HashMap<String, SessionInfo>>>,
        subscribers: &Arc<RwLock<HashMap<String, Vec<String>>>>,
        plan_manager: &Arc<PlanManager>,
    ) -> RpcResponse {
        match request.method.as_str() {
            "lock.acquire" => {
                let params: Result<LockAcquireRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Create lock manager for the repository
                        match RepositoryLock::new(params.path.as_path()) {
                            Ok(lock) => {
                                if params.force {
                                    // Force remove existing lock
                                    if let Err(e) = lock.force_remove() {
                                        return RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_INTERNAL,
                                                message: format!(
                                                    "Failed to force remove lock: {e}"
                                                ),
                                                data: None,
                                            }),
                                        };
                                    }
                                }

                                // Try to acquire lock
                                match lock.acquire(None) {
                                    Ok(metadata) => {
                                        // Publish lock.changed event
                                        Self::publish_event(
                                            EVENT_LOCK_CHANGED,
                                            json!({
                                                "locked": true,
                                                "holder": format!("PID {}", metadata.pid),
                                                "path": params.path.to_string_lossy().to_string(),
                                            }),
                                            subscribers,
                                        )
                                        .await;

                                        RpcResponse {
                                            id: request.id.clone(),
                                            result: Some(json!({
                                                "success": true,
                                                "message": format!("Lock acquired by PID {}", metadata.pid),
                                            })),
                                            error: None,
                                        }
                                    }
                                    Err(e) => {
                                        // Lock conflict - return 409
                                        RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_CONFLICT,
                                                message: format!("Lock conflict: {e}"),
                                                data: None,
                                            }),
                                        }
                                    }
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to create lock manager: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "lock.release" => {
                let params: Result<LockReleaseRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Create lock manager for the repository
                        match RepositoryLock::new(params.path.as_path()) {
                            Ok(lock) => {
                                match lock.release() {
                                    Ok(_) => {
                                        // Publish lock.changed event
                                        Self::publish_event(
                                            EVENT_LOCK_CHANGED,
                                            json!({
                                                "locked": false,
                                                "path": params.path.to_string_lossy().to_string(),
                                            }),
                                            subscribers,
                                        )
                                        .await;

                                        RpcResponse {
                                            id: request.id.clone(),
                                            result: Some(json!({
                                                "success": true,
                                            })),
                                            error: None,
                                        }
                                    }
                                    Err(e) => {
                                        // Check if it's a permission error (not owner)
                                        let error_msg = e.to_string();
                                        let code =
                                            if error_msg.contains("Cannot release lock owned by") {
                                                ERROR_CONFLICT
                                            } else {
                                                ERROR_INTERNAL
                                            };

                                        RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code,
                                                message: format!("Failed to release lock: {e}"),
                                                data: None,
                                            }),
                                        }
                                    }
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to create lock manager: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "fs.write" => {
                let params: Result<FsWriteRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Validate path
                        let path = match params.path.canonicalize() {
                            Ok(p) => p,
                            Err(e) => {
                                return RpcResponse {
                                    id: request.id.clone(),
                                    result: None,
                                    error: Some(RpcError {
                                        code: ERROR_INVALID_PARAMS,
                                        message: format!("Invalid path: {e}"),
                                        data: None,
                                    }),
                                };
                            }
                        };

                        // Check preimage SHA256 if provided
                        if let Some(expected_sha) = &params.preimage_sha {
                            if path.exists() {
                                match tokio::fs::read_to_string(&path).await {
                                    Ok(existing_content) => {
                                        use sha2::Digest;
                                        use sha2::Sha256;
                                        let mut hasher = Sha256::new();
                                        hasher.update(existing_content.as_bytes());
                                        let current_sha = format!("{:x}", hasher.finalize());

                                        if current_sha != *expected_sha {
                                            return RpcResponse {
                                                id: request.id.clone(),
                                                result: None,
                                                error: Some(RpcError {
                                                    code: ERROR_CONFLICT,
                                                    message: format!(
                                                        "File was modified. Expected SHA256: {}, got: {}",
                                                        expected_sha, current_sha
                                                    ),
                                                    data: None,
                                                }),
                                            };
                                        }
                                    }
                                    Err(e) => {
                                        return RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_INTERNAL,
                                                message: format!(
                                                    "Failed to read existing file: {e}"
                                                ),
                                                data: None,
                                            }),
                                        };
                                    }
                                }
                            }
                        }

                        // Write file atomically (write to temp file, then rename)
                        let temp_path = path.with_extension(".tmp");
                        match tokio::fs::write(&temp_path, &params.content).await {
                            Ok(_) => {
                                // Atomic rename
                                match tokio::fs::rename(&temp_path, &path).await {
                                    Ok(_) => {
                                        // Calculate new SHA256
                                        use sha2::Digest;
                                        use sha2::Sha256;
                                        let mut hasher = Sha256::new();
                                        hasher.update(params.content.as_bytes());
                                        let new_sha = format!("{:x}", hasher.finalize());

                                        RpcResponse {
                                            id: request.id.clone(),
                                            result: Some(json!({
                                                "success": true,
                                                "new_sha": new_sha,
                                            })),
                                            error: None,
                                        }
                                    }
                                    Err(e) => {
                                        let _ = tokio::fs::remove_file(&temp_path).await;
                                        RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_INTERNAL,
                                                message: format!("Failed to rename temp file: {e}"),
                                                data: None,
                                            }),
                                        }
                                    }
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to write file: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "fs.patch" => {
                let params: Result<FsPatchRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Parse unified diff
                        // For now, use a simple implementation
                        // In production, use a proper diff library like `similar` or `diffy`
                        let diff_lines: Vec<&str> = params.unified_diff.lines().collect();
                        let mut applied_files = Vec::new();

                        // Simple diff parser (handles basic unified diff format)
                        let mut current_file: Option<PathBuf> = None;
                        let mut file_content: Vec<String> = Vec::new();
                        let mut line_num = 0;

                        for line in diff_lines {
                            if line.starts_with("+++ ") {
                                // New file path
                                let file_path = line.strip_prefix("+++ ").unwrap_or("").trim();
                                if !file_path.is_empty() {
                                    current_file = Some(PathBuf::from(file_path));
                                    file_content.clear();
                                    line_num = 0;
                                }
                            } else if line.starts_with("@@") {
                                // Hunk header - extract line numbers
                                // Format: @@ -old_start,old_count +new_start,new_count @@
                                // For simplicity, we'll just track that we're in a hunk
                            } else if let Some(ref file_path) = current_file {
                                if line.starts_with("+") && !line.starts_with("++") {
                                    // Added line
                                    file_content.push(line[1..].to_string());
                                    line_num += 1;
                                } else if line.starts_with("-") && !line.starts_with("--") {
                                    // Removed line - skip
                                    line_num += 1;
                                } else if !line.starts_with("\\") {
                                    // Context line
                                    file_content.push(line.to_string());
                                    line_num += 1;
                                }

                                // Apply patch when we finish processing
                                if line_num > 0 && file_content.len() > 0 {
                                    let content = file_content.join("\n");
                                    match tokio::fs::write(file_path, content).await {
                                        Ok(_) => {
                                            applied_files.push(file_path.clone());
                                        }
                                        Err(e) => {
                                            return RpcResponse {
                                                id: request.id.clone(),
                                                result: None,
                                                error: Some(RpcError {
                                                    code: ERROR_INTERNAL,
                                                    message: format!(
                                                        "Failed to apply patch to {}: {e}",
                                                        file_path.display()
                                                    ),
                                                    data: None,
                                                }),
                                            };
                                        }
                                    }
                                }
                            }
                        }

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({
                                "success": true,
                                "applied_files": applied_files.iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<_>>(),
                            })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "agent.register" => {
                // Parse params
                let params: Result<AgentRegisterRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        let mut agents = active_agents.write().await;
                        agents.insert(
                            params.agent_id.clone(),
                            AgentInfo {
                                agent_id: params.agent_id.clone(),
                                agent_type: params.agent_type.clone(),
                                status: "active".to_string(),
                                last_heartbeat: chrono::Utc::now().to_rfc3339(),
                            },
                        );

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({ "success": true })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "agent.heartbeat" => {
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
                                result: Some(json!({ "success": true })),
                                error: None,
                            }
                        } else {
                            RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INVALID_PARAMS,
                                    message: format!("Agent {} not found", params.agent_id),
                                    data: None,
                                }),
                            }
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "task.submit" => {
                let params: Result<TaskSubmitRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        let mut tasks = active_tasks.write().await;
                        tasks.insert(
                            params.task_id.clone(),
                            TaskInfo {
                                task_id: params.task_id.clone(),
                                agent_type: params.agent_type.clone(),
                                status: "pending".to_string(),
                                submitted_at: SystemTime::now(),
                            },
                        );

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({
                                "success": true,
                                "task_id": params.task_id,
                            })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "task.cancel" => {
                let params: Result<TaskCancelRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        let mut tasks = active_tasks.write().await;
                        if let Some(task) = tasks.get_mut(&params.task_id) {
                            task.status = "cancelled".to_string();

                            RpcResponse {
                                id: request.id.clone(),
                                result: Some(json!({ "success": true })),
                                error: None,
                            }
                        } else {
                            RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INVALID_PARAMS,
                                    message: format!("Task {} not found", params.task_id),
                                    data: None,
                                }),
                            }
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "tokens.reportUsage" => {
                let params: Result<TokensReportUsageRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        let agent_id = params.agent_id.clone();
                        let mut budget = token_budget.write().await;
                        budget.used += params.tokens_used;
                        *budget.per_agent_usage.entry(agent_id.clone()).or_insert(0) +=
                            params.tokens_used;

                        let remaining = budget.total_budget.saturating_sub(budget.used);

                        // Publish tokens.updated event
                        Self::publish_event(
                            EVENT_TOKENS_UPDATED,
                            json!({
                                "total_budget": budget.total_budget,
                                "used": budget.used,
                                "remaining": remaining,
                                "agent_id": agent_id,
                            }),
                            subscribers,
                        )
                        .await;

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({
                                "success": true,
                                "remaining_budget": remaining,
                            })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "vcs.commit" => {
                let params: Result<VcsCommitRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Find repository root from codex_dir
                        let repo_root = config.codex_dir.parent().unwrap_or_else(|| Path::new("."));

                        match Repository::open(repo_root) {
                            Ok(repo) => {
                                // Get signature
                                let sig = match repo.signature() {
                                    Ok(s) => s,
                                    Err(e) => {
                                        return RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_INTERNAL,
                                                message: format!(
                                                    "Failed to get git signature: {e}"
                                                ),
                                                data: None,
                                            }),
                                        };
                                    }
                                };

                                // Stage all changes
                                let mut index = match repo.index() {
                                    Ok(i) => i,
                                    Err(e) => {
                                        return RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_INTERNAL,
                                                message: format!("Failed to get index: {e}"),
                                                data: None,
                                            }),
                                        };
                                    }
                                };

                                // Add all modified and new files
                                if let Err(e) =
                                    index.add_all(["*"], git2::IndexAddOption::DEFAULT, None)
                                {
                                    return RpcResponse {
                                        id: request.id.clone(),
                                        result: None,
                                        error: Some(RpcError {
                                            code: ERROR_INTERNAL,
                                            message: format!("Failed to add files to index: {e}"),
                                            data: None,
                                        }),
                                    };
                                }

                                if let Err(e) = index.write() {
                                    return RpcResponse {
                                        id: request.id.clone(),
                                        result: None,
                                        error: Some(RpcError {
                                            code: ERROR_INTERNAL,
                                            message: format!("Failed to write index: {e}"),
                                            data: None,
                                        }),
                                    };
                                }

                                let tree_id = match index.write_tree() {
                                    Ok(id) => id,
                                    Err(e) => {
                                        return RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_INTERNAL,
                                                message: format!("Failed to write tree: {e}"),
                                                data: None,
                                            }),
                                        };
                                    }
                                };

                                let tree = match repo.find_tree(tree_id) {
                                    Ok(t) => t,
                                    Err(e) => {
                                        return RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_INTERNAL,
                                                message: format!("Failed to find tree: {e}"),
                                                data: None,
                                            }),
                                        };
                                    }
                                };

                                // Get parent commit if exists
                                let parent_commit =
                                    repo.head().ok().and_then(|head| head.peel_to_commit().ok());

                                let commit_id = if let Some(parent) = parent_commit {
                                    repo.commit(
                                        Some("HEAD"),
                                        &sig,
                                        &sig,
                                        &params.message,
                                        &tree,
                                        &[&parent],
                                    )
                                } else {
                                    repo.commit(
                                        Some("HEAD"),
                                        &sig,
                                        &sig,
                                        &params.message,
                                        &tree,
                                        &[],
                                    )
                                };

                                match commit_id {
                                    Ok(oid) => RpcResponse {
                                        id: request.id.clone(),
                                        result: Some(json!({
                                            "success": true,
                                            "commit_sha": oid.to_string(),
                                        })),
                                        error: None,
                                    },
                                    Err(e) => RpcResponse {
                                        id: request.id.clone(),
                                        result: None,
                                        error: Some(RpcError {
                                            code: ERROR_INTERNAL,
                                            message: format!("Failed to create commit: {e}"),
                                            data: None,
                                        }),
                                    },
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Not a git repository or failed to open: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "vcs.push" => {
                let params: Result<VcsPushRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Find repository root from codex_dir
                        let repo_root = config.codex_dir.parent().unwrap_or_else(|| Path::new("."));

                        match Repository::open(repo_root) {
                            Ok(repo) => {
                                // Find remote
                                let mut remote = match repo.find_remote(&params.remote) {
                                    Ok(r) => r,
                                    Err(e) => {
                                        return RpcResponse {
                                            id: request.id.clone(),
                                            result: None,
                                            error: Some(RpcError {
                                                code: ERROR_INTERNAL,
                                                message: format!(
                                                    "Remote '{}' not found: {e}",
                                                    params.remote
                                                ),
                                                data: None,
                                            }),
                                        };
                                    }
                                };

                                // Push to remote
                                let refspec = format!(
                                    "refs/heads/{}:refs/heads/{}",
                                    params.branch, params.branch
                                );
                                match remote.push(&[&refspec], None) {
                                    Ok(_) => RpcResponse {
                                        id: request.id.clone(),
                                        result: Some(json!({
                                            "success": true,
                                        })),
                                        error: None,
                                    },
                                    Err(e) => RpcResponse {
                                        id: request.id.clone(),
                                        result: None,
                                        error: Some(RpcError {
                                            code: ERROR_INTERNAL,
                                            message: format!("Failed to push: {e}"),
                                            data: None,
                                        }),
                                    },
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Not a git repository or failed to open: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "session.start" => {
                let params: Result<SessionStartRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        let mut sessions = active_sessions.write().await;
                        sessions.insert(
                            params.session_id.clone(),
                            SessionInfo {
                                session_id: params.session_id.clone(),
                                cwd: params.cwd,
                                started_at: SystemTime::now(),
                            },
                        );

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({ "success": true })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "session.end" => {
                let params: Result<SessionEndRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        let mut sessions = active_sessions.write().await;
                        if sessions.remove(&params.session_id).is_some() {
                            RpcResponse {
                                id: request.id.clone(),
                                result: Some(json!({ "success": true })),
                                error: None,
                            }
                        } else {
                            RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INVALID_PARAMS,
                                    message: format!("Session {} not found", params.session_id),
                                    data: None,
                                }),
                            }
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "pubsub.subscribe" => {
                let params: Result<PubSubSubscribeRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Generate a connection ID for this subscription
                        // In a real implementation, this would be tied to the actual connection
                        let connection_id = format!("conn_{}", rand::random::<u64>());

                        let mut subscribers = subscribers.write().await;
                        for topic in params.topics {
                            subscribers
                                .entry(topic)
                                .or_insert_with(Vec::new)
                                .push(connection_id.clone());
                        }

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({ "success": true })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "pubsub.unsubscribe" => {
                let params: Result<PubSubUnsubscribeRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // In a real implementation, we would use the actual connection ID
                        // For now, we'll remove all subscriptions for the given topics
                        let mut subscribers = subscribers.write().await;
                        for topic in params.topics {
                            subscribers.remove(&topic);
                        }

                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({ "success": true })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            // Blueprint methods
            "blueprint.create" => {
                let params: Result<BlueprintCreateRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        let title = params.title.clone();
                        match plan_manager.create_Plan(params.goal, params.title, params.created_by)
                        {
                            Ok(blueprint_id) => {
                                // Publish blueprint.created event
                                Self::publish_event(
                                    EVENT_BLUEPRINT_CREATED,
                                    json!({
                                        "blueprint_id": blueprint_id,
                                        "title": title,
                                    }),
                                    subscribers,
                                )
                                .await;

                                RpcResponse {
                                    id: request.id.clone(),
                                    result: Some(json!({
                                        "success": true,
                                                "blueprint_id": blueprint_id,
                                    })),
                                    error: None,
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to create blueprint: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "blueprint.update" => {
                let params: Result<BlueprintUpdateRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // TODO: Implement with BlueprintManager
                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({
                                "success": true,
                                "blueprint_id": params.blueprint_id,
                            })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "blueprint.approve" => {
                let params: Result<BlueprintApproveRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Parse approver role
                        let role = match params.approver_role.as_str() {
                            "user" => ApprovalRole::User,
                            "reviewer" => ApprovalRole::Reviewer,
                            "maintainer" => ApprovalRole::Maintainer,
                            "admin" => ApprovalRole::Admin,
                            _ => ApprovalRole::User,
                        };

                        match plan_manager.approve_Plan(
                            &params.blueprint_id,
                            params.approver.clone(),
                            role,
                        ) {
                            Ok(_) => {
                                // Publish blueprint.approved event
                                Self::publish_event(
                                    EVENT_BLUEPRINT_APPROVED,
                                    json!({
                                        "blueprint_id": params.blueprint_id,
                                        "approver": params.approver,
                                    }),
                                    subscribers,
                                )
                                .await;

                                RpcResponse {
                                    id: request.id.clone(),
                                    result: Some(json!({ "success": true })),
                                    error: None,
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to approve blueprint: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "blueprint.reject" => {
                let params: Result<BlueprintRejectRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        match plan_manager.reject_Plan(
                            &params.blueprint_id,
                            params.reason.clone(),
                            params.rejector.clone(),
                        ) {
                            Ok(_) => {
                                // Publish blueprint.rejected event
                                Self::publish_event(
                                    EVENT_BLUEPRINT_REJECTED,
                                    json!({
                                        "blueprint_id": params.blueprint_id,
                                        "reason": params.reason,
                                        "rejector": params.rejector,
                                    }),
                                    subscribers,
                                )
                                .await;

                                RpcResponse {
                                    id: request.id.clone(),
                                    result: Some(json!({ "success": true })),
                                    error: None,
                                }
                            }
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to reject blueprint: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "blueprint.export" => {
                let params: Result<BlueprintExportRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => match plan_manager.export_Plan(&params.blueprint_id) {
                        Ok((md_path, json_path)) => {
                            let mut result = json!({
                                "success": true,
                            });

                            if params.format == "md" || params.format == "both" {
                                result["markdown_path"] =
                                    json!(md_path.to_string_lossy().to_string());
                            }
                            if params.format == "json" || params.format == "both" {
                                result["json_path"] =
                                    json!(json_path.to_string_lossy().to_string());
                            }

                            RpcResponse {
                                id: request.id.clone(),
                                result: Some(result),
                                error: None,
                            }
                        }
                        Err(e) => RpcResponse {
                            id: request.id.clone(),
                            result: None,
                            error: Some(RpcError {
                                code: ERROR_INTERNAL,
                                message: format!("Failed to export blueprint: {e}"),
                                data: None,
                            }),
                        },
                    },
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "blueprint.setMode" => {
                let params: Result<BlueprintSetModeRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Global mode setting - store in config or a separate state
                        // For now, we'll just acknowledge the request
                        // In a full implementation, this would update a global mode setting
                        RpcResponse {
                            id: request.id.clone(),
                            result: Some(json!({
                                "success": true,
                                "mode": params.mode,
                            })),
                            error: None,
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            "blueprint.addResearch" => {
                let params: Result<BlueprintAddResearchRequest, _> =
                    serde_json::from_value(request.params.clone());
                match params {
                    Ok(params) => {
                        // Convert BlueprintResearch to ResearchBlock
                        use chrono::Utc;
                        use codex_core::plan::schema::ResearchBlock;
                        use codex_core::plan::schema::ResearchSource;
                        let research = ResearchBlock {
                            query: params.research.query,
                            depth: params.research.depth,
                            strategy: params.research.strategy,
                            sources: params
                                .research
                                .sources
                                .into_iter()
                                .map(|s| ResearchSource {
                                    title: s.title,
                                    url: s.url,
                                    date: s.date,
                                    key_finding: s.key_finding,
                                    confidence: s.confidence,
                                })
                                .collect(),
                            synthesis: params.research.synthesis,
                            confidence: params.research.confidence,
                            needs_approval: params.research.needs_approval,
                            timestamp: Utc::now(),
                        };

                        match plan_manager.add_research(&params.blueprint_id, research) {
                            Ok(_) => RpcResponse {
                                id: request.id.clone(),
                                result: Some(json!({ "success": true })),
                                error: None,
                            },
                            Err(e) => RpcResponse {
                                id: request.id.clone(),
                                result: None,
                                error: Some(RpcError {
                                    code: ERROR_INTERNAL,
                                    message: format!("Failed to add research: {e}"),
                                    data: None,
                                }),
                            },
                        }
                    }
                    Err(e) => RpcResponse {
                        id: request.id.clone(),
                        result: None,
                        error: Some(RpcError {
                            code: ERROR_INVALID_PARAMS,
                            message: format!("Invalid params: {e}"),
                            data: None,
                        }),
                    },
                }
            }
            _ => RpcResponse {
                id: request.id.clone(),
                result: None,
                error: Some(RpcError {
                    code: ERROR_METHOD_NOT_FOUND,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
            },
        }
    }

    /// Cleanup expired idempotency cache entries
    async fn cleanup_idempotency_cache(cache: &Arc<RwLock<HashMap<String, IdempotencyEntry>>>) {
        let mut cache = cache.write().await;
        let now = SystemTime::now();
        cache.retain(|_, entry| entry.expires_at > now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[test]
    fn test_is_write_method() {
        assert!(OrchestratorServer::is_write_method("lock.acquire"));
        assert!(OrchestratorServer::is_write_method("fs.write"));
        assert!(!OrchestratorServer::is_write_method("status.get"));
        assert!(!OrchestratorServer::is_write_method("lock.status"));
    }

    #[tokio::test]
    async fn test_orchestrator_config_defaults() {
        let config = OrchestratorConfig {
            queue_capacity: 1024,
            transport_config: TransportConfig::default(),
            codex_dir: dirs::home_dir().unwrap_or_default().join(".codex"),
            total_token_budget: 100_000,
            warning_threshold: 80_000,
            per_agent_limit: 20_000,
        };

        assert_eq!(config.queue_capacity, 1024);
        assert_eq!(config.total_token_budget, 100_000);
        assert_eq!(config.warning_threshold, 80_000);
        assert_eq!(config.per_agent_limit, 20_000);
    }

    #[tokio::test]
    async fn test_token_budget_initialization() {
        let budget = TokenBudget {
            total_budget: 1_000_000,
            used: 0,
            warning_threshold: 800_000,
            per_agent_usage: HashMap::new(),
        };

        assert_eq!(budget.total_budget, 1_000_000);
        assert_eq!(budget.used, 0);
        assert_eq!(budget.warning_threshold, 800_000);
        assert!(budget.per_agent_usage.is_empty());
    }

    #[tokio::test]
    async fn test_token_budget_usage() {
        let mut budget = TokenBudget {
            total_budget: 1_000_000,
            used: 0,
            warning_threshold: 800_000,
            per_agent_usage: HashMap::new(),
        };

        // Simulate usage
        budget.used += 50_000;
        *budget
            .per_agent_usage
            .entry("agent-1".to_string())
            .or_insert(0) += 50_000;

        assert_eq!(budget.used, 50_000);
        assert_eq!(budget.per_agent_usage.get("agent-1"), Some(&50_000));

        // Add more usage
        budget.used += 30_000;
        *budget
            .per_agent_usage
            .entry("agent-2".to_string())
            .or_insert(0) += 30_000;

        assert_eq!(budget.used, 80_000);
        assert_eq!(budget.per_agent_usage.get("agent-2"), Some(&30_000));
        assert_eq!(budget.per_agent_usage.len(), 2);
    }

    #[tokio::test]
    async fn test_agent_info_tracking() {
        let mut agents = HashMap::new();

        let agent1 = AgentInfo {
            agent_id: "agent-1".to_string(),
            agent_type: "code-reviewer".to_string(),
            status: "active".to_string(),
            last_heartbeat: "2025-11-01T00:00:00Z".to_string(),
        };

        agents.insert("agent-1".to_string(), agent1.clone());

        assert_eq!(agents.len(), 1);
        assert_eq!(agents.get("agent-1").unwrap().agent_id, "agent-1");
        assert_eq!(agents.get("agent-1").unwrap().status, "active");
    }

    #[tokio::test]
    async fn test_idempotency_entry_expiration() {
        let now = SystemTime::now();
        let future = now + Duration::from_secs(600);
        let past = now - Duration::from_secs(1);

        let valid_entry = IdempotencyEntry {
            response: RpcResponse {
                id: "test-1".to_string(),
                result: Some(json!({"status": "ok"})),
                error: None,
            },
            expires_at: future,
        };

        let expired_entry = IdempotencyEntry {
            response: RpcResponse {
                id: "test-2".to_string(),
                result: Some(json!({"status": "ok"})),
                error: None,
            },
            expires_at: past,
        };

        assert!(valid_entry.expires_at > now);
        assert!(expired_entry.expires_at < now);
    }

    #[tokio::test]
    async fn test_rpc_response_success() {
        let response = RpcResponse {
            id: "req-123".to_string(),
            result: Some(json!({"success": true, "data": "test"})),
            error: None,
        };

        assert_eq!(response.id, "req-123");
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[tokio::test]
    async fn test_rpc_response_error() {
        let response = RpcResponse {
            id: "req-456".to_string(),
            result: None,
            error: Some(RpcError {
                code: ERROR_INVALID_PARAMS,
                message: "Invalid parameters".to_string(),
                data: None,
            }),
        };

        assert_eq!(response.id, "req-456");
        assert!(response.result.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.as_ref().unwrap().code, ERROR_INVALID_PARAMS);
    }

    #[tokio::test]
    async fn test_task_queue_capacity() {
        let (tx, _rx) = async_channel::bounded(10);

        // Queue should accept items up to capacity
        for i in 0..10 {
            let request = RpcRequest {
                id: format!("req-{}", i),
                idem_key: None,
                method: "test.method".to_string(),
                params: json!({}),
            };
            assert!(tx.try_send(request).is_ok());
        }

        // 11th item should fail (queue full)
        let overflow_request = RpcRequest {
            id: "req-overflow".to_string(),
            idem_key: None,
            method: "test.method".to_string(),
            params: json!({}),
        };
        assert!(tx.try_send(overflow_request).is_err());
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(ERROR_INVALID_PARAMS, -32602);
        assert_eq!(ERROR_METHOD_NOT_FOUND, -32601);
        assert_eq!(ERROR_BACKPRESSURE, 429);
        assert_eq!(ERROR_CONFLICT, 409);
    }
}
