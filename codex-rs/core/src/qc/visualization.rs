//! Quality Control Visualization
//!
//! Provides code quality visualization capabilities using:
//! - Statistical charts and graphs
//! - Optimization trend analysis
//! - Quality score dashboards
//! - Performance bottleneck visualizations

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

/// Visualization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    /// Output format (png, svg, pdf)
    pub format: String,
    /// Chart width
    pub width: u32,
    /// Chart height
    pub height: u32,
    /// Color scheme
    pub color_scheme: String,
    /// Enable interactive charts
    pub interactive: bool,
}

/// Code quality visualization generator
pub struct QualityVisualizer {
    #[allow(dead_code)]
    config: VisualizationConfig,
}

impl QualityVisualizer {
    /// Create new visualizer with default config
    pub fn new() -> Self {
        Self {
            config: VisualizationConfig {
                format: "png".to_string(),
                width: 800,
                height: 600,
                color_scheme: "cyberpunk".to_string(),
                interactive: false,
            },
        }
    }

    /// Create new visualizer with custom config
    pub fn with_config(config: VisualizationConfig) -> Self {
        Self { config }
    }

    /// Generate complexity distribution chart
    pub fn generate_complexity_chart(
        &self,
        complexity_data: &HashMap<u8, usize>,
        output_path: &str,
    ) -> Result<(), String> {
        let script = self.create_complexity_script(complexity_data, output_path);

        self.execute_python_script(&script)
    }

    /// Generate quality metrics dashboard
    pub fn generate_quality_dashboard(
        &self,
        stats: &super::statistical::CodeStatistics,
        output_path: &str,
    ) -> Result<(), String> {
        let script = self.create_dashboard_script(stats, output_path);

        self.execute_python_script(&script)
    }

    /// Generate optimization impact chart
    pub fn generate_optimization_chart(
        &self,
        optimizations: &[super::quantum::OptimizationSuggestion],
        output_path: &str,
    ) -> Result<(), String> {
        let script = self.create_optimization_script(optimizations, output_path);

        self.execute_python_script(&script)
    }

    /// Generate resource allocation visualization
    pub fn generate_resource_chart(
        &self,
        allocation: &super::mathematical::ResourceAllocation,
        output_path: &str,
    ) -> Result<(), String> {
        let script = self.create_resource_script(allocation, output_path);

        self.execute_python_script(&script)
    }

    /// Create Python script for complexity visualization
    fn create_complexity_script(&self, data: &HashMap<u8, usize>, output_path: &str) -> String {
        let mut data_str = String::new();
        for (complexity, count) in data {
            data_str.push_str(&format!("({}, {}), ", complexity, count));
        }
        if data_str.ends_with(", ") {
            data_str.truncate(data_str.len() - 2);
        }

        format!(
            r#"
import matplotlib.pyplot as plt
import seaborn as sns
from matplotlib import style

# Set cyberpunk style
plt.style.use('dark_background')
plt.rcParams['figure.facecolor'] = '#0a0a0a'
plt.rcParams['axes.facecolor'] = '#1a1a1a'
plt.rcParams['axes.edgecolor'] = '#00ff41'
plt.rcParams['axes.labelcolor'] = '#00ff41'
plt.rcParams['text.color'] = '#00ff41'
plt.rcParams['xtick.color'] = '#00ff41'
plt.rcParams['ytick.color'] = '#00ff41'

# Data
complexity_data = [{}]

if complexity_data:
    complexities, counts = zip(*complexity_data)

    # Create figure
    fig, ax = plt.subplots(figsize=(10, 6))

    # Create bar chart
    bars = ax.bar(complexities, counts, color='#00ff41', alpha=0.8, edgecolor='#ffffff', linewidth=1)

    # Add glow effect
    for bar in bars:
        bar.set_linewidth(2)
        bar.set_edgecolor('#00ff41')

    # Labels and title
    ax.set_xlabel('Cyclomatic Complexity', fontsize=12, fontweight='bold')
    ax.set_ylabel('Function Count', fontsize=12, fontweight='bold')
    ax.set_title('Code Complexity Distribution', fontsize=14, fontweight='bold', color='#00ff41')

    # Grid
    ax.grid(True, alpha=0.3, color='#333333')

    # Add value labels on bars
    for bar in bars:
        height = bar.get_height()
        ax.text(bar.get_x() + bar.get_width()/2., height + 0.1,
                f'{{:.0f}}'.format(height),
                ha='center', va='bottom', fontsize=10, color='#00ff41')

    plt.tight_layout()
    plt.savefig('{}', dpi=150, bbox_inches='tight', facecolor='#0a0a0a')
    print("Complexity chart saved to {}")
else:
    print("No complexity data available")
"#,
            data_str, output_path, output_path
        )
    }

