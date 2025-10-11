// Delegate command - temporarily disabled
// TODO: Update to new AgentRuntime API

use anyhow::Result;
use std::path::PathBuf;

pub async fn run_delegate_command(
    _agent: String,
    _goal: Option<String>,
    _scope: Option<PathBuf>,
    _budget: Option<usize>,
    _deadline: Option<u64>,
    _out: Option<PathBuf>,
) -> Result<()> {
    eprintln!("Delegate command is currently under maintenance.");
    eprintln!("Please use 'codex supervisor' command instead.");
    std::process::exit(1);
}
