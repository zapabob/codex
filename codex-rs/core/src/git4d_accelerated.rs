#[cfg(all(feature = "custom-features", feature = "cuda"))]
use crate::cuda_accelerator::CudaGit4DAccelerator;
#[cfg(feature = "custom-features")]
use crate::cuda_accelerator::{GitCommitVertex, RenderParameters, TransformationMatrix};
use crate::vr_ar_integration::{VRARIntegration, VREvent, VRInteraction, XRPlatform};
use anyhow::Context;
use git2::{Commit, Oid, Repository};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use tokio::sync::{broadcast, mpsc};
use tokio::time::{self, Duration};

/// Detect VirtualDesktop connection
///
/// Checks for VirtualDesktop streamer process and environment variables
fn detect_virtual_desktop() -> bool {
    // 1. 環境変数チェック
    if std::env::var("VIRTUAL_DESKTOP_STREAMER").is_ok() {
        tracing::debug!("VirtualDesktop detected via environment variable");
        return true;
    }

    // 2. プロセスチェック（Windows）
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

    // 3. レジストリチェック（Windows、オプション）
    #[cfg(windows)]
    {
        // VirtualDesktopのインストールパスをチェック
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

/// Accelerated Git4D visualization with CUDA and VR/AR support
pub struct Git4DAcceleratedVisualizer {
    #[cfg(all(feature = "custom-features", feature = "cuda"))]
    cuda_accelerator: Option<CudaGit4DAccelerator>,
    vr_ar_integration: Option<VRARIntegration>,
    pub(crate) repository: Repository,
    #[cfg(feature = "custom-features")]
    commit_cache: Mutex<HashMap<Oid, GitCommitVertex>>,
    branch_cache: Mutex<HashMap<String, Vec<Oid>>>,
    time_range: Mutex<(f32, f32)>,
    visible_branches: Mutex<HashSet<u32>>,
    event_sender: broadcast::Sender<Git4DEvent>,
    interaction_receiver: mpsc::Receiver<VRInteraction>,
    interaction_sender: mpsc::Sender<VRInteraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Git4D Visualization Session
///
/// Represents an active visualization session started from GUI
#[derive(Debug, Clone)]
pub struct Git4DVisualizationSession {
    pub session_id: String,
    pub repository_path: PathBuf,
    pub mode: String,
    pub config: Git4DVisualizationConfig,
    pub created_at: Instant,
    pub status: SessionStatus,
    pub last_activity: Instant,
    pub event_sender: Option<Arc<broadcast::Sender<Git4DEvent>>>,
}

/// Global session storage
static SESSIONS: Lazy<Arc<RwLock<HashMap<String, Git4DVisualizationSession>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

/// Check VR/AR device availability for Git4D visualization
///
/// Returns device availability status based on the requested mode
/// VirtualDesktop is checked first for VR mode, then falls back to WebXR
pub async fn check_vr_ar_device_availability(
    mode: &str,
) -> Result<DeviceAvailability, Box<dyn std::error::Error + Send + Sync>> {
    if mode == "desktop" {
        return Ok(DeviceAvailability::Desktop);
    }

    // VRモード時はVirtualDesktopを優先的に検出
    if mode == "vr" {
        if detect_virtual_desktop() {
            tracing::info!("VirtualDesktop detected for VR mode");
            // VirtualDesktopプラットフォームとして初期化を試みる
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
                            // VirtualDesktop初期化に失敗した場合はWebXRにフォールバック
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
    }

    // Try to initialize VR/AR integration to check device availability
    match VRARIntegration::new() {
        Ok(mut vr_integration) => {
            // Determine platform based on mode
            let platform = if mode == "vr" {
                // VirtualDesktopが検出されなかった場合、WebXRを試す
                XRPlatform::WebXR
            } else if mode == "ar" {
                // Try WebXR for AR (browser-based AR)
                XRPlatform::WebXR
            } else {
                return Ok(DeviceAvailability::Desktop);
            };

            // Try to initialize the platform
            match vr_integration.initialize_platform(platform.clone()).await {
                Ok(_) => {
                    tracing::info!("VR/AR device available: {:?}", platform);
                    let platform_clone = platform.clone();
                    Ok(DeviceAvailability::Available {
                        platform,
                        device_name: Some(format!("{:?}", platform_clone)),
                    })
                }
                Err(e) => {
                    tracing::warn!("VR/AR device not available: {}", e);
                    Ok(DeviceAvailability::NotAvailable {
                        reason: format!("Failed to initialize {:?}: {}", platform, e),
                    })
                }
            }
        }
        Err(e) => {
            tracing::warn!("VR/AR integration not available: {}", e);
            Ok(DeviceAvailability::NotAvailable {
                reason: format!("VR/AR integration failed: {}", e),
            })
        }
    }
}

impl Git4DAcceleratedVisualizer {
    pub fn new(
        repo_path: &Path,
        config: Git4DVisualizationConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let repository = Repository::open(repo_path).with_context(|| {
            format!("Failed to open git repository at: {}", repo_path.display())
        })?;

        // Initialize CUDA accelerator if enabled
        #[cfg(all(feature = "custom-features", feature = "cuda"))]
        let cuda_accelerator = if config.enable_cuda {
            match CudaGit4DAccelerator::new() {
                Ok(acc) => Some(acc),
                Err(e) => {
                    eprintln!("CUDA not available, falling back to CPU rendering: {}", e);
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
                    eprintln!("VR/AR not available: {}", e);
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

    /// Launch Git4D visualization for GUI
    ///
    /// Creates a new visualization session from GUI request
    pub async fn launch_for_gui(
        repository_path: PathBuf,
        mode: String,
    ) -> Result<Git4DVisualizationSession, Box<dyn std::error::Error + Send + Sync>> {
        use uuid::Uuid;

        let session_id = Uuid::new_v4().to_string();

        // Determine visualization mode settings
        let enable_vr_ar = mode == "vr" || mode == "ar";
        let vr_platform = if enable_vr_ar {
            Some(XRPlatform::WebXR) // Default to WebXR for browser-based VR/AR
        } else {
            None
        };

        let config = Git4DVisualizationConfig {
            enable_cuda: true, // Try to enable CUDA if available
            enable_vr_ar,
            vr_platform,
            max_commits: 10000,
            time_compression: 1.0,
            branch_spread: 2.0,
            render_width: 1920,
            render_height: 1080,
        };

        // Verify repository exists
        if !repository_path.exists() {
            return Err(format!(
                "Repository path does not exist: {}",
                repository_path.display()
            )
            .into());
        }

        // Check device availability if VR/AR mode
        let device_availability = if mode == "vr" || mode == "ar" {
            check_vr_ar_device_availability(&mode).await?
        } else {
            DeviceAvailability::Desktop
        };

        // If device is not available, fall back to desktop mode
        let (effective_mode, device_availability_for_log) = match &device_availability {
            DeviceAvailability::Available { .. } => {
                (mode.clone(), format!("{:?}", device_availability))
            }
            DeviceAvailability::NotAvailable { reason } => {
                tracing::warn!(
                    "VR/AR device not available ({}), falling back to desktop mode",
                    reason
                );
                ("desktop".to_string(), format!("NotAvailable: {}", reason))
            }
            DeviceAvailability::Desktop => ("desktop".to_string(), "Desktop".to_string()),
        };

        // Update config if mode changed
        let mut effective_config = config.clone();
        if effective_mode != mode {
            effective_config.enable_vr_ar = false;
            effective_config.vr_platform = None;
        }

        // Create visualizer to verify it can be initialized
        let _visualizer = Self::new(&repository_path, effective_config.clone())?;

        // Create event sender for this session
        let (event_sender, _) = broadcast::channel(100);
        let event_sender_arc = Arc::new(event_sender);

        let session = Git4DVisualizationSession {
            session_id: session_id.clone(),
            repository_path: repository_path.clone(),
            mode: effective_mode.clone(),
            config: effective_config,
            created_at: Instant::now(),
            status: SessionStatus::Starting,
            last_activity: Instant::now(),
            event_sender: Some(event_sender_arc),
        };

        // Store session in global storage
        {
            let mut sessions = SESSIONS.write().map_err(|e| {
                anyhow::anyhow!("Failed to acquire write lock for session storage: {}", e)
            })?;
            sessions.insert(session_id.clone(), session.clone());
        }

        tracing::info!(
            "Created Git4D visualization session: id={}, mode={}, path={:?}, device={}",
            session_id,
            effective_mode,
            repository_path,
            device_availability_for_log
        );

        Ok(session)
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

    /// Update session status
    pub fn update_session_status(session_id: &str, status: SessionStatus) -> Result<(), String> {
        let mut sessions = SESSIONS
            .write()
            .map_err(|e| format!("Failed to acquire write lock for session storage: {}", e))?;
        if let Some(session) = sessions.get_mut(session_id) {
            session.status = status;
            session.last_activity = Instant::now();
            Ok(())
        } else {
            Err(format!("Session not found: {}", session_id))
        }
    }

    /// Get session event receiver
    pub fn get_session_event_receiver(session_id: &str) -> Option<broadcast::Receiver<Git4DEvent>> {
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
    pub async fn load_commits(
        &self,
        config: &Git4DVisualizationConfig,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let start_time = Instant::now();
        tracing::debug!(
            "Starting to load commits (max_commits: {})",
            config.max_commits
        );

        let mut commit_cache = self
            .commit_cache
            .lock()
            .map_err(|e| format!("Failed to lock commit_cache: {}", e))?;
        let mut branch_cache = self
            .branch_cache
            .lock()
            .map_err(|e| format!("Failed to lock branch_cache: {}", e))?;

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
        let total_commits: usize = branch_cache.values().map(|ids| ids.len()).sum();

        for (branch_name, commit_ids) in branch_cache.iter() {
            let branch_id = *branch_ids.get(branch_name).unwrap_or(&0);

            for (index, &commit_id) in commit_ids.iter().enumerate() {
                if let Ok(commit) = self
                    .repository
                    .find_commit(commit_id)
                    .with_context(|| format!("Failed to find commit: {}", commit_id))
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
                        branch_id: branch_id as u32,
                        commit_hash: commit_id.as_bytes()[0..8]
                            .iter()
                            .fold(0u64, |acc, &b| (acc << 8) | b as u64),
                    };

                    commit_cache.insert(commit_id, vertex.clone());
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
            .map_err(|e| format!("Failed to lock time_range: {}", e))? = time_range;

        // Send loaded event
        let commits_count = all_commits.len();
        let _ = self.event_sender.send(Git4DEvent::CommitsLoaded {
            commits: all_commits,
        });

        // Convert Oid to String for serialization
        let branches_string: HashMap<String, Vec<String>> = branch_cache
            .iter()
            .map(|(name, oids)| {
                let oid_strings: Vec<String> = oids.iter().map(|oid| oid.to_string()).collect();
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
    pub async fn render(
        &self,
        config: &Git4DVisualizationConfig,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let render_start = Instant::now();
        tracing::debug!(
            "Starting render: {}x{}",
            config.render_width,
            config.render_height
        );

        let commit_cache = self
            .commit_cache
            .lock()
            .map_err(|e| format!("Failed to lock commit_cache: {}", e))?;
        let time_range = *self
            .time_range
            .lock()
            .map_err(|e| format!("Failed to lock time_range: {}", e))?;
        let visible_branches = self
            .visible_branches
            .lock()
            .map_err(|e| format!("Failed to lock visible_branches: {}", e))?;

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
        let params = RenderParameters {
            viewport_width: config.render_width,
            viewport_height: config.render_height,
            camera_position: camera_pos,
            camera_target,
            camera_up: [0.0, 1.0, 0.0],
            projection_matrix: self.create_projection_matrix(config),
            time_filter: time_range,
            branch_filter: visible_branches.iter().cloned().collect(),
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
    pub async fn process_vr_interactions(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                            vr_ar.update().await.map_err(|e| e.to_string().into())?
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
    async fn handle_vr_event(
        &mut self,
        event: VREvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                        .map_err(|e| e.to_string().into())?
                    {
                        let _ = self.interaction_sender.send(interaction.clone()).await;
                        let _ = self.event_sender.send(Git4DEvent::InteractionProcessed {
                            interaction: format!("{:?}", interaction),
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
    async fn handle_interaction(
        &mut self,
        interaction: VRInteraction,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                    .map_err(|e| format!("Failed to lock visible_branches: {}", e))?;
                if visible_branches.is_empty() {
                    // Show all branches
                    let branch_cache = self
                        .branch_cache
                        .lock()
                        .map_err(|e| format!("Failed to lock branch_cache: {}", e))?;
                    for (_branch_name, commits) in branch_cache.iter() {
                        if let Some(first_commit) = commits.first() {
                            let commit_cache = self
                                .commit_cache
                                .lock()
                                .map_err(|e| format!("Failed to lock commit_cache: {}", e))?;
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
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.handle_interaction(interaction).await
    }

    /// Traverse commit graph to collect commits
    fn traverse_commits(
        &self,
        start_commit: &Commit,
        commits: &mut Vec<Oid>,
        max_commits: usize,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                .with_context(|| format!("Failed to find commit during traversal: {}", commit_id))
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
                let mut transformed = vertex.clone();

                // Apply time filtering
                if vertex.time < params.time_filter.0 || vertex.time > params.time_filter.1 {
                    transformed.color[3] = 0.0;
                    return transformed;
                }

                // Apply branch filtering
                let branch_visible = params.branch_filter.is_empty()
                    || params.branch_filter.contains(&vertex.branch_id);

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
    use std::path::PathBuf;
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
                &format!("Commit {}", i),
                &tree,
                &parents,
            )
            .expect(&format!("Failed to create commit {}", i));
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
