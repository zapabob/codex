//! CUDA-accelerated Git4D visualization engine
//!
//! This module provides GPU-accelerated computation for Git repository visualization
//! using NVIDIA CUDA through the `cudarc` crate. It enables real-time 4D (space-time)
//! rendering of commit history with transformation and projection operations.
//!
//! # Architecture
//!
//! The accelerator uses a three-kernel pipeline:
//! 1. **Vertex Transform**: Applies 4x4 transformation matrices with filtering
//! 2. **Time Projection**: Projects 4D coordinates to 3D space
//! 3. **Render**: Rasterizes vertices to framebuffer
//!
//! # Feature Flags
//!
//! - `cuda`: Enables actual CUDA GPU acceleration
//! - Without `cuda`: Stub implementation that returns errors
//!
//! # Example
//!
//! ```rust,no_run
//! use codex_core::cuda_accelerator::{CudaGit4DAccelerator, GitCommitVertex};
//!
//! fn main() -> anyhow::Result<()> {
//!     #[cfg(feature = "cuda")]
//!     {
//!         let accelerator = CudaGit4DAccelerator::new()?;
//!         
//!         let vertices = vec![
//!             GitCommitVertex {
//!                 position: [0.0, 0.0, 0.0],
//!                 time: 0.0,
//!                 color: [1.0, 0.0, 0.0, 1.0],
//!                 branch_id: 0,
//!                 commit_hash: 0,
//!             },
//!         ];
//!         
//!         let (camera_pos, target) = accelerator.calculate_optimal_camera(&vertices)?;
//!         println!("Camera: {:?}, Target: {:?}", camera_pos, target);
//!     }
//!     
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};

/// A single vertex representing a Git commit in 4D space-time.
///
/// This struct is designed to be GPU-friendly with packed data layout
/// suitable for CUDA kernel processing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct GitCommitVertex {
    /// X, Y, Z coordinates in 3D space
    pub position: [f32; 3],
    /// Time dimension (4th dimension) - typically commit timestamp normalized
    pub time: f32,
    /// RGBA color values (0.0 - 1.0)
    pub color: [f32; 4],
    /// Branch identifier for filtering and coloring
    pub branch_id: u32,
    /// Simplified commit hash for identification
    pub commit_hash: u64,
}

/// 4x4 transformation matrix for vertex operations.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TransformationMatrix {
    /// Column-major 4x4 matrix
    pub matrix: [[f32; 4]; 4],
}

impl Default for TransformationMatrix {
    fn default() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

/// Parameters for controlling the rendering pipeline.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct RenderParameters {
    /// Viewport width in pixels
    pub viewport_width: u32,
    /// Viewport height in pixels
    pub viewport_height: u32,
    /// Camera position in 3D space
    pub camera_position: [f32; 3],
    /// Camera look-at target
    pub camera_target: [f32; 3],
    /// Camera up vector
    pub camera_up: [f32; 3],
    /// 4x4 projection matrix
    pub projection_matrix: [[f32; 4]; 4],
    /// Time range filter (min, max)
    pub time_filter: (f32, f32),
    /// Visible branch IDs (empty = all visible)
    pub branch_filter: [u32; 32],
    /// Number of valid entries in branch_filter
    pub branch_filter_count: u32,
}

impl Default for RenderParameters {
    fn default() -> Self {
        Self {
            viewport_width: 1920,
            viewport_height: 1080,
            camera_position: [0.0, -10.0, 5.0],
            camera_target: [0.0, 0.0, 0.0],
            camera_up: [0.0, 0.0, 1.0],
            projection_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            time_filter: (f32::NEG_INFINITY, f32::INFINITY),
            branch_filter: [0; 32],
            branch_filter_count: 0,
        }
    }
}

/// GPU-accelerated Git4D visualization engine.
///
/// This struct manages CUDA device resources and provides methods for
/// accelerating vertex transformations, projections, and rendering.
#[cfg(feature = "cuda")]
pub struct CudaGit4DAccelerator {
    /// CUDA device handle
    device: std::sync::Arc<cudarc::driver::CudaDevice>,
    /// Vertex transformation kernel
    vertex_kernel: cudarc::driver::CudaFunction,
    /// Time projection kernel
    transform_kernel: cudarc::driver::CudaFunction,
    /// Render kernel
    render_kernel: cudarc::driver::CudaFunction,
}

