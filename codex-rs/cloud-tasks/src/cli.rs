use clap::Args;
use clap::Parser;
use codex_utils_cli::CliConfigOverrides;

#[derive(Parser, Debug, Default)]
#[command(version)]
pub struct Cli {
    #[clap(skip)]
    pub config_overrides: CliConfigOverrides,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Submit a new Codex Cloud task without launching the TUI.
    Exec(ExecCommand),
    /// Show status for a Codex Cloud task.
    Status(StatusCommand),
    /// Show diff output for a Codex Cloud task.
    Diff(DiffCommand),
    /// List Codex Cloud tasks.
    List(ListCommand),
    /// Apply a Codex Cloud task diff.
    Apply(ApplyCommand),
}

#[derive(Debug, Args)]
pub struct ExecCommand {
    /// Task prompt to run in Codex Cloud.
    #[arg(value_name = "QUERY")]
    pub query: Option<String>,

    /// Target environment identifier (see `codex cloud` to browse).
    #[arg(long = "env", value_name = "ENV_ID")]
    pub environment: String,

    /// Number of assistant attempts (best-of-N).
    #[arg(
        long = "attempts",
        default_value_t = 1usize,
        value_parser = parse_attempts
    )]
    pub attempts: usize,

    /// Git branch to run in Codex Cloud (defaults to current branch).
    #[arg(long = "branch", value_name = "BRANCH")]
    pub branch: Option<String>,
}

#[derive(Debug, Args)]
pub struct StatusCommand {
    /// Task id or URL to check.
    #[arg(value_name = "TASK_ID")]
    pub task_id: String,
}

#[derive(Debug, Args)]
pub struct DiffCommand {
    /// Task id or URL to diff.
    #[arg(value_name = "TASK_ID")]
    pub task_id: String,

    /// Attempt number to show (defaults to 1).
    #[arg(long = "attempt", value_name = "N")]
    pub attempt: Option<usize>,
}

#[derive(Debug, Args)]
pub struct ApplyCommand {
    /// Task id or URL to apply.
    #[arg(value_name = "TASK_ID")]
    pub task_id: String,

    /// Attempt number to apply (defaults to 1).
    #[arg(long = "attempt", value_name = "N")]
    pub attempt: Option<usize>,
}

#[derive(Debug, Args)]
pub struct ListCommand {
    /// Filter by environment identifier.
    #[arg(long = "env", value_name = "ENV_ID")]
    pub environment: Option<String>,

    /// Limit the number of tasks to list (default 20).
    #[arg(long = "limit", default_value_t = 20usize)]
    pub limit: usize,
}

fn parse_attempts(input: &str) -> Result<usize, String> {
    let value: usize = input
        .parse()
        .map_err(|_| "attempts must be an integer between 1 and 4".to_string())?;
    if (1..=4).contains(&value) {
        Ok(value)
    } else {
        Err("attempts must be between 1 and 4".to_string())
    }
}
