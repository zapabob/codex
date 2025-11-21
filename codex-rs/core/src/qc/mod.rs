//! Quality Control (QC) Agent Module
//!
//! This module provides advanced quality control capabilities including:
//! - Statistical code quality analysis
//! - Quantum optimization algorithms
//! - Mathematical optimization for resource usage
//! - Code quality visualization and reporting

pub mod statistical;
pub mod quantum;
pub mod mathematical;
pub mod visualization;
pub mod agent;

pub use agent::QcAgent;
pub use agent::QcConfig;
pub use agent::QcReport;
pub use agent::QcMetrics;
pub use agent::OptimizationResult;
pub use agent::QualityScore;
