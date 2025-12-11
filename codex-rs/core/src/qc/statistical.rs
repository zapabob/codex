//! Statistical Quality Control
//!
//! Provides statistical analysis of code quality metrics including:
//! - Cyclomatic complexity analysis
//! - Code duplication detection
//! - Function length distribution
//! - Import dependency analysis

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// Statistical metrics for code analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeStatistics {
    /// Total lines of code
    pub total_lines: usize,
    /// Lines of actual code (excluding comments and blank lines)
    pub code_lines: usize,
    /// Total number of functions/methods
    pub function_count: usize,
    /// Total number of structs/classes
    pub struct_count: usize,
    /// Total number of imports/dependencies
    pub import_count: usize,
    /// Average function length (lines)
    pub avg_function_length: f64,
    /// Maximum function length
    pub max_function_length: usize,
    /// Cyclomatic complexity distribution
    pub complexity_distribution: HashMap<u8, usize>,
    /// Code duplication percentage
    pub duplication_percentage: f64,
}

/// ANOVA test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnovaResult {
    /// F-statistic value
    pub f_statistic: f64,
    /// p-value for the test
    pub p_value: f64,
    /// Degrees of freedom between groups
    pub df_between: usize,
    /// Degrees of freedom within groups
    pub df_within: usize,
    /// Sum of squares between groups
    pub ss_between: f64,
    /// Sum of squares within groups
    pub ss_within: f64,
    /// Total sum of squares
    pub ss_total: f64,
    /// Mean square between groups
    pub ms_between: f64,
    /// Mean square within groups
    pub ms_within: f64,
    /// Test significance (alpha = 0.05)
    pub significant: bool,
}

/// Statistical QC analyzer
pub struct StatisticalAnalyzer;

impl StatisticalAnalyzer {
    /// Perform ANOVA test on multiple sample groups
    ///
    /// # Arguments
    /// * `samples` - Vector of sample groups to compare
    ///
    /// # Returns
    /// ANOVA test result with F-statistic and p-value
    ///
    /// # Example
    /// ```
    /// use codex_core::qc::statistical::StatisticalAnalyzer;
    ///
    /// let analyzer = StatisticalAnalyzer;
    /// let group1 = vec![1.0, 2.0, 3.0];
    /// let group2 = vec![2.0, 3.0, 4.0];
    /// let group3 = vec![3.0, 4.0, 5.0];
    /// let samples = vec![group1, group2, group3];
    ///
    /// let result = analyzer.anova_test(&samples);
    /// println!("F-statistic: {:.3}", result.f_statistic);
    /// println!("p-value: {:.6}", result.p_value);
    /// println!("Significant: {}", result.significant);
    /// ```
    pub fn anova_test(&self, samples: &[Vec<f64>]) -> Result<AnovaResult, String> {
        if samples.is_empty() {
            return Err("No sample groups provided".to_string());
        }

        let k = samples.len(); // number of groups
        if k < 2 {
            return Err("ANOVA requires at least 2 groups".to_string());
        }

        // Check if all groups have data
        for (i, group) in samples.iter().enumerate() {
            if group.is_empty() {
                return Err(format!("Group {} has no data", i));
            }
        }

        // Calculate total number of observations
        let n_total: usize = samples.iter().map(|g| g.len()).sum();
        if n_total < 3 {
            return Err("ANOVA requires at least 3 total observations".to_string());
        }

        // Calculate overall mean
        let total_sum: f64 = samples.iter().flatten().sum();
        let overall_mean = total_sum / n_total as f64;

        // Calculate SSB (Sum of Squares Between)
        let ss_between = samples
            .iter()
            .map(|group| {
                let group_mean = group.iter().sum::<f64>() / group.len() as f64;
                let group_size = group.len() as f64;
                group_size * (group_mean - overall_mean).powi(2)
            })
            .sum::<f64>();

        // Calculate SSW (Sum of Squares Within)
        let ss_within = samples
            .iter()
            .map(|group| {
                let group_mean = group.iter().sum::<f64>() / group.len() as f64;
                group
                    .iter()
                    .map(|&x| (x - group_mean).powi(2))
                    .sum::<f64>()
            })
            .sum::<f64>();

        // Calculate SST (Sum of Squares Total)
        let ss_total = samples
            .iter()
            .flatten()
            .map(|&x| (x - overall_mean).powi(2))
            .sum::<f64>();

        // Degrees of freedom
        let df_between = k - 1;
        let df_within = n_total - k;

        // Mean squares
        let ms_between = ss_between / df_between as f64;
        let ms_within = ss_within / df_within as f64;

        // F-statistic
        let f_statistic = if ms_within > 0.0 {
            ms_between / ms_within
        } else {
            // Handle division by zero
            if ms_between > 0.0 { f64::INFINITY } else { 0.0 }
        };

        // Calculate p-value using F-distribution approximation
        // Using simplified approximation for p-value calculation
        let p_value = self.calculate_f_p_value(f_statistic, df_between as f64, df_within as f64);

        // Determine significance (alpha = 0.05)
        let significant = p_value < 0.05;

        Ok(AnovaResult {
            f_statistic,
            p_value,
            df_between,
            df_within,
            ss_between,
            ss_within,
            ss_total,
            ms_between,
            ms_within,
            significant,
        })
    }

