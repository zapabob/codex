//! Plan schema definitions
//!
//! Defines the core data structures for plan mode, including the
//! PlanBlock which represents a complete planning artifact.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// Execution mode for Plan execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionMode {
    /// Single agent, no orchestration
    Single,
    /// Orchestrated control with central planner + sub-agents
    Orchestrated,
    /// Worktree competition with multiple variants
    Competition,
}

impl Default for ExecutionMode {
    fn default() -> Self {
        Self::Orchestrated
    }
}

impl std::fmt::Display for ExecutionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single => write!(f, "single"),
            Self::Orchestrated => write!(f, "orchestrated"),
            Self::Competition => write!(f, "competition"),
        }
    }
}

/// A single work item in the Plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkItem {
    /// Work item name
    pub name: String,
    /// Files that will be touched
    pub files_touched: Vec<String>,
    /// Diff contract (patch, full, etc.)
    pub diff_contract: String,
    /// Required tests
    pub tests: Vec<String>,
}

/// A risk item with mitigation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    /// Risk description
    pub item: String,
    /// Mitigation strategy
    pub mitigation: String,
}

/// Evaluation criteria for the Plan
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EvalCriteria {
    /// Tests that must pass
    pub tests: Vec<String>,
    /// Performance metrics and thresholds
    pub metrics: HashMap<String, String>,
}

/// Budget constraints for execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    /// Token budget per step
    pub max_step: Option<u64>,
    /// Session-wide token cap
    pub session_cap: Option<u64>,
    /// Time estimate in minutes
    pub estimate_min: Option<u64>,
    /// Time cap in minutes
    pub cap_min: Option<u64>,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            max_step: Some(20000),
            session_cap: Some(100000),
            estimate_min: Some(10),
            cap_min: Some(30),
        }
    }
}

/// Research source from DeepResearch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchSource {
    /// Source title
    pub title: String,
    /// Source URL
    pub url: String,
    /// Publication/access date
    pub date: String,
    /// Key finding from this source
    pub key_finding: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
}

/// Research block appended to Plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchBlock {
    /// Research query
    pub query: String,
    /// Search depth (1-3)
    pub depth: u8,
    /// Research strategy used
    pub strategy: String,
    /// Collected sources
    pub sources: Vec<ResearchSource>,
    /// Synthesized summary
    pub synthesis: String,
    /// Overall confidence (0.0-1.0)
    pub confidence: f64,
    /// Whether approval was required
    pub needs_approval: bool,
    /// Timestamp of research
    pub timestamp: DateTime<Utc>,
}

/// QC Quality Assurance block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcQualityBlock {
    /// Quality requirements to enforce
    pub quality_requirements: QcQualityRequirements,
    /// Quality analysis results
    pub analysis_results: Option<QcAnalysisResults>,
    /// Quality improvement recommendations
    pub recommendations: Vec<String>,
    /// Whether quality gates passed
    pub quality_gates_passed: bool,
    /// Timestamp of QC analysis
    pub timestamp: DateTime<Utc>,
}

/// QC Quality Requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcQualityRequirements {
    /// Minimum readability score (0.0-1.0)
    pub min_readability_score: f64,
    /// Minimum maintainability score (0.0-1.0)
    pub min_maintainability_score: f64,
    /// Minimum performance score (0.0-1.0)
    pub min_performance_score: f64,
    /// Minimum security score (0.0-1.0)
    pub min_security_score: f64,
    /// Maximum complexity score (0.0-1.0)
    pub max_complexity_score: f64,
    /// Enable statistical analysis
    pub enable_statistical_analysis: bool,
    /// Enable quantum optimization
    pub enable_quantum_optimization: bool,
    /// Enable mathematical optimization
    pub enable_mathematical_optimization: bool,
}

