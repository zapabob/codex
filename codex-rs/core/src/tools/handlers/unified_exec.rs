<<<<<<< HEAD
use async_trait::async_trait;
use serde::Deserialize;

=======
>>>>>>> upstream/main
use crate::function_tool::FunctionCallError;
use crate::protocol::EventMsg;
<<<<<<< HEAD
use crate::protocol::ExecCommandOutputDeltaEvent;
use crate::protocol::ExecOutputStream;
=======
use crate::protocol::ExecCommandSource;
use crate::protocol::TerminalInteractionEvent;
use crate::sandboxing::SandboxPermissions;
use crate::shell::Shell;
use crate::shell::get_shell_by_model_provided_path;
>>>>>>> upstream/main
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolOutput;
use crate::tools::context::ToolPayload;
use crate::tools::events::ToolEmitter;
use crate::tools::events::ToolEventCtx;
use crate::tools::events::ToolEventStage;
use crate::tools::registry::ToolHandler;
use crate::tools::registry::ToolKind;
use crate::unified_exec::ExecCommandRequest;
use crate::unified_exec::UnifiedExecContext;
use crate::unified_exec::UnifiedExecResponse;
use crate::unified_exec::UnifiedExecSessionManager;
use crate::unified_exec::WriteStdinRequest;
<<<<<<< HEAD
=======
use async_trait::async_trait;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;
>>>>>>> upstream/main

pub struct UnifiedExecHandler;

#[derive(Debug, Deserialize)]
struct ExecCommandArgs {
    cmd: String,
    #[serde(default = "default_shell")]
    shell: String,
    #[serde(default = "default_login")]
    login: bool,
    #[serde(default)]
    yield_time_ms: Option<u64>,
    #[serde(default)]
    max_output_tokens: Option<usize>,
<<<<<<< HEAD
=======
    #[serde(default)]
    sandbox_permissions: SandboxPermissions,
    #[serde(default)]
    justification: Option<String>,
>>>>>>> upstream/main
}

#[derive(Debug, Deserialize)]
struct WriteStdinArgs {
    session_id: i32,
    #[serde(default)]
    chars: String,
    #[serde(default)]
    yield_time_ms: Option<u64>,
    #[serde(default)]
    max_output_tokens: Option<usize>,
}

fn default_shell() -> String {
    "/bin/bash".to_string()
}

fn default_login() -> bool {
    true
}

#[async_trait]
impl ToolHandler for UnifiedExecHandler {
    fn kind(&self) -> ToolKind {
        ToolKind::Function
    }

