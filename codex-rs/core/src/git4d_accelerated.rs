#[cfg(all(feature = "custom-features", feature = "cuda"))]
use crate::cuda_accelerator::CudaGit4DAccelerator;
use crate::cuda_accelerator::GitCommitVertex;
use crate::cuda_accelerator::RenderParameters;
use crate::cuda_accelerator::TransformationMatrix;
use crate::vr_ar_integration::VRARIntegration;
use crate::vr_ar_integration::VREvent;
use crate::vr_ar_integration::VRInteraction;
use crate::vr_ar_integration::XRPlatform;
use anyhow::Context;
use git2::Commit;
use git2::Oid;
use git2::Repository;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::time::Instant;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::{self};

const SESSION_EVENT_REPLAY_LIMIT: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Git4DMode {
    Desktop,
    Vr,
    Ar,
}

impl Git4DMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Vr => "vr",
            Self::Ar => "ar",
        }
    }
}

impl FromStr for Git4DMode {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "desktop" => Ok(Self::Desktop),
            "vr" => Ok(Self::Vr),
            "ar" => Ok(Self::Ar),
            other => Err(anyhow::anyhow!(
                "Invalid Git4D mode: {other}. Must be one of: desktop, vr, ar"
            )),
        }
    }
}

/// Detect VirtualDesktop connection
///
/// Checks for VirtualDesktop streamer process and environment variables
fn detect_virtual_desktop() -> bool {
    // 1. 迺ｰ蠅・､画焚繝√ぉ繝・け
    if std::env::var("VIRTUAL_DESKTOP_STREAMER").is_ok() {
        tracing::debug!("VirtualDesktop detected via environment variable");
        return true;
    }

    // 2. 繝励Ο繧ｻ繧ｹ繝√ぉ繝・け・・indows・・    #[cfg(windows)]
    #[cfg(windows)]
    {
        use std::process::Command;
        let output = Command::new("tasklist")
            .arg("/FI")
            .arg("IMAGENAME eq VirtualDesktop.Streamer.exe")
            .output();
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("VirtualDesktop.Streamer.exe") {
                tracing::debug!("VirtualDesktop detected via process check");
                return true;
            }
        }
    }

    // 3. 繝ｬ繧ｸ繧ｹ繝医Μ繝√ぉ繝・け・・indows縲√が繝励す繝ｧ繝ｳ・・    #[cfg(windows)]
    {
        // VirtualDesktop縺ｮ繧､繝ｳ繧ｹ繝医・繝ｫ繝代せ繧偵メ繧ｧ繝・け
        let local_app_data = std::env::var("LOCALAPPDATA");
        if let Ok(local_app_data) = local_app_data {
            let vd_path = PathBuf::from(local_app_data)
                .join("VirtualDesktop.Streamer")
                .join("VirtualDesktop.Streamer.exe");
            if vd_path.exists() {
                tracing::debug!("VirtualDesktop detected via installation path");
                return true;
            }
        }
    }

    false
}

fn openxr_runtime_configured() -> bool {
    std::env::var_os("OPENXR_RUNTIME_JSON").is_some_and(|value| !value.is_empty())
}

