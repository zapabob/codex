//! Orchestrator server implementation.
//!
//! The server coordinates all write operations through a single-writer queue
//! and broadcasts events to subscribers.

use anyhow::Result;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info};

use crate::auth::{AuthConfig, AuthManager};
use crate::idempotency::IdempotencyCache;
use crate::protocol::{Envelope, MessageType, ResponseStatus, topics};
use crate::queue::{QueueConfig, SingleWriterQueue, Task, TaskExecutor};
use crate::transport::{TransportConfig, TransportConnection, TransportServer};

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub transport: TransportConfig,
    pub auth: AuthConfig,
    pub queue: QueueConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            transport: TransportConfig::default(),
            auth: AuthConfig::default(),
            queue: QueueConfig::default(),
        }
    }
}

/// Event broadcaster
type EventBroadcaster = broadcast::Sender<Envelope>;

/// Orchestrator server
pub struct OrchestratorServer {
    config: ServerConfig,
    auth: Arc<AuthManager>,
    queue: Arc<SingleWriterQueue>,
    idempotency: Arc<IdempotencyCache>,
    event_tx: EventBroadcaster,
    subscriptions: Arc<DashMap<String, broadcast::Receiver<Envelope>>>,
}

impl OrchestratorServer {
    /// Create a new orchestrator server
    pub async fn new(config: ServerConfig) -> Result<Self> {
        let auth = Arc::new(AuthManager::new(config.auth.clone()).await?);
        let queue = Arc::new(SingleWriterQueue::new(config.queue.clone()));
        let idempotency = Arc::new(IdempotencyCache::default());
        
        // Event broadcaster with capacity of 1000
        let (event_tx, _) = broadcast::channel(1000);
        
        Ok(Self {
            config,
            auth,
            queue,
            idempotency,
            event_tx,
            subscriptions: Arc::new(DashMap::new()),
        })
    }
    
