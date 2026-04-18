use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use std::time::Duration;

use codex_app_server_protocol::Git4DCapabilitiesReadParams;
use codex_app_server_protocol::Git4DCapabilitiesResponse;
use codex_app_server_protocol::Git4DMode as ApiGit4DMode;
use codex_app_server_protocol::Git4DSessionEvent as ApiGit4DSessionEvent;
use codex_app_server_protocol::Git4DSessionEventNotification;
use codex_app_server_protocol::Git4DSessionListParams;
use codex_app_server_protocol::Git4DSessionListResponse;
use codex_app_server_protocol::Git4DSessionStartParams;
use codex_app_server_protocol::Git4DSessionStartResponse;
use codex_app_server_protocol::Git4DSessionStatus as ApiGit4DSessionStatus;
use codex_app_server_protocol::Git4DSessionSummary;
use codex_app_server_protocol::Git4DSessionUnwatchParams;
use codex_app_server_protocol::Git4DSessionUnwatchResponse;
use codex_app_server_protocol::Git4DSessionWatchParams;
use codex_app_server_protocol::Git4DSessionWatchReplayMode;
use codex_app_server_protocol::Git4DSessionWatchResponse;
use codex_app_server_protocol::JSONRPCErrorError;
use codex_app_server_protocol::ServerNotification;
use codex_core::git4d_accelerated::Git4DAcceleratedVisualizer;
use codex_core::git4d_accelerated::Git4DCapabilitySnapshot;
use codex_core::git4d_accelerated::Git4DEvent as CoreGit4DEvent;
use codex_core::git4d_accelerated::Git4DMode as CoreGit4DMode;
use codex_core::git4d_accelerated::Git4DSequencedEvent;
use codex_core::git4d_accelerated::Git4DSessionSnapshot;
use codex_core::git4d_accelerated::SessionStatus as CoreSessionStatus;
use codex_utils_absolute_path::AbsolutePathBuf;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tracing::warn;

use crate::error_code::INTERNAL_ERROR_CODE;
use crate::error_code::INVALID_REQUEST_ERROR_CODE;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;

type WatchKey = (ConnectionId, String);

struct WatchRegistration {
    watch_id: u64,
    cancellation: CancellationToken,
}

pub(crate) struct Git4DBridge {
    outgoing: Arc<OutgoingMessageSender>,
    background_tasks: TaskTracker,
    watchers: Arc<Mutex<HashMap<WatchKey, WatchRegistration>>>,
    next_watch_id: AtomicU64,
}

impl Git4DBridge {
    pub(crate) fn new(outgoing: Arc<OutgoingMessageSender>) -> Self {
        Self {
            outgoing,
            background_tasks: TaskTracker::new(),
            watchers: Arc::new(Mutex::new(HashMap::new())),
            next_watch_id: AtomicU64::new(1),
        }
    }

    pub(crate) async fn shutdown(&self) {
        self.cancel_all_watches().await;
        self.background_tasks.close();
        if tokio::time::timeout(Duration::from_secs(5), self.background_tasks.wait())
            .await
            .is_err()
        {
            warn!("timed out waiting for Git4D bridge tasks to shut down");
        }
    }

    pub(crate) async fn connection_closed(&self, connection_id: ConnectionId) {
        let cancellations = {
            let mut watchers = self.watchers.lock().await;
            let keys = watchers
                .keys()
                .filter(|(watch_connection_id, _)| *watch_connection_id == connection_id)
                .cloned()
                .collect::<Vec<_>>();
            keys.into_iter()
                .filter_map(|key| {
                    watchers
                        .remove(&key)
                        .map(|registration| registration.cancellation)
                })
                .collect::<Vec<_>>()
        };
        for cancellation in cancellations {
            cancellation.cancel();
        }
    }

    pub(crate) async fn capabilities_read(
        &self,
        request_id: ConnectionRequestId,
        params: Git4DCapabilitiesReadParams,
    ) {
        match codex_core::git4d_accelerated::read_capabilities(to_core_mode(params.mode)).await {
            Ok(snapshot) => {
                self.outgoing
                    .send_response(request_id, to_api_capabilities(snapshot))
                    .await;
            }
            Err(err) => {
                self.send_invalid_request_error(request_id, err.to_string())
                    .await;
            }
        }
    }

