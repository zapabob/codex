use mcp_types::Tool;
use serde_json::json;

#[allow(unused)]
pub(crate) fn webhook_tool_info() -> Tool {
    Tool {
        name: "codex-webhook".to_string(),
        description: Some(
            "Execute webhook calls to external services (GitHub, Slack, or custom endpoints). \
             Enables integration with external APIs for automated workflows."
                .to_string(),
        ),
        input_schema: mcp_types::ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "service": {
                    "type": "string",
                    "enum": ["github", "slack", "custom"],
                    "description": "Service to call"
                },
                "action": {
                    "type": "string",
                    "description": "API endpoint or action"
                },
                "data": {
                    "type": "object",
                    "description": "Payload data to send"
                },
                "headers": {
                    "type": "object",
                    "description": "Optional custom headers",
                    "additionalProperties": {"type": "string"}
                }
            })),
            required: Some(vec![
                "service".to_string(),
                "action".to_string(),
                "data".to_string(),
            ]),
        },
        title: Some("Webhook".to_string()),
        output_schema: None,
        annotations: None,
    }
}
