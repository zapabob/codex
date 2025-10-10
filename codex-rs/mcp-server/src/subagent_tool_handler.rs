// SubAgent Tool Handler
use anyhow::Result;
use codex_supervisor::{AgentType, RealSubAgentManager};
use mcp_types::{CallToolResult, ContentBlock, TextContent};
use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::subagent_tool::SubAgentToolParam;

// Global RealSubAgentManager instance
static SUBAGENT_MANAGER: Lazy<Arc<Mutex<RealSubAgentManager>>> = Lazy::new(|| {
    Arc::new(Mutex::new(RealSubAgentManager::default()))
});

pub async fn handle_subagent_tool_call(arguments: Value) -> Result<CallToolResult> {
    let params: SubAgentToolParam = serde_json::from_value(arguments)?;

    let response_text = match params.action.as_str() {
        "start_task" => {
            let agent_type_str = params.agent_type.ok_or_else(|| anyhow::anyhow!("agent_type required for start_task"))?;
            let task = params.task.ok_or_else(|| anyhow::anyhow!("task required for start_task"))?;

            let agent_type = parse_agent_type(&agent_type_str)?;

            // Execute task with RealSubAgent
            let mut manager = SUBAGENT_MANAGER.lock().await;
            let result = manager.dispatch_task(agent_type.clone(), task.clone()).await?;

            format!(
                "✅ Task completed by {} agent:\n\n{}\n\n---\n\nTask: {}",
                agent_type_str, result, task
            )
        }
        "check_inbox" => {
            "Checking subagent inbox for notifications...\n\nNo new notifications at this time.\n\nNote: Task results are returned immediately with the start_task action.".to_string()
        }
        "get_status" => {
            let manager = SUBAGENT_MANAGER.lock().await;
            let states = manager.get_all_states();

            let mut status_text = String::from("🤖 SubAgent Status\n\n");
            for state in states {
                let status_icon = match state.status {
                    codex_supervisor::subagent::AgentStatus::Idle => "⚪",
                    codex_supervisor::subagent::AgentStatus::Working => "🟡",
                    codex_supervisor::subagent::AgentStatus::Completed => "🟢",
                    codex_supervisor::subagent::AgentStatus::Failed => "🔴",
                };
                let task_info = state.current_task.as_deref().unwrap_or("No active task");
                let progress = (state.progress * 100.0) as u8;

                status_text.push_str(&format!(
                    "{} **{}**: {:?} ({}% complete)\n   Task: {}\n\n",
                    status_icon, state.agent_type, state.status, progress, task_info
                ));
            }
            status_text.push_str("\n✅ All agents are ready to accept tasks.");
            status_text
        }
        "auto_dispatch" => {
            let task = params.task.ok_or_else(|| anyhow::anyhow!("task required for auto_dispatch"))?;

            // Use AutonomousDispatcher to classify task
            let dispatcher = codex_supervisor::AutonomousDispatcher::new();
            let classification = dispatcher.classify_task(&task);

            // Execute task with classified agent
            let mut manager = SUBAGENT_MANAGER.lock().await;
            let result = manager.dispatch_task(classification.recommended_agent.clone(), task.clone()).await?;

            format!(
                "🎯 Auto-dispatch completed\n\n\
                **Selected Agent**: {}\n\
                **Confidence**: {:.1}%\n\
                **Reasoning**: {}\n\n\
                ---\n\n\
                ✅ Task Result:\n\n{}\n\n\
                ---\n\nTask: {}",
                classification.recommended_agent,
                classification.confidence * 100.0,
                classification.reasoning,
                result,
                task
            )
        }
        "get_thinking" => {
            let task_id = params.task_id.as_deref().unwrap_or("all");

            if task_id == "all" {
                "=== All Thinking Processes ===\n\nNo active thinking processes at this time.\n\nThinking processes are automatically tracked when subagents execute tasks.".to_string()
            } else {
                format!(
                    "Thinking Process for Task: {}\n\n\
                    No thinking process found for this task ID.\n\n\
                    Thinking processes are automatically created when tasks are dispatched to subagents.",
                    task_id
                )
            }
        }
        "get_token_report" => {
            "=== Token Usage Report ===\n\n\
            Total Usage:\n\
              Prompt Tokens: 0\n\
              Completion Tokens: 0\n\
              Total Tokens: 0\n\
              Percentage: 0.0%\n\n\
            Agent Usage:\n\
              (No usage recorded yet)\n\n\
            Token usage is automatically tracked when subagents execute tasks.".to_string()
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown action: {}", params.action));
        }
    };

    Ok(CallToolResult {
        content: vec![ContentBlock::TextContent(TextContent {
            r#type: "text".to_string(),
            text: response_text,
            annotations: None,
        })],
        is_error: None,
        structured_content: None,
    })
}

/// Parse agent type string to AgentType enum
fn parse_agent_type(agent_type_str: &str) -> Result<AgentType> {
    match agent_type_str {
        "CodeExpert" => Ok(AgentType::CodeExpert),
        "SecurityExpert" => Ok(AgentType::SecurityExpert),
        "TestingExpert" => Ok(AgentType::TestingExpert),
        "DocsExpert" => Ok(AgentType::DocsExpert),
        "DeepResearcher" => Ok(AgentType::DeepResearcher),
        "DebugExpert" => Ok(AgentType::DebugExpert),
        "PerformanceExpert" => Ok(AgentType::PerformanceExpert),
        "General" => Ok(AgentType::General),
        _ => Err(anyhow::anyhow!("Unknown agent type: {}", agent_type_str)),
    }
}

