//! Agent Coordination for Parallel QC Analysis
//!
//! This module provides coordination mechanisms for parallel execution
//! of QC agents across multiple worktrees and processes.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};

/// Coordination message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CoordinationMessage {
    /// Task assignment to agent
    TaskAssignment {
        task_id: String,
        agent_type: AgentType,
        priority: TaskPriority,
        payload: serde_json::Value,
    },
    /// Task completion notification
    TaskCompleted {
        task_id: String,
        result: serde_json::Value,
        duration_ms: u64,
    },
    /// Resource request
    ResourceRequest {
        agent_id: String,
        resource_type: ResourceType,
        amount: f64,
    },
    /// Resource allocation response
    ResourceAllocation {
        agent_id: String,
        resource_type: ResourceType,
        allocated: f64,
        available: f64,
    },
    /// Coordination heartbeat
    Heartbeat {
        agent_id: String,
        status: AgentStatus,
        load_factor: f64,
    },
}

/// Agent types for QC coordination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentType {
    StatisticalAnalyzer,
    QuantumOptimizer,
    MathematicalOptimizer,
    CodeReviewer,
    TestGenerator,
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Resource types for coordination
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    CpuCores,
    MemoryMb,
    GpuMemoryMb,
    DiskSpaceGb,
    NetworkBandwidthMbps,
}

/// Agent status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Busy,
    Error,
    Offline,
}

/// Parallel task execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelExecutionResult {
    pub task_id: String,
    pub agent_type: AgentType,
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
    pub resource_usage: HashMap<ResourceType, f64>,
}

/// Agent coordinator for parallel QC execution
pub struct AgentCoordinator {
    /// Message channel for coordination
    message_tx: mpsc::UnboundedSender<CoordinationMessage>,
    message_rx: Arc<Mutex<mpsc::UnboundedReceiver<CoordinationMessage>>>,

    /// Active agents and their status
    agent_status: Arc<Mutex<HashMap<String, AgentStatus>>>,

    /// Resource pools
    resource_pools: Arc<Mutex<HashMap<ResourceType, f64>>>,

    /// Task queue
    task_queue: Arc<Mutex<VecDeque<QueuedTask>>>,

    /// Completed tasks
    completed_tasks: Arc<Mutex<Vec<ParallelExecutionResult>>>,
}

/// Queued task with metadata
#[derive(Debug, Clone)]
struct QueuedTask {
    id: String,
    agent_type: AgentType,
    priority: TaskPriority,
    payload: serde_json::Value,
    submitted_at: std::time::Instant,
    timeout_ms: Option<u64>,
}

impl AgentCoordinator {
    /// Create new agent coordinator
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut resource_pools = HashMap::new();
        // Initialize default resource pools
        resource_pools.insert(ResourceType::CpuCores, 8.0);
        resource_pools.insert(ResourceType::MemoryMb, 8192.0);
        resource_pools.insert(ResourceType::GpuMemoryMb, 4096.0);
        resource_pools.insert(ResourceType::DiskSpaceGb, 100.0);
        resource_pools.insert(ResourceType::NetworkBandwidthMbps, 1000.0);