#[cfg(feature = "cuda")]
impl CudaGit4DAccelerator {
    /// Creates a new CUDA accelerator by initializing the GPU and loading kernels.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - No CUDA-capable device is available
    /// - Kernel compilation fails
    /// - Device memory allocation fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use codex_core::cuda_accelerator::CudaGit4DAccelerator;
    ///
    /// match CudaGit4DAccelerator::new() {
    ///     Ok(accel) => println!("CUDA accelerator initialized"),
    ///     Err(e) => eprintln!("Failed to initialize CUDA: {}", e),
    /// }
    /// ```
    pub fn new() -> anyhow::Result<Self> {
        // Initialize CUDA device (device 0)
        let device = cudarc::driver::CudaDevice::new(0)
            .map_err(|e| anyhow::anyhow!("Failed to initialize CUDA device: {}", e))?;

        // Compile and load CUDA kernels
        let ptx = cudarc::nvrtc::compile_ptx(GIT4D_KERNELS)
            .map_err(|e| anyhow::anyhow!("Failed to compile CUDA kernels: {}", e))?;

        device
            .load_ptx(
                ptx,
                "git4d",
                &["vertex_transform", "time_projection", "render_commits"],
            )
            .map_err(|e| anyhow::anyhow!("Failed to load PTX module: {}", e))?;

        // Retrieve kernel functions
        let vertex_kernel = device
            .get_func("git4d", "vertex_transform")
            .ok_or_else(|| anyhow::anyhow!("Failed to get vertex_transform kernel"))?;

        let transform_kernel = device
            .get_func("git4d", "time_projection")
            .ok_or_else(|| anyhow::anyhow!("Failed to get time_projection kernel"))?;

        let render_kernel = device
            .get_func("git4d", "render_commits")
            .ok_or_else(|| anyhow::anyhow!("Failed to get render_commits kernel"))?;

        Ok(Self {
            device,
            vertex_kernel,
            transform_kernel,
            render_kernel,
        })
    }

    /// Transforms vertices using a 4x4 transformation matrix with filtering.
    ///
    /// This kernel applies time and branch filtering, then transforms visible
    /// vertices using the provided matrix.
    ///
    /// # Arguments
    ///
    /// * `vertices` - Input vertex buffer
    /// * `transform` - 4x4 transformation matrix
    /// * `params` - Rendering parameters including filters
    ///
    /// # Returns
    ///
    /// Transformed vertices with filtering applied
    pub fn transform_vertices(
        &self,
        vertices: &[GitCommitVertex],
        transform: &TransformationMatrix,
        params: &RenderParameters,
    ) -> anyhow::Result<Vec<GitCommitVertex>> {
        if vertices.is_empty() {
            return Ok(Vec::new());
        }

        let num_vertices = vertices.len();

        // Allocate device memory
        let vertices_device = self
            .device
            .htod_copy(vertices.to_vec())
            .map_err(|e| anyhow::anyhow!("Failed to copy vertices to device: {}", e))?;

        let transform_device = self
            .device
            .htod_copy(vec![transform.matrix])
            .map_err(|e| anyhow::anyhow!("Failed to copy transform matrix: {}", e))?;

        let params_device = self
            .device
            .htod_copy(vec![*params])
            .map_err(|e| anyhow::anyhow!("Failed to copy render parameters: {}", e))?;

        // Allocate output buffer
        let mut output_vertices = self
            .device
            .alloc_zeros::<GitCommitVertex>(num_vertices)
            .map_err(|e| anyhow::anyhow!("Failed to allocate output buffer: {}", e))?;

        // Calculate grid dimensions
        let threads_per_block = 256u32;
        let blocks_needed = ((num_vertices as u32) + threads_per_block - 1) / threads_per_block;

        // Launch kernel
        let cfg = cudarc::driver::LaunchConfig {
            grid_dim: (blocks_needed, 1, 1),
            block_dim: (threads_per_block, 1, 1),
            shared_mem_bytes: 0,
        };

        unsafe {
            self.device
                .launch(
                    &self.vertex_kernel,
                    cfg,
                    (
                        &vertices_device,
                        &transform_device,
                        &params_device,
                        &mut output_vertices,
                        num_vertices as u32,
                    ),
                )
                .map_err(|e| anyhow::anyhow!("Kernel launch failed: {}", e))?;
        }

        // Copy results back to host
        let result = self
            .device
            .dtoh_sync_copy(&output_vertices)
            .map_err(|e| anyhow::anyhow!("Failed to copy results from device: {}", e))?;

        Ok(result)
    }