    fn matches_kind(&self, payload: &ToolPayload) -> bool {
        matches!(
            payload,
            ToolPayload::Function { .. } | ToolPayload::UnifiedExec { .. }
        )
    }

<<<<<<< HEAD
=======
    async fn is_mutating(&self, invocation: &ToolInvocation) -> bool {
        let (ToolPayload::Function { arguments } | ToolPayload::UnifiedExec { arguments }) =
            &invocation.payload
        else {
            return true;
        };

        let Ok(params) = serde_json::from_str::<ExecCommandArgs>(arguments) else {
            return true;
        };
        let command = get_command(&params, invocation.session.user_shell());
        !is_known_safe_command(&command)
    }

>>>>>>> upstream/main
    async fn handle(&self, invocation: ToolInvocation) -> Result<ToolOutput, FunctionCallError> {
        let ToolInvocation {
            session,
            turn,
            call_id,
            tool_name,
            payload,
            ..
        } = invocation;

        let arguments = match payload {
            ToolPayload::Function { arguments } => arguments,
            ToolPayload::UnifiedExec { arguments } => arguments,
            _ => {
                return Err(FunctionCallError::RespondToModel(
                    "unified_exec handler received unsupported payload".to_string(),
                ));
            }
        };

        let manager: &UnifiedExecSessionManager = &session.services.unified_exec_manager;
        let context = UnifiedExecContext::new(session.clone(), turn.clone(), call_id.clone());

        let response = match tool_name.as_str() {
            "exec_command" => {
                let args: ExecCommandArgs = serde_json::from_str(&arguments).map_err(|err| {
                    FunctionCallError::RespondToModel(format!(
                        "failed to parse exec_command arguments: {err:?}"
                    ))
                })?;
<<<<<<< HEAD
=======
                let process_id = manager.allocate_process_id().await;
                let command = get_command(&args, session.user_shell());

                let ExecCommandArgs {
                    workdir,
                    yield_time_ms,
                    max_output_tokens,
                    sandbox_permissions,
                    justification,
                    ..
                } = args;

                if sandbox_permissions.requires_escalated_permissions()
                    && !matches!(
                        context.turn.approval_policy,
                        codex_protocol::protocol::AskForApproval::OnRequest
                    )
                {
                    manager.release_process_id(&process_id).await;
                    return Err(FunctionCallError::RespondToModel(format!(
                        "approval policy is {policy:?}; reject command — you cannot ask for escalated permissions if the approval policy is {policy:?}",
                        policy = context.turn.approval_policy
                    )));
                }

                let workdir = workdir.filter(|value| !value.is_empty());

                let workdir = workdir.map(|dir| context.turn.resolve_path(Some(dir)));
                let cwd = workdir.clone().unwrap_or_else(|| context.turn.cwd.clone());

                if let Some(output) = intercept_apply_patch(
                    &command,
                    &cwd,
                    Some(yield_time_ms),
                    context.session.as_ref(),
                    context.turn.as_ref(),
                    Some(&tracker),
                    &context.call_id,
                    tool_name.as_str(),
                )
                .await?
                {
                    manager.release_process_id(&process_id).await;
                    return Ok(output);
                }
>>>>>>> upstream/main

                let event_ctx = ToolEventCtx::new(
                    context.session.as_ref(),
                    context.turn.as_ref(),
                    &context.call_id,
                    None,
                );
<<<<<<< HEAD
                let emitter =
                    ToolEmitter::unified_exec(args.cmd.clone(), context.turn.cwd.clone(), true);
=======
                let emitter = ToolEmitter::unified_exec(
                    &command,
                    cwd.clone(),
                    ExecCommandSource::UnifiedExecStartup,
                    Some(process_id.clone()),
                );
>>>>>>> upstream/main
                emitter.emit(event_ctx, ToolEventStage::Begin).await;

                manager
                    .exec_command(
                        ExecCommandRequest {
<<<<<<< HEAD
                            command: &args.cmd,
                            shell: &args.shell,
                            login: args.login,
                            yield_time_ms: args.yield_time_ms,
                            max_output_tokens: args.max_output_tokens,
=======
                            command,
                            process_id,
                            yield_time_ms,
                            max_output_tokens,
                            workdir,
                            sandbox_permissions,
                            justification,
>>>>>>> upstream/main
                        },
                        &context,
                    )
                    .await
                    .map_err(|err| {
                        FunctionCallError::RespondToModel(format!("exec_command failed: {err:?}"))
                    })?
            }
            "write_stdin" => {
                let args: WriteStdinArgs = serde_json::from_str(&arguments).map_err(|err| {
                    FunctionCallError::RespondToModel(format!(
                        "failed to parse write_stdin arguments: {err:?}"
                    ))
                })?;
                let response = manager
                    .write_stdin(WriteStdinRequest {
<<<<<<< HEAD
                        session_id: args.session_id,
=======
                        process_id: &args.session_id.to_string(),
>>>>>>> upstream/main
                        input: &args.chars,
                        yield_time_ms: args.yield_time_ms,
                        max_output_tokens: args.max_output_tokens,
                    })
                    .await
                    .map_err(|err| {
                        FunctionCallError::RespondToModel(format!("write_stdin failed: {err:?}"))
                    })?;

                let interaction = TerminalInteractionEvent {
                    call_id: response.event_call_id.clone(),
                    process_id: args.session_id.to_string(),
                    stdin: args.chars.clone(),
                };
                session
                    .send_event(turn.as_ref(), EventMsg::TerminalInteraction(interaction))
                    .await;

                response
            }
            other => {
                return Err(FunctionCallError::RespondToModel(format!(
                    "unsupported unified exec function {other}"
                )));
            }
        };

        let content = format_response(&response);

        Ok(ToolOutput::Function {
            content,
            content_items: None,
            success: Some(true),
        })
    }
}