        Self {
            message_tx: tx,
            message_rx: Arc::new(Mutex::new(rx)),
            agent_status: Arc::new(Mutex::new(HashMap::new())),
            resource_pools: Arc::new(Mutex::new(resource_pools)),
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            completed_tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register an agent with the coordinator
    pub fn register_agent(&self, agent_id: String, agent_type: AgentType) {
        let mut status = self.agent_status.lock().unwrap();
        status.insert(agent_id.clone(), AgentStatus::Idle);

        println!("✅ Registered agent: {} ({:?})", agent_id, agent_type);
    }

    /// Submit task for parallel execution
    pub async fn submit_task(
        &self,
        task_id: String,
        agent_type: AgentType,
        priority: TaskPriority,
        payload: serde_json::Value,
        timeout_ms: Option<u64>,
    ) -> Result<String, String> {
        let task = QueuedTask {
            id: task_id.clone(),
            agent_type,
            priority,
            payload: payload.clone(),
            submitted_at: std::time::Instant::now(),
            timeout_ms,
        };

        {
            let mut queue = self.task_queue.lock().unwrap();
            queue.push_back(task);
        }

        // Send task assignment message
        let message = CoordinationMessage::TaskAssignment {
            task_id: task_id.clone(),
            agent_type,
            priority,
            payload,
        };

        if let Err(e) = self.message_tx.send(message) {
            return Err(format!("Failed to send task assignment: {}", e));
        }

        Ok(task_id)
    }

    /// Execute parallel QC analysis across multiple agents
    pub async fn execute_parallel_qc_analysis(
        &self,
        code_samples: &[String],
        analysis_types: &[AgentType],
        max_concurrent: usize,
    ) -> Result<Vec<ParallelExecutionResult>, String> {
        let mut tasks = Vec::new();
        let mut results = Vec::new();

        // Submit tasks for each code sample and analysis type
        for (sample_idx, code) in code_samples.iter().enumerate() {
            for (type_idx, &analysis_type) in analysis_types.iter().enumerate() {
                let task_id = format!("qc_analysis_{}_{}_{}", sample_idx, type_idx, chrono::Utc::now().timestamp());

                let payload = serde_json::json!({
                    "code": code,
                    "analysis_type": analysis_type,
                    "sample_index": sample_idx
                });

                match self.submit_task(
                    task_id.clone(),
                    analysis_type,
                    TaskPriority::Medium,
                    payload,
                    Some(30000), // 30 second timeout
                ).await {
                    Ok(_) => {
                        tasks.push(task_id);
                    }
                    Err(e) => {
                        println!("⚠️  Failed to submit task: {}", e);
                    }
                }
            }
        }

        // Wait for results with concurrency control
        let mut active_tasks = 0;
        let mut task_iter = tasks.into_iter();

        while active_tasks > 0 || task_iter.size_hint().0 > 0 {
            // Submit new tasks up to concurrency limit
            while active_tasks < max_concurrent {
                if let Some(task_id) = task_iter.next() {
                    active_tasks += 1;
                    println!("🚀 Started task: {} (active: {})", task_id, active_tasks);
                } else {
                    break;
                }
            }

            // Check for completed tasks
            {
                let completed = self.completed_tasks.lock().unwrap();
                for result in completed.iter() {
                    results.push(result.clone());
                    active_tasks = active_tasks.saturating_sub(1);
                    println!("✅ Completed task: {} (remaining active: {})",
                            result.task_id, active_tasks);
                }
                // Clear completed tasks
                self.completed_tasks.lock().unwrap().clear();
            }

            // Small delay to prevent busy waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(results)
    }

    /// Get resource allocation for agent
    pub fn allocate_resources(
        &self,
        agent_id: &str,
        resource_type: ResourceType,
        requested: f64,
    ) -> f64 {
        let mut pools = self.resource_pools.lock().unwrap();

        if let Some(available) = pools.get_mut(&resource_type) {
            let allocated = requested.min(*available);
            *available -= allocated;

            println!("📊 Allocated {:.2} {} to agent {}",
                    allocated, self.resource_type_name(resource_type), agent_id);

            allocated
        } else {
            0.0
        }
    }

    /// Release resources back to pool
    pub fn release_resources(&self, agent_id: &str, resource_type: ResourceType, amount: f64) {
        let mut pools = self.resource_pools.lock().unwrap();

        if let Some(available) = pools.get_mut(&resource_type) {
            *available += amount;

            println!("🔄 Released {:.2} {} from agent {}",
                    amount, self.resource_type_name(resource_type), agent_id);
        }
    }

    /// Get coordinator statistics
    pub fn get_statistics(&self) -> CoordinatorStatistics {
        let status = self.agent_status.lock().unwrap();
        let pools = self.resource_pools.lock().unwrap();
        let queue = self.task_queue.lock().unwrap();
        let completed = self.completed_tasks.lock().unwrap();

        CoordinatorStatistics {
            total_agents: status.len(),
            active_agents: status.values().filter(|&&s| s == AgentStatus::Busy).count(),
            idle_agents: status.values().filter(|&&s| s == AgentStatus::Idle).count(),
            queued_tasks: queue.len(),
            completed_tasks: completed.len(),
            resource_utilization: pools.clone(),
        }
    }

    /// Helper to get resource type name
    fn resource_type_name(&self, resource_type: ResourceType) -> &'static str {
        match resource_type {
            ResourceType::CpuCores => "CPU cores",
            ResourceType::MemoryMb => "MB memory",
            ResourceType::GpuMemoryMb => "MB GPU memory",
            ResourceType::DiskSpaceGb => "GB disk space",
            ResourceType::NetworkBandwidthMbps => "Mbps bandwidth",
        }
    }
}

/// Coordinator statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatorStatistics {
    pub total_agents: usize,
    pub active_agents: usize,
    pub idle_agents: usize,
    pub queued_tasks: usize,
    pub completed_tasks: usize,
    pub resource_utilization: HashMap<ResourceType, f64>,
}

impl Default for AgentCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_registration() {
        let coordinator = AgentCoordinator::new();

        coordinator.register_agent("test_agent".to_string(), AgentType::StatisticalAnalyzer);

        let stats = coordinator.get_statistics();
        assert_eq!(stats.total_agents, 1);
        assert_eq!(stats.idle_agents, 1);
    }

    #[tokio::test]
    async fn test_task_submission() {
        let coordinator = AgentCoordinator::new();

        let payload = serde_json::json!({"test": "data"});
        let result = coordinator.submit_task(
            "test_task".to_string(),
            AgentType::StatisticalAnalyzer,
            TaskPriority::Medium,
            payload,
            Some(5000),
        ).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_task");
    }

    #[tokio::test]
    async fn test_resource_allocation() {
        let coordinator = AgentCoordinator::new();

        let allocated = coordinator.allocate_resources("test_agent", ResourceType::CpuCores, 2.0);
        assert_eq!(allocated, 2.0);

        let stats = coordinator.get_statistics();
        assert_eq!(stats.resource_utilization[&ResourceType::CpuCores], 6.0); // 8.0 - 2.0

        coordinator.release_resources("test_agent", ResourceType::CpuCores, 2.0);
        let stats_after = coordinator.get_statistics();
        assert_eq!(stats_after.resource_utilization[&ResourceType::CpuCores], 8.0);
    }
}
