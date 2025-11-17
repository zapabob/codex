//! Codex CLI - AI-Native OS Command Line Interface

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

fn launch_tui() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching Terminal User Interface...");
    
    let tui_path = std::env::current_exe()?
        .parent()
        .unwrap()
        .join("codex-tui");

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
    println!("");
    println!("USAGE:");
    println!("    codex [COMMAND]");
    println!("");
    println!("COMMANDS:");
    println!("    tui     Launch Terminal User Interface");
    println!("    gui     Launch Graphical User Interface");
    println!("    --help  Show this help message");
}
