use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GuiError {
    #[error("action `{0}` not found")]
    ActionNotFound(String),
    #[error("{message}")]
    Validation { field: String, message: String },
    #[error("action `{0}` is not supported")]
    UnknownAction(String),
    #[error("failed to run command: {0}")]
    CommandIo(#[from] std::io::Error),
    #[error("database error: {0}")]
    Database(String),
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
            GuiError::Database(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                error.clone(),
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
