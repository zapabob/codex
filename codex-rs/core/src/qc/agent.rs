//! Quality Control Agent
//!
//! Main QC agent that orchestrates statistical analysis, quantum optimization,
//! mathematical optimization, and visualization to provide comprehensive
//! code quality assessment and improvement suggestions.

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

/// Quality Control Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcConfig {
    /// Enable statistical analysis
    pub enable_statistical: bool,
    /// Enable quantum optimization
    pub enable_quantum: bool,
    /// Enable mathematical optimization
    pub enable_mathematical: bool,
    /// Enable visualization
    pub enable_visualization: bool,
    /// Output directory for reports and charts
    pub output_dir: String,
    /// Minimum confidence threshold for suggestions
    pub min_confidence: f64,
    /// Enable detailed logging
    pub verbose: bool,
}

impl Default for QcConfig {
    fn default() -> Self {
        Self {
            enable_statistical: true,
            enable_quantum: true,
            enable_mathematical: true,
            enable_visualization: true,
            output_dir: "qc_reports".to_string(),
            min_confidence: 0.6,
            verbose: false,
        }
    }
}

impl Default for GpuConfig {
    fn default() -> Self {
        Self {
            enable_cuda: true,
            device_id: 0,
            memory_limit_mb: 4096,
            enable_parallel: true,
            num_threads: num_cpus::get(),
        }
    }
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            max_concurrent_agents: 4,
            communication_timeout_sec: 30,
            load_balancing_strategy: LoadBalancingStrategy::Adaptive,
        }
    }
}

/// Quality metrics aggregated from all analyses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcMetrics {
    /// Statistical metrics
    pub statistical: StatisticalMetrics,
    /// Quantum optimization results
    pub quantum: QuantumMetrics,
    /// Mathematical optimization results
    pub mathematical: MathematicalMetrics,
    /// Overall quality score (0.0-1.0)
    pub overall_score: f64,
    /// Analysis timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Statistical analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalMetrics {
    /// Code statistics
    pub code_stats: super::statistical::CodeStatistics,
    /// Quality indicators
    pub quality_indicators: HashMap<String, f64>,
}

/// Quantum optimization results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumMetrics {
    /// Optimization suggestions
    pub suggestions: Vec<super::quantum::OptimizationSuggestion>,
    /// Potential improvement percentage
    pub total_improvement_potential: f64,
}

/// Mathematical optimization results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathematicalMetrics {
    /// Resource allocation recommendations
    pub resource_allocation: Option<super::mathematical::ResourceAllocation>,
    /// Performance bottlenecks
    pub bottlenecks: Vec<super::mathematical::Bottleneck>,
    /// Optimization score
    pub optimization_score: f64,
}

/// Quality score breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScore {
    /// Readability score (0.0-1.0)
    pub readability: f64,
    /// Maintainability score (0.0-1.0)
    pub maintainability: f64,
    /// Performance score (0.0-1.0)
    pub performance: f64,
    /// Security score (0.0-1.0)
    pub security: f64,
    /// Overall score (0.0-1.0)
    pub overall: f64,
}

/// Complete QC analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcReport {
    /// Project name or file path
    pub target: String,
    /// Analysis configuration
    pub config: QcConfig,
    /// Quality metrics
    pub metrics: QcMetrics,
    /// Quality scores
    pub scores: QualityScore,
    /// Generated reports and charts
    pub outputs: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Optimization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationResult {
    /// Optimization type
    pub optimization_type: String,
    /// Success status
    pub success: bool,
    /// Performance improvement
    pub improvement: f64,
    /// Applied changes
    pub changes: Vec<String>,
}

/// GPU acceleration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    /// Enable CUDA acceleration
    pub enable_cuda: bool,
    /// GPU device ID
    pub device_id: usize,
    /// Memory limit (MB)
    pub memory_limit_mb: usize,
    /// Enable parallel processing
    pub enable_parallel: bool,
    /// Number of parallel threads
    pub num_threads: usize,
}

/// Parallel processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParallelConfig {
    /// Maximum concurrent agents
    pub max_concurrent_agents: usize,
    /// Agent communication timeout (seconds)
    pub communication_timeout_sec: u64,
    /// Load balancing strategy
    pub load_balancing_strategy: LoadBalancingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution
    RoundRobin,
    /// Load-based distribution
    LoadBased,
    /// Adaptive distribution
    Adaptive,
}

/// Main Quality Control Agent
pub struct QcAgent {
    config: QcConfig,
    #[allow(dead_code)]
    gpu_config: GpuConfig,
    #[allow(dead_code)]
    parallel_config: ParallelConfig,
    statistical_analyzer: super::statistical::StatisticalAnalyzer,
    quantum_optimizer: super::quantum::QuantumOptimizer,
    mathematical_optimizer: super::mathematical::MathematicalOptimizer,
    visualizer: super::visualization::QualityVisualizer,
}

