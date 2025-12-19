use strum::IntoEnumIterator;
use strum_macros::AsRefStr;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

/// Commands that can be invoked by starting a message with a leading slash.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, AsRefStr, IntoStaticStr,
)]
#[strum(serialize_all = "kebab-case")]
pub enum SlashCommand {
    // DO NOT ALPHA-SORT! Enum order is presentation order in the popup, so
    // more frequently used commands should be listed first.
    Model,
    Approvals,
    Review,
    Delegate,
    Orchestrate,
    CentralDev,
    ParallelDev,
    Research,
    Plan,
    Qc,
    Hook,
    New,
    Init,
    Compact,
    Undo,
    Diff,
    Mention,
    Status,
    Mcp,
    Logout,
    Quit,
    Exit,
    Feedback,
    Rollout,
    TestApproval,
}

impl SlashCommand {
    /// User-visible description shown in the popup.
    pub fn description(self) -> &'static str {
        match self {
            SlashCommand::Feedback => "send logs to maintainers",
            SlashCommand::New => "start a new chat during a conversation",
            SlashCommand::Init => "create an AGENTS.md file with instructions for Codex",
            SlashCommand::Compact => "summarize conversation to prevent hitting the context limit",
            SlashCommand::Review => "review my current changes and find issues",
            SlashCommand::Delegate => "delegate a task to a sub-agent using natural language",
            SlashCommand::Orchestrate => "kick off auto orchestration / supervisor flows",
            SlashCommand::CentralDev => {
                "centralized development with main agent coordinating sub-agents"
            }
            SlashCommand::ParallelDev => "parallel development using git worktrees for each agent",
            SlashCommand::Research => "conduct deep research (Gemini, MCP, web)",
            SlashCommand::Plan => "create execution plan with approval gates",
            SlashCommand::Qc => "run quality control agent for code analysis and optimization",
            SlashCommand::Hook => "trigger webhook integrations (Slack, etc.)",
            SlashCommand::Undo => "ask Codex to undo a turn",
            SlashCommand::Quit | SlashCommand::Exit => "exit Codex",
            SlashCommand::Diff => "show git diff (including untracked files)",
            SlashCommand::Mention => "mention a file",
            SlashCommand::Status => "show current session configuration and token usage",
            SlashCommand::Model => "choose what model and reasoning effort to use",
            SlashCommand::Approvals => "choose what Codex can do without approval",
            SlashCommand::Mcp => "list configured MCP tools",
            SlashCommand::Logout => "log out of Codex",
            SlashCommand::Rollout => "print the rollout file path",
            SlashCommand::TestApproval => "test approval request",
        }
    }

    /// Command string without the leading '/'. Provided for compatibility with
    /// existing code that expects a method named `command()`.
    pub fn command(self) -> &'static str {
        self.into()
    }

    /// Whether this command can be run while a task is in progress.
    pub fn available_during_task(self) -> bool {
        match self {
            SlashCommand::New
            | SlashCommand::Init
            | SlashCommand::Compact
            | SlashCommand::Undo
            | SlashCommand::Model
            | SlashCommand::Approvals
            | SlashCommand::Review
            | SlashCommand::Delegate
            | SlashCommand::Orchestrate
            | SlashCommand::Research
            | SlashCommand::Plan
            | SlashCommand::Hook
            | SlashCommand::Logout => false,
            SlashCommand::Diff
            | SlashCommand::Mention
            | SlashCommand::Status
            | SlashCommand::Mcp
            | SlashCommand::Feedback
            | SlashCommand::Quit
            | SlashCommand::Exit => true,
            SlashCommand::Rollout => true,
            SlashCommand::TestApproval => true,
            SlashCommand::CentralDev => true,
            SlashCommand::ParallelDev => true,
            SlashCommand::Qc => true,
        }
    }

    /// English aliases for the command.
    pub fn aliases(self) -> &'static [&'static str] {
        match self {
            SlashCommand::Quit | SlashCommand::Exit => &["exit", "quit", "bye"],
            SlashCommand::Review => &["check", "audit"],
            SlashCommand::Delegate => &["assign", "handover"],
            SlashCommand::Research => &["search", "investigate", "deepsearch"],
            SlashCommand::Plan => &["schedule", "blueprint"],
            SlashCommand::New => &["clear", "reset"],
            SlashCommand::Undo => &["back", "revert"],
            _ => &[],
        }
    }

    /// Japanese aliases for the command.
    pub fn japanese_aliases(self) -> &'static [&'static str] {
        match self {
            SlashCommand::Model => &["モデル", "設定"],
            SlashCommand::Approvals => &["承認", "許可"],
            SlashCommand::Review => &["レビュー", "添削", "修正"],
            SlashCommand::Delegate => &["委譲", "依頼"],
            SlashCommand::Orchestrate => &["オーケストレーション", "指揮"],
            SlashCommand::Research => &["調査", "検索", "リサーチ"],
            SlashCommand::Plan => &["計画", "プラン"],
            SlashCommand::Qc => &["品質", "検証"],
            SlashCommand::New => &["新規", "クリア"],
            SlashCommand::Undo => &["元に戻す", "取り消し"],
            SlashCommand::Diff => &["差分", "変更"],
            SlashCommand::Status => &["ステータス", "状態"],
            SlashCommand::Mcp => &["ツール"],
            SlashCommand::Quit | SlashCommand::Exit => &["終了", "閉じる"],
            SlashCommand::Feedback => &["フィードバック", "報告"],
            _ => &[],
        }
    }

    fn is_visible(self) -> bool {
        match self {
            SlashCommand::Rollout | SlashCommand::TestApproval => cfg!(debug_assertions),
            _ => true,
        }
    }
}

/// Return all built-in commands in a Vec paired with their command string.
pub fn built_in_slash_commands() -> Vec<(&'static str, SlashCommand)> {
    SlashCommand::iter()
        .filter(|command| command.is_visible())
        .map(|c| (c.command(), c))
        .collect()
}
