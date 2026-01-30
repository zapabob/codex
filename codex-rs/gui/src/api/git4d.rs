use axum::body::Body;
use axum::http::{StatusCode, header};
use axum::{Json, extract::Path, response::Response};
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
    pub platform: Option<String>, // Detected platform (VirtualDesktop, WebXR, etc.)
    pub device_name: Option<String>, // Device name if available
}

#[axum::debug_handler]
pub async fn launch_git4d_visualization(
    Json(payload): Json<Git4DLaunchRequest>,
) -> Result<Json<Git4DLaunchResponse>, GuiError> {
    let mode = payload.mode.as_str();
    let repository_path = payload
        .repository_path
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
        codex_core::git4d_accelerated::check_vr_ar_device_availability(mode)
            .await
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
        codex_core::git4d_accelerated::DeviceAvailability::Available {
            platform,
            device_name,
        } => {
            let platform_str = format!("{:?}", platform);
            let msg = format!(
                "Git4D visualization started in {} mode with {:?} device{}",
                effective_mode,
                platform,
                device_name
                    .as_ref()
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
        codex_core::git4d_accelerated::DeviceAvailability::Desktop => (
            format!("Git4D visualization started in desktop mode"),
            Some("Desktop".to_string()),
            None,
        ),
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
pub struct Git4DSessionInfo {
    pub session_id: String,
    pub mode: String,
    pub repository_path: String,
    pub status: String,
    pub created_at: String,
    pub last_activity: String,
}

#[axum::debug_handler]
pub async fn list_git4d_sessions() -> Json<Vec<Git4DSessionInfo>> {
    use codex_core::git4d_accelerated::Git4DAcceleratedVisualizer;

    let sessions = Git4DAcceleratedVisualizer::list_sessions();

    let session_infos: Vec<Git4DSessionInfo> = sessions
        .into_iter()
        .map(|session| Git4DSessionInfo {
            session_id: session.session_id,
            mode: session.mode,
            repository_path: session.repository_path.to_string_lossy().to_string(),
            status: format!("{:?}", session.status),
            created_at: format!("{:?}", session.created_at.elapsed()),
            last_activity: format!("{:?}", session.last_activity.elapsed()),
        })
        .collect();

    Json(session_infos)
}

// Git4D Events SSE Stream
#[axum::debug_handler]
pub async fn git4d_events_stream(Path(session_id): Path<String>) -> Result<Response, GuiError> {
    use codex_core::git4d_accelerated::Git4DAcceleratedVisualizer;
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
