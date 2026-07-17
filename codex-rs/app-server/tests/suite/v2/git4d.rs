use std::process::Command;
use std::time::Duration;

use anyhow::Result;
use app_test_support::TestAppServer;
use app_test_support::to_response;
use codex_app_server_protocol::Git4DCapabilitiesReadParams;
use codex_app_server_protocol::Git4DCapabilitiesResponse;
use codex_app_server_protocol::Git4DMode;
use codex_app_server_protocol::Git4DSessionEvent;
use codex_app_server_protocol::Git4DSessionEventNotification;
use codex_app_server_protocol::Git4DSessionListParams;
use codex_app_server_protocol::Git4DSessionListResponse;
use codex_app_server_protocol::Git4DSessionStartParams;
use codex_app_server_protocol::Git4DSessionStartResponse;
use codex_app_server_protocol::Git4DSessionStatus;
use codex_app_server_protocol::Git4DSessionUnwatchParams;
use codex_app_server_protocol::Git4DSessionUnwatchResponse;
use codex_app_server_protocol::Git4DSessionWatchParams;
use codex_app_server_protocol::Git4DSessionWatchReplayMode;
use codex_app_server_protocol::Git4DSessionWatchResponse;
use codex_app_server_protocol::JSONRPCNotification;
use codex_app_server_protocol::JSONRPCResponse;
use codex_app_server_protocol::RequestId;
use codex_utils_absolute_path::AbsolutePathBuf;
use pretty_assertions::assert_eq;
use serde::Serialize;
use tempfile::TempDir;
use tokio::time::timeout;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

fn init_git_repo(path: &std::path::Path) {
    let status = Command::new("git")
        .arg("init")
        .arg(path)
        .status()
        .expect("git init should run");
    assert!(status.success(), "git init should succeed");
}

fn session_env_overrides(home: &str) -> [(&str, Option<&str>); 4] {
    [
        ("HOME", Some(home)),
        ("USERPROFILE", Some(home)),
        ("LOCALAPPDATA", Some(home)),
        ("OPENXR_RUNTIME_JSON", None),
    ]
}

async fn send_git4d_request<T: Serialize>(
    app_server: &mut TestAppServer,
    method: &str,
    params: T,
) -> Result<i64> {
    app_server
        .send_raw_request(method, Some(serde_json::to_value(params)?))
        .await
}

#[tokio::test]
async fn git4d_capabilities_read_prefers_desktop_fallback_without_runtime() -> Result<()> {
    let codex_home = TempDir::new()?;
    let home = codex_home.path().to_string_lossy().into_owned();
    let mut mcp = TestAppServer::builder()
        .with_codex_home(codex_home.path())
        .without_auto_env()
        .with_env_overrides(&session_env_overrides(&home))
        .build()
        .await?;
    timeout(DEFAULT_TIMEOUT, mcp.initialize()).await??;

    let request_id = send_git4d_request(
        &mut mcp,
        "git4d/capabilities/read",
        Git4DCapabilitiesReadParams {
            mode: Git4DMode::Ar,
        },
    )
    .await?;
    let response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    let response: Git4DCapabilitiesResponse = to_response(response)?;

    assert_eq!(response.requested_mode, Git4DMode::Ar);
    assert_eq!(response.effective_mode, Git4DMode::Desktop);
    assert!(!response.native_supported);
    assert!(response.fallback_reason.is_some());
    Ok(())
}

#[tokio::test]
async fn git4d_session_start_and_list_round_trip() -> Result<()> {
    let codex_home = TempDir::new()?;
    let repo = TempDir::new()?;
    init_git_repo(repo.path());
    let home = codex_home.path().to_string_lossy().into_owned();
    let mut mcp = TestAppServer::builder()
        .with_codex_home(codex_home.path())
        .without_auto_env()
        .with_env_overrides(&session_env_overrides(&home))
        .build()
        .await?;
    timeout(DEFAULT_TIMEOUT, mcp.initialize()).await??;

    let request_id = send_git4d_request(
        &mut mcp,
        "git4d/session/start",
        Git4DSessionStartParams {
            repository_path: Some(AbsolutePathBuf::try_from(repo.path())?),
            mode: Git4DMode::Desktop,
        },
    )
    .await?;
    let response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    let response: Git4DSessionStartResponse = to_response(response)?;
    let started_session = response.session.clone();

    assert_eq!(started_session.requested_mode, Git4DMode::Desktop);
    assert_eq!(started_session.effective_mode, Git4DMode::Desktop);
    assert_eq!(started_session.status, Git4DSessionStatus::Starting);

    let request_id = send_git4d_request(
        &mut mcp,
        "git4d/session/list",
        Git4DSessionListParams::default(),
    )
    .await?;
    let response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    let response: Git4DSessionListResponse = to_response(response)?;
    let listed_session = response
        .sessions
        .into_iter()
        .find(|session| session.session_id == started_session.session_id)
        .unwrap_or_else(|| panic!("session {} should be listed", started_session.session_id));

    assert_eq!(listed_session.requested_mode, Git4DMode::Desktop);
    assert_eq!(listed_session.effective_mode, Git4DMode::Desktop);
    assert!(listed_session.uptime_ms >= listed_session.idle_ms);
    Ok(())
}