/// Accelerated Git4D visualization with CUDA and VR/AR support
pub struct Git4DAcceleratedVisualizer {
    #[cfg(all(feature = "custom-features", feature = "cuda"))]
    cuda_accelerator: Option<CudaGit4DAccelerator>,
    vr_ar_integration: Option<VRARIntegration>,
    pub(crate) repository: Repository,
    commit_cache: Mutex<HashMap<Oid, GitCommitVertex>>,
    branch_cache: Mutex<HashMap<String, Vec<Oid>>>,
    time_range: Mutex<(f32, f32)>,
    visible_branches: Mutex<HashSet<u32>>,
    event_sender: broadcast::Sender<Git4DEvent>,
    interaction_receiver: mpsc::Receiver<VRInteraction>,
    interaction_sender: mpsc::Sender<VRInteraction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Git4DEvent {
    CommitsLoaded {
        commits: Vec<GitCommitVertex>,
    },
    BranchesUpdated {
        branches: HashMap<String, Vec<String>>, // Oid as string
    },
    CameraUpdated {
        position: [f32; 3],
        target: [f32; 3],
    },
    RenderComplete {
        #[serde(skip)]
        pixel_data: Vec<u8>, // Skip serialization for large data
    },
    InteractionProcessed {
        interaction: String, // Serialize VRInteraction as string
    },
    Error {
        message: String,
    },
    SessionStatusChanged {
        status: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Git4DVisualizationConfig {
    pub enable_cuda: bool,
    pub enable_vr_ar: bool,
    pub vr_platform: Option<XRPlatform>,
    pub max_commits: usize,
    pub time_compression: f32,
    pub branch_spread: f32,
    pub render_width: u32,
    pub render_height: u32,
}

/// Device availability status
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceAvailability {
    Available {
        platform: XRPlatform,
        device_name: Option<String>,
    },
    NotAvailable {
        reason: String,
    },
    Desktop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Git4DCapabilitySnapshot {
    pub requested_mode: Git4DMode,
    pub effective_mode: Git4DMode,
    pub native_supported: bool,
    pub platform: Option<String>,
    pub device_name: Option<String>,
    pub fallback_reason: Option<String>,
}

/// Session status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionStatus {
    Starting,
    Active,
    Paused,
    Stopping,
    Stopped,
    Error,
}

impl SessionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Git4DSessionSnapshot {
    pub session_id: String,
    pub repository_path: PathBuf,
    pub requested_mode: Git4DMode,
    pub effective_mode: Git4DMode,
    pub status: SessionStatus,
    pub platform: Option<String>,
    pub device_name: Option<String>,
    pub fallback_reason: Option<String>,
    pub uptime_ms: u64,
    pub idle_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Git4DSequencedEvent {
    pub sequence: u64,
    pub event: Git4DEvent,
}

/// Git4D Visualization Session
///
/// Represents an active visualization session started from GUI
#[derive(Debug, Clone)]
pub struct Git4DVisualizationSession {
    pub session_id: String,
    pub repository_path: PathBuf,
    pub mode: String,
    pub requested_mode: Git4DMode,
    pub effective_mode: Git4DMode,
    pub config: Git4DVisualizationConfig,
    pub created_at: Instant,
    pub status: SessionStatus,
    pub last_activity: Instant,
    pub platform: Option<String>,
    pub device_name: Option<String>,
    pub fallback_reason: Option<String>,
    pub event_sender: Option<Arc<broadcast::Sender<Git4DSequencedEvent>>>,
    next_event_sequence: u64,
    replay_buffer: VecDeque<Git4DSequencedEvent>,
}

/// Global session storage
static SESSIONS: Lazy<Arc<RwLock<HashMap<String, Git4DVisualizationSession>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

struct ResolvedModeDetails {
    snapshot: Git4DCapabilitySnapshot,
    vr_platform: Option<XRPlatform>,
}

fn build_capability_snapshot(
    requested_mode: Git4DMode,
    device_availability: &DeviceAvailability,
) -> ResolvedModeDetails {
    match device_availability {
        DeviceAvailability::Available {
            platform,
            device_name,
        } => ResolvedModeDetails {
            snapshot: Git4DCapabilitySnapshot {
                requested_mode,
                effective_mode: requested_mode,
                native_supported: true,
                platform: Some(format!("{platform:?}")),
                device_name: device_name.clone(),
                fallback_reason: None,
            },
            vr_platform: Some(platform.clone()),
        },
        DeviceAvailability::NotAvailable { reason } => ResolvedModeDetails {
            snapshot: Git4DCapabilitySnapshot {
                requested_mode,
                effective_mode: Git4DMode::Desktop,
                native_supported: false,
                platform: Some("Desktop".to_string()),
                device_name: None,
                fallback_reason: Some(reason.clone()),
            },
            vr_platform: None,
        },
        DeviceAvailability::Desktop => ResolvedModeDetails {
            snapshot: Git4DCapabilitySnapshot {
                requested_mode,
                effective_mode: Git4DMode::Desktop,
                native_supported: true,
                platform: Some("Desktop".to_string()),
                device_name: None,
                fallback_reason: None,
            },
            vr_platform: None,
        },
    }
}

/// Check VR/AR device availability for Git4D visualization
///
/// Returns device availability status based on the requested mode
/// VirtualDesktop is checked first for VR mode, then falls back to WebXR
pub async fn check_vr_ar_device_availability(mode: &str) -> anyhow::Result<DeviceAvailability> {
    if mode == "desktop" {
        return Ok(DeviceAvailability::Desktop);
    }

    if mode == "vr" && detect_virtual_desktop() {
        tracing::info!("VirtualDesktop detected for VR mode");
        match VRARIntegration::new() {
            Ok(mut vr_integration) => {
                match vr_integration
                    .initialize_platform(XRPlatform::VirtualDesktop)
                    .await
                {
                    Ok(_) => {
                        tracing::info!("VirtualDesktop platform initialized successfully");
                        return Ok(DeviceAvailability::Available {
                            platform: XRPlatform::VirtualDesktop,
                            device_name: Some("VirtualDesktop Streamer".to_string()),
                        });
                    }
                    Err(e) => {
                        tracing::warn!(
                            "VirtualDesktop initialization failed: {}, falling back to WebXR",
                            e
                        );
                    }
                }
            }
            Err(e) => {
                tracing::warn!(
                    "VR/AR integration not available: {}, falling back to WebXR",
                    e
                );
            }
        }
    }

    if !openxr_runtime_configured() {
        return Ok(DeviceAvailability::NotAvailable {
            reason: "OPENXR_RUNTIME_JSON is not configured; using desktop Git4D fallback"
                .to_string(),
        });
    }

    match VRARIntegration::new() {
        Ok(mut vr_integration) => {
            let platform = if mode == "vr" || mode == "ar" {
                XRPlatform::WebXR
            } else {
                return Ok(DeviceAvailability::Desktop);
            };

            match vr_integration.initialize_platform(platform.clone()).await {
                Ok(_) => {
                    tracing::info!("VR/AR device available: {:?}", platform);
                    Ok(DeviceAvailability::Available {
                        platform: platform.clone(),
                        device_name: Some(format!("{platform:?}")),
                    })
                }
                Err(e) => {
                    tracing::warn!("VR/AR device not available: {}", e);
                    Ok(DeviceAvailability::NotAvailable {
                        reason: format!("Failed to initialize {platform:?}: {e}"),
                    })
                }
            }
        }
        Err(e) => {
            tracing::warn!("VR/AR integration not available: {}", e);
            Ok(DeviceAvailability::NotAvailable {
                reason: format!("VR/AR integration failed: {e}"),
            })
        }
    }
}

pub async fn read_capabilities(mode: Git4DMode) -> anyhow::Result<Git4DCapabilitySnapshot> {
    let device_availability = if matches!(mode, Git4DMode::Vr | Git4DMode::Ar) {
        check_vr_ar_device_availability(mode.as_str()).await?
    } else {
        DeviceAvailability::Desktop
    };

    Ok(build_capability_snapshot(mode, &device_availability).snapshot)
}

impl Git4DAcceleratedVisualizer {
    pub fn new(repo_path: &Path, config: Git4DVisualizationConfig) -> anyhow::Result<Self> {
        let repository = Repository::open(repo_path).with_context(|| {
            format!("Failed to open git repository at: {}", repo_path.display())
        })?;

        // Initialize CUDA accelerator if enabled
        #[cfg(all(feature = "custom-features", feature = "cuda"))]
        let cuda_accelerator = if config.enable_cuda {
            match CudaGit4DAccelerator::new() {
                Ok(acc) => Some(acc),
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "CUDA is unavailable; falling back to CPU rendering"
                    );
                    None
                }
            }
        } else {
            None
        };

        // Initialize VR/AR integration if enabled
        let vr_ar_integration = if config.enable_vr_ar {
            match VRARIntegration::new() {
                Ok(vr) => Some(vr),
                Err(e) => {
                    tracing::warn!(error = %e, "VR/AR integration is unavailable");
                    None
                }
            }
        } else {
            None
        };

        let (event_sender, _) = broadcast::channel(100);
        let (interaction_sender, interaction_receiver) = mpsc::channel(100);

        #[cfg(all(feature = "custom-features", feature = "cuda"))]
        {
            Ok(Self {
                cuda_accelerator,
                vr_ar_integration,
                repository,
                commit_cache: Mutex::new(HashMap::new()),
                branch_cache: Mutex::new(HashMap::new()),
                time_range: Mutex::new((0.0, 1.0)),
                visible_branches: Mutex::new(HashSet::new()),
                event_sender,
                interaction_receiver,
                interaction_sender,
            })
        }
        #[cfg(not(all(feature = "custom-features", feature = "cuda")))]
        {
            Ok(Self {
                vr_ar_integration,
                repository,
                commit_cache: Mutex::new(HashMap::new()),
                branch_cache: Mutex::new(HashMap::new()),
                time_range: Mutex::new((0.0, 1.0)),
                visible_branches: Mutex::new(HashSet::new()),
                event_sender,
                interaction_receiver,
                interaction_sender,
            })
        }
    }

    fn record_event_locked(
        session: &mut Git4DVisualizationSession,
        event: Git4DEvent,
    ) -> Git4DSequencedEvent {
        let sequenced_event = Git4DSequencedEvent {
            sequence: session.next_event_sequence,
            event,
        };
        session.next_event_sequence += 1;
        session.last_activity = Instant::now();
        session.replay_buffer.push_back(sequenced_event.clone());
        if session.replay_buffer.len() > SESSION_EVENT_REPLAY_LIMIT {
            session.replay_buffer.pop_front();
        }
        if let Some(event_sender) = session.event_sender.as_ref() {
            let _ = event_sender.send(sequenced_event.clone());
        }
        sequenced_event
    }

    fn build_session_snapshot(session: &Git4DVisualizationSession) -> Git4DSessionSnapshot {
        Git4DSessionSnapshot {
            session_id: session.session_id.clone(),
            repository_path: session.repository_path.clone(),
            requested_mode: session.requested_mode,
            effective_mode: session.effective_mode,
            status: session.status,
            platform: session.platform.clone(),
            device_name: session.device_name.clone(),
            fallback_reason: session.fallback_reason.clone(),
            uptime_ms: session.created_at.elapsed().as_millis() as u64,
            idle_ms: session.last_activity.elapsed().as_millis() as u64,
        }
    }

    pub async fn launch_session(
        repository_path: PathBuf,
        requested_mode: Git4DMode,
    ) -> anyhow::Result<Git4DVisualizationSession> {
        use uuid::Uuid;

        if !repository_path.exists() {
            return Err(anyhow::anyhow!(
                "Repository path does not exist: {}",
                repository_path.display()
            ));
        }

        let repository_path = repository_path
            .canonicalize()
            .with_context(|| format!("Failed to canonicalize {}", repository_path.display()))?;
        let device_availability = if matches!(requested_mode, Git4DMode::Vr | Git4DMode::Ar) {
            check_vr_ar_device_availability(requested_mode.as_str()).await?
        } else {
            DeviceAvailability::Desktop
        };
        let resolved_mode = build_capability_snapshot(requested_mode, &device_availability);
        let config = Git4DVisualizationConfig {
            enable_cuda: true,
            enable_vr_ar: !matches!(resolved_mode.snapshot.effective_mode, Git4DMode::Desktop),
            vr_platform: resolved_mode.vr_platform,
            max_commits: 10000,
            time_compression: 1.0,
            branch_spread: 2.0,
            render_width: 1920,
            render_height: 1080,
        };

        let _visualizer = Self::new(&repository_path, config.clone())?;

        let session_id = Uuid::new_v4().to_string();
        let (event_sender, _) = broadcast::channel(100);
        let session = Git4DVisualizationSession {
            session_id: session_id.clone(),
            repository_path: repository_path.clone(),
            mode: resolved_mode.snapshot.effective_mode.as_str().to_string(),
            requested_mode,
            effective_mode: resolved_mode.snapshot.effective_mode,
            config,
            created_at: Instant::now(),
            status: SessionStatus::Starting,
            last_activity: Instant::now(),
            platform: resolved_mode.snapshot.platform.clone(),
            device_name: resolved_mode.snapshot.device_name.clone(),
            fallback_reason: resolved_mode.snapshot.fallback_reason.clone(),
            event_sender: Some(Arc::new(event_sender)),
            next_event_sequence: 1,
            replay_buffer: VecDeque::new(),
        };

        {
            let mut sessions = SESSIONS.write().map_err(|e| {
                anyhow::anyhow!("Failed to acquire write lock for session storage: {e}")
            })?;
            sessions.insert(session_id.clone(), session);
            let session = sessions
                .get_mut(&session_id)
                .ok_or_else(|| anyhow::anyhow!("Git4D session disappeared after insertion"))?;
            Self::record_event_locked(
                session,
                Git4DEvent::SessionStatusChanged {
                    status: SessionStatus::Starting.as_str().to_string(),
                },
            );
        }

        tracing::info!(
            session_id = session_id.as_str(),
            requested_mode = requested_mode.as_str(),
            effective_mode = resolved_mode.snapshot.effective_mode.as_str(),
            repository_path = ?repository_path,
            platform = ?resolved_mode.snapshot.platform,
            fallback_reason = ?resolved_mode.snapshot.fallback_reason,
            "Created Git4D visualization session"
        );

        let session_id_for_activation = session_id.clone();
        tokio::spawn(async move {
            time::sleep(Duration::from_millis(25)).await;
            if let Err(err) = Git4DAcceleratedVisualizer::update_session_status(
                &session_id_for_activation,
                SessionStatus::Active,
            ) {
                tracing::debug!(
                    session_id = session_id_for_activation.as_str(),
                    error = %err,
                    "failed to activate Git4D session"
                );
            }
        });

        Self::get_session(&session_id)
            .ok_or_else(|| anyhow::anyhow!("Session disappeared after creation: {session_id}"))
    }

    /// Launch Git4D visualization for GUI
    ///
    /// Creates a new visualization session from GUI request
    pub async fn launch_for_gui(
        repository_path: PathBuf,
        mode: String,
    ) -> anyhow::Result<Git4DVisualizationSession> {
        let requested_mode = Git4DMode::from_str(&mode)?;
        Self::launch_session(repository_path, requested_mode).await
    }

    /// Get session by ID
    pub fn get_session(session_id: &str) -> Option<Git4DVisualizationSession> {
        let sessions = SESSIONS
            .read()
            .map_err(|e| {
                tracing::error!("Failed to acquire read lock for session storage: {}", e);
                e
            })
            .ok()?;
        sessions.get(session_id).cloned()
    }

    /// List all active sessions
    pub fn list_sessions() -> Vec<Git4DVisualizationSession> {
        let sessions = match SESSIONS.read() {
            Ok(sessions) => sessions,
            Err(e) => {
                tracing::error!("Failed to acquire read lock for session storage: {}", e);
                return Vec::new();
            }
        };
        sessions.values().cloned().collect()
    }

    pub fn get_session_snapshot(session_id: &str) -> Option<Git4DSessionSnapshot> {
        Self::get_session(session_id).map(|session| Self::build_session_snapshot(&session))
    }

    pub fn list_session_snapshots() -> Vec<Git4DSessionSnapshot> {
        let mut snapshots = Self::list_sessions()
            .into_iter()
            .map(|session| Self::build_session_snapshot(&session))
            .collect::<Vec<_>>();
        snapshots.sort_by(|left, right| left.session_id.cmp(&right.session_id));
        snapshots
    }

    pub fn get_session_replay_events(session_id: &str) -> Option<Vec<Git4DSequencedEvent>> {
        Self::get_session(session_id).map(|session| session.replay_buffer.into_iter().collect())
    }

    pub fn record_session_event(
        session_id: &str,
        event: Git4DEvent,
    ) -> anyhow::Result<Git4DSequencedEvent> {
        let mut sessions = SESSIONS.write().map_err(|e| {
            anyhow::anyhow!("Failed to acquire write lock for session storage: {e}")
        })?;
        if let Some(session) = sessions.get_mut(session_id) {
            Ok(Self::record_event_locked(session, event))
        } else {
            Err(anyhow::anyhow!("Session not found: {session_id}"))
        }
    }

    /// Update session status
    pub fn update_session_status(session_id: &str, status: SessionStatus) -> anyhow::Result<()> {
        let mut sessions = SESSIONS.write().map_err(|e| {
            anyhow::anyhow!("Failed to acquire write lock for session storage: {e}")
        })?;
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = status;
            Self::record_event_locked(
                session,
                Git4DEvent::SessionStatusChanged {
                    status: status.as_str().to_string(),
                },
            );
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found: {session_id}"))
        }
    }

    /// Get session event receiver
    pub fn get_session_event_receiver(
        session_id: &str,
    ) -> Option<broadcast::Receiver<Git4DSequencedEvent>> {
        let sessions = SESSIONS
            .read()
            .map_err(|e| {
                tracing::error!("Failed to acquire read lock for session storage: {}", e);
                e
            })
            .ok()?;
        sessions
            .get(session_id)
            .and_then(|session| session.event_sender.as_ref())
            .map(|sender| sender.subscribe())
    }

    /// Remove session
    pub fn remove_session(session_id: &str) -> bool {
        let mut sessions = match SESSIONS.write() {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("Failed to acquire write lock for session storage: {}", e);
                return false;
            }
        };
        sessions.remove(session_id).is_some()
    }

