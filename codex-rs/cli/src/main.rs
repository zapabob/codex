use clap::Args;
use clap::CommandFactory;
use clap::Parser;
use clap_complete::Shell;
use clap_complete::generate;
use codex_arg0::arg0_dispatch_or_else;
use codex_chatgpt::apply_command::ApplyCommand;
use codex_chatgpt::apply_command::run_apply_command;
use codex_cli::LandlockCommand;
use codex_cli::SeatbeltCommand;
use codex_cli::WindowsCommand;
use codex_cli::login::read_api_key_from_stdin;
use codex_cli::login::run_login_status;
use codex_cli::login::run_login_with_api_key;
use codex_cli::login::run_login_with_chatgpt;
use codex_cli::login::run_login_with_device_code;
use codex_cli::login::run_logout;
use codex_cloud_tasks::Cli as CloudTasksCli;
use codex_common::CliConfigOverrides;
use codex_exec::Cli as ExecCli;
use codex_exec::Command as ExecCommand;
use codex_exec::ReviewArgs;
use codex_execpolicy::ExecPolicyCheckCommand;
use codex_responses_api_proxy::Args as ResponsesApiProxyArgs;
use codex_tui::AppExitInfo;
use codex_tui::Cli as TuiCli;
use codex_tui::ExitReason;
use codex_tui::update_action::UpdateAction;
use owo_colors::OwoColorize;
use std::io::IsTerminal;
use std::path::PathBuf;
use supports_color::Stream;

mod mcp_cmd;
#[cfg(feature = "custom-features")]
mod research_cmd;
#[cfg(not(windows))]
mod wsl_paths;

use crate::mcp_cmd::McpCli;

#[cfg(feature = "custom-features")]
use codex_core::a2a_communication::{
    A2ACommunicationManager, A2AConfig, AgentCapability, AgentIdentity, AgentRole,
};
#[cfg(feature = "custom-features")]
use codex_core::autonomous_orchestration::{
    AutonomousOrchestrationConfig, AutonomousOrchestrationManager, TaskPriority, TaskRequest,
};
use codex_core::config::Config;
use codex_core::config::ConfigOverrides;
use codex_core::config::edit::ConfigEditsBuilder;
use codex_core::config::find_codex_home;
use codex_core::features::Stage;
use codex_core::features::is_known_feature_key;
#[cfg(feature = "custom-features")]
use codex_core::llmops::{
    LLMOpsConfig, LLMOpsManager, ModelProvider, ModelVersion, PromptTemplate,
};
#[cfg(feature = "custom-features")]
use codex_core::skill_mcp_integration::{
    MCPResource, MCPTool, SkillDefinition, SkillMCPConfig, SkillMCPIntegrationManager,
};
use codex_utils_absolute_path::AbsolutePathBuf;

/// Codex CLI
///
/// If no subcommand is specified, options will be forwarded to the interactive CLI.
#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    // If a sub‑command is given, ignore requirements of the default args.
    subcommand_negates_reqs = true,
    // The executable is sometimes invoked via a platform‑specific name like
    // `codex-x86_64-unknown-linux-musl`, but the help output should always use
    // the generic `codex` command name that users run.
    bin_name = "codex",
    override_usage = "codex [OPTIONS] [PROMPT]\n       codex [OPTIONS] <COMMAND> [ARGS]"
)]
struct MultitoolCli {
    #[clap(flatten)]
    pub config_overrides: CliConfigOverrides,

    #[clap(flatten)]
    pub feature_toggles: FeatureToggles,

    #[clap(flatten)]
    interactive: TuiCli,

    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    /// Run Codex non-interactively.
    #[clap(visible_alias = "e")]
    Exec(ExecCli),

    /// Run a code review non-interactively.
    Review(ReviewArgs),

    /// [experimental] Conduct deep research on a topic.
    #[cfg(feature = "custom-features")]
    Research(ResearchCommand),

    /// [experimental] LLMOps management and monitoring.
    #[cfg(feature = "custom-features")]
    LlmOps(LlmOpsCommand),

    /// [experimental] Agent-to-Agent communication management.
    #[cfg(feature = "custom-features")]
    A2a(A2aCommand),

    /// [experimental] Skill and MCP integration management.
    #[cfg(feature = "custom-features")]
    SkillMcp(SkillMcpCommand),

    /// [experimental] Autonomous orchestration management.
    #[cfg(feature = "custom-features")]
    Orchestrate(OrchestrateCommand),

    /// Manage login.
    Login(LoginCommand),

    /// Remove stored authentication credentials.
    Logout(LogoutCommand),

    /// [experimental] Run Codex as an MCP server and manage MCP servers.
    Mcp(McpCli),

    /// [experimental] Run the Codex MCP server (stdio transport).
    McpServer,

    /// [experimental] Run the app server or related tooling.
    AppServer(AppServerCommand),

    /// Generate shell completion scripts.
    Completion(CompletionCommand),

    /// Run commands within a Codex-provided sandbox.
    #[clap(visible_alias = "debug")]
    Sandbox(SandboxArgs),

    /// Execpolicy tooling.
    #[clap(hide = true)]
    Execpolicy(ExecpolicyCommand),

    /// Apply the latest diff produced by Codex agent as a `git apply` to your local working tree.
    #[clap(visible_alias = "a")]
    Apply(ApplyCommand),

    /// Resume a previous interactive session (picker by default; use --last to continue the most recent).
    Resume(ResumeCommand),

    /// Fork a previous interactive session (picker by default; use --last to fork the most recent).
    Fork(ForkCommand),

    /// [EXPERIMENTAL] Browse tasks from Codex Cloud and apply changes locally.
    #[clap(name = "cloud", alias = "cloud-tasks")]
    Cloud(CloudTasksCli),

    /// Internal: run the responses API proxy.
    #[clap(hide = true)]
    ResponsesApiProxy(ResponsesApiProxyArgs),

    /// Internal: relay stdio to a Unix domain socket.
    #[clap(hide = true, name = "stdio-to-uds")]
    StdioToUds(StdioToUdsCommand),

    /// Inspect feature flags.
    Features(FeaturesCli),
}

#[derive(Debug, Parser)]
struct CompletionCommand {
    /// Shell to generate completions for
    #[clap(value_enum, default_value_t = Shell::Bash)]
    shell: Shell,
}

#[derive(Debug, Parser)]
struct ResumeCommand {
    /// Conversation/session id (UUID). When provided, resumes this session.
    /// If omitted, use --last to pick the most recent recorded session.
    #[arg(value_name = "SESSION_ID")]
    session_id: Option<String>,

    /// Continue the most recent session without showing the picker.
    #[arg(long = "last", default_value_t = false)]
    last: bool,

    /// Show all sessions (disables cwd filtering and shows CWD column).
    #[arg(long = "all", default_value_t = false)]
    all: bool,

    #[clap(flatten)]
    config_overrides: TuiCli,
}

#[derive(Debug, Parser)]
struct ForkCommand {
    /// Conversation/session id (UUID). When provided, forks this session.
    /// If omitted, use --last to pick the most recent recorded session.
    #[arg(value_name = "SESSION_ID")]
    session_id: Option<String>,

    /// Fork the most recent session without showing the picker.
    #[arg(long = "last", default_value_t = false, conflicts_with = "session_id")]
    last: bool,

    /// Show all sessions (disables cwd filtering and shows CWD column).
    #[arg(long = "all", default_value_t = false)]
    all: bool,