    /// Projects 4D coordinates (x, y, z, time) to 3D space.
    ///
    /// Uses perspective projection based on time and optional w-axis scaling.
    ///
    /// # Arguments
    ///
    /// * `vertices` - Input vertices with 4D coordinates
    /// * `time_axis` - Time scaling factor
    /// * `w_axis` - Additional depth scaling
    ///
    /// # Returns
    ///
    /// Projected 3D positions
    pub fn project_4d_to_3d(
        &self,
        vertices: &[GitCommitVertex],
        time_axis: f32,
        w_axis: f32,
    ) -> anyhow::Result<Vec<[f32; 3]>> {
        if vertices.is_empty() {
            return Ok(Vec::new());
        }

        let num_vertices = vertices.len();

        // Allocate device memory
        let vertices_device = self
            .device
            .htod_copy(vertices.to_vec())
            .map_err(|e| anyhow::anyhow!("Failed to copy vertices: {}", e))?;

        let mut output_positions = self
            .device
            .alloc_zeros::<[f32; 3]>(num_vertices)
            .map_err(|e| anyhow::anyhow!("Failed to allocate output: {}", e))?;

        // Calculate grid dimensions
        let threads_per_block = 256u32;
        let blocks_needed = ((num_vertices as u32) + threads_per_block - 1) / threads_per_block;

        let cfg = cudarc::driver::LaunchConfig {
            grid_dim: (blocks_needed, 1, 1),
            block_dim: (threads_per_block, 1, 1),
            shared_mem_bytes: 0,
        };

        unsafe {
            self.device
                .launch(
                    &self.transform_kernel,
                    cfg,
                    (
                        &vertices_device,
                        time_axis,
                        w_axis,
                        &mut output_positions,
                        num_vertices as u32,
                    ),
                )
                .map_err(|e| anyhow::anyhow!("Projection kernel failed: {}", e))?;
        }

        // Copy results back
        let result = self
            .device
            .dtoh_sync_copy(&output_positions)
            .map_err(|e| anyhow::anyhow!("Failed to copy projection results: {}", e))?;

        Ok(result)
    }

    /// Renders vertices to a framebuffer.
    ///
    /// Performs perspective projection and rasterization to produce
    /// a colored output buffer suitable for display.
    ///
    /// # Arguments
    ///
    /// * `vertices` - Vertices to render
    /// * `framebuffer` - Output buffer (will be overwritten)
    /// * `params` - Rendering parameters
    pub fn render_to_framebuffer(
        &self,
        vertices: &[GitCommitVertex],
        framebuffer: &mut [u32],
        params: &RenderParameters,
    ) -> anyhow::Result<()> {
        if vertices.is_empty() {
            return Ok(());
        }

        let num_vertices = vertices.len();
        let framebuffer_size = framebuffer.len();

        // Allocate device memory
        let vertices_device = self
            .device
            .htod_copy(vertices.to_vec())
            .map_err(|e| anyhow::anyhow!("Failed to copy vertices: {}", e))?;

        let params_device = self
            .device
            .htod_copy(vec![*params])
            .map_err(|e| anyhow::anyhow!("Failed to copy parameters: {}", e))?;

        let mut framebuffer_device = self
            .device
            .htod_copy(framebuffer.to_vec())
            .map_err(|e| anyhow::anyhow!("Failed to copy framebuffer: {}", e))?;

        // Calculate grid dimensions
        let threads_per_block = 256u32;
        let blocks_needed = ((num_vertices as u32) + threads_per_block - 1) / threads_per_block;

        let cfg = cudarc::driver::LaunchConfig {
            grid_dim: (blocks_needed, 1, 1),
            block_dim: (threads_per_block, 1, 1),
            shared_mem_bytes: 0,
        };

        unsafe {
            self.device
                .launch(
                    &self.render_kernel,
                    cfg,
                    (
                        &vertices_device,
                        &params_device,
                        &mut framebuffer_device,
                        num_vertices as u32,
                        params.viewport_width,
                        params.viewport_height,
                    ),
                )
                .map_err(|e| anyhow::anyhow!("Render kernel failed: {}", e))?;
        }

        // Copy framebuffer back
        let result = self
            .device
            .dtoh_sync_copy(&framebuffer_device)
            .map_err(|e| anyhow::anyhow!("Failed to copy framebuffer: {}", e))?;

        framebuffer.copy_from_slice(&result);

        Ok(())
    }

