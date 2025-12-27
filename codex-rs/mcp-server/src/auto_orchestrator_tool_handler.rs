//! Handler for auto-orchestrator tool calls via MCP.

use crate::auto_orchestrator_tool::AutoOrchestratorToolParam;
use mcp_types::CallToolResult;
use mcp_types::ContentBlock;
use mcp_types::RequestId;
use mcp_types::TextContent;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;
use std::collections::BTreeSet;

#[derive(Clone, Serialize)]
struct AgentConfigMetadata {
    agent: String,
    skill_tag: String,
    scope: String,
    config_path: String,
    capabilities: Vec<String>,
}

struct AutoOrchestrationArtifacts {
    content_text: String,
    structured: Value,
}

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
    let orchestration_result = match execute_auto_orchestration(&params).await {
        Ok(output) => output,
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
            text: orchestration_result.content_text,
            annotations: None,
        })],
        is_error: Some(false),
        structured_content: Some(orchestration_result.structured),
    }
}

/// Execute the auto-orchestration logic.
async fn execute_auto_orchestration(
    params: &AutoOrchestratorToolParam,
) -> anyhow::Result<AutoOrchestrationArtifacts> {
    use codex_core::orchestration::TaskAnalyzer;

    // 1. Create TaskAnalyzer and analyze the goal
    let analyzer = TaskAnalyzer::new(params.auto_threshold);
    let analysis = analyzer.analyze(&params.goal);

    let complexity = analysis.complexity_score;
    let recommended_agents = analysis.recommended_agents.clone();
    let detected_keywords = analysis.detected_keywords.clone();
    let subtasks = analysis.subtasks.clone();
    let (agent_configs, skills_used) = collect_agent_metadata(&recommended_agents);
    let fallbacks = fallbacks_for_strategy(
        &params.strategy,
        analysis.should_orchestrate(params.auto_threshold),
    );
    let skills_text = display_comma_list(&skills_used);
    let fallbacks_text = display_comma_list(&fallbacks);
    let agent_configs_text = agent_config_lines(&agent_configs);

    // 2. Check if complexity > threshold
    if analysis.should_orchestrate(params.auto_threshold) {
        // 3. Execute orchestration
        // Note: Full orchestration requires AgentRuntime which needs Config, Auth, etc.
        // For MCP tool context, we return the analysis and recommended plan
        // The actual execution happens in codex.rs when this is called from main agent

        let execution_summary = format!(
            "Task complexity ({:.2}) exceeds threshold ({:.2}). \
             Recommending {} specialized agents using {} strategy.",
            complexity,
            params.auto_threshold,
            recommended_agents.len(),
            params.strategy
        );
        let structured = json!({
            "was_orchestrated": true,
            "complexity_score": complexity,
            "threshold": params.auto_threshold,
            "recommended_agents": recommended_agents,
            "skills_used": skills_used,
            "strategy": params.strategy,
            "fallbacks": fallbacks,
            "agent_configs": agent_configs,
            "subtasks": subtasks,
            "detected_keywords": detected_keywords,
            "execution_summary": execution_summary,
            "task_analysis": {
                "complexity_score": complexity,
                "detected_keywords": analysis.detected_keywords,
                "recommended_agents": analysis.recommended_agents,
                "subtasks": analysis.subtasks
            }
        });
        let content_text = if params.format == "json" {
            serde_json::to_string_pretty(&structured)?
        } else {
            format!(
                "# Auto-Orchestration Result\n\n\
                 **Goal**: {}\n\n\
                 **Threshold**: {:.2}\n\n\
                 **Strategy**: {}\n\n\
                 **Skills Used**: {}\n\n\
                 **Fallbacks**: {}\n\n\
                 ## Analysis & Execution\n\n\
                 **Complexity Analysis**: {:.2} (threshold: {:.2}) ✅ **Will Orchestrate**\n\n\
                 **Recommended Agents**: {}\n\n\
                 **Execution Strategy**: {}\n\n\
                 **Detected Keywords**: {}\n\n\
                 **Subtasks**:\n{}\n\n\
                 **Agent Configs**:\n{}\n\n\
                 **Summary**: {}",
                params.goal,
                params.auto_threshold,
                params.strategy,
                skills_text,
                fallbacks_text,
                complexity,
                params.auto_threshold,
                recommended_agents.join(", "),
                params.strategy,
                detected_keywords.join(", "),
                subtasks
                    .iter()
                    .enumerate()
                    .map(|(i, t)| format!("{}. {}", i + 1, t))
                    .collect::<Vec<_>>()
                    .join("\n"),
                agent_configs_text,
                execution_summary
            )
        };
        Ok(AutoOrchestrationArtifacts {
            content_text,
            structured,
        })
    } else {
        // Would not be orchestrated
        let execution_summary = format!(
            "Task complexity ({:.2}) below threshold ({:.2}). Using normal execution.",
            complexity, params.auto_threshold
        );
        let structured = json!({
            "was_orchestrated": false,
            "complexity_score": complexity,
            "threshold": params.auto_threshold,
            "recommended_agents": recommended_agents,
            "skills_used": skills_used,
            "strategy": params.strategy,
            "fallbacks": fallbacks,
            "agent_configs": agent_configs,
            "detected_keywords": detected_keywords,
            "execution_summary": execution_summary
        });
        let content_text = if params.format == "json" {
            serde_json::to_string_pretty(&structured)?
        } else {
            format!(
                "# Auto-Orchestration Result\n\n\
                 **Goal**: {}\n\n\
                 **Threshold**: {:.2}\n\n\
                 **Strategy**: {}\n\n\
                 **Skills Used**: {}\n\n\
                 **Fallbacks**: {}\n\n\
                 ## Analysis & Execution\n\n\
                 **Complexity Analysis**: {:.2} (threshold: {:.2}) ❌ **Normal Execution**\n\n\
                 **Recommended Agents**: {}\n\n\
                 **Detected Keywords**: {}\n\n\
                 **Agent Configs**:\n{}\n\n\
                 **Summary**: {}",
                params.goal,
                params.auto_threshold,
                params.strategy,
                skills_text,
                fallbacks_text,
                complexity,
                params.auto_threshold,
                recommended_agents.join(", "),
                detected_keywords.join(", "),
                agent_configs_text,
                execution_summary
            )
        };
        Ok(AutoOrchestrationArtifacts {
            content_text,
            structured,
        })
    }
}