    /// Start the orchestrator server
    pub async fn run(self: Arc<Self>) -> Result<()> {
        info!("Starting orchestrator server");
        
        // Create transport server
        let transport = TransportServer::new(self.config.transport.clone()).await?;
        
        // Spawn task processor
        let processor_handle = {
            let server = Arc::clone(&self);
            tokio::spawn(async move {
                server.process_queue().await;
            })
        };
        
        // Spawn cleanup task
        let cleanup_handle = {
            let server = Arc::clone(&self);
            tokio::spawn(async move {
                server.cleanup_loop().await;
            })
        };
        
        // Accept connections
        loop {
            match transport.accept().await {
                Ok(conn) => {
                    let server = Arc::clone(&self);
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(conn).await {
                            error!("Connection handler error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
    
    /// Handle a client connection
    async fn handle_connection(&self, mut conn: TransportConnection) -> Result<()> {
        debug!("New connection established");
        
        loop {
            match conn.read_message().await {
                Ok(envelope) => {
                    let response = self.handle_message(envelope).await;
                    conn.write_message(&response).await?;
                }
                Err(e) => {
                    debug!("Connection closed: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Handle a single message
    async fn handle_message(&self, envelope: Envelope) -> Envelope {
        debug!("Handling message: op={}, type={:?}", envelope.op, envelope.msg_type);
        
        // Check idempotency
        if let Some(idem_key) = &envelope.idem_key {
            if let Some(cached_response) = self.idempotency.get(idem_key) {
                debug!("Returning cached response for idempotency key: {}", idem_key);
                return Envelope::response_for(
                    &envelope,
                    ResponseStatus::ok(),
                    cached_response,
                );
            }
        }
        
        // Route by operation
        let result = match envelope.op.as_str() {
            // Lock operations
            op if op.starts_with("lock.") => self.handle_lock_op(&envelope).await,
            
            // Status operations
            op if op.starts_with("status.") => self.handle_status_op(&envelope).await,
            
            // File system operations
            op if op.starts_with("fs.") => self.handle_fs_op(&envelope).await,
            
            // VCS operations
            op if op.starts_with("vcs.") => self.handle_vcs_op(&envelope).await,
            
            // Agent operations
            op if op.starts_with("agent.") => self.handle_agent_op(&envelope).await,
            
            // Task operations
            op if op.starts_with("task.") => self.handle_task_op(&envelope).await,
            
            // Token operations
            op if op.starts_with("tokens.") => self.handle_tokens_op(&envelope).await,
            
            // Session operations
            op if op.starts_with("session.") => self.handle_session_op(&envelope).await,
            
            // Pub/Sub operations
            "subscribe" => self.handle_subscribe(&envelope).await,
            "unsubscribe" => self.handle_unsubscribe(&envelope).await,
            
            _ => {
                Err(anyhow::anyhow!("Unknown operation: {}", envelope.op))
            }
        };
        
        // Create response
        let response = match result {
            Ok(body) => {
                let resp = Envelope::response_for(&envelope, ResponseStatus::ok(), body.clone());
                
                // Cache if idempotent
                if let Some(idem_key) = &envelope.idem_key {
                    self.idempotency.put(idem_key.clone(), body);
                }
                
                resp
            }
            Err(e) => {
                error!("Operation failed: {}", e);
                Envelope::response_for(
                    &envelope,
                    ResponseStatus::error(500, e.to_string()),
                    serde_json::json!(null),
                )
            }
        };
        
        response
    }
    
    /// Handle lock operations
    async fn handle_lock_op(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        // TODO: Implement lock operations
        Ok(serde_json::json!({
            "locked": false,
            "owner": null,
        }))
    }
    
    /// Handle status operations
    async fn handle_status_op(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        Ok(serde_json::json!({
            "queue_size": self.queue.size(),
            "idempotency_cache_size": self.idempotency.len(),
        }))
    }
    
    /// Handle file system operations (queued)
    async fn handle_fs_op(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        // Submit to queue for serialized execution
        let status = self.queue.submit(envelope.clone()).await?;
        
        Ok(serde_json::json!({
            "task_id": status.id,
            "position": status.position,
            "total": status.total,
        }))
    }
    
    /// Handle VCS operations (queued)
    async fn handle_vcs_op(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        // Submit to queue for serialized execution
        let status = self.queue.submit(envelope.clone()).await?;
        
        Ok(serde_json::json!({
            "task_id": status.id,
            "position": status.position,
            "total": status.total,
        }))
    }
    
    /// Handle agent operations
    async fn handle_agent_op(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        // TODO: Implement agent operations
        Ok(serde_json::json!({
            "agents": [],
        }))
    }
    
    /// Handle task operations
    async fn handle_task_op(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        // TODO: Implement task operations
        Ok(serde_json::json!({
            "tasks": [],
        }))
    }
    
    /// Handle token operations
    async fn handle_tokens_op(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        // TODO: Implement token operations
        Ok(serde_json::json!({
            "budget": {},
        }))
    }
    
    /// Handle session operations
    async fn handle_session_op(&self, _envelope: &Envelope) -> Result<serde_json::Value> {
        // TODO: Implement session operations
        Ok(serde_json::json!({
            "session_id": "todo",
        }))
    }
    
    /// Handle subscribe operation
    async fn handle_subscribe(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        let topics: Vec<String> = serde_json::from_value(
            envelope.body.get("topics").cloned().unwrap_or_default()
        )?;
        
        // Create subscription for this client
        let rx = self.event_tx.subscribe();
        
        // Store subscription
        if let Some(session) = &envelope.session {
            self.subscriptions.insert(session.clone(), rx);
        }
        
        Ok(serde_json::json!({
            "subscribed": topics,
        }))
    }
    
    /// Handle unsubscribe operation
    async fn handle_unsubscribe(&self, envelope: &Envelope) -> Result<serde_json::Value> {
        if let Some(session) = &envelope.session {
            self.subscriptions.remove(session);
        }
        
        Ok(serde_json::json!({
            "unsubscribed": true,
        }))
    }
    
    /// Broadcast an event to all subscribers
    pub fn broadcast_event(&self, topic: String, data: serde_json::Value) {
        let event = Envelope::new_event(topic, data);
        
        if let Err(e) = self.event_tx.send(event) {
            debug!("Failed to broadcast event: {}", e);
        }
    }
    
    /// Process tasks from the queue
    async fn process_queue(&self) {
        info!("Starting queue processor");
        
        loop {
            if let Some(task) = self.queue.next().await {
                debug!("Processing task: {}", task.id);
                
                // Execute task
                match self.execute_task(task).await {
                    Ok(result) => {
                        debug!("Task completed: {:?}", result);
                    }
                    Err(e) => {
                        error!("Task failed: {}", e);
                    }
                }
            }
        }
    }
    
    /// Execute a task
    async fn execute_task(&self, task: Task) -> Result<serde_json::Value> {
        // TODO: Implement actual task execution
        // For now, just return success
        
        // Broadcast task completion event
        self.broadcast_event(
            topics::TASK_COMPLETED.to_string(),
            serde_json::json!({
                "task_id": task.id,
                "op": task.envelope.op,
            }),
        );
        
        Ok(serde_json::json!({
            "task_id": task.id,
            "status": "completed",
        }))
    }
    
    /// Cleanup loop for expired cache entries
    async fn cleanup_loop(&self) {
        info!("Starting cleanup loop");
        
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            
            debug!("Running cleanup");
            self.idempotency.cleanup();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_server_creation() {
        let config = ServerConfig::default();
        let server = OrchestratorServer::new(config).await.unwrap();
        
        assert_eq!(server.queue.size(), 0);
        assert!(server.idempotency.is_empty());
    }
}