    /// Create Python script for quality dashboard
    fn create_dashboard_script(
        &self,
        stats: &super::statistical::CodeStatistics,
        output_path: &str,
    ) -> String {
        format!(
            r#"
import matplotlib.pyplot as plt
import numpy as np

# Set cyberpunk style
plt.style.use('dark_background')
plt.rcParams['figure.facecolor'] = '#0a0a0a'
plt.rcParams['axes.facecolor'] = '#1a1a1a'
plt.rcParams['axes.edgecolor'] = '#00ff41'

# Data from Rust
total_lines = {}
code_lines = {}
function_count = {}
struct_count = {}
import_count = {}
avg_function_length = {:.1}
duplication_percentage = {:.1}
readability_score = {:.2}
maintainability_score = {:.2}
performance_score = {:.2}
security_score = {:.2}
output_file = '{}'

# Create subplots
fig, ((ax1, ax2), (ax3, ax4)) = plt.subplots(2, 2, figsize=(12, 8))
fig.suptitle('Code Quality Dashboard', fontsize=16, fontweight='bold', color='#00ff41')

# Metrics bar chart
metrics = ['Lines', 'Code Lines', 'Functions', 'Structs', 'Imports']
values = [total_lines, code_lines, function_count, struct_count, import_count]
colors = ['#00ff41', '#ff0080', '#00ffff', '#ffff00', '#ff8000']

bars = ax1.bar(metrics, values, color=colors, alpha=0.8, edgecolor='#ffffff', linewidth=1)
ax1.set_title('Code Metrics', fontsize=12, fontweight='bold', color='#00ff41')
ax1.tick_params(axis='x', rotation=45)

# Add value labels
for bar in bars:
    height = bar.get_height()
    ax1.text(bar.get_x() + bar.get_width()/2., height + max(values) * 0.02,
             f'{{:.0f}}'.format(height), ha='center', va='bottom', fontsize=9, color='#00ff41')

# Function length distribution (placeholder)
function_lengths = [10, 15, 20, 25, 30, 35, 40]  # Placeholder data
ax2.hist(function_lengths, bins=10, color='#00ff41', alpha=0.7, edgecolor='#ffffff')
ax2.set_title('Function Length Distribution', fontsize=12, fontweight='bold', color='#00ff41')
ax2.set_xlabel('Lines of Code', fontsize=10)
ax2.set_ylabel('Frequency', fontsize=10)

# Quality scores
quality_scores = ['Readability', 'Maintainability', 'Performance', 'Security']
scores = [readability_score, maintainability_score, performance_score, security_score]
ax3.barh(quality_scores, scores, color='#00ff41', alpha=0.8)
ax3.set_title('Quality Scores', fontsize=12, fontweight='bold', color='#00ff41')
ax3.set_xlim(0, 1)

# Add score labels
for i, score in enumerate(scores):
    ax3.text(score + 0.01, i, f'{{:.2f}}'.format(score), va='center', fontsize=9, color='#00ff41')

# Summary text
summary_text = f'''Total Lines: {{total_lines}}
Code Lines: {{code_lines}}
Avg Function Length: {{avg_function_length:.1f}}
Duplication: {{duplication_percentage:.1f}}%'''

ax4.text(0.1, 0.5, summary_text, transform=ax4.transAxes,
         fontsize=10, verticalalignment='center', color='#00ff41',
         bbox=dict(boxstyle="round,pad=0.3", facecolor='#1a1a1a', edgecolor='#00ff41'))
ax4.set_title('Summary', fontsize=12, fontweight='bold', color='#00ff41')
ax4.axis('off')

plt.tight_layout()
plt.savefig(output_file, dpi=150, bbox_inches='tight', facecolor='#0a0a0a')
print(f"Quality dashboard saved to {{output_file}}")
"#,
            stats.total_lines,
            stats.code_lines,
            stats.function_count,
            stats.struct_count,
            stats.import_count,
            stats.avg_function_length,
            stats.duplication_percentage,
            // Quality scores (simplified calculation)
            1.0 - (stats.duplication_percentage / 100.0).min(1.0),
            1.0 - (stats.max_function_length as f64 / 100.0).min(1.0),
            if stats.function_count > 0 { 0.8 } else { 0.5 },
            if stats.import_count < 20 { 0.9 } else { 0.7 },
            output_path
        )
    }

