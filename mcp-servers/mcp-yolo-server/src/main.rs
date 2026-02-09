use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

fn list_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "yolo_select_gpu".to_string(),
            description: "Select GPU model for YOLO task".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "model": {"type": "string"},
                    "priority": {"type": "string"}
                },
                "required": ["model"]
            }),
        },
        Tool {
            name: "yolo_distribute_task".to_string(),
            description: "Distribute task across workers".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_type": {"type": "string"},
                    "workload": {"type": "object"},
                    "num_workers": {"type": "integer"}
                },
                "required": ["task_type", "workload"]
            }),
        },
        Tool {
            name: "yolo_execute_workflow".to_string(),
            description: "Execute multi-step workflow".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "steps": {"type": "array"},
                    "parallel": {"type": "boolean"}
                },
                "required": ["steps"]
            }),
        },
        Tool {
            name: "yolo_aggregate_results".to_string(),
            description: "Aggregate results".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_ids": {"type": "array"},
                    "strategy": {"type": "string"}
                },
                "required": ["task_ids"]
            }),
        },
        Tool {
            name: "yolo_monitor_progress".to_string(),
            description: "Monitor progress".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_id": {"type": "string"}
                },
                "required": ["task_id"]
            }),
        },
    ]
}

fn handle_tool_call(method: &str, _params: serde_json::Value) -> serde_json::Value {
    match method {
        "yolo_select_gpu" => {
            let timestamp = Utc::now().to_rfc3339();
            json!({"content": [{"type": "text", "text": format!("GPU selected - {}", timestamp)}]})
        }
        "yolo_distribute_task" => {
            let task_id = format!("task-{}", Utc::now().timestamp());
            json!({"content": [{"type": "text", "text": format!("Task distributed: {}", task_id)}], "structuredContent": {"task_id": task_id}})
        }
        "yolo_execute_workflow" => {
            let workflow_id = format!("workflow-{}", Utc::now().timestamp());
            json!({"content": [{"type": "text", "text": format!("Workflow started: {}", workflow_id)}], "structuredContent": {"workflow_id": workflow_id}})
        }
        "yolo_aggregate_results" => {
            json!({"content": [{"type": "text", "text": "Results aggregated"}]})
        }
        "yolo_monitor_progress" => {
            json!({"content": [{"type": "text", "text": "Progress: 42%"}], "structuredContent": {"progress": 42}})
        }
        _ => json!({"error": "Unknown tool"}),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: serde_json::Value,
    method: String,
    params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: serde_json::Value,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let line = input.trim();
                if line.is_empty() {
                    continue;
                }

                match serde_json::from_str::<JsonRpcRequest>(line) {
                    Ok(req) => {
                        let response = match req.method.as_str() {
                            "initialize" => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req.id.clone(),
                                result: Some(json!({
                                    "protocolVersion": "2024-11-05",
                                    "capabilities": {"tools": {}},
                                    "serverInfo": {"name": "yolo-mcp-server", "version": "2.14.1"}
                                })),
                                error: None,
                            },
                            "tools/list" => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req.id.clone(),
                                result: Some(json!({"tools": list_tools()})),
                                error: None,
                            },
                            "tools/call" => {
                                let params = req.params.unwrap_or_default();
                                let tool_name =
                                    params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                                let arguments =
                                    params.get("arguments").cloned().unwrap_or_default();
                                JsonRpcResponse {
                                    jsonrpc: "2.0".to_string(),
                                    id: req.id.clone(),
                                    result: Some(handle_tool_call(tool_name, arguments)),
                                    error: None,
                                }
                            }
                            _ => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req.id.clone(),
                                result: None,
                                error: Some(json!({"code": -32601, "message": "Method not found"})),
                            },
                        };

                        if let Ok(response_str) = serde_json::to_string(&response) {
                            let _ = io::stdout().write_all(response_str.as_bytes());
                            let _ = io::stdout().write_all(b"\n");
                            let _ = io::stdout().flush();
                        }
                    }
                    Err(e) => {
                        eprintln!("Parse error: {e}");
                    }
                }
            }
            Err(_) => break,
        }
    }
}