    #[clap(flatten)]
    config_overrides: TuiCli,
}

#[derive(Debug, Parser)]
struct SandboxArgs {
    #[command(subcommand)]
    cmd: SandboxCommand,
}

#[derive(Debug, clap::Subcommand)]
enum SandboxCommand {
    /// Run a command under Seatbelt (macOS only).
    #[clap(visible_alias = "seatbelt")]
    Macos(SeatbeltCommand),

    /// Run a command under Landlock+seccomp (Linux only).
    #[clap(visible_alias = "landlock")]
    Linux(LandlockCommand),

    /// Run a command under Windows restricted token (Windows only).
    Windows(WindowsCommand),
}

#[derive(Debug, Parser)]
struct ExecpolicyCommand {
    #[command(subcommand)]
    sub: ExecpolicySubcommand,
}

#[derive(Debug, clap::Subcommand)]
enum ExecpolicySubcommand {
    /// Check execpolicy files against a command.
    #[clap(name = "check")]
    Check(ExecPolicyCheckCommand),
}

#[derive(Debug, Parser)]
struct LoginCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,

    #[arg(
        long = "with-api-key",
        help = "Read the API key from stdin (e.g. `printenv OPENAI_API_KEY | codex login --with-api-key`)"
    )]
    with_api_key: bool,

    #[arg(
        long = "api-key",
        value_name = "API_KEY",
        help = "(deprecated) Previously accepted the API key directly; now exits with guidance to use --with-api-key",
        hide = true
    )]
    api_key: Option<String>,

    #[arg(long = "device-auth")]
    use_device_code: bool,

    /// EXPERIMENTAL: Use custom OAuth issuer base URL (advanced)
    /// Override the OAuth issuer base URL (advanced)
    #[arg(long = "experimental_issuer", value_name = "URL", hide = true)]
    issuer_base_url: Option<String>,

    /// EXPERIMENTAL: Use custom OAuth client ID (advanced)
    #[arg(long = "experimental_client-id", value_name = "CLIENT_ID", hide = true)]
    client_id: Option<String>,

    #[command(subcommand)]
    action: Option<LoginSubcommand>,
}

#[derive(Debug, clap::Subcommand)]
enum LoginSubcommand {
    /// Show login status.
    Status,
}

#[derive(Debug, Parser)]
struct LogoutCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,
}

#[derive(Debug, Parser)]
struct AppServerCommand {
    /// Omit to run the app server; specify a subcommand for tooling.
    #[command(subcommand)]
    subcommand: Option<AppServerSubcommand>,

    /// Controls whether analytics are enabled by default.
    ///
    /// Analytics are disabled by default for app-server. Users have to explicitly opt in
    /// via the `analytics` section in the config.toml file.
    ///
    /// However, for first-party use cases like the VSCode IDE extension, we default analytics
    /// to be enabled by default by setting this flag. Users can still opt out by setting this
    /// in their config.toml:
    ///
    /// ```toml
    /// [analytics]
    /// enabled = false
    /// ```
    ///
    /// See https://developers.openai.com/codex/config-advanced/#metrics for more details.
    #[arg(long = "analytics-default-enabled")]
    analytics_default_enabled: bool,
}

#[derive(Debug, clap::Subcommand)]
enum AppServerSubcommand {
    /// [experimental] Generate TypeScript bindings for the app server protocol.
    GenerateTs(GenerateTsCommand),

    /// [experimental] Generate JSON Schema for the app server protocol.
    GenerateJsonSchema(GenerateJsonSchemaCommand),
}

#[derive(Debug, Args)]
struct GenerateTsCommand {
    /// Output directory where .ts files will be written
    #[arg(short = 'o', long = "out", value_name = "DIR")]
    out_dir: PathBuf,

    /// Optional path to the Prettier executable to format generated files
    #[arg(short = 'p', long = "prettier", value_name = "PRETTIER_BIN")]
    prettier: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct GenerateJsonSchemaCommand {
    /// Output directory where the schema bundle will be written
    #[arg(short = 'o', long = "out", value_name = "DIR")]
    out_dir: PathBuf,
}

#[derive(Debug, Parser)]
struct StdioToUdsCommand {
    /// Path to the Unix domain socket to connect to.
    #[arg(value_name = "SOCKET_PATH")]
    socket_path: PathBuf,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, Parser)]
struct ResearchCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,

    /// Topic to research
    #[arg(value_name = "TOPIC")]
    topic: String,

    /// Research depth (1-5)
    #[arg(short, long, value_name = "DEPTH", default_value = "3")]
    depth: u8,

    /// Search breadth (number of sources)
    #[arg(short, long, value_name = "BREADTH", default_value = "8")]
    breadth: u8,

    /// Token budget
    #[arg(long, value_name = "TOKENS", default_value = "60000")]
    budget: usize,

    /// Require citations
    #[arg(long, default_value = "true")]
    citations: bool,

    /// MCP tools to use (comma-separated)
    #[arg(long, value_name = "TOOLS")]
    mcp: Option<String>,

    /// Enable lightweight fallback
    #[arg(long, default_value = "false")]
    lightweight_fallback: bool,

    /// Use Gemini CLI with Google Search (OAuth 2.0 authentication)
    #[arg(long, default_value = "false")]
    gemini: bool,

    /// Use MCP mode (Codex → MCP → Gemini CLI)
    #[arg(long, default_value = "false")]
    use_mcp: bool,

    /// Output file for the report
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, Parser)]
struct LlmOpsCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,

    #[clap(subcommand)]
    subcommand: LlmOpsSubcommand,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, clap::Subcommand)]
enum LlmOpsSubcommand {
    /// Register a new model version
    RegisterModel {
        /// Model ID
        #[arg(value_name = "MODEL_ID")]
        model_id: String,

        /// Model name
        #[arg(long, value_name = "NAME")]
        name: String,

        /// Version
        #[arg(long, value_name = "VERSION")]
        version: String,

        /// Provider (openai, anthropic, google, etc.)
        #[arg(long, value_name = "PROVIDER")]
        provider: String,
    },

    /// Register a new prompt template
    RegisterPrompt {
        /// Prompt ID
        #[arg(value_name = "PROMPT_ID")]
        prompt_id: String,

        /// Prompt name
        #[arg(long, value_name = "NAME")]
        name: String,

        /// Template file path
        #[arg(long, value_name = "TEMPLATE")]
        template: PathBuf,
    },

    /// Show LLMOps status
    Status,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, Parser)]
struct A2aCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,

    #[clap(subcommand)]
    subcommand: A2aSubcommand,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, clap::Subcommand)]
enum A2aSubcommand {
    /// Register an agent
    RegisterAgent {
        /// Agent ID
        #[arg(value_name = "AGENT_ID")]
        agent_id: String,

        /// Agent role
        #[arg(long, value_name = "ROLE")]
        role: String,

        /// Capabilities (comma-separated)
        #[arg(long, value_name = "CAPABILITIES")]
        capabilities: String,
    },

    /// Send message to agent
    SendMessage {
        /// Target agent ID
        #[arg(value_name = "AGENT_ID")]
        agent_id: String,

        /// Message content
        #[arg(value_name = "MESSAGE")]
        message: String,
    },

    /// Show A2A status
    Status,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, Parser)]
struct SkillMcpCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,

    #[clap(subcommand)]
    subcommand: SkillMcpSubcommand,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, clap::Subcommand)]
