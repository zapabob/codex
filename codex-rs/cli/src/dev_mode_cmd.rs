//! Development Mode Commands
//!
//! CLI commands for managing centralized and parallel development modes

use clap::Args;
use clap::Subcommand;
use codex_core::mcp_integration_manager::McpIntegrationManager;
use codex_core::ai_orchestrator::{AIOrchestrator, DevelopmentMode};
use std::collections::HashMap;

/// Development mode CLI commands
#[derive(Debug, Args)]
pub struct DevModeCli {
    #[clap(subcommand)]
    pub command: DevModeCommand,
}

/// Development mode subcommands
#[derive(Debug, Subcommand)]
pub enum DevModeCommand {
    /// Start centralized development mode
    Central {
        /// Task description
        #[clap(long)]
        task: Option<String>,
        /// Target agents (comma-separated)
        #[clap(long, value_delimiter = ',')]
        agents: Option<Vec<String>>,
    },
    /// Start parallel development mode
    Parallel {
        /// Task description
        #[clap(long)]
        task: Option<String>,
        /// Target agents (comma-separated)
        #[clap(long, value_delimiter = ',')]
        agents: Option<Vec<String>>,
        /// Git worktree base path
        #[clap(long)]
        worktree_base: Option<String>,
    },
    /// Show current development mode status
    Status,
    /// Stop current development mode
    Stop,
    /// List available MCP servers
    ListServers,
    /// Start specific MCP servers
    StartServers {
        /// Server names (comma-separated)
        #[clap(value_delimiter = ',')]
        servers: Vec<String>,
    },
    /// Stop specific MCP servers
    StopServers {
        /// Server names (comma-separated)
        #[clap(value_delimiter = ',')]
        servers: Vec<String>,
    },
}

/// Run development mode commands
pub async fn run_dev_mode_command(cli: DevModeCli) -> anyhow::Result<()> {
    match cli.command {
        DevModeCommand::Central { task, agents } => {
            run_centralized_dev(task, agents).await
        }
        DevModeCommand::Parallel { task, agents, worktree_base } => {
            run_parallel_dev(task, agents, worktree_base).await
        }
        DevModeCommand::Status => {
            show_dev_status().await
        }
        DevModeCommand::Stop => {
            stop_dev_mode().await
        }
        DevModeCommand::ListServers => {
            list_mcp_servers().await
        }
        DevModeCommand::StartServers { servers } => {
            start_mcp_servers(servers).await
        }
        DevModeCommand::StopServers { servers } => {
            stop_mcp_servers(servers).await
        }
    }
}

/// Run centralized development mode
async fn run_centralized_dev(
    task: Option<String>,
    agents: Option<Vec<String>>
) -> anyhow::Result<()> {
    println!("🚀 Starting centralized development mode...");

    // Initialize orchestrator
    let orchestrator = AIOrchestrator::with_mode(DevelopmentMode::Centralized);

    // Initialize MCP manager and start servers
    let mcp_manager = McpIntegrationManager::new();
    mcp_manager.start_servers_for_mode("centralized").await?;

    println!("✅ Centralized development mode initialized");
    println!("📋 Active MCP servers: serena, arxiv, youtube, gemini-cli");

    if let Some(task_desc) = task {
        println!("🎯 Task: {}", task_desc);

        // Create orchestrated task
        let task = codex_core::ai_orchestrator::OrchestratedTask {
            id: format!("central-{}", chrono::Utc::now().timestamp()),
            description: task_desc,
            priority: codex_core::ai_orchestrator::TaskPriority::High,
            dependencies: vec![],
            assigned_agent: None,
            status: codex_core::ai_orchestrator::TaskStatus::Pending,
            created_at: chrono::Utc::now(),
            estimated_complexity: 5.0,
            tags: agents.unwrap_or_else(|| vec![
                "architect".to_string(),
                "code-reviewer".to_string(),
                "researcher".to_string(),
            ]),
        };

        let task_id = orchestrator.submit_task(task).await?;
        println!("📝 Task submitted with ID: {}", task_id);
    }

    // Keep running until interrupted
    println!("🔄 Press Ctrl+C to stop...");
    tokio::signal::ctrl_c().await?;

    Ok(())
}

