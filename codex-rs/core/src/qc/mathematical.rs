//! Mathematical Optimization
//!
//! Provides mathematical optimization algorithms for:
//! - Resource allocation optimization
//! - Performance bottleneck identification
//! - Cost-benefit analysis
//! - Linear programming for resource constraints

use serde::Deserialize;
use serde::Serialize;

/// Resource allocation constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    /// Available CPU cores
    pub cpu_cores: usize,
    /// Available memory (MB)
    pub memory_mb: usize,
    /// Available disk space (GB)
    pub disk_gb: usize,
    /// Time budget (seconds)
    pub time_budget_sec: u64,
}

/// Resource allocation solution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    /// Allocated CPU cores
    pub allocated_cpu: usize,
    /// Allocated memory (MB)
    pub allocated_memory: usize,
    /// Allocated disk space (GB)
    pub allocated_disk: usize,
    /// Estimated execution time (seconds)
    pub estimated_time: u64,
    /// Optimization score (0.0-1.0)
    pub optimization_score: f64,
}

/// Linear programming problem definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearProgram {
    /// Objective function coefficients (maximize: c^T * x)
    pub objective_coeffs: Vec<f64>,
    /// Constraint matrix (A * x <= b)
    pub constraint_matrix: Vec<Vec<f64>>,
    /// Constraint bounds (b)
    pub constraint_bounds: Vec<f64>,
    /// Variable bounds (lower, upper) for each variable
    pub variable_bounds: Vec<(f64, f64)>,
}

/// Convex optimization problem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvexProblem {
    /// Objective function (convex)
    pub objective: ConvexFunction,
    /// Constraints (must be convex)
    pub constraints: Vec<ConvexConstraint>,
}

/// Convex function types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConvexFunction {
    /// Linear: c^T * x
    Linear(Vec<f64>),
    /// Quadratic: (1/2)x^T * Q * x + c^T * x
    Quadratic {
        quadratic_matrix: Vec<Vec<f64>>,
        linear_coeffs: Vec<f64>,
    },
    /// Sum of convex functions
    Sum(Vec<ConvexFunction>),
}

/// Convex constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvexConstraint {
    pub function: ConvexFunction,
    pub bound: f64,
    pub constraint_type: ConstraintType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    LessEqual,
    Equal,
    GreaterEqual,
}

/// Linear programming solution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LPSolution {
    pub optimal_value: f64,
    pub optimal_point: Vec<f64>,
    pub is_feasible: bool,
    pub iterations: usize,
}

/// Convex optimization solution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvexSolution {
    pub optimal_value: f64,
    pub optimal_point: Vec<f64>,
    pub is_feasible: bool,
    pub convergence: bool,
    pub iterations: usize,
}

impl ConvexFunction {
    /// Get dimension of the function
    pub fn dimension(&self) -> usize {
        match self {
            ConvexFunction::Linear(coeffs) => coeffs.len(),
            ConvexFunction::Quadratic {
                quadratic_matrix, ..
            } => quadratic_matrix.len(),
            ConvexFunction::Sum(functions) => functions.first().map(|f| f.dimension()).unwrap_or(0),
        }
    }

    /// Check if function is empty
    pub fn is_empty(&self) -> bool {
        self.dimension() == 0
    }
}

/// CUDA-accelerated linear algebra operations for optimization
#[cfg(feature = "cuda")]
pub mod cuda_math {
    use cudarc::driver::CudaDevice;
    use cudarc::driver::LaunchAsync;
    use cudarc::driver::LaunchConfig;
    use std::sync::Arc;

    /// CUDA-accelerated matrix-vector multiplication
    pub struct CudaLinearAlgebra {
        device: Arc<CudaDevice>,
        mat_vec_kernel: cudarc::driver::CudaFunction,
    }

    impl CudaLinearAlgebra {
        pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
            let device = CudaDevice::new(0)?;

