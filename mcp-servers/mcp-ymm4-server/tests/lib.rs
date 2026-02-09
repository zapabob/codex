#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_scene_params_serialization() {
        let params = json!({
            "name": "TestScene",
            "duration": 120.0,
            "camera_position": [0.0, 5.0, 10.0]
        });
        assert_eq!(params["name"], "TestScene");
        assert_eq!(params["duration"], 120.0);
    }

    #[test]
    fn test_character_params_serialization() {
        let params = json!({
            "name": "YukkuriCharacter",
            "ymm4_name": "character001",
            "position": [100.0, 200.0],
            "scale": 1.5
        });
        assert_eq!(params["name"], "YukkuriCharacter");
        assert_eq!(params["scale"], 1.5);
    }

    #[test]
    fn test_render_params_serialization() {
        let params = json!({
            "output_path": "./output/video.mp4",
            "quality": "high",
            "resolution": [1920, 1080],
            "fps": 60
        });
        assert_eq!(params["fps"], 60);
        assert_eq!(params["resolution"][0], 1920);
    }

    #[test]
    fn test_tool_list_format() {
        let response = json!({
            "tools": [
                {
                    "name": "ymm4_create_scene",
                    "description": "Create new scene",
                    "input_schema": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"}
                        }
                    }
                }
            ]
        });
        assert!(response["tools"].is_array());
        assert_eq!(response["tools"][0]["name"], "ymm4_create_scene");
    }
}
