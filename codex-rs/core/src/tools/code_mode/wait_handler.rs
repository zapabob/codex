use async_trait::async_trait;
use serde::Deserialize;

use crate::function_tool::FunctionCallError;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::registry::ToolHandler;
use crate::tools::registry::ToolKind;

use super::DEFAULT_WAIT_YIELD_TIME_MS;
use super::ExecContext;
use super::WAIT_TOOL_NAME;
use super::handle_runtime_response;
use super::CodeModeSessionProgress;
use super::PUBLIC_TOOL_NAME;
use super::handle_node_message;
use super::protocol::HostToNodeMessage;


pub struct CodeModeWaitHandler;

#[derive(Debug, Deserialize)]
struct ExecWaitArgs {
    cell_id: String,
    #[serde(default = "default_wait_yield_time_ms")]
    yield_time_ms: u64,
    #[serde(default)]
    max_tokens: Option<usize>,
    #[serde(default)]
    terminate: bool,
}

fn default_wait_yield_time_ms() -> u64 {
    DEFAULT_WAIT_YIELD_TIME_MS
}

fn parse_arguments<T>(arguments: &str) -> Result<T, FunctionCallError>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_str(arguments).map_err(|err| {
        FunctionCallError::RespondToModel(format!("failed to parse function arguments: {err}"))
    })
}

#[async_trait]
impl ToolHandler for CodeModeWaitHandler {
    type Output = FunctionToolOutput;

    fn kind(&self) -> ToolKind {
        ToolKind::Function
    }

    async fn handle(&self, invocation: ToolInvocation) -> Result<Self::Output, FunctionCallError> {
        let ToolInvocation {
            session,
            turn,
            tool_name,
            payload,
            ..
        } = invocation;

        match payload {
            ToolPayload::Function { arguments } if tool_name == WAIT_TOOL_NAME => {
                let args: ExecWaitArgs = parse_arguments(&arguments)?;
                let exec = ExecContext { session, turn };
                let started_at = std::time::Instant::now();
                let response = exec
                    .session
                    .services
                    .code_mode_service
                    .wait(codex_code_mode::WaitRequest {
                        cell_id: args.cell_id,
                        yield_time_ms: args.yield_time_ms,
                        terminate: args.terminate,
                    })
                    .await
                    .map_err(FunctionCallError::RespondToModel)?;
                handle_runtime_response(&exec, response, args.max_tokens, started_at)
                    .await
                    .map_err(FunctionCallError::RespondToModel)
                let request_id = exec
                    .allocate_request_id()
                    .await;
                let message = if args.terminate {
                    HostToNodeMessage::Terminate {
                        request_id: request_id.clone(),
                        cell_id: args.cell_id.clone(),
                    }
                } else {
                    HostToNodeMessage::Poll {
                };
                let process_slot = exec
                    .ensure_started()
                    .map_err(|err| FunctionCallError::RespondToModel(err.to_string()))?;
                let result = {
                    let mut process_slot = process_slot;
                    let Some(process) = process_slot.as_mut() else {
                        return Err(FunctionCallError::RespondToModel(format!(
                            "{PUBLIC_TOOL_NAME} runner failed to start"
                        )));
                    };
                    if !matches!(process.has_exited(), Ok(false)) {
                    let message = process
                        .send(&request_id, &message)
                        .await
                        .map_err(|err| err.to_string());
                    let message = match message {
                        Ok(message) => message,
                        Err(error) => return Err(FunctionCallError::RespondToModel(error)),
                    handle_node_message(
                        &exec,
                        args.cell_id,
                        message,
                        Some(args.max_tokens),
                        started_at,
                    )
                match result {
                    Ok(CodeModeSessionProgress::Finished(output))
                    | Ok(CodeModeSessionProgress::Yielded { output }) => Ok(output),
                    Err(error) => Err(FunctionCallError::RespondToModel(error)),
                }

            }
            _ => Err(FunctionCallError::RespondToModel(format!(
                "{WAIT_TOOL_NAME} expects JSON arguments"
            ))),
        }
    }
}
