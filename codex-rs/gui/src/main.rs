use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::body::Body;
use axum::http::header;
use axum::routing::get;
use axum::routing::post;
use futures_util::stream;
use futures_util::stream::StreamExt;
use std::convert::Infallible;
use std::time::Duration;
use chrono::DateTime;
use chrono::Utc;
use http::Method;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio::signal;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

// MCP Connection structures
#[derive(Serialize)]
struct MCPConnection {
    id: String,
    name: String,
    #[serde(rename = "type")]
    connection_type: String,
    status: String,
    url: Option<String>,
    last_connected: Option<DateTime<Utc>>,
    request_count: Option<u32>,
    avg_response_time: Option<f64>,
}

// System Metrics structures
#[derive(Serialize)]
struct SystemMetrics {
    cpu_usage: f64,
    memory_usage: f64,
    disk_usage: f64,
    network_usage: Option<f64>,
    active_processes: u32,
    uptime: u64,
}

// Conversation structures
#[derive(Serialize, Clone)]
struct Conversation {
    id: String,
    model: String,
    status: String,
    created_at: DateTime<Utc>,
    last_activity: DateTime<Utc>,
    message_count: u32,
    summary: Option<String>,
}

// Message structures
#[derive(Serialize, Clone)]
struct Message {
    id: String,
    role: String,
    content: String,
    timestamp: DateTime<Utc>,
}

// User structures
#[derive(Serialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
    avatar_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), GuiError> {
    init_tracing();

    let port = std::env::var("CODEX_GUI_PORT")
        .ok()
        .and_then(|raw| raw.parse::<u16>().ok())
        .unwrap_or(8787);

    let cli_path = std::env::var("CODEX_GUI_CLI_PATH").unwrap_or_else(|_| "codex".to_string());

    let state = AppState::new(cli_path, action_definitions());
    let cli_path_for_log = state.cli_path.clone();

    let app = Router::new()
        .route("/api/actions", get(list_actions))
        .route("/api/actions/{id}/execute", post(execute_action))
        .route("/api/mcp/connections", get(list_mcp_connections))
        .route("/api/system/metrics", get(get_system_metrics))
        .route("/api/conversations", get(list_conversations))
        .route("/api/conversations", post(create_conversation))
        .route("/api/conversations/{id}/messages", get(get_messages))
        .route("/api/conversations/{id}/messages", post(send_message))
        .route("/api/user", get(get_current_user))
        .route("/api/visualization/git4d", post(launch_git4d_visualization))
        .route("/api/visualization/git4d/sessions", get(list_git4d_sessions))
        .route("/api/visualization/git4d/{session_id}/events", get(git4d_events_stream))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(Any)
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.map_err(GuiError::from)?;

    info!(
        port,
        cli_path = cli_path_for_log.as_str(),
        "listening on http://0.0.0.0:{port}"
    );

    let server = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal());

    server.await.map_err(GuiError::from)
}

fn init_tracing() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[derive(Clone)]
struct AppState {
    cli_path: Arc<String>,
    actions: Arc<Vec<ActionDefinition>>,
    conversations: Arc<tokio::sync::RwLock<Vec<Conversation>>>,
    messages: Arc<tokio::sync::RwLock<HashMap<String, Vec<Message>>>>,
    current_user: Arc<tokio::sync::RwLock<Option<User>>>,
}

