use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use clap::Args;
use codex_core::qc::QcAgent;
use codex_core::qc::QcConfig;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

const MAX_DEPTH: usize = 6;

/// Run Quality Control analysis for a file or directory.
#[derive(Debug, Args)]
pub struct QcCli {
    /// Target file or directory
    #[clap(long, default_value = ".")]
    pub path: PathBuf,

    /// Output directory for QC reports
    #[clap(long, default_value = "qc_reports")]
    pub output_dir: String,

    /// Disable visualization outputs
    #[clap(long, default_value_t = false)]
    pub no_visualization: bool,

    /// Disable statistical analysis
    #[clap(long, default_value_t = false)]
    pub no_statistical: bool,

    /// Disable quantum optimization analysis
    #[clap(long, default_value_t = false)]
    pub no_quantum: bool,

    /// Disable mathematical optimization analysis
    #[clap(long, default_value_t = false)]
    pub no_mathematical: bool,

    /// Minimum confidence threshold for recommendations
    #[clap(long, default_value_t = 0.6)]
    pub min_confidence: f64,

    /// Enable verbose QC logging
    #[clap(long, default_value_t = false)]
    pub verbose: bool,
}

pub async fn run_qc_command(cli: QcCli) -> Result<()> {
    let resolved_path = if cli.path.is_absolute() {
        cli.path
    } else {
        std::env::current_dir()?.join(cli.path)
    };

    let (source, target_name, file_count) =
        load_source(&resolved_path).context("failed to load source for QC analysis")?;

    if file_count == 0 {
        let display = resolved_path.display();
        anyhow::bail!("No source files found under {display}");
    }

    let mut config = QcConfig::default();
    config.output_dir = cli.output_dir;
    config.enable_visualization = !cli.no_visualization;
    config.enable_statistical = !cli.no_statistical;
    config.enable_quantum = !cli.no_quantum;
    config.enable_mathematical = !cli.no_mathematical;
    config.min_confidence = cli.min_confidence;
    config.verbose = cli.verbose;

    let qc_agent = QcAgent::with_config(config);
    let report = qc_agent
        .analyze(&source, &target_name)
        .await
        .map_err(Error::msg)
        .context("QC analysis failed")?;

    let overall = report.scores.overall;
    let readability = report.scores.readability;
    let maintainability = report.scores.maintainability;
    let performance = report.scores.performance;
    let security = report.scores.security;

    println!("QC analysis completed for {}", report.target);
    println!("Files analyzed: {file_count}");
    println!("Overall score: {overall:.3}");
    println!("Readability: {readability:.3}");
    println!("Maintainability: {maintainability:.3}");
    println!("Performance: {performance:.3}");
    println!("Security: {security:.3}");

    if !report.recommendations.is_empty() {
        println!();
        println!("Recommendations:");
        for recommendation in &report.recommendations {
            println!("- {recommendation}");
        }
    }

    if !report.outputs.is_empty() {
        println!();
        println!("Artifacts:");
        for output in &report.outputs {
            println!("- {output}");
        }
    }

    Ok(())
}

fn load_source(path: &Path) -> Result<(String, String, usize)> {
    if path.is_file() {
        let content = std::fs::read_to_string(path).with_context(|| {
            let display = path.display();
            format!("failed to read {display}")
        })?;
        let target_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("file")
            .to_string();
        return Ok((content, target_name, 1));
    }

    let mut source_content = String::new();
    let mut file_count = 0;
    let target_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("workspace")
        .to_string();

    for entry in WalkDir::new(path)
        .max_depth(MAX_DEPTH)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        let entry_path = entry.path();
        if entry_path.is_dir() && is_ignored_dir(entry_path) {
            continue;
        }
        if !entry_path.is_file() {
            continue;
        }
        if !is_source_file(entry_path) {
            continue;
        }

        let content = std::fs::read_to_string(entry_path).with_context(|| {
            let display = entry_path.display();
            format!("failed to read {display}")
        })?;
        let display = entry_path.display();
        source_content.push_str(&format!("\n// File: {display}\n"));
        source_content.push_str(&content);
        source_content.push('\n');
        file_count += 1;
    }

    Ok((source_content, target_name, file_count))
}

fn is_ignored_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, ".git" | "target" | "node_modules" | "dist" | "build"))
}

fn is_source_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| {
            matches!(
                ext,
                "rs" | "ts"
                    | "tsx"
                    | "js"
                    | "jsx"
                    | "py"
                    | "go"
                    | "java"
                    | "cpp"
                    | "c"
                    | "h"
                    | "hpp"
                    | "cs"
                    | "kt"
                    | "swift"
                    | "rb"
                    | "php"
                    | "scala"
                    | "clj"
                    | "md"
            )
        })
}
