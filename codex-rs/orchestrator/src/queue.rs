//! Single-writer queue for serializing write operations.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use uuid::Uuid;

use crate::protocol::Envelope;

/// Queue capacity configuration
#[derive(Debug, Clone)]
pub struct QueueConfig {
    pub capacity: usize,
    pub retry_after_seconds: u64,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            capacity: 1024,
            retry_after_seconds: 5,
        }
    }
}

/// Task in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub envelope: Envelope,
    pub submitted_at: chrono::DateTime<chrono::Utc>,
}

impl Task {
    pub fn new(envelope: Envelope) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            envelope,
            submitted_at: chrono::Utc::now(),
        }
    }
}

/// Task status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub id: String,
    pub position: usize,
    pub total: usize,
}

/// Single-writer queue
pub struct SingleWriterQueue {
    sender: mpsc::Sender<Task>,
    receiver: Arc<RwLock<mpsc::Receiver<Task>>>,
    config: QueueConfig,
}

impl SingleWriterQueue {
    /// Create a new single-writer queue
    pub fn new(config: QueueConfig) -> Self {
        let (sender, receiver) = mpsc::channel(config.capacity);
        
        Self {
            sender,
            receiver: Arc::new(RwLock::new(receiver)),
            config,
        }
    }
    
    /// Submit a task to the queue
    pub async fn submit(&self, envelope: Envelope) -> Result<TaskStatus> {
        let task = Task::new(envelope);
        
        match self.sender.try_send(task.clone()) {
            Ok(_) => {
                // Task queued successfully
                let position = self.sender.max_capacity() - self.sender.capacity();
                Ok(TaskStatus {
                    id: task.id.clone(),
                    position,
                    total: self.sender.max_capacity(),
                })
            }
            Err(mpsc::error::TrySendError::Full(_)) => {
                // Queue is full - return 429
                anyhow::bail!(
                    "Queue full (capacity: {}), retry after {} seconds",
                    self.config.capacity,
                    self.config.retry_after_seconds
                );
            }
            Err(mpsc::error::TrySendError::Closed(_)) => {
                anyhow::bail!("Queue is closed");
            }
        }
    }
    
    /// Get the next task from the queue
    pub async fn next(&self) -> Option<Task> {
        let mut receiver = self.receiver.write().await;
        receiver.recv().await
    }
    
    /// Get current queue size
    pub fn size(&self) -> usize {
        self.sender.max_capacity() - self.sender.capacity()
    }
    
    /// Check if queue is full
    pub fn is_full(&self) -> bool {
        self.sender.capacity() == 0
    }
}

/// Task executor trait
#[async_trait::async_trait]
pub trait TaskExecutor: Send + Sync {
    async fn execute(&self, task: Task) -> Result<serde_json::Value>;
}

/// Queue processor
pub struct QueueProcessor {
    queue: Arc<SingleWriterQueue>,
    executor: Arc<dyn TaskExecutor>,
}

impl QueueProcessor {
    pub fn new(queue: Arc<SingleWriterQueue>, executor: Arc<dyn TaskExecutor>) -> Self {
        Self { queue, executor }
    }
    
    /// Start processing tasks from the queue
    pub async fn run(&self) {
        loop {
            if let Some(task) = self.queue.next().await {
                match self.executor.execute(task).await {
                    Ok(result) => {
                        tracing::info!("Task completed successfully: {:?}", result);
                    }
                    Err(e) => {
                        tracing::error!("Task execution failed: {}", e);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::Envelope;
    
    #[tokio::test]
    async fn test_queue_submit() {
        let config = QueueConfig {
            capacity: 10,
            retry_after_seconds: 5,
        };
        
        let queue = SingleWriterQueue::new(config);
        
        let envelope = Envelope::new_request(
            "test.op".to_string(),
            serde_json::json!({}),
        );
        
        let status = queue.submit(envelope).await.unwrap();
        assert_eq!(status.position, 1);
    }
    
    #[tokio::test]
    async fn test_queue_overflow() {
        let config = QueueConfig {
            capacity: 2,
            retry_after_seconds: 5,
        };
        
        let queue = SingleWriterQueue::new(config);
        
        // Fill the queue
        for _ in 0..2 {
            let envelope = Envelope::new_request(
                "test.op".to_string(),
                serde_json::json!({}),
            );
            queue.submit(envelope).await.unwrap();
        }
        
        // Next submission should fail
        let envelope = Envelope::new_request(
            "test.op".to_string(),
            serde_json::json!({}),
        );
        
        assert!(queue.submit(envelope).await.is_err());
    }
}
