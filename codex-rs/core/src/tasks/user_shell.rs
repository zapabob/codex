use std::sync::Arc;

use async_trait::async_trait;
use codex_protocol::models::ShellToolCallParams;
use codex_protocol::user_input::UserInput;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::error;
use uuid::Uuid;

use crate::codex::TurnContext;
use crate::protocol::EventMsg;
use crate::protocol::TaskStartedEvent;
<<<<<<< HEAD
=======
use crate::sandboxing::ExecEnv;
use crate::sandboxing::SandboxPermissions;
>>>>>>> upstream/main
use crate::state::TaskKind;
use crate::tools::context::ToolPayload;
use crate::tools::parallel::ToolCallRuntime;
use crate::tools::router::ToolCall;
use crate::tools::router::ToolRouter;
use crate::turn_diff_tracker::TurnDiffTracker;

use super::SessionTask;
use super::SessionTaskContext;

const USER_SHELL_TOOL_NAME: &str = "local_shell";

#[derive(Clone)]
pub(crate) struct UserShellCommandTask {
    command: String,
}

impl UserShellCommandTask {
    pub(crate) fn new(command: String) -> Self {
        Self { command }
    }
}

#[async_trait]
impl SessionTask for UserShellCommandTask {
    fn kind(&self) -> TaskKind {
        TaskKind::Regular
    }

    async fn run(
        self: Arc<Self>,
        session: Arc<SessionTaskContext>,
        turn_context: Arc<TurnContext>,
        _input: Vec<UserInput>,
        cancellation_token: CancellationToken,
    ) -> Option<String> {
        let event = EventMsg::TaskStarted(TaskStartedEvent {
            model_context_window: turn_context.client.get_model_context_window(),
        });
        let session = session.clone_session();
        session.send_event(turn_context.as_ref(), event).await;

        // Execute the user's script under their default shell when known; this
        // allows commands that use shell features (pipes, &&, redirects, etc.).
        // We do not source rc files or otherwise reformat the script.
<<<<<<< HEAD
        let shell_invocation = match session.user_shell() {
            crate::shell::Shell::Zsh(zsh) => vec![
                zsh.shell_path.clone(),
                "-lc".to_string(),
                self.command.clone(),
            ],
            crate::shell::Shell::Bash(bash) => vec![
                bash.shell_path.clone(),
                "-lc".to_string(),
                self.command.clone(),
            ],
            crate::shell::Shell::PowerShell(ps) => vec![
                ps.exe.clone(),
                "-NoProfile".to_string(),
                "-Command".to_string(),
                self.command.clone(),
            ],
            crate::shell::Shell::Unknown => {
                shlex::split(&self.command).unwrap_or_else(|| vec![self.command.clone()])
            }
=======
        let use_login_shell = true;
        let command = session
            .user_shell()
            .derive_exec_args(&self.command, use_login_shell);

        let call_id = Uuid::new_v4().to_string();
        let raw_command = self.command.clone();
        let cwd = turn_context.cwd.clone();

        let parsed_cmd = parse_command(&command);
        session
            .send_event(
                turn_context.as_ref(),
                EventMsg::ExecCommandBegin(ExecCommandBeginEvent {
                    call_id: call_id.clone(),
                    process_id: None,
                    turn_id: turn_context.sub_id.clone(),
                    command: command.clone(),
                    cwd: cwd.clone(),
                    parsed_cmd: parsed_cmd.clone(),
                    source: ExecCommandSource::UserShell,
                    interaction_input: None,
                }),
            )
            .await;

        let exec_env = ExecEnv {
            command: command.clone(),
            cwd: cwd.clone(),
            env: create_env(&turn_context.shell_environment_policy),
            // TODO(zhao-oai): Now that we have ExecExpiration::Cancellation, we
            // should use that instead of an "arbitrarily large" timeout here.
            expiration: USER_SHELL_TIMEOUT_MS.into(),
            sandbox: SandboxType::None,
            sandbox_permissions: SandboxPermissions::UseDefault,
            justification: None,
            arg0: None,
>>>>>>> upstream/main
        };

        let params = ShellToolCallParams {
            command: shell_invocation,
            workdir: None,
            timeout_ms: None,
            with_escalated_permissions: None,
            justification: None,
        };

        let tool_call = ToolCall {
            tool_name: USER_SHELL_TOOL_NAME.to_string(),
            call_id: Uuid::new_v4().to_string(),
            payload: ToolPayload::LocalShell { params },
        };

        let router = Arc::new(ToolRouter::from_config(&turn_context.tools_config, None));
        let tracker = Arc::new(Mutex::new(TurnDiffTracker::new()));
        let runtime = ToolCallRuntime::new(
            Arc::clone(&router),
            Arc::clone(&session),
            Arc::clone(&turn_context),
            Arc::clone(&tracker),
        );

        if let Err(err) = runtime
            .handle_tool_call(tool_call, cancellation_token)
            .await
        {
            error!("user shell command failed: {err:?}");
        }
        None
    }
}