    pub(crate) async fn session_start(
        &self,
        request_id: ConnectionRequestId,
        params: Git4DSessionStartParams,
    ) {
        let repository_path = match params.repository_path {
            Some(path) => path.into_path_buf(),
            None => match std::env::current_dir() {
                Ok(path) => path,
                Err(err) => {
                    self.send_internal_error(
                        request_id,
                        format!("failed to resolve current directory: {err}"),
                    )
                    .await;
                    return;
                }
            },
        };

        match Git4DAcceleratedVisualizer::launch_session(repository_path, to_core_mode(params.mode))
            .await
        {
            Ok(session) => {
                match Git4DAcceleratedVisualizer::get_session_snapshot(&session.session_id) {
                    Some(snapshot) => {
                        self.outgoing
                            .send_response(
                                request_id,
                                Git4DSessionStartResponse {
                                    session: match to_api_session_summary(snapshot) {
                                        Ok(summary) => summary,
                                        Err(err) => {
                                            self.send_internal_error(request_id, err).await;
                                            return;
                                        }
                                    },
                                },
                            )
                            .await;
                    }
                    None => {
                        self.send_internal_error(
                            request_id,
                            format!(
                                "started Git4D session `{}` but it could not be reloaded",
                                session.session_id
                            ),
                        )
                        .await;
                    }
                }
            }
            Err(err) => {
                self.send_invalid_request_error(request_id, err.to_string())
                    .await;
            }
        }
    }

    pub(crate) async fn session_list(
        &self,
        request_id: ConnectionRequestId,
        _params: Git4DSessionListParams,
    ) {
        let sessions = match Git4DAcceleratedVisualizer::list_session_snapshots()
            .into_iter()
            .map(to_api_session_summary)
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(sessions) => sessions,
            Err(err) => {
                self.send_internal_error(request_id, err).await;
                return;
            }
        };

