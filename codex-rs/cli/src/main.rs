//! Codex CLI - AI-Native OS Command Line Interface

use clap::{Parser, Subcommand};
use codex_core::qc_orchestrator::{QcConfig, QcInput, TestProfile};
use std::process::Command;

#[derive(Parser)]
#[command(name = "codex")]
#[command(about = "Codex AI-Native OS v2.3.0", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch Terminal User Interface
    Tui,
    /// Launch Graphical User Interface
    Gui,
    /// Run QC (Quality Control) orchestrator
    #[command(name = "qc")]
    Qc {
        /// Feature description
        #[arg(long)]
        feature: Option<String>,

        /// Test profile: minimal, standard, or full
        #[arg(long)]
        profile: Option<String>,

        /// Agent name
        #[arg(long, default_value = "codex-cli-agent")]
        agent_name: String,

        /// AI model name
        #[arg(long, default_value = "claude-code")]
        ai_name: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tui => launch_tui(),
        Commands::Gui => launch_gui(),
        Commands::Qc {
            feature,
            profile,
            agent_name,
            ai_name,
        } => run_qc_command(feature, profile, agent_name, ai_name),
    }
}

fn run_qc_command(
    feature: Option<String>,
    profile: Option<String>,
    agent_name: String,
    ai_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get current working directory as repo root
    let repo_root = std::env::current_dir()?;

    // Load config (use defaults for now)
    let config = QcConfig::default();

    // Parse profile
    let test_profile = if let Some(profile_str) = profile {
        TestProfile::from_str(&profile_str)?
    } else {
        config.default_profile
    };

    // Build input
    let input = QcInput {
        feature: feature.unwrap_or_else(|| "No description provided".to_string()),
        agent_name,
        ai_name,
        profile: test_profile,
    };

    println!("🔍 Running QC orchestrator...");
    println!("   Profile: {}", input.profile.as_str());
    println!("   Repository: {}", repo_root.display());
    println!();

    // Run QC
    let result = codex_core::qc_orchestrator::run_qc(&repo_root, input, config)?;

    // Print summary
    println!("📊 QC Summary");
    println!("─────────────────────────────────────────");
    println!("Timestamp:      {}", result.timestamp);
    println!("Worktree:       {}", result.worktree);
    println!();
    println!("Changed Files:  {}", result.diff.changed_files);
    println!("Changed Lines:  {}", result.diff.changed_lines);
    println!();
    println!("Risk Score:     {:.2}", result.risk_score);
    println!("Recommendation: {}", result.recommendation.as_str());
    println!();

    if !result.reasons.is_empty() {
        println!("Reasons:");
        for reason in &result.reasons {
            println!("  • {reason}");
        }
        println!();
    }

    if !result.issues.is_empty() {
        println!("Issues Found:");
        for issue in &result.issues {
            println!("  ✗ {issue}");
        }
        println!();
    }

    println!("Test Results:");
    for test in &result.tests {
        let status_icon = match &test.status {
            codex_core::qc_orchestrator::CommandStatus::Passed => "✓",
            codex_core::qc_orchestrator::CommandStatus::Failed { .. } => "✗",
            codex_core::qc_orchestrator::CommandStatus::NotRun { .. } => "⊘",
        };
        println!("  {status_icon} {}", test.label);
    }
    println!();

    println!("Log written to: {}", result.log_path.display());

    Ok(())
}

fn launch_tui() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Terminal User Interface...");

    let tui_path = std::env::current_exe()?.parent().unwrap().join("codex-tui");

    match Command::new(tui_path).spawn() {
        Ok(mut child) => {
            println!("TUI launched successfully (PID: {})", child.id());
            let status = child.wait()?;
            println!("TUI exited with status: {status}");
        }
        Err(e) => {
            eprintln!("Failed to launch TUI: {e}");
            eprintln!("Please ensure the TUI application is installed.");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn launch_gui() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Graphical User Interface...");

    let gui_path = std::env::current_exe()?
        .parent()
        .unwrap()
        .join("codex-tauri-gui");

    match Command::new(gui_path).spawn() {
        Ok(mut child) => {
            println!("GUI launched successfully (PID: {})", child.id());
            let status = child.wait()?;
            println!("GUI exited with status: {status}");
        }
        Err(e) => {
            eprintln!("Failed to launch GUI: {e}");
            eprintln!("Please ensure the GUI application is installed.");
            std::process::exit(1);
        }
    }

    Ok(())
}