    /// Create Python script for optimization visualization
    fn create_optimization_script(
        &self,
        optimizations: &[super::quantum::OptimizationSuggestion],
        output_path: &str,
    ) -> String {
        let mut data_str = String::new();
        for opt in optimizations {
            data_str.push_str(&format!(
                "('{}', {:.1}, {:.2}), ",
                opt.description.chars().take(30).collect::<String>(),
                opt.improvement_percentage,
                opt.confidence
            ));
        }
        if data_str.ends_with(", ") {
            data_str.truncate(data_str.len() - 2);
        }

        format!(
            r#"
import matplotlib.pyplot as plt
import numpy as np
from matplotlib import style

# Set cyberpunk style
plt.style.use('dark_background')
plt.rcParams['figure.facecolor'] = '#0a0a0a'
plt.rcParams['axes.facecolor'] = '#1a1a1a'
plt.rcParams['axes.edgecolor'] = '#00ff41'

# Data
optimization_data = [{}]

if optimization_data:
    descriptions, improvements, confidences = zip(*optimization_data)

    # Create figure with subplots
    fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(15, 6))
    fig.suptitle('Quantum Optimization Analysis', fontsize=16, fontweight='bold', color='#00ff41')

    # Improvement percentage chart
    y_pos = np.arange(len(descriptions))
    bars1 = ax1.barh(y_pos, improvements, color='#00ff41', alpha=0.8, edgecolor='#ffffff')
    ax1.set_yticks(y_pos)
    ax1.set_yticklabels([d[:25] + '...' if len(d) > 25 else d for d in descriptions])
    ax1.set_xlabel('Expected Improvement (%)', fontsize=12, fontweight='bold', color='#00ff41')
    ax1.set_title('Performance Improvements', fontsize=14, fontweight='bold', color='#00ff41')

    # Confidence chart
    bars2 = ax2.barh(y_pos, confidences, color='#ff0080', alpha=0.8, edgecolor='#ffffff')
    ax2.set_yticks(y_pos)
    ax2.set_yticklabels([''] * len(descriptions))  # Hide labels on second chart
    ax2.set_xlabel('Confidence Score', fontsize=12, fontweight='bold', color='#00ff41')
    ax2.set_title('Optimization Confidence', fontsize=14, fontweight='bold', color='#00ff41')

    # Add value labels
    for i, (bar1, bar2) in enumerate(zip(bars1, bars2)):
        width1 = bar1.get_width()
        width2 = bar2.get_width()
        ax1.text(width1 + 0.5, bar1.get_y() + bar1.get_height()/2,
                f'{{:.1f}}%'.format(width1), ha='left', va='center', fontsize=9, color='#00ff41')
        ax2.text(width2 + 0.01, bar2.get_y() + bar2.get_height()/2,
                f'{{:.2f}}'.format(width2), ha='left', va='center', fontsize=9, color='#00ff41')

    plt.tight_layout()
    plt.savefig('{}', dpi=150, bbox_inches='tight', facecolor='#0a0a0a')
    print("Optimization chart saved to {}")
else:
    print("No optimization data available")
"#,
            data_str, output_path, output_path
        )
    }

    /// Create Python script for resource allocation visualization
    fn create_resource_script(
        &self,
        allocation: &super::mathematical::ResourceAllocation,
        output_path: &str,
    ) -> String {
        format!(
            r#"
import matplotlib.pyplot as plt
from matplotlib import style

# Set cyberpunk style
plt.style.use('dark_background')
plt.rcParams['figure.facecolor'] = '#0a0a0a'
plt.rcParams['axes.facecolor'] = '#1a1a1a'
plt.rcParams['axes.edgecolor'] = '#00ff41'

# Data
resources = ['CPU Cores', 'Memory (MB)', 'Disk (GB)']
allocated = [{}, {}, {}]
estimated_time = {}

# Create figure
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(12, 5))
fig.suptitle('Resource Allocation Optimization', fontsize=16, fontweight='bold', color='#00ff41')

# Resource allocation pie chart
colors = ['#00ff41', '#ff0080', '#00ffff']
ax1.pie(allocated, labels=resources, autopct='%1.1f%%', colors=colors, startangle=90)
ax1.set_title('Resource Distribution', fontsize=14, fontweight='bold', color='#00ff41')

# Time estimation gauge (simplified bar)
ax2.bar(['Estimated Time'], [estimated_time], color='#00ff41', alpha=0.8, width=0.5)
ax2.set_ylabel('Time (seconds)', fontsize=12, fontweight='bold', color='#00ff41')
ax2.set_title('Execution Time Estimate', fontsize=14, fontweight='bold', color='#00ff41')

# Add time label
ax2.text(0, estimated_time + estimated_time * 0.05,
         f'{{}}s'.format(estimated_time), ha='center', va='bottom',
         fontsize=12, fontweight='bold', color='#00ff41')

plt.tight_layout()
plt.savefig('{}', dpi=150, bbox_inches='tight', facecolor='#0a0a0a')
print("Resource allocation chart saved to {}")
"#,
            allocation.allocated_cpu,
            allocation.allocated_memory,
            allocation.allocated_disk,
            allocation.estimated_time,
            output_path,
            output_path
        )
    }

    /// Execute Python script for visualization
    fn execute_python_script(&self, script: &str) -> Result<(), String> {
        // Write script to temporary file
        let script_path = "temp_viz_script.py";
        fs::write(script_path, script).map_err(|e| format!("Failed to write script: {}", e))?;

        // Execute Python script
        let output = Command::new("python")
            .arg(script_path)
            .output()
            .map_err(|e| format!("Failed to execute Python script: {}", e))?;

        // Clean up temporary file
        let _ = fs::remove_file(script_path);

        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Python script failed: {}", stderr))
        }
    }
}

impl Default for QualityVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_visualizer_creation() {
        let visualizer = QualityVisualizer::new();
        assert_eq!(visualizer.config.format, "png");
    }

    #[test]
    fn test_complexity_script_generation() {
        let visualizer = QualityVisualizer::new();
        let mut data = HashMap::new();
        data.insert(1u8, 5usize);
        data.insert(2u8, 3usize);

        let script = visualizer.create_complexity_script(&data, "test.png");
        assert!(script.contains("complexity_data"));
        assert!(script.contains("test.png"));
    }
}
