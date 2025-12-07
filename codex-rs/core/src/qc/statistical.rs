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

/// Statistical QC analyzer
pub struct StatisticalAnalyzer;

impl StatisticalAnalyzer {
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
}
