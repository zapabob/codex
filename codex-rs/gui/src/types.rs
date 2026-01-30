use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

// MCP Connection structures
#[derive(Serialize)]
pub struct MCPConnection {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub status: String,
    pub url: Option<String>,
    pub last_connected: Option<DateTime<Utc>>,
    pub request_count: Option<u32>,
    pub avg_response_time: Option<f64>,
}

// System Metrics structures
#[derive(Serialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: Option<f64>,
    pub active_processes: u32,
    pub uptime: u64,
}

// Conversation structures
#[derive(Serialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub model: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub message_count: u32,
    pub summary: Option<String>,
}

// Message structures
#[derive(Serialize, Clone)]
pub struct Message {
    pub id: String,
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

// User structures
#[derive(Serialize, Clone, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

// Action Definitions

#[derive(Clone, Debug)]
pub struct ActionDefinition {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub category: ActionCategory,
    pub cta_label: &'static str,
    pub fields: Vec<ActionFieldDefinition>,
}

impl ActionDefinition {
    pub fn build_args(
        &self,
        values: &HashMap<String, String>,
    ) -> Result<Vec<String>, crate::error::GuiError> {
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
            other => Err(crate::error::GuiError::UnknownAction(other.to_string())),
        }
    }

    fn required_value(
        &self,
        values: &HashMap<String, String>,
        field_id: &str,
    ) -> Result<String, crate::error::GuiError> {
        let value = self.value_or_default(values, field_id);
        if value.trim().is_empty() {
            return Err(crate::error::GuiError::Validation {
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

#[derive(Clone, Debug)]
pub struct ActionFieldDefinition {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: FieldKind,
    pub placeholder: Option<&'static str>,
    pub helper_text: Option<&'static str>,
    pub required: bool,
    pub default_value: Option<&'static str>,
    pub options: Vec<FieldOption>,
}

impl ActionFieldDefinition {
    pub fn text_area(id: &'static str, label: &'static str) -> Self {
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

    pub fn text(id: &'static str, label: &'static str) -> Self {
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

    pub fn select(
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

    pub fn with_placeholder(mut self, placeholder: &'static str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn with_helper_text(mut self, helper_text: &'static str) -> Self {
        self.helper_text = Some(helper_text);
        self
    }

    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FieldOption {
    pub value: &'static str,
    pub label: &'static str,
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldKind {
    Text,
    TextArea,
    Select,
}

#[derive(Clone, Copy, Debug)]
pub enum ActionCategory {
    Launchpad,
    Collaboration,
    Quality,
}

impl ActionCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionCategory::Launchpad => "Launchpad",
            ActionCategory::Collaboration => "Collaboration",
            ActionCategory::Quality => "Quality",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionMetadata {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub cta_label: &'static str,
    pub fields: Vec<ActionField>,
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
pub struct ActionField {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: FieldKind,
    pub placeholder: Option<&'static str>,
    pub helper_text: Option<&'static str>,
    pub required: bool,
    pub default_value: Option<&'static str>,
    pub options: Vec<FieldOption>,
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

pub fn action_definitions() -> Vec<ActionDefinition> {
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