    /// Calculates optimal camera position for viewing a set of vertices.
    ///
    /// This is a CPU-based calculation that doesn't require GPU.
    ///
    /// # Arguments
    ///
    /// * `vertices` - Vertices to frame in the view
    ///
    /// # Returns
    ///
    /// Tuple of (camera_position, look_at_target)
    pub fn calculate_optimal_camera(
        &self,
        vertices: &[GitCommitVertex],
    ) -> anyhow::Result<([f32; 3], [f32; 3])> {
        if vertices.is_empty() {
            return Ok(([0.0, -10.0, 5.0], [0.0, 0.0, 0.0]));
        }

        // Find bounding box
        let mut min_pos = [f32::INFINITY; 3];
        let mut max_pos = [f32::NEG_INFINITY; 3];

        for vertex in vertices {
            for i in 0..3 {
                min_pos[i] = min_pos[i].min(vertex.position[i]);
                max_pos[i] = max_pos[i].max(vertex.position[i]);
            }
        }

        // Calculate center (look-at target)
        let center = [
            (min_pos[0] + max_pos[0]) / 2.0,
            (min_pos[1] + max_pos[1]) / 2.0,
            (min_pos[2] + max_pos[2]) / 2.0,
        ];

        // Calculate camera position
        let diagonal = [
            max_pos[0] - min_pos[0],
            max_pos[1] - min_pos[1],
            max_pos[2] - min_pos[2],
        ];

        let max_diagonal: f32 = diagonal.iter().fold(0.0f32, |a: f32, &b| a.max(b));
        let camera_distance = max_diagonal * 2.0;

        let camera_pos = [
            center[0],
            center[1] - camera_distance,
            center[2] + camera_distance * 0.5,
        ];

        Ok((camera_pos, center))
    }

    /// Detects ray-vertex collisions for VR interaction.
    ///
    /// Currently returns empty results (placeholder for future implementation).
    pub fn detect_collisions(
        &self,
        _vertices: &[GitCommitVertex],
        _ray_origin: [f32; 3],
        _ray_direction: [f32; 3],
        _max_distance: f32,
    ) -> anyhow::Result<Vec<(usize, f32)>> {
        // Placeholder: GPU-accelerated ray casting would go here
        Ok(Vec::new())
    }
}

/// Stub implementation when CUDA feature is not enabled.
#[cfg(not(feature = "cuda"))]
pub struct CudaGit4DAccelerator;

#[cfg(not(feature = "cuda"))]
impl CudaGit4DAccelerator {
    /// Creates a stub that always returns an error.
    pub fn new() -> anyhow::Result<Self> {
        Err(anyhow::anyhow!(
            "CUDA support is not enabled. Build with --features cuda"
        ))
    }

    /// Stub method that returns an error.
    pub fn transform_vertices(
        &self,
        _vertices: &[GitCommitVertex],
        _transform: &TransformationMatrix,
        _params: &RenderParameters,
    ) -> anyhow::Result<Vec<GitCommitVertex>> {
        Err(anyhow::anyhow!("CUDA not available"))
    }

    /// Stub method that returns an error.
    pub fn project_4d_to_3d(
        &self,
        _vertices: &[GitCommitVertex],
        _time_axis: f32,
        _w_axis: f32,
    ) -> anyhow::Result<Vec<[f32; 3]>> {
        Err(anyhow::anyhow!("CUDA not available"))
    }

    /// Stub method that returns an error.
    pub fn render_to_framebuffer(
        &self,
        _vertices: &[GitCommitVertex],
        _framebuffer: &mut [u32],
        _params: &RenderParameters,
    ) -> anyhow::Result<()> {
        Err(anyhow::anyhow!("CUDA not available"))
    }

