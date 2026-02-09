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
            name: "blender_create_geometry".to_string(),
            description: "Create primitive geometry in Blender".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "mesh_type": {"type": "string", "enum": ["cube", "sphere", "cylinder", "cone", "plane", "torus"]},
                    "name": {"type": "string"},
                    "location": {"type": "array"},
                    "dimensions": {"type": "array"}
                },
                "required": ["mesh_type", "name"]
            }),
        },
        Tool {
            name: "blender_assign_material".to_string(),
            description: "Assign or create material".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "base_color": {"type": "array"},
                    "metallic": {"type": "number"},
                    "roughness": {"type": "number"}
                },
                "required": ["name"]
            }),
        },
        Tool {
            name: "blender_render".to_string(),
            description: "Configure and trigger rendering".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "engine": {"type": "string"},
                    "samples": {"type": "integer"},
                    "resolution": {"type": "array"}
                }
            }),
        },
        Tool {
            name: "blender_export".to_string(),
            description: "Export scene to various formats".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "format": {"type": "string"},
                    "filepath": {"type": "string"},
                    "apply_modifiers": {"type": "boolean"}
                },
                "required": ["format", "filepath"]
            }),
        },
        Tool {
            name: "blender_geometry_nodes".to_string(),
            description: "Apply Geometry Nodes modifier".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "node_tree_name": {"type": "string"},
                    "inputs": {"type": "object"}
                },
                "required": ["node_tree_name"]
            }),
        },
    ]
}

fn handle_tool_call(method: &str, _params: serde_json::Value) -> serde_json::Value {
    match method {
        "blender_create_geometry" => {
            json!({"content": [{"type": "text", "text": "Geometry created"}]})
        }
        "blender_assign_material" => {
            json!({"content": [{"type": "text", "text": "Material assigned"}]})
        }
        "blender_render" => {
            json!({"content": [{"type": "text", "text": "Render configured"}]})
        }
        "blender_export" => {
            json!({"content": [{"type": "text", "text": "Export prepared"}]})
        }
        "blender_geometry_nodes" => {
            json!({"content": [{"type": "text", "text": "Geometry Nodes applied"}]})
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
                                    "serverInfo": {"name": "blender-mcp-server", "version": "2.14.1"}
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
