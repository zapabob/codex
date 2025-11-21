//! Mathematical Optimization
//!
//! Provides mathematical optimization algorithms for:
//! - Resource allocation optimization
//! - Performance bottleneck identification
//! - Cost-benefit analysis
//! - Linear programming for resource constraints

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

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

/// Mathematical optimizer using linear programming concepts
pub struct MathematicalOptimizer;

impl MathematicalOptimizer {
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
        let optimizer = MathematicalOptimizer;
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
        let optimizer = MathematicalOptimizer;
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
