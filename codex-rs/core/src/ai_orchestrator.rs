//! AI Orchestration System for Central Development
//!
//! This module provides centralized task orchestration capabilities,
//! coordinating multiple sub-agents for complex development tasks.

use crate::conflict_detector::AstConflictDetector;
use crate::error::CodexErr;
use crate::error::Result;
use crate::git_lock_manager::ConflictDetectorTrait;
use crate::git_lock_manager::GitLockManager;
use crate::qc::agent_coordination::AgentCoordinator;
use crate::qc::agent_coordination::AgentType;
use crate::qc::agent_coordination::ParallelExecutionResult;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TaskPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Orchestrated task with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratedTask {
    pub id: String,
    pub description: String,
    pub priority: TaskPriority,
    pub dependencies: Vec<String>,
    pub assigned_agent: Option<String>,
    pub status: TaskStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub estimated_complexity: f64,
    pub tags: Vec<String>,
    /// QC quality requirements (for quality assurance tasks)
    pub quality_requirements: Option<QualityRequirements>,
}

/// Quality requirements for QC integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityRequirements {
    pub min_readability_score: f64,
    pub min_maintainability_score: f64,
    pub min_performance_score: f64,
    pub min_security_score: f64,
    pub max_complexity_score: f64,
    pub enable_statistical_analysis: bool,
    pub enable_quantum_optimization: bool,
    pub enable_mathematical_optimization: bool,
}

/// Task execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Blocked,
}

/// Development orchestration mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevelopmentMode {
    /// Central agent coordinates all sub-agents
    Centralized,
    /// Each agent works in separate git worktree
    Parallel,
}

/// Agent capabilities and workload
#[derive(Debug, Clone)]
pub struct AgentCapability {
    pub name: String,
    pub max_concurrent_tasks: usize,
    pub current_tasks: usize,
    pub specialization: Vec<String>,
    pub performance_score: f64,
}

/// Central orchestration engine
pub struct AIOrchestrator {
    tasks: Arc<Mutex<BTreeMap<String, OrchestratedTask>>>,
    agents: Arc<Mutex<BTreeMap<String, AgentCapability>>>,
    task_queue: Arc<Mutex<VecDeque<OrchestratedTask>>>,
    command_tx: mpsc::UnboundedSender<OrchestrationCommand>,
    command_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<OrchestrationCommand>>>>,
    qc_optimizer: Arc<QCOptimizer>,
    development_mode: DevelopmentMode,
    worktree_manager: Option<Arc<crate::orchestration::worktree_manager::WorktreeManager>>,
    #[allow(dead_code)]
    git_lock_manager: Option<Arc<GitLockManager>>,
    #[allow(dead_code)]
    conflict_detector: Option<Arc<Mutex<Box<dyn ConflictDetectorTrait + Send + Sync>>>>,
}

/// Commands for the orchestrator
#[derive(Debug)]
pub enum OrchestrationCommand {
    SubmitTask {
        task: OrchestratedTask,
        response: oneshot::Sender<Result<String>>,
    },
    UpdateTaskStatus {
        task_id: String,
        status: TaskStatus,
        agent_name: Option<String>,
    },
    RegisterAgent {
        name: String,
        capability: AgentCapability,
    },
    GetTaskStatus {
        task_id: String,
        response: oneshot::Sender<Result<TaskStatus>>,
    },
    OptimizeAssignment {
        response: oneshot::Sender<Result<Vec<(String, String)>>>,
    },
    SetDevelopmentMode {
        mode: DevelopmentMode,
        response: oneshot::Sender<Result<()>>,
    },
    Shutdown,
}

/// Quality Control and Optimization Engine
pub struct QCOptimizer {
    optimization_algorithms: Vec<Box<dyn OptimizationAlgorithm>>,
    #[allow(dead_code)]
    quality_metrics: BTreeMap<String, f64>,
}

/// Optimization algorithm trait
pub trait OptimizationAlgorithm {
    fn name(&self) -> &str;
    fn optimize(
        &self,
        tasks: &[OrchestratedTask],
        agents: &[AgentCapability],
    ) -> Vec<(String, String)>;
}