enum SkillMcpSubcommand {
    /// Register a skill
    RegisterSkill {
        /// Skill definition file
        #[arg(value_name = "SKILL_FILE")]
        skill_file: PathBuf,
    },

    /// Register MCP resource
    RegisterResource {
        /// Resource URI
        #[arg(value_name = "URI")]
        uri: String,

        /// Resource name
        #[arg(long, value_name = "NAME")]
        name: String,
    },

    /// Execute skill
    ExecuteSkill {
        /// Skill ID
        #[arg(value_name = "SKILL_ID")]
        skill_id: String,

        /// Input data (JSON)
        #[arg(value_name = "INPUT")]
        input: String,
    },

    /// Show Skill/MCP status
    Status,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, Parser)]
struct OrchestrateCommand {
    #[clap(skip)]
    config_overrides: CliConfigOverrides,

    #[clap(subcommand)]
    subcommand: OrchestrateSubcommand,
}

#[cfg(feature = "custom-features")]
#[derive(Debug, clap::Subcommand)]
enum OrchestrateSubcommand {
    /// Submit a task for orchestration
    SubmitTask {
        /// Task name
        #[arg(value_name = "NAME")]
        name: String,

        /// Task description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Priority (critical, high, medium, low, trivial)
        #[arg(long, value_name = "PRIORITY", default_value = "medium")]
        priority: String,

        /// Required capabilities (comma-separated)
        #[arg(long, value_name = "CAPABILITIES")]
        capabilities: Option<String>,
    },

    /// Get task status
    TaskStatus {
        /// Task ID
        #[arg(value_name = "TASK_ID")]
        task_id: String,
    },

    /// Show orchestration status
    Status,
}

fn format_exit_messages(exit_info: AppExitInfo, color_enabled: bool) -> Vec<String> {
    let AppExitInfo {
        token_usage,
        thread_id: conversation_id,
        ..
    } = exit_info;

    if token_usage.is_zero() {
        return Vec::new();
    }

    let mut lines = vec![format!(
        "{}",
        codex_core::protocol::FinalOutput::from(token_usage)
    )];

    if let Some(session_id) = conversation_id {
        let resume_cmd = format!("codex resume {session_id}");
        let command = if color_enabled {
            resume_cmd.cyan().to_string()
        } else {
            resume_cmd
        };
        lines.push(format!("To continue this session, run {command}"));
    }

    lines
}

/// Handle the app exit and print the results. Optionally run the update action.
fn handle_app_exit(exit_info: AppExitInfo) -> anyhow::Result<()> {
    match exit_info.exit_reason {
        ExitReason::Fatal(message) => {
            eprintln!("ERROR: {message}");
            std::process::exit(1);
        }
        ExitReason::UserRequested => { /* normal exit */ }
    }

    let update_action = exit_info.update_action;
    let color_enabled = supports_color::on(Stream::Stdout).is_some();
    for line in format_exit_messages(exit_info, color_enabled) {
        println!("{line}");
    }
    if let Some(action) = update_action {
        run_update_action(action)?;
    }
    Ok(())
}

/// Run the update action and print the result.
fn run_update_action(action: UpdateAction) -> anyhow::Result<()> {
    println!();
    let cmd_str = action.command_str();
    println!("Updating Codex via `{cmd_str}`...");

    let status = {
        #[cfg(windows)]
        {
            // On Windows, run via cmd.exe so .CMD/.BAT are correctly resolved (PATHEXT semantics).
            std::process::Command::new("cmd")
                .args(["/C", &cmd_str])
                .status()?
        }
        #[cfg(not(windows))]
        {
            let (cmd, args) = action.command_args();
            let command_path = crate::wsl_paths::normalize_for_wsl(cmd);
            let normalized_args: Vec<String> = args
                .iter()
                .map(crate::wsl_paths::normalize_for_wsl)
                .collect();
            std::process::Command::new(&command_path)
                .args(&normalized_args)
                .status()?
        }
    };
    if !status.success() {
        anyhow::bail!("`{cmd_str}` failed with status {status}");
    }
    println!("\n🎉 Update ran successfully! Please restart Codex.");
    Ok(())
}

fn run_execpolicycheck(cmd: ExecPolicyCheckCommand) -> anyhow::Result<()> {
    cmd.run()
}

#[derive(Debug, Default, Parser, Clone)]
struct FeatureToggles {
    /// Enable a feature (repeatable). Equivalent to `-c features.<name>=true`.
    #[arg(long = "enable", value_name = "FEATURE", action = clap::ArgAction::Append, global = true)]
    enable: Vec<String>,

    /// Disable a feature (repeatable). Equivalent to `-c features.<name>=false`.
    #[arg(long = "disable", value_name = "FEATURE", action = clap::ArgAction::Append, global = true)]
    disable: Vec<String>,
}

impl FeatureToggles {
    fn to_overrides(&self) -> anyhow::Result<Vec<String>> {
        let mut v = Vec::new();
        for feature in &self.enable {
            Self::validate_feature(feature)?;
            v.push(format!("features.{feature}=true"));
        }
        for feature in &self.disable {
            Self::validate_feature(feature)?;
            v.push(format!("features.{feature}=false"));
        }
        Ok(v)
    }

    fn validate_feature(feature: &str) -> anyhow::Result<()> {
        if is_known_feature_key(feature) {
            Ok(())
        } else {
            anyhow::bail!("Unknown feature flag: {feature}")
        }
    }
}

#[derive(Debug, Parser)]
struct FeaturesCli {
    #[command(subcommand)]
    sub: FeaturesSubcommand,
}

#[derive(Debug, Parser)]
enum FeaturesSubcommand {
    /// List known features with their stage and effective state.
    List,
    /// Enable a feature in config.toml.
    Enable(FeatureSetArgs),
    /// Disable a feature in config.toml.
    Disable(FeatureSetArgs),
}

#[derive(Debug, Parser)]
struct FeatureSetArgs {
    /// Feature key to update (for example: unified_exec).
    feature: String,
}

fn stage_str(stage: codex_core::features::Stage) -> &'static str {
    use codex_core::features::Stage;
    match stage {
        Stage::UnderDevelopment => "under development",
        Stage::Experimental { .. } => "experimental",
        Stage::Stable => "stable",
        Stage::Deprecated => "deprecated",
        Stage::Removed => "removed",
    }
}

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|codex_linux_sandbox_exe| async move {
        cli_main(codex_linux_sandbox_exe).await?;
        Ok(())
    })
}

