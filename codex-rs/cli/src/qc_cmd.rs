use anyhow::Context;
use anyhow::Result;
use codex_core::qc::QcAgent;
use codex_core::qc::QcConfig;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;

const MAX_DEPTH: usize = 6;

pub async fn run_qc_command(
    path: PathBuf,
    output_dir: String,
    no_visualization: bool,
) -> Result<()> {
    let resolved_path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };

    let (source, target_name, file_count) =
        load_source(&resolved_path).context("failed to load source for QC analysis")?;

    if file_count == 0 {
        anyhow::bail!("No source files found under {}", resolved_path.display());
    }

    let mut config = QcConfig::default();
    config.output_dir = output_dir;
    config.enable_visualization = !no_visualization;

    let qc_agent = QcAgent::with_config(config);
    let report = qc_agent
        .analyze(&source, &target_name)
        .await
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
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
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
        .filter_map(Result::ok)
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

        let content = std::fs::read_to_string(entry_path)
            .with_context(|| format!("failed to read {}", entry_path.display()))?;
        source_content.push_str(&format!("\n// File: {}\n", entry_path.display()));
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
                "rs"
                    | "ts"
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
