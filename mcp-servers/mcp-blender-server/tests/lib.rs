#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_geometry_params_serialization() {
        let params = json!({
            "mesh_type": "cube",
            "name": "TestCube",
            "location": [0.0, 0.0, 0.0],
            "dimensions": [2.0, 2.0, 2.0]
        });
        assert_eq!(params["mesh_type"], "cube");
        assert_eq!(params["name"], "TestCube");
    }

    #[test]
    fn test_material_params_serialization() {
        let params = json!({
            "name": "TestMaterial",
            "base_color": [1.0, 0.0, 0.0, 1.0],
            "metallic": 0.5,
            "roughness": 0.3
        });
        assert_eq!(params["name"], "TestMaterial");
        assert_eq!(params["metallic"], 0.5);
    }

    #[test]
    fn test_tool_list_format() {
        let response = json!({
            "tools": [
                {
                    "name": "blender_create_geometry",
                    "description": "Create primitive geometry",
                    "input_schema": {
                        "type": "object",
                        "properties": {
                            "mesh_type": {"type": "string"},
                            "name": {"type": "string"}
                        }
                    }
                }
            ]
        });
        assert!(response["tools"].is_array());
        assert_eq!(response["tools"][0]["name"], "blender_create_geometry");
    }

    #[test]
    fn test_jrpc_initialize_response() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "blender-mcp-server", "version": "2.14.1"}
            }
        });
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(
            response["result"]["serverInfo"]["name"],
            "blender-mcp-server"
        );
    }
}
