use codex_utils_absolute_path::AbsolutePathBuf;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "lowercase")]
#[ts(rename_all = "lowercase")]
#[ts(export_to = "v2/")]
pub enum Git4DMode {
    Desktop,
    Vr,
    Ar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "lowercase")]
#[ts(rename_all = "lowercase")]
#[ts(export_to = "v2/")]
pub enum Git4DSessionStatus {
    Starting,
    Active,
    Paused,
    Stopping,
    Stopped,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub enum Git4DSessionWatchReplayMode {
    Buffered,
    LiveOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DCapabilitiesReadParams {
    pub mode: Git4DMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DCapabilitiesResponse {
    pub requested_mode: Git4DMode,
    pub effective_mode: Git4DMode,
    pub native_supported: bool,
    pub platform: Option<String>,
    pub device_name: Option<String>,
    pub fallback_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionSummary {
    pub session_id: String,
    pub repository_path: AbsolutePathBuf,
    pub requested_mode: Git4DMode,
    pub effective_mode: Git4DMode,
    pub status: Git4DSessionStatus,
    pub platform: Option<String>,
    pub device_name: Option<String>,
    pub fallback_reason: Option<String>,
    pub uptime_ms: u64,
    pub idle_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionStartParams {
    #[ts(optional = nullable)]
    pub repository_path: Option<AbsolutePathBuf>,
    pub mode: Git4DMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionStartResponse {
    pub session: Git4DSessionSummary,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionListParams {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionListResponse {
    pub sessions: Vec<Git4DSessionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionWatchParams {
    pub session_id: String,
    pub replay_mode: Git4DSessionWatchReplayMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionWatchResponse {
    pub session: Git4DSessionSummary,
    pub replay_mode: Git4DSessionWatchReplayMode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionUnwatchParams {
    pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionUnwatchResponse {
    pub session_id: String,
    pub unsubscribed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(tag = "type", rename_all = "snake_case")]
#[ts(export_to = "v2/")]
pub enum Git4DSessionEvent {
    CommitsLoaded {
        commit_count: usize,
    },
    BranchesUpdated {
        branch_count: usize,
        branch_names: Vec<String>,
    },
    CameraUpdated {
        position: [f32; 3],
        target: [f32; 3],
    },
    RenderComplete {
        pixel_bytes: usize,
    },
    InteractionProcessed {
        interaction: String,
    },
    Error {
        message: String,
    },
    SessionStatusChanged {
        status: Git4DSessionStatus,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct Git4DSessionEventNotification {
    pub session_id: String,
    pub sequence: u64,
    pub event: Git4DSessionEvent,
}

#[cfg(test)]
mod tests {
    use super::*;
    use codex_utils_absolute_path::AbsolutePathBuf;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    fn absolute_path(path: &str) -> AbsolutePathBuf {
        let value = format!("/{}", path.trim_start_matches('/'));
        AbsolutePathBuf::try_from(std::path::PathBuf::from(value)).expect("absolute path")
    }

    #[test]
    fn git4d_session_start_params_round_trip() {
        let repository_path = absolute_path("tmp/repo");
        let params = Git4DSessionStartParams {
            repository_path: Some(repository_path.clone()),
            mode: Git4DMode::Vr,
        };

        let value = serde_json::to_value(&params).expect("serialize git4d/session/start params");
        assert_eq!(
            value,
            json!({
                "repositoryPath": repository_path.display().to_string(),
                "mode": "vr",
            })
        );

        let decoded = serde_json::from_value::<Git4DSessionStartParams>(value)
            .expect("deserialize git4d/session/start params");
        assert_eq!(decoded, params);
    }

    #[test]
    fn git4d_capabilities_response_round_trip() {
        let response = Git4DCapabilitiesResponse {
            requested_mode: Git4DMode::Ar,
            effective_mode: Git4DMode::Desktop,
            native_supported: false,
            platform: Some("Desktop".to_string()),
            device_name: None,
            fallback_reason: Some("OpenXR runtime missing".to_string()),
        };

        let value =
            serde_json::to_value(&response).expect("serialize git4d/capabilities/read response");
        assert_eq!(
            value,
            json!({
                "requestedMode": "ar",
                "effectiveMode": "desktop",
                "nativeSupported": false,
                "platform": "Desktop",
                "deviceName": null,
                "fallbackReason": "OpenXR runtime missing",
            })
        );

        let decoded = serde_json::from_value::<Git4DCapabilitiesResponse>(value)
            .expect("deserialize git4d/capabilities/read response");
        assert_eq!(decoded, response);
    }

    #[test]
    fn git4d_session_event_notification_round_trip() {
        let notification = Git4DSessionEventNotification {
            session_id: "git4d_123".to_string(),
            sequence: 7,
            event: Git4DSessionEvent::SessionStatusChanged {
                status: Git4DSessionStatus::Active,
            },
        };

        let value = serde_json::to_value(&notification)
            .expect("serialize git4d/session/event notification");
        assert_eq!(
            value,
            json!({
                "sessionId": "git4d_123",
                "sequence": 7,
                "event": {
                    "type": "session_status_changed",
                    "status": "active",
                },
            })
        );

        let decoded = serde_json::from_value::<Git4DSessionEventNotification>(value)
            .expect("deserialize git4d/session/event notification");
        assert_eq!(decoded, notification);
    }
}