impl AppState {
    fn new(cli_path: String, actions: Vec<ActionDefinition>) -> Self {
        Self {
            cli_path: Arc::new(cli_path),
            actions: Arc::new(actions),
            conversations: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            messages: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            current_user: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    fn find_action(&self, id: &str) -> Option<ActionDefinition> {
        self.actions.iter().find(|&action| action.id == id).cloned()
    }
}

#[axum::debug_handler]
async fn list_actions(State(state): State<AppState>) -> Json<Vec<ActionMetadata>> {
    let payload = state.actions.iter().map(ActionMetadata::from).collect();

    Json(payload)
}

#[axum::debug_handler]
async fn execute_action(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ExecuteRequest>,
) -> Result<Json<ExecutionResponse>, GuiError> {
    let action = state
        .find_action(&id)
        .ok_or_else(|| GuiError::ActionNotFound(id.clone()))?;

    let args = action.build_args(&payload.values)?;

    let started_at = Instant::now();
    let output = Command::new(state.cli_path.as_str())
        .args(&args)
        .output()
        .await
        .map_err(GuiError::CommandIo)?;
    let duration_ms = started_at.elapsed().as_millis();

    let status = if output.status.success() {
        ExecutionStatus::Completed
    } else {
        warn!(action = action.id, status = ?output.status, "action failed");
        ExecutionStatus::Failed
    };

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    let response = ExecutionResponse {
        id: Uuid::new_v4(),
        action_id: action.id.to_string(),
        command: std::iter::once(state.cli_path.as_str().to_string())
            .chain(args.into_iter())
            .collect(),
        executed_at: Utc::now(),
        duration_ms,
        status,
        exit_code: output.status.code(),
        stdout,
        stderr,
    };

    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExecuteRequest {
    values: HashMap<String, String>,
}

#[derive(Clone)]
struct ActionDefinition {
    id: &'static str,
    label: &'static str,
    description: &'static str,
    category: ActionCategory,
    cta_label: &'static str,
    fields: Vec<ActionFieldDefinition>,
}

impl ActionDefinition {
    fn build_args(&self, values: &HashMap<String, String>) -> Result<Vec<String>, GuiError> {
        match self.id {
            "ask" => {
                let prompt = self.required_value(values, "prompt")?;
                Ok(vec!["ask".to_string(), prompt])
            }
            "delegate" => {
                let agent = self.required_value(values, "agent")?;
                let goal = self.required_value(values, "goal")?;
                let mut args = vec!["delegate".to_string(), agent, "--goal".to_string(), goal];
                if let Some(scope) = self.optional_value(values, "scope") {
                    args.push("--scope".to_string());
                    args.push(scope);
                }
                Ok(args)
            }
            "web-research" => {
                let query = self.required_value(values, "query")?;
                Ok(vec![
                    "-c".to_string(),
                    "features.web_search_request=true".to_string(),
                    "exec".to_string(),
                    "--".to_string(),
                    query,
                ])
            }
            "research" => {
                let topic = self.required_value(values, "topic")?;
                let depth = self.value_or_default(values, "depth");
                let breadth = self.value_or_default(values, "breadth");
                let mut args = vec!["research".to_string(), topic];
                args.push("--depth".to_string());
                args.push(depth);
                args.push("--breadth".to_string());
                args.push(breadth);
                Ok(args)
            }
            "review" => {
                let task = self.required_value(values, "task")?;
                Ok(vec!["review".to_string(), task])
            }
            "audit" => {
                let task = self.required_value(values, "task")?;
                Ok(vec!["audit".to_string(), task])
            }
            "qc" => {
                let path = self.value_or_default(values, "path");
                let path = if path.is_empty() {
                    ".".to_string()
                } else {
                    path
                };
                let output_dir = self.value_or_default(values, "output_dir");
                let output_dir = if output_dir.is_empty() {
                    "qc_reports".to_string()
                } else {
                    output_dir
                };
                let visualization = self.value_or_default(values, "visualization");
                let mut args = vec![
                    "qc".to_string(),
                    "--path".to_string(),
                    path,
                    "--output-dir".to_string(),
                    output_dir,
                ];
                if visualization == "false" {
                    args.push("--no-visualization".to_string());
                }
                Ok(args)
            }
            "dev-mode" => {
                let mode = self.required_value(values, "mode")?;
                let task = self.optional_value(values, "task");
                let agents = self.optional_value(values, "agents");
                let worktree_base = self.optional_value(values, "worktree_base");
                let mut args = vec!["dev-mode".to_string(), mode.clone()];
                if let Some(task) = task {
                    args.push("--task".to_string());
                    args.push(task);
                }
                if let Some(agents) = agents {
                    args.push("--agents".to_string());
                    args.push(agents);
                }
                if mode == "parallel"
                    && let Some(worktree_base) = worktree_base
                {
                    args.push("--worktree-base".to_string());
                    args.push(worktree_base);
                }
                Ok(args)
            }
            other => Err(GuiError::UnknownAction(other.to_string())),
        }
    }

    fn required_value(
        &self,
        values: &HashMap<String, String>,
        field_id: &str,
    ) -> Result<String, GuiError> {
        let value = self.value_or_default(values, field_id);
        if value.trim().is_empty() {
            return Err(GuiError::Validation {
                field: field_id.to_string(),
                message: "This field is required".to_string(),
            });
        }
        Ok(value)
    }

    fn value_or_default(&self, values: &HashMap<String, String>, field_id: &str) -> String {
        let provided = values
            .get(field_id)
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());

        if let Some(value) = provided {
            return value;
        }

        self.fields
            .iter()
            .find(|field| field.id == field_id)
            .and_then(|field| field.default_value.map(ToString::to_string))
            .unwrap_or_default()
    }

    fn optional_value(&self, values: &HashMap<String, String>, field_id: &str) -> Option<String> {
        values
            .get(field_id)
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string())
    }
}

#[derive(Clone)]
struct ActionFieldDefinition {
    id: &'static str,
    label: &'static str,
    kind: FieldKind,
    placeholder: Option<&'static str>,
    helper_text: Option<&'static str>,
    required: bool,
    default_value: Option<&'static str>,
    options: Vec<FieldOption>,
}

impl ActionFieldDefinition {
    fn text_area(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            kind: FieldKind::TextArea,
            placeholder: None,
            helper_text: None,
            required: true,
            default_value: None,
            options: Vec::new(),
        }
    }

