use anyhow::Result;
use futures::future::join_all;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::time::Duration;
use tokio::time::Instant;
use tokio::time::{self};

/// Multi-AI tool orchestration system for parallel development
pub struct AIToolManager {
    tools: HashMap<String, AITool>,
    active_sessions: Mutex<HashMap<String, ActiveSession>>,
    execution_engine: Arc<AsyncExecutionEngine>,
    task_distributor: TaskDistributor,
    result_integrator: ResultIntegrator,
    event_sender: broadcast::Sender<AIToolEvent>,
    resource_semaphore: Arc<Semaphore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AITool {
    pub id: String,
    pub name: String,
    pub command: Vec<String>,
    pub working_directory: Option<String>,
    pub environment_variables: HashMap<String, String>,
    pub capabilities: Vec<AICapability>,
    pub resource_requirements: ResourceRequirements,
    pub timeout_seconds: u64,
    pub max_concurrent_sessions: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AICapability {
    CodeGeneration,
    CodeReview,
    Testing,
    Documentation,
    Refactoring,
    Analysis,
    Chat,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_cores: f64,
    pub memory_mb: u64,
    pub concurrent_limit: usize,
}

#[derive(Debug, Clone)]
pub struct ActiveSession {
    pub session_id: String,
    pub tool_id: String,
    pub task_id: String,
    pub start_time: Instant,
    pub status: SessionStatus,
    pub process_handle: Option<tokio::process::Child>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionStatus {
    Starting,
    Running,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub enum AIToolEvent {
    SessionStarted(String),
    SessionCompleted(String, ExecutionResult),
    SessionFailed(String, String),
    TaskDistributed(String, Vec<String>),
    ResultsIntegrated(String, Vec<ExecutionResult>),
    ResourceAllocated(String),
    ResourceReleased(String),
}

#[derive(Debug, Clone)]
pub struct DevelopmentTask {
    pub id: String,
    pub description: String,
    pub requirements: Vec<AICapability>,
    pub complexity: TaskComplexity,
    pub dependencies: Vec<String>,
    pub priority: TaskPriority,
    pub estimated_duration: Duration,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskComplexity {
    Simple,   // Single file changes
    Medium,   // Multiple file changes
    Complex,  // Architecture changes
    Critical, // Breaking changes
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub session_id: String,
    pub tool_id: String,
    pub task_id: String,
    pub success: bool,
    pub output: String,
    pub error: String,
    pub execution_time: Duration,
    pub files_modified: Vec<String>,
    pub quality_score: f32,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl AIToolManager {
    pub fn new() -> Result<Self> {
        let (event_sender, _) = broadcast::channel(100);
        let execution_engine = Arc::new(AsyncExecutionEngine::new());
        let task_distributor = TaskDistributor::new();
        let result_integrator = ResultIntegrator::new();

        // Initialize with default AI tools
        let tools = Self::create_default_tools();

        Ok(Self {
            tools,
            active_sessions: Mutex::new(HashMap::new()),
            execution_engine,
            task_distributor,
            result_integrator,
            event_sender,
            resource_semaphore: Arc::new(Semaphore::new(10)), // Max 10 concurrent operations
        })
    }

    /// Execute a development task using multiple AI tools in parallel
    pub async fn execute_task_parallel(
        &self,
        task: DevelopmentTask,
    ) -> Result<TaskExecutionResult> {
        // Distribute task across available AI tools
        let subtasks = self.task_distributor.distribute_task(task.clone())?;

        let _ = self.event_sender.send(AIToolEvent::TaskDistributed(
            task.id.clone(),
            subtasks.iter().map(|st| st.id.clone()).collect(),
        ));

        // Execute subtasks in parallel
        let execution_futures: Vec<_> = subtasks
            .into_iter()
            .map(|subtask| {
                let execution_engine = Arc::clone(&self.execution_engine);
                let event_sender = self.event_sender.clone();

                async move {
                    // Acquire resource permit
                    let _permit = self.resource_semaphore.acquire().await.unwrap();

                    let result = execution_engine.execute_subtask(subtask.clone()).await;

                    // Send completion event
                    match &result {
                        Ok(exec_result) => {
                            let _ = event_sender.send(AIToolEvent::SessionCompleted(
                                exec_result.session_id.clone(),
                                exec_result.clone(),
                            ));
                        }
                        Err(e) => {
                            let _ = event_sender.send(AIToolEvent::SessionFailed(
                                subtask.id.clone(),
                                e.to_string(),
                            ));
                        }
                    }

                    result
                }
            })
            .collect();

        // Wait for all executions to complete
        let results = join_all(execution_futures).await;

        // Separate successful results and errors
        let mut successful_results = Vec::new();
        let mut errors = Vec::new();

        for result in results {
            match result {
                Ok(exec_result) => successful_results.push(exec_result),
                Err(e) => errors.push(e.to_string()),
            }
        }

        // Integrate results
        let integrated_result = self
            .result_integrator
            .integrate_results(task.clone(), successful_results, errors)
            .await?;

        let _ = self.event_sender.send(AIToolEvent::ResultsIntegrated(
            task.id.clone(),
            integrated_result.subtask_results.clone(),
        ));

        Ok(integrated_result)
    }

    /// Start a single AI tool session
    pub async fn start_tool_session(
        &self,
        tool_id: &str,
        task_description: &str,
        working_directory: Option<&str>,
    ) -> Result<String> {
        let tool = self
            .tools
            .get(tool_id)
            .ok_or_else(|| anyhow::anyhow!("AI tool not found"))?;

        // Check resource availability
        if self.get_active_sessions_for_tool(tool_id).await >= tool.max_concurrent_sessions {
            return Err(anyhow::anyhow!(
                "Maximum concurrent sessions reached for this tool"
            ));
        }

        // Create session
        let session_id = format!("session_{}_{}", tool_id, chrono::Utc::now().timestamp());
        let task_id = format!("task_{}", chrono::Utc::now().timestamp());

        let active_session = ActiveSession {
            session_id: session_id.clone(),
            tool_id: tool_id.to_string(),
            task_id: task_id.clone(),
            start_time: Instant::now(),
            status: SessionStatus::Starting,
            process_handle: None,
        };

        // Store session
        {
            let mut sessions = self.active_sessions.lock().unwrap();
            sessions.insert(session_id.clone(), active_session);
        }

        // Start the AI tool process
        let process_handle = self
            .execution_engine
            .start_tool_process(tool, &task_id, task_description, working_directory)
            .await?;

        // Update session with process handle
        {
            let mut sessions = self.active_sessions.lock().unwrap();
            if let Some(session) = sessions.get_mut(&session_id) {
                session.status = SessionStatus::Running;
                session.process_handle = Some(process_handle);
            }
        }

        let _ = self
            .event_sender
            .send(AIToolEvent::SessionStarted(session_id.clone()));

        Ok(session_id)
    }

    /// Stop an active session
    pub async fn stop_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.active_sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(session_id) {
            if let Some(mut process) = session.process_handle.take() {
                let _ = process.kill().await;
            }
            session.status = SessionStatus::Cancelled;
        }
        Ok(())
    }

    /// Get session status
    pub fn get_session_status(&self, session_id: &str) -> Option<SessionStatus> {
        self.active_sessions
            .lock()
            .unwrap()
            .get(session_id)
            .map(|s| s.status.clone())
    }

    /// List available AI tools
    pub fn list_tools(&self) -> Vec<&AITool> {
        self.tools.values().collect()
    }

    /// Get tool by ID
    pub fn get_tool(&self, tool_id: &str) -> Option<&AITool> {
        self.tools.get(tool_id)
    }

    /// Subscribe to AI tool events
    pub fn subscribe_events(&self) -> broadcast::Receiver<AIToolEvent> {
        self.event_sender.subscribe()
    }

    /// Get active sessions count for a tool
    async fn get_active_sessions_for_tool(&self, tool_id: &str) -> usize {
        self.active_sessions
            .lock()
            .unwrap()
            .values()
            .filter(|s| {
                s.tool_id == tool_id
                    && matches!(s.status, SessionStatus::Running | SessionStatus::Starting)
            })
            .count()
    }

    /// Create default AI tools configuration
    fn create_default_tools() -> HashMap<String, AITool> {
        let mut tools = HashMap::new();

        // Codex
        tools.insert(
            "codex".to_string(),
            AITool {
                id: "codex".to_string(),
                name: "Codex".to_string(),
                command: vec!["codex".to_string()],
                working_directory: None,
                environment_variables: HashMap::new(),
                capabilities: vec![
                    AICapability::CodeGeneration,
                    AICapability::CodeReview,
                    AICapability::Testing,
                    AICapability::Refactoring,
                    AICapability::Documentation,
                ],
                resource_requirements: ResourceRequirements {
                    cpu_cores: 2.0,
                    memory_mb: 4096,
                    concurrent_limit: 3,
                },
                timeout_seconds: 300,
                max_concurrent_sessions: 3,
            },
        );

        // Gemini CLI
        tools.insert(
            "gemini-cli".to_string(),
            AITool {
                id: "gemini-cli".to_string(),
                name: "Gemini CLI".to_string(),
                command: vec!["gemini".to_string(), "chat".to_string()],
                working_directory: None,
                environment_variables: HashMap::new(),
                capabilities: vec![
                    AICapability::CodeGeneration,
                    AICapability::Analysis,
                    AICapability::Chat,
                    AICapability::Documentation,
                ],
                resource_requirements: ResourceRequirements {
                    cpu_cores: 1.5,
                    memory_mb: 2048,
                    concurrent_limit: 5,
                },
                timeout_seconds: 180,
                max_concurrent_sessions: 5,
            },
        );

        // Claude Code
        tools.insert(
            "claude-code".to_string(),
            AITool {
                id: "claude-code".to_string(),
                name: "Claude Code".to_string(),
                command: vec!["claude".to_string(), "code".to_string()],
                working_directory: None,
                environment_variables: HashMap::new(),
                capabilities: vec![
                    AICapability::CodeGeneration,
                    AICapability::CodeReview,
                    AICapability::Refactoring,
                    AICapability::Testing,
                    AICapability::Analysis,
                ],
                resource_requirements: ResourceRequirements {
                    cpu_cores: 2.5,
                    memory_mb: 6144,
                    concurrent_limit: 2,
                },
                timeout_seconds: 600,
                max_concurrent_sessions: 2,
            },
        );

        tools
    }
}

/// Asynchronous execution engine for AI tools
pub struct AsyncExecutionEngine {
    active_processes: Mutex<HashMap<String, tokio::process::Child>>,
}

impl AsyncExecutionEngine {
    pub fn new() -> Self {
        Self {
            active_processes: Mutex::new(HashMap::new()),
        }
    }

    pub async fn execute_subtask(&self, subtask: SubTask) -> Result<ExecutionResult> {
        let tool = subtask.tool.clone();

        // Start tool process
        let mut child = self
            .start_tool_process(
                &tool,
                &subtask.id,
                &subtask.description,
                subtask.working_directory.as_deref(),
            )
            .await?;

        // Store process handle
        {
            let mut processes = self.active_processes.lock().unwrap();
            processes.insert(subtask.id.clone(), child);
        }

        // Wait for completion with timeout
        let start_time = Instant::now();
        let timeout_duration = Duration::from_secs(tool.timeout_seconds);

        let result = tokio::time::timeout(timeout_duration, async {
            let output = child.wait_with_output().await?;
            Ok(output)
        })
        .await;

        // Remove from active processes
        {
            let mut processes = self.active_processes.lock().unwrap();
            processes.remove(&subtask.id);
        }

        let execution_time = start_time.elapsed();

        match result {
            Ok(Ok(output)) => {
                let success = output.status.success();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                Ok(ExecutionResult {
                    session_id: subtask.id.clone(),
                    tool_id: tool.id.clone(),
                    task_id: subtask.parent_task_id.clone(),
                    success,
                    output: stdout,
                    error: stderr,
                    execution_time,
                    files_modified: vec![], // Would be populated by parsing output
                    quality_score: self.calculate_quality_score(&stdout, &stderr),
                    metadata: HashMap::new(),
                })
            }
            Ok(Err(e)) => Err(e.into()),
            Err(_) => Err(anyhow::anyhow!("Execution timeout")),
        }
    }

    pub async fn start_tool_process(
        &self,
        tool: &AITool,
        task_id: &str,
        task_description: &str,
        working_directory: Option<&str>,
    ) -> Result<tokio::process::Child> {
        let mut command = tokio::process::Command::new(&tool.command[0]);

        // Add remaining command arguments
        for arg in &tool.command[1..] {
            command.arg(arg);
        }

        // Set working directory
        if let Some(cwd) = working_directory.or(tool.working_directory.as_deref()) {
            command.current_dir(cwd);
        }

        // Set environment variables
        for (key, value) in &tool.environment_variables {
            command.env(key, value);
        }

        // Configure process
        command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let child = command.spawn()?;

        // Send initial task description if stdin is available
        if let Some(mut stdin) = child.stdin.take() {
            let input = format!("{}\n", task_description);
            let _ = tokio::io::AsyncWriteExt::write_all(&mut stdin, input.as_bytes()).await;
        }

        Ok(child)
    }

    fn calculate_quality_score(&self, stdout: &str, stderr: &str) -> f32 {
        // Simple quality scoring based on output characteristics
        let mut score = 0.5; // Base score

        // Positive indicators
        if stdout.contains("completed") || stdout.contains("success") {
            score += 0.2;
        }
        if stdout.lines().count() > 5 {
            score += 0.1; // Substantial output
        }
        if !stdout.is_empty() && stderr.is_empty() {
            score += 0.1; // Clean execution
        }

        // Negative indicators
        if stderr.contains("error") || stderr.contains("failed") {
            score -= 0.3;
        }
        if stdout.is_empty() && stderr.is_empty() {
            score -= 0.2; // No meaningful output
        }

        score.max(0.0).min(1.0)
    }
}

/// Task distribution system
pub struct TaskDistributor {
    task_queue: Mutex<VecDeque<DevelopmentTask>>,
}

impl TaskDistributor {
    pub fn new() -> Self {
        Self {
            task_queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn distribute_task(&self, task: DevelopmentTask) -> Result<Vec<SubTask>> {
        // Analyze task complexity and requirements
        let subtask_count = match task.complexity {
            TaskComplexity::Simple => 1,
            TaskComplexity::Medium => 2,
            TaskComplexity::Complex => 3,
            TaskComplexity::Critical => 4,
        };

        let mut subtasks = Vec::new();

        for i in 0..subtask_count {
            let subtask_description = self.generate_subtask_description(&task, i, subtask_count);

            let subtask = SubTask {
                id: format!("{}_subtask_{}", task.id, i),
                parent_task_id: task.id.clone(),
                description: subtask_description,
                required_capabilities: self.select_capabilities_for_subtask(&task.requirements, i),
                estimated_duration: task.estimated_duration / subtask_count as u32,
                priority: task.priority.clone(),
                tool: self.select_tool_for_subtask(&task.requirements, i).clone(),
                working_directory: None,
            };

            subtasks.push(subtask);
        }

        Ok(subtasks)
    }

    fn generate_subtask_description(
        &self,
        task: &DevelopmentTask,
        index: usize,
        total: usize,
    ) -> String {
        match task.complexity {
            TaskComplexity::Simple => task.description.clone(),
            TaskComplexity::Medium => match index {
                0 => format!("Implement core functionality: {}", task.description),
                1 => format!("Add tests and validation: {}", task.description),
                _ => task.description.clone(),
            },
            TaskComplexity::Complex => match index {
                0 => format!("Design architecture: {}", task.description),
                1 => format!("Implement core components: {}", task.description),
                2 => format!("Add testing and documentation: {}", task.description),
                _ => task.description.clone(),
            },
            TaskComplexity::Critical => match index {
                0 => format!("Analysis and design: {}", task.description),
                1 => format!("Core implementation: {}", task.description),
                2 => format!("Testing and validation: {}", task.description),
                3 => format!("Documentation and deployment: {}", task.description),
                _ => task.description.clone(),
            },
        }
    }

    fn select_capabilities_for_subtask(
        &self,
        requirements: &[AICapability],
        index: usize,
    ) -> Vec<AICapability> {
        match index {
            0 => requirements
                .iter()
                .filter(|cap| matches!(cap, AICapability::CodeGeneration | AICapability::Analysis))
                .cloned()
                .collect(),
            1 => requirements
                .iter()
                .filter(|cap| matches!(cap, AICapability::CodeGeneration | AICapability::Testing))
                .cloned()
                .collect(),
            2 => requirements
                .iter()
                .filter(|cap| matches!(cap, AICapability::Testing | AICapability::Documentation))
                .cloned()
                .collect(),
            _ => requirements.to_vec(),
        }
    }

    fn select_tool_for_subtask(
        &self,
        requirements: &[AICapability],
        index: usize,
    ) -> &'static AITool {
        // In real implementation, this would be more sophisticated
        // For now, return a default tool
        // This needs to be fixed to return an actual tool reference
        static DEFAULT_TOOL: AITool = AITool {
            id: "codex".to_string(),
            name: "Codex".to_string(),
            command: vec!["codex".to_string()],
            working_directory: None,
            environment_variables: HashMap::new(),
            capabilities: vec![AICapability::CodeGeneration],
            resource_requirements: ResourceRequirements {
                cpu_cores: 1.0,
                memory_mb: 1024,
                concurrent_limit: 1,
            },
            timeout_seconds: 300,
            max_concurrent_sessions: 1,
        };

        &DEFAULT_TOOL
    }
}

/// Result integration system
pub struct ResultIntegrator {
    integration_strategies: HashMap<String, IntegrationStrategy>,
}

impl ResultIntegrator {
    pub fn new() -> Self {
        let mut strategies = HashMap::new();
        strategies.insert("default".to_string(), IntegrationStrategy::Merge);
        strategies.insert(
            "code_generation".to_string(),
            IntegrationStrategy::BestQuality,
        );
        strategies.insert("testing".to_string(), IntegrationStrategy::Combine);

        Self {
            integration_strategies: strategies,
        }
    }

    pub async fn integrate_results(
        &self,
        original_task: DevelopmentTask,
        results: Vec<ExecutionResult>,
        errors: Vec<String>,
    ) -> Result<TaskExecutionResult> {
        // Determine integration strategy
        let strategy = self.select_integration_strategy(&original_task);

        // Apply integration strategy
        let integrated_result = match strategy {
            IntegrationStrategy::Merge => self.merge_results(results).await?,
            IntegrationStrategy::BestQuality => self.select_best_quality(results).await?,
            IntegrationStrategy::Combine => self.combine_results(results).await?,
            IntegrationStrategy::Vote => self.vote_on_results(results).await?,
        };

        Ok(TaskExecutionResult {
            task_id: original_task.id,
            success: integrated_result.success,
            integrated_output: integrated_result.output,
            subtask_results: results,
            errors,
            execution_time: results
                .iter()
                .map(|r| r.execution_time)
                .max()
                .unwrap_or(Duration::from_secs(0)),
            quality_score: integrated_result.quality_score,
            recommendations: integrated_result.recommendations,
        })
    }

    fn select_integration_strategy(&self, task: &DevelopmentTask) -> &IntegrationStrategy {
        if task.requirements.contains(&AICapability::CodeGeneration) {
            &IntegrationStrategy::BestQuality
        } else if task.requirements.contains(&AICapability::Testing) {
            &IntegrationStrategy::Combine
        } else {
            &IntegrationStrategy::Merge
        }
    }

    async fn merge_results(&self, results: Vec<ExecutionResult>) -> Result<IntegratedResult> {
        let mut combined_output = String::new();
        let mut total_quality = 0.0;

        for result in results {
            if result.success {
                combined_output.push_str(&format!(
                    "--- {} ---\n{}\n\n",
                    result.tool_id, result.output
                ));
                total_quality += result.quality_score;
            }
        }

        Ok(IntegratedResult {
            success: !results.is_empty() && results.iter().any(|r| r.success),
            output: combined_output,
            quality_score: total_quality / results.len() as f32,
            recommendations: vec!["Review merged results for consistency".to_string()],
        })
    }

    async fn select_best_quality(&self, results: Vec<ExecutionResult>) -> Result<IntegratedResult> {
        let best_result = results
            .into_iter()
            .filter(|r| r.success)
            .max_by(|a, b| a.quality_score.partial_cmp(&b.quality_score).unwrap())
            .ok_or_else(|| anyhow::anyhow!("No successful results"))?;

        Ok(IntegratedResult {
            success: true,
            output: best_result.output,
            quality_score: best_result.quality_score,
            recommendations: vec!["Selected highest quality result".to_string()],
        })
    }

    async fn combine_results(&self, results: Vec<ExecutionResult>) -> Result<IntegratedResult> {
        // Combine complementary results
        let mut combined_output = String::new();
        let mut all_files = HashSet::new();
        let mut total_quality = 0.0;

        for result in results {
            if result.success {
                combined_output.push_str(&format!(
                    "{} Results:\n{}\n\n",
                    result.tool_id, result.output
                ));
                all_files.extend(result.files_modified);
                total_quality += result.quality_score;
            }
        }

        Ok(IntegratedResult {
            success: true,
            output: combined_output,
            quality_score: total_quality / results.len() as f32,
            recommendations: vec![
                format!("Combined results from {} tools", results.len()),
                format!("Total files affected: {}", all_files.len()),
            ],
        })
    }

    async fn vote_on_results(&self, results: Vec<ExecutionResult>) -> Result<IntegratedResult> {
        // Simple voting mechanism - most common successful result
        let successful_results: Vec<_> = results.into_iter().filter(|r| r.success).collect();

        if successful_results.is_empty() {
            return Err(anyhow::anyhow!("No successful results to vote on"));
        }

        // For simplicity, return the first successful result
        let winner = &successful_results[0];

        Ok(IntegratedResult {
            success: true,
            output: winner.output.clone(),
            quality_score: winner.quality_score,
            recommendations: vec!["Selected by voting mechanism".to_string()],
        })
    }
}

#[derive(Debug, Clone)]
pub enum IntegrationStrategy {
    Merge,       // Combine all results
    BestQuality, // Select highest quality result
    Combine,     // Combine complementary results
    Vote,        // Vote on best result
}

#[derive(Debug, Clone)]
pub struct SubTask {
    pub id: String,
    pub parent_task_id: String,
    pub description: String,
    pub required_capabilities: Vec<AICapability>,
    pub estimated_duration: Duration,
    pub priority: TaskPriority,
    pub tool: &'static AITool, // This needs to be fixed - should not use static reference
    pub working_directory: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TaskExecutionResult {
    pub task_id: String,
    pub success: bool,
    pub integrated_output: String,
    pub subtask_results: Vec<ExecutionResult>,
    pub errors: Vec<String>,
    pub execution_time: Duration,
    pub quality_score: f32,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
struct IntegratedResult {
    success: bool,
    output: String,
    quality_score: f32,
    recommendations: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_tool_manager_creation() {
        let manager = AIToolManager::new().unwrap();
        assert!(manager.list_tools().len() > 0);
    }

    #[test]
    fn test_task_distribution() {
        let distributor = TaskDistributor::new();

        let task = DevelopmentTask {
            id: "test_task".to_string(),
            description: "Test task".to_string(),
            requirements: vec![AICapability::CodeGeneration],
            complexity: TaskComplexity::Simple,
            dependencies: vec![],
            priority: TaskPriority::Medium,
            estimated_duration: Duration::from_secs(60),
        };

        let subtasks = distributor.distribute_task(task).unwrap();
        assert!(!subtasks.is_empty());
    }
}