<<<<<<< HEAD
=======
fn get_command(args: &ExecCommandArgs, session_shell: Arc<Shell>) -> Vec<String> {
    let model_shell = args.shell.as_ref().map(|shell_str| {
        let mut shell = get_shell_by_model_provided_path(&PathBuf::from(shell_str));
        shell.shell_snapshot = None;
        shell
    });

    let shell = model_shell.as_ref().unwrap_or(session_shell.as_ref());

    shell.derive_exec_args(&args.cmd, args.login)
}

>>>>>>> upstream/main
fn format_response(response: &UnifiedExecResponse) -> String {
    let mut sections = Vec::new();

    if !response.chunk_id.is_empty() {
        sections.push(format!("Chunk ID: {}", response.chunk_id));
    }

    let wall_time_seconds = response.wall_time.as_secs_f64();
    sections.push(format!("Wall time: {wall_time_seconds:.4} seconds"));

    if let Some(exit_code) = response.exit_code {
        sections.push(format!("Process exited with code {exit_code}"));
    }

    if let Some(session_id) = response.session_id {
        sections.push(format!("Process running with session ID {session_id}"));
    }

    if let Some(original_token_count) = response.original_token_count {
        sections.push(format!("Original token count: {original_token_count}"));
    }

    sections.push("Output:".to_string());
    sections.push(response.output.clone());

    sections.join("\n")
}
<<<<<<< HEAD
=======

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::default_user_shell;
    use std::sync::Arc;

    #[test]
    fn test_get_command_uses_default_shell_when_unspecified() {
        let json = r#"{"cmd": "echo hello"}"#;

        let args: ExecCommandArgs =
            serde_json::from_str(json).expect("deserialize ExecCommandArgs");

        assert!(args.shell.is_none());

        let command = get_command(&args, Arc::new(default_user_shell()));

        assert_eq!(command.len(), 3);
        assert_eq!(command[2], "echo hello");
    }

    #[test]
    fn test_get_command_respects_explicit_bash_shell() {
        let json = r#"{"cmd": "echo hello", "shell": "/bin/bash"}"#;

        let args: ExecCommandArgs =
            serde_json::from_str(json).expect("deserialize ExecCommandArgs");

        assert_eq!(args.shell.as_deref(), Some("/bin/bash"));

        let command = get_command(&args, Arc::new(default_user_shell()));

        assert_eq!(command.last(), Some(&"echo hello".to_string()));
        if command
            .iter()
            .any(|arg| arg.eq_ignore_ascii_case("-Command"))
        {
            assert!(command.contains(&"-NoProfile".to_string()));
        }
    }

    #[test]
    fn test_get_command_respects_explicit_powershell_shell() {
        let json = r#"{"cmd": "echo hello", "shell": "powershell"}"#;

        let args: ExecCommandArgs =
            serde_json::from_str(json).expect("deserialize ExecCommandArgs");

        assert_eq!(args.shell.as_deref(), Some("powershell"));

        let command = get_command(&args, Arc::new(default_user_shell()));

        assert_eq!(command[2], "echo hello");
    }

    #[test]
    fn test_get_command_respects_explicit_cmd_shell() {
        let json = r#"{"cmd": "echo hello", "shell": "cmd"}"#;

        let args: ExecCommandArgs =
            serde_json::from_str(json).expect("deserialize ExecCommandArgs");

        assert_eq!(args.shell.as_deref(), Some("cmd"));

        let command = get_command(&args, Arc::new(default_user_shell()));

        assert_eq!(command[2], "echo hello");
    }
}
>>>>>>> upstream/main