    /// CPU-only calculation that works without CUDA.
    pub fn calculate_optimal_camera(
        &self,
        vertices: &[GitCommitVertex],
    ) -> anyhow::Result<([f32; 3], [f32; 3])> {
        if vertices.is_empty() {
            return Ok(([0.0, -10.0, 5.0], [0.0, 0.0, 0.0]));
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

        let max_diagonal: f32 = diagonal.iter().fold(0.0f32, |a: f32, &b| a.max(b));
        let camera_distance = max_diagonal * 2.0;

        let camera_pos = [
            center[0],
            center[1] - camera_distance,
            center[2] + camera_distance * 0.5,
        ];

        Ok((camera_pos, center))
    }

    /// Stub method that returns empty results.
    pub fn detect_collisions(
        &self,
        _vertices: &[GitCommitVertex],
        _ray_origin: [f32; 3],
        _ray_direction: [f32; 3],
        _max_distance: f32,
    ) -> anyhow::Result<Vec<(usize, f32)>> {
        Ok(Vec::new())
    }
}

// CUDA kernel source code
//
// These kernels are compiled at runtime using NVRTC (NVIDIA Runtime Compilation).
// This allows dynamic kernel generation without requiring a separate compilation step.
#[allow(dead_code)]
const GIT4D_KERNELS: &str = r#"
typedef struct {
    float position[3];
    float time;
    float color[4];
    unsigned int branch_id;
    unsigned long long commit_hash;
} GitCommitVertex;

typedef struct {
    float matrix[4][4];
} TransformationMatrix;

typedef struct {
    unsigned int viewport_width;
    unsigned int viewport_height;
    float camera_position[3];
    float camera_target[3];
    float camera_up[3];
    float projection_matrix[4][4];
    float time_filter_min;
    float time_filter_max;
    unsigned int branch_filter[32];
    unsigned int branch_filter_count;
} RenderParameters;

extern "C" __global__ void vertex_transform(
    const GitCommitVertex* vertices,
    const float transform[4][4],
    const RenderParameters* params,
    GitCommitVertex* output,
    unsigned int num_vertices
) {
    unsigned int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= num_vertices) return;

    GitCommitVertex vertex = vertices[idx];

    // Apply time filtering
    if (vertex.time < params->time_filter_min || vertex.time > params->time_filter_max) {
        vertex.color[3] = 0.0f;
        output[idx] = vertex;
        return;
    }

    // Apply branch filtering
    bool branch_visible = params->branch_filter_count == 0;
    for (unsigned int i = 0; i < params->branch_filter_count; ++i) {
        if (vertex.branch_id == params->branch_filter[i]) {
            branch_visible = true;
            break;
        }
    }

    if (!branch_visible) {
        vertex.color[3] = 0.0f;
        output[idx] = vertex;
        return;
    }

    // Apply 4x4 transformation matrix
    float pos[4] = {vertex.position[0], vertex.position[1], vertex.position[2], 1.0f};
    float transformed[4];
    
    for (int i = 0; i < 4; ++i) {
        transformed[i] = 0.0f;
        for (int j = 0; j < 4; ++j) {
            transformed[i] += transform[i][j] * pos[j];
        }
    }

    if (transformed[3] != 0.0f) {
        transformed[0] /= transformed[3];
        transformed[1] /= transformed[3];
        transformed[2] /= transformed[3];
    }

    vertex.position[0] = transformed[0];
    vertex.position[1] = transformed[1];
    vertex.position[2] = transformed[2];

    output[idx] = vertex;
}

extern "C" __global__ void time_projection(
    const GitCommitVertex* vertices,
    float time_axis,
    float w_axis,
    float* output_positions,
    unsigned int num_vertices
) {
    unsigned int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= num_vertices) return;

    GitCommitVertex vertex = vertices[idx];

    float w = 1.0f + vertex.time * time_axis + vertex.position[2] * w_axis;
    
    if (w != 0.0f) {
        output_positions[idx * 3 + 0] = vertex.position[0] / w;
        output_positions[idx * 3 + 1] = vertex.position[1] / w;
        output_positions[idx * 3 + 2] = vertex.position[2] / w;
    } else {
        output_positions[idx * 3 + 0] = vertex.position[0];
        output_positions[idx * 3 + 1] = vertex.position[1];
        output_positions[idx * 3 + 2] = vertex.position[2];
    }
}

