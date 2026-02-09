#[cfg(test)]
mod tests {
    use chrono::Utc;
    use serde_json::json;

    #[test]
    fn test_gpu_model_params_serialization() {
        let params = json!({
            "model": "claude-3-opus",
            "priority": "high"
        });
        assert_eq!(params["model"], "claude-3-opus");
    }

    #[test]
    fn test_task_distribution_params_serialization() {
        let params = json!({
            "task_type": "inference",
            "workload": {"batch_size": 32},
            "num_workers": 4
        });
        assert_eq!(params["task_type"], "inference");
        assert_eq!(params["num_workers"], 4);
    }

    #[test]
    fn test_workflow_params_serialization() {
        let params = json!({
            "steps": [{"task": "process"}, {"task": "validate"}],
            "parallel": true
        });
        assert!(params["parallel"].as_bool().unwrap());
        assert_eq!(params["steps"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_timestamp_format() {
        let timestamp = Utc::now().to_rfc3339();
        assert!(timestamp.contains("T"));
        assert!(timestamp.ends_with("+00:00"));
    }

    #[test]
    fn test_tool_list_format() {
        let response = json!({
            "tools": [
                {
                    "name": "yolo_select_gpu",
                    "description": "Select GPU model",
                    "input_schema": {
                        "type": "object",
                        "properties": {
                            "model": {"type": "string"}
                        }
                    }
                }
            ]
        });
        assert!(response["tools"].is_array());
        assert_eq!(response["tools"][0]["name"], "yolo_select_gpu");
    }
}