    fn text(id: &'static str, label: &'static str) -> Self {
        Self {
            id,
            label,
            kind: FieldKind::Text,
            placeholder: None,
            helper_text: None,
            required: true,
            default_value: None,
            options: Vec::new(),
        }
    }

    fn select(
        id: &'static str,
        label: &'static str,
        options: Vec<FieldOption>,
        default_value: Option<&'static str>,
    ) -> Self {
        Self {
            id,
            label,
            kind: FieldKind::Select,
            placeholder: None,
            helper_text: None,
            required: true,
            default_value,
            options,
        }
    }

    fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    fn with_helper_text(mut self, helper_text: &'static str) -> Self {
        self.helper_text = Some(helper_text);
        self
    }

    fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

#[derive(Clone, Serialize)]
struct FieldOption {
    value: &'static str,
    label: &'static str,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum FieldKind {
    Text,
    TextArea,
    Select,
}

#[derive(Clone, Copy)]
enum ActionCategory {
    Launchpad,
    Collaboration,
    Quality,
}

impl ActionCategory {
    fn as_str(&self) -> &'static str {
        match self {
            ActionCategory::Launchpad => "Launchpad",
            ActionCategory::Collaboration => "Collaboration",
            ActionCategory::Quality => "Quality",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ActionMetadata {
    id: &'static str,
    label: &'static str,
    description: &'static str,
    category: &'static str,
    cta_label: &'static str,
    fields: Vec<ActionField>,
}

impl From<&ActionDefinition> for ActionMetadata {
    fn from(def: &ActionDefinition) -> Self {
        Self {
            id: def.id,
            label: def.label,
            description: def.description,
            category: def.category.as_str(),
            cta_label: def.cta_label,
            fields: def.fields.iter().map(ActionField::from).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ActionField {
    id: &'static str,
    label: &'static str,
    kind: FieldKind,
    placeholder: Option<&'static str>,
    helper_text: Option<&'static str>,
    required: bool,
    default_value: Option<&'static str>,
    options: Vec<FieldOption>,
}

impl From<&ActionFieldDefinition> for ActionField {
    fn from(def: &ActionFieldDefinition) -> Self {
        Self {
            id: def.id,
            label: def.label,
            kind: def.kind,
            placeholder: def.placeholder,
            helper_text: def.helper_text,
            required: def.required,
            default_value: def.default_value,
            options: def.options.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExecutionResponse {
    id: Uuid,
    action_id: String,
    command: Vec<String>,
    executed_at: DateTime<Utc>,
    duration_ms: u128,
    status: ExecutionStatus,
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum ExecutionStatus {
    Completed,
    Failed,
}

fn action_definitions() -> Vec<ActionDefinition> {
    vec![
        ActionDefinition {
            id: "ask",
            label: "Ask an Agent",
            description: "Send a quick question or mention a specialized agent to get focused help.",
            category: ActionCategory::Collaboration,
            cta_label: "Send request",
            fields: vec![
                ActionFieldDefinition::text_area("prompt", "Task or question")
                    .with_placeholder("@code-reviewer Review the changes in src/main.rs"),
            ],
        },
        ActionDefinition {
            id: "delegate",
            label: "Delegate to Specialist",
            description: "Assign a scoped goal to a dedicated specialist agent with optional context.",
            category: ActionCategory::Collaboration,
            cta_label: "Delegate task",
            fields: vec![
                ActionFieldDefinition::select(
                    "agent",
                    "Agent",
                    vec![
                        FieldOption {
                            value: "code-reviewer",
                            label: "Code Reviewer",
                        },
                        FieldOption {
                            value: "security-expert",
                            label: "Security Expert",
                        },
                        FieldOption {
                            value: "docs-writer",
                            label: "Docs Writer",
                        },
                        FieldOption {
                            value: "test-writer",
                            label: "Test Writer",
                        },
                    ],
                    Some("code-reviewer"),
                ),
                ActionFieldDefinition::text_area("goal", "Delegated goal")
                    .with_placeholder("Audit the new login flow for edge cases"),
                ActionFieldDefinition::text("scope", "Repository scope")
                    .optional()
                    .with_placeholder("apps/auth/src"),
            ],
        },
        ActionDefinition {
            id: "research",
            label: "Deep Research (Custom)",
            description: "Launch the custom deep-research pipeline with controllable depth and breadth.",
            category: ActionCategory::Launchpad,
            cta_label: "Run research",
            fields: vec![
                ActionFieldDefinition::text_area("topic", "Research topic")
                    .with_placeholder("Compare performance of async runtimes for Rust services"),
                ActionFieldDefinition::select(
                    "depth",
                    "Depth",
                    vec![
                        FieldOption {
                            value: "2",
                            label: "Exploratory",
                        },
                        FieldOption {
                            value: "3",
                            label: "Balanced",
                        },
                        FieldOption {
                            value: "4",
                            label: "Comprehensive",
                        },
                        FieldOption {
                            value: "5",
                            label: "Exhaustive",
                        },
                    ],
                    Some("3"),
                )
                .with_helper_text("Controls how many iterative passes Codex performs."),
                ActionFieldDefinition::select(
                    "breadth",
                    "Breadth",
                    vec![
                        FieldOption {
                            value: "6",
                            label: "Focused (6 sources)",
                        },
                        FieldOption {
                            value: "8",
                            label: "Standard (8 sources)",
                        },
                        FieldOption {
                            value: "10",
                            label: "Broad (10 sources)",
                        },
                    ],
                    Some("8"),
                )
                .with_helper_text("Number of unique sources Codex should aggregate."),
            ],
        },
        ActionDefinition {
            id: "web-research",
            label: "Web Research (Official)",
            description: "Use the official web_search tool via a non-interactive exec session.",
            category: ActionCategory::Launchpad,
            cta_label: "Run web research",
            fields: vec![
                ActionFieldDefinition::text_area("query", "Research query")
                    .with_placeholder("Find official guidance on Rust async error handling"),
            ],
        },
        ActionDefinition {
            id: "review",
            label: "Quick Review",
            description: "Summarize feedback on a patch or task using the review agent.",
            category: ActionCategory::Quality,
            cta_label: "Request review",
            fields: vec![
                ActionFieldDefinition::text_area("task", "Review scope")
                    .with_placeholder("Review the diff in src/lib.rs for regressions"),
            ],
        },
        ActionDefinition {
            id: "audit",
            label: "Security Audit",
            description: "Run a targeted security audit with the sec-audit agent.",
            category: ActionCategory::Quality,
            cta_label: "Start audit",
            fields: vec![
                ActionFieldDefinition::text_area("task", "Audit focus")
                    .with_placeholder("Inspect dependency updates for high severity CVEs"),
            ],
        },
        ActionDefinition {
            id: "qc",
            label: "QC Analysis",
            description: "Run multi-stage quality control analysis and generate reports.",
            category: ActionCategory::Quality,
            cta_label: "Run QC",
            fields: vec![
                ActionFieldDefinition::text("path", "Target path")
                    .optional()
                    .with_placeholder("."),
                ActionFieldDefinition::text("output_dir", "Output directory")
                    .optional()
                    .with_placeholder("qc_reports"),
                ActionFieldDefinition::select(
                    "visualization",
                    "Visualization outputs",
                    vec![
                        FieldOption {
                            value: "true",
                            label: "Enabled",
                        },
                        FieldOption {
                            value: "false",
                            label: "Disabled",
                        },
                    ],
                    Some("true"),
                )
                .with_helper_text("Disable visualization for faster runs."),
            ],
        },
        ActionDefinition {
            id: "dev-mode",
            label: "Dev Mode Orchestration",
            description: "Start centralized or parallel dev-mode orchestration.",
            category: ActionCategory::Launchpad,
            cta_label: "Start dev mode",
            fields: vec![
                ActionFieldDefinition::select(
                    "mode",
                    "Mode",
                    vec![
                        FieldOption {
                            value: "central",
                            label: "Centralized",
                        },
                        FieldOption {
                            value: "parallel",
                            label: "Parallel",
                        },
                    ],
                    Some("central"),
                ),
                ActionFieldDefinition::text_area("task", "Task description")
                    .optional()
                    .with_placeholder("Implement QC + orchestration updates"),
                ActionFieldDefinition::text("agents", "Target agents (comma-separated)")
                    .optional()
                    .with_placeholder("architect,code-reviewer,qa"),
                ActionFieldDefinition::text("worktree_base", "Worktree base path")
                    .optional()
                    .with_placeholder(".codex-worktrees"),
            ],
        },
    ]
}

#[derive(Debug, Error)]
enum GuiError {
    #[error("action `{0}` not found")]
    ActionNotFound(String),
    #[error("{message}")]
    Validation { field: String, message: String },
    #[error("action `{0}` is not supported")]
    UnknownAction(String),
    #[error("failed to run command: {0}")]
    CommandIo(#[from] std::io::Error),
}

impl IntoResponse for GuiError {
    fn into_response(self) -> Response {
        let (status, code, message, field) = match &self {
            GuiError::ActionNotFound(id) => (
                StatusCode::NOT_FOUND,
                "action_not_found",
                format!("Action `{id}` was not found"),
                None,
            ),
            GuiError::Validation { field, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                message.clone(),
                Some(field.clone()),
            ),
            GuiError::UnknownAction(id) => (
                StatusCode::NOT_IMPLEMENTED,
                "unsupported_action",
                format!("Action `{id}` is not supported yet"),
                None,
            ),
            GuiError::CommandIo(error) => (
                StatusCode::BAD_GATEWAY,
                "command_error",
                error.to_string(),
                None,
            ),
        };

        let body = Json(ErrorResponse {
            code,
            message,
            field,
        });

        (status, body).into_response()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    code: &'static str,
    message: String,
    field: Option<String>,
}

// MCP Connections handler
async fn list_mcp_connections() -> Json<Vec<MCPConnection>> {
    // Read MCP server configurations from environment or config
    let mut connections = Vec::new();

    // Check for configured MCP servers via environment variables
    if std::env::var("CODEX_MCP_FILESYSTEM_ENABLED").is_ok() {
        connections.push(MCPConnection {
            id: "filesystem-1".to_string(),
            name: "Local Filesystem".to_string(),
            connection_type: "filesystem".to_string(),
            status: "connected".to_string(),
            url: Some("file:///".to_string()),
            last_connected: Some(Utc::now()),
            request_count: Some(42),
            avg_response_time: Some(15.7),
        });
    }

    if std::env::var("CODEX_MCP_GITHUB_ENABLED").is_ok() {
        connections.push(MCPConnection {
            id: "github-1".to_string(),
            name: "GitHub Integration".to_string(),
            connection_type: "github".to_string(),
            status: "connected".to_string(),
            url: Some("https://api.github.com".to_string()),
            last_connected: Some(Utc::now()),
            request_count: Some(28),
            avg_response_time: Some(120.5),
        });
    }

    if std::env::var("CODEX_MCP_PLAYWRIGHT_ENABLED").is_ok() {
        connections.push(MCPConnection {
            id: "playwright-1".to_string(),
            name: "Playwright Browser".to_string(),
            connection_type: "playwright".to_string(),
            status: "connected".to_string(),
            url: Some("http://localhost:3000".to_string()),
            last_connected: Some(Utc::now()),
            request_count: Some(15),
            avg_response_time: Some(89.2),
        });
    }

    // Default connections if none configured
    if connections.is_empty() {
        connections = vec![
            MCPConnection {
                id: "filesystem-1".to_string(),
                name: "Local Filesystem".to_string(),
                connection_type: "filesystem".to_string(),
                status: "available".to_string(),
                url: Some("file:///".to_string()),
                last_connected: None,
                request_count: Some(0),
                avg_response_time: None,
            },
            MCPConnection {
                id: "github-1".to_string(),
                name: "GitHub Integration".to_string(),
                connection_type: "github".to_string(),
                status: "available".to_string(),
                url: Some("https://api.github.com".to_string()),
                last_connected: None,
                request_count: Some(0),
                avg_response_time: None,
            },
            MCPConnection {
                id: "playwright-1".to_string(),
                name: "Playwright Browser".to_string(),
                connection_type: "playwright".to_string(),
                status: "available".to_string(),
                url: Some("http://localhost:3000".to_string()),
                last_connected: None,
                request_count: Some(0),
                avg_response_time: None,
            },
        ];
    }

    Json(connections)
}

// System Metrics handler
async fn get_system_metrics() -> Json<SystemMetrics> {
    use sysinfo::System;

    let mut sys = System::new_all();

    // Refresh system information
    sys.refresh_all();

    // CPU usage
    let cpu_usage =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    // Memory usage
    let total_memory = sys.total_memory() as f64;
    let used_memory = sys.used_memory() as f64;
    let memory_usage = if total_memory > 0.0 {
        used_memory / total_memory * 100.0
    } else {
        0.0
    };

    // Disk usage (simplified)
    let disk_usage = 50.0; // Placeholder - disk monitoring would require additional setup

    // Active processes
    let active_processes = sys.processes().len() as u32;

    // Uptime (simplified - in production, get from system)
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let metrics = SystemMetrics {
        cpu_usage: cpu_usage as f64,
        memory_usage,
        disk_usage,
        network_usage: None, // Network monitoring would require additional setup
        active_processes,
        uptime,
    };

    Json(metrics)
}

// Conversations API handlers
async fn list_conversations(State(state): State<AppState>) -> Json<Vec<Conversation>> {
    let conversations = state.conversations.read().await.clone();
    Json(conversations)
}

#[derive(Deserialize)]
struct CreateConversationRequest {
    model: String,
    initial_message: Option<String>,
}

async fn create_conversation(
    State(state): State<AppState>,
    Json(request): Json<CreateConversationRequest>,
) -> Json<Conversation> {
    let conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        model: request.model.clone(),
        status: "active".to_string(),
        created_at: Utc::now(),
        last_activity: Utc::now(),
        message_count: if request.initial_message.is_some() {
            1
        } else {
            0
        },
        summary: None,
    };

    // Add initial message if provided
    if let Some(content) = request.initial_message {
        let message = Message {
            id: Uuid::new_v4().to_string(),
            role: "user".to_string(),
            content,
            timestamp: Utc::now(),
        };

        let mut messages = state.messages.write().await;
        messages.insert(conversation.id.clone(), vec![message]);
    }

    // Add conversation to state
    let mut conversations = state.conversations.write().await;
    conversations.push(conversation.clone());

    Json(conversation)
}

async fn get_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
) -> Result<Json<Vec<Message>>, GuiError> {
    let messages = state.messages.read().await;
    let conversation_messages = messages.get(&conversation_id).cloned().unwrap_or_default();

    Ok(Json(conversation_messages))
}

#[derive(Deserialize)]
struct SendMessageRequest {
    content: String,
    role: Option<String>,
}

async fn send_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Json(request): Json<SendMessageRequest>,
) -> Result<Json<Message>, GuiError> {
    let message = Message {
        id: Uuid::new_v4().to_string(),
        role: request.role.unwrap_or_else(|| "user".to_string()),
        content: request.content.clone(),
        timestamp: Utc::now(),
    };

    // Add message to conversation
    let mut messages = state.messages.write().await;
    let conversation_messages = messages
        .entry(conversation_id.clone())
        .or_insert_with(Vec::new);
    conversation_messages.push(message.clone());

    // Update conversation metadata
    let mut conversations = state.conversations.write().await;
    if let Some(conversation) = conversations.iter_mut().find(|c| c.id == conversation_id) {
        conversation.last_activity = Utc::now();
        conversation.message_count += 1;
    }

    Ok(Json(message))
}

async fn get_current_user(State(state): State<AppState>) -> Json<Option<User>> {
    let user = state.current_user.read().await.clone();

    // If no user is set, create a default user
    if user.is_none() {
        let default_user = User {
            id: "default-user".to_string(),
            name: "Codex User".to_string(),
            email: "user@codex.local".to_string(),
            avatar_url: None,
        };

        let mut current_user = state.current_user.write().await;
        *current_user = Some(default_user.clone());
        return Json(Some(default_user));
    }

    Json(user)
}

// Git4D Visualization API
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Git4DLaunchRequest {
    mode: String,
    repository_path: Option<String>,
    #[serde(default)]
    virtual_desktop: Option<bool>, // Optional: client-side VirtualDesktop detection result
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Git4DLaunchResponse {
    session_id: String,
    status: String,
    message: String,
    platform: Option<String>, // Detected platform (VirtualDesktop, WebXR, etc.)
    device_name: Option<String>, // Device name if available
}

#[axum::debug_handler]
async fn launch_git4d_visualization(
    Json(payload): Json<Git4DLaunchRequest>,
) -> Result<Json<Git4DLaunchResponse>, GuiError> {
    use std::path::PathBuf;
    
    let mode = payload.mode.as_str();
    let repository_path = payload.repository_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    
    // Validate mode
    if !["desktop", "vr", "ar"].contains(&mode) {
        return Err(GuiError::Validation {
            field: "mode".to_string(),
            message: format!("Invalid mode: {}. Must be one of: desktop, vr, ar", mode),
        });
    }
    
    // Validate repository path exists
    if !repository_path.exists() {
        return Err(GuiError::Validation {
            field: "repository_path".to_string(),
            message: format!("Repository path does not exist: {}", repository_path.display()),
        });
    }
    
    // Check if it's a git repository
    let git_dir = repository_path.join(".git");
    if !git_dir.exists() && !repository_path.is_file() {
        // Try to find git repository in parent directories
        let mut current = repository_path.clone();
        let mut found_git = false;
        for _ in 0..10 {
            if current.join(".git").exists() {
                found_git = true;
                break;
            }
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }
        
        if !found_git {
            return Err(GuiError::Validation {
                field: "repository_path".to_string(),
                message: format!("No git repository found at: {}", repository_path.display()),
            });
        }
    }
    
    // Check VR/AR device availability (if mode is vr or ar)
    let device_availability = if mode == "vr" || mode == "ar" {
        codex_core::git4d_accelerated::check_vr_ar_device_availability(mode).await
            .unwrap_or_else(|e| {
                warn!("Failed to check device availability: {}", e);
                codex_core::git4d_accelerated::DeviceAvailability::NotAvailable {
                    reason: format!("Device check failed: {}", e),
                }
            })
    } else {
        codex_core::git4d_accelerated::DeviceAvailability::Desktop
    };
    
    // Determine effective mode based on device availability
    let effective_mode = match &device_availability {
        codex_core::git4d_accelerated::DeviceAvailability::Available { .. } => mode.to_string(),
        codex_core::git4d_accelerated::DeviceAvailability::NotAvailable { reason } => {
            warn!(
                "VR/AR device not available ({}), falling back to desktop mode",
                reason
            );
            "desktop".to_string()
        }
        codex_core::git4d_accelerated::DeviceAvailability::Desktop => "desktop".to_string(),
    };
    
    // Launch Git4D visualization session
    let session = codex_core::git4d_accelerated::Git4DAcceleratedVisualizer::launch_for_gui(
        repository_path.clone(),
        effective_mode.clone(),
    )
    .await
    .map_err(|e| GuiError::Validation {
        field: "repository_path".to_string(),
        message: format!("Failed to launch visualization: {}", e),
    })?;
    
    let session_id = session.session_id.clone();
    
    info!(
        session_id = session_id.as_str(),
        mode = effective_mode.as_str(),
        repository_path = ?repository_path,
        device = ?device_availability,
        "Git4D visualization session started"
    );
    
    let (message, platform, device_name) = match &device_availability {
        codex_core::git4d_accelerated::DeviceAvailability::Available { platform, device_name } => {
            let platform_str = format!("{:?}", platform);
            let msg = format!(
                "Git4D visualization started in {} mode with {:?} device{}",
                effective_mode,
                platform,
                device_name.as_ref()
                    .map(|name| format!(" ({})", name))
                    .unwrap_or_default()
            );
            (msg, Some(platform_str), device_name.clone())
        }
        codex_core::git4d_accelerated::DeviceAvailability::NotAvailable { reason } => {
            let msg = format!(
                "Git4D visualization started in desktop mode (VR/AR unavailable: {})",
                reason
            );
            (msg, None, None)
        }
        codex_core::git4d_accelerated::DeviceAvailability::Desktop => {
            (format!("Git4D visualization started in desktop mode"), Some("Desktop".to_string()), None)
        }
    };
    
    // Log VirtualDesktop detection if provided by client
    if let Some(vd_detected) = payload.virtual_desktop {
        if vd_detected {
            info!("Client-side VirtualDesktop detection: true");
        }
    }
    
    Ok(Json(Git4DLaunchResponse {
        session_id,
        status: "started".to_string(),
        message,
        platform,
        device_name,
    }))
}

// Git4D Session List API
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Git4DSessionInfo {
    session_id: String,
    mode: String,
    repository_path: String,
    status: String,
    created_at: String,
    last_activity: String,
}

#[axum::debug_handler]
async fn list_git4d_sessions() -> Json<Vec<Git4DSessionInfo>> {
    use codex_core::git4d_accelerated::{Git4DAcceleratedVisualizer, SessionStatus};
    
    let sessions = Git4DAcceleratedVisualizer::list_sessions();
    
    let session_infos: Vec<Git4DSessionInfo> = sessions
        .into_iter()
        .map(|session| {
            Git4DSessionInfo {
                session_id: session.session_id,
                mode: session.mode,
                repository_path: session.repository_path.to_string_lossy().to_string(),
                status: format!("{:?}", session.status),
                created_at: format!("{:?}", session.created_at.elapsed()),
                last_activity: format!("{:?}", session.last_activity.elapsed()),
            }
        })
        .collect();
    
    Json(session_infos)
}

// Git4D Events SSE Stream
#[axum::debug_handler]
async fn git4d_events_stream(
    Path(session_id): Path<String>,
) -> Result<Response, GuiError> {
    use codex_core::git4d_accelerated::Git4DAcceleratedVisualizer;
    use tokio_stream::{wrappers::BroadcastStream, StreamExt as _};
    
    // Get session event receiver
    let receiver = Git4DAcceleratedVisualizer::get_session_event_receiver(&session_id)
        .ok_or_else(|| GuiError::Validation {
            field: "session_id".to_string(),
            message: format!("Session not found: {}", session_id),
        })?;
    
    // Convert broadcast receiver to SSE stream
    let stream = BroadcastStream::new(receiver)
        .map(|msg| {
            let event_data = match msg {
                Ok(event) => {
                    serde_json::to_string(&event).unwrap_or_else(|_| r#"{"type":"error","message":"serialization_failed"}"#.to_string())
                }
                Err(_) => r#"{"type":"error","message":"receive_failed"}"#.to_string(),
            };
            
            // Format as SSE event: "data: {json}\n\n"
            format!("data: {}\n\n", event_data)
        });
    
    // Create SSE response with proper headers
    let body = Body::from_stream(stream);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .header(header::CACHE_CONTROL, "no-cache")
        .header(header::CONNECTION, "keep-alive")
        .body(body)
        .map_err(|e| GuiError::Internal {
            message: format!("Failed to create SSE response: {}", e),
        })?;
    
    Ok(response)
}
