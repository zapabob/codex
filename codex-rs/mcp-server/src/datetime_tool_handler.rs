//! Handler for datetime tool calls.

use crate::datetime_tool::DateTimeToolParam;
use chrono::Local;
use chrono::Utc;
use mcp_types::CallToolResult;
use mcp_types::ContentBlock;
use mcp_types::RequestId;
use mcp_types::TextContent;
use serde_json::json;

/// Handle datetime tool call.
pub async fn handle_datetime_tool_call(
    _id: RequestId,
    arguments: Option<serde_json::Value>,
) -> CallToolResult {
    let params = match arguments {
        Some(json_val) => match serde_json::from_value::<DateTimeToolParam>(json_val) {
            Ok(p) => p,
            Err(e) => {
                return CallToolResult {
                    content: vec![ContentBlock::TextContent(TextContent {
                        r#type: "text".to_string(),
                        text: format!("Invalid datetime parameters: {e}"),
                        annotations: None,
                    })],
                    is_error: Some(true),
                    structured_content: None,
                };
            }
        },
        None => DateTimeToolParam {
            format: default_format(),
            timezone: None,
        },
    };

    handle_datetime_tool(params).await
}

fn default_format() -> String {
    "yyyy-MM-dd HH:mm:ss".to_string()
}

/// Handle datetime tool call with parsed parameters.
async fn handle_datetime_tool(params: DateTimeToolParam) -> CallToolResult {
    

    match get_current_datetime(&params) {
        Ok(datetime_str) => {
            let result_json = json!({
                "datetime": datetime_str,
                "format": params.format,
                "timezone": params.timezone.as_ref().unwrap_or(&"system".to_string()),
            });

            CallToolResult {
                content: vec![ContentBlock::TextContent(TextContent {
                    r#type: "text".to_string(),
                    text: format!("Current datetime: {datetime_str}"),
                    annotations: None,
                })],
                is_error: None,
                structured_content: Some(result_json),
            }
        }
        Err(e) => CallToolResult {
            content: vec![ContentBlock::TextContent(TextContent {
                r#type: "text".to_string(),
                text: format!("Failed to get datetime: {e}"),
                annotations: None,
            })],
            is_error: Some(true),
            structured_content: None,
        },
    }
}

/// Get current datetime based on format and timezone.
fn get_current_datetime(params: &DateTimeToolParam) -> Result<String, String> {
    let dt = if let Some(ref tz) = params.timezone {
        // For now, we only support UTC and system timezone
        // Full timezone support would require chrono-tz dependency
        match tz.to_uppercase().as_str() {
            "UTC" => Utc::now(),
            _ => {
                // Use system timezone (Local) and convert to UTC
                Local::now().with_timezone(&Utc)
            }
        }
    } else {
        // Use system timezone (Local) and convert to UTC
        Local::now().with_timezone(&Utc)
    };

    let formatted = match params.format.as_str() {
        "yyyy-MM-dd HH:mm:ss" => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        "yyyy-MM-dd" => dt.format("%Y-%m-%d").to_string(),
        "HH:mm:ss" => dt.format("%H:%M:%S").to_string(),
        "ISO8601" => dt.to_rfc3339(),
        "timestamp" => dt.timestamp().to_string(),
        _ => {
            // Convert format string from yyyy-MM-dd to %Y-%m-%d format
            let chrono_format = params
                .format
                .replace("yyyy", "%Y")
                .replace("MM", "%m")
                .replace("dd", "%d")
                .replace("HH", "%H")
                .replace("mm", "%M")
                .replace("ss", "%S");
            dt.format(&chrono_format).to_string()
        }
    };

    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_datetime_tool_default_format() {
        let params = DateTimeToolParam {
            format: "yyyy-MM-dd HH:mm:ss".to_string(),
            timezone: None,
        };
        let result = handle_datetime_tool(params).await;
        assert!(result.is_error.is_none());
        assert!(!result.content.is_empty());
    }

    #[tokio::test]
    async fn test_datetime_tool_date_only() {
        let params = DateTimeToolParam {
            format: "yyyy-MM-dd".to_string(),
            timezone: None,
        };
        let result = handle_datetime_tool(params).await;
        assert!(result.is_error.is_none());
    }

    #[test]
    fn test_get_current_datetime() {
        let params = DateTimeToolParam {
            format: "yyyy-MM-dd".to_string(),
            timezone: None,
        };
        let result = get_current_datetime(&params);
        assert!(result.is_ok());
        let datetime_str = result.unwrap();
        assert!(datetime_str.len() == 10); // yyyy-MM-dd format
        assert!(datetime_str.contains('-'));
    }
}