        self.outgoing
            .send_response(request_id, Git4DSessionListResponse { sessions })
            .await;
    }

    pub(crate) async fn session_watch(
        &self,
        request_id: ConnectionRequestId,
        params: Git4DSessionWatchParams,
    ) {
        let Some(session_snapshot) =
            Git4DAcceleratedVisualizer::get_session_snapshot(&params.session_id)
        else {
            self.send_invalid_request_error(
                request_id,
                format!("Git4D session not found: {}", params.session_id),
            )
            .await;
            return;
        };
        let Some(receiver) =
            Git4DAcceleratedVisualizer::get_session_event_receiver(&params.session_id)
        else {
            self.send_invalid_request_error(
                request_id,
                format!(
                    "Git4D session not available for watch: {}",
                    params.session_id
                ),
            )
            .await;
            return;
        };
        let replay_events = if matches!(params.replay_mode, Git4DSessionWatchReplayMode::Buffered) {
            Git4DAcceleratedVisualizer::get_session_replay_events(&params.session_id)
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let session = match to_api_session_summary(session_snapshot) {
            Ok(session) => session,
            Err(err) => {
                self.send_internal_error(request_id, err).await;
                return;
            }
        };

        let watch_id = self.next_watch_id.fetch_add(1, Ordering::Relaxed);
        let cancellation = CancellationToken::new();
        self.replace_watch(
            request_id.connection_id,
            params.session_id.clone(),
            watch_id,
            cancellation.clone(),
        )
        .await;

        self.outgoing
            .send_response(
                request_id.clone(),
                Git4DSessionWatchResponse {
                    session,
                    replay_mode: params.replay_mode,
                },
            )
            .await;

        let session_id = params.session_id.clone();
        let connection_id = request_id.connection_id;
        let outgoing = Arc::clone(&self.outgoing);
        let watchers = Arc::clone(&self.watchers);
        self.background_tasks.spawn(async move {
            let replay_notifications = replay_events;
            for event in replay_notifications {
                if cancellation.is_cancelled() {
                    break;
                }
                send_event_notification(&outgoing, connection_id, &session_id, event).await;
            }

            let mut receiver = receiver;
            loop {
                tokio::select! {
                    _ = cancellation.cancelled() => break,
                    received = receiver.recv() => {
                        match received {
                            Ok(sequenced_event) => {
                                send_event_notification(
                                    &outgoing,
                                    connection_id,
                                    &session_id,
                                    sequenced_event,
                                )
                                .await;
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                                warn!(
                                    session_id = session_id.as_str(),
                                    connection_id = connection_id.0,
                                    skipped,
                                    "Git4D watch lagged"
                                );
                            }
                            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                        }
                    }
                }
            }

            let mut watchers = watchers.lock().await;
            let key = (connection_id, session_id.clone());
            if watchers
                .get(&key)
                .is_some_and(|registration| registration.watch_id == watch_id)
            {
                watchers.remove(&key);
            }
        });
    }

    pub(crate) async fn session_unwatch(
        &self,
        request_id: ConnectionRequestId,
        params: Git4DSessionUnwatchParams,
    ) {
        let unsubscribed = self
            .remove_watch(request_id.connection_id, &params.session_id)
            .await;
        self.outgoing
            .send_response(
                request_id,
                Git4DSessionUnwatchResponse {
                    session_id: params.session_id,
                    unsubscribed,
                },
            )
            .await;
    }

    async fn replace_watch(
        &self,
        connection_id: ConnectionId,
        session_id: String,
        watch_id: u64,
        cancellation: CancellationToken,
    ) {
        let previous = {
            let mut watchers = self.watchers.lock().await;
            watchers.insert(
                (connection_id, session_id),
                WatchRegistration {
                    watch_id,
                    cancellation: cancellation.clone(),
                },
            )
        };
        if let Some(previous) = previous {
            previous.cancellation.cancel();
        }
    }

    async fn remove_watch(&self, connection_id: ConnectionId, session_id: &str) -> bool {
        let removed = {
            let mut watchers = self.watchers.lock().await;
            watchers.remove(&(connection_id, session_id.to_string()))
        };
        if let Some(registration) = removed {
            registration.cancellation.cancel();
            true
        } else {
            false
        }
    }

    async fn cancel_all_watches(&self) {
        let cancellations = {
            let mut watchers = self.watchers.lock().await;
            watchers
                .drain()
                .map(|(_, registration)| registration.cancellation)
                .collect::<Vec<_>>()
        };
        for cancellation in cancellations {
            cancellation.cancel();
        }
    }

    async fn send_invalid_request_error(&self, request_id: ConnectionRequestId, message: String) {
        self.outgoing
            .send_error(
                request_id,
                JSONRPCErrorError {
                    code: INVALID_REQUEST_ERROR_CODE,
                    message,
                    data: None,
                },
            )
            .await;
    }

    async fn send_internal_error(&self, request_id: ConnectionRequestId, message: String) {
        self.outgoing
            .send_error(
                request_id,
                JSONRPCErrorError {
                    code: INTERNAL_ERROR_CODE,
                    message,
                    data: None,
                },
            )
            .await;
    }
}

fn to_core_mode(mode: ApiGit4DMode) -> CoreGit4DMode {
    match mode {
        ApiGit4DMode::Desktop => CoreGit4DMode::Desktop,
        ApiGit4DMode::Vr => CoreGit4DMode::Vr,
        ApiGit4DMode::Ar => CoreGit4DMode::Ar,
    }
}

fn to_api_mode(mode: CoreGit4DMode) -> ApiGit4DMode {
    match mode {
        CoreGit4DMode::Desktop => ApiGit4DMode::Desktop,
        CoreGit4DMode::Vr => ApiGit4DMode::Vr,
        CoreGit4DMode::Ar => ApiGit4DMode::Ar,
    }
}