/// Mathematical optimization using linear programming
pub struct MathematicalOptimizer;

impl OptimizationAlgorithm for MathematicalOptimizer {
    fn name(&self) -> &str {
        "mathematical"
    }

    fn optimize(
        &self,
        tasks: &[OrchestratedTask],
        agents: &[AgentCapability],
    ) -> Vec<(String, String)> {
        // Implement linear programming for task-agent assignment
        // This is a simplified version - real implementation would use a solver
        let mut assignments = Vec::new();

        for task in tasks.iter().filter(|t| t.assigned_agent.is_none()) {
            let best_agent = agents
                .iter()
                .filter(|a| a.current_tasks < a.max_concurrent_tasks)
                .max_by(|a, b| {
                    let a_score = self.calculate_assignment_score(task, a);
                    let b_score = self.calculate_assignment_score(task, b);
                    a_score
                        .partial_cmp(&b_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

            if let Some(agent) = best_agent {
                assignments.push((task.id.clone(), agent.name.clone()));
            }
        }

        assignments
    }
}

impl MathematicalOptimizer {
    fn calculate_assignment_score(&self, task: &OrchestratedTask, agent: &AgentCapability) -> f64 {
        let mut score = 0.0;

        // Priority factor
        score += task.priority as i32 as f64 * 10.0;

        // Specialization match
        let specialization_match = task
            .tags
            .iter()
            .filter(|tag| agent.specialization.contains(tag))
            .count() as f64;
        score += specialization_match * 5.0;

        // Workload factor (prefer less busy agents)
        let workload_factor =
            1.0 - (agent.current_tasks as f64 / agent.max_concurrent_tasks as f64);
        score += workload_factor * 3.0;

        // Performance factor
        score += agent.performance_score;

        // Complexity compatibility
        let complexity_factor =
            1.0 - (task.estimated_complexity - agent.performance_score).abs() / 10.0;
        score += complexity_factor.max(0.0) * 2.0;

        score
    }
}

/// Quantum-inspired optimization using QAOA principles
pub struct QuantumOptimizer;

impl OptimizationAlgorithm for QuantumOptimizer {
    fn name(&self) -> &str {
        "quantum"
    }

    fn optimize(
        &self,
        tasks: &[OrchestratedTask],
        agents: &[AgentCapability],
    ) -> Vec<(String, String)> {
        // Simplified quantum-inspired optimization
        // Real implementation would use quantum algorithms
        let mut assignments = Vec::new();
        let mut used_agents = std::collections::BTreeSet::new();

        // Sort tasks by priority and complexity
        let mut sorted_tasks = tasks.to_vec();
        sorted_tasks.sort_by(|a, b| {
            let a_score = (a.priority as i32 as f64) * a.estimated_complexity;
            let b_score = (b.priority as i32 as f64) * b.estimated_complexity;
            b_score
                .partial_cmp(&a_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for task in sorted_tasks.iter().filter(|t| t.assigned_agent.is_none()) {
            let best_agent = agents
                .iter()
                .filter(|a| {
                    !used_agents.contains(&a.name) && a.current_tasks < a.max_concurrent_tasks
                })
                .max_by(|a, b| {
                    let a_score = self.quantum_assignment_score(task, a);
                    let b_score = self.quantum_assignment_score(task, b);
                    a_score
                        .partial_cmp(&b_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

            if let Some(agent) = best_agent {
                assignments.push((task.id.clone(), agent.name.clone()));
                used_agents.insert(agent.name.clone());
            }
        }

        assignments
    }
}

impl QuantumOptimizer {
    fn quantum_assignment_score(&self, task: &OrchestratedTask, agent: &AgentCapability) -> f64 {
        // Quantum-inspired scoring using superposition-like weighting
        let base_score = MathematicalOptimizer.calculate_assignment_score(task, agent);

        // Add quantum noise for exploration
        let quantum_noise = (fastrand::f64() - 0.5) * 0.1;
        base_score * (1.0 + quantum_noise)
    }
}

impl AIOrchestrator {
    pub fn new() -> Self {
        Self::with_mode(DevelopmentMode::Centralized)
    }

    pub fn with_mode(mode: DevelopmentMode) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let qc_optimizer = Arc::new(QCOptimizer {
            optimization_algorithms: vec![
                Box::new(MathematicalOptimizer),
                Box::new(QuantumOptimizer),
            ],
            quality_metrics: BTreeMap::new(),
        });

        let worktree_manager = if mode == DevelopmentMode::Parallel {
            Some(Arc::new(
                crate::orchestration::worktree_manager::WorktreeManager::new(".").unwrap(),
            ))
        } else {
            None
        };

        // Initialize Git lock manager if in parallel mode
        let git_lock_manager = if mode == DevelopmentMode::Parallel {
            Some(Arc::new(
                GitLockManager::new(".").unwrap().with_concurrency_limit(5),
            ))
        } else {
            None
        };

        let conflict_detector = if mode == DevelopmentMode::Parallel {
            Some(Arc::new(Mutex::new(
                Box::new(AstConflictDetector::new(std::path::PathBuf::from(".")))
                    as Box<dyn ConflictDetectorTrait + Send + Sync>,
            )))
        } else {
            None
        };

        Self {
            tasks: Arc::new(Mutex::new(BTreeMap::new())),
            agents: Arc::new(Mutex::new(BTreeMap::new())),
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
            command_tx: tx,
            command_rx: Arc::new(Mutex::new(Some(rx))),
            qc_optimizer,
            development_mode: mode,
            worktree_manager,
            git_lock_manager,
            conflict_detector,
        }
    }

    /// Set development mode
    pub async fn set_development_mode(&self, mode: DevelopmentMode) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(OrchestrationCommand::SetDevelopmentMode { mode, response: tx })
            .map_err(|_| CodexErr::InternalAgentDied)?;

        rx.await.map_err(|_| CodexErr::InternalAgentDied)?
    }

    /// Submit a new task for orchestration
    pub async fn submit_task(&self, task: OrchestratedTask) -> Result<String> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(OrchestrationCommand::SubmitTask { task, response: tx })
            .map_err(|_| CodexErr::InternalAgentDied)?;

        rx.await.map_err(|_| CodexErr::InternalAgentDied)?
    }

    /// Register a new agent
    pub async fn register_agent(&self, name: String, capability: AgentCapability) -> Result<()> {
        self.command_tx
            .send(OrchestrationCommand::RegisterAgent { name, capability })
            .map_err(|_| CodexErr::InternalAgentDied)?;
        Ok(())
    }

    /// Optimize task assignments using QC algorithms
    pub async fn optimize_assignments(&self) -> Result<Vec<(String, String)>> {
        let (tx, rx) = oneshot::channel();

        self.command_tx
            .send(OrchestrationCommand::OptimizeAssignment { response: tx })
            .map_err(|_| CodexErr::InternalAgentDied)?;

        rx.await.map_err(|_| CodexErr::InternalAgentDied)?
    }

    /// Run the orchestration engine
    pub async fn run(mut self) -> Result<()> {
        let mut rx = self.command_rx.lock().unwrap().take().unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                OrchestrationCommand::SubmitTask { task, response } => {
                    let task_id = task.id.clone();
                    self.tasks
                        .lock()
                        .unwrap()
                        .insert(task.id.clone(), task.clone());
                    self.task_queue.lock().unwrap().push_back(task);

                    let _ = response.send(Ok(task_id));
                }
                OrchestrationCommand::UpdateTaskStatus {
                    task_id,
                    status,
                    agent_name,
                } => {
                    if let Some(task) = self.tasks.lock().unwrap().get_mut(&task_id) {
                        task.status = status;
                        task.assigned_agent = agent_name;
                    }
                }
                OrchestrationCommand::RegisterAgent { name, capability } => {
                    self.agents.lock().unwrap().insert(name, capability);
                }
                OrchestrationCommand::GetTaskStatus { task_id, response } => {
                    let status = self
                        .tasks
                        .lock()
                        .unwrap()
                        .get(&task_id)
                        .map(|t| t.status)
                        .unwrap_or(TaskStatus::Pending);

                    let _ = response.send(Ok(status));
                }
                OrchestrationCommand::OptimizeAssignment { response } => {
                    let tasks: Vec<_> = self.tasks.lock().unwrap().values().cloned().collect();
                    let agents: Vec<_> = self.agents.lock().unwrap().values().cloned().collect();

                    let assignments = self.qc_optimizer.optimize_assignments(&tasks, &agents);
                    let _ = response.send(Ok(assignments));
                }
                OrchestrationCommand::SetDevelopmentMode { mode, response } => {
                    self.development_mode = mode;
                    self.worktree_manager = if mode == DevelopmentMode::Parallel {
                        Some(Arc::new(
                            crate::orchestration::worktree_manager::WorktreeManager::new(".")
                                .unwrap(),
                        ))
                    } else {
                        None
                    };
                    let _ = response.send(Ok(()));
                }
                OrchestrationCommand::Shutdown => break,
            }
        }

        Ok(())
    }
}

impl QCOptimizer {
    fn optimize_assignments(
        &self,
        tasks: &[OrchestratedTask],
        agents: &[AgentCapability],
    ) -> Vec<(String, String)> {
        let mut best_assignments = Vec::new();
        let mut best_score = f64::NEG_INFINITY;

        // Try different optimization algorithms
        for algorithm in &self.optimization_algorithms {
            let assignments = algorithm.optimize(tasks, agents);
            let score = self.evaluate_assignments(&assignments, tasks, agents);

            if score > best_score {
                best_score = score;
                best_assignments = assignments;
            }
        }

        best_assignments
    }

    fn evaluate_assignments(
        &self,
        assignments: &[(String, String)],
        tasks: &[OrchestratedTask],
        agents: &[AgentCapability],
    ) -> f64 {
        let mut total_score = 0.0;

        for (task_id, agent_name) in assignments {
            if let Some(task) = tasks.iter().find(|t| t.id == *task_id) {
                if let Some(agent) = agents.iter().find(|a| a.name == *agent_name) {
                    // Calculate assignment quality score
                    let priority_score = task.priority as i32 as f64 * 10.0;
                    let specialization_match = task
                        .tags
                        .iter()
                        .filter(|tag| agent.specialization.contains(tag))
                        .count() as f64
                        * 5.0;
                    let workload_penalty = if agent.current_tasks >= agent.max_concurrent_tasks {
                        -20.0
                    } else {
                        0.0
                    };

                    total_score += priority_score + specialization_match + workload_penalty;
                }
            }
        }

        total_score
    }
}

impl Default for AIOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_task_submission() {
        let orchestrator = AIOrchestrator::new();

        let task = OrchestratedTask {
            id: "test-task".to_string(),
            description: "Test task".to_string(),
            priority: TaskPriority::High,
            dependencies: vec![],
            assigned_agent: None,
            status: TaskStatus::Pending,
            created_at: chrono::Utc::now(),
            estimated_complexity: 5.0,
            tags: vec!["test".to_string()],
            quality_requirements: None,
        };

        let result = orchestrator.submit_task(task).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_mathematical_optimization() {
        let optimizer = MathematicalOptimizer;

        let tasks = vec![OrchestratedTask {
            id: "task1".to_string(),
            description: "Test task".to_string(),
            priority: TaskPriority::High,
            dependencies: vec![],
            assigned_agent: None,
            status: TaskStatus::Pending,
            created_at: chrono::Utc::now(),
            estimated_complexity: 5.0,
            tags: vec!["rust".to_string()],
            quality_requirements: None,
        }];

        let agents = vec![AgentCapability {
            name: "agent1".to_string(),
            max_concurrent_tasks: 5,
            current_tasks: 0,
            specialization: vec!["rust".to_string()],
            performance_score: 8.0,
        }];

        let assignments = optimizer.optimize(&tasks, &agents);
        assert_eq!(assignments.len(), 1);
        assert_eq!(assignments[0], ("task1".to_string(), "agent1".to_string()));
    }
}

impl AIOrchestrator {
    /// Execute QC quality assurance workflow
    ///
    /// # Arguments
    /// * `codebase_path` - Path to the codebase to analyze
    /// * `quality_requirements` - Quality requirements to enforce
    /// * `max_concurrent_agents` - Maximum number of concurrent QC agents
    ///
    /// # Returns
    /// Quality assurance results
    ///
    /// # Example
    /// ```no_run
    /// use codex_core::ai_orchestrator::{AIOrchestrator, QualityRequirements};
    ///
    /// # async fn example() {
    /// let orchestrator = AIOrchestrator::new();
    /// let requirements = QualityRequirements {
    ///     min_readability_score: 0.8,
    ///     min_maintainability_score: 0.75,
    ///     min_performance_score: 0.7,
    ///     min_security_score: 0.85,
    ///     max_complexity_score: 0.3,
    ///     enable_statistical_analysis: true,
    ///     enable_quantum_optimization: true,
    ///     enable_mathematical_optimization: true,
    /// };
    ///
    /// let _result = orchestrator
    ///     .execute_qc_quality_assurance("/path/to/codebase", requirements, 4)
    ///     .await;
    /// # }
    /// ```
    pub async fn execute_qc_quality_assurance(
        &self,
        codebase_path: &str,
        quality_requirements: QualityRequirements,
        max_concurrent_agents: usize,
    ) -> std::result::Result<QcQualityAssuranceResult, String> {
        println!(
            "🔍 Starting QC Quality Assurance workflow for: {}",
            codebase_path
        );

        // Initialize QC agent coordinator
        let coordinator = AgentCoordinator::new();

        // Register QC agents
        if quality_requirements.enable_statistical_analysis {
            coordinator.register_agent(
                "statistical_agent".to_string(),
                AgentType::StatisticalAnalyzer,
            );
        }
        if quality_requirements.enable_quantum_optimization {
            coordinator.register_agent("quantum_agent".to_string(), AgentType::QuantumOptimizer);
        }
        if quality_requirements.enable_mathematical_optimization {
            coordinator.register_agent(
                "mathematical_agent".to_string(),
                AgentType::MathematicalOptimizer,
            );
        }

        // Discover code files to analyze
        let code_files = self.discover_code_files(codebase_path)?;

        if code_files.is_empty() {
            return Err("No code files found for analysis".to_string());
        }

        println!("📁 Found {} code files for analysis", code_files.len());

        // Prepare analysis tasks
        let analysis_types = self.get_enabled_analysis_types(&quality_requirements);

        // Execute parallel QC analysis
        let parallel_results = coordinator
            .execute_parallel_qc_analysis(&code_files, &analysis_types, max_concurrent_agents)
            .await?;

        // Aggregate results
        let aggregated_results = self.aggregate_qc_results(&parallel_results)?;

        // Evaluate against quality requirements
        let compliance_result =
            self.evaluate_quality_compliance(&aggregated_results, &quality_requirements);

        // Generate improvement recommendations
        let recommendations =
            self.generate_qc_improvement_plan(&aggregated_results, &quality_requirements);

        let overall_compliance = compliance_result.overall_compliance;
        let result = QcQualityAssuranceResult {
            codebase_path: codebase_path.to_string(),
            total_files_analyzed: code_files.len(),
            quality_requirements,
            aggregated_results,
            compliance_result,
            recommendations,
            analysis_timestamp: chrono::Utc::now(),
            execution_duration_ms: 0, // Would be calculated in real implementation
        };

        println!("✅ QC Quality Assurance completed");
        println!("📊 Overall compliance: {:.1}%", overall_compliance * 100.0);

        Ok(result)
    }

    /// Discover code files in the codebase
    fn discover_code_files(&self, path: &str) -> std::result::Result<Vec<String>, String> {
        use std::fs;
        use std::path::Path;

        let path = Path::new(path);
        if !path.exists() {
            let path_display = path.display();
            return Err(format!("Path does not exist: {path_display}"));
        }

        let mut code_files = Vec::new();

        fn visit_dirs(dir: &Path, files: &mut Vec<String>) -> std::io::Result<()> {
            if dir.is_dir() {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();

                    if path.is_dir() {
                        // Skip common non-code directories
                        let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                        if !matches!(
                            dir_name,
                            "target" | "node_modules" | ".git" | "build" | "dist"
                        ) {
                            visit_dirs(&path, files)?;
                        }
                    } else if let Some(extension) = path.extension() {
                        // Include common programming language files
                        let ext_str = extension.to_string_lossy().to_lowercase();
                        if matches!(
                            ext_str.as_str(),
                            "rs" | "py"
                                | "js"
                                | "ts"
                                | "java"
                                | "cpp"
                                | "c"
                                | "h"
                                | "hpp"
                                | "go"
                                | "rb"
                                | "php"
                                | "cs"
                                | "swift"
                                | "kt"
                                | "scala"
                                | "clj"
                        ) {
                            if let Ok(content) = fs::read_to_string(&path) {
                                files.push(content);
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        visit_dirs(path, &mut code_files)
            .map_err(|e| format!("Failed to discover code files: {e}"))?;

        Ok(code_files)
    }

    /// Get enabled analysis types based on requirements
    fn get_enabled_analysis_types(&self, requirements: &QualityRequirements) -> Vec<AgentType> {
        let mut types = Vec::new();

        if requirements.enable_statistical_analysis {
            types.push(AgentType::StatisticalAnalyzer);
        }
        if requirements.enable_quantum_optimization {
            types.push(AgentType::QuantumOptimizer);
        }
        if requirements.enable_mathematical_optimization {
            types.push(AgentType::MathematicalOptimizer);
        }

        types
    }

    /// Aggregate parallel QC results
    fn aggregate_qc_results(
        &self,
        parallel_results: &[ParallelExecutionResult],
    ) -> std::result::Result<QcAggregatedResults, String> {
        let mut total_files = 0;
        let mut successful_analyses = 0;
        let mut average_scores = QcQualityScores {
            readability: 0.0,
            maintainability: 0.0,
            performance: 0.0,
            security: 0.0,
            overall: 0.0,
        };
        let mut total_execution_time = 0u64;

        for result in parallel_results {
            if result.success {
                successful_analyses += 1;

                if let Some(result_data) = &result.result {
                    // Parse QC scores from result (simplified)
                    if let Some(scores) = result_data.get("quality_scores") {
                        if let Some(scores_obj) = scores.as_object() {
                            average_scores.readability += scores_obj
                                .get("readability")
                                .and_then(|v| v.as_f64())
                                .unwrap_or(0.0);
                            average_scores.maintainability += scores_obj
                                .get("maintainability")
                                .and_then(|v| v.as_f64())
                                .unwrap_or(0.0);
                            average_scores.performance += scores_obj
                                .get("performance")
                                .and_then(|v| v.as_f64())
                                .unwrap_or(0.0);
                            average_scores.security += scores_obj
                                .get("security")
                                .and_then(|v| v.as_f64())
                                .unwrap_or(0.0);
                            average_scores.overall += scores_obj
                                .get("overall")
                                .and_then(|v| v.as_f64())
                                .unwrap_or(0.0);
                        }
                    }
                }
            }

            total_files += 1;
            total_execution_time += result.execution_time_ms;
        }

        // Calculate averages
        let count = successful_analyses as f64;
        if count > 0.0 {
            average_scores.readability /= count;
            average_scores.maintainability /= count;
            average_scores.performance /= count;
            average_scores.security /= count;
            average_scores.overall /= count;
        }

        Ok(QcAggregatedResults {
            total_files_analyzed: total_files,
            successful_analyses,
            average_quality_scores: average_scores,
            total_execution_time_ms: total_execution_time,
        })
    }

    /// Evaluate quality compliance against requirements
    fn evaluate_quality_compliance(
        &self,
        results: &QcAggregatedResults,
        requirements: &QualityRequirements,
    ) -> QualityComplianceResult {
        let scores = &results.average_quality_scores;

        let readability_ok = scores.readability >= requirements.min_readability_score;
        let maintainability_ok = scores.maintainability >= requirements.min_maintainability_score;
        let performance_ok = scores.performance >= requirements.min_performance_score;
        let security_ok = scores.security >= requirements.min_security_score;
        let complexity_ok = scores.overall <= requirements.max_complexity_score; // Note: using overall as complexity proxy

        let compliant_categories = [
            readability_ok,
            maintainability_ok,
            performance_ok,
            security_ok,
            complexity_ok,
        ]
        .iter()
        .filter(|&&ok| ok)
        .count();

        let overall_compliance = compliant_categories as f64 / 5.0;

        QualityComplianceResult {
            readability_compliant: readability_ok,
            maintainability_compliant: maintainability_ok,
            performance_compliant: performance_ok,
            security_compliant: security_ok,
            complexity_compliant: complexity_ok,
            overall_compliance,
            compliant_categories,
            total_categories: 5,
        }
    }

    /// Generate QC improvement recommendations
    fn generate_qc_improvement_plan(
        &self,
        results: &QcAggregatedResults,
        requirements: &QualityRequirements,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        let scores = &results.average_quality_scores;

        if scores.readability < requirements.min_readability_score {
            let readability = scores.readability;
            let target = requirements.min_readability_score;
            recommendations.push(format!(
                "Improve code readability (current: {readability:.2}, target: {target:.2}). Consider using consistent formatting and meaningful variable names.",
            ));
        }

        if scores.maintainability < requirements.min_maintainability_score {
            let maintainability = scores.maintainability;
            let target = requirements.min_maintainability_score;
            recommendations.push(format!(
                "Enhance code maintainability (current: {maintainability:.2}, target: {target:.2}). Focus on reducing code duplication and improving modularity.",
            ));
        }

        if scores.performance < requirements.min_performance_score {
            let performance = scores.performance;
            let target = requirements.min_performance_score;
            recommendations.push(format!(
                "Optimize code performance (current: {performance:.2}, target: {target:.2}). Consider algorithmic improvements and resource utilization optimization.",
            ));
        }

        if scores.security < requirements.min_security_score {
            let security = scores.security;
            let target = requirements.min_security_score;
            recommendations.push(format!(
                "Strengthen security measures (current: {security:.2}, target: {target:.2}). Implement input validation and secure coding practices.",
            ));
        }

        if scores.overall > requirements.max_complexity_score {
            let overall = scores.overall;
            let max_allowed = requirements.max_complexity_score;
            recommendations.push(format!(
                "Reduce code complexity (current: {overall:.2}, max allowed: {max_allowed:.2}). Break down complex functions and improve code structure.",
            ));
        }

        if recommendations.is_empty() {
            recommendations.push("All quality requirements are met. Consider implementing advanced optimization techniques.".to_string());
        }

        recommendations
    }
}

/// QC Quality Assurance Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcQualityAssuranceResult {
    pub codebase_path: String,
    pub total_files_analyzed: usize,
    pub quality_requirements: QualityRequirements,
    pub aggregated_results: QcAggregatedResults,
    pub compliance_result: QualityComplianceResult,
    pub recommendations: Vec<String>,
    pub analysis_timestamp: chrono::DateTime<chrono::Utc>,
    pub execution_duration_ms: u64,
}

/// Aggregated QC Results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcAggregatedResults {
    pub total_files_analyzed: usize,
    pub successful_analyses: usize,
    pub average_quality_scores: QcQualityScores,
    pub total_execution_time_ms: u64,
}

/// QC Quality Scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcQualityScores {
    pub readability: f64,
    pub maintainability: f64,
    pub performance: f64,
    pub security: f64,
    pub overall: f64,
}

/// Quality Compliance Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityComplianceResult {
    pub readability_compliant: bool,
    pub maintainability_compliant: bool,
    pub performance_compliant: bool,
    pub security_compliant: bool,
    pub complexity_compliant: bool,
    pub overall_compliance: f64,
    pub compliant_categories: usize,
    pub total_categories: usize,
}