async fn cli_main(codex_linux_sandbox_exe: Option<PathBuf>) -> anyhow::Result<()> {
    let MultitoolCli {
        config_overrides: mut root_config_overrides,
        feature_toggles,
        mut interactive,
        subcommand,
    } = MultitoolCli::parse();

    // Fold --enable/--disable into config overrides so they flow to all subcommands.
    let toggle_overrides = feature_toggles.to_overrides()?;
    root_config_overrides.raw_overrides.extend(toggle_overrides);

    match subcommand {
        None => {
            prepend_config_flags(
                &mut interactive.config_overrides,
                root_config_overrides.clone(),
            );
            let exit_info = run_interactive_tui(interactive, codex_linux_sandbox_exe).await?;
            handle_app_exit(exit_info)?;
        }
        Some(Subcommand::Exec(mut exec_cli)) => {
            prepend_config_flags(
                &mut exec_cli.config_overrides,
                root_config_overrides.clone(),
            );
            codex_exec::run_main(exec_cli, codex_linux_sandbox_exe).await?;
        }
        Some(Subcommand::Review(review_args)) => {
            let mut exec_cli = ExecCli::try_parse_from(["codex", "exec"])?;
            exec_cli.command = Some(ExecCommand::Review(review_args));
            prepend_config_flags(
                &mut exec_cli.config_overrides,
                root_config_overrides.clone(),
            );
            codex_exec::run_main(exec_cli, codex_linux_sandbox_exe).await?;
        }
        #[cfg(feature = "custom-features")]
        Some(Subcommand::Research(mut research_cmd)) => {
            prepend_config_flags(
                &mut research_cmd.config_overrides,
                root_config_overrides.clone(),
            );
            research_cmd::run_research_command(
                research_cmd.topic,
                research_cmd.depth,
                research_cmd.breadth,
                research_cmd.budget,
                research_cmd.citations,
                research_cmd.mcp,
                research_cmd.lightweight_fallback,
                research_cmd.out,
                research_cmd.gemini,
                research_cmd.use_mcp,
            )
            .await?;
        }
        #[cfg(feature = "custom-features")]
        Some(Subcommand::LlmOps(llmops_cmd)) => {
            run_llmops_command(llmops_cmd).await?;
        }
        #[cfg(feature = "custom-features")]
        Some(Subcommand::A2a(a2a_cmd)) => {
            run_a2a_command(a2a_cmd).await?;
        }
        #[cfg(feature = "custom-features")]
        Some(Subcommand::SkillMcp(skill_mcp_cmd)) => {
            run_skill_mcp_command(skill_mcp_cmd).await?;
        }
        #[cfg(feature = "custom-features")]
        Some(Subcommand::Orchestrate(orchestrate_cmd)) => {
            run_orchestrate_command(orchestrate_cmd).await?;
        }
        Some(Subcommand::McpServer) => {
            codex_mcp_server::run_main(codex_linux_sandbox_exe, root_config_overrides).await?;
        }
        Some(Subcommand::Mcp(mut mcp_cli)) => {
            // Propagate any root-level config overrides (e.g. `-c key=value`).
            prepend_config_flags(&mut mcp_cli.config_overrides, root_config_overrides.clone());
            mcp_cli.run().await?;
        }
        Some(Subcommand::AppServer(app_server_cli)) => match app_server_cli.subcommand {
            None => {
                codex_app_server::run_main(
                    codex_linux_sandbox_exe,
                    root_config_overrides,
                    codex_core::config_loader::LoaderOverrides::default(),
                    app_server_cli.analytics_default_enabled,
                )
                .await?;
            }
            Some(AppServerSubcommand::GenerateTs(gen_cli)) => {
                codex_app_server_protocol::generate_ts(
                    &gen_cli.out_dir,
                    gen_cli.prettier.as_deref(),
                )?;
            }
            Some(AppServerSubcommand::GenerateJsonSchema(gen_cli)) => {
                codex_app_server_protocol::generate_json(&gen_cli.out_dir)?;
            }
        },
        Some(Subcommand::Resume(ResumeCommand {
            session_id,
            last,
            all,
            config_overrides,
        })) => {
            interactive = finalize_resume_interactive(
                interactive,
                root_config_overrides.clone(),
                session_id,
                last,
                all,
                config_overrides,
            );
            let exit_info = run_interactive_tui(interactive, codex_linux_sandbox_exe).await?;
            handle_app_exit(exit_info)?;
        }
        Some(Subcommand::Fork(ForkCommand {
            session_id,
            last,
            all,
            config_overrides,
        })) => {
            interactive = finalize_fork_interactive(
                interactive,
                root_config_overrides.clone(),
                session_id,
                last,
                all,
                config_overrides,
            );
            let exit_info = run_interactive_tui(interactive, codex_linux_sandbox_exe).await?;
            handle_app_exit(exit_info)?;
        }
        Some(Subcommand::Login(mut login_cli)) => {
            prepend_config_flags(
                &mut login_cli.config_overrides,
                root_config_overrides.clone(),
            );
            match login_cli.action {
                Some(LoginSubcommand::Status) => {
                    run_login_status(login_cli.config_overrides).await;
                }
                None => {
                    if login_cli.use_device_code {
                        run_login_with_device_code(
                            login_cli.config_overrides,
                            login_cli.issuer_base_url,
                            login_cli.client_id,
                        )
                        .await;
                    } else if login_cli.api_key.is_some() {
                        eprintln!(
                            "The --api-key flag is no longer supported. Pipe the key instead, e.g. `printenv OPENAI_API_KEY | codex login --with-api-key`."
                        );
                        std::process::exit(1);
                    } else if login_cli.with_api_key {
                        let api_key = read_api_key_from_stdin();
                        run_login_with_api_key(login_cli.config_overrides, api_key).await;
                    } else {
                        run_login_with_chatgpt(login_cli.config_overrides).await;
                    }
                }
            }
        }
        Some(Subcommand::Logout(mut logout_cli)) => {
            prepend_config_flags(
                &mut logout_cli.config_overrides,
                root_config_overrides.clone(),
            );
            run_logout(logout_cli.config_overrides).await;
        }
        Some(Subcommand::Completion(completion_cli)) => {
            print_completion(completion_cli);
        }
        Some(Subcommand::Cloud(mut cloud_cli)) => {
            prepend_config_flags(
                &mut cloud_cli.config_overrides,
                root_config_overrides.clone(),
            );
            codex_cloud_tasks::run_main(cloud_cli, codex_linux_sandbox_exe).await?;
        }
        Some(Subcommand::Sandbox(sandbox_args)) => match sandbox_args.cmd {
            SandboxCommand::Macos(mut seatbelt_cli) => {
                prepend_config_flags(
                    &mut seatbelt_cli.config_overrides,
                    root_config_overrides.clone(),
                );
                codex_cli::debug_sandbox::run_command_under_seatbelt(
                    seatbelt_cli,
                    codex_linux_sandbox_exe,
                )
                .await?;
            }
            SandboxCommand::Linux(mut landlock_cli) => {
                prepend_config_flags(
                    &mut landlock_cli.config_overrides,
                    root_config_overrides.clone(),
                );
                codex_cli::debug_sandbox::run_command_under_landlock(
                    landlock_cli,
                    codex_linux_sandbox_exe,
                )
                .await?;
            }
            SandboxCommand::Windows(mut windows_cli) => {
                prepend_config_flags(
                    &mut windows_cli.config_overrides,
                    root_config_overrides.clone(),
                );
                codex_cli::debug_sandbox::run_command_under_windows(
                    windows_cli,
                    codex_linux_sandbox_exe,
                )
                .await?;
            }
        },
        Some(Subcommand::Execpolicy(ExecpolicyCommand { sub })) => match sub {
            ExecpolicySubcommand::Check(cmd) => run_execpolicycheck(cmd)?,
        },
        Some(Subcommand::Apply(mut apply_cli)) => {
            prepend_config_flags(
                &mut apply_cli.config_overrides,
                root_config_overrides.clone(),
            );
            run_apply_command(apply_cli, None).await?;
        }
        Some(Subcommand::ResponsesApiProxy(args)) => {
            tokio::task::spawn_blocking(move || codex_responses_api_proxy::run_main(args))
                .await??;
        }
        Some(Subcommand::StdioToUds(cmd)) => {
            let socket_path = cmd.socket_path;
            tokio::task::spawn_blocking(move || codex_stdio_to_uds::run(socket_path.as_path()))
                .await??;
        }
        Some(Subcommand::Features(FeaturesCli { sub })) => match sub {
            FeaturesSubcommand::List => {
                // Respect root-level `-c` overrides plus top-level flags like `--profile`.
                let mut cli_kv_overrides = root_config_overrides
                    .parse_overrides()
                    .map_err(anyhow::Error::msg)?;

                // Honor `--search` via the canonical web_search mode.
                if interactive.web_search {
                    cli_kv_overrides.push((
                        "web_search".to_string(),
                        toml::Value::String("live".to_string()),
                    ));
                }

                // Thread through relevant top-level flags (at minimum, `--profile`).
                let overrides = ConfigOverrides {
                    config_profile: interactive.config_profile.clone(),
                    ..Default::default()
                };

                let config = Config::load_with_cli_overrides_and_harness_overrides(
                    cli_kv_overrides,
                    overrides,
                )
                .await?;
                let mut rows = Vec::with_capacity(codex_core::features::FEATURES.len());
                let mut name_width = 0;
                let mut stage_width = 0;
                for def in codex_core::features::FEATURES.iter() {
                    let name = def.key;
                    let stage = stage_str(def.stage);
                    let enabled = config.features.enabled(def.id);
                    name_width = name_width.max(name.len());
                    stage_width = stage_width.max(stage.len());
                    rows.push((name, stage, enabled));
                }

                for (name, stage, enabled) in rows {
                    println!("{name:<name_width$}  {stage:<stage_width$}  {enabled}");
                }
            }
            FeaturesSubcommand::Enable(FeatureSetArgs { feature }) => {
                enable_feature_in_config(&interactive, &feature).await?;
            }
            FeaturesSubcommand::Disable(FeatureSetArgs { feature }) => {
                disable_feature_in_config(&interactive, &feature).await?;
            }
        },
    }

    Ok(())
}

