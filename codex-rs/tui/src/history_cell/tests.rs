use super::*;
use crate::exec_cell::CommandOutput;
use crate::exec_cell::ExecCall;
use crate::exec_cell::ExecCell;
use crate::history_cell::basic::PrefixedWrappedHistoryCell;
use codex_core::config::Config;
use codex_core::config::ConfigBuilder;
use codex_core::config::types::McpServerConfig;
use codex_core::config::types::McpServerTransportConfig;
use codex_core::protocol::McpAuthStatus;
use codex_core::protocol::McpInvocation;
use codex_protocol::models::WebSearchAction;
use codex_protocol::openai_models::ReasoningEffort as ReasoningEffortConfig;
use codex_protocol::parse_command::ParsedCommand;
use codex_protocol::plan_tool::{PlanItemArg, StepStatus, UpdatePlanArgs};
use dirs::home_dir;
use pretty_assertions::assert_eq;
use serde_json::json;
use std::collections::HashMap;

use crate::history_cell::exec::UnifiedExecProcessDetails;

use codex_protocol::mcp::CallToolResult;
use codex_protocol::mcp::Tool;

use std::time::Duration;
use std::time::Instant;

const SMALL_PNG_BASE64: &str = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR4nGP4z8DwHwAFAAH/iZk9HQAAAABJRU5ErkJggg==";
async fn test_config() -> Config {
    let codex_home = std::env::temp_dir();
    ConfigBuilder::default()
        .codex_home(codex_home.clone())
        .build()
        .await
        .expect("config")
}

fn render_lines(lines: &[Line<'static>]) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            line.spans
                .iter()
                .map(|span| span.content.as_ref())
                .collect::<String>()
        })
        .collect()
}

fn render_transcript(cell: &dyn HistoryCell) -> Vec<String> {
    render_lines(&cell.transcript_lines(u16::MAX))
}

#[test]
fn unified_exec_interaction_cell_renders_input() {
    let cell = new_unified_exec_interaction(Some("echo hello".to_string()), "ls\npwd".to_string());
    let lines = render_transcript(&cell);
    assert_eq!(
        lines,
        vec![
            "↳ Interacted with background terminal · echo hello",
            "  └ ls",
            "    pwd",
        ],
    );
}

#[test]
fn unified_exec_interaction_cell_renders_wait() {
    let cell = new_unified_exec_interaction(None, String::new());
    let lines = render_transcript(&cell);
    assert_eq!(
        lines,
        vec!["↳ Interacted with background terminal", "  └ (waited)"],
    );
}