    /// Approximate p-value for F-distribution using beta function approximation
    /// This is a simplified implementation for demonstration
    fn calculate_f_p_value(&self, f: f64, df1: f64, df2: f64) -> f64 {
        if f <= 0.0 {
            return 1.0;
        }

        // Simplified approximation using incomplete beta function
        // For production use, consider using a proper statistical library
        let x = df2 / (df2 + df1 * f);

        // Approximation using series expansion (simplified)
        if x >= 1.0 {
            return 0.0;
        }

        // Use regularized incomplete beta function approximation
        // This is a basic approximation - in production, use statrs or similar
        self.approximate_incomplete_beta(x, df2 / 2.0, df1 / 2.0)
    }

    /// Approximate regularized incomplete beta function
    /// Using continued fraction approximation (simplified)
    fn approximate_incomplete_beta(&self, x: f64, a: f64, b: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        if x >= 1.0 {
            return 1.0;
        }

        // Simplified approximation for demonstration
        // In production, use a proper implementation
        let beta = self.approximate_beta_function(a, b);

        if beta == 0.0 {
            return 0.0;
        }

        // Use series expansion for small x
        if x < 0.5 {
            let mut sum = 0.0;
            let mut term = 1.0;
            let mut k = 0;

            while k < 20 && term.abs() > 1e-10 {
                sum += term;
                k += 1;
                term *= (a + k as f64 - 1.0) / (a + b + k as f64 - 1.0) * x;
            }

            sum * self.approximate_beta_function(a, b)
        } else {
            // Use complement for large x
            1.0 - self.approximate_incomplete_beta(1.0 - x, b, a)
        }
    }

    /// Approximate beta function B(a,b) = Γ(a)Γ(b)/Γ(a+b)
    fn approximate_beta_function(&self, a: f64, b: f64) -> f64 {
        if a <= 0.0 || b <= 0.0 {
            return 0.0;
        }

        // Approximation using Gamma function approximation
        // Γ(z+1) ≈ sqrt(2πz) * (z/e)^z for large z
        // For small values, use known values and recurrence

        if a >= 1.0 && b >= 1.0 {
            // Approximation for a,b >= 1
            let sqrt_2pi = (2.0 * std::f64::consts::PI).sqrt();
            let gamma_a = sqrt_2pi * a.sqrt() * (a / std::f64::consts::E).powf(a);
            let gamma_b = sqrt_2pi * b.sqrt() * (b / std::f64::consts::E).powf(b);
            let gamma_ab = sqrt_2pi * (a + b).sqrt() * ((a + b) / std::f64::consts::E).powf(a + b);

            gamma_a * gamma_b / gamma_ab
        } else {
            // Use recurrence relation and known small values
            // This is simplified - in production use a proper Gamma function
            self.beta_recurrence(a, b)
        }
    }

