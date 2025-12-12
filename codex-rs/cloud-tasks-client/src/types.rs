//! Cloud Tasks Types

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

/// Task ID - wrapper around String for type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub String);

/// Cloud Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudTask {
    pub id: String,
    pub name: String,
    pub status: String,
}

/// Task Status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Ready,
    Running,
    Applied,
    Error,
}

/// Attempt Status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AttemptStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for AttemptStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

/// Apply Status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ApplyStatus {
    Success,
    Partial,
    Error,
}

/// Diff Summary
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffSummary {
    pub files_changed: i32,
    pub lines_added: i32,
    pub lines_removed: i32,
}

/// Task Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSummary {
    pub id: TaskId,
    pub title: String,
    pub status: TaskStatus,
    pub updated_at: DateTime<Utc>,
    pub environment_id: Option<String>,
    pub environment_label: Option<String>,
    pub summary: DiffSummary,
    pub is_review: bool,
    pub attempt_total: Option<i32>,
}

/// Turn Attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnAttempt {
    pub turn_id: String,
    pub status: AttemptStatus,
    pub diff: String,
    pub messages: Vec<String>,
    pub attempt_placement: Option<String>,
}

/// Created Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedTask {
    pub id: TaskId,
    pub name: String,
}

/// Task Text - detailed task information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskText {
    pub prompt: Option<String>,
    pub messages: Vec<String>,
    pub turn_id: Option<String>,
    pub sibling_turn_ids: Vec<String>,
    pub attempt_placement: Option<i32>,
    pub attempt_status: AttemptStatus,
}

/// Apply Outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyOutcome {
    pub status: ApplyStatus,
    pub message: String,
    pub conflict_paths: Vec<String>,
    pub skipped_paths: Vec<String>,
}