            // Load PTX with matrix-vector multiplication kernel
            let ptx = cudarc::nvrtc::compile_ptx(LINEAR_ALGEBRA_KERNELS)?;
            device.load_ptx(ptx, "math", &["matrix_vector_mul"])?;

            let mat_vec_kernel = device.get_func("math", "matrix_vector_mul").unwrap();

            Ok(Self {
                device,
                mat_vec_kernel,
            })
        }

        /// Perform matrix-vector multiplication on GPU
        pub fn matrix_vector_mul(
            &self,
            matrix: &[Vec<f64>],
            vector: &[f64],
            result: &mut [f64],
        ) -> Result<(), Box<dyn std::error::Error>> {
            let rows = matrix.len();
            let cols = matrix[0].len();

            // Flatten matrix for GPU
            let mut flat_matrix = Vec::with_capacity(rows * cols);
            for row in matrix {
                flat_matrix.extend_from_slice(row);
            }

            // Allocate GPU memory
            let d_matrix = self.device.htod_copy(flat_matrix)?;
            let d_vector = self.device.htod_copy(vector.to_vec())?;
            let mut d_result = self.device.alloc_zeros::<f64>(rows)?;

            // Launch kernel
            let config = LaunchConfig::for_num_elems(rows as u32);
            unsafe {
                self.mat_vec_kernel.clone().launch(
                    config,
                    (
                        &d_matrix,
                        &d_vector,
                        &mut d_result,
                        rows as i32,
                        cols as i32,
                    ),
                )?;
            }

            // Copy result back
            self.device.dtoh_sync_copy_into(&d_result, result)?;

            Ok(())
        }
    }

    static LINEAR_ALGEBRA_KERNELS: &str = r#"
    extern "C" __global__ void matrix_vector_mul(
        const double* matrix,
        const double* vector,
        double* result,
        int rows,
        int cols
    ) {
        int row = blockIdx.x * blockDim.x + threadIdx.x;
        if (row < rows) {
            double sum = 0.0;
            for (int col = 0; col < cols; col++) {
                sum += matrix[row * cols + col] * vector[col];
            }
            result[row] = sum;
        }
    }
    "#;
}

/// Mathematical optimizer using linear programming concepts
pub struct MathematicalOptimizer {
    /// CUDA acceleration support (optional)
    #[cfg(feature = "cuda")]
    cuda_accel: Option<cuda_math::CudaLinearAlgebra>,
}

