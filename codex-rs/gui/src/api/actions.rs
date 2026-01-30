use axum::{
    Json,
    extract::{Path, State},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use tokio::process::Command;
use tracing::warn;
use uuid::Uuid;

use crate::error::GuiError;
use crate::state::AppState;
use crate::types::ActionMetadata;

#[axum::debug_handler]
pub async fn list_actions(State(state): State<AppState>) -> Json<Vec<ActionMetadata>> {
    let payload = state.actions.iter().map(ActionMetadata::from).collect();
    Json(payload)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRequest {
    pub values: HashMap<String, String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionResponse {
    pub id: Uuid,
    pub action_id: String,
    pub command: Vec<String>,
    pub executed_at: chrono::DateTime<Utc>,
    pub duration_ms: u128,
    pub status: ExecutionStatus,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Completed,
    Failed,
}

#[axum::debug_handler]
pub async fn execute_action(
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