    /// Load and process Git commits for 4D visualization
    pub async fn load_commits(&self, config: &Git4DVisualizationConfig) -> anyhow::Result<()> {
        let start_time = Instant::now();
        tracing::debug!(
            "Starting to load commits (max_commits: {})",
            config.max_commits
        );

        let mut commit_cache = self
            .commit_cache
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock commit_cache: {e}"))?;
        let mut branch_cache = self
            .branch_cache
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock branch_cache: {e}"))?;

        // Clear existing data
        commit_cache.clear();
        branch_cache.clear();

        // Load branches
        let branches = self
            .repository
            .branches(None)
            .context("Failed to get repository branches")?;
        let mut branch_id_counter = 0;
        let mut branch_ids = HashMap::new();

        let branch_load_start = Instant::now();
        for branch_result in branches {
            let (branch, _) = branch_result?;
            if let Ok(Some(name)) = branch.name() {
                let reference = branch.get();
                if let Ok(commit) = reference.peel_to_commit() {
                    let mut commits: Vec<Oid> = Vec::new();
                    self.traverse_commits(&commit, &mut commits, config.max_commits)?;

                    branch_cache.insert(name.to_string(), commits.clone());

                    // Assign branch ID
                    branch_ids.insert(name.to_string(), branch_id_counter);
                    branch_id_counter += 1;
                }
            }
        }
        tracing::debug!(
            "Loaded {} branches in {:?}",
            branch_ids.len(),
            branch_load_start.elapsed()
        );

        tracing::debug!(
            "Loaded {} branches in {:?}",
            branch_ids.len(),
            start_time.elapsed()
        );

        // Process commits into 4D vertices
        let vertex_process_start = Instant::now();
        let mut all_commits = Vec::new();
        let mut time_min = f64::INFINITY;
        let mut time_max = f64::NEG_INFINITY;
        let total_commits: usize = branch_cache.values().map(std::vec::Vec::len).sum();

        for (branch_name, commit_ids) in branch_cache.iter() {
            let branch_id = *branch_ids.get(branch_name).unwrap_or(&0);

            for (index, &commit_id) in commit_ids.iter().enumerate() {
                if let Ok(commit) = self
                    .repository
                    .find_commit(commit_id)
                    .with_context(|| format!("Failed to find commit: {commit_id}"))
                {
                    let time = commit.time().seconds() as f64;
                    time_min = time_min.min(time);
                    time_max = time_max.max(time);

                    // Calculate 3D position based on branch and time
                    let branch_offset = (branch_id as f32 - (branch_ids.len() as f32 - 1.0) / 2.0)
                        * config.branch_spread;
                    let time_pos = (index as f32) * config.time_compression;

                    let vertex = GitCommitVertex {
                        position: [branch_offset, time_pos, 0.0],
                        time: time as f32,
                        color: self.get_commit_color(&commit, branch_id),
                        branch_id,
                        commit_hash: commit_id.as_bytes()[0..8]
                            .iter()
                            .fold(0u64, |acc, &b| (acc << 8) | b as u64),
                    };

                    commit_cache.insert(commit_id, vertex);
                    all_commits.push(vertex);
                }
            }
        }
        tracing::debug!(
            "Processed {} commits into vertices in {:?}",
            total_commits,
            vertex_process_start.elapsed()
        );

        // Update time range
        let time_range = ((time_min as f32).max(0.0), time_max as f32);
        *self
            .time_range
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock time_range: {e}"))? = time_range;

        // Send loaded event
        let commits_count = all_commits.len();
        let _ = self.event_sender.send(Git4DEvent::CommitsLoaded {
            commits: all_commits,
        });

        // Convert Oid to String for serialization
        let branches_string: HashMap<String, Vec<String>> = branch_cache
            .iter()
            .map(|(name, oids)| {
                let oid_strings: Vec<String> =
                    oids.iter().map(std::string::ToString::to_string).collect();
                (name.clone(), oid_strings)
            })
            .collect();
        let _ = self.event_sender.send(Git4DEvent::BranchesUpdated {
            branches: branches_string,
        });

        tracing::info!(
            "Completed loading commits: {} commits, {} branches, total time: {:?}",
            commits_count,
            branch_ids.len(),
            start_time.elapsed()
        );

        Ok(())
    }