impl MathematicalOptimizer {
    /// Create new optimizer with optional CUDA acceleration
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "cuda")]
            cuda_accel: cuda_math::CudaLinearAlgebra::new().ok(),
        }
    }

    /// Create optimizer with explicit CUDA enable/disable
    pub fn with_cuda(cuda_enabled: bool) -> Self {
        #[cfg(feature = "cuda")]
        {
            Self {
                cuda_accel: if cuda_enabled {
                    cuda_math::CudaLinearAlgebra::new().ok()
                } else {
                    None
                },
            }
        }

        #[cfg(not(feature = "cuda"))]
        {
            let _ = cuda_enabled;
            Self {}
        }
    }
    /// Solve linear programming problem using simplex method approximation
    pub fn solve_linear_program(&self, problem: &LinearProgram) -> Result<LPSolution, String> {
        if problem.objective_coeffs.is_empty() {
            return Err("Objective function cannot be empty".to_string());
        }

        let n_vars = problem.objective_coeffs.len();
        let n_constraints = problem.constraint_matrix.len();

        // Validate dimensions
        for (i, row) in problem.constraint_matrix.iter().enumerate() {
            if row.len() != n_vars {
                return Err(format!(
                    "Constraint {} has {} variables, expected {}",
                    i,
                    row.len(),
                    n_vars
                ));
            }
        }

        if problem.constraint_bounds.len() != n_constraints {
            return Err(format!(
                "Expected {} constraint bounds, got {}",
                n_constraints,
                problem.constraint_bounds.len()
            ));
        }

        if problem.variable_bounds.len() != n_vars {
            return Err(format!(
                "Expected {} variable bounds, got {}",
                n_vars,
                problem.variable_bounds.len()
            ));
        }

        // Use simplified interior point method approximation
        self.solve_lp_interior_point(problem)
    }

    /// Solve convex optimization problem using gradient descent with projections
    pub fn solve_convex_problem(
        &self,
        problem: &ConvexProblem,
        max_iterations: usize,
        tolerance: f64,
    ) -> Result<ConvexSolution, String> {
        if problem.objective.is_empty() {
            return Err("Objective function cannot be empty".to_string());
        }

        let n_vars = problem.objective.dimension();

        // Initialize at feasible point (simple projection)
        let mut x = vec![0.0; n_vars];

        // Project onto feasible set
        x = self.project_onto_feasible_set(&x, &problem.constraints)?;

        let mut prev_value = self.evaluate_convex_function(&problem.objective, &x);

        for iteration in 0..max_iterations {
            // Compute gradient
            let gradient = self.compute_convex_gradient(&problem.objective, &x)?;

            // Compute step size (simplified line search)
            let step_size = self.compute_step_size(&x, &gradient, &problem.constraints, 1.0);

            // Update
            for i in 0..n_vars {
                x[i] -= step_size * gradient[i];
            }

            // Project onto feasible set
            x = self.project_onto_feasible_set(&x, &problem.constraints)?;

            let current_value = self.evaluate_convex_function(&problem.objective, &x);

            // Check convergence
            if (prev_value - current_value).abs() < tolerance {
                return Ok(ConvexSolution {
                    optimal_value: current_value,
                    optimal_point: x,
                    is_feasible: true,
                    convergence: true,
                    iterations: iteration + 1,
                });
            }

            prev_value = current_value;
        }

        // Maximum iterations reached
        let final_value = self.evaluate_convex_function(&problem.objective, &x);
        Ok(ConvexSolution {
            optimal_value: final_value,
            optimal_point: x,
            is_feasible: true,
            convergence: false,
            iterations: max_iterations,
        })
    }

    /// Interior point method for linear programming (simplified)
    fn solve_lp_interior_point(&self, problem: &LinearProgram) -> Result<LPSolution, String> {
        let n_vars = problem.objective_coeffs.len();
        let n_constraints = problem.constraint_matrix.len();

        // Initialize at interior point
        let mut x = vec![1.0; n_vars]; // Start with positive values
        let mut iterations = 0;
        let max_iterations = 100;
        let tolerance = 1e-6;

        for iteration in 0..max_iterations {
            iterations = iteration + 1;

            // Check feasibility
            let mut feasible = true;
            let mut constraint_violations = Vec::new();

            for i in 0..n_constraints {
                let constraint_value: f64 = problem.constraint_matrix[i]
                    .iter()
                    .zip(&x)
                    .map(|(a, xi)| a * xi)
                    .sum();

                if constraint_value > problem.constraint_bounds[i] + tolerance {
                    feasible = false;
                    constraint_violations.push(i);
                }
            }

            // Check variable bounds
            for i in 0..n_vars {
                let (lower, upper) = problem.variable_bounds[i];
                if x[i] < lower - tolerance || x[i] > upper + tolerance {
                    feasible = false;
                    break;
                }
            }

            if feasible {
                // Compute objective value
                let objective_value: f64 = problem
                    .objective_coeffs
                    .iter()
                    .zip(&x)
                    .map(|(c, xi)| c * xi)
                    .sum();

                return Ok(LPSolution {
                    optimal_value: objective_value,
                    optimal_point: x,
                    is_feasible: true,
                    iterations,
                });
            }

            // Take step towards feasibility (simplified gradient projection)
            for &violation_idx in &constraint_violations {
                let constraint_value: f64 = problem.constraint_matrix[violation_idx]
                    .iter()
                    .zip(&x)
                    .map(|(a, xi)| a * xi)
                    .sum();

                let violation = constraint_value - problem.constraint_bounds[violation_idx];

                // Project back onto constraint
                let step_size = violation
                    / problem.constraint_matrix[violation_idx]
                        .iter()
                        .map(|a| a * a)
                        .sum::<f64>()
                        .sqrt();

                for j in 0..n_vars {
                    x[j] -= step_size * problem.constraint_matrix[violation_idx][j];
                }
            }

            // Ensure variable bounds
            for i in 0..n_vars {
                let (lower, upper) = problem.variable_bounds[i];
                x[i] = x[i].max(lower).min(upper);
            }
        }

        // Maximum iterations reached, return best solution found
        let objective_value: f64 = problem
            .objective_coeffs
            .iter()
            .zip(&x)
            .map(|(c, xi)| c * xi)
            .sum();

        Ok(LPSolution {
            optimal_value: objective_value,
            optimal_point: x,
            is_feasible: false, // Could not find feasible solution within iterations
            iterations,
        })
    }

    /// Evaluate convex function at given point
    fn evaluate_convex_function(&self, function: &ConvexFunction, x: &[f64]) -> f64 {
        match function {
            ConvexFunction::Linear(coeffs) => coeffs.iter().zip(x).map(|(c, xi)| c * xi).sum(),
            ConvexFunction::Quadratic {
                quadratic_matrix,
                linear_coeffs,
            } => {
                let mut value = 0.0;

                // Quadratic term: (1/2)x^T * Q * x
                for i in 0..x.len() {
                    for j in 0..x.len() {
                        value += 0.5 * quadratic_matrix[i][j] * x[i] * x[j];
                    }
                }

                // Linear term: c^T * x
                for i in 0..linear_coeffs.len() {
                    value += linear_coeffs[i] * x[i];
                }

                value
            }
            ConvexFunction::Sum(functions) => functions
                .iter()
                .map(|f| self.evaluate_convex_function(f, x))
                .sum(),
        }
    }

    /// Compute gradient of convex function
    fn compute_convex_gradient(
        &self,
        function: &ConvexFunction,
        x: &[f64],
    ) -> Result<Vec<f64>, String> {
        let n = x.len();
        let mut gradient = vec![0.0; n];

        match function {
            ConvexFunction::Linear(coeffs) => {
                gradient.copy_from_slice(coeffs);
            }
            ConvexFunction::Quadratic {
                quadratic_matrix,
                linear_coeffs,
            } => {
                // Gradient of quadratic: Q * x + c
                for i in 0..n {
                    gradient[i] += linear_coeffs[i];
                    for j in 0..n {
                        gradient[i] += quadratic_matrix[i][j] * x[j];
                    }
                }
            }
            ConvexFunction::Sum(functions) => {
                for f in functions {
                    let grad_f = self.compute_convex_gradient(f, x)?;
                    for i in 0..n {
                        gradient[i] += grad_f[i];
                    }
                }
            }
        }

        Ok(gradient)
    }

    /// Compute step size for gradient descent (simplified line search)
    fn compute_step_size(
        &self,
        x: &[f64],
        gradient: &[f64],
        constraints: &[ConvexConstraint],
        initial_step: f64,
    ) -> f64 {
        let mut step_size = initial_step;
        let mut new_x = x.to_vec();

        // Simple backtracking line search
        for _ in 0..10 {
            // Try step
            for i in 0..x.len() {
                new_x[i] = x[i] - step_size * gradient[i];
            }

            // Check if still feasible
            if self.is_point_feasible(&new_x, constraints) {
                return step_size;
            }

            // Reduce step size
            step_size *= 0.5;
        }

        // Return minimum step size if no feasible step found
        step_size
    }

    /// Project point onto feasible set (simplified projection)
    fn project_onto_feasible_set(
        &self,
        x: &[f64],
        constraints: &[ConvexConstraint],
    ) -> Result<Vec<f64>, String> {
        let mut projected = x.to_vec();

        // For each constraint, project if violated
        for constraint in constraints {
            let value = self.evaluate_convex_function(&constraint.function, &projected);

            match constraint.constraint_type {
                ConstraintType::LessEqual => {
                    if value > constraint.bound {
                        // Simple projection for linear constraints
                        // In practice, this would be more sophisticated
                        if let ConvexFunction::Linear(coeffs) = &constraint.function {
                            let norm_squared: f64 = coeffs.iter().map(|c| c * c).sum();
                            if norm_squared > 0.0 {
                                let violation = value - constraint.bound;
                                let step_size = violation / norm_squared;

                                for i in 0..projected.len() {
                                    projected[i] -= step_size * coeffs[i];
                                }
                            }
                        }
                    }
                }
                ConstraintType::GreaterEqual => {
                    if value < constraint.bound {
                        // Similar projection for lower bounds
                        if let ConvexFunction::Linear(coeffs) = &constraint.function {
                            let norm_squared: f64 = coeffs.iter().map(|c| c * c).sum();
                            if norm_squared > 0.0 {
                                let violation = constraint.bound - value;
                                let step_size = violation / norm_squared;

                                for i in 0..projected.len() {
                                    projected[i] += step_size * coeffs[i];
                                }
                            }
                        }
                    }
                }
                ConstraintType::Equal => {
                    // Equality constraints are more complex - simplified handling
                    let diff = value - constraint.bound;
                    if diff.abs() > 1e-6 {
                        if let ConvexFunction::Linear(coeffs) = &constraint.function {
                            let norm_squared: f64 = coeffs.iter().map(|c| c * c).sum();
                            if norm_squared > 0.0 {
                                let step_size = diff / norm_squared;

                                for i in 0..projected.len() {
                                    projected[i] -= step_size * coeffs[i];
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(projected)
    }

    /// Check if point is feasible
    fn is_point_feasible(&self, x: &[f64], constraints: &[ConvexConstraint]) -> bool {
        for constraint in constraints {
            let value = self.evaluate_convex_function(&constraint.function, x);

            match constraint.constraint_type {
                ConstraintType::LessEqual => {
                    if value > constraint.bound + 1e-6 {
                        return false;
                    }
                }
                ConstraintType::GreaterEqual => {
                    if value < constraint.bound - 1e-6 {
                        return false;
                    }
                }
                ConstraintType::Equal => {
                    if (value - constraint.bound).abs() > 1e-6 {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Optimize resource allocation for given constraints
    pub fn optimize_allocation(
        &self,
        constraints: &ResourceConstraints,
        workload: &WorkloadProfile,
    ) -> ResourceAllocation {
        // Simplified linear programming approach
        // In a real implementation, this would use proper LP solvers

        let cpu_allocation = self.optimize_cpu_allocation(constraints.cpu_cores, workload);
        let memory_allocation = self.optimize_memory_allocation(constraints.memory_mb, workload);
        let disk_allocation = self.optimize_disk_allocation(constraints.disk_gb, workload);
        let time_estimation = self.estimate_execution_time(workload, cpu_allocation);

        let optimization_score = self.calculate_optimization_score(
            cpu_allocation,
            memory_allocation,
            disk_allocation,
            time_estimation,
            constraints,
        );

        ResourceAllocation {
            allocated_cpu: cpu_allocation,
            allocated_memory: memory_allocation,
            allocated_disk: disk_allocation,
            estimated_time: time_estimation,
            optimization_score,
        }
    }

    /// Optimize CPU core allocation
    fn optimize_cpu_allocation(&self, available_cores: usize, workload: &WorkloadProfile) -> usize {
        match workload.parallelism_factor {
            p if p > 0.8 => (available_cores as f64 * 0.9).min(available_cores as f64) as usize, // High parallelism
            p if p > 0.5 => (available_cores as f64 * 0.7).min(available_cores as f64) as usize, // Medium parallelism
            _ => (available_cores as f64 * 0.4).max(1.0) as usize, // Low parallelism
        }
    }

    /// Optimize memory allocation
    fn optimize_memory_allocation(
        &self,
        available_memory: usize,
        workload: &WorkloadProfile,
    ) -> usize {
        let base_memory = workload.estimated_memory_mb;
        let safety_margin = (base_memory as f64 * 1.2) as usize; // 20% safety margin
        safety_margin.min(available_memory)
    }

    /// Optimize disk allocation
    fn optimize_disk_allocation(&self, available_disk: usize, workload: &WorkloadProfile) -> usize {
        let estimated_disk = workload.estimated_disk_gb;
        estimated_disk.min(available_disk)
    }

    /// Estimate execution time
    fn estimate_execution_time(&self, workload: &WorkloadProfile, cpu_cores: usize) -> u64 {
        let base_time = workload.estimated_time_sec;
        let speedup = 1.0 - (cpu_cores as f64 - 1.0) * workload.parallelism_factor * 0.5;
        (base_time as f64 * speedup) as u64
    }

    /// Calculate overall optimization score
    fn calculate_optimization_score(
        &self,
        cpu: usize,
        memory: usize,
        disk: usize,
        time: u64,
        constraints: &ResourceConstraints,
    ) -> f64 {
        let cpu_efficiency = cpu as f64 / constraints.cpu_cores as f64;
        let memory_efficiency = memory as f64 / constraints.memory_mb as f64;
        let disk_efficiency = disk as f64 / constraints.disk_gb as f64;
        let time_efficiency = 1.0 - (time as f64 / constraints.time_budget_sec as f64).min(1.0);

        // Weighted average (time efficiency has highest weight)
        (cpu_efficiency * 0.2
            + memory_efficiency * 0.3
            + disk_efficiency * 0.2
            + time_efficiency * 0.3)
            .max(0.0)
            .min(1.0)
    }

    /// Identify performance bottlenecks
    pub fn identify_bottlenecks(&self, metrics: &SystemMetrics) -> Vec<Bottleneck> {
        let mut bottlenecks = Vec::new();

        // CPU bottleneck detection
        if metrics.cpu_usage > 90.0 {
            bottlenecks.push(Bottleneck {
                resource_type: ResourceType::Cpu,
                severity: Severity::Critical,
                description: "CPU usage is critically high".to_string(),
                recommendation: "Consider parallelization or algorithm optimization".to_string(),
                impact_score: 0.9,
            });
        } else if metrics.cpu_usage > 70.0 {
            bottlenecks.push(Bottleneck {
                resource_type: ResourceType::Cpu,
                severity: Severity::High,
                description: "CPU usage is high".to_string(),
                recommendation: "Monitor CPU usage and consider optimization".to_string(),
                impact_score: 0.6,
            });
        }

        // Memory bottleneck detection
        if metrics.memory_usage > 90.0 {
            bottlenecks.push(Bottleneck {
                resource_type: ResourceType::Memory,
                severity: Severity::Critical,
                description: "Memory usage is critically high".to_string(),
                recommendation: "Implement memory pooling or reduce memory footprint".to_string(),
                impact_score: 0.95,
            });
        }

        // I/O bottleneck detection
        if metrics.io_operations_per_sec > 1000.0 {
            bottlenecks.push(Bottleneck {
                resource_type: ResourceType::Io,
                severity: Severity::Medium,
                description: "High I/O operations detected".to_string(),
                recommendation: "Consider caching or batch operations".to_string(),
                impact_score: 0.4,
            });
        }

        bottlenecks
    }

    /// Generate mathematical optimization report
    pub fn generate_report(
        &self,
        allocation: &ResourceAllocation,
        bottlenecks: &[Bottleneck],
    ) -> String {
        let mut report = format!("Mathematical Optimization Report\n{}\n", "=".repeat(40));

        report.push_str("Resource Allocation:\n");
        report.push_str(&format!("  CPU Cores: {}\n", allocation.allocated_cpu));
        report.push_str(&format!("  Memory: {} MB\n", allocation.allocated_memory));
        report.push_str(&format!("  Disk: {} GB\n", allocation.allocated_disk));
        report.push_str(&format!(
            "  Estimated Time: {} seconds\n",
            allocation.estimated_time
        ));
        report.push_str(&format!(
            "  Optimization Score: {:.1}%\n\n",
            allocation.optimization_score * 100.0
        ));

        if !bottlenecks.is_empty() {
            report.push_str("Performance Bottlenecks:\n");
            for (i, bottleneck) in bottlenecks.iter().enumerate() {
                report.push_str(&format!(
                    "{}. {} ({})\n",
                    i + 1,
                    bottleneck.description,
                    match bottleneck.severity {
                        Severity::Low => "Low",
                        Severity::Medium => "Medium",
                        Severity::High => "High",
                        Severity::Critical => "Critical",
                    }
                ));
                report.push_str(&format!(
                    "   Recommendation: {}\n",
                    bottleneck.recommendation
                ));
                report.push_str(&format!(
                    "   Impact Score: {:.1}%\n\n",
                    bottleneck.impact_score * 100.0
                ));
            }
        } else {
            report.push_str("No significant bottlenecks detected.\n");
        }

        report
    }
}

/// Workload profile for optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadProfile {
    /// Estimated memory usage (MB)
    pub estimated_memory_mb: usize,
    /// Estimated disk usage (GB)
    pub estimated_disk_gb: usize,
    /// Estimated execution time (seconds)
    pub estimated_time_sec: u64,
    /// Parallelism factor (0.0-1.0)
    pub parallelism_factor: f64,
    /// CPU intensity factor (0.0-1.0)
    pub cpu_intensity: f64,
}

/// System metrics for bottleneck analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// I/O operations per second
    pub io_operations_per_sec: f64,
    /// Network bandwidth usage (Mbps)
    pub network_bandwidth_mbps: f64,
}

/// Performance bottleneck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    /// Type of resource bottleneck
    pub resource_type: ResourceType,
    /// Severity level
    pub severity: Severity,
    /// Description of the bottleneck
    pub description: String,
    /// Recommendation for resolution
    pub recommendation: String,
    /// Impact score (0.0-1.0)
    pub impact_score: f64,
}

/// Resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    Cpu,
    Memory,
    Io,
    Network,
}

/// Bottleneck severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_allocation() {
        let optimizer = MathematicalOptimizer::new();
        let constraints = ResourceConstraints {
            cpu_cores: 8,
            memory_mb: 8192,
            disk_gb: 100,
            time_budget_sec: 3600,
        };

        let workload = WorkloadProfile {
            estimated_memory_mb: 1024,
            estimated_disk_gb: 10,
            estimated_time_sec: 600,
            parallelism_factor: 0.8,
            cpu_intensity: 0.7,
        };

        let allocation = optimizer.optimize_allocation(&constraints, &workload);

        assert!(allocation.allocated_cpu > 0);
        assert!(allocation.allocated_memory > 0);
        assert!(allocation.allocated_cpu <= constraints.cpu_cores);
        assert!(allocation.allocated_memory <= constraints.memory_mb);
        assert!(allocation.optimization_score >= 0.0 && allocation.optimization_score <= 1.0);
    }

    #[test]
    fn test_bottleneck_detection() {
        let optimizer = MathematicalOptimizer::new();
        let metrics = SystemMetrics {
            cpu_usage: 95.0,
            memory_usage: 85.0,
            io_operations_per_sec: 500.0,
            network_bandwidth_mbps: 100.0,
        };

        let bottlenecks = optimizer.identify_bottlenecks(&metrics);

        assert!(
            bottlenecks
                .iter()
                .any(|b| matches!(b.resource_type, ResourceType::Cpu))
        );
        assert!(
            bottlenecks
                .iter()
                .any(|b| matches!(b.severity, Severity::Critical))
        );
    }
}
