use anyhow::Context;
use anyhow::Result;
use codex_common::CliConfigOverrides;
use codex_core::agent_interpreter::AgentInterpreter;
use codex_core::agents::AgentAliases;
use std::path::PathBuf;

pub async fn run_ask_command(
    config_overrides: CliConfigOverrides,
    prompt: String,
    scope: Option<PathBuf>,
    budget: Option<usize>,
    out: Option<PathBuf>,
) -> Result<()> {
    // Load aliases
    let aliases = AgentAliases::load().unwrap_or_default();

    // Check if prompt starts with @mention
    let (agent_name, task) = if AgentAliases::has_mention(&prompt) {
        let (agent, rest) =
            AgentAliases::extract_mention(&prompt).context("Failed to parse @mention")?;
        let resolved = aliases.resolve(agent);
        (resolved, rest.to_string())
    } else {
        // Default to researcher if no @mention
        ("researcher".to_string(), prompt.clone())
    };

    println!("🤖 Using agent: {agent_name}");
    println!("📝 Task: {task}\n");

    // Use the existing delegate logic
    crate::delegate_cmd::run_delegate_command(
        config_overrides,
        agent_name,
        Some(task),
        scope,
        budget,
        None, // deadline
        out,
    )
    .await
}

/// Natural language agent command with AI-powered agent selection
pub async fn run_natural_language_agent(
    config_overrides: CliConfigOverrides,
    prompt: String,
    scope: Option<PathBuf>,
    budget: Option<usize>,
    out: Option<PathBuf>,
) -> Result<()> {
    let interpreter = AgentInterpreter::new();
    let invocation = interpreter
        .parse(&prompt)
        .context("Failed to interpret natural language command")?;

    println!("🧠 Interpreted command:");
    println!("   Agent: {}", invocation.agent_name);
    println!("   Confidence: {:.0}%", invocation.confidence * 100.0);
    if !invocation.parameters.is_empty() {
        println!("   Parameters:");
        for (key, value) in &invocation.parameters {
            println!("     {}: {}", key, value);
        }
    }
    println!("   Task: {}\n", invocation.goal);

    crate::delegate_cmd::run_delegate_command(
        config_overrides,
        invocation.agent_name,
        Some(invocation.goal),
        scope,
        budget,
        None, // deadline
        out,
    )
    .await
}

/// Shortcut command that automatically selects the appropriate agent
pub async fn run_shortcut_command(
    config_overrides: CliConfigOverrides,
    shortcut: &str,
    prompt: String,
    scope: Option<PathBuf>,
    budget: Option<usize>,
    out: Option<PathBuf>,
) -> Result<()> {
    let aliases = AgentAliases::load().unwrap_or_default();
    let agent_name = aliases.resolve(shortcut);

    println!("🚀 Shortcut: {shortcut} → {agent_name}");
    println!("📝 Task: {prompt}\n");

    crate::delegate_cmd::run_delegate_command(
        config_overrides,
        agent_name,
        Some(prompt),
        scope,
        budget,
        None,
        out,
    )
    .await
}