    /// Render Git4D visualization
    pub async fn render(&self, config: &Git4DVisualizationConfig) -> anyhow::Result<Vec<u8>> {
        let render_start = Instant::now();
        tracing::debug!(
            "Starting render: {}x{}",
            config.render_width,
            config.render_height
        );

        let commit_cache = self
            .commit_cache
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock commit_cache: {e}"))?;
        let time_range = *self
            .time_range
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock time_range: {e}"))?;
        let visible_branches = self
            .visible_branches
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to lock visible_branches: {e}"))?;

        // Filter visible commits
        let mut vertices: Vec<GitCommitVertex> = commit_cache
            .values()
            .filter(|v| visible_branches.is_empty() || visible_branches.contains(&v.branch_id))
            .cloned()
            .collect();

        if vertices.is_empty() {
            // Return empty framebuffer
            return Ok(vec![
                0;
                (config.render_width * config.render_height * 4)
                    as usize
            ]);
        }

        // Calculate optimal camera position
        let (camera_pos, camera_target) = {
            #[cfg(all(feature = "custom-features", feature = "cuda"))]
            {
                if let Some(cuda) = &self.cuda_accelerator {
                    cuda.calculate_optimal_camera(&vertices)?
                } else {
                    self.calculate_optimal_camera_cpu(&vertices)
                }
            }
            #[cfg(not(all(feature = "custom-features", feature = "cuda")))]
            {
                self.calculate_optimal_camera_cpu(&vertices)
            }
        };

        // Create transformation matrix (identity for now)
        let transform = TransformationMatrix {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        // Create render parameters
        let (branch_filter, branch_filter_count) = {
            let mut filter = [0u32; 32];
            let mut count = 0u32;

            if !visible_branches.is_empty() {
                for (idx, branch_id) in visible_branches.iter().take(32).enumerate() {
                    filter[idx] = *branch_id;
                    count += 1;
                }
            }

            (filter, count)
        };

        let params = RenderParameters {
            viewport_width: config.render_width,
            viewport_height: config.render_height,
            camera_position: camera_pos,
            camera_target,
            camera_up: [0.0, 1.0, 0.0],
            projection_matrix: self.create_projection_matrix(config),
            time_filter: time_range,
            branch_filter,
            branch_filter_count,
        };

        // Send camera update event
        let _ = self.event_sender.send(Git4DEvent::CameraUpdated {
            position: camera_pos,
            target: camera_target,
        });

        // Transform vertices
        {
            #[cfg(all(feature = "custom-features", feature = "cuda"))]
            {
                if let Some(cuda) = &self.cuda_accelerator {
                    vertices = cuda.transform_vertices(&vertices, &transform, &params)?;
                } else {
                    vertices = self.transform_vertices_cpu(&vertices, &transform, &params);
                }
            }
            #[cfg(not(all(feature = "custom-features", feature = "cuda")))]
            {
                vertices = self.transform_vertices_cpu(&vertices, &transform, &params);
            }
        }

        // Render to framebuffer
        let mut framebuffer = vec![0u32; (config.render_width * config.render_height) as usize];

        {
            #[cfg(all(feature = "custom-features", feature = "cuda"))]
            {
                if let Some(cuda) = &self.cuda_accelerator {
                    cuda.render_to_framebuffer(&vertices, &mut framebuffer, &params)?;
                } else {
                    self.render_to_framebuffer_cpu(&vertices, &mut framebuffer, &params);
                }
            }
            #[cfg(not(all(feature = "custom-features", feature = "cuda")))]
            {
                self.render_to_framebuffer_cpu(&vertices, &mut framebuffer, &params);
            }
        }

        // Convert to RGBA bytes
        let rgba_data: Vec<u8> = framebuffer
            .into_iter()
            .flat_map(|pixel| {
                let r = ((pixel >> 16) & 0xFF) as u8;
                let g = ((pixel >> 8) & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;
                let a = ((pixel >> 24) & 0xFF) as u8;
                [r, g, b, a]
            })
            .collect();

        // Send render complete event
        let _ = self.event_sender.send(Git4DEvent::RenderComplete {
            pixel_data: rgba_data.clone(),
        });

        tracing::debug!(
            "Render completed: {} bytes, {} vertices, time: {:?}",
            rgba_data.len(),
            vertices.len(),
            render_start.elapsed()
        );

        Ok(rgba_data)
    }

    /// Process VR/AR interactions
    pub async fn process_vr_interactions(&mut self) -> anyhow::Result<()> {
        tracing::debug!("Starting VR/AR interaction processing");
        if self.vr_ar_integration.is_none() {
            return Ok(());
        }

        // Subscribe to VR events (borrow temporarily)
        let mut event_receiver = {
            if let Some(vr_ar) = &mut self.vr_ar_integration {
                vr_ar.subscribe_events()
            } else {
                return Ok(());
            }
        };
        let mut pending_events = Vec::new();

        loop {
            tokio::select! {
                // Process VR events
                event = event_receiver.recv() => {
                    if let Ok(event) = event {
                        pending_events.push(event);
                    }
                }

                // Process interactions
                interaction = self.interaction_receiver.recv() => {
                    if let Some(interaction) = interaction {
                        // Process pending events first
                        for event in pending_events.drain(..) {
                            self.handle_vr_event(event).await?;
                        }
                        self.handle_interaction(interaction).await?;
                    } else {
                        // Process any remaining events before breaking
                        for event in pending_events.drain(..) {
                            self.handle_vr_event(event).await?;
                        }
                        break;
                    }
                }

                // Periodic update
                _ = time::sleep(Duration::from_millis(16)) => {
                    // Borrow vr_ar temporarily to get events
                    let events = {
                        if let Some(vr_ar) = &mut self.vr_ar_integration {
                            vr_ar.update().await.map_err(|e| std::io::Error::other(e.to_string()))?
                        } else {
                            Vec::new()
                        }
                    };
                    // Process pending events first
                    for event in pending_events.drain(..) {
                        self.handle_vr_event(event).await?;
                    }
                    // Then process new events
                    for event in events {
                        self.handle_vr_event(event).await?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle VR event
    async fn handle_vr_event(&mut self, event: VREvent) -> anyhow::Result<()> {
        tracing::debug!("Handling VR event: {:?}", event);
        match event {
            VREvent::GestureRecognized(gesture, hand) => {
                tracing::debug!("Gesture recognized: {:?} with {:?}", gesture, hand);
                if let Some(vr_ar) = &mut self.vr_ar_integration {
                    // Get hand position (simplified)
                    let position = [0.0, 0.0, 0.0]; // Would get from VR system

                    if let Some(interaction) = vr_ar
                        .handle_gesture_interaction(gesture, hand, position)
                        .await
                        .map_err(|e| std::io::Error::other(e.to_string()))?
                    {
                        let _ = self.interaction_sender.send(interaction.clone()).await;
                        let _ = self.event_sender.send(Git4DEvent::InteractionProcessed {
                            interaction: format!("{interaction:?}"),
                        });
                    }
                }
            }
            VREvent::AnchorCreated(anchor) => {
                // Handle anchor creation for Git commits
                tracing::info!(
                    "Anchor created: {} at {:?} (type: {:?})",
                    anchor.id,
                    anchor.position,
                    anchor.anchor_type
                );
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle VR interaction
    async fn handle_interaction(&mut self, interaction: VRInteraction) -> anyhow::Result<()> {
        tracing::debug!("Handling VR interaction: {:?}", interaction);
        match interaction {
            VRInteraction::SelectAnchor(anchor_id) => {
                tracing::info!("Selected anchor: {}", anchor_id);
                // Could highlight commit, show details, etc.
            }
            VRInteraction::CreateTimeAnchor(anchor_id) => {
                tracing::info!("Created time anchor: {}", anchor_id);
                // Could create time bookmark
            }
            VRInteraction::ToggleBranchVisibility => {
                tracing::debug!("Toggling branch visibility");
                // Toggle all branches or cycle through them
                let mut visible_branches = self
                    .visible_branches
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Failed to lock visible_branches: {e}"))?;
                if visible_branches.is_empty() {
                    // Show all branches
                    let branch_cache = self
                        .branch_cache
                        .lock()
                        .map_err(|e| anyhow::anyhow!("Failed to lock branch_cache: {e}"))?;
                    for (_branch_name, commits) in branch_cache.iter() {
                        if let Some(first_commit) = commits.first() {
                            let commit_cache = self
                                .commit_cache
                                .lock()
                                .map_err(|e| anyhow::anyhow!("Failed to lock commit_cache: {e}"))?;
                            if let Some(vertex) = commit_cache.get(first_commit) {
                                visible_branches.insert(vertex.branch_id);
                            }
                        }
                    }
                } else {
                    visible_branches.clear();
                }
            }
            VRInteraction::ZoomToFit => {
                // Reset camera to show all commits
                tracing::info!("Zoom to fit requested");
            }
            _ => {}
        }

        Ok(())
    }

    /// Process VR interaction (public method for external use)
    pub async fn process_vr_interaction(
        &mut self,
        interaction: VRInteraction,
    ) -> anyhow::Result<()> {
        self.handle_interaction(interaction).await
    }

    /// Traverse commit graph to collect commits
    fn traverse_commits(
        &self,
        start_commit: &Commit,
        commits: &mut Vec<Oid>,
        max_commits: usize,
    ) -> anyhow::Result<()> {
        let traverse_start = Instant::now();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start_commit.id());
        visited.insert(start_commit.id());

        while let Some(commit_id) = queue.pop_front() {
            if commits.len() >= max_commits {
                break;
            }

            commits.push(commit_id);

            if let Ok(commit) = self
                .repository
                .find_commit(commit_id)
                .with_context(|| format!("Failed to find commit during traversal: {commit_id}"))
            {
                for parent in commit.parents() {
                    if !visited.contains(&parent.id()) {
                        visited.insert(parent.id());
                        queue.push_back(parent.id());
                    }
                }
            }
        }

        tracing::debug!(
            "Traversed {} commits in {:?}",
            commits.len(),
            traverse_start.elapsed()
        );

        Ok(())
    }

    /// Get color for commit based on branch and author
    fn get_commit_color(&self, _commit: &Commit, branch_id: u32) -> [f32; 4] {
        // Color based on branch
        let hue = (branch_id as f32 * 137.5) % 360.0; // Golden angle approximation
        let saturation = 0.7;
        let value = 0.9;

        // Convert HSV to RGB
        let c = value * saturation;
        let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let m = value - c;

        let (r, g, b) = match hue {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        [r + m, g + m, b + m, 1.0]
    }

    /// CPU fallback for optimal camera calculation
    fn calculate_optimal_camera_cpu(&self, vertices: &[GitCommitVertex]) -> ([f32; 3], [f32; 3]) {
        if vertices.is_empty() {
            return ([0.0, 0.0, 10.0], [0.0, 0.0, 0.0]);
        }

        let mut min_pos = [f32::INFINITY; 3];
        let mut max_pos = [f32::NEG_INFINITY; 3];

        for vertex in vertices {
            for i in 0..3 {
                min_pos[i] = min_pos[i].min(vertex.position[i]);
                max_pos[i] = max_pos[i].max(vertex.position[i]);
            }
        }

        let center = [
            (min_pos[0] + max_pos[0]) / 2.0,
            (min_pos[1] + max_pos[1]) / 2.0,
            (min_pos[2] + max_pos[2]) / 2.0,
        ];

        let diagonal = [
            max_pos[0] - min_pos[0],
            max_pos[1] - min_pos[1],
            max_pos[2] - min_pos[2],
        ];

        let max_diagonal = diagonal.iter().fold(0.0f32, |a, &b| a.max(b));
        let camera_distance = max_diagonal * 2.0;

        let camera_pos = [
            center[0],
            center[1] - camera_distance,
            center[2] + camera_distance * 0.5,
        ];

        (camera_pos, center)
    }

    /// CPU fallback for vertex transformation
    fn transform_vertices_cpu(
        &self,
        vertices: &[GitCommitVertex],
        transform: &TransformationMatrix,
        params: &RenderParameters,
    ) -> Vec<GitCommitVertex> {
        vertices
            .iter()
            .map(|vertex| {
                let mut transformed = *vertex;

                // Apply time filtering
                if vertex.time < params.time_filter.0 || vertex.time > params.time_filter.1 {
                    transformed.color[3] = 0.0;
                    return transformed;
                }

                // Apply branch filtering
                let branch_visible = params.branch_filter_count == 0
                    || params.branch_filter[..params.branch_filter_count as usize]
                        .contains(&vertex.branch_id);

                if !branch_visible {
                    transformed.color[3] = 0.0;
                    return transformed;
                }

                // Apply transformation matrix
                let pos = [
                    vertex.position[0],
                    vertex.position[1],
                    vertex.position[2],
                    1.0,
                ];

                let transformed_pos = [
                    transform.matrix[0][0] * pos[0]
                        + transform.matrix[0][1] * pos[1]
                        + transform.matrix[0][2] * pos[2]
                        + transform.matrix[0][3] * pos[3],
                    transform.matrix[1][0] * pos[0]
                        + transform.matrix[1][1] * pos[1]
                        + transform.matrix[1][2] * pos[2]
                        + transform.matrix[1][3] * pos[3],
                    transform.matrix[2][0] * pos[0]
                        + transform.matrix[2][1] * pos[1]
                        + transform.matrix[2][2] * pos[2]
                        + transform.matrix[2][3] * pos[3],
                    transform.matrix[3][0] * pos[0]
                        + transform.matrix[3][1] * pos[1]
                        + transform.matrix[3][2] * pos[2]
                        + transform.matrix[3][3] * pos[3],
                ];

                if transformed_pos[3] != 0.0 {
                    transformed.position[0] = transformed_pos[0] / transformed_pos[3];
                    transformed.position[1] = transformed_pos[1] / transformed_pos[3];
                    transformed.position[2] = transformed_pos[2] / transformed_pos[3];
                }

                transformed
            })
            .collect()
    }

    /// CPU fallback for framebuffer rendering
    fn render_to_framebuffer_cpu(
        &self,
        vertices: &[GitCommitVertex],
        framebuffer: &mut [u32],
        params: &RenderParameters,
    ) {
        // Simple CPU-based rendering
        for vertex in vertices {
            if vertex.color[3] <= 0.0 {
                continue;
            }

            // Simple projection (simplified for CPU fallback)
            let screen_x = ((vertex.position[0] + 1.0) * 0.5 * params.viewport_width as f32) as i32;
            let screen_y =
                ((1.0 - (vertex.position[1] + 1.0) * 0.5) * params.viewport_height as f32) as i32;

            if screen_x >= 0
                && screen_x < params.viewport_width as i32
                && screen_y >= 0
                && screen_y < params.viewport_height as i32
            {
                let pixel_index = (screen_y * params.viewport_width as i32 + screen_x) as usize;

                if pixel_index < framebuffer.len() {
                    let r = (vertex.color[0] * 255.0) as u32;
                    let g = (vertex.color[1] * 255.0) as u32;
                    let b = (vertex.color[2] * 255.0) as u32;
                    let a = (vertex.color[3] * 255.0) as u32;

                    framebuffer[pixel_index] = (a << 24) | (r << 16) | (g << 8) | b;
                }
            }
        }
    }

    /// Create perspective projection matrix
    fn create_projection_matrix(&self, config: &Git4DVisualizationConfig) -> [[f32; 4]; 4] {
        let aspect = config.render_width as f32 / config.render_height as f32;
        let fov = std::f32::consts::PI / 4.0; // 45 degrees
        let near = 0.1;
        let far = 1000.0;

        let tan_half_fov = (fov / 2.0).tan();

        [
            [1.0 / (aspect * tan_half_fov), 0.0, 0.0, 0.0],
            [0.0, 1.0 / tan_half_fov, 0.0, 0.0],
            [0.0, 0.0, -(far + near) / (far - near), -1.0],
            [0.0, 0.0, -(2.0 * far * near) / (far - near), 0.0],
        ]
    }

    /// Get event receiver
    pub fn subscribe_events(&self) -> broadcast::Receiver<Git4DEvent> {
        self.event_sender.subscribe()
    }

    /// Get interaction sender
    pub fn get_interaction_sender(&self) -> mpsc::Sender<VRInteraction> {
        self.interaction_sender.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use tempfile::TempDir;

    #[tokio::test]
    async fn test_git4d_visualizer_creation() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let repo_path = temp_dir.path();

        // Initialize a test git repo
        let repo =
            git2::Repository::init(repo_path).expect("Failed to initialize test git repository");

        // Create initial commit
        let sig = git2::Signature::now("Test Author", "test@example.com")
            .expect("Failed to create git signature");
        let tree_id = {
            let mut index = repo.index().expect("Failed to get repository index");
            index.write_tree().expect("Failed to write tree to index")
        };
        let tree = repo.find_tree(tree_id).expect("Failed to find tree");
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .expect("Failed to create initial commit");

        let config = Git4DVisualizationConfig {
            enable_cuda: false,
            enable_vr_ar: false,
            vr_platform: None,
            max_commits: 1000,
            time_compression: 1.0,
            branch_spread: 2.0,
            render_width: 1920,
            render_height: 1080,
        };

        let visualizer = Git4DAcceleratedVisualizer::new(repo_path, config);
        assert!(visualizer.is_ok());
    }

    #[tokio::test]
    async fn test_load_commits() {
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        let repo_path = temp_dir.path();

        // Initialize test repo with some commits
        let repo =
            git2::Repository::init(repo_path).expect("Failed to initialize test git repository");
        let sig = git2::Signature::now("Test Author", "test@example.com")
            .expect("Failed to create git signature");

        // Create a few commits
        for i in 0..3 {
            let tree_id = {
                let mut index = repo.index().expect("Failed to get repository index");
                index.write_tree().expect("Failed to write tree to index")
            };
            let tree = repo.find_tree(tree_id).expect("Failed to find tree");

            let parent = if i > 0 {
                Some(
                    repo.head()
                        .expect("Failed to get HEAD reference")
                        .peel_to_commit()
                        .expect("Failed to peel HEAD to commit"),
                )
            } else {
                None
            };

            let parents = parent.as_ref().map(|p| vec![p]).unwrap_or_default();
            repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                &format!("Commit {i}"),
                &tree,
                &parents,
            )
            .unwrap_or_else(|_| panic!("Failed to create commit {i}"));
        }

        let config = Git4DVisualizationConfig {
            enable_cuda: false,
            enable_vr_ar: false,
            vr_platform: None,
            max_commits: 1000,
            time_compression: 1.0,
            branch_spread: 2.0,
            render_width: 1920,
            render_height: 1080,
        };

        let visualizer = Git4DAcceleratedVisualizer::new(repo_path, config.clone())
            .expect("Failed to create Git4DAcceleratedVisualizer");
        assert!(visualizer.load_commits(&config).await.is_ok());
    }
}