impl QcAgent {
    /// Create new QC agent with default configuration
    pub fn new() -> Self {
        Self::with_full_config(
            QcConfig::default(),
            GpuConfig::default(),
            ParallelConfig::default(),
        )
    }

    /// Create new QC agent with custom QC configuration
    pub fn with_config(config: QcConfig) -> Self {
        Self::with_full_config(config, GpuConfig::default(), ParallelConfig::default())
    }

    /// Create new QC agent with full configuration
    pub fn with_full_config(
        config: QcConfig,
        gpu_config: GpuConfig,
        parallel_config: ParallelConfig,
    ) -> Self {
        Self {
            statistical_analyzer: super::statistical::StatisticalAnalyzer,
            quantum_optimizer: super::quantum::QuantumOptimizer,
            mathematical_optimizer: super::mathematical::MathematicalOptimizer::new(),
            visualizer: super::visualization::QualityVisualizer::new(),
            config,
            gpu_config,
            parallel_config,
        }
    }

    /// Perform comprehensive quality control analysis
    pub async fn analyze(&self, source: &str, target_name: &str) -> Result<QcReport, String> {
        if self.config.verbose {
            println!("🔍 Starting QC analysis for: {target_name}");
        }

        let mut outputs = Vec::new();
        let mut recommendations = Vec::new();

        // Statistical Analysis
        let statistical_metrics = if self.config.enable_statistical {
            if self.config.verbose {
                println!("📊 Performing statistical analysis...");
            }

            let code_stats = self.statistical_analyzer.analyze_code(source);

            // Generate statistical report
            let stats_report = self.statistical_analyzer.generate_report(&code_stats);
            let stats_path = format!("{}/{}_stats.txt", self.config.output_dir, target_name);
            self.save_report(&stats_path, &stats_report)?;
            outputs.push(stats_path);

            // Calculate quality indicators
            let quality_indicators = self.calculate_quality_indicators(&code_stats);

            Some(StatisticalMetrics {
                code_stats,
                quality_indicators,
            })
        } else {
            None
        };

        // Quantum Optimization Analysis
        let quantum_metrics = if self.config.enable_quantum {
            if self.config.verbose {
                println!("⚛️ Performing quantum optimization analysis...");
            }

            let suggestions = self.quantum_optimizer.analyze_optimizations(source);
            let filtered_suggestions: Vec<_> = suggestions
                .into_iter()
                .filter(|s| s.confidence >= self.config.min_confidence)
                .collect();

            let total_improvement = filtered_suggestions
                .iter()
                .map(|s| s.improvement_percentage)
                .sum::<f64>()
                / filtered_suggestions.len() as f64;

            // Generate optimization report
            let opt_report = self
                .quantum_optimizer
                .generate_report(&filtered_suggestions);
            let opt_path = format!(
                "{}/{}_optimizations.txt",
                self.config.output_dir, target_name
            );
            self.save_report(&opt_path, &opt_report)?;
            outputs.push(opt_path);

            // Generate visualization if enabled
            if self.config.enable_visualization {
                let chart_path = format!(
                    "{}/{}_optimizations.png",
                    self.config.output_dir, target_name
                );
                if let Err(e) = self
                    .visualizer
                    .generate_optimization_chart(&filtered_suggestions, &chart_path)
                {
                    eprintln!("Warning: Failed to generate optimization chart: {e}");
                } else {
                    outputs.push(chart_path);
                }
            }

            Some(QuantumMetrics {
                suggestions: filtered_suggestions,
                total_improvement_potential: total_improvement,
            })
        } else {
            None
        };

        // Mathematical Optimization (Resource Analysis)
        let mathematical_metrics = if self.config.enable_mathematical {
            if self.config.verbose {
                println!("🔢 Performing mathematical optimization analysis...");
            }

            // Create mock workload profile for demonstration
            let workload = super::mathematical::WorkloadProfile {
                estimated_memory_mb: 512,
                estimated_disk_gb: 5,
                estimated_time_sec: 300,
                parallelism_factor: 0.7,
                cpu_intensity: 0.6,
            };

            let constraints = super::mathematical::ResourceConstraints {
                cpu_cores: 8,
                memory_mb: 8192,
                disk_gb: 50,
                time_budget_sec: 1800,
            };

            let allocation = self
                .mathematical_optimizer
                .optimize_allocation(&constraints, &workload);

            // Create mock system metrics
            let metrics = super::mathematical::SystemMetrics {
                cpu_usage: 65.0,
                memory_usage: 70.0,
                io_operations_per_sec: 150.0,
                network_bandwidth_mbps: 50.0,
            };

            let bottlenecks = self.mathematical_optimizer.identify_bottlenecks(&metrics);

            // Generate resource report
            let resource_report = self
                .mathematical_optimizer
                .generate_report(&allocation, &bottlenecks);
            let resource_path = format!("{}/{}_resources.txt", self.config.output_dir, target_name);
            self.save_report(&resource_path, &resource_report)?;
            outputs.push(resource_path);

            // Generate visualization if enabled
            if self.config.enable_visualization {
                let chart_path =
                    format!("{}/{}_resources.png", self.config.output_dir, target_name);
                if let Err(e) = self
                    .visualizer
                    .generate_resource_chart(&allocation, &chart_path)
                {
                    eprintln!("Warning: Failed to generate resource chart: {e}");
                } else {
                    outputs.push(chart_path);
                }
            }

            Some(MathematicalMetrics {
                resource_allocation: Some(allocation),
                bottlenecks,
                optimization_score: 0.75, // Mock score
            })
        } else {
            None
        };

        // Calculate overall quality scores
        let scores = self.calculate_quality_scores(
            &statistical_metrics,
            &quantum_metrics,
            &mathematical_metrics,
        );

        // Generate recommendations
        recommendations.extend(self.generate_recommendations(
            &statistical_metrics,
            &quantum_metrics,
            &mathematical_metrics,
        ));

        // Create comprehensive report
        let has_statistical = statistical_metrics.is_some();
        let metrics = QcMetrics {
            statistical: statistical_metrics.unwrap_or_else(|| StatisticalMetrics {
                code_stats: super::statistical::CodeStatistics {
                    total_lines: 0,
                    code_lines: 0,
                    function_count: 0,
                    struct_count: 0,
                    import_count: 0,
                    avg_function_length: 0.0,
                    max_function_length: 0,
                    complexity_distribution: HashMap::new(),
                    duplication_percentage: 0.0,
                },
                quality_indicators: HashMap::new(),
            }),
            quantum: quantum_metrics.unwrap_or_else(|| QuantumMetrics {
                suggestions: Vec::new(),
                total_improvement_potential: 0.0,
            }),
            mathematical: mathematical_metrics.unwrap_or_else(|| MathematicalMetrics {
                resource_allocation: None,
                bottlenecks: Vec::new(),
                optimization_score: 0.0,
            }),
            overall_score: scores.overall,
            timestamp: chrono::Utc::now(),
        };

        // Generate dashboard if visualization is enabled
        if self.config.enable_visualization && has_statistical {
            let dashboard_path =
                format!("{}/{}_dashboard.png", self.config.output_dir, target_name);
            if let Err(e) = self
                .visualizer
                .generate_quality_dashboard(&metrics.statistical.code_stats, &dashboard_path)
            {
                eprintln!("Warning: Failed to generate quality dashboard: {e}");
            } else {
                outputs.push(dashboard_path);
            }
        }

        let report = QcReport {
            target: target_name.to_string(),
            config: self.config.clone(),
            metrics,
            scores: scores.clone(),
            outputs,
            recommendations,
        };

        if self.config.verbose {
            println!("✅ QC analysis completed for: {target_name}");
            println!(
                "📊 Overall quality score: {:.2}/1.0",
                scores.overall
            );
        }

        Ok(report)
    }

