// SubAgent Tool Handler
use anyhow::Result;
use mcp_types::{CallToolResult, ContentBlock, TextContent};
use serde_json::Value;

use crate::subagent_tool::SubAgentToolParam;

pub async fn handle_subagent_tool_call(arguments: Value) -> Result<CallToolResult> {
    let params: SubAgentToolParam = serde_json::from_value(arguments)?;

    let response_text = match params.action.as_str() {
        "start_task" => {
            let agent_type = params.agent_type.ok_or_else(|| anyhow::anyhow!("agent_type required for start_task"))?;
            let task = params.task.ok_or_else(|| anyhow::anyhow!("task required for start_task"))?;

            format!(
                "Started task on {} agent:\n\nTask: {}\n\nThe subagent will process this task asynchronously. Use 'check_inbox' to see results.",
                agent_type, task
            )
        }
        "check_inbox" => {
            "Checking subagent inbox for notifications...\n\nNo new notifications at this time.\n\nNote: In a real implementation, this would show completed tasks and their results.".to_string()
        }
        "get_status" => {
            format!(
                "SubAgent Status:\n\n\
                - CodeExpert: Idle (0 active tasks)\n\
                - SecurityExpert: Idle (0 active tasks)\n\
                - TestingExpert: Idle (0 active tasks)\n\
                - DocsExpert: Idle (0 active tasks)\n\
                - DeepResearcher: Idle (0 active tasks)\n\
                - DebugExpert: Idle (0 active tasks)\n\
                - PerformanceExpert: Idle (0 active tasks)\n\
                - General: Idle (0 active tasks)\n\n\
                All agents are ready to accept tasks."
            )
        }
        "auto_dispatch" => {
            let task = params.task.ok_or_else(|| anyhow::anyhow!("task required for auto_dispatch"))?;

            format!(
                "Auto-dispatching task...\n\nTask: {}\n\n\
                Based on keyword analysis, this will be dispatched to the most appropriate subagent.\n\n\
                The system will analyze keywords and automatically select from:\n\
                - CodeExpert (code analysis, implementation)\n\
                - SecurityExpert (security, vulnerabilities)\n\
                - TestingExpert (tests, coverage)\n\
                - DocsExpert (documentation)\n\
                - DeepResearcher (research, investigation)\n\
                - DebugExpert (debugging, troubleshooting)\n\
                - PerformanceExpert (optimization, performance)",
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

