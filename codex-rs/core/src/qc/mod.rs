//! Quality Control (QC) Agent Module
//!
//! This module provides advanced quality control capabilities including:
//! - Statistical code quality analysis
//! - Quantum optimization algorithms
//! - Mathematical optimization for resource usage
//! - Code quality visualization and reporting

pub mod agent;
pub mod agent_coordination;
pub mod mathematical;
pub mod monitoring;
pub mod prediction;
pub mod quantum;
pub mod statistical;
pub mod visualization;

pub use agent::OptimizationResult;
pub use agent::QcAgent;
pub use agent::QcConfig;
pub use agent::QcMetrics;
pub use agent::QcReport;
pub use agent::QualityScore;