    /// Calculate quality indicators from statistical data
    fn calculate_quality_indicators(
        &self,
        stats: &super::statistical::CodeStatistics,
    ) -> HashMap<String, f64> {
        let mut indicators = HashMap::new();

        // Code density (higher is better, up to 0.8)
        let code_density = if stats.total_lines > 0 {
            stats.code_lines as f64 / stats.total_lines as f64
        } else {
            0.0
        };
        indicators.insert("code_density".to_string(), code_density.min(0.8) / 0.8);

        // Function size score (lower average length is better)
        let function_size_score = if stats.avg_function_length > 0.0 {
            (50.0 / stats.avg_function_length.max(10.0)).min(1.0)
        } else {
            0.0
        };
        indicators.insert("function_size_score".to_string(), function_size_score);

        // Complexity score (lower complexity is better)
        let avg_complexity = stats
            .complexity_distribution
            .iter()
            .map(|(complexity, count)| *complexity as f64 * *count as f64)
            .sum::<f64>()
            / stats.complexity_distribution.values().sum::<usize>() as f64;

        let complexity_score = if avg_complexity.is_finite() {
            (10.0 / avg_complexity.max(1.0)).min(1.0)
        } else {
            1.0
        };
        indicators.insert("complexity_score".to_string(), complexity_score);

        // Import efficiency (fewer imports per function is better)
        let import_efficiency = if stats.function_count > 0 {
            (20.0 / stats.import_count as f64).min(1.0)
        } else {
            0.0
        };
        indicators.insert("import_efficiency".to_string(), import_efficiency);

        indicators
    }

