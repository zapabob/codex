use core_test_support::responses;
use serde_json::json;
use std::path::Path;

pub fn create_shell_sse_response(
    command: Vec<String>,
    workdir: Option<&Path>,
    timeout_ms: Option<u64>,
    call_id: &str,
) -> anyhow::Result<String> {
    // The `arguments`` for the `shell` tool is a serialized JSON object.
    let tool_call_arguments = serde_json::to_string(&json!({
        "command": command,
        "workdir": workdir.map(|w| w.to_string_lossy()),
        "timeout": timeout_ms
    }))?;
    Ok(responses::sse(vec![
        responses::ev_response_created("resp-1"),
        responses::ev_function_call(call_id, "shell_command", &tool_call_arguments),
        responses::ev_completed("resp-1"),
    ]))
}

pub fn create_final_assistant_message_sse_response(message: &str) -> anyhow::Result<String> {
    Ok(responses::sse(vec![
        responses::ev_response_created("resp-1"),
        responses::ev_assistant_message("msg-1", message),
        responses::ev_completed("resp-1"),
    ]))
}

pub fn create_apply_patch_sse_response(
    patch_content: &str,
    call_id: &str,
) -> anyhow::Result<String> {
    Ok(responses::sse(vec![
        responses::ev_response_created("resp-1"),
        responses::ev_apply_patch_shell_command_call_via_heredoc(call_id, patch_content),
        responses::ev_completed("resp-1"),
    ]))
}

/// Alias for create_shell_sse_response for backward compatibility
pub fn create_shell_command_sse_response(
    command: Vec<String>,
    workdir: Option<&Path>,
    timeout_ms: Option<u64>,
    call_id: &str,
) -> anyhow::Result<String> {
    create_shell_sse_response(command, workdir, timeout_ms, call_id)
}

/// Create SSE response for exec_command (unified_exec tool)
pub fn create_exec_command_sse_response(
    command: Vec<String>,
    workdir: Option<&Path>,
    timeout_ms: Option<u64>,
    call_id: &str,
) -> anyhow::Result<String> {
    // The `arguments` for the `unified_exec` tool is a serialized JSON object.
    let tool_call_arguments = serde_json::to_string(&json!({
        "command": command,
        "workdir": workdir.map(|w| w.to_string_lossy()),
        "timeout": timeout_ms
    }))?;
    Ok(responses::sse(vec![
        responses::ev_response_created("resp-1"),
        responses::ev_function_call(call_id, "exec_command", &tool_call_arguments),
        responses::ev_completed("resp-1"),
    ]))
}

pub fn create_request_user_input_sse_response(call_id: &str) -> anyhow::Result<String> {
    let tool_call_arguments = serde_json::to_string(&json!({
        "questions": [{
            "id": "confirm_path",
            "header": "Confirm",
            "question": "Proceed with the plan?",
            "options": [{
                "label": "Yes (Recommended)",
                "description": "Continue the current plan."
            }, {
                "label": "No",
                "description": "Stop and revisit the approach."
            }]
        }]
    }))?;

    Ok(responses::sse(vec![
        responses::ev_response_created("resp-1"),
        responses::ev_function_call(call_id, "request_user_input", &tool_call_arguments),
        responses::ev_completed("resp-1"),
    ]))
}

pub fn create_request_permissions_sse_response(call_id: &str) -> anyhow::Result<String> {
    let tool_call_arguments = serde_json::to_string(&json!({
        "reason": "Select a workspace root",
        "permissions": {
            "file_system": {
                "write": [
                    ".",
                    "../shared"
                ]
            }
        }
    }))?;

    Ok(responses::sse(vec![
        responses::ev_response_created("resp-1"),
        responses::ev_function_call(call_id, "request_permissions", &tool_call_arguments),
        responses::ev_completed("resp-1"),
    ]))
}
