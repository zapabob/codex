use axum::body::Body;
use axum::http::{StatusCode, header};
use axum::{Json, extract::Path, response::Response};
use codex_core::git4d_accelerated::{
    DeviceAvailability, Git4DAcceleratedVisualizer, check_vr_ar_device_availability,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, warn};

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

fn validate_git4d_mode(mode: &str) -> Result<(), GuiError> {
    if ["desktop", "vr", "ar"].contains(&mode) {
        return Ok(());
    }

    Err(GuiError::Validation {
        field: "mode".to_string(),
        message: format!("Invalid mode: {mode}. Must be one of: desktop, vr, ar"),
    })
}

fn build_capability_response(
    requested_mode: &str,
    device_availability: &DeviceAvailability,
) -> Git4DCapabilityResponse {
    match device_availability {
        DeviceAvailability::Available {
            platform,
            device_name,
        } => Git4DCapabilityResponse {
            requested_mode: requested_mode.to_string(),
            effective_mode: requested_mode.to_string(),
            supported: true,
            device_available: true,
            platform: Some(format!("{platform:?}")),
            device_name: device_name.clone(),
            fallback_reason: None,
            transport: "sse".to_string(),
        },
        DeviceAvailability::NotAvailable { reason } => Git4DCapabilityResponse {
            requested_mode: requested_mode.to_string(),
            effective_mode: "desktop".to_string(),
            supported: true,
            device_available: false,
            platform: Some("Desktop".to_string()),
            device_name: None,
            fallback_reason: Some(reason.clone()),
            transport: "sse".to_string(),
        },
        DeviceAvailability::Desktop => Git4DCapabilityResponse {
            requested_mode: requested_mode.to_string(),
            effective_mode: "desktop".to_string(),
            supported: true,
            device_available: true,
            platform: Some("Desktop".to_string()),
            device_name: None,
            fallback_reason: None,
            transport: "sse".to_string(),
        },
    }
}

#[axum::debug_handler]
pub async fn launch_git4d_visualization(
    Json(payload): Json<Git4DLaunchRequest>,
) -> Result<Json<Git4DLaunchResponse>, GuiError> {
    let mode = payload.mode.trim().to_ascii_lowercase();
    let repository_path = payload
        .repository_path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    // Validate mode
    validate_git4d_mode(&mode)?;

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

    // Check VR/AR device availability (if mode is vr or ar)
    let device_availability = if mode == "vr" || mode == "ar" {
        check_vr_ar_device_availability(&mode)
            .await
            .unwrap_or_else(|e| {
                warn!("Failed to check device availability: {}", e);
                DeviceAvailability::NotAvailable {
                    reason: format!("Device check failed: {}", e),
                }
            })
    } else {
        DeviceAvailability::Desktop
    };

    let capability = build_capability_response(&mode, &device_availability);
    if let Some(reason) = capability.fallback_reason.as_deref() {
        warn!("VR/AR device not available ({reason}), falling back to desktop mode");
    }

    // Launch Git4D visualization session
    let session = Git4DAcceleratedVisualizer::launch_for_gui(
        repository_path.clone(),
        capability.effective_mode.clone(),
    )
    .await
    .map_err(|e| GuiError::Validation {
        field: "repository_path".to_string(),
        message: format!("Failed to launch visualization: {}", e),
    })?;

    let session_id = session.session_id.clone();

    info!(
        session_id = session_id.as_str(),
        mode = capability.effective_mode.as_str(),
        repository_path = ?repository_path,
        device = ?device_availability,
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
    validate_git4d_mode(&mode)?;

    let device_availability = if mode == "vr" || mode == "ar" {
        check_vr_ar_device_availability(&mode)
            .await
            .unwrap_or_else(|e| {
                warn!("Failed to check device availability: {}", e);
                DeviceAvailability::NotAvailable {
                    reason: format!("Device check failed: {}", e),
                }
            })
    } else {
        DeviceAvailability::Desktop
    };

    Ok(Json(build_capability_response(&mode, &device_availability)))
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
    let sessions = Git4DAcceleratedVisualizer::list_sessions();

    let session_infos: Vec<Git4DSessionInfo> = sessions
        .into_iter()
        .map(|session| {
            let session_id = session.session_id;
            Git4DSessionInfo {
                mode: session.mode,
                repository_path: session.repository_path.to_string_lossy().to_string(),
                status: format!("{:?}", session.status),
                created_at: format!("{:?}", session.created_at.elapsed()),
                last_activity: format!("{:?}", session.last_activity.elapsed()),
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
        let event_data = match msg {
            Ok(event) => serde_json::to_string(&event).unwrap_or_else(|_| {
                r#"{"type":"error","message":"serialization_failed"}"#.to_string()
            }),
            Err(_) => r#"{"type":"error","message":"receive_failed"}"#.to_string(),
        };

        // Format as SSE event: "data: {json}\n\n"
        Ok::<_, axum::Error>(format!("data: {}\n\n", event_data))
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
    use codex_core::git4d_accelerated::DeviceAvailability;

    #[test]
    fn capability_response_keeps_desktop_mode_when_requested() {
        let response = build_capability_response("desktop", &DeviceAvailability::Desktop);

        assert_eq!(response.requested_mode, "desktop");
        assert_eq!(response.effective_mode, "desktop");
        assert!(response.device_available);
        assert_eq!(response.platform.as_deref(), Some("Desktop"));
        assert_eq!(response.fallback_reason, None);
    }

    #[test]
    fn capability_response_falls_back_to_desktop_when_device_is_missing() {
        let response = build_capability_response(
            "ar",
            &DeviceAvailability::NotAvailable {
                reason: "OpenXR runtime missing".to_string(),
            },
        );

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