fn collect_agent_metadata(
    recommended_agents: &[String],
) -> (Vec<AgentConfigMetadata>, Vec<String>) {
    let mut skills_used = BTreeSet::new();
    let mut agent_configs = Vec::new();

    for agent in recommended_agents {
        if let Some(config) = agent_config_for(agent) {
            skills_used.insert(config.skill_tag.clone());
            agent_configs.push(config);
        } else {
            skills_used.insert(agent.clone());
        }
    }

    (agent_configs, skills_used.into_iter().collect())
}

fn agent_config_for(agent: &str) -> Option<AgentConfigMetadata> {
    match agent {
        "sec-audit" => Some(AgentConfigMetadata {
            agent: "sec-audit".to_string(),
            skill_tag: "security-review".to_string(),
            scope: "specialist".to_string(),
            config_path: ".codex/agents/sec-audit.yaml".to_string(),
            capabilities: vec![
                "Threat modeling".to_string(),
                "Static security scan".to_string(),
                "Secrets and credential review".to_string(),
            ],
        }),
        "test-gen" => Some(AgentConfigMetadata {
            agent: "test-gen".to_string(),
            skill_tag: "testing".to_string(),
            scope: "specialist".to_string(),
            config_path: ".codex/agents/test-gen.yaml".to_string(),
            capabilities: vec![
                "Unit/integration test authoring".to_string(),
                "Edge case discovery".to_string(),
                "Snapshot verification".to_string(),
            ],
        }),
        "code-reviewer" => Some(AgentConfigMetadata {
            agent: "code-reviewer".to_string(),
            skill_tag: "code-quality".to_string(),
            scope: "generalist".to_string(),
            config_path: ".codex/agents/code-reviewer.yaml".to_string(),
            capabilities: vec![
                "Defect discovery".to_string(),
                "Readability and API design feedback".to_string(),
                "Incremental diff review".to_string(),
            ],
        }),
        "researcher" => Some(AgentConfigMetadata {
            agent: "researcher".to_string(),
            skill_tag: "research".to_string(),
            scope: "generalist".to_string(),
            config_path: ".codex/agents/researcher.yaml".to_string(),
            capabilities: vec![
                "Docs synthesis".to_string(),
                "Knowledge base lookups".to_string(),
                "Standards cross-referencing".to_string(),
            ],
        }),
        "dependency-analyst" => Some(AgentConfigMetadata {
            agent: "dependency-analyst".to_string(),
            skill_tag: "dependency-analysis".to_string(),
            scope: "specialist".to_string(),
            config_path: ".codex/agents/dependency-analyst.yaml".to_string(),
            capabilities: vec![
                "Manifest and lockfile parsing".to_string(),
                "Version drift detection".to_string(),
                "Supply-chain risk triage".to_string(),
            ],
        }),
        "dependency-scout" => Some(AgentConfigMetadata {
            agent: "dependency-scout".to_string(),
            skill_tag: "dependency-analysis".to_string(),
            scope: "generalist".to_string(),
            config_path: ".codex/agents/dependency-scout.yaml".to_string(),
            capabilities: vec![
                "Lightweight manifest inspection".to_string(),
                "Transitive dependency surfacing".to_string(),
                "License note collection".to_string(),
            ],
        }),
        "performance-analyst" => Some(AgentConfigMetadata {
            agent: "performance-analyst".to_string(),
            skill_tag: "performance".to_string(),
            scope: "specialist".to_string(),
            config_path: ".codex/agents/performance-analyst.yaml".to_string(),
            capabilities: vec![
                "Profile interpretation".to_string(),
                "Bottleneck localization".to_string(),
                "Regression risk scoring".to_string(),
            ],
        }),
        "performance-scout" => Some(AgentConfigMetadata {
            agent: "performance-scout".to_string(),
            skill_tag: "performance".to_string(),
            scope: "generalist".to_string(),
            config_path: ".codex/agents/performance-scout.yaml".to_string(),
            capabilities: vec![
                "Log-based latency checks".to_string(),
                "Config sanity review".to_string(),
                "Quick benchmark suggestions".to_string(),
            ],
        }),
        _ => None,
    }
}