/// Run parallel development mode
async fn run_parallel_dev(
    task: Option<String>,
    agents: Option<Vec<String>>,
    worktree_base: Option<String>
) -> anyhow::Result<()> {
    println!("🚀 Starting parallel development mode...");

    // Initialize orchestrator
    let orchestrator = AIOrchestrator::with_mode(DevelopmentMode::Parallel);

    // Initialize MCP manager and start servers
    let mcp_manager = McpIntegrationManager::new();
    mcp_manager.start_servers_for_mode("parallel").await?;

    println!("✅ Parallel development mode initialized");
    println!("📋 Active MCP servers: serena, github, git-enhanced, filesystem, playwright");

    if let Some(task_desc) = task {
        println!("🎯 Task: {}", task_desc);
        println!("📁 Worktree base: {}", worktree_base.unwrap_or_else(|| ".codex-worktrees".to_string()));

        // Create orchestrated task for parallel execution
        let task = codex_core::ai_orchestrator::OrchestratedTask {
            id: format!("parallel-{}", chrono::Utc::now().timestamp()),
            description: task_desc,
            priority: codex_core::ai_orchestrator::TaskPriority::High,
            dependencies: vec![],
            assigned_agent: None,
            status: codex_core::ai_orchestrator::TaskStatus::Pending,
            created_at: chrono::Utc::now(),
            estimated_complexity: 7.0,
            tags: agents.unwrap_or_else(|| vec![
                "architect".to_string(),
                "code-reviewer".to_string(),
                "researcher".to_string(),
                "qc-optimizer".to_string(),
            ]),
        };

        let task_id = orchestrator.submit_task(task).await?;
        println!("📝 Task submitted with ID: {}", task_id);
    }

    // Keep running until interrupted
    println!("🔄 Press Ctrl+C to stop...");
    tokio::signal::ctrl_c().await?;

    Ok(())
}

/// Show development mode status
async fn show_dev_status() -> anyhow::Result<()> {
    let mcp_manager = McpIntegrationManager::new();

    println!("📊 Development Mode Status");
    println!("==========================");

    let active_servers = mcp_manager.get_active_servers().await;
    if active_servers.is_empty() {
        println!("❌ No active MCP servers");
    } else {
        println!("✅ Active MCP servers:");
        for (name, server) in active_servers {
            println!("  • {}: {}", name, server.capabilities.join(", "));
        }
    }

    // TODO: Show orchestrator status when available
    println!("\n🤖 Orchestrator: Not implemented yet");

    Ok(())
}

/// Stop current development mode
async fn stop_dev_mode() -> anyhow::Result<()> {
    println!("🛑 Stopping development mode...");

    let mcp_manager = McpIntegrationManager::new();

    // Stop all active servers
    let active_servers = mcp_manager.get_active_servers().await;
    for server_name in active_servers.keys() {
        mcp_manager.stop_server(server_name).await?;
        println!("✅ Stopped MCP server: {}", server_name);
    }

    println!("✅ Development mode stopped");

    Ok(())
}

/// List available MCP servers
async fn list_mcp_servers() -> anyhow::Result<()> {
    let mcp_manager = McpIntegrationManager::new();
    let config = mcp_manager.load_config().await?;

    println!("🔧 Available MCP Servers");
    println!("========================");

    for (name, server) in &config.servers {
        let status = if mcp_manager.is_server_running(name).await {
            "🟢 Running"
        } else {
            "🔴 Stopped"
        };

        println!("{} {}: {}", status, name, server.capabilities.join(", "));
        println!("  Command: {} {}", server.command, server.args.join(" "));
        println!();
    }

    Ok(())
}

/// Start specific MCP servers
async fn start_mcp_servers(servers: Vec<String>) -> anyhow::Result<()> {
    let mcp_manager = McpIntegrationManager::new();

    for server_name in servers {
        if mcp_manager.is_server_running(&server_name).await {
            println!("⚠️  MCP server '{}' is already running", server_name);
            continue;
        }

        match mcp_manager.start_server(&server_name).await {
            Ok(_) => println!("✅ Started MCP server: {}", server_name),
            Err(e) => println!("❌ Failed to start MCP server '{}': {}", server_name, e),
        }
    }

    Ok(())
}

/// Stop specific MCP servers
async fn stop_mcp_servers(servers: Vec<String>) -> anyhow::Result<()> {
    let mcp_manager = McpIntegrationManager::new();

    for server_name in servers {
        if !mcp_manager.is_server_running(&server_name).await {
            println!("⚠️  MCP server '{}' is not running", server_name);
            continue;
        }

        match mcp_manager.stop_server(&server_name).await {
            Ok(_) => println!("✅ Stopped MCP server: {}", server_name),
            Err(e) => println!("❌ Failed to stop MCP server '{}': {}", server_name, e),
        }
    }

    Ok(())
}
