use axum::Json;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct PlansState {
    pub db: Arc<SqlitePool>,
    pub cli_path: Arc<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: String,
    pub title: String,
    pub goal: String,
    pub approach: String,
    pub mode: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub approved_by: Option<String>,
    pub rejected_reason: Option<String>,
    pub budget: PlanBudget,
    pub work_items: Vec<WorkItem>,
    pub risks: Vec<Risk>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanBudget {
    pub session_cap: Option<u64>,
    pub cap_min: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkItem {
    pub name: String,
    pub files_touched: Vec<String>,
    pub diff_contract: String,
    pub tests: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Risk {
    pub item: String,
    pub mitigation: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub title: String,
    pub mode: Option<String>,
    pub budget_tokens: Option<u64>,
    pub budget_time: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct RejectPlanRequest {
    pub reason: String,
}

pub async fn list_plans(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Plan>>, PlansError> {
    init_db(&state.db).await?;

    let state_filter = params.get("state");

    // Execute codex Plan list command
    let mut cmd = Command::new(state.cli_path.as_str());
    cmd.arg("Plan").arg("list").arg("--json");

    if let Some(state) = state_filter {
        cmd.arg("--state").arg(state);
    }

    let output = cmd
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(PlansError::CommandExecution(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let plans: Vec<Plan> = serde_json::from_slice(&output.stdout)
        .map_err(|e| PlansError::ParseError(e.to_string()))?;

    Ok(Json(plans))
}

pub async fn create_plan(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
    Json(request): Json<CreatePlanRequest>,
) -> Result<Json<Plan>, PlansError> {
    init_db(&state.db).await?;

    let mode = request.mode.unwrap_or_else(|| "orchestrated".to_string());
    let budget_tokens = request.budget_tokens.unwrap_or(100000);
    let budget_time = request.budget_time.unwrap_or(30);

    // Execute codex Plan create command
    let output = Command::new(state.cli_path.as_str())
        .arg("Plan")
        .arg("create")
        .arg(&request.title)
        .arg("--mode")
        .arg(&mode)
        .arg("--budget-tokens")
        .arg(budget_tokens.to_string())
        .arg("--budget-time")
        .arg(budget_time.to_string())
        .arg("--json")
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(PlansError::CommandExecution(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let plan: Plan = serde_json::from_slice(&output.stdout)
        .map_err(|e| PlansError::ParseError(e.to_string()))?;

    Ok(Json(plan))
}

pub async fn get_plan(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
    Path(id): Path<String>,
) -> Result<Json<Plan>, PlansError> {
    init_db(&state.db).await?;

    // Execute codex Plan status command
    let output = Command::new(state.cli_path.as_str())
        .arg("Plan")
        .arg("status")
        .arg(&id)
        .arg("--json")
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(PlansError::CommandExecution(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let plan: Plan = serde_json::from_slice(&output.stdout)
        .map_err(|e| PlansError::ParseError(e.to_string()))?;

    Ok(Json(plan))
}

pub async fn approve_plan(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
    Path(id): Path<String>,
) -> Result<Json<Plan>, PlansError> {
    init_db(&state.db).await?;

    // Execute codex Plan approve command
    let output = Command::new(state.cli_path.as_str())
        .arg("Plan")
        .arg("approve")
        .arg(&id)
        .arg("--json")
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(PlansError::CommandExecution(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let plan: Plan = serde_json::from_slice(&output.stdout)
        .map_err(|e| PlansError::ParseError(e.to_string()))?;

    Ok(Json(plan))
}

pub async fn reject_plan(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
    Path(id): Path<String>,
    Json(request): Json<RejectPlanRequest>,
) -> Result<Json<Plan>, PlansError> {
    init_db(&state.db).await?;

    // Execute codex Plan reject command
    let output = Command::new(state.cli_path.as_str())
        .arg("Plan")
        .arg("reject")
        .arg(&id)
        .arg("--reason")
        .arg(&request.reason)
        .arg("--json")
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(PlansError::CommandExecution(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let plan: Plan = serde_json::from_slice(&output.stdout)
        .map_err(|e| PlansError::ParseError(e.to_string()))?;

    Ok(Json(plan))
}

pub async fn execute_plan(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, PlansError> {
    init_db(&state.db).await?;

    // Execute codex Plan execute command
    let output = Command::new(state.cli_path.as_str())
        .arg("Plan")
        .arg("execute")
        .arg(&id)
        .arg("--json")
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(PlansError::CommandExecution(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let result: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| PlansError::ParseError(e.to_string()))?;

    Ok(Json(result))
}

pub async fn export_plan(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, PlansError> {
    init_db(&state.db).await?;

    let format = params.get("format").map(|s| s.as_str()).unwrap_or("both");

    // Execute codex Plan export command
    let output = Command::new(state.cli_path.as_str())
        .arg("Plan")
        .arg("export")
        .arg(&id)
        .arg("--format")
        .arg(format)
        .arg("--json")
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(PlansError::CommandExecution(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    let result: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| PlansError::ParseError(e.to_string()))?;

    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct TogglePlanModeRequest {
    pub enabled: bool,
}

pub async fn toggle_plan_mode(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
    Json(request): Json<TogglePlanModeRequest>,
) -> Result<StatusCode, PlansError> {
    let flag = if request.enabled { "on" } else { "off" };

    // Execute codex Plan toggle command
    let output = Command::new(state.cli_path.as_str())
        .arg("Plan")
        .arg("toggle")
        .arg(flag)
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        return Err(PlansError::CommandExecution(format!(
            "Command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(StatusCode::OK)
}

pub async fn get_plan_mode_status(
    axum::extract::Extension(state): axum::extract::Extension<PlansState>,
) -> Result<Json<serde_json::Value>, PlansError> {
    // Execute codex plan mode-status command
    let output = Command::new(state.cli_path.as_str())
        .arg("plan")
        .arg("mode-status")
        .arg("--json")
        .output()
        .await
        .map_err(|e| PlansError::CommandExecution(e.to_string()))?;

    if !output.status.success() {
        // Return default if command fails
        return Ok(Json(serde_json::json!({
            "enabled": false,
            "timestamp": Utc::now().to_rfc3339()
        })));
    }

    let result: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| PlansError::ParseError(e.to_string()))?;

    Ok(Json(result))
}

async fn init_db(pool: &SqlitePool) -> Result<(), PlansError> {
    // Plans are managed by the CLI, so we don't need a separate table
    // But we can store plan metadata if needed
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS plan_metadata (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            state TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| PlansError::Database(e.to_string()))?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum PlansError {
    #[error("Command execution error: {0}")]
    CommandExecution(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Database error: {0}")]
    Database(String),
}

impl IntoResponse for PlansError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            PlansError::CommandExecution(msg) => (StatusCode::BAD_GATEWAY, msg),
            PlansError::ParseError(msg) => (StatusCode::BAD_REQUEST, msg),
            PlansError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(serde_json::json!({
            "error": message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