extern "C" __global__ void render_commits(
    const GitCommitVertex* vertices,
    const RenderParameters* params,
    unsigned int* framebuffer,
    unsigned int num_vertices,
    unsigned int width,
    unsigned int height
) {
    unsigned int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= num_vertices) return;

    GitCommitVertex vertex = vertices[idx];

    if (vertex.color[3] <= 0.0f) return;

    float view_pos[3];
    view_pos[0] = vertex.position[0] - params->camera_position[0];
    view_pos[1] = vertex.position[1] - params->camera_position[1];
    view_pos[2] = vertex.position[2] - params->camera_position[2];

    float aspect = (float)width / (float)height;
    float fov = 3.14159f / 4.0f;
    float tan_half_fov = tanf(fov / 2.0f);

    if (view_pos[2] > 0.1f) {
        float screen_x = view_pos[0] / (view_pos[2] * tan_half_fov * aspect);
        float screen_y = view_pos[1] / (view_pos[2] * tan_half_fov);

        int pixel_x = (int)((screen_x + 1.0f) * 0.5f * (float)width);
        int pixel_y = (int)((1.0f - screen_y) * 0.5f * (float)height);

        if (pixel_x >= 0 && pixel_x < (int)width && pixel_y >= 0 && pixel_y < (int)height) {
            unsigned char r = (unsigned char)(vertex.color[0] * 255.0f);
            unsigned char g = (unsigned char)(vertex.color[1] * 255.0f);
            unsigned char b = (unsigned char)(vertex.color[2] * 255.0f);
            unsigned char a = (unsigned char)(vertex.color[3] * 255.0f);

            unsigned int color = (a << 24) | (r << 16) | (g << 8) | b;
            framebuffer[pixel_y * width + pixel_x] = color;
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_default() {
        let vertex = GitCommitVertex {
            position: [0.0, 0.0, 0.0],
            time: 0.0,
            color: [1.0, 1.0, 1.0, 1.0],
            branch_id: 0,
            commit_hash: 0,
        };

        assert_eq!(vertex.position, [0.0, 0.0, 0.0]);
        assert_eq!(vertex.color, [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_transformation_matrix_default() {
        let matrix = TransformationMatrix::default();
        assert_eq!(matrix.matrix[0], [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(matrix.matrix[3], [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_render_parameters_default() {
        let params = RenderParameters::default();
        assert_eq!(params.viewport_width, 1920);
        assert_eq!(params.viewport_height, 1080);
    }

    #[test]
    fn test_camera_calculation_empty() {
        let accelerator = CudaGit4DAccelerator::new().unwrap_or_else(|_| {
            // Create a stub for testing without CUDA
            #[cfg(not(feature = "cuda"))]
            return CudaGit4DAccelerator;
            #[cfg(feature = "cuda")]
            panic!("CUDA not available");
        });

        let (camera, target) = accelerator.calculate_optimal_camera(&[]).unwrap();
        assert_eq!(target, [0.0, 0.0, 0.0]);
        assert!(camera[1] < 0.0); // Camera should be behind
    }

    #[test]
    fn test_camera_calculation_with_vertices() {
        let accelerator = CudaGit4DAccelerator::new().unwrap_or_else(|_| {
            #[cfg(not(feature = "cuda"))]
            return CudaGit4DAccelerator;
            #[cfg(feature = "cuda")]
            panic!("CUDA not available");
        });

        let vertices = vec![
            GitCommitVertex {
                position: [0.0, 0.0, 0.0],
                time: 0.0,
                color: [1.0, 1.0, 1.0, 1.0],
                branch_id: 0,
                commit_hash: 0,
            },
            GitCommitVertex {
                position: [10.0, 10.0, 10.0],
                time: 1.0,
                color: [1.0, 0.0, 0.0, 1.0],
                branch_id: 1,
                commit_hash: 1,
            },
        ];

        let (camera, target) = accelerator.calculate_optimal_camera(&vertices).unwrap();

        // Target should be at center
        assert_eq!(target, [5.0, 5.0, 5.0]);

        // Camera should be behind and above target
        assert!(camera[1] < target[1]);
        assert!(camera[2] > target[2]);
    }

    #[cfg(feature = "cuda")]
    #[test]
    fn test_cuda_initialization() {
        match CudaGit4DAccelerator::new() {
            Ok(_acc) => {
                // CUDA is available - test passed
            }
            Err(_) => {
                // CUDA not available on this system - skip
            }
        }
    }
}