/// QC Analysis Results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcAnalysisResults {
    /// Overall quality scores
    pub quality_scores: QcQualityScores,
    /// Number of files analyzed
    pub files_analyzed: usize,
    /// Analysis execution time (ms)
    pub execution_time_ms: u64,
    /// Quality compliance status
    pub compliance_status: QcComplianceStatus,
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

/// QC Compliance Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QcComplianceStatus {
    pub overall_compliance: f64,
    pub compliant_categories: usize,
    pub total_categories: usize,
    pub critical_issues: Vec<String>,
}

/// Complete Plan block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanBlock {
    /// Unique Plan ID (timestamp-based)
    pub id: String,

    /// Plan title
    pub title: String,

    /// High-level goal
    pub goal: String,

    /// Assumptions made
    pub assumptions: Vec<String>,

    /// Clarifying questions
    pub clarifying_questions: Vec<String>,

    /// Approach description
    pub approach: String,

    /// Execution mode
    pub mode: ExecutionMode,

    /// Work items to complete
    pub work_items: Vec<WorkItem>,

    /// Identified risks
    pub risks: Vec<Risk>,

    /// Evaluation criteria
    pub eval: EvalCriteria,

    /// Budget constraints
    pub budget: Budget,

    /// Rollback plan
    pub rollback: String,

    /// Artifact paths (generated files)
    pub artifacts: Vec<String>,

    /// Optional research results
    pub research: Option<ResearchBlock>,

    /// Optional QC quality assurance results
    pub quality_assurance: Option<QcQualityBlock>,

    /// Current state
    pub state: super::state::PlanState,

    /// Whether approval is required
    pub need_approval: bool,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,

    /// User who created the Plan
    pub created_by: Option<String>,
}

impl PlanBlock {
    /// Create a new Plan from a goal
    pub fn new(goal: String, title: String) -> Self {
        let now = Utc::now();
        let id = format!(
            "{}_{}",
            now.format("%Y-%m-%dT%H:%M:%SZ"),
            title.to_lowercase().replace(' ', "-")
        );

        Self {
            id,
            title,
            goal,
            assumptions: Vec::new(),
            clarifying_questions: Vec::new(),
            approach: String::new(),
            mode: ExecutionMode::default(),
            work_items: Vec::new(),
            risks: Vec::new(),
            eval: EvalCriteria::default(),
            budget: Budget::default(),
            rollback: String::new(),
            artifacts: Vec::new(),
            research: None,
            quality_assurance: None,
            state: super::state::PlanState::Drafting,
            need_approval: true,
            created_at: now,
            updated_at: now,
            created_by: None,
        }
    }

    /// Update the timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Check if Plan can be executed
    pub fn can_execute(&self) -> bool {
        matches!(self.state, super::state::PlanState::Approved { .. })
    }

    /// Add a work item
    pub fn add_work_item(&mut self, item: WorkItem) {
        self.work_items.push(item);
        self.touch();
    }

    /// Add a risk
    pub fn add_risk(&mut self, risk: Risk) {
        self.risks.push(risk);
        self.touch();
    }

    /// Set research results
    pub fn set_research(&mut self, research: ResearchBlock) {
        self.research = Some(research);
        self.touch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Plan_creation() {
        let bp = PlanBlock::new("Add telemetry".to_string(), "feat-telemetry".to_string());

        assert!(bp.id.contains("feat-telemetry"));
        assert_eq!(bp.goal, "Add telemetry");
        assert!(matches!(bp.state, super::super::state::PlanState::Drafting));
        assert!(!bp.can_execute());
    }

    #[test]
    fn test_execution_mode_display() {
        assert_eq!(ExecutionMode::Single.to_string(), "single");
        assert_eq!(ExecutionMode::Orchestrated.to_string(), "orchestrated");
        assert_eq!(ExecutionMode::Competition.to_string(), "competition");
    }

    #[test]
    fn test_budget_defaults() {
        let budget = Budget::default();
        assert_eq!(budget.max_step, Some(20000));
        assert_eq!(budget.session_cap, Some(100000));
    }
}