async fn enable_feature_in_config(interactive: &TuiCli, feature: &str) -> anyhow::Result<()> {
    FeatureToggles::validate_feature(feature)?;
    let codex_home = find_codex_home()?;
    ConfigEditsBuilder::new(&codex_home)
        .with_profile(interactive.config_profile.as_deref())
        .set_feature_enabled(feature, true)
        .apply()
        .await?;
    println!("Enabled feature `{feature}` in config.toml.");
    maybe_print_under_development_feature_warning(&codex_home, interactive, feature);
    Ok(())
}

async fn disable_feature_in_config(interactive: &TuiCli, feature: &str) -> anyhow::Result<()> {
    FeatureToggles::validate_feature(feature)?;
    let codex_home = find_codex_home()?;
    ConfigEditsBuilder::new(&codex_home)
        .with_profile(interactive.config_profile.as_deref())
        .set_feature_enabled(feature, false)
        .apply()
        .await?;
    println!("Disabled feature `{feature}` in config.toml.");
    Ok(())
}

fn maybe_print_under_development_feature_warning(
    codex_home: &std::path::Path,
    interactive: &TuiCli,
    feature: &str,
) {
    if interactive.config_profile.is_some() {
        return;
    }

    let Some(spec) = codex_core::features::FEATURES
        .iter()
        .find(|spec| spec.key == feature)
    else {
        return;
    };
    if !matches!(spec.stage, Stage::UnderDevelopment) {
        return;
    }

    let config_path = codex_home.join(codex_core::config::CONFIG_TOML_FILE);
    eprintln!(
        "Under-development features enabled: {feature}. Under-development features are incomplete and may behave unpredictably. To suppress this warning, set `suppress_unstable_features_warning = true` in {}.",
        config_path.display()
    );
}

/// Prepend root-level overrides so they have lower precedence than
/// CLI-specific ones specified after the subcommand (if any).
fn prepend_config_flags(
    subcommand_config_overrides: &mut CliConfigOverrides,
    cli_config_overrides: CliConfigOverrides,
) {
    subcommand_config_overrides
        .raw_overrides
        .splice(0..0, cli_config_overrides.raw_overrides);
}

async fn run_interactive_tui(
    mut interactive: TuiCli,
    codex_linux_sandbox_exe: Option<PathBuf>,
) -> std::io::Result<AppExitInfo> {
    if let Some(prompt) = interactive.prompt.take() {
        // Normalize CRLF/CR to LF so CLI-provided text can't leak `\r` into TUI state.
        interactive.prompt = Some(prompt.replace("\r\n", "\n").replace('\r', "\n"));
    }

    let terminal_info = codex_core::terminal::terminal_info();
    if terminal_info.name == TerminalName::Dumb {
        if !(std::io::stdin().is_terminal() && std::io::stderr().is_terminal()) {
            return Ok(AppExitInfo::fatal(
                "TERM is set to \"dumb\". Refusing to start the interactive TUI because no terminal is available for a confirmation prompt (stdin/stderr is not a TTY). Run in a supported terminal or unset TERM.",
            ));
        }

        eprintln!(
            "WARNING: TERM is set to \"dumb\". Codex's interactive TUI may not work in this terminal."
        );
        if !confirm("Continue anyway? [y/N]: ")? {
            return Ok(AppExitInfo::fatal(
                "Refusing to start the interactive TUI because TERM is set to \"dumb\". Run in a supported terminal or unset TERM.",
            ));
        }
    }

    codex_tui::run_main(interactive, codex_linux_sandbox_exe).await
}

fn confirm(prompt: &str) -> std::io::Result<bool> {
    eprintln!("{prompt}");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let answer = input.trim();
    Ok(answer.eq_ignore_ascii_case("y") || answer.eq_ignore_ascii_case("yes"))
}

/// Build the final `TuiCli` for a `codex resume` invocation.
fn finalize_resume_interactive(
    mut interactive: TuiCli,
    root_config_overrides: CliConfigOverrides,
    session_id: Option<String>,
    last: bool,
    show_all: bool,
    resume_cli: TuiCli,
) -> TuiCli {
    // Start with the parsed interactive CLI so resume shares the same
    // configuration surface area as `codex` without additional flags.
    let resume_session_id = session_id;
    interactive.resume_picker = resume_session_id.is_none() && !last;
    interactive.resume_last = last;
    interactive.resume_session_id = resume_session_id;
    interactive.resume_show_all = show_all;

    // Merge resume-scoped flags and overrides with highest precedence.
    merge_interactive_cli_flags(&mut interactive, resume_cli);

    // Propagate any root-level config overrides (e.g. `-c key=value`).
    prepend_config_flags(&mut interactive.config_overrides, root_config_overrides);

    interactive
}

/// Build the final `TuiCli` for a `codex fork` invocation.
fn finalize_fork_interactive(
    mut interactive: TuiCli,
    root_config_overrides: CliConfigOverrides,
    session_id: Option<String>,
    last: bool,
    show_all: bool,
    fork_cli: TuiCli,
) -> TuiCli {
    // Start with the parsed interactive CLI so fork shares the same
    // configuration surface area as `codex` without additional flags.
    let fork_session_id = session_id;
    interactive.fork_picker = fork_session_id.is_none() && !last;
    interactive.fork_last = last;
    interactive.fork_session_id = fork_session_id;
    interactive.fork_show_all = show_all;

    // Merge fork-scoped flags and overrides with highest precedence.
    merge_interactive_cli_flags(&mut interactive, fork_cli);

    // Propagate any root-level config overrides (e.g. `-c key=value`).
    prepend_config_flags(&mut interactive.config_overrides, root_config_overrides);

    interactive
}

