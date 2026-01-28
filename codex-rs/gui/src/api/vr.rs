use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct VRState {
    // VR state can be managed in memory or via codex-core
}

#[derive(Debug, Serialize)]
pub struct VRStatus {
    pub supported: bool,
    pub vr_available: bool,
    pub ar_available: bool,
    pub hand_tracking_available: bool,
}

#[derive(Debug, Deserialize)]
pub struct VRSessionRequest {
    pub mode: String, // "vr" or "ar"
}

#[derive(Debug, Serialize)]
pub struct VRSessionResponse {
    pub session_id: String,
    pub status: String,
    pub mode: String,
}

pub async fn get_vr_status() -> Json<VRStatus> {
    // Check WebXR support (this would be done client-side, but we can provide a status endpoint)
    Json(VRStatus {
        supported: true, // Would check actual support
        vr_available: true,
        ar_available: true,
        hand_tracking_available: true,
    })
}

pub async fn create_vr_session(
    axum::extract::Extension(_state): axum::extract::Extension<VRState>,
    Json(request): Json<VRSessionRequest>,
) -> Result<Json<VRSessionResponse>, VRError> {
    // VR sessions are managed client-side via WebXR API
    // This endpoint can be used for logging or coordination

    if request.mode != "vr" && request.mode != "ar" {
        return Err(VRError::InvalidMode);
    }

    Ok(Json(VRSessionResponse {
        session_id: uuid::Uuid::new_v4().to_string(),
        status: "created".to_string(),
        mode: request.mode,
    }))
}

#[derive(Debug, thiserror::Error)]
pub enum VRError {
    #[error("Invalid VR mode")]
    InvalidMode,
}

impl IntoResponse for VRError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            VRError::InvalidMode => StatusCode::BAD_REQUEST,
        };

        let body = Json(serde_json::json!({
            "error": self.to_string(),
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
