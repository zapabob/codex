//! Prompt builder with selective MCP tool inclusion.
//!
//! This module provides functionality to build prompts with only the necessary
//! MCP tools, reducing token consumption.

use anyhow::Result;
use mcp_types::Tool;
use std::sync::Arc;
use tracing::debug;
use tracing::info;
use tracing::warn;

use crate::client_common::Prompt;
use crate::mcp_token_optimizer::McpTokenOptimizer;

/// Prompt builder with tool selection
pub struct McpPromptBuilder {
    optimizer: Arc<McpTokenOptimizer>,
    token_budget: Option<u64>,
    max_tools_per_prompt: usize,
}

impl McpPromptBuilder {
    /// Create a new prompt builder
    pub fn new(
        optimizer: Arc<McpTokenOptimizer>,
        token_budget: Option<u64>,
        max_tools_per_prompt: usize,
    ) -> Self {
        Self {
            optimizer,
            token_budget,
            max_tools_per_prompt,
        }
    }

    /// Build prompt with selected tools
    pub fn build_prompt_with_tools(
        &self,
        _base_prompt: &str,
        available_tools: &[Tool],
        task_context: &str,
    ) -> Result<Prompt> {
        // Select relevant tools for the task
        let selected_tools = self.select_tools_for_prompt(available_tools, task_context);

        info!(
            "Selected {} tools out of {} available for prompt",
            selected_tools.len(),
            available_tools.len()
        );

        // Compress tool descriptions if enabled
        let _tool_descriptions = if self.optimizer.is_compression_enabled() {
            self.compress_tool_descriptions(&selected_tools)
        } else {
            selected_tools
                .iter()
                .map(|t| t.description.clone().unwrap_or_default())
                .collect()
        };

        // Estimate tokens
        let estimated_tokens = self.optimizer.estimate_tokens(&selected_tools);
        debug!("Estimated tokens for tools: {}", estimated_tokens);

        // Check token budget
        if let Some(budget) = self.token_budget {
            if estimated_tokens > budget {
                warn!(
                    "Tool descriptions exceed token budget ({} > {}), truncating",
                    estimated_tokens, budget
                );
                // TODO: Implement tool truncation based on priority
            }
        }

        // Build prompt (simplified - actual implementation would integrate with Prompt struct)
        // For now, this is a placeholder that shows the structure
        debug!("Building prompt with {} tools", selected_tools.len());

        // Note: Actual Prompt construction would require access to Prompt internals
        // This is a placeholder showing the selection logic
        Ok(Prompt::default())
    }

    /// Select tools for prompt based on context
    pub fn select_tools_for_prompt(&self, tools: &[Tool], context: &str) -> Vec<Tool> {
        // Use optimizer to select relevant tools
        let relevant = self.optimizer.select_relevant_tools(context, tools);

        // Limit to max_tools_per_prompt
        if relevant.len() > self.max_tools_per_prompt {
            info!(
                "Limiting tools from {} to {} (max_tools_per_prompt)",
                relevant.len(),
                self.max_tools_per_prompt
            );
            relevant.into_iter().take(self.max_tools_per_prompt).collect()
        } else {
            relevant
        }
    }

    /// Compress tool descriptions
    pub fn compress_tool_descriptions(&self, tools: &[Tool]) -> Vec<String> {
        tools
            .iter()
            .map(|tool| self.optimizer.compress_tool_description(tool))
            .collect()
    }
}
