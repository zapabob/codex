use mcp_types::Tool;
use serde_json::json;

/// Create dom_read tool definition
pub fn create_dom_read_tool() -> Tool {
    Tool {
        name: "dom_read".to_string(),
        description: Some("Read DOM from active Chrome tab".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "selector": {
                    "type": "string",
                    "description": "CSS selector to read (optional, reads entire page if not specified)"
                },
                "max_chars": {
                    "type": "number",
                    "description": "Maximum characters to read",
                    "default": 5000
                }
            }
        }),
    }
}

/// Create console_get_logs tool definition
pub fn create_console_get_logs_tool() -> Tool {
    Tool {
        name: "console_get_logs".to_string(),
        description: Some("Get console logs from active Chrome tab".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "level": {
                    "type": "string",
                    "description": "Filter logs by level (log, warn, error, info, debug)",
                    "enum": ["log", "warn", "error", "info", "debug"]
                },
                "filter": {
                    "type": "string",
                    "description": "Filter logs by message content"
                },
                "limit": {
                    "type": "number",
                    "description": "Maximum number of logs to retrieve",
                    "default": 50
                }
            }
        }),
    }
}

/// Create network_get_logs tool definition
pub fn create_network_get_logs_tool() -> Tool {
    Tool {
        name: "network_get_logs".to_string(),
        description: Some("Get network request logs from active Chrome tab".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "filter": {
                    "type": "string",
                    "description": "Filter requests by URL pattern"
                },
                "limit": {
                    "type": "number",
                    "description": "Maximum number of requests to retrieve",
                    "default": 50
                }
            }
        }),
    }
}

/// Get all Chrome extension tools
pub fn get_chrome_tools() -> Vec<Tool> {
    vec![
        create_dom_read_tool(),
        create_console_get_logs_tool(),
        create_network_get_logs_tool(),
    ]
}
