//! Handler for auto-orchestrator tool calls via MCP.

use crate::auto_orchestrator_tool::AutoOrchestratorToolParam;
use mcp_types::CallToolResult;
use mcp_types::ContentBlock;
use mcp_types::RequestId;
use mcp_types::TextContent;
use serde_json::json;

/// Handle an auto-orchestrator tool call.
pub async fn handle_auto_orchestrator_tool_call(
    _id: RequestId,
    arguments: Option<serde_json::Value>,
) -> CallToolResult {
    let params = match arguments {
        Some(json_val) => match serde_json::from_value::<AutoOrchestratorToolParam>(json_val) {
            Ok(p) => p,
            Err(e) => {
                return CallToolResult {
                    content: vec![ContentBlock::TextContent(TextContent {
                        r#type: "text".to_string(),
                        text: format!("Invalid auto-orchestrator parameters: {e}"),
                        annotations: None,
                    })],
                    is_error: Some(true),
                    structured_content: None,
                };
            }
        },
        None => {
            return CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: "Missing auto-orchestrator parameters".to_string(),
                    annotations: None,
                })],
                is_error: Some(true),
                structured_content: None,
            };
        }
    };

    // Execute auto-orchestration
    let result_text = match execute_auto_orchestration(&params).await {
        Ok(output) => {
            if params.format == "json" {
                output
            } else {
                format!(
                    "# Auto-Orchestration Result\n\n\
                     **Goal**: {}\n\n\
                     **Threshold**: {}\n\n\
                     **Strategy**: {}\n\n\
                     ## Analysis & Execution\n\n\
                     {}",
                    params.goal, params.auto_threshold, params.strategy, output
                )
            }
        }
        Err(e) => {
            return CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: format!("Auto-orchestration execution failed: {e}"),
                    annotations: None,
                })],
                is_error: Some(true),
                structured_content: None,
            };
        }
    };

    CallToolResult {
        content: vec![ContentBlock::TextContent(TextContent {
            r#type: "text".to_string(),
            text: result_text,
            annotations: None,
        })],
        is_error: None,
        structured_content: None,
    }
}

