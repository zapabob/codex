use clap::Parser;
use clap::ValueEnum;
use codex_common::CliConfigOverrides;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Cli {
    /// Action to perform. If omitted, runs a new non-interactive session.
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Optional image(s) to attach to the initial prompt.
    #[arg(long = "image", short = 'i', value_name = "FILE", value_delimiter = ',', num_args = 1..)]
    pub images: Vec<PathBuf>,

    /// Model the agent should use.
    #[arg(long, short = 'm')]
    pub model: Option<String>,

    #[arg(long = "oss", default_value_t = false)]
    pub oss: bool,

    /// OSS provider to use (lmstudio or ollama)
    #[arg(long = "oss-provider")]
    pub oss_provider: Option<String>,

    /// Select the sandbox policy to use when executing model-generated shell
    /// commands.
    #[arg(long = "sandbox", short = 's', value_enum)]
    pub sandbox_mode: Option<codex_common::SandboxModeCliArg>,

    /// Configuration profile from config.toml to specify default options.
    #[arg(long = "profile", short = 'p')]
    pub config_profile: Option<String>,

    /// Convenience alias for low-friction sandboxed automatic execution (-a on-failure, --sandbox workspace-write).
    #[arg(long = "full-auto", default_value_t = false)]
    pub full_auto: bool,

    /// Skip all confirmation prompts and execute commands without sandboxing.
    /// EXTREMELY DANGEROUS. Intended solely for running in environments that are externally sandboxed.
    #[arg(
        long = "dangerously-bypass-approvals-and-sandbox",
        alias = "yolo",
        default_value_t = false,
        conflicts_with = "full_auto"
    )]
    pub dangerously_bypass_approvals_and_sandbox: bool,

    /// Tell the agent to use the specified directory as its working root.
    #[clap(long = "cd", short = 'C', value_name = "DIR")]
    pub cwd: Option<PathBuf>,

    /// Allow running Codex outside a Git repository.
    #[arg(long = "skip-git-repo-check", default_value_t = false)]
    pub skip_git_repo_check: bool,

    /// Additional directories to include in context
    #[arg(long = "add-dir", value_name = "DIR")]
    pub add_dir: Vec<PathBuf>,

    /// Path to a JSON Schema file describing the model's final response shape.
    #[arg(long = "output-schema", value_name = "FILE")]
    pub output_schema: Option<PathBuf>,

    /// Complexity threshold for triggering auto-orchestration (0.0-1.0).
    #[arg(long = "auto-threshold", default_value_t = 0.7)]
    pub auto_threshold: f64,

    /// Auto-orchestration strategy when threshold is met.
    #[arg(long = "strategy", value_enum, default_value = "hybrid")]
    pub strategy: OrchestrationStrategy,

    /// Explicit skills to bias orchestrator selection (comma-separated).
    #[arg(long = "skills", value_delimiter = ',', value_name = "SKILL,SKILL")]
    pub skills: Vec<String>,

    #[clap(skip)]
    pub config_overrides: CliConfigOverrides,

    /// Specifies color settings for use in the output.
    #[arg(long = "color", value_enum, default_value_t = Color::Auto)]
    pub color: Color,

    /// Print events to stdout as JSONL.
    #[arg(long = "json", alias = "experimental-json", default_value_t = false)]
    pub json: bool,

    /// Specifies file where the last message from the agent should be written.
    #[arg(long = "output-last-message", short = 'o', value_name = "FILE")]
    pub last_message_file: Option<PathBuf>,

    /// Initial instructions for the agent. If not provided as an argument (or
    /// if `-` is used), instructions are read from stdin.
    #[arg(value_name = "PROMPT", value_hint = clap::ValueHint::Other)]
    pub prompt: Option<String>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Resume a previous session by id or pick the most recent with --last.
    Resume(ResumeArgs),
    /// Request a code review
    Review(ReviewArgs),
}

#[derive(Parser, Debug)]
pub struct ReviewArgs {
    /// Review uncommitted changes
    #[arg(long, default_value_t = false)]
    pub uncommitted: bool,

    /// Base branch to compare against
    #[arg(long)]
    pub base: Option<String>,

    /// Specific commit to review
    #[arg(long)]
    pub commit: Option<String>,

    /// Title for the commit being reviewed
    #[arg(long)]
    pub commit_title: Option<String>,

    /// Custom review instructions
    #[arg(value_name = "PROMPT", value_hint = clap::ValueHint::Other)]
    pub prompt: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ResumeArgs {
    /// Conversation/session id (UUID). When provided, resumes this session.
    /// If omitted, use --last to pick the most recent recorded session.
    #[arg(value_name = "SESSION_ID")]
    pub session_id: Option<String>,

    /// Resume the most recent recorded session (newest) without specifying an id.
    #[arg(long = "last", default_value_t = false, conflicts_with = "session_id")]
    pub last: bool,

    /// Prompt to send after resuming the session. If `-` is used, read from stdin.
    #[arg(value_name = "PROMPT", value_hint = clap::ValueHint::Other)]
    pub prompt: Option<String>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum Color {
    Always,
    Never,
    #[default]
    Auto,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum OrchestrationStrategy {
    Sequential,
    Parallel,
    #[default]
    Hybrid,
}
