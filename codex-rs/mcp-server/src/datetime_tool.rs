//! MCP tool for getting current date and time.

use mcp_types::Tool;
use mcp_types::ToolInputSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

/// Parameters for the datetime tool call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateTimeToolParam {
    /// Format for the datetime output (e.g., "yyyy-MM-dd HH:mm:ss", "yyyy-MM-dd", "ISO8601").
    #[serde(default = "default_format")]
    pub format: String,

    /// Timezone (e.g., "UTC", "Asia/Tokyo", "America/New_York"). Defaults to system timezone.
    #[serde(default)]
    pub timezone: Option<String>,
}

fn default_format() -> String {
    "yyyy-MM-dd HH:mm:ss".to_string()
}

/// Create the MCP tool definition for getting current datetime.
pub fn create_datetime_tool() -> Tool {
    Tool {
        name: "codex-datetime".to_string(),
        title: Some("Get Current Date and Time".to_string()),
        description: Some(
            "Get the current date and time in various formats. \
             This tool is useful for creating implementation logs with timestamps, \
             tracking when tasks were completed, and generating time-based file names.\n\n\
             Supported formats:\n\
             - 'yyyy-MM-dd HH:mm:ss' (default): Standard datetime format\n\
             - 'yyyy-MM-dd': Date only\n\
             - 'HH:mm:ss': Time only\n\
             - 'ISO8601': ISO 8601 format\n\
             - 'timestamp': Unix timestamp\n\n\
             Example: Get current datetime for implementation log filename"
                .to_string(),
        ),
        input_schema: ToolInputSchema {
            r#type: "object".to_string(),
            properties: Some(json!({
                "format": {
                    "type": "string",
                    "description": "Format for the datetime output. Options: 'yyyy-MM-dd HH:mm:ss' (default), 'yyyy-MM-dd', 'HH:mm:ss', 'ISO8601', 'timestamp'",
                    "default": "yyyy-MM-dd HH:mm:ss"
                },
                "timezone": {
                    "type": "string",
                    "description": "Optional timezone (e.g., 'UTC', 'Asia/Tokyo'). Defaults to system timezone."
                }
            })),
            required: Some(vec![]),
        },
        output_schema: None,
        annotations: None,
    }
}
