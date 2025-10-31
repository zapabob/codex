//! Lock management commands

use anyhow::Result;
use clap::Parser;
use codex_core::lock::RepositoryLock;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct LockCli {
    #[clap(subcommand)]
    pub command: LockCommand,
}

#[derive(Debug, Parser)]
pub enum LockCommand {
    /// Show current lock status
    Status(LockStatusCommand),
    
    /// Remove the lock (requires --force if held by another process)
    Remove(LockRemoveCommand),
}

#[derive(Debug, Parser)]
pub struct LockStatusCommand {
    /// Repository path (defaults to current directory)
    #[arg(short, long, value_name = "PATH")]
    pub repo: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub struct LockRemoveCommand {
    /// Repository path (defaults to current directory)
    #[arg(short, long, value_name = "PATH")]
    pub repo: Option<PathBuf>,
    
    /// Force remove the lock even if held by another process
    #[arg(short, long)]
    pub force: bool,
}

pub fn run_lock_status(cmd: LockStatusCommand) -> Result<()> {
    let repo_path = cmd.repo.unwrap_or_else(|| PathBuf::from("."));
    let lock = RepositoryLock::new(&repo_path)?;
    
    match lock.status()? {
        Some(metadata) => {
            println!("Lock Status: LOCKED");
            println!("  Version: {}", metadata.version);
            println!("  PID: {}", metadata.pid);
            if let Some(ppid) = metadata.ppid {
                println!("  Parent PID: {}", ppid);
            }
            #[cfg(unix)]
            if let Some(uid) = metadata.uid {
                println!("  UID: {}", uid);
            }
            if let Some(hostname) = &metadata.hostname {
                println!("  Hostname: {}", hostname);
            }
            println!("  Repository: {}", metadata.repo_path);
            println!("  Started At: {} (Unix timestamp)", metadata.started_at);
            if let Some(expires_at) = metadata.expires_at {
                println!("  Expires At: {} (Unix timestamp)", expires_at);
            }
            
            // Check if lock is stale
            if !lock.is_locked() {
                eprintln!("\nWarning: Lock appears stale (process not alive or expired)");
            }
        }
        None => {
            println!("Lock Status: UNLOCKED");
        }
    }
    
    Ok(())
}

pub fn run_lock_remove(cmd: LockRemoveCommand) -> Result<()> {
    let repo_path = cmd.repo.unwrap_or_else(|| PathBuf::from("."));
    let lock = RepositoryLock::new(&repo_path)?;
    
    if cmd.force {
        lock.force_remove()?;
        println!("Lock forcibly removed");
    } else {
        lock.release()?;
        println!("Lock released");
    }
    
    Ok(())
}