#[test]
fn ps_output_empty_snapshot() {
    let cell = new_unified_exec_processes_output(Vec::new());
    let rendered = render_lines(&cell.display_lines(60)).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn ps_output_multiline_snapshot() {
    let cell = new_unified_exec_processes_output(vec![
        UnifiedExecProcessDetails {
            command_display: "echo hello\nand then some extra text".to_string(),
            recent_chunks: Vec::new(),
        },
        UnifiedExecProcessDetails {
            command_display: "rg \"foo\" src".to_string(),
            recent_chunks: Vec::new(),
        },
    ]);
    let rendered = render_lines(&cell.display_lines(40)).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn ps_output_long_command_snapshot() {
    let cell = new_unified_exec_processes_output(vec![UnifiedExecProcessDetails {
        command_display: String::from(
            "rg \"foo\" src --glob '**/*.rs' --max-count 1000 --no-ignore --hidden --follow --glob '!target/**'",
        ),
        recent_chunks: Vec::new(),
    }]);
    let rendered = render_lines(&cell.display_lines(36)).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn ps_output_many_sessions_snapshot() {
    let cell = new_unified_exec_processes_output(
        (0..20)
            .map(|idx| UnifiedExecProcessDetails {
                command_display: format!("command {idx}"),
                recent_chunks: Vec::new(),
            })
            .collect(),
    );
    let rendered = render_lines(&cell.display_lines(32)).join("\n");
    insta::assert_snapshot!(rendered);
}

#[tokio::test]
async fn mcp_tools_output_masks_sensitive_values() {
    let mut config = test_config().await;
    let mut env = HashMap::new();
    env.insert("TOKEN".to_string(), "secret".to_string());
    let stdio_config = McpServerConfig {
        transport: McpServerTransportConfig::Stdio {
            command: "docs-server".to_string(),
            args: vec![],
            env: Some(env),
            env_vars: vec!["APP_TOKEN".to_string()],
            cwd: None,
        },
        enabled: true,
        disabled_reason: None,
        startup_timeout_sec: None,
        tool_timeout_sec: None,
        enabled_tools: None,
        disabled_tools: None,
        scopes: None,
        required: false,
    };
    let mut servers = config.mcp_servers.get().clone();
    servers.insert("docs".to_string(), stdio_config);

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Bearer secret".to_string());
    let mut env_headers = HashMap::new();
    env_headers.insert("X-API-Key".to_string(), "API_KEY_ENV".to_string());
    let http_config = McpServerConfig {
        transport: McpServerTransportConfig::StreamableHttp {
            url: "https://example.com/mcp".to_string(),
            bearer_token_env_var: Some("MCP_TOKEN".to_string()),
            http_headers: Some(headers),
            env_http_headers: Some(env_headers),
        },
        enabled: true,
        disabled_reason: None,
        startup_timeout_sec: None,
        tool_timeout_sec: None,
        enabled_tools: None,
        disabled_tools: None,
        scopes: None,
        required: false,
    };
    servers.insert("http".to_string(), http_config);
    config
        .mcp_servers
        .set(servers)
        .expect("test mcp servers should accept any configuration");

    let mut tools: HashMap<String, Tool> = HashMap::new();
    tools.insert(
        "mcp__docs__list".to_string(),
        Tool {
            annotations: None,
            description: None,
            input_schema: json!({
                "type": "object"
            }),
            name: "list".to_string(),
            output_schema: None,
            title: None,
            icons: None,
            meta: None,
        },
    );
    tools.insert(
        "mcp__http__ping".to_string(),
        Tool {
            annotations: None,
            description: None,
            input_schema: json!({
                "type": "object"
            }),
            name: "ping".to_string(),
            output_schema: None,
            title: None,
            icons: None,
            meta: None,
        },
    );

    let auth_statuses: HashMap<String, McpAuthStatus> = HashMap::new();
    let cell = new_mcp_tools_output(
        &config,
        tools,
        HashMap::new(),
        HashMap::new(),
        &auth_statuses,
    );
    let rendered = render_lines(&cell.display_lines(120)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn empty_agent_message_cell_transcript() {
    let cell = AgentMessageCell::new(vec![Line::default()], false);
    assert_eq!(cell.transcript_lines(80), vec![Line::from("  ")]);
    assert_eq!(cell.desired_transcript_height(80), 1);
}

#[test]
fn prefixed_wrapped_history_cell_indents_wrapped_lines() {
    let summary = Line::from(vec![
        "You ".into(),
        "approved".bold(),
        " codex to run ".into(),
        "echo something really long to ensure wrapping happens".dim(),
        " this time".bold(),
    ]);
    let cell = PrefixedWrappedHistoryCell::new(summary, "✔ ".green(), "  ");
    let rendered = render_lines(&cell.display_lines(24));
    assert_eq!(
        rendered,
        vec![
            "✔ You approved codex to".to_string(),
            "  run echo something".to_string(),
            "  really long to ensure".to_string(),
            "  wrapping happens this".to_string(),
            "  time".to_string(),
        ]
    );
}

#[test]
fn web_search_history_cell_snapshot() {
    let query = "example search query with several generic words to exercise wrapping".to_string();
    let cell = new_web_search_call(
        "call-1".to_string(),
        query.clone(),
        WebSearchAction::Search {
            query: Some(query),
            queries: None,
        },
    );
    let rendered = render_lines(&cell.display_lines(64)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn web_search_history_cell_wraps_with_indented_continuation() {
    let query = "example search query with several generic words to exercise wrapping".to_string();
    let cell = new_web_search_call(
        "call-1".to_string(),
        query.clone(),
        WebSearchAction::Search {
            query: Some(query),
            queries: None,
        },
    );
    let rendered = render_lines(&cell.display_lines(64));

    assert_eq!(
        rendered,
        vec![
            "• Searched example search query with several generic words to".to_string(),
            "  exercise wrapping".to_string(),
        ]
    );
}

#[test]
fn web_search_history_cell_short_query_does_not_wrap() {
    let query = "short query".to_string();
    let cell = new_web_search_call(
        "call-1".to_string(),
        query.clone(),
        WebSearchAction::Search {
            query: Some(query),
            queries: None,
        },
    );
    let rendered = render_lines(&cell.display_lines(64));

    assert_eq!(rendered, vec!["• Searched short query".to_string()]);
}

#[test]
fn web_search_history_cell_transcript_snapshot() {
    let query = "example search query with several generic words to exercise wrapping".to_string();
    let cell = new_web_search_call(
        "call-1".to_string(),
        query.clone(),
        WebSearchAction::Search {
            query: Some(query),
            queries: None,
        },
    );
    let rendered = render_lines(&cell.transcript_lines(64)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn active_mcp_tool_call_snapshot() {
    let invocation = McpInvocation {
        server: "search".into(),
        tool: "find_docs".into(),
        arguments: Some(json!({
            "query": "ratatui styling",
            "limit": 3,
        })),
    };

    let cell = new_active_mcp_tool_call("call-1".into(), invocation, true);
    let rendered = render_lines(&cell.display_lines(80)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn completed_mcp_tool_call_success_snapshot() {
    let invocation = McpInvocation {
        server: "search".into(),
        tool: "find_docs".into(),
        arguments: Some(json!({
            "query": "ratatui styling",
            "limit": 3,
        })),
    };

    let result = CallToolResult {
        content: vec![serde_json::json!({
            "type": "text",
            "text": "Found styling guidance in styles.md"
        })],
        is_error: None,
        structured_content: None,
        meta: None,
    };

    let mut cell = new_active_mcp_tool_call("call-2".into(), invocation, true);
    assert!(
        cell.complete(Duration::from_millis(1420), Ok(result))
            .is_none()
    );

    let rendered = render_lines(&cell.display_lines(80)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn completed_mcp_tool_call_image_after_text_returns_extra_cell() {
    let invocation = McpInvocation {
        server: "image".into(),
        tool: "generate".into(),
        arguments: Some(json!({
            "prompt": "tiny image",
        })),
    };

    let result = CallToolResult {
        content: vec![
            serde_json::json!({
                "type": "text",
                "text": "Here is the image:"
            }),
            serde_json::json!({
                "type": "image",
                "data": SMALL_PNG_BASE64,
                "mimeType": "image/png"
            }),
        ],
        is_error: None,
        structured_content: None,
        meta: None,
    };

    let mut cell = new_active_mcp_tool_call("call-image".into(), invocation, true);
    let extra_cell = cell
        .complete(Duration::from_millis(25), Ok(result))
        .expect("expected image cell");

    let rendered = render_lines(&extra_cell.display_lines(80));
    assert_eq!(rendered, vec!["tool result (image output)"]);
}

#[test]
fn completed_mcp_tool_call_skips_invalid_image_blocks() {
    let invocation = McpInvocation {
        server: "image".into(),
        tool: "generate".into(),
        arguments: Some(json!({
            "prompt": "tiny image",
        })),
    };

    let result = CallToolResult {
        content: vec![
            serde_json::json!({
                "type": "image",
                "data": "not-base64",
                "mimeType": "image/png"
            }),
            serde_json::json!({
                "type": "image",
                "data": SMALL_PNG_BASE64,
                "mimeType": "image/png"
            }),
        ],
        is_error: None,
        structured_content: None,
        meta: None,
    };

    let mut cell = new_active_mcp_tool_call("call-image-2".into(), invocation, true);
    let extra_cell = cell
        .complete(Duration::from_millis(25), Ok(result))
        .expect("expected image cell");

    let rendered = render_lines(&extra_cell.display_lines(80));
    assert_eq!(rendered, vec!["tool result (image output)"]);
}

#[test]
fn completed_mcp_tool_call_error_snapshot() {
    let invocation = McpInvocation {
        server: "search".into(),
        tool: "find_docs".into(),
        arguments: Some(json!({
            "query": "ratatui styling",
            "limit": 3,
        })),
    };

    let mut cell = new_active_mcp_tool_call("call-3".into(), invocation, true);
    assert!(
        cell.complete(Duration::from_secs(2), Err("network timeout".into()))
            .is_none()
    );

    let rendered = render_lines(&cell.display_lines(80)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn completed_mcp_tool_call_multiple_outputs_snapshot() {
    let invocation = McpInvocation {
        server: "search".into(),
        tool: "find_docs".into(),
        arguments: Some(json!({
            "query": "ratatui styling",
            "limit": 3,
        })),
    };

    let result = CallToolResult {
        content: vec![
            serde_json::json!({
                "type": "text",
                "text": "Found styling guidance in styles.md and additional notes in CONTRIBUTING.md."
            }),
            serde_json::json!({
                "type": "resource",
                "uri": "file:///docs/styles.md",
                "name": "styles.md",
                "description": "Link to styles documentation",
                "title": "Styles"
            }),
        ],
        is_error: None,
        structured_content: None,
        meta: None,
    };

    let mut cell = new_active_mcp_tool_call("call-4".into(), invocation, true);
    assert!(
        cell.complete(Duration::from_millis(640), Ok(result))
            .is_none()
    );

    let rendered = render_lines(&cell.display_lines(48)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn completed_mcp_tool_call_wrapped_outputs_snapshot() {
    let invocation = McpInvocation {
        server: "metrics".into(),
        tool: "get_nearby_metric".into(),
        arguments: Some(json!({
            "query": "very_long_query_that_needs_wrapping_to_display_properly_in_the_history",
            "limit": 1,
        })),
    };

    let result = CallToolResult {
        content: vec![serde_json::json!({
            "type": "text",
            "text": "Line one of the response, which is quite long and needs wrapping.\nLine two continues the response with more detail."
        })],
        is_error: None,
        structured_content: None,
        meta: None,
    };

    let mut cell = new_active_mcp_tool_call("call-5".into(), invocation, true);
    assert!(
        cell.complete(Duration::from_millis(1280), Ok(result))
            .is_none()
    );

    let rendered = render_lines(&cell.display_lines(40)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn completed_mcp_tool_call_multiple_outputs_inline_snapshot() {
    let invocation = McpInvocation {
        server: "metrics".into(),
        tool: "summary".into(),
        arguments: Some(json!({
            "metric": "trace.latency",
            "window": "15m",
        })),
    };

    let result = CallToolResult {
        content: vec![
            serde_json::json!({
                "type": "text",
                "text": "Latency summary: p50=120ms, p95=480ms."
            }),
            serde_json::json!({
                "type": "text",
                "text": "No anomalies detected."
            }),
        ],
        is_error: None,
        structured_content: None,
        meta: None,
    };

    let mut cell = new_active_mcp_tool_call("call-6".into(), invocation, true);
    assert!(
        cell.complete(Duration::from_millis(320), Ok(result))
            .is_none()
    );

    let rendered = render_lines(&cell.display_lines(120)).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn session_header_includes_reasoning_level_when_present() {
    let cell = SessionHeaderHistoryCell::new(
        "gpt-4o".to_string(),
        Some(ReasoningEffortConfig::High),
        std::env::temp_dir(),
        "test",
    );

    let lines = render_lines(&cell.display_lines(80));
    let model_line = lines
        .into_iter()
        .find(|line| line.contains("model:"))
        .expect("model line");

    assert!(model_line.contains("gpt-4o high"));
    assert!(model_line.contains("/model to change"));
}

#[test]
fn session_header_directory_center_truncates() {
    let mut dir = home_dir().expect("home directory");
    for part in ["hello", "the", "fox", "is", "very", "fast"] {
        dir.push(part);
    }

    let formatted = SessionHeaderHistoryCell::format_directory_inner(&dir, Some(24));
    let sep = std::path::MAIN_SEPARATOR;
    let expected = format!("~{sep}hello{sep}the{sep}…{sep}very{sep}fast");
    assert_eq!(formatted, expected);
}

#[test]
fn session_header_directory_front_truncates_long_segment() {
    let mut dir = home_dir().expect("home directory");
    dir.push("supercalifragilisticexpialidocious");

    let formatted = SessionHeaderHistoryCell::format_directory_inner(&dir, Some(18));
    let sep = std::path::MAIN_SEPARATOR;
    let expected = format!("~{sep}…cexpialidocious");
    assert_eq!(formatted, expected);
}

#[test]
fn coalesces_sequential_reads_within_one_call() {
    // Build one exec cell with a Search followed by two Reads
    let call_id = "c1".to_string();
    let mut cell = ExecCell::new(ExecCall {
        call_id: call_id.clone(),
        command: vec!["bash".into(), "-lc".into(), "echo".into()],
        parsed: vec![
            ParsedCommand::Search {
                query: Some("shimmer_spans".into()),
                path: None,
                cmd: "rg shimmer_spans".into(),
            },
            ParsedCommand::Read {
                name: "shimmer.rs".into(),
                cmd: "cat shimmer.rs".into(),
                path: "shimmer.rs".into(),
            },
            ParsedCommand::Read {
                name: "status_indicator_widget.rs".into(),
                cmd: "cat status_indicator_widget.rs".into(),
                path: "status_indicator_widget.rs".into(),
            },
        ],
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    // Mark call complete so markers are ✓
    cell.complete_call(&call_id, CommandOutput::default(), Duration::from_millis(1));

    let lines = cell.display_lines(80);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn coalesces_reads_across_multiple_calls() {
    let mut cell = ExecCell::new(ExecCall {
        call_id: "c1".to_string(),
        command: vec!["bash".into(), "-lc".into(), "echo".into()],
        parsed: vec![ParsedCommand::Search {
            query: Some("shimmer_spans".into()),
            path: None,
            cmd: "rg shimmer_spans".into(),
        }],
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    // Call 1: Search only
    cell.complete_call("c1", CommandOutput::default(), Duration::from_millis(1));
    // Call 2: Read A
    cell = cell
        .with_added_call(
            "c2".into(),
            vec!["bash".into(), "-lc".into(), "echo".into()],
            vec![ParsedCommand::Read {
                name: "shimmer.rs".into(),
                cmd: "cat shimmer.rs".into(),
                path: "shimmer.rs".into(),
            }],
            false,
        )
        .unwrap();
    cell.complete_call("c2", CommandOutput::default(), Duration::from_millis(1));
    // Call 3: Read B
    cell = cell
        .with_added_call(
            "c3".into(),
            vec!["bash".into(), "-lc".into(), "echo".into()],
            vec![ParsedCommand::Read {
                name: "status_indicator_widget.rs".into(),
                cmd: "cat status_indicator_widget.rs".into(),
                path: "status_indicator_widget.rs".into(),
            }],
            false,
        )
        .unwrap();
    cell.complete_call("c3", CommandOutput::default(), Duration::from_millis(1));

    let lines = cell.display_lines(80);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn coalesced_reads_dedupe_names() {
    let mut cell = ExecCell::new(ExecCall {
        call_id: "c1".to_string(),
        command: vec!["bash".into(), "-lc".into(), "echo".into()],
        parsed: vec![
            ParsedCommand::Read {
                name: "auth.rs".into(),
                cmd: "cat auth.rs".into(),
                path: "auth.rs".into(),
            },
            ParsedCommand::Read {
                name: "auth.rs".into(),
                cmd: "cat auth.rs".into(),
                path: "auth.rs".into(),
            },
            ParsedCommand::Read {
                name: "shimmer.rs".into(),
                cmd: "cat shimmer.rs".into(),
                path: "shimmer.rs".into(),
            },
        ],
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    cell.complete_call("c1", CommandOutput::default(), Duration::from_millis(1));
    let lines = cell.display_lines(80);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn multiline_command_wraps_with_extra_indent_on_subsequent_lines() {
    // Create a completed exec cell with a multiline command
    let cmd = "set -o pipefail\ncargo test --all-features --quiet".to_string();
    let call_id = "c1".to_string();
    let mut cell = ExecCell::new(ExecCall {
        call_id: call_id.clone(),
        command: vec!["bash".into(), "-lc".into(), cmd],
        parsed: Vec::new(),
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    // Mark call complete so it renders as "Ran"
    cell.complete_call(&call_id, CommandOutput::default(), Duration::from_millis(1));

    // Small width to force wrapping on both lines
    let width: u16 = 28;
    let lines = cell.display_lines(width);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn single_line_command_compact_when_fits() {
    let call_id = "c1".to_string();
    let mut cell = ExecCell::new(ExecCall {
        call_id: call_id.clone(),
        command: vec!["echo".into(), "ok".into()],
        parsed: Vec::new(),
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    cell.complete_call(&call_id, CommandOutput::default(), Duration::from_millis(1));
    // Wide enough that it fits inline
    let lines = cell.display_lines(80);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn single_line_command_wraps_with_four_space_continuation() {
    let call_id = "c1".to_string();
    let long = "a_very_long_token_without_spaces_to_force_wrapping".to_string();
    let mut cell = ExecCell::new(ExecCall {
        call_id: call_id.clone(),
        command: vec!["bash".into(), "-lc".into(), long],
        parsed: Vec::new(),
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    cell.complete_call(&call_id, CommandOutput::default(), Duration::from_millis(1));
    let lines = cell.display_lines(24);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn multiline_command_without_wrap_uses_branch_then_eight_spaces() {
    let call_id = "c1".to_string();
    let cmd = "echo one\necho two".to_string();
    let mut cell = ExecCell::new(ExecCall {
        call_id: call_id.clone(),
        command: vec!["bash".into(), "-lc".into(), cmd],
        parsed: Vec::new(),
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    cell.complete_call(&call_id, CommandOutput::default(), Duration::from_millis(1));
    let lines = cell.display_lines(80);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn multiline_command_both_lines_wrap_with_correct_prefixes() {
    let call_id = "c1".to_string();
    let cmd =
        "first_token_is_long_enough_to_wrap\nsecond_token_is_also_long_enough_to_wrap".to_string();
    let mut cell = ExecCell::new(ExecCall {
        call_id: call_id.clone(),
        command: vec!["bash".into(), "-lc".into(), cmd],
        parsed: Vec::new(),
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    cell.complete_call(&call_id, CommandOutput::default(), Duration::from_millis(1));
    let lines = cell.display_lines(28);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn stderr_tail_more_than_five_lines_snapshot() {
    // Build an exec cell with a non-zero exit and 10 lines on stderr to exercise
    // the head/tail rendering and gutter prefixes.
    let call_id = "c_err".to_string();
    let mut cell = ExecCell::new(ExecCall {
        call_id: call_id.clone(),
        command: vec!["bash".into(), "-lc".into(), "seq 1 10 1>&2 && false".into()],
        parsed: Vec::new(),
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });
    let stderr: String = (1..=10)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    cell.complete_call(
        &call_id,
        CommandOutput {
            exit_code: 1,
            formatted_output: String::new(),
            aggregated_output: stderr,
        },
        Duration::from_millis(1),
    );

    let rendered = cell
        .display_lines(80)
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn ran_cell_multiline_with_stderr_snapshot() {
    // Build an exec cell that completes (so it renders as "Ran") with a
    // command long enough that it must render on its own line under the
    // header, and include a couple of stderr lines to verify the output
    // block prefixes and wrapping.
    let call_id = "c_wrap_err".to_string();
    let long_cmd =
        "echo this_is_a_very_long_single_token_that_will_wrap_across_the_available_width";
    let mut cell = ExecCell::new(ExecCall {
        call_id: call_id.clone(),
        command: vec!["bash".into(), "-lc".into(), long_cmd.to_string()],
        parsed: Vec::new(),
        output: None,
        is_user_shell_command: false,
        start_time: Some(Instant::now()),
        duration: None,
    });

    let stderr = "error: first line on stderr\nerror: second line on stderr".to_string();
    cell.complete_call(
        &call_id,
        CommandOutput {
            exit_code: 1,
            formatted_output: String::new(),
            aggregated_output: stderr,
        },
        Duration::from_millis(5),
    );

    // Narrow width to force the command to render under the header line.
    let width: u16 = 28;
    let rendered = cell
        .display_lines(width)
        .iter()
        .map(|l| {
            l.spans
                .iter()
                .map(|s| s.content.as_ref())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    insta::assert_snapshot!(rendered);
}
#[test]
fn user_history_cell_wraps_and_prefixes_each_line_snapshot() {
    let msg = "one two three four five six seven";
    let cell = UserHistoryCell {
        message: msg.to_string(),
        text_elements: Vec::new(),
        local_image_paths: Vec::new(),
    };

    // Small width to force wrapping more clearly. Effective wrap width is width-2 due to the ▌ prefix and trailing space.
    let width: u16 = 12;
    let lines = cell.display_lines(width);
    let rendered = render_lines(&lines).join("\n");

    insta::assert_snapshot!(rendered);
}

#[test]
fn plan_update_with_note_and_wrapping_snapshot() {
    // Long explanation forces wrapping; include long step text to verify step wrapping and alignment.
    let update = UpdatePlanArgs {
        explanation: Some(
            "I’ll update Grafana call error handling by adding retries and clearer messages when the backend is unreachable."
                .to_string(),
        ),
        plan: vec![
            PlanItemArg {
                step: "Investigate existing error paths and logging around HTTP timeouts".into(),
                status: StepStatus::Completed,
            },
            PlanItemArg {
                step: "Harden Grafana client error handling with retry/backoff and user‑friendly messages".into(),
                status: StepStatus::InProgress,
            },
            PlanItemArg {
                step: "Add tests for transient failure scenarios and surfacing to the UI".into(),
                status: StepStatus::Pending,
            },
        ],
    };

    let cell = new_plan_update(update);
    // Narrow width to force wrapping for both the note and steps
    let lines = cell.display_lines(32);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}

#[test]
fn plan_update_without_note_snapshot() {
    let update = UpdatePlanArgs {
        explanation: None,
        plan: vec![
            PlanItemArg {
                step: "Define error taxonomy".into(),
                status: StepStatus::InProgress,
            },
            PlanItemArg {
                step: "Implement mapping to user messages".into(),
                status: StepStatus::Pending,
            },
        ],
    };

    let cell = new_plan_update(update);
    let lines = cell.display_lines(40);
    let rendered = render_lines(&lines).join("\n");
    insta::assert_snapshot!(rendered);
}
#[test]
fn reasoning_summary_block() {
    let cell = new_reasoning_summary_block(
        "**High level reasoning**\n\nDetailed reasoning goes here.".to_string(),
    );

    let rendered_display = render_lines(&cell.display_lines(80));
    assert_eq!(rendered_display, vec!["• Detailed reasoning goes here."]);

    let rendered_transcript = render_transcript(cell.as_ref());
    assert_eq!(rendered_transcript, vec!["• Detailed reasoning goes here."]);
}

#[test]
fn reasoning_summary_block_returns_reasoning_cell_when_feature_disabled() {
    let cell = new_reasoning_summary_block("Detailed reasoning goes here.".to_string());

    let rendered = render_transcript(cell.as_ref());
    assert_eq!(rendered, vec!["• Detailed reasoning goes here."]);
}

#[tokio::test]
async fn reasoning_summary_block_respects_config_overrides() {
    let mut config = test_config().await;
    config.model = Some("gpt-3.5-turbo".to_string());
    config.model_supports_reasoning_summaries = Some(true);
    let cell = new_reasoning_summary_block(
        "**High level reasoning**\n\nDetailed reasoning goes here.".to_string(),
    );

    let rendered_display = render_lines(&cell.display_lines(80));
    assert_eq!(rendered_display, vec!["• Detailed reasoning goes here."]);
}

#[test]
fn reasoning_summary_block_falls_back_when_header_is_missing() {
    let cell = new_reasoning_summary_block("**High level reasoning without closing".to_string());

    let rendered = render_transcript(cell.as_ref());
    assert_eq!(rendered, vec!["• **High level reasoning without closing"]);
}

#[test]
fn reasoning_summary_block_falls_back_when_summary_is_missing() {
    let cell = new_reasoning_summary_block("**High level reasoning without closing**".to_string());

    let rendered = render_transcript(cell.as_ref());
    assert_eq!(rendered, vec!["• High level reasoning without closing"]);

    let cell =
        new_reasoning_summary_block("**High level reasoning without closing**\n\n  ".to_string());

    let rendered = render_transcript(cell.as_ref());
    assert_eq!(rendered, vec!["• High level reasoning without closing"]);
}

#[test]
fn reasoning_summary_block_splits_header_and_summary_when_present() {
    let cell = new_reasoning_summary_block(
        "**High level plan**\n\nWe should fix the bug next.".to_string(),
    );

    let rendered_display = render_lines(&cell.display_lines(80));
    assert_eq!(rendered_display, vec!["• We should fix the bug next."]);

    let rendered_transcript = render_transcript(cell.as_ref());
    assert_eq!(rendered_transcript, vec!["• We should fix the bug next."]);
}

#[test]
fn deprecation_notice_renders_summary_with_details() {
    let cell = new_deprecation_notice(
        "Feature flag `foo`".to_string(),
        Some("Use flag `bar` instead.".to_string()),
    );
    let lines = cell.display_lines(80);
    let rendered = render_lines(&lines);
    assert_eq!(
        rendered,
        vec![
            "⚠ Feature flag `foo`".to_string(),
            "Use flag `bar` instead.".to_string(),
        ]
    );
}
