//! Parallel agent delegation command.

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use codex_core::AuthManager;
use codex_core::agents::AgentAliases;
use codex_core::agents::AgentRuntime;
use codex_core::agents::AgentStatus;
use codex_core::auth::CODEX_API_KEY_ENV_VAR;
use codex_core::auth::OPENAI_API_KEY_ENV_VAR;
use codex_core::config::Config;
use codex_core::protocol::SessionSource;
use codex_core::terminal;
use codex_otel::otel_event_manager::OtelEventManager;
use codex_protocol::ThreadId;
use codex_utils_cli::CliConfigOverrides;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::resolve_runtime_budget;

const DEFAULT_SUBAGENT_RUNTIME_BUDGET: i64 = 200_000;

fn resolve_agent_name(raw: &str, aliases: &AgentAliases) -> String {
    let trimmed = raw.trim();
    let lower = trimmed.to_ascii_lowercase();
    match lower.as_str() {
        "backend" => "executor".to_string(),
        "qa" => "code-reviewer".to_string(),
        "milspec" => "sec-audit".to_string(),
        "gui" => "ts-reviewer".to_string(),
        _ => aliases.resolve(trimmed),
    }
}

/// Run the parallel delegate command.
pub async fn run_parallel_delegate_command(
    agents: Vec<String>,
    goals: Vec<String>,
    scopes: Vec<Option<PathBuf>>,
    budgets: Vec<Option<usize>>,
    deadline: Option<u64>,
    out: Option<PathBuf>,
    config_overrides: CliConfigOverrides,
) -> Result<()> {
    if agents.is_empty() {
        bail!("No agents specified");
    }

    if !goals.is_empty() && goals.len() != agents.len() {
        bail!(
            "Number of goals ({}) must match number of agents ({})",
            goals.len(),
            agents.len()
        );
    }

    let aliases = AgentAliases::load().unwrap_or_else(|err| {
        eprintln!("Warning: failed to load aliases.yaml, using defaults: {err}");
        AgentAliases::default()
    });

    let requested_agents = agents;
    let resolved_agents: Vec<String> = requested_agents
        .iter()
        .map(|agent| resolve_agent_name(agent, &aliases))
        .collect();

    let cli_overrides = config_overrides
        .parse_overrides()
        .map_err(|err| anyhow!("failed to parse -c overrides: {err}"))?;

    let config = Config::load_with_cli_overrides(cli_overrides)
        .await
        .context("failed to load configuration")?;
    let config = Arc::new(config);

    let workspace_dir = config.cwd.clone();

    let auth_manager = AuthManager::shared(
        config.codex_home.clone(),
        true,
        config.cli_auth_credentials_store_mode,
    );
    let auth_snapshot = auth_manager.auth().await;

    if config.model_provider.requires_openai_auth
        && auth_snapshot.is_none()
        && std::env::var(OPENAI_API_KEY_ENV_VAR).is_err()
        && std::env::var(CODEX_API_KEY_ENV_VAR).is_err()
    {
        bail!(
            "No authentication credentials found. Run `codex login` or set the {OPENAI_API_KEY_ENV_VAR} environment variable."
        );
    }

    let conversation_id = ThreadId::default();
    let model = config
        .model
        .as_deref()
        .unwrap_or(config.review_model.as_deref().unwrap_or("gpt-4o"));
    let otel_manager = OtelEventManager::new(
        conversation_id,
        model,
        model,
        auth_snapshot
            .as_ref()
            .and_then(|auth| auth.get_account_id()),
        auth_snapshot
            .as_ref()
            .and_then(|auth| auth.get_account_email()),
        auth_snapshot.as_ref().map(|auth| auth.auth_mode().into()),
        String::from("codex-cli"),
        config.otel.log_user_prompt,
        terminal::user_agent(),
        SessionSource::Cli,
    );

    let runtime_budget = resolve_runtime_budget(&config, DEFAULT_SUBAGENT_RUNTIME_BUDGET);

    let runtime = AgentRuntime::new(
        workspace_dir.clone(),
        runtime_budget,
        Arc::clone(&config),
        Some(Arc::clone(&auth_manager)),
        otel_manager,
        config.model_provider.clone(),
        conversation_id,
        config.model_reasoning_effort.unwrap_or_default(),
        config.model_reasoning_summary,
        config.model_verbosity.unwrap_or_default(),
    );

    println!("Starting parallel delegation...");
    println!("Requested agents: {:?}", requested_agents);
    println!("Resolved agents:  {:?}", resolved_agents);
    if let Some(minutes) = deadline {
        println!("Deadline: {minutes} minutes");
    }
    println!();

    let mut agent_configs = Vec::new();
    for (i, resolved_agent_name) in resolved_agents.iter().enumerate() {
        let goal = goals
            .get(i)
            .cloned()
            .unwrap_or_else(|| "Complete task".to_string());

        let resolved_scope = scopes.get(i).and_then(|opt_scope| {
            opt_scope.as_ref().map(|path| {
                if path.is_absolute() {
                    path.clone()
                } else {
                    workspace_dir.join(path)
                }
            })
        });

        let budget = budgets.get(i).and_then(|opt_budget| *opt_budget);

        let mut inputs = HashMap::new();
        inputs.insert("goal".to_string(), goal.clone());
        inputs.insert("workspace".to_string(), workspace_dir.display().to_string());
        if let Some(ref path) = resolved_scope {
            inputs.insert("scope".to_string(), path.display().to_string());
        }

        println!(
            "Agent {}/{}: {} -> {}",
            i + 1,
            resolved_agents.len(),
            requested_agents[i],
            resolved_agent_name
        );
        println!("  Goal: {goal}");
        if let Some(ref path) = resolved_scope {
            println!("  Scope: {}", path.display());
        }
        if let Some(b) = budget {
            println!("  Budget: {b} tokens");
        }
        println!();

        agent_configs.push((resolved_agent_name.clone(), goal, inputs, budget));
    }

    println!("Executing {} agents in parallel...", resolved_agents.len());
    println!();

    let results = runtime
        .delegate_parallel(agent_configs, deadline)
        .await
        .context("parallel agent execution failed")?;

    println!("\nExecution results:");
    let mut success_count = 0;
    for (i, result) in results.iter().enumerate() {
        println!(
            "\n  Agent {}/{}: {} -> {}",
            i + 1,
            results.len(),
            requested_agents[i],
            resolved_agents[i]
        );
        println!("    Status: {:?}", result.status);
        println!("    Tokens used: {}", result.tokens_used);
        println!("    Duration: {:.2}s", result.duration_secs);

        if result.status == AgentStatus::Completed {
            success_count += 1;
        }

        if !result.artifacts.is_empty() {
            println!("    Artifacts:");
            for artifact in &result.artifacts {
                println!("      - {artifact}");
            }
        }

        if let Some(ref error) = result.error {
            eprintln!("    Error: {error}");
        }
    }

    println!("\nParallel delegation completed.");
    println!("Success: {}/{}", success_count, resolved_agents.len());

    if let Some(out_file) = out {
        let report = serde_json::json!({
            "requested_agents": requested_agents,
            "resolved_agents": resolved_agents,
            "results": results,
            "success_count": success_count,
            "total_count": results.len(),
        });
        std::fs::write(&out_file, serde_json::to_string_pretty(&report)?)
            .context("failed to write results")?;
        println!("\nResults saved to: {}", out_file.display());
    }

    Ok(())
}

