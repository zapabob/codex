//! QC optimization evaluator
//!
//! Small, deterministic helper for competition scoring.
//! Adds a conservative bonus derived from QC's mathematical + quantum metrics.

use crate::qc::QcReport;

/// Extra scoring info derived from QC optimization subsystems.
#[derive(Debug, Clone)]
pub struct QcOptimizationBonus {
    /// Bonus score in 0.0..=0.10 (intended to be used as a tie-breaker / small add-on).
    pub bonus: f64,
    /// Human-readable rationale (for logs).
    pub rationale: String,
}

/// Evaluate a QC report and compute a small bonus score.
pub fn evaluate_bonus(report: &QcReport) -> QcOptimizationBonus {
    let math = report.metrics.mathematical.optimization_score.clamp(0.0, 1.0);
    let quantum_pct = report.metrics.quantum.total_improvement_potential.max(0.0);
    let quantum = (quantum_pct / 100.0).clamp(0.0, 1.0);

    // Weighted blend: math slightly stronger than quantum.
    let blended = (0.65 * math) + (0.35 * quantum);

    // Keep bonus small: max 0.10 added to overall score.
    let bonus = (0.10 * blended).clamp(0.0, 0.10);

    let rationale = format!(
        "QC optimization bonus: +{bonus:.3} (math={math:.3}, quantum={quantum:.3} from {quantum_pct:.2}%)"
    );

    QcOptimizationBonus { bonus, rationale }
}

