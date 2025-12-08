use crate::cuda_accelerator::{CudaGit4DAccelerator, GitCommitVertex, TransformationMatrix, RenderParameters};
use crate::vr_ar_integration::{VRARIntegration, VRInteraction, VREvent, XRPlatform, Anchor, AnchorType};
use git2::{Repository, Commit, Oid};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, mpsc};
use tokio::time::{self, Duration};

/// Accelerated Git4D visualization with CUDA and VR/AR support
pub struct Git4DAcceleratedVisualizer {
    cuda_accelerator: Option<CudaGit4DAccelerator>,
    vr_ar_integration: Option<VRARIntegration>,
    repository: Repository,
    commit_cache: Mutex<HashMap<Oid, GitCommitVertex>>,
    branch_cache: Mutex<HashMap<String, Vec<Oid>>>,
    time_range: Mutex<(f32, f32)>,
    visible_branches: Mutex<HashSet<u32>>,
    event_sender: broadcast::Sender<Git4DEvent>,
    interaction_receiver: mpsc::Receiver<VRInteraction>,
    interaction_sender: mpsc::Sender<VRInteraction>,
}

#[derive(Debug, Clone)]
pub enum Git4DEvent {
    CommitsLoaded(Vec<GitCommitVertex>),
    BranchesUpdated(HashMap<String, Vec<Oid>>),
    CameraUpdated([f32; 3], [f32; 3]), // position, target
    RenderComplete(Vec<u8>), // RGBA pixel data
    InteractionProcessed(VRInteraction),
    Error(String),
}

#[derive(Debug, Clone)]
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

