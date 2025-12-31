//! Token optimizer for MCP tools to reduce token consumption.
//!
//! This module provides functionality to track tool usage, automatically unload
//! unused tools, compress tool descriptions, and optimize token usage.

use anyhow::Result;
use mcp_types::Tool;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::debug;
use tracing::info;
use tracing::warn;

use crate::mcp_dynamic_loader::DynamicMcpLoader;

/// Tool usage statistics
#[derive(Debug, Clone)]
pub struct ToolUsageStats {
    pub tool_name: String,
    pub server_name: String,
    pub usage_count: u64,
    pub last_used: SystemTime,
    pub total_tokens_consumed: u64,
    pub average_tokens_per_call: f64,
}

/// Token estimate for a tool
#[derive(Debug, Clone)]
pub struct ToolTokenEstimate {
    pub tool_name: String,
    pub description_tokens: u64,
    pub schema_tokens: u64,
    pub total_tokens: u64,
}

/// Token optimizer for MCP tools
pub struct McpTokenOptimizer {
    tool_usage_stats: Arc<Mutex<HashMap<String, ToolUsageStats>>>,
    auto_unload_threshold: Duration,
    min_usage_count: u64,
    compress_descriptions: bool,
}

impl McpTokenOptimizer {
    /// Create a new token optimizer
    pub fn new(
        auto_unload_threshold: Duration,
        min_usage_count: u64,
        compress_descriptions: bool,
    ) -> Self {
        Self {
            tool_usage_stats: Arc::new(Mutex::new(HashMap::new())),
            auto_unload_threshold,
            min_usage_count,
            compress_descriptions,
        }
    }

    /// Track tool usage
    pub async fn track_tool_usage(&self, tool_name: &str, server_name: &str, tokens: u64) {
        let mut stats = self.tool_usage_stats.lock().await;
        let key = format!("{server_name}__{tool_name}");

        let stat = stats.entry(key).or_insert_with(|| ToolUsageStats {
            tool_name: tool_name.to_string(),
            server_name: server_name.to_string(),
            usage_count: 0,
            last_used: SystemTime::now(),
            total_tokens_consumed: 0,
            average_tokens_per_call: 0.0,
        });

        stat.usage_count += 1;
        stat.last_used = SystemTime::now();
        stat.total_tokens_consumed += tokens;
        stat.average_tokens_per_call = stat.total_tokens_consumed as f64 / stat.usage_count as f64;

        debug!(
            "Tracked tool usage: {} (count: {}, tokens: {})",
            tool_name, stat.usage_count, tokens
        );
    }

    /// Get unused tools that exceed the threshold
    pub async fn get_unused_tools(&self, threshold: Duration) -> Vec<String> {
        let stats = self.tool_usage_stats.lock().await;
        let now = SystemTime::now();
        let mut unused = Vec::new();

        for (key, stat) in stats.iter() {
            if stat.usage_count < self.min_usage_count {
                continue;
            }

            if let Ok(elapsed) = now.duration_since(stat.last_used)
                && elapsed > threshold
            {
                unused.push(key.clone());
            }
        }

        unused
    }

    /// Automatically unload unused tools
    pub async fn auto_unload_unused(&self, _loader: &DynamicMcpLoader) -> Result<()> {
        let unused_tools = self.get_unused_tools(self.auto_unload_threshold).await;

        if unused_tools.is_empty() {
            return Ok(());
        }

        info!("Auto-unloading {} unused tools", unused_tools.len());

        for tool_key in unused_tools {
            // Parse server name from key (format: "server__tool")
            if let Some((server_name, _tool_name)) = tool_key.split_once("__") {
                // Note: This would require tracking which server a tool belongs to
                // For now, we just log the unused tools
                debug!(
                    "Tool {} from server {} is unused and could be unloaded",
                    tool_key, server_name
                );
                // TODO: Implement actual server unloading when tool-to-server mapping is available
            }
        }

        Ok(())
    }

    /// Compress tool description
    pub fn compress_tool_description(&self, tool: &Tool) -> String {
        if !self.compress_descriptions {
            return tool.description.clone().unwrap_or_default();
        }

        let description = tool.description.as_deref().unwrap_or("");

        // Simple compression: keep first sentence and key parameters

        if description.len() > 200 {
            // Take first sentence or first 150 chars
            let first_sentence = description
                .split('.')
                .next()
                .unwrap_or(&description[..description.len().min(150)]);
            format!("{}...", first_sentence.trim())
        } else {
            description.to_string()
        }
    }

    /// Estimate tokens for a list of tools
    pub fn estimate_tokens(&self, tools: &[Tool]) -> u64 {
        // Rough estimation: 1 token ≈ 4 characters
        let mut total_chars = 0u64;

        for tool in tools {
            // Tool name
            total_chars += tool.name.len() as u64;

            // Description (compressed if enabled)
            let desc = if self.compress_descriptions {
                self.compress_tool_description(tool)
            } else {
                tool.description.clone().unwrap_or_default()
            };
            total_chars += desc.len() as u64;

            // Schema (simplified estimate)
            if let Some(props) = tool.input_schema.properties.as_ref() {
                total_chars += serde_json::to_string(props).unwrap_or_default().len() as u64;
            }
        }

        // Convert to token estimate (rough: 1 token ≈ 4 chars)
        total_chars / 4
    }

    /// Select relevant tools for a task
    pub fn select_relevant_tools(&self, task: &str, available_tools: &[Tool]) -> Vec<Tool> {
        // Simple keyword-based selection
        // TODO: Implement more sophisticated selection using LLM or semantic search
        let task_lower = task.to_lowercase();
        let mut relevant = Vec::new();

        for tool in available_tools {
            let tool_name_lower = tool.name.to_lowercase();
            let description_lower = tool.description.as_deref().unwrap_or("").to_lowercase();

            // Check if tool name or description contains task keywords
            if task_lower.contains(&tool_name_lower)
                || tool_name_lower.contains(&task_lower)
                || description_lower.contains(&task_lower)
            {
                relevant.push(tool.clone());
            }
        }

        // If no matches, return all tools (fallback)
        if relevant.is_empty() {
            available_tools.to_vec()
        } else {
            relevant
        }
    }

    /// Start background task for auto-unloading
    pub fn start_auto_unload_task(
        self: Arc<Self>,
        loader: Arc<DynamicMcpLoader>,
        interval: Duration,
    ) {
        tokio::spawn(async move {
            loop {
                sleep(interval).await;

                if let Err(e) = self.auto_unload_unused(&loader).await {
                    warn!("Failed to auto-unload unused tools: {}", e);
                }
            }
        });
    }

    /// Get usage statistics
    pub async fn get_usage_stats(&self) -> HashMap<String, ToolUsageStats> {
        let stats = self.tool_usage_stats.lock().await;
        stats.clone()
    }

    /// Check if description compression is enabled
    pub fn is_compression_enabled(&self) -> bool {
        self.compress_descriptions
    }
}
