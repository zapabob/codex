use self::types::{AgentState, IdempotencyEntry, SessionInfo, TaskInfo, TokenBudget, WriteRequest};
use crate::audit::AuditLogger;
use crate::auth::AuthManager;
use crate::rate_limit::RateLimiter;
use crate::replay_protection::ReplayProtection;
use crate::rpc::*;
use crate::session::SessionManager;
use crate::transport::{Connection, Transport, TransportInfo};
use codex_core::plan::manager::PlanManager;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::{RwLock, mpsc};

pub mod auth;
pub mod config;
pub mod handlers;
pub mod types;
pub mod utils;

pub use config::OrchestratorConfig;

/// Orchestrator server state
pub struct OrchestratorServer {
    pub(crate) config: OrchestratorConfig,
    pub(crate) transport: Box<dyn Transport>,
    pub(crate) auth_manager: Arc<AuthManager>,
    pub(crate) idempotency_cache: Arc<RwLock<HashMap<String, IdempotencyEntry>>>,
    pub(crate) write_queue: mpsc::Sender<WriteRequest>,
    pub(crate) write_queue_rx: Option<mpsc::Receiver<WriteRequest>>,
    pub(crate) queue_size: Arc<RwLock<usize>>,
    pub(crate) start_time: SystemTime,
    pub(crate) active_agents: Arc<RwLock<HashMap<String, AgentState>>>,
    pub(crate) active_tasks: Arc<RwLock<HashMap<String, TaskInfo>>>,
    pub(crate) token_budget: Arc<RwLock<TokenBudget>>,
    pub(crate) active_sessions: Arc<RwLock<HashMap<String, SessionInfo>>>,
    pub(crate) subscribers: Arc<RwLock<HashMap<String, Vec<String>>>>,
    pub(crate) plan_manager: Arc<PlanManager>,
    pub(crate) rate_limiter: Arc<RateLimiter>,
    pub(crate) replay_protection: Arc<ReplayProtection>,
    pub(crate) audit_logger: Arc<Option<AuditLogger>>,
    pub(crate) session_manager: Arc<SessionManager>,
}

// Redundant types removed, using crate::server::types instead.

impl OrchestratorServer {
    /// Create a new orchestrator server
    pub async fn new(config: OrchestratorConfig) -> anyhow::Result<Self> {
        use crate::transport::create_transport;
        use anyhow::Context;

        // Create transport
        let transport = create_transport(config.transport_config.clone(), &config.codex_dir)
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

        // Initialize rate limiter
        let rate_limit_config = config.rate_limit_config.clone().unwrap_or_default();
        let rate_limiter = Arc::new(RateLimiter::new(rate_limit_config));

        // Initialize replay protection
        let replay_protection_config = config.replay_protection_config.clone().unwrap_or_default();
        let replay_protection = Arc::new(ReplayProtection::new(replay_protection_config));

        // Initialize audit logger
        let audit_logger = if let Some(audit_config) = config.audit_logger_config.clone() {
            Arc::new(Some(
                AuditLogger::new(audit_config)
                    .await
                    .context("Failed to initialize audit logger")?,
            ))
        } else {
            Arc::new(None)
        };

        // Initialize session manager (30 min timeout, 24 hour max lifetime)
        let session_manager = Arc::new(SessionManager::new(1800, 86400));

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
            rate_limiter,
            replay_protection,
            audit_logger,
            session_manager,
        })
    }

    /// Get transport info
    pub fn transport_info(&self) -> TransportInfo {
        self.transport.info()
    }

    /// Start the orchestrator server
    pub async fn run(&mut self) -> anyhow::Result<()> {
        use anyhow::Context;
        use std::time::Duration;
        use tokio::time::sleep;

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
                    let rate_limiter = Arc::clone(&self.rate_limiter);
                    let replay_protection = Arc::clone(&self.replay_protection);
                    let audit_logger = Arc::clone(&self.audit_logger);
                    let session_manager = Arc::clone(&self.session_manager);

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
                            &rate_limiter,
                            &replay_protection,
                            &audit_logger,
                            &session_manager,
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

    async fn cleanup_idempotency_cache(cache: &Arc<RwLock<HashMap<String, IdempotencyEntry>>>) {
        let mut cache = cache.write().await;
        let now = SystemTime::now();
        cache.retain(|_, entry| entry.expires_at > now);
    }
}