/// Merge flags provided to `codex resume`/`codex fork` so they take precedence over any
/// root-level flags. Only overrides fields explicitly set on the subcommand-scoped
/// CLI. Also appends `-c key=value` overrides with highest precedence.
fn merge_interactive_cli_flags(interactive: &mut TuiCli, subcommand_cli: TuiCli) {
    if let Some(model) = subcommand_cli.model {
        interactive.model = Some(model);
    }
    if subcommand_cli.oss {
        interactive.oss = true;
    }
    if let Some(profile) = subcommand_cli.config_profile {
        interactive.config_profile = Some(profile);
    }
    if let Some(sandbox) = subcommand_cli.sandbox_mode {
        interactive.sandbox_mode = Some(sandbox);
    }
    if let Some(approval) = subcommand_cli.approval_policy {
        interactive.approval_policy = Some(approval);
    }
    if subcommand_cli.full_auto {
        interactive.full_auto = true;
    }
    if subcommand_cli.dangerously_bypass_approvals_and_sandbox {
        interactive.dangerously_bypass_approvals_and_sandbox = true;
    }
    if let Some(cwd) = subcommand_cli.cwd {
        interactive.cwd = Some(cwd);
    }
    if subcommand_cli.web_search {
        interactive.web_search = true;
    }
    if !subcommand_cli.images.is_empty() {
        interactive.images = subcommand_cli.images;
    }
    if !subcommand_cli.add_dir.is_empty() {
        interactive.add_dir.extend(subcommand_cli.add_dir);
    }
    if let Some(prompt) = subcommand_cli.prompt {
        // Normalize CRLF/CR to LF so CLI-provided text can't leak `\r` into TUI state.
        interactive.prompt = Some(prompt.replace("\r\n", "\n").replace('\r', "\n"));
    }

    interactive
        .config_overrides
        .raw_overrides
        .extend(subcommand_cli.config_overrides.raw_overrides);
}