#[tokio::test]
async fn git4d_session_watch_buffered_replays_status_and_then_observes_follow_up() -> Result<()> {
    let codex_home = TempDir::new()?;
    let repo = TempDir::new()?;
    init_git_repo(repo.path());
    let home = codex_home.path().to_string_lossy().into_owned();
    let mut mcp = TestAppServer::builder()
        .with_codex_home(codex_home.path())
        .without_auto_env()
        .with_env_overrides(&session_env_overrides(&home))
        .build()
        .await?;
    timeout(DEFAULT_TIMEOUT, mcp.initialize()).await??;

    let request_id = send_git4d_request(
        &mut mcp,
        "git4d/session/start",
        Git4DSessionStartParams {
            repository_path: Some(AbsolutePathBuf::try_from(repo.path())?),
            mode: Git4DMode::Desktop,
        },
    )
    .await?;
    let response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    let response: Git4DSessionStartResponse = to_response(response)?;

    let request_id = send_git4d_request(
        &mut mcp,
        "git4d/session/watch",
        Git4DSessionWatchParams {
            session_id: response.session.session_id.clone(),
            replay_mode: Git4DSessionWatchReplayMode::Buffered,
        },
    )
    .await?;
    let watch_response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    let _: Git4DSessionWatchResponse = to_response(watch_response)?;

    let first_notification = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_notification_message("git4d/session/event"),
    )
    .await??;
    let first_notification = parse_git4d_event(first_notification)?;
    assert_eq!(first_notification.sequence, 1);
    assert_eq!(
        first_notification.event,
        Git4DSessionEvent::SessionStatusChanged {
            status: Git4DSessionStatus::Starting,
        }
    );

    let second_notification = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_notification_message("git4d/session/event"),
    )
    .await??;
    let second_notification = parse_git4d_event(second_notification)?;
    assert!(second_notification.sequence > first_notification.sequence);
    assert_eq!(
        second_notification.event,
        Git4DSessionEvent::SessionStatusChanged {
            status: Git4DSessionStatus::Active,
        }
    );
    Ok(())
}

#[tokio::test]
async fn git4d_session_unwatch_stops_future_notifications() -> Result<()> {
    let codex_home = TempDir::new()?;
    let repo = TempDir::new()?;
    init_git_repo(repo.path());
    let home = codex_home.path().to_string_lossy().into_owned();
    let mut mcp = TestAppServer::builder()
        .with_codex_home(codex_home.path())
        .without_auto_env()
        .with_env_overrides(&session_env_overrides(&home))
        .build()
        .await?;
    timeout(DEFAULT_TIMEOUT, mcp.initialize()).await??;

    let request_id = send_git4d_request(
        &mut mcp,
        "git4d/session/start",
        Git4DSessionStartParams {
            repository_path: Some(AbsolutePathBuf::try_from(repo.path())?),
            mode: Git4DMode::Desktop,
        },
    )
    .await?;
    let response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    let response: Git4DSessionStartResponse = to_response(response)?;

    let request_id = send_git4d_request(
        &mut mcp,
        "git4d/session/watch",
        Git4DSessionWatchParams {
            session_id: response.session.session_id.clone(),
            replay_mode: Git4DSessionWatchReplayMode::LiveOnly,
        },
    )
    .await?;
    let watch_response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    let _: Git4DSessionWatchResponse = to_response(watch_response)?;

    let request_id = send_git4d_request(
        &mut mcp,
        "git4d/session/unwatch",
        Git4DSessionUnwatchParams {
            session_id: response.session.session_id.clone(),
        },
    )
    .await?;
    let unwatch_response: JSONRPCResponse = timeout(
        DEFAULT_TIMEOUT,
        mcp.read_stream_until_response_message(RequestId::Integer(request_id)),
    )
    .await??;
    let unwatch_response: Git4DSessionUnwatchResponse = to_response(unwatch_response)?;
    assert!(unwatch_response.unsubscribed);

    let no_notification = timeout(
        Duration::from_millis(200),
        mcp.read_stream_until_notification_message("git4d/session/event"),
    )
    .await;
    assert!(
        no_notification.is_err(),
        "no Git4D notifications should arrive after unwatch"
    );
    Ok(())
}

fn parse_git4d_event(notification: JSONRPCNotification) -> Result<Git4DSessionEventNotification> {
    let params = notification
        .params
        .ok_or_else(|| anyhow::anyhow!("git4d/session/event notification should have params"))?;
    Ok(serde_json::from_value(params)?)
}
