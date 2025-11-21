//! Codex CLI - AI-Native OS Command Line Interface

use codex_core::qc::QcLogger;
use codex_core::qc::QcOrchestrator;
use codex_core::qc::TestProfile;
use codex_core::qc::WorktreeInfo;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "tui" => launch_tui(),
        "gui" => launch_gui(),
        "qc" => run_qc(&args[2..]),
        "--help" | "-h" => {
            print_help();
            Ok(())
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            Ok(())
        }
    }
}

fn run_qc(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Running QC checks...\n");

    // Parse profile argument
    let mut profile = TestProfile::default();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--profile" && i + 1 < args.len() {
            profile = args[i + 1]
                .parse()
                .map_err(|e| format!("Invalid profile: {}", e))?;
            i += 2;
        } else {
            i += 1;
        }
    }

    println!("Profile: {}", profile);

    // Detect worktree
    let worktree =
        WorktreeInfo::detect().map_err(|e| format!("Failed to detect worktree: {}", e))?;

    println!("Worktree: {}", worktree.name);
    println!("Branch: {}", worktree.branch);
    println!();

    // Get repo root (assume current directory or find .git)
    let repo_root = std::env::current_dir()?;

    // Run QC orchestrator
    let orchestrator = QcOrchestrator::new(&repo_root, profile);
    let result = orchestrator.run()?;

    // Print summary
    println!("\n📊 QC Summary:");
    println!("─────────────────────────────────────");
    println!(
        "Changed lines: +{} / -{} (Total: {})",
        result.lines_added,
        result.lines_deleted,
        result.total_changed_lines()
    );
    println!("Files changed: {}", result.files_changed);
    println!("Risk score: {:.2}", result.risk_score);
    println!("Recommendation: {}", result.recommendation);

    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("  - {}", warning);
        }
    }

    // Log the results
    let logs_dir = repo_root.join("_docs").join("logs");
    let logger = QcLogger::new(logs_dir);
    let log_path = logger
        .log(&worktree, &result)
        .map_err(|e| format!("Failed to write log: {}", e))?;

    println!("\n📝 Log written to: {}", log_path.display());

    Ok(())
}

fn launch_tui() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Terminal User Interface...");

    let tui_path = std::env::current_exe()?.parent().unwrap().join("codex-tui");

    match Command::new(tui_path).spawn() {
        Ok(mut child) => {
            println!("TUI launched successfully (PID: {})", child.id());
            let status = child.wait()?;
            println!("TUI exited with status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to launch TUI: {}", e);
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
            println!("GUI exited with status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to launch GUI: {}", e);
            eprintln!("Please ensure the GUI application is installed.");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn print_help() {
    println!("Codex AI-Native OS v2.3.0");
    println!();
    println!("USAGE:");
    println!("    codex [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!("    tui            Launch Terminal User Interface");
    println!("    gui            Launch Graphical User Interface");
    println!("    qc [OPTIONS]   Run pre-merge quality checks");
    println!("    --help         Show this help message");
    println!();
    println!("QC OPTIONS:");
    println!("    --profile <PROFILE>    Test profile to use (minimal, standard, full)");
    println!("                           Default: standard");
}
