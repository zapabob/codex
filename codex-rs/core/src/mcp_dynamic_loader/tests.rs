#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::types::McpServerConfig;
    use crate::config::types::McpServerTransportConfig;
    use async_channel::unbounded;
    use codex_rmcp_client::OAuthCredentialsStoreMode;
    use tokio_util::sync::CancellationToken;

    #[tokio::test]
    async fn test_add_server() {
        let (tx_event, _rx_event) = unbounded();
        let cancel_token = CancellationToken::new();
        let sandbox_state = SandboxState {
            sandbox_policy: SandboxPolicy::ReadOnly,
            codex_linux_sandbox_exe: None,
            sandbox_cwd: std::path::PathBuf::from("/"),
        };

        let connection_manager =
            Arc::new(tokio::sync::RwLock::new(McpConnectionManager::default()));
        let loader = DynamicMcpLoader::new(
            connection_manager,
            OAuthCredentialsStoreMode::File,
            HashMap::new(),
            tx_event,
            cancel_token,
            sandbox_state,
        );

        let config = McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "echo".to_string(),
                args: vec!["test".to_string()],
                env: None,
                env_vars: Vec::new(),
                cwd: None,
            },
            enabled: true,
            startup_timeout_sec: None,
            tool_timeout_sec: None,
            enabled_tools: None,
            disabled_tools: None,
        };

        let result = loader.add_server("test-server".to_string(), config).await;
        assert!(result.is_ok());

        let servers = loader.list_servers().await;
        assert!(servers.contains(&"test-server".to_string()));
    }

    #[tokio::test]
    async fn test_remove_server() {
        let (tx_event, _rx_event) = unbounded();
        let cancel_token = CancellationToken::new();
        let sandbox_state = SandboxState {
            sandbox_policy: SandboxPolicy::ReadOnly,
            codex_linux_sandbox_exe: None,
            sandbox_cwd: std::path::PathBuf::from("/"),
        };

        let connection_manager =
            Arc::new(tokio::sync::RwLock::new(McpConnectionManager::default()));
        let loader = DynamicMcpLoader::new(
            connection_manager,
            OAuthCredentialsStoreMode::File,
            HashMap::new(),
            tx_event,
            cancel_token,
            sandbox_state,
        );

        let config = McpServerConfig {
            transport: McpServerTransportConfig::Stdio {
                command: "echo".to_string(),
                args: vec!["test".to_string()],
                env: None,
                env_vars: Vec::new(),
                cwd: None,
            },
            enabled: true,
            startup_timeout_sec: None,
            tool_timeout_sec: None,
            enabled_tools: None,
            disabled_tools: None,
        };

        loader
            .add_server("test-server".to_string(), config)
            .await
            .unwrap();
        let result = loader.remove_server("test-server").await;
        assert!(result.is_ok());

        let servers = loader.list_servers().await;
        assert!(!servers.contains(&"test-server".to_string()));
    }
}