    /// Calculate overall quality scores
    fn calculate_quality_scores(
        &self,
        statistical: &Option<StatisticalMetrics>,
        quantum: &Option<QuantumMetrics>,
        mathematical: &Option<MathematicalMetrics>,
    ) -> QualityScore {
        let mut readability = 0.5;
        let mut maintainability = 0.5;
        let mut performance = 0.5;
        let mut security = 0.5;

        if let Some(stats) = statistical {
            // Readability based on code structure
            readability = stats.quality_indicators.get("code_density").unwrap_or(&0.5) * 0.7
                + stats
                    .quality_indicators
                    .get("function_size_score")
                    .unwrap_or(&0.5)
                    * 0.3;

            // Maintainability based on complexity and structure
            maintainability = stats
                .quality_indicators
                .get("complexity_score")
                .unwrap_or(&0.5)
                * 0.6
                + stats
                    .quality_indicators
                    .get("import_efficiency")
                    .unwrap_or(&0.5)
                    * 0.4;
        }

        if let Some(quantum) = quantum {
            // Performance based on optimization potential
            performance = (1.0 - quantum.total_improvement_potential / 100.0).max(0.0);
        }

        if let Some(mathematical) = mathematical {
            // Security based on resource optimization and bottleneck analysis
            security = mathematical.optimization_score;

            // Performance also influenced by mathematical optimization
            performance = (performance + mathematical.optimization_score) / 2.0;
        }

        let overall = (readability + maintainability + performance + security) / 4.0;

        QualityScore {
            readability: readability.min(1.0),
            maintainability: maintainability.min(1.0),
            performance: performance.min(1.0),
            security: security.min(1.0),
            overall: overall.min(1.0),
        }
    }

    /// Generate recommendations based on analysis
    fn generate_recommendations(
        &self,
        statistical: &Option<StatisticalMetrics>,
        quantum: &Option<QuantumMetrics>,
        mathematical: &Option<MathematicalMetrics>,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if let Some(stats) = statistical {
            if stats.code_stats.duplication_percentage > 20.0 {
                recommendations.push(format!("High code duplication ({:.1}%). Consider extracting common functionality into shared functions or modules.", stats.code_stats.duplication_percentage));
            }

            if stats.code_stats.avg_function_length > 30.0 {
                recommendations.push(format!("Average function length ({:.1} lines) is high. Consider breaking down large functions into smaller, focused functions.", stats.code_stats.avg_function_length));
            }

            if stats.code_stats.max_function_length > 100 {
                recommendations.push("Some functions are very long. Consider refactoring into smaller functions with single responsibilities.".to_string());
            }
        }

        if let Some(quantum) = quantum {
            for suggestion in &quantum.suggestions {
                if suggestion.confidence > 0.8 {
                    recommendations.push(format!(
                        "High-confidence optimization: {} (Expected improvement: {:.1}%)",
                        suggestion.description, suggestion.improvement_percentage
                    ));
                }
            }
        }

        if let Some(mathematical) = mathematical {
            for bottleneck in &mathematical.bottlenecks {
                recommendations.push(format!(
                    "Performance bottleneck: {} - {}",
                    bottleneck.description, bottleneck.recommendation
                ));
            }
        }

        if recommendations.is_empty() {
            recommendations
                .push("Code quality analysis completed. No major issues found.".to_string());
        }

        recommendations
    }

    /// Save report to file
    fn save_report(&self, path: &str, content: &str) -> Result<(), String> {
        // Create output directory if it doesn't exist
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {e}"))?;
        }

        std::fs::write(path, content).map_err(|e| format!("Failed to write report: {e}"))?;
        Ok(())
    }

    /// Get current configuration
    pub fn config(&self) -> &QcConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: QcConfig) {
        self.config = config;
    }
}

impl Default for QcAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qc_agent_creation() {
        let agent = QcAgent::new();
        assert_eq!(agent.config.output_dir, "qc_reports");
    }

    #[test]
    fn test_quality_score_calculation() {
        let agent = QcAgent::new();

        let scores = agent.calculate_quality_scores(&None, &None, &None);

        // Should return default scores
        assert!(scores.readability >= 0.0 && scores.readability <= 1.0);
        assert!(scores.maintainability >= 0.0 && scores.maintainability <= 1.0);
        assert!(scores.performance >= 0.0 && scores.performance <= 1.0);
        assert!(scores.security >= 0.0 && scores.security <= 1.0);
        assert!(scores.overall >= 0.0 && scores.overall <= 1.0);
    }

    #[test]
    fn test_config_update() {
        let mut agent = QcAgent::new();
        let new_config = QcConfig {
            enable_visualization: false,
            output_dir: "custom_reports".to_string(),
            ..Default::default()
        };

        agent.update_config(new_config);
        assert_eq!(agent.config.output_dir, "custom_reports");
        assert!(!agent.config.enable_visualization);
    }
}