/// Execute the auto-orchestration logic.
async fn execute_auto_orchestration(params: &AutoOrchestratorToolParam) -> anyhow::Result<String> {
    use codex_core::orchestration::TaskAnalyzer;

    // 1. Create TaskAnalyzer and analyze the goal
    let analyzer = TaskAnalyzer::new(params.auto_threshold);
    let mut analysis = analyzer.analyze(&params.goal);
    let (skill_warnings, mapped_agents) = validate_and_map_skills(&params.skills);
    if !mapped_agents.is_empty() {
        for agent in mapped_agents {
            if !analysis.recommended_agents.contains(&agent) {
                analysis.recommended_agents.push(agent);
            }
        }
    }

    let complexity = analysis.complexity_score;
    let strategy = params.strategy.as_str();
    let threshold_warning = if complexity <= params.auto_threshold && !params.skills.is_empty() {
        Some(format!(
            "Provided skills will not trigger orchestration unless complexity exceeds the threshold ({:.2}).",
            params.auto_threshold
        ))
    } else {
        None
    };

    // 2. Check if complexity > threshold
    if analysis.should_orchestrate(params.auto_threshold) {
        // 3. Execute orchestration
        // Note: Full orchestration requires AgentRuntime which needs Config, Auth, etc.
        // For MCP tool context, we return the analysis and recommended plan
        // The actual execution happens in codex.rs when this is called from main agent

        if params.format == "json" {
            Ok(json!({
                "was_orchestrated": true,
                "complexity_score": complexity,
                "threshold": params.auto_threshold,
                "strategy": strategy,
                "recommended_agents": analysis.recommended_agents,
                "subtasks": analysis.subtasks,
                "detected_keywords": analysis.detected_keywords,
                "skills": params.skills.clone(),
                "skill_warnings": skill_warnings,
                "threshold_warning": threshold_warning,
                "execution_summary": format!(
                    "Task complexity ({:.2}) exceeds threshold ({:.2}). \
                     Recommending {} specialized agents using {strategy} strategy.",
                    complexity,
                    params.auto_threshold,
                    analysis.recommended_agents.len(),
                ),
                "task_analysis": {
                    "complexity_score": complexity,
                    "detected_keywords": analysis.detected_keywords,
                    "recommended_agents": analysis.recommended_agents,
                    "subtasks": analysis.subtasks,
                    "skills": params.skills.clone(),
                    "skill_warnings": skill_warnings,
                }
            })
            .to_string())
        } else {
            Ok(format!(
                "**Complexity Analysis**: {:.2} (threshold: {:.2}) ✅ **Will Orchestrate**\n\n\
                 **Recommended Agents**: {}\n\n\
                 **Execution Strategy**: {}\n\n\
                 **Detected Keywords**: {}\n\n\
                 {}\n\
                 **Subtasks**:\n{}\n\n\
                 **Summary**: Task complexity exceeds threshold. \
                 Recommending {} specialized agents to handle this task.",
                complexity,
                params.auto_threshold,
                analysis.recommended_agents.join(", "),
                strategy,
                analysis.detected_keywords.join(", "),
                skill_warnings
                    .iter()
                    .map(|w| format!("⚠️ {w}"))
                    .chain(
                        threshold_warning
                            .as_ref()
                            .map(|w| format!("⚠️ {w}"))
                            .into_iter(),
                    )
                    .collect::<Vec<_>>()
                    .join("\n"),
                analysis
                    .subtasks
                    .iter()
                    .enumerate()
                    .map(|(i, t)| format!("{}. {}", i + 1, t))
                    .collect::<Vec<_>>()
                    .join("\n"),
                analysis.recommended_agents.len()
            ))
        }
    } else {
        // Would not be orchestrated
        if params.format == "json" {
            Ok(json!({
                "was_orchestrated": false,
                "complexity_score": complexity,
                "threshold": params.auto_threshold,
                "detected_keywords": analysis.detected_keywords,
                "skills": params.skills.clone(),
                "skill_warnings": skill_warnings,
                "threshold_warning": threshold_warning,
                "execution_summary": format!(
                    "Task complexity ({:.2}) below threshold ({:.2}). Using normal execution.",
                    complexity,
                    params.auto_threshold
                )
            })
            .to_string())
        } else {
            Ok(format!(
                "**Complexity Analysis**: {:.2} (threshold: {:.2}) ❌ **Normal Execution**\n\n\
                 **Detected Keywords**: {}\n\n\
                 {}\n\
                 **Summary**: Task complexity is below threshold. \
                 Will use standard single-agent execution.",
                complexity,
                params.auto_threshold,
                analysis.detected_keywords.join(", "),
                skill_warnings
                    .iter()
                    .map(|w| format!("⚠️ {w}"))
                    .chain(
                        threshold_warning
                            .as_ref()
                            .map(|w| format!("⚠️ {w}"))
                            .into_iter(),
                    )
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
        }
    }
}

fn validate_and_map_skills(skills: &[String]) -> (Vec<String>, Vec<String>) {
    if skills.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let mut warnings = Vec::new();
    let mut agents = Vec::new();

    for skill in skills {
        let normalized = skill.trim().to_lowercase();
        match normalized.as_str() {
            "security" | "sec" | "auth" => agents.push("sec-audit".to_string()),
            "testing" | "qa" | "test" => agents.push("test-gen".to_string()),
            "performance" | "perf" => agents.push("code-reviewer".to_string()),
            "docs" | "documentation" | "doc" => agents.push("researcher".to_string()),
            "" => {}
            _ => warnings.push(format!(
                "Unknown skill '{skill}', using TaskAnalyzer suggestions."
            )),
        }
    }

    (warnings, agents)
}