fn fallbacks_for_strategy(strategy: &str, orchestrated: bool) -> Vec<String> {
    match (strategy, orchestrated) {
        ("parallel", true) => vec![
            "retry_failed_agents_sequentially".to_string(),
            "reduce_scope_and_rerun".to_string(),
            "fallback_to_single_agent_execution".to_string(),
        ],
        ("sequential", true) => vec![
            "promote_hot_paths_to_parallel".to_string(),
            "collapse_to_primary_agent_only".to_string(),
        ],
        ("hybrid", true) => vec![
            "switch_to_parallel_on_blocking_tasks".to_string(),
            "defer_non_blocking_to_single_agent".to_string(),
        ],
        (_, false) => vec!["escalate_to_orchestration_if_complexity_spikes".to_string()],
        _ => vec!["fallback_to_single_agent_execution".to_string()],
    }
}

fn display_comma_list(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

fn agent_config_lines(agent_configs: &[AgentConfigMetadata]) -> String {
    if agent_configs.is_empty() {
        "- (no agent configuration metadata mapped)".to_string()
    } else {
        agent_configs
            .iter()
            .map(|config| {
                format!(
                    "- {} [{} | {}] ({}): {}",
                    config.agent,
                    config.skill_tag,
                    config.scope,
                    config.config_path,
                    config.capabilities.join(", ")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