    /// Beta function using recurrence relations for small values
    fn beta_recurrence(&self, a: f64, b: f64) -> f64 {
        // B(a,b) = B(a+1,b) * a/b for a < 1
        // B(a,b) = B(a,b+1) * b/a for b < 1

        let mut aa = a;
        let mut bb = b;

        // Boost a to >= 1
        let mut result = 1.0;
        while aa < 1.0 {
            result *= aa / bb;
            aa += 1.0;
        }

        // Boost b to >= 1
        while bb < 1.0 {
            result *= bb / aa;
            bb += 1.0;
        }

        // Now both >= 1, use approximation
        let sqrt_2pi = (2.0 * std::f64::consts::PI).sqrt();
        let gamma_a = sqrt_2pi * aa.sqrt() * (aa / std::f64::consts::E).powf(aa);
        let gamma_b = sqrt_2pi * bb.sqrt() * (bb / std::f64::consts::E).powf(bb);
        let gamma_ab = sqrt_2pi * (aa + bb).sqrt() * ((aa + bb) / std::f64::consts::E).powf(aa + bb);

        result * gamma_a * gamma_b / gamma_ab
    }
    /// Analyze code statistics from source text
    pub fn analyze_code(&self, source: &str) -> CodeStatistics {
        let lines: Vec<&str> = source.lines().collect();
        let total_lines = lines.len();

        let code_lines = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.is_empty()
                    && !trimmed.starts_with("//")
                    && !trimmed.starts_with("/*")
                    && !trimmed.starts_with("*")
                    && !trimmed.starts_with("///")
            })
            .count();

        let function_count = source.matches("fn ").count() + source.matches("pub fn ").count();
        let struct_count =
            source.matches("struct ").count() + source.matches("pub struct ").count();
        let import_count = source.matches("use ").count();

        // Calculate function lengths (simplified)
        let functions: Vec<&str> = source.split("fn ").collect();
        let mut function_lengths = Vec::new();

        for func in functions.iter().skip(1) {
            if let Some(end_pos) = func.find('{') {
                let func_body = &func[end_pos..];
                let brace_count = func_body.chars().fold(0, |count, c| match c {
                    '{' => count + 1,
                    '}' => count - 1,
                    _ => count,
                });
                if brace_count > 0 {
                    function_lengths.push(func_body.lines().count());
                }
            }
        }

        let avg_function_length = if !function_lengths.is_empty() {
            function_lengths.iter().sum::<usize>() as f64 / function_lengths.len() as f64
        } else {
            0.0
        };

        let max_function_length = function_lengths.iter().cloned().max().unwrap_or(0);

        // Simplified complexity analysis (count of control structures)
        let mut complexity_distribution = HashMap::new();
        for func in functions.iter().skip(1) {
            let complexity = func.matches("if ").count()
                + func.matches("while ").count()
                + func.matches("for ").count()
                + func.matches("match ").count()
                + 1; // base complexity
            *complexity_distribution.entry(complexity as u8).or_insert(0) += 1;
        }

        // Simplified duplication detection (very basic)
        let duplication_percentage = self.calculate_duplication(source);

        CodeStatistics {
            total_lines,
            code_lines,
            function_count,
            struct_count,
            import_count,
            avg_function_length,
            max_function_length,
            complexity_distribution,
            duplication_percentage,
        }
    }

    /// Calculate code duplication percentage (simplified implementation)
    fn calculate_duplication(&self, source: &str) -> f64 {
        let lines: Vec<&str> = source.lines().collect();
        if lines.len() < 2 {
            return 0.0;
        }

        let mut duplicate_lines = 0;
        let mut seen = HashMap::new();

        for line in &lines {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("//") {
                *seen.entry(trimmed).or_insert(0) += 1;
            }
        }

        for &count in seen.values() {
            if count > 1 {
                duplicate_lines += count - 1;
            }
        }

        if lines.len() > 0 {
            (duplicate_lines as f64 / lines.len() as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Generate statistical report
    pub fn generate_report(&self, stats: &CodeStatistics) -> String {
        format!(
            "Code Statistics Report\n{}\n\
            Total Lines: {}\n\
            Code Lines: {}\n\
            Functions: {}\n\
            Structs: {}\n\
            Imports: {}\n\
            Avg Function Length: {:.1} lines\n\
            Max Function Length: {} lines\n\
            Code Duplication: {:.1}%\n\
            Complexity Distribution: {:?}",
            "=".repeat(50),
            stats.total_lines,
            stats.code_lines,
            stats.function_count,
            stats.struct_count,
            stats.import_count,
            stats.avg_function_length,
            stats.max_function_length,
            stats.duplication_percentage,
            stats.complexity_distribution
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_analysis() {
        let analyzer = StatisticalAnalyzer;
        let test_code = r#"
// Simple test function
fn main() {
    println!("Hello, world!");
}

// Another function
fn add(a: i32, b: i32) -> i32 {
    if a > 0 {
        return a + b;
    }
    b
}
"#;

        let stats = analyzer.analyze_code(test_code);

        assert!(stats.total_lines > 0);
        assert!(stats.code_lines > 0);
        assert_eq!(stats.function_count, 2);
        assert!(stats.avg_function_length > 0.0);
    }

    #[test]
    fn test_duplication_calculation() {
        let analyzer = StatisticalAnalyzer;
        let duplicated_code = "line1\nline1\nline2\nline2\nline2\n";
        let duplication = analyzer.calculate_duplication(duplicated_code);

        assert!(duplication > 0.0);
        assert!(duplication <= 100.0);
    }

    #[test]
    fn test_anova_basic() {
        let analyzer = StatisticalAnalyzer;

        // Test with clearly different groups
        let group1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let group2 = vec![6.0, 7.0, 8.0, 9.0, 10.0];
        let samples = vec![group1, group2];

        let result = analyzer.anova_test(&samples).unwrap();

        // Groups are clearly different, so should be significant
        assert!(result.f_statistic > 0.0);
        assert!(result.p_value < 0.05); // Should be statistically significant
        assert!(result.significant);
        assert_eq!(result.df_between, 1); // 2 groups - 1
        assert_eq!(result.df_within, 8); // 10 total - 2 groups
    }

    #[test]
    fn test_anova_identical_groups() {
        let analyzer = StatisticalAnalyzer;

        // Test with identical groups (should not be significant)
        let group1 = vec![5.0, 5.0, 5.0];
        let group2 = vec![5.0, 5.0, 5.0];
        let samples = vec![group1, group2];

        let result = analyzer.anova_test(&samples).unwrap();

        // Groups are identical, F-statistic should be very small or zero
        assert!(result.f_statistic >= 0.0);
        assert!(result.p_value >= 0.05); // Should not be statistically significant
        assert!(!result.significant);
    }

    #[test]
    fn test_anova_three_groups() {
        let analyzer = StatisticalAnalyzer;

        // Test with three groups
        let group1 = vec![1.0, 1.5, 2.0];
        let group2 = vec![3.0, 3.5, 4.0];
        let group3 = vec![5.0, 5.5, 6.0];
        let samples = vec![group1, group2, group3];

        let result = analyzer.anova_test(&samples).unwrap();

        assert_eq!(result.df_between, 2); // 3 groups - 1
        assert_eq!(result.df_within, 6); // 9 total - 3 groups
        assert!(result.f_statistic > 0.0);
    }

    #[test]
    fn test_anova_error_cases() {
        let analyzer = StatisticalAnalyzer;

        // Test empty samples
        assert!(analyzer.anova_test(&[]).is_err());

        // Test single group
        let single_group = vec![vec![1.0, 2.0, 3.0]];
        assert!(analyzer.anova_test(&single_group).is_err());

        // Test empty group
        let empty_group = vec![vec![], vec![1.0, 2.0]];
        assert!(analyzer.anova_test(&empty_group).is_err());

        // Test insufficient observations
        let insufficient = vec![vec![1.0], vec![2.0]];
        assert!(analyzer.anova_test(&insufficient).is_err());
    }

    #[test]
    fn test_anova_code_quality_comparison() {
        let analyzer = StatisticalAnalyzer;

        // Simulate code quality metrics comparison between different codebases
        // Group 1: High quality code (low complexity, good structure)
        let high_quality = vec![0.1, 0.15, 0.12, 0.18, 0.14];

        // Group 2: Medium quality code
        let medium_quality = vec![0.3, 0.35, 0.28, 0.42, 0.31];

        // Group 3: Low quality code (high complexity, poor structure)
        let low_quality = vec![0.7, 0.65, 0.82, 0.58, 0.71];

        let samples = vec![high_quality, medium_quality, low_quality];
        let result = analyzer.anova_test(&samples).unwrap();

        // Should detect significant differences between quality levels
        assert!(result.f_statistic > 1.0);
        assert!(result.significant || result.p_value < 0.1); // At least marginally significant

        // Verify degrees of freedom
        assert_eq!(result.df_between, 2); // 3 groups - 1
        assert_eq!(result.df_within, 12); // 15 observations - 3 groups
    }
}
