//! Selective tool loader for task-based MCP tool loading.
//!
//! This module provides functionality to analyze tasks and load only the necessary
//! MCP tools, reducing token consumption.

use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;
use tracing::info;

use crate::mcp_dynamic_loader::DynamicMcpLoader;
use crate::mcp_token_optimizer::McpTokenOptimizer;

/// Selective tool loader for task-based loading
pub struct SelectiveToolLoader {
    optimizer: Arc<McpTokenOptimizer>,
    dynamic_loader: Arc<DynamicMcpLoader>,
    task_tool_mapping: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl SelectiveToolLoader {
    /// Create a new selective tool loader
    pub fn new(optimizer: Arc<McpTokenOptimizer>, dynamic_loader: Arc<DynamicMcpLoader>) -> Self {
        Self {
            optimizer,
            dynamic_loader,
            task_tool_mapping: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Load tools for a specific task
    pub async fn load_for_task(&self, task_description: &str) -> Result<Vec<String>> {
        info!("Analyzing task requirements: {}", task_description);

        // Analyze task to determine required tools
        let required_tools = self.analyze_task_requirements(task_description).await;

        info!(
            "Task requires {} tools: {:?}",
            required_tools.len(),
            required_tools
        );

        // Generate task ID
        let task_id = format!("task_{}", uuid::Uuid::new_v4());

        // Store mapping
        let mut mapping = self.task_tool_mapping.lock().await;
        mapping.insert(task_id.clone(), required_tools.clone());

        Ok(required_tools)
    }

    /// Unload tools after task completion
    pub async fn unload_after_task(&self, task_id: &str) -> Result<()> {
        let mut mapping = self.task_tool_mapping.lock().await;

        if let Some(tools) = mapping.remove(task_id) {
            info!(
                "Unloading {} tools after task completion: {}",
                tools.len(),
                task_id
            );
            // TODO: Implement actual tool unloading
            // For now, tools remain loaded but are marked as unused
        }

        Ok(())
    }

    /// Analyze task requirements to determine needed tools
    pub async fn analyze_task_requirements(&self, task: &str) -> Vec<String> {
        // Simple keyword-based analysis
        // TODO: Implement more sophisticated analysis using LLM or semantic search

        let task_lower = task.to_lowercase();
        let mut required = Vec::new();

        // Keyword mapping to tool categories
        if task_lower.contains("github")
            || task_lower.contains("git")
            || task_lower.contains("repo")
        {
            required.push("github".to_string());
        }
        if task_lower.contains("search")
            || task_lower.contains("research")
            || task_lower.contains("web")
        {
            required.push("web-search".to_string());
            required.push("deep-research".to_string());
        }
        if task_lower.contains("code")
            || task_lower.contains("analyze")
            || task_lower.contains("review")
        {
            required.push("serena".to_string());
        }
        if task_lower.contains("file")
            || task_lower.contains("read")
            || task_lower.contains("write")
        {
            required.push("filesystem".to_string());
        }

        // If no specific tools identified, return empty (will use default tools)
        if required.is_empty() {
            debug!("No specific tools identified for task, using defaults");
        }

        required
    }

    /// Get tools loaded for a task
    pub async fn get_task_tools(&self, task_id: &str) -> Option<Vec<String>> {
        let mapping = self.task_tool_mapping.lock().await;
        mapping.get(task_id).cloned()
    }
}