impl Git4DAcceleratedVisualizer {
    pub fn new(repo_path: &Path, config: Git4DVisualizationConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let repository = Repository::open(repo_path)?;

        // Initialize CUDA accelerator if enabled
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

    /// Load and process Git commits for 4D visualization
    pub async fn load_commits(&self, config: &Git4DVisualizationConfig) -> Result<(), Box<dyn std::error::Error>> {
        let mut commit_cache = self.commit_cache.lock().unwrap();
        let mut branch_cache = self.branch_cache.lock().unwrap();

        // Clear existing data
        commit_cache.clear();
        branch_cache.clear();

        // Load branches
        let branches = self.repository.branches(None)?;
        let mut branch_id_counter = 0;
        let mut branch_ids = HashMap::new();

        for (branch, _) in branches {
            if let Ok(Some(name)) = branch.name() {
                if let Ok(Some(reference)) = branch.get() {
                    if let Ok(commit) = reference.peel_to_commit() {
                        let mut commits = Vec::new();
                        self.traverse_commits(&commit, &mut commits, config.max_commits)?;

                        branch_cache.insert(name.to_string(), commits.clone());

                        // Assign branch ID
                        branch_ids.insert(name.to_string(), branch_id_counter);
                        branch_id_counter += 1;
                    }
                }
            }
        }

        // Process commits into 4D vertices
        let mut all_commits = Vec::new();
        let mut time_min = f64::INFINITY;
        let mut time_max = f64::NEG_INFINITY;

        for (branch_name, commit_ids) in branch_cache.iter() {
            let branch_id = *branch_ids.get(branch_name).unwrap_or(&0);

            for (index, &commit_id) in commit_ids.iter().enumerate() {
                if let Ok(commit) = self.repository.find_commit(commit_id) {
                    let time = commit.time().seconds() as f64;
                    time_min = time_min.min(time);
                    time_max = time_max.max(time);

                    // Calculate 3D position based on branch and time
                    let branch_offset = (branch_id as f32 - (branch_ids.len() as f32 - 1.0) / 2.0) * config.branch_spread;
                    let time_pos = (index as f32) * config.time_compression;

                    let vertex = GitCommitVertex {
                        position: [branch_offset, time_pos, 0.0],
                        time: time as f32,
                        color: self.get_commit_color(&commit, branch_id),
                        branch_id: branch_id as u32,
                        commit_hash: commit_id.as_bytes()[0..8].iter().fold(0u64, |acc, &b| (acc << 8) | b as u64),
                    };

                    commit_cache.insert(commit_id, vertex.clone());
                    all_commits.push(vertex);
                }
            }
        }

        // Update time range
        let time_range = ((time_min as f32).max(0.0), time_max as f32);
        *self.time_range.lock().unwrap() = time_range;

        // Send loaded event
        let _ = self.event_sender.send(Git4DEvent::CommitsLoaded(all_commits));
        let _ = self.event_sender.send(Git4DEvent::BranchesUpdated(branch_cache.clone()));

        Ok(())
    }

    /// Render Git4D visualization
    pub async fn render(&self, config: &Git4DVisualizationConfig) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let commit_cache = self.commit_cache.lock().unwrap();
        let time_range = *self.time_range.lock().unwrap();
        let visible_branches = self.visible_branches.lock().unwrap();

        // Filter visible commits
        let mut vertices: Vec<GitCommitVertex> = commit_cache.values()
            .filter(|v| visible_branches.is_empty() || visible_branches.contains(&v.branch_id))
            .cloned()
            .collect();

        if vertices.is_empty() {
            // Return empty framebuffer
            return Ok(vec![0; (config.render_width * config.render_height * 4) as usize]);
        }

        // Calculate optimal camera position
        let (camera_pos, camera_target) = if let Some(cuda) = &self.cuda_accelerator {
            cuda.calculate_optimal_camera(&vertices)?
        } else {
            self.calculate_optimal_camera_cpu(&vertices)
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
        let _ = self.event_sender.send(Git4DEvent::CameraUpdated(camera_pos, camera_target));

        // Transform vertices
        if let Some(cuda) = &self.cuda_accelerator {
            vertices = cuda.transform_vertices(&vertices, &transform, &params)?;
        } else {
            vertices = self.transform_vertices_cpu(&vertices, &transform, &params);
        }

        // Render to framebuffer
        let mut framebuffer = vec![0u32; (config.render_width * config.render_height) as usize];

        if let Some(cuda) = &self.cuda_accelerator {
            cuda.render_to_framebuffer(&vertices, &mut framebuffer, &params)?;
        } else {
            self.render_to_framebuffer_cpu(&vertices, &mut framebuffer, &params);
        }

        // Convert to RGBA bytes
        let rgba_data: Vec<u8> = framebuffer.into_iter()
            .flat_map(|pixel| {
                let r = ((pixel >> 16) & 0xFF) as u8;
                let g = ((pixel >> 8) & 0xFF) as u8;
                let b = (pixel & 0xFF) as u8;
                let a = ((pixel >> 24) & 0xFF) as u8;
                [r, g, b, a]
            })
            .collect();

        // Send render complete event
        let _ = self.event_sender.send(Git4DEvent::RenderComplete(rgba_data.clone()));

        Ok(rgba_data)
    }

    /// Process VR/AR interactions
    pub async fn process_vr_interactions(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(vr_ar) = &mut self.vr_ar_integration {
            // Subscribe to VR events
            let mut event_receiver = vr_ar.subscribe_events();

            loop {
                tokio::select! {
                    // Process VR events
                    event = event_receiver.recv() => {
                        if let Ok(event) = event {
                            self.handle_vr_event(event).await?;
                        }
                    }

                    // Process interactions
                    interaction = self.interaction_receiver.recv() => {
                        if let Some(interaction) = interaction {
                            self.handle_interaction(interaction).await?;
                        } else {
                            break;
                        }
                    }

                    // Periodic update
                    _ = time::sleep(Duration::from_millis(16)) => {
                        let events = vr_ar.update().await?;
                        for event in events {
                            self.handle_vr_event(event).await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle VR event
    async fn handle_vr_event(&mut self, event: VREvent) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            VREvent::GestureRecognized(gesture, hand) => {
                if let Some(vr_ar) = &mut self.vr_ar_integration {
                    // Get hand position (simplified)
                    let position = [0.0, 0.0, 0.0]; // Would get from VR system

                    if let Some(interaction) = vr_ar.handle_gesture_interaction(gesture, hand, position).await? {
                        let _ = self.interaction_sender.send(interaction.clone()).await;
                        let _ = self.event_sender.send(Git4DEvent::InteractionProcessed(interaction));
                    }
                }
            }
            VREvent::AnchorCreated(anchor) => {
                // Handle anchor creation for Git commits
                println!("Anchor created: {} at {:?}", anchor.id, anchor.position);
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle VR interaction
    async fn handle_interaction(&mut self, interaction: VRInteraction) -> Result<(), Box<dyn std::error::Error>> {
        match interaction {
            VRInteraction::SelectAnchor(anchor_id) => {
                println!("Selected anchor: {}", anchor_id);
                // Could highlight commit, show details, etc.
            }
            VRInteraction::CreateTimeAnchor(anchor_id) => {
                println!("Created time anchor: {}", anchor_id);
                // Could create time bookmark
            }
            VRInteraction::ToggleBranchVisibility => {
                // Toggle all branches or cycle through them
                let mut visible_branches = self.visible_branches.lock().unwrap();
                if visible_branches.is_empty() {
                    // Show all branches
                    let branch_cache = self.branch_cache.lock().unwrap();
                    for (branch_name, commits) in branch_cache.iter() {
                        if let Some(first_commit) = commits.first() {
                            if let Ok(vertex) = self.commit_cache.lock().unwrap().get(first_commit) {
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
                println!("Zoom to fit requested");
            }
            _ => {}
        }

        Ok(())
    }

    /// Traverse commit graph to collect commits
    fn traverse_commits(&self, start_commit: &Commit, commits: &mut Vec<Oid>, max_commits: usize) -> Result<(), Box<dyn std::error::Error>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(start_commit.id());
        visited.insert(start_commit.id());

        while let Some(commit_id) = queue.pop_front() {
            if commits.len() >= max_commits {
                break;
            }

            commits.push(commit_id);

            if let Ok(commit) = self.repository.find_commit(commit_id) {
                for parent in commit.parents() {
                    if !visited.contains(&parent.id()) {
                        visited.insert(parent.id());
                        queue.push_back(parent.id());
                    }
                }
            }
        }

        Ok(())
    }

    /// Get color for commit based on branch and author
    fn get_commit_color(&self, commit: &Commit, branch_id: u32) -> [f32; 4] {
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

        let max_diagonal = diagonal.iter().fold(0.0, |a, &b| a.max(b));
        let camera_distance = max_diagonal * 2.0;

        let camera_pos = [
            center[0],
            center[1] - camera_distance,
            center[2] + camera_distance * 0.5,
        ];

        (camera_pos, center)
    }

    /// CPU fallback for vertex transformation
    fn transform_vertices_cpu(&self, vertices: &[GitCommitVertex], transform: &TransformationMatrix, params: &RenderParameters) -> Vec<GitCommitVertex> {
        vertices.iter().map(|vertex| {
            let mut transformed = vertex.clone();

            // Apply time filtering
            if vertex.time < params.time_filter.0 || vertex.time > params.time_filter.1 {
                transformed.color[3] = 0.0;
                return transformed;
            }

            // Apply branch filtering
            let branch_visible = params.branch_filter.is_empty() ||
                params.branch_filter.contains(&vertex.branch_id);

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
                transform.matrix[0][0] * pos[0] + transform.matrix[0][1] * pos[1] + transform.matrix[0][2] * pos[2] + transform.matrix[0][3] * pos[3],
                transform.matrix[1][0] * pos[0] + transform.matrix[1][1] * pos[1] + transform.matrix[1][2] * pos[2] + transform.matrix[1][3] * pos[3],
                transform.matrix[2][0] * pos[0] + transform.matrix[2][1] * pos[1] + transform.matrix[2][2] * pos[2] + transform.matrix[2][3] * pos[3],
                transform.matrix[3][0] * pos[0] + transform.matrix[3][1] * pos[1] + transform.matrix[3][2] * pos[2] + transform.matrix[3][3] * pos[3],
            ];

            if transformed_pos[3] != 0.0 {
                transformed.position[0] = transformed_pos[0] / transformed_pos[3];
                transformed.position[1] = transformed_pos[1] / transformed_pos[3];
                transformed.position[2] = transformed_pos[2] / transformed_pos[3];
            }

            transformed
        }).collect()
    }

    /// CPU fallback for framebuffer rendering
    fn render_to_framebuffer_cpu(&self, vertices: &[GitCommitVertex], framebuffer: &mut [u32], params: &RenderParameters) {
        // Simple CPU-based rendering
        for vertex in vertices {
            if vertex.color[3] <= 0.0 {
                continue;
            }

            // Simple projection (simplified for CPU fallback)
            let screen_x = ((vertex.position[0] + 1.0) * 0.5 * params.viewport_width as f32) as i32;
            let screen_y = ((1.0 - (vertex.position[1] + 1.0) * 0.5) * params.viewport_height as f32) as i32;

            if screen_x >= 0 && screen_x < params.viewport_width as i32 &&
               screen_y >= 0 && screen_y < params.viewport_height as i32 {
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
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // Initialize a test git repo
        let repo = git2::Repository::init(repo_path).unwrap();

        // Create initial commit
        let sig = git2::Signature::now("Test Author", "test@example.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_id).unwrap();
        repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[]).unwrap();

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
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        // Initialize test repo with some commits
        let repo = git2::Repository::init(repo_path).unwrap();
        let sig = git2::Signature::now("Test Author", "test@example.com").unwrap();

        // Create a few commits
        for i in 0..3 {
            let tree_id = {
                let mut index = repo.index().unwrap();
                index.write_tree().unwrap()
            };
            let tree = repo.find_tree(tree_id).unwrap();

            let parent = if i > 0 {
                Some(repo.head().unwrap().peel_to_commit().unwrap())
            } else {
                None
            };

            let parents = parent.as_ref().map(|p| vec![p]).unwrap_or_default();
            repo.commit(Some("HEAD"), &sig, &sig, &format!("Commit {}", i), &tree, &parents).unwrap();
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

        let visualizer = Git4DAcceleratedVisualizer::new(repo_path, config).unwrap();
        assert!(visualizer.load_commits(&config).await.is_ok());
    }
}
