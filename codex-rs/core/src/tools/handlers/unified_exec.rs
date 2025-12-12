use async_trait::async_trait;
use serde::Deserialize;

use crate::function_tool::FunctionCallError;
use crate::protocol::EventMsg;
use crate::protocol::TerminalInteractionEvent;
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

pub struct UnifiedExecHandler;

#[derive(Debug, Deserialize)]
struct ExecCommandArgs {
    cmd: String,
    #[serde(default)]
    workdir: Option<String>,
    #[serde(default)]
    yield_time_ms: Option<u64>,
    #[serde(default)]
    max_output_tokens: Option<usize>,
    #[serde(default)]
    sandbox_permissions: crate::sandboxing::SandboxPermissions,
    #[serde(default)]
    justification: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WriteStdinArgs {
    process_id: String,
    #[serde(default)]
    chars: String,
    #[serde(default)]
    yield_time_ms: Option<u64>,
    #[serde(default)]
    max_output_tokens: Option<usize>,
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

    async fn is_mutating(&self, _invocation: &ToolInvocation) -> bool {
        true
    }

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

        let response: UnifiedExecResponse = match tool_name.as_str() {
            "exec_command" => {
                let args: ExecCommandArgs = serde_json::from_str(&arguments).map_err(|err| {
                    FunctionCallError::RespondToModel(format!(
                        "failed to parse exec_command arguments: {err:?}"
                    ))
                })?;

                let process_id = manager.allocate_process_id().await;
                let workdir = args
                    .workdir
                    .clone()
                    .filter(|s| !s.is_empty())
                    .map(|dir| context.turn.resolve_path(Some(dir)));
                let cwd = workdir.clone().unwrap_or_else(|| context.turn.cwd.clone());

                let emitter = ToolEmitter::unified_exec(
                    &[args.cmd.clone()],
                    cwd.clone(),
                    crate::protocol::ExecCommandSource::UnifiedExecStartup,
                    Some(process_id.clone()),
                );
                let event_ctx = ToolEventCtx::new(
                    context.session.as_ref(),
                    context.turn.as_ref(),
                    &context.call_id,
                    None,
                );
                emitter.emit(event_ctx, ToolEventStage::Begin).await;

                manager
                    .exec_command(
                        ExecCommandRequest {
                            command: vec!["/bin/bash".to_string(), "-lc".to_string(), args.cmd],
                            process_id,
                            yield_time_ms: args.yield_time_ms,
                            max_output_tokens: args.max_output_tokens,
                            workdir,
                            sandbox_permissions: args.sandbox_permissions,
                            justification: args.justification,
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
                        process_id: &args.process_id,
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
                    process_id: args.process_id.clone(),
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

    if let Some(process_id) = response.process_id.as_ref() {
        sections.push(format!("Process running with id {process_id}"));
    }

    if let Some(original_token_count) = response.original_token_count {
        sections.push(format!("Original token count: {original_token_count}"));
    }

    sections.push("Output:".to_string());
    sections.push(response.output.clone());

    sections.join("\n")
}


