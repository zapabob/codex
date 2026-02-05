use std::path::Path;

<<<<<<< HEAD
pub fn create_shell_sse_response(
=======
use core_test_support::responses;
use serde_json::json;

pub fn create_shell_command_sse_response(
>>>>>>> upstream/main
    command: Vec<String>,
    workdir: Option<&Path>,
    timeout_ms: Option<u64>,
    call_id: &str,
) -> anyhow::Result<String> {
<<<<<<< HEAD
    // The `arguments`` for the `shell` tool is a serialized JSON object.
    let tool_call_arguments = serde_json::to_string(&json!({
        "command": command,
        "workdir": workdir.map(|w| w.to_string_lossy()),
        "timeout": timeout_ms
    }))?;
    let tool_call = json!({
        "choices": [
            {
                "delta": {
                    "tool_calls": [
                        {
                            "id": call_id,
                            "function": {
                                "name": "shell",
                                "arguments": tool_call_arguments
                            }
                        }
                    ]
                },
                "finish_reason": "tool_calls"
            }
        ]
    });

    let sse = format!(
        "data: {}\n\ndata: DONE\n\n",
        serde_json::to_string(&tool_call)?
    );
    Ok(sse)
=======
    let command_str = shlex::try_join(command.iter().map(String::as_str))?;
    let arguments = serde_json::to_string(&json!({
        "command": command_str,
        "workdir": workdir.map(|w| w.to_string_lossy()),
        "timeout_ms": timeout_ms,
    }))?;
    let response_id = format!("resp-{call_id}");
    Ok(responses::sse(vec![
        responses::ev_response_created(&response_id),
        responses::ev_function_call(call_id, "shell_command", &arguments),
        responses::ev_completed(&response_id),
    ]))
>>>>>>> upstream/main
}

pub fn create_final_assistant_message_sse_response(message: &str) -> anyhow::Result<String> {
    let response_id = "resp-final";
    Ok(responses::sse(vec![
        responses::ev_response_created(response_id),
        responses::ev_assistant_message("msg-final", message),
        responses::ev_completed(response_id),
    ]))
}

pub fn create_apply_patch_sse_response(
    patch_content: &str,
    call_id: &str,
) -> anyhow::Result<String> {
<<<<<<< HEAD
    // Use shell command to call apply_patch with heredoc format
    let shell_command = format!("apply_patch <<'EOF'\n{patch_content}\nEOF");
    let tool_call_arguments = serde_json::to_string(&json!({
        "command": ["bash", "-lc", shell_command]
    }))?;

    let tool_call = json!({
        "choices": [
            {
                "delta": {
                    "tool_calls": [
                        {
                            "id": call_id,
                            "function": {
                                "name": "shell",
                                "arguments": tool_call_arguments
                            }
                        }
                    ]
                },
                "finish_reason": "tool_calls"
            }
        ]
    });

    let sse = format!(
        "data: {}\n\ndata: DONE\n\n",
        serde_json::to_string(&tool_call)?
    );
    Ok(sse)
=======
    let command = format!("apply_patch <<'EOF'\n{patch_content}\nEOF");
    let arguments = serde_json::to_string(&json!({ "command": command }))?;
    let response_id = format!("resp-{call_id}");
    Ok(responses::sse(vec![
        responses::ev_response_created(&response_id),
        responses::ev_function_call(call_id, "shell_command", &arguments),
        responses::ev_completed(&response_id),
    ]))
>>>>>>> upstream/main
}
