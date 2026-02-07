use codex_core::protocol::ExecCommandSource;
use codex_protocol::parse_command::ParsedCommand;

pub(crate) struct UnifiedExecProcessSummary {
    #[allow(dead_code)]
    pub(crate) key: String,
    #[allow(dead_code)]
    pub(crate) call_id: String,
    #[allow(dead_code)]
    pub(crate) command_display: String,
}

pub(crate) struct UnifiedExecWaitState {
    pub(crate) command_display: String,
}

impl UnifiedExecWaitState {
    pub(crate) fn new(command_display: String) -> Self {
        Self { command_display }
    }

    pub(crate) fn is_duplicate(&self, command_display: &str) -> bool {
        self.command_display == command_display
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UnifiedExecWaitStreak {
    #[allow(dead_code)]
    pub(crate) process_id: String,
    pub(crate) command_display: Option<String>,
}

impl UnifiedExecWaitStreak {
    pub(crate) fn new(process_id: String, command_display: Option<String>) -> Self {
        Self {
            process_id,
            command_display: command_display.filter(|display| !display.is_empty()),
        }
    }

    pub(crate) fn update_command_display(&mut self, command_display: Option<String>) {
        if self.command_display.is_some() {
            return;
        }
        self.command_display = command_display.filter(|display| !display.is_empty());
    }
}

pub(crate) fn is_unified_exec_source(source: ExecCommandSource) -> bool {
    matches!(
        source,
        ExecCommandSource::UnifiedExecStartup | ExecCommandSource::UnifiedExecInteraction
    )
}

pub(crate) fn is_standard_tool_call(parsed_cmd: &[ParsedCommand]) -> bool {
    !parsed_cmd.is_empty()
        && parsed_cmd
            .iter()
            .all(|parsed| !matches!(parsed, ParsedCommand::Unknown { .. }))
}
