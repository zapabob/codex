#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_udon_sharp_code_serialization() {
        let code = json!({
            "code": "public class MyClass {}",
            "class_name": "MyClass"
        });
        assert!(code.is_object());
        assert_eq!(code["class_name"], "MyClass");
    }

    #[test]
    fn test_world_config_serialization() {
        let config = json!({
            "name": "Test World",
            "description": "A test world",
            "capacity": 32
        });
        assert_eq!(config["name"], "Test World");
        assert_eq!(config["capacity"], 32);
    }

    #[test]
    fn test_tool_list_response_format() {
        let response = json!({
            "tools": [
                {
                    "name": "vrchat_compile_udon",
                    "description": "Compile UdonSharp code",
                    "input_schema": {
                        "type": "object",
                        "properties": {
                            "code": {"type": "string"},
                            "class_name": {"type": "string"}
                        }
                    }
                }
            ]
        });
        assert!(response["tools"].is_array());
        assert_eq!(response["tools"][0]["name"], "vrchat_compile_udon");
    }

    #[test]
    fn test_jrpc_response_format() {
        let response = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "vrchat-mcp-server", "version": "2.14.1"}
            }
        });
        assert_eq!(response["jsonrpc"], "2.0");
        assert_eq!(
            response["result"]["serverInfo"]["name"],
            "vrchat-mcp-server"
        );
    }

    #[test]
    fn test_error_response_format() {
        let error = json!({
            "jsonrpc": "2.0",
            "id": null,
            "error": {
                "code": -32601,
                "message": "Method not found"
            }
        });
        assert_eq!(error["error"]["code"], -32601);
        assert_eq!(error["error"]["message"], "Method not found");
    }
}
