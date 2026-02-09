use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct UdonSharpCode {
    code: String,
    class_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorldConfig {
    name: String,
    description: String,
    capacity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tool {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

fn list_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "vrchat_compile_udon".to_string(),
            description: "Compile UdonSharp code for VRChat worlds".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "code": {"type": "string"},
                    "class_name": {"type": "string"}
                },
                "required": ["code", "class_name"]
            }),
        },
        Tool {
            name: "vrchat_upload_world".to_string(),
            description: "Upload world to VRChat".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "description": {"type": "string"},
                    "capacity": {"type": "integer"}
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "vrchat_configure_avatar".to_string(),
            description: "Configure avatar with modularavatar settings".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "asset_url": {"type": "string"}
                },
                "required": ["name", "asset_url"]
            }),
        },
        Tool {
            name: "vrchat_setup_physbones".to_string(),
            description: "Setup PhysBones configuration".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "bone_name": {"type": "string"},
                    "parameter_name": {"type": "string"},
                    "stiffness": {"type": "number"},
                    "drag": {"type": "number"},
                    "gravity": {"type": "number"}
                },
                "required": ["bone_name", "parameter_name"]
            }),
        },
        Tool {
            name: "vrchat_create_contact".to_string(),
            description: "Create contact system".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "contact_name": {"type": "string"},
                    "root_transform": {"type": "string"},
                    "shape_type": {"type": "string"},
                    "radius": {"type": "number"},
                    "position": {"type": "array"},
                    "collision": {"type": "string"}
                },
                "required": ["contact_name", "root_transform", "shape_type"]
            }),
        },
    ]
}

fn handle_tool_call(method: &str, params: serde_json::Value) -> serde_json::Value {
    match method {
        "vrchat_compile_udon" => {
            if let Ok(code) = serde_json::from_value::<UdonSharpCode>(params) {
                json!({
                    "content": [{
                        "type": "text",
                        "text": format!("UdonSharp compilation prepared:\nClass: {}\nCode length: {} chars", code.class_name, code.code.len())
                    }]
                })
            } else {
                json!({"error": "Invalid params"})
            }
        }
        "vrchat_upload_world" => {
            if let Ok(config) = serde_json::from_value::<WorldConfig>(params) {
                json!({
                    "content": [{
                        "type": "text",
                        "text": format!("World config: {}", config.name)
                    }]
                })
            } else {
                json!({"error": "Invalid params"})
            }
        }
        "vrchat_configure_avatar" => {
            json!({"content": [{"type": "text", "text": "Avatar configured"}]})
        }
        "vrchat_setup_physbones" => {
            json!({"content": [{"type": "text", "text": "PhysBones configured"}]})
        }
        "vrchat_create_contact" => {
            json!({"content": [{"type": "text", "text": "Contact system created"}]})
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
                                    "serverInfo": {"name": "vrchat-mcp-server", "version": "2.14.1"}
                                })),
                                error: None,
                            },
                            "tools/list" => JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: req.id.clone(),
                                result: Some(json!({
                                    "tools": list_tools()
                                })),
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
                                error: Some(json!({
                                    "code": -32601,
                                    "message": "Method not found"
                                })),
                            },
                        };

                        if let Ok(response_str) = serde_json::to_string(&response) {
                            println!("{}", response_str);
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