fn print_completion(cmd: CompletionCommand) {
    let mut app = MultitoolCli::command();
    let name = "codex";
    generate(cmd.shell, &mut app, name, &mut std::io::stdout());
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use codex_core::protocol::TokenUsage;
    use codex_protocol::ThreadId;
    use pretty_assertions::assert_eq;

    fn finalize_resume_from_args(args: &[&str]) -> TuiCli {
        let cli = MultitoolCli::try_parse_from(args).expect("parse");
        let MultitoolCli {
            interactive,
            config_overrides: root_overrides,
            subcommand,
            feature_toggles: _,
        } = cli;

        let Subcommand::Resume(ResumeCommand {
            session_id,
            last,
            all,
            config_overrides: resume_cli,
        }) = subcommand.expect("resume present")
        else {
            unreachable!()
        };

        finalize_resume_interactive(
            interactive,
            root_overrides,
            session_id,
            last,
            all,
            resume_cli,
        )
    }

    fn finalize_fork_from_args(args: &[&str]) -> TuiCli {
        let cli = MultitoolCli::try_parse_from(args).expect("parse");
        let MultitoolCli {
            interactive,
            config_overrides: root_overrides,
            subcommand,
            feature_toggles: _,
        } = cli;

        let Subcommand::Fork(ForkCommand {
            session_id,
            last,
            all,
            config_overrides: fork_cli,
        }) = subcommand.expect("fork present")
        else {
            unreachable!()
        };

        finalize_fork_interactive(interactive, root_overrides, session_id, last, all, fork_cli)
    }

    #[test]
    fn exec_resume_last_accepts_prompt_positional() {
        let cli =
            MultitoolCli::try_parse_from(["codex", "exec", "--json", "resume", "--last", "2+2"])
                .expect("parse should succeed");

        let Some(Subcommand::Exec(exec)) = cli.subcommand else {
            panic!("expected exec subcommand");
        };
        let Some(codex_exec::Command::Resume(args)) = exec.command else {
            panic!("expected exec resume");
        };

        assert!(args.last);
        assert_eq!(args.session_id, None);
        assert_eq!(args.prompt.as_deref(), Some("2+2"));
    }

    fn app_server_from_args(args: &[&str]) -> AppServerCommand {
        let cli = MultitoolCli::try_parse_from(args).expect("parse");
        let Subcommand::AppServer(app_server) = cli.subcommand.expect("app-server present") else {
            unreachable!()
        };
        app_server
    }

    fn sample_exit_info(conversation: Option<&str>) -> AppExitInfo {
        let token_usage = TokenUsage {
            output_tokens: 2,
            total_tokens: 2,
            ..Default::default()
        };
        AppExitInfo {
            token_usage,
            thread_id: conversation.map(ThreadId::from_string).map(Result::unwrap),
            update_action: None,
            exit_reason: ExitReason::UserRequested,
        }
    }

    #[test]
    fn format_exit_messages_skips_zero_usage() {
        let exit_info = AppExitInfo {
            token_usage: TokenUsage::default(),
            thread_id: None,
            update_action: None,
            exit_reason: ExitReason::UserRequested,
        };
        let lines = format_exit_messages(exit_info, false);
        assert!(lines.is_empty());
    }

    #[test]
    fn format_exit_messages_includes_resume_hint_without_color() {
        let exit_info = sample_exit_info(Some("123e4567-e89b-12d3-a456-426614174000"));
        let lines = format_exit_messages(exit_info, false);
        assert_eq!(
            lines,
            vec![
                "Token usage: total=2 input=0 output=2".to_string(),
                "To continue this session, run codex resume 123e4567-e89b-12d3-a456-426614174000"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn format_exit_messages_applies_color_when_enabled() {
        let exit_info = sample_exit_info(Some("123e4567-e89b-12d3-a456-426614174000"));
        let lines = format_exit_messages(exit_info, true);
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("\u{1b}[36m"));
    }

    #[test]
    fn resume_model_flag_applies_when_no_root_flags() {
        let interactive =
            finalize_resume_from_args(["codex", "resume", "-m", "gpt-5.1-test"].as_ref());

        assert_eq!(interactive.model.as_deref(), Some("gpt-5.1-test"));
        assert!(interactive.resume_picker);
        assert!(!interactive.resume_last);
        assert_eq!(interactive.resume_session_id, None);
    }

    #[test]
    fn resume_picker_logic_none_and_not_last() {
        let interactive = finalize_resume_from_args(["codex", "resume"].as_ref());
        assert!(interactive.resume_picker);
        assert!(!interactive.resume_last);
        assert_eq!(interactive.resume_session_id, None);
        assert!(!interactive.resume_show_all);
    }

    #[test]
    fn resume_picker_logic_last() {
        let interactive = finalize_resume_from_args(["codex", "resume", "--last"].as_ref());
        assert!(!interactive.resume_picker);
        assert!(interactive.resume_last);
        assert_eq!(interactive.resume_session_id, None);
        assert!(!interactive.resume_show_all);
    }

    #[test]
    fn resume_picker_logic_with_session_id() {
        let interactive = finalize_resume_from_args(["codex", "resume", "1234"].as_ref());
        assert!(!interactive.resume_picker);
        assert!(!interactive.resume_last);
        assert_eq!(interactive.resume_session_id.as_deref(), Some("1234"));
        assert!(!interactive.resume_show_all);
    }

    #[test]
    fn resume_all_flag_sets_show_all() {
        let interactive = finalize_resume_from_args(["codex", "resume", "--all"].as_ref());
        assert!(interactive.resume_picker);
        assert!(interactive.resume_show_all);
    }

    #[test]
    fn resume_merges_option_flags_and_full_auto() {
        let interactive = finalize_resume_from_args(
            [
                "codex",
                "resume",
                "sid",
                "--oss",
                "--full-auto",
                "--search",
                "--sandbox",
                "workspace-write",
                "--ask-for-approval",
                "on-request",
                "-m",
                "gpt-5.1-test",
                "-p",
                "my-profile",
                "-C",
                "/tmp",
                "-i",
                "/tmp/a.png,/tmp/b.png",
            ]
            .as_ref(),
        );

        assert_eq!(interactive.model.as_deref(), Some("gpt-5.1-test"));
        assert!(interactive.oss);
        assert_eq!(interactive.config_profile.as_deref(), Some("my-profile"));
        assert_matches!(
            interactive.sandbox_mode,
            Some(codex_common::SandboxModeCliArg::WorkspaceWrite)
        );
        assert_matches!(
            interactive.approval_policy,
            Some(codex_common::ApprovalModeCliArg::OnRequest)
        );
        assert!(interactive.full_auto);
        assert_eq!(
            interactive.cwd.as_deref(),
            Some(std::path::Path::new("/tmp"))
        );
        assert!(interactive.web_search);
        let has_a = interactive
            .images
            .iter()
            .any(|p| p == std::path::Path::new("/tmp/a.png"));
        let has_b = interactive
            .images
            .iter()
            .any(|p| p == std::path::Path::new("/tmp/b.png"));
        assert!(has_a && has_b);
        assert!(!interactive.resume_picker);
        assert!(!interactive.resume_last);
        assert_eq!(interactive.resume_session_id.as_deref(), Some("sid"));
    }

    #[test]
    fn resume_merges_dangerously_bypass_flag() {
        let interactive = finalize_resume_from_args(
            [
                "codex",
                "resume",
                "--dangerously-bypass-approvals-and-sandbox",
            ]
            .as_ref(),
        );
        assert!(interactive.dangerously_bypass_approvals_and_sandbox);
        assert!(interactive.resume_picker);
        assert!(!interactive.resume_last);
        assert_eq!(interactive.resume_session_id, None);
    }

    #[test]
    fn fork_picker_logic_none_and_not_last() {
        let interactive = finalize_fork_from_args(["codex", "fork"].as_ref());
        assert!(interactive.fork_picker);
        assert!(!interactive.fork_last);
        assert_eq!(interactive.fork_session_id, None);
        assert!(!interactive.fork_show_all);
    }

    #[test]
    fn fork_picker_logic_last() {
        let interactive = finalize_fork_from_args(["codex", "fork", "--last"].as_ref());
        assert!(!interactive.fork_picker);
        assert!(interactive.fork_last);
        assert_eq!(interactive.fork_session_id, None);
        assert!(!interactive.fork_show_all);
    }

    #[test]
    fn fork_picker_logic_with_session_id() {
        let interactive = finalize_fork_from_args(["codex", "fork", "1234"].as_ref());
        assert!(!interactive.fork_picker);
        assert!(!interactive.fork_last);
        assert_eq!(interactive.fork_session_id.as_deref(), Some("1234"));
        assert!(!interactive.fork_show_all);
    }

    #[test]
    fn fork_all_flag_sets_show_all() {
        let interactive = finalize_fork_from_args(["codex", "fork", "--all"].as_ref());
        assert!(interactive.fork_picker);
        assert!(interactive.fork_show_all);
    }

    #[test]
    fn app_server_analytics_default_disabled_without_flag() {
        let app_server = app_server_from_args(["codex", "app-server"].as_ref());
        assert!(!app_server.analytics_default_enabled);
    }

    #[test]
    fn app_server_analytics_default_enabled_with_flag() {
        let app_server =
            app_server_from_args(["codex", "app-server", "--analytics-default-enabled"].as_ref());
        assert!(app_server.analytics_default_enabled);
    }

    #[test]
    fn features_enable_parses_feature_name() {
        let cli = MultitoolCli::try_parse_from(["codex", "features", "enable", "unified_exec"])
            .expect("parse should succeed");
        let Some(Subcommand::Features(FeaturesCli { sub })) = cli.subcommand else {
            panic!("expected features subcommand");
        };
        let FeaturesSubcommand::Enable(FeatureSetArgs { feature }) = sub else {
            panic!("expected features enable");
        };
        assert_eq!(feature, "unified_exec");
    }

    #[test]
    fn features_disable_parses_feature_name() {
        let cli = MultitoolCli::try_parse_from(["codex", "features", "disable", "shell_tool"])
            .expect("parse should succeed");
        let Some(Subcommand::Features(FeaturesCli { sub })) = cli.subcommand else {
            panic!("expected features subcommand");
        };
        let FeaturesSubcommand::Disable(FeatureSetArgs { feature }) = sub else {
            panic!("expected features disable");
        };
        assert_eq!(feature, "shell_tool");
    }

    #[test]
    fn feature_toggles_known_features_generate_overrides() {
        let toggles = FeatureToggles {
            enable: vec!["web_search_request".to_string()],
            disable: vec!["unified_exec".to_string()],
        };
        let overrides = toggles.to_overrides().expect("valid features");
        assert_eq!(
            overrides,
            vec![
                "features.web_search_request=true".to_string(),
                "features.unified_exec=false".to_string(),
            ]
        );
    }
}

// LLMOps command handlers
#[cfg(feature = "custom-features")]
async fn run_llmops_command(cmd: LlmOpsCommand) -> Result<(), Box<dyn std::error::Error>> {
    let config = LLMOpsConfig {
        enable_model_versioning: true,
        enable_prompt_versioning: true,
        enable_performance_monitoring: true,
        enable_cost_optimization: true,
        enable_security_hardening: true,
        enable_observability: true,
        max_tokens_per_request: 100000,
        cost_budget_per_hour: 10.0,
        security_level: codex_core::llmops::SecurityLevel::Standard,
        observability_retention_days: 30,
    };

    let manager = LLMOpsManager::new(config)?;

    match cmd.subcommand {
        LlmOpsSubcommand::RegisterModel {
            model_id,
            name,
            version,
            provider,
        } => {
            let provider_enum = match provider.as_str() {
                "openai" => ModelProvider::OpenAI,
                "anthropic" => ModelProvider::Anthropic,
                "google" => ModelProvider::Google,
                _ => ModelProvider::Custom(provider),
            };

            let model = ModelVersion {
                id: model_id,
                model_name: name,
                version,
                provider: provider_enum,
                capabilities: vec![codex_core::llmops::ModelCapability::TextGeneration],
                performance_metrics: Default::default(),
                security_assessment: Default::default(),
                created_at: chrono::Utc::now(),
                deprecated_at: None,
            };

            manager.register_model(model).await?;
            println!("Model registered successfully");
        }
        LlmOpsSubcommand::RegisterPrompt {
            prompt_id,
            name,
            template,
        } => {
            let template_content = std::fs::read_to_string(template)?;
            let prompt = PromptTemplate {
                id: prompt_id,
                name,
                version: "1.0.0".to_string(),
                template: template_content,
                variables: vec![],
                context_requirements: vec![],
                security_constraints: vec![],
                performance_characteristics: Default::default(),
                created_at: chrono::Utc::now(),
            };

            manager.register_prompt(prompt).await?;
            println!("Prompt template registered successfully");
        }
        LlmOpsSubcommand::Status => {
            let status = manager.get_system_status();
            println!("LLMOps Status:");
            println!("  Models: {}", status.model_count);
            println!("  Prompts: {}", status.prompt_count);
            println!("  Cost: ${:.2}", status.cost_metrics.total_cost);
            println!("  Security: {}", status.security_status);
        }
    }

    Ok(())
}

// A2A command handlers
#[cfg(feature = "custom-features")]
async fn run_a2a_command(cmd: A2aCommand) -> Result<(), Box<dyn std::error::Error>> {
    let config = A2AConfig {
        enable_encryption: true,
        enable_authentication: true,
        enable_authorization: true,
        enable_trust_management: true,
        max_message_size: 1048576,
        message_ttl_seconds: 300,
        retry_attempts: 3,
        heartbeat_interval_seconds: 30,
        coordination_timeout_seconds: 60,
    };

    let identity = AgentIdentity {
        id: "cli-agent".to_string(),
        name: "CLI Agent".to_string(),
        role: AgentRole::Orchestrator,
        capabilities: vec![
            AgentCapability::Communication,
            AgentCapability::Coordination,
        ],
        trust_score: 1.0,
        last_seen: chrono::Utc::now(),
    };

    let manager = A2ACommunicationManager::new(config, identity);

    match cmd.subcommand {
        A2aSubcommand::RegisterAgent {
            agent_id,
            role,
            capabilities,
        } => {
            let role_enum = match role.as_str() {
                "orchestrator" => AgentRole::Orchestrator,
                "reviewer" => AgentRole::CodeReviewer,
                _ => AgentRole::Custom(role),
            };

            let caps: Vec<AgentCapability> = capabilities
                .split(',')
                .map(|s| match s.trim() {
                    "communication" => AgentCapability::Communication,
                    "analysis" => AgentCapability::CodeAnalysis,
                    _ => AgentCapability::TaskExecution,
                })
                .collect();

            let agent = AgentIdentity {
                id: agent_id,
                name: format!("Agent {}", agent_id),
                role: role_enum,
                capabilities: caps,
                trust_score: 0.8,
                last_seen: chrono::Utc::now(),
            };

            println!("Agent registered: {}", agent.id);
        }
        A2aSubcommand::SendMessage { agent_id, message } => {
            println!("Message sent to {}: {}", agent_id, message);
        }
        A2aSubcommand::Status => {
            let status = manager.get_system_status().await;
            println!("A2A Status:");
            println!("  Agents: {}", status.agent_count);
            println!("  Connections: {}", status.active_connections);
            println!("  Tasks: {}", status.task_count);
            println!("  Message Queue: {}", status.message_queue_size);
        }
    }

    Ok(())
}

// Skill/MCP command handlers
#[cfg(feature = "custom-features")]
async fn run_skill_mcp_command(cmd: SkillMcpCommand) -> Result<(), Box<dyn std::error::Error>> {
    let config = SkillMCPConfig {
        enable_dynamic_loading: true,
        enable_safe_execution: true,
        enable_resource_management: true,
        enable_performance_monitoring: true,
        max_concurrent_skills: 10,
        skill_timeout_seconds: 300,
        mcp_context_budget: 10000,
        security_level: codex_core::skill_mcp_integration::MCPSecurityLevel::Standard,
        observability_enabled: true,
    };

    let manager = SkillMCPIntegrationManager::new(config);

    match cmd.subcommand {
        SkillMcpSubcommand::RegisterSkill { skill_file } => {
            let content = std::fs::read_to_string(skill_file)?;
            let skill: SkillDefinition = serde_json::from_str(&content)?;
            manager.register_skill(skill).await?;
            println!("Skill registered successfully");
        }
        SkillMcpSubcommand::RegisterResource { uri, name } => {
            let resource = MCPResource {
                uri,
                name,
                description: format!("Resource {}", name),
                mime_type: "application/json".to_string(),
                metadata: std::collections::HashMap::new(),
                access_control: Default::default(),
                caching_policy: Default::default(),
            };
            manager.register_mcp_resource(resource).await?;
            println!("MCP resource registered successfully");
        }
        SkillMcpSubcommand::ExecuteSkill { skill_id, input } => {
            let input_data: serde_json::Value = serde_json::from_str(&input)?;
            let result = manager.execute_skill(&skill_id, input_data).await?;
            println!("Skill execution result: {}", result);
        }
        SkillMcpSubcommand::Status => {
            let status = manager.get_system_status().await;
            println!("Skill/MCP Status:");
            println!("  Skills: {}", status.skill_count);
            println!("  Resources: {}", status.resource_count);
            println!("  Tools: {}", status.tool_count);
            println!(
                "  Context Usage: {}/{}",
                status.context_usage.current_tokens, status.context_usage.max_tokens
            );
        }
    }

    Ok(())
}

// Orchestration command handlers
#[cfg(feature = "custom-features")]
async fn run_orchestrate_command(
    cmd: OrchestrateCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = AutonomousOrchestrationConfig {
        enable_task_decomposition: true,
        enable_agent_coordination: true,
        enable_token_management: true,
        enable_terminal_invocation: true,
        enable_loose_coupling: true,
        enable_self_healing: true,
        max_concurrent_tasks: 5,
        task_timeout_seconds: 3600,
        coordination_timeout_seconds: 300,
        token_budget_per_task: 10000,
        terminal_pool_size: 3,
        healing_retry_attempts: 3,
    };

    let manager = AutonomousOrchestrationManager::new(config);

    match cmd.subcommand {
        OrchestrateSubcommand::SubmitTask {
            name,
            description,
            priority,
            capabilities,
        } => {
            let priority_enum = match priority.as_str() {
                "critical" => TaskPriority::Critical,
                "high" => TaskPriority::High,
                "low" => TaskPriority::Low,
                "trivial" => TaskPriority::Trivial,
                _ => TaskPriority::Medium,
            };

            let caps: Vec<AgentCapability> = capabilities
                .as_ref()
                .map(|s| {
                    s.split(',')
                        .map(|c| match c.trim() {
                            "communication" => AgentCapability::Communication,
                            "analysis" => AgentCapability::CodeAnalysis,
                            _ => AgentCapability::TaskExecution,
                        })
                        .collect()
                })
                .unwrap_or_default();

            let request = TaskRequest {
                name,
                description,
                priority: priority_enum,
                required_capabilities: caps,
                resource_requirements: Default::default(),
                dependencies: vec![],
                deadline: None,
                metadata: std::collections::HashMap::new(),
            };

            let task_id = manager.submit_task(request).await?;
            println!("Task submitted: {}", task_id);
        }
        OrchestrateSubcommand::TaskStatus { task_id } => {
            let status = manager.get_task_status(&task_id).await?;
            println!("Task Status:");
            println!("  ID: {}", status.task_id);
            println!("  Status: {:?}", status.status);
            println!("  Progress: {:.1}%", status.progress * 100.0);
            println!("  Assigned Agent: {:?}", status.assigned_agent);
            println!("  Subtasks: {}", status.subtasks.len());
        }
        OrchestrateSubcommand::Status => {
            let status = manager.get_system_status().await;
            println!("Orchestration Status:");
            println!("  Tasks: {}", status.task_count);
            println!("  Agents: {}", status.agent_count);
            println!("  Active Tasks: {}", status.active_tasks);
            println!(
                "  Token Usage: {:.1}%",
                status.token_usage.utilization_percent
            );
            println!("  Terminal Sessions: {}", status.terminal_sessions);
            println!("  System Health: {:?}", status.system_health.overall_status);
        }
    }

    Ok(())
}