fn to_api_status(status: CoreSessionStatus) -> ApiGit4DSessionStatus {
    match status {
        CoreSessionStatus::Starting => ApiGit4DSessionStatus::Starting,
        CoreSessionStatus::Active => ApiGit4DSessionStatus::Active,
        CoreSessionStatus::Paused => ApiGit4DSessionStatus::Paused,
        CoreSessionStatus::Stopping => ApiGit4DSessionStatus::Stopping,
        CoreSessionStatus::Stopped => ApiGit4DSessionStatus::Stopped,
        CoreSessionStatus::Error => ApiGit4DSessionStatus::Error,
    }
}

fn to_api_capabilities(snapshot: Git4DCapabilitySnapshot) -> Git4DCapabilitiesResponse {
    Git4DCapabilitiesResponse {
        requested_mode: to_api_mode(snapshot.requested_mode),
        effective_mode: to_api_mode(snapshot.effective_mode),
        native_supported: snapshot.native_supported,
        platform: snapshot.platform,
        device_name: snapshot.device_name,
        fallback_reason: snapshot.fallback_reason,
    }
}

fn to_api_session_summary(snapshot: Git4DSessionSnapshot) -> Result<Git4DSessionSummary, String> {
    let repository_path = AbsolutePathBuf::try_from(snapshot.repository_path)
        .map_err(|err| format!("Git4D session path was not absolute: {err}"))?;
    Ok(Git4DSessionSummary {
        session_id: snapshot.session_id,
        repository_path,
        requested_mode: to_api_mode(snapshot.requested_mode),
        effective_mode: to_api_mode(snapshot.effective_mode),
        status: to_api_status(snapshot.status),
        platform: snapshot.platform,
        device_name: snapshot.device_name,
        fallback_reason: snapshot.fallback_reason,
        uptime_ms: snapshot.uptime_ms,
        idle_ms: snapshot.idle_ms,
    })
}

fn to_api_event(event: CoreGit4DEvent) -> ApiGit4DSessionEvent {
    match event {
        CoreGit4DEvent::CommitsLoaded { commits } => ApiGit4DSessionEvent::CommitsLoaded {
            commit_count: commits.len(),
        },
        CoreGit4DEvent::BranchesUpdated { branches } => {
            let mut branch_names = branches.keys().cloned().collect::<Vec<_>>();
            branch_names.sort();
            ApiGit4DSessionEvent::BranchesUpdated {
                branch_count: branch_names.len(),
                branch_names,
            }
        }
        CoreGit4DEvent::CameraUpdated { position, target } => {
            ApiGit4DSessionEvent::CameraUpdated { position, target }
        }
        CoreGit4DEvent::RenderComplete { pixel_data } => ApiGit4DSessionEvent::RenderComplete {
            pixel_bytes: pixel_data.len(),
        },
        CoreGit4DEvent::InteractionProcessed { interaction } => {
            ApiGit4DSessionEvent::InteractionProcessed { interaction }
        }
        CoreGit4DEvent::Error { message } => ApiGit4DSessionEvent::Error { message },
        CoreGit4DEvent::SessionStatusChanged { status } => {
            let status = match status.as_str() {
                "starting" => ApiGit4DSessionStatus::Starting,
                "active" => ApiGit4DSessionStatus::Active,
                "paused" => ApiGit4DSessionStatus::Paused,
                "stopping" => ApiGit4DSessionStatus::Stopping,
                "stopped" => ApiGit4DSessionStatus::Stopped,
                _ => ApiGit4DSessionStatus::Error,
            };
            ApiGit4DSessionEvent::SessionStatusChanged { status }
        }
    }
}

async fn send_event_notification(
    outgoing: &Arc<OutgoingMessageSender>,
    connection_id: ConnectionId,
    session_id: &str,
    event: Git4DSequencedEvent,
) {
    outgoing
        .send_server_notification_to_connections(
            &[connection_id],
            ServerNotification::Git4DSessionEvent(Git4DSessionEventNotification {
                session_id: session_id.to_string(),
                sequence: event.sequence,
                event: to_api_event(event.event),
            }),
        )
        .await;
}
