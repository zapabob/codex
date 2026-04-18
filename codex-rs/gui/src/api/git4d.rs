use axum::body::Body;
use axum::http::{StatusCode, header};
use axum::{Json, extract::Path, response::Response};
use codex_core::git4d_accelerated::{
    Git4DAcceleratedVisualizer, Git4DCapabilitySnapshot, Git4DMode, read_capabilities,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::info;

use crate::error::GuiError;

// Git4D Visualization API
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Git4DLaunchRequest {
    pub mode: String,
    pub repository_path: Option<String>,
    #[serde(default)]
    pub virtual_desktop: Option<bool>, // Optional: client-side VirtualDesktop detection result
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Git4DLaunchResponse {
    pub session_id: String,
    pub status: String,
    pub message: String,
    pub requested_mode: String,
    pub effective_mode: String,
    pub platform: Option<String>, // Detected platform (VirtualDesktop, WebXR, etc.)
    pub device_name: Option<String>, // Device name if available
    pub fallback_reason: Option<String>,
    pub events_path: String,
    pub capability_path: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Git4DCapabilityResponse {
    pub requested_mode: String,
    pub effective_mode: String,
    pub supported: bool,
    pub device_available: bool,
    pub platform: Option<String>,
    pub device_name: Option<String>,
    pub fallback_reason: Option<String>,
    pub transport: String,
}

fn parse_git4d_mode(mode: &str) -> Result<Git4DMode, GuiError> {
    mode.parse::<Git4DMode>().map_err(|err| GuiError::Validation {
        field: "mode".to_string(),
        message: err.to_string(),
    })
}

fn build_capability_response(snapshot: Git4DCapabilitySnapshot) -> Git4DCapabilityResponse {
    Git4DCapabilityResponse {
        requested_mode: snapshot.requested_mode.as_str().to_string(),
        effective_mode: snapshot.effective_mode.as_str().to_string(),
        supported: true,
        device_available: snapshot.native_supported,
        platform: snapshot.platform,
        device_name: snapshot.device_name,
        fallback_reason: snapshot.fallback_reason,
        transport: "sse".to_string(),
    }
}

#[axum::debug_handler]
pub async fn launch_git4d_visualization(
    Json(payload): Json<Git4DLaunchRequest>,
) -> Result<Json<Git4DLaunchResponse>, GuiError> {
    let mode = payload.mode.trim().to_ascii_lowercase();
    let requested_mode = parse_git4d_mode(&mode)?;
    let repository_path = payload
        .repository_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    // Validate repository path exists
    if !repository_path.exists() {
        return Err(GuiError::Validation {
            field: "repository_path".to_string(),
            message: format!(
                "Repository path does not exist: {}",
                repository_path.display()
            ),
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

    let session = Git4DAcceleratedVisualizer::launch_session(repository_path.clone(), requested_mode)
        .await
        .map_err(|e| GuiError::Validation {
            field: "repository_path".to_string(),
            message: format!("Failed to launch visualization: {e}"),
        })?;
    let capability = build_capability_response(Git4DCapabilitySnapshot {
        requested_mode: session.requested_mode,
        effective_mode: session.effective_mode,
        native_supported: session.requested_mode == session.effective_mode,
        platform: session.platform.clone(),
        device_name: session.device_name.clone(),
        fallback_reason: session.fallback_reason.clone(),
    });

    let session_id = session.session_id.clone();

    info!(
        session_id = session_id.as_str(),
        mode = capability.effective_mode.as_str(),
        repository_path = ?repository_path,
        requested_mode = capability.requested_mode.as_str(),
        platform = ?capability.platform,
        fallback_reason = ?capability.fallback_reason,
        "Git4D visualization session started"
    );

    let message = match (
        capability.platform.as_deref(),
        capability.device_name.as_deref(),
        capability.fallback_reason.as_deref(),
    ) {
        (_, _, Some(reason)) => format!(
            "Git4D visualization started in {} mode (requested {} mode, fallback reason: {})",
            capability.effective_mode, capability.requested_mode, reason
        ),
        (Some("Desktop"), _, None) | (None, _, None) => {
            format!(
                "Git4D visualization started in {} mode",
                capability.effective_mode
            )
        }
        (Some(platform), device_name, None) => {
            let device_suffix = device_name
                .map(|name| format!(" ({name})"))
                .unwrap_or_default();
            format!(
                "Git4D visualization started in {} mode with {} device{}",
                capability.effective_mode, platform, device_suffix
            )
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
        requested_mode: capability.requested_mode.clone(),
        effective_mode: capability.effective_mode,
        platform: capability.platform,
        device_name: capability.device_name,
        fallback_reason: capability.fallback_reason,
        events_path: format!("/api/visualization/git4d/{session_id}/events"),
        capability_path: format!("/api/visualization/git4d/capabilities/{mode}"),
    }))
}

#[axum::debug_handler]
pub async fn get_git4d_capabilities(
    Path(mode): Path<String>,
) -> Result<Json<Git4DCapabilityResponse>, GuiError> {
    let mode = mode.trim().to_ascii_lowercase();
    let requested_mode = parse_git4d_mode(&mode)?;
    let capability = read_capabilities(requested_mode)
        .await
        .map_err(|e| GuiError::Validation {
            field: "mode".to_string(),
            message: format!("Failed to read Git4D capabilities: {e}"),
        })?;

    Ok(Json(build_capability_response(capability)))
}

// Git4D Session List API
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Git4DSessionInfo {
    pub session_id: String,
    pub mode: String,
    pub repository_path: String,
    pub status: String,
    pub created_at: String,
    pub last_activity: String,
    pub events_path: String,
}

#[axum::debug_handler]
pub async fn list_git4d_sessions() -> Json<Vec<Git4DSessionInfo>> {
    let sessions = Git4DAcceleratedVisualizer::list_session_snapshots();

    let session_infos: Vec<Git4DSessionInfo> = sessions
        .into_iter()
        .map(|session| {
            let session_id = session.session_id.clone();
            Git4DSessionInfo {
                mode: session.effective_mode.as_str().to_string(),
                repository_path: session.repository_path.to_string_lossy().to_string(),
                status: session.status.as_str().to_string(),
                created_at: format!("{}ms", session.uptime_ms),
                last_activity: format!("{}ms", session.idle_ms),
                events_path: format!("/api/visualization/git4d/{session_id}/events"),
                session_id,
            }
        })
        .collect();

    Json(session_infos)
}

// Git4D Events SSE Stream
#[axum::debug_handler]
pub async fn git4d_events_stream(Path(session_id): Path<String>) -> Result<Response, GuiError> {
    use tokio_stream::{StreamExt as _, wrappers::BroadcastStream};

    // Get session event receiver
    let receiver =
        Git4DAcceleratedVisualizer::get_session_event_receiver(&session_id).ok_or_else(|| {
            GuiError::Validation {
                field: "session_id".to_string(),
                message: format!("Session not found: {}", session_id),
            }
        })?;

    // Convert broadcast receiver to SSE stream
    let stream = BroadcastStream::new(receiver).map(|msg| {
        let frame = match msg {
            Ok(event) => {
                let event_data = serde_json::to_string(&event.event).unwrap_or_else(|_| {
                    r#"{"type":"error","message":"serialization_failed"}"#.to_string()
                });
                format!("id: {}\ndata: {}\n\n", event.sequence, event_data)
            }
            Err(_) => "data: {\"type\":\"error\",\"message\":\"receive_failed\"}\n\n".to_string(),
        };

        Ok::<_, axum::Error>(frame)
    });

    // Create SSE response with proper headers
    let body = Body::from_stream(stream);
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/event-stream")
        .header(header::CACHE_CONTROL, "no-cache")
        .header(header::CONNECTION, "keep-alive")
        .body(body)
        .map_err(|e| GuiError::Database(format!("Failed to create SSE response: {}", e)))?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::build_capability_response;
    use codex_core::git4d_accelerated::Git4DCapabilitySnapshot;
    use codex_core::git4d_accelerated::Git4DMode;

    #[test]
    fn capability_response_keeps_desktop_mode_when_requested() {
        let response = build_capability_response(Git4DCapabilitySnapshot {
            requested_mode: Git4DMode::Desktop,
            effective_mode: Git4DMode::Desktop,
            native_supported: true,
            platform: Some("Desktop".to_string()),
            device_name: None,
            fallback_reason: None,
        });

        assert_eq!(response.requested_mode, "desktop");
        assert_eq!(response.effective_mode, "desktop");
        assert!(response.device_available);
        assert_eq!(response.platform.as_deref(), Some("Desktop"));
        assert_eq!(response.fallback_reason, None);
    }

    #[test]
    fn capability_response_falls_back_to_desktop_when_device_is_missing() {
        let response = build_capability_response(Git4DCapabilitySnapshot {
            requested_mode: Git4DMode::Ar,
            effective_mode: Git4DMode::Desktop,
            native_supported: false,
            platform: Some("Desktop".to_string()),
            device_name: None,
            fallback_reason: Some("OpenXR runtime missing".to_string()),
        });

        assert_eq!(response.requested_mode, "ar");
        assert_eq!(response.effective_mode, "desktop");
        assert!(!response.device_available);
        assert_eq!(response.platform.as_deref(), Some("Desktop"));
        assert_eq!(
            response.fallback_reason.as_deref(),
            Some("OpenXR runtime missing")
        );
    }
}
