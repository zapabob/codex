//! Custom agent creation command

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use codex_common::CliConfigOverrides;
use codex_core::agents::AgentRuntime;
use codex_core::agents::AgentStatus;
use codex_core::auth::CODEX_API_KEY_ENV_VAR;
use codex_core::auth::OPENAI_API_KEY_ENV_VAR;
use codex_core::config::Config;
use codex_core::config::ConfigOverrides;
use codex_core::terminal;
use codex_core::AuthManager;
use codex_otel::otel_event_manager::OtelEventManager;
use codex_protocol::ConversationId;
use std::path::PathBuf;
use std::sync::Arc;

const DEFAULT_SUBAGENT_RUNTIME_BUDGET: u64 = 200_000;

/// Run the agent create command (custom agent from prompt)
pub async fn run_agent_create_command(
    prompt: String,
    budget: Option<usize>,
    save: bool,
    out: Option<PathBuf>,
    config_overrides: CliConfigOverrides,
) -> Result<()> {
    println!("🤖 Creating custom agent from prompt...");
    println!("   Prompt: {}", prompt);
    if let Some(budget) = budget {
        println!("   Budget: {} tokens", budget);
    }
    println!();

    // 設定読み込み
    let cli_overrides = config_overrides
        .parse_overrides()
        .map_err(|err| anyhow!("failed to parse -c overrides: {err}"))?;

    let config = Config::load_with_cli_overrides(cli_overrides, ConfigOverrides::default())
        .await
        .context("failed to load configuration")?;
    let config = Arc::new(config);

    let workspace_dir = config.cwd.clone();

    // 認証確認
    let auth_manager = AuthManager::shared(config.codex_home.clone(), true);
    let auth_snapshot = auth_manager.auth();

    if config.model_provider.requires_openai_auth
        && auth_snapshot.is_none()
        && std::env::var(OPENAI_API_KEY_ENV_VAR).is_err()
        && std::env::var(CODEX_API_KEY_ENV_VAR).is_err()
    {
        bail!(
            "No authentication credentials found. Run `codex login` or set the {OPENAI_API_KEY_ENV_VAR} environment variable."
        );
    }

    // Runtime初期化
    let conversation_id = ConversationId::default();
    let otel_manager = OtelEventManager::new(
        conversation_id,
        config.model.as_str(),
        config.model_family.slug.as_str(),
        auth_snapshot
            .as_ref()
            .and_then(|auth| auth.get_account_id()),
        auth_snapshot
            .as_ref()
            .and_then(|auth| auth.get_account_email()),
        auth_snapshot.as_ref().map(|auth| auth.mode),
        config.otel.log_user_prompt,
        terminal::user_agent(),
    );

    let runtime_budget = config
        .model_context_window
        .unwrap_or(DEFAULT_SUBAGENT_RUNTIME_BUDGET)
        .min(usize::MAX as u64) as usize;

    let runtime = AgentRuntime::new(
        workspace_dir.clone(),
        runtime_budget,
        Arc::clone(&config),
        Some(Arc::clone(&auth_manager)),
        otel_manager,
        config.model_provider.clone(),
        conversation_id,
    );

    println!("🚀 Creating and running custom agent...");
    println!();

    // カスタムエージェント作成＆実行
    let result = runtime
        .create_and_run_custom_agent(&prompt, budget)
        .await
        .context("custom agent execution failed")?;

    println!("\n📊 Execution summary:");
    println!("   Status: {:?}", result.status);
    println!("   Tokens used: {}", result.tokens_used);
    println!("   Duration: {:.2}s", result.duration_secs);

    if !result.artifacts.is_empty() {
        println!("\n🗂️  Generated artifacts:");
        for artifact in &result.artifacts {
            println!("   - {artifact}");
        }
    }

    if let Some(ref error) = result.error {
        eprintln!("\n⚠️  Agent reported an error: {error}");
    }

    if save {
        println!("\n💾 Agent definition can be saved to .codex/agents/");
        println!("   (YAML save feature coming soon)");
    }

    if let Some(out_file) = out {
        let report = serde_json::json!({
            "prompt": prompt,
            "status": format!("{:?}", result.status),
            "tokens_used": result.tokens_used,
            "duration_secs": result.duration_secs,
            "artifacts": result.artifacts,
            "error": result.error,
        });
        std::fs::write(&out_file, serde_json::to_string_pretty(&report)?)
            .context("failed to write results")?;
        println!("\n📄 Results saved to: {}", out_file.display());
    }

    if result.status == AgentStatus::Completed {
        println!("\n✅ Custom agent completed successfully!");
    } else {
        bail!("Custom agent execution failed");
    }

    Ok(())
}
