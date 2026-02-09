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
            name: "ymm4_create_scene".to_string(),
            description: "Create new scene in YukkuriMovieMaker".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "duration": {"type": "number"},
                    "camera_position": {"type": "array"}
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "ymm4_add_character".to_string(),
            description: "Add character to scene".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "ymm4_name": {"type": "string"},
                    "position": {"type": "array"},
                    "scale": {"type": "number"}
                },
                "required": ["name", "ymm4_name"]
            }),
        },
        Tool {
            name: "ymm4_audio_effect".to_string(),
            description: "Apply audio effect".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "effect_type": {"type": "string"},
                    "parameters": {"type": "object"}
                },
                "required": ["name", "effect_type"]
            }),
        },
        Tool {
            name: "ymm4_video_effect".to_string(),
            description: "Apply video effect".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "effect_type": {"type": "string"},
                    "parameters": {"type": "object"}
                },
                "required": ["name", "effect_type"]
            }),
        },
        Tool {
            name: "ymm4_render".to_string(),
            description: "Render video".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "output_path": {"type": "string"},
                    "quality": {"type": "string"},
                    "resolution": {"type": "array"},
                    "fps": {"type": "integer"}
                },
                "required": ["output_path"]
            }),
        },
    ]
}

fn handle_tool_call(method: &str, _params: serde_json::Value) -> serde_json::Value {
    match method {
        "ymm4_create_scene" => {
            json!({"content": [{"type": "text", "text": "Scene created"}]})
        }
        "ymm4_add_character" => {
            json!({"content": [{"type": "text", "text": "Character added"}]})
        }
        "ymm4_audio_effect" => {
            json!({"content": [{"type": "text", "text": "Audio effect applied"}]})
        }
        "ymm4_video_effect" => {
            json!({"content": [{"type": "text", "text": "Video effect applied"}]})
        }
        "ymm4_render" => {
            json!({"content": [{"type": "text", "text": "Render started"}]})
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
                                    "serverInfo": {"name": "ymm4-mcp-server", "version": "2.14.1"}
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
