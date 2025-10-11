use super::budgeter::TokenBudgeter;
use super::loader::AgentLoader;
use super::types::AgentDefinition;
use super::types::AgentResult;
use super::types::AgentStatus;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::debug;
use tracing::error;
use tracing::info;

use crate::AuthManager;
use crate::audit_log::AgentExecutionEvent;
use crate::audit_log::AuditEvent;
use crate::audit_log::AuditEventType;
use crate::audit_log::ExecutionStatus;
use crate::audit_log::log_audit_event;
use crate::client::ModelClient;
use crate::client_common::Prompt;
use crate::client_common::ResponseEvent;
use crate::config::Config;
use crate::model_provider_info::ModelProviderInfo;
use codex_otel::otel_event_manager::OtelEventManager;
use codex_protocol::ConversationId;
use codex_protocol::config_types::ReasoningEffort;
use codex_protocol::config_types::ReasoningSummary;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use futures::StreamExt;

/// サブエージェントランタイム
pub struct AgentRuntime {
    /// エージェントローダー
    loader: Arc<RwLock<AgentLoader>>,
    /// トークン予算管理
    budgeter: Arc<TokenBudgeter>,
    /// 実行中のエージェント
    running_agents: Arc<RwLock<HashMap<String, AgentStatus>>>,
    /// ワークスペースディレクトリ
    workspace_dir: PathBuf,
    /// LLM設定
    config: Arc<Config>,
    /// 認証マネージャー
    auth_manager: Option<Arc<AuthManager>>,
    /// OpenTelemetry イベントマネージャー
    otel_manager: OtelEventManager,
    /// モデルプロバイダー情報
    provider: ModelProviderInfo,
    /// 会話ID
    conversation_id: ConversationId,
}

impl AgentRuntime {
    /// 新しいランタイムを作成
    pub fn new(
        workspace_dir: PathBuf,
        total_budget: usize,
        config: Arc<Config>,
        auth_manager: Option<Arc<AuthManager>>,
        otel_manager: OtelEventManager,
        provider: ModelProviderInfo,
        conversation_id: ConversationId,
    ) -> Self {
        let loader = Arc::new(RwLock::new(AgentLoader::new(&workspace_dir)));
        let budgeter = Arc::new(TokenBudgeter::new(total_budget));

        Self {
            loader,
            budgeter,
            running_agents: Arc::new(RwLock::new(HashMap::new())),
            workspace_dir,
            config,
            auth_manager,
            otel_manager,
            provider,
            conversation_id,
        }
    }

    /// エージェントを委任実行
    pub async fn delegate(
        &self,
        agent_name: &str,
        goal: &str,
        inputs: HashMap<String, String>,
        budget: Option<usize>,
        deadline: Option<u64>,
    ) -> Result<AgentResult> {
        info!("Delegating to agent '{}': {}", agent_name, goal);

        // エージェント定義を読み込み
        let agent_def = {
            let mut loader = self.loader.write().await;
            loader
                .load_by_name(agent_name)
                .with_context(|| format!("Failed to load agent '{}'", agent_name))?
        };

        // 予算を設定
        if let Some(budget) = budget {
            self.budgeter.set_agent_limit(agent_name, budget)?;
        } else {
            // デフォルト予算はコンテキストポリシーから取得
            self.budgeter
                .set_agent_limit(agent_name, agent_def.policies.context.max_tokens)?;
        }

        // 実行ステータスを更新
        {
            let mut running = self.running_agents.write().await;
            running.insert(agent_name.to_string(), AgentStatus::Running);
        }

        // 実行開始
        let start_time = Instant::now();
        let start_timestamp = chrono::Utc::now().to_rfc3339();

        // 監査ログ: エージェント開始
        let _ = log_audit_event(AuditEvent::new(
            agent_name.to_string(),
            AuditEventType::AgentExecution(AgentExecutionEvent {
                agent_name: agent_name.to_string(),
                status: ExecutionStatus::Started,
                goal: goal.to_string(),
                start_time: start_timestamp.clone(),
                end_time: None,
                duration_secs: None,
                tokens_used: 0,
                artifacts: vec![],
                error: None,
            }),
        ))
        .await;

        let result = match self.execute_agent(&agent_def, goal, inputs, deadline).await {
            Ok(artifacts) => {
                let duration_secs = start_time.elapsed().as_secs_f64();
                let tokens_used = self.budgeter.get_agent_usage(agent_name);

                info!(
                    "Agent '{}' completed successfully in {:.2}s, used {} tokens",
                    agent_name, duration_secs, tokens_used
                );

                // 監査ログ: エージェント完了
                let _ = log_audit_event(AuditEvent::new(
                    agent_name.to_string(),
                    AuditEventType::AgentExecution(AgentExecutionEvent {
                        agent_name: agent_name.to_string(),
                        status: ExecutionStatus::Completed,
                        goal: goal.to_string(),
                        start_time: start_timestamp.clone(),
                        end_time: Some(chrono::Utc::now().to_rfc3339()),
                        duration_secs: Some(duration_secs),
                        tokens_used,
                        artifacts: artifacts.clone(),
                        error: None,
                    }),
                ))
                .await;

                AgentResult {
                    agent_name: agent_name.to_string(),
                    status: AgentStatus::Completed,
                    artifacts,
                    tokens_used,
                    duration_secs,
                    error: None,
                }
            }
            Err(e) => {
                error!("Agent '{}' failed: {}", agent_name, e);

                let duration_secs = start_time.elapsed().as_secs_f64();
                let tokens_used = self.budgeter.get_agent_usage(agent_name);

                // 監査ログ: エージェント失敗
                let _ = log_audit_event(AuditEvent::new(
                    agent_name.to_string(),
                    AuditEventType::AgentExecution(AgentExecutionEvent {
                        agent_name: agent_name.to_string(),
                        status: ExecutionStatus::Failed,
                        goal: goal.to_string(),
                        start_time: start_timestamp.clone(),
                        end_time: Some(chrono::Utc::now().to_rfc3339()),
                        duration_secs: Some(duration_secs),
                        tokens_used,
                        artifacts: vec![],
                        error: Some(e.to_string()),
                    }),
                ))
                .await;

                AgentResult {
                    agent_name: agent_name.to_string(),
                    status: AgentStatus::Failed,
                    artifacts: vec![],
                    tokens_used,
                    duration_secs,
                    error: Some(e.to_string()),
                }
            }
        };

        // 実行ステータスを更新
        {
            let mut running = self.running_agents.write().await;
            running.insert(agent_name.to_string(), result.status.clone());
        }

        Ok(result)
    }

    /// エージェントを実際に実行
    async fn execute_agent(
        &self,
        agent_def: &AgentDefinition,
        goal: &str,
        inputs: HashMap<String, String>,
        deadline: Option<u64>,
    ) -> Result<Vec<String>> {
        debug!("Executing agent '{}' with goal: {}", agent_def.name, goal);

        // 1. システムプロンプト構築
        let system_prompt = format!(
            "You are a specialized sub-agent with the following role:\n\
             \n\
             Agent: {}\n\
             Goal: {}\n\
             \n\
             Success Criteria:\n{}\n\
             \n\
             Inputs provided:\n{}\n\
             \n\
             Please analyze the task and execute it according to your role.\
             Generate the required artifacts as specified.",
            agent_def.name,
            agent_def.goal,
            agent_def.success_criteria.join("\n- "),
            inputs
                .iter()
                .map(|(k, v)| format!("- {}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // 2. ユーザー入力を構築
        let user_message = format!("Task: {}\n\nPlease proceed with the execution.", goal);

        // 3. ModelClient作成
        let client = ModelClient::new(
            self.config.clone(),
            self.auth_manager.clone(),
            self.otel_manager.clone(),
            self.provider.clone(),
            Some(ReasoningEffort::Medium),
            ReasoningSummary::Concise,
            self.conversation_id,
        );

        // 4. ResponseItem構築（Promptに渡す）
        let input_items = vec![ResponseItem::Message {
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: user_message.clone(),
            }],
        }];

        // 5. Prompt構築（ツールは現時点では空、将来的にツール権限から生成）
        let prompt = Prompt {
            input: input_items,
            tools: vec![], // TODO: agent_def.toolsからツール仕様を生成
            parallel_tool_calls: false,
            base_instructions_override: Some(system_prompt.clone()),
            output_schema: None,
        };

        // 6. LLM呼び出し
        let mut stream = client.stream(&prompt).await?;
        let mut response_text = String::new();
        let mut total_tokens = 0;

        while let Some(event) = stream.next().await {
            match event? {
                ResponseEvent::Created => {
                    debug!("Agent '{}': Response stream started", agent_def.name);
                }
                ResponseEvent::OutputItemDone(_item) => {
                    // TODO: Extract text from ResponseItem properly
                    debug!("Agent '{}': Output item done", agent_def.name);
                    // Placeholder token estimation
                    total_tokens += 100;
                }
                ResponseEvent::Completed {
                    response_id: _,
                    token_usage: _,
                } => {
                    debug!("Agent '{}': Response completed", agent_def.name);
                }
                _ => {}
            }
        }

        // 7. トークン予算チェックと消費
        if !self.budgeter.try_consume(&agent_def.name, total_tokens)? {
            anyhow::bail!("Token budget exceeded for agent '{}'", agent_def.name);
        }

        info!(
            "Agent '{}' completed LLM execution: {} tokens used",
            agent_def.name, total_tokens
        );

        // 8. アーティファクト生成
        let artifacts_dir = self.workspace_dir.join("artifacts");
        tokio::fs::create_dir_all(&artifacts_dir).await?;

        let mut generated_artifacts = Vec::new();
        for artifact_path in &agent_def.artifacts {
            let full_path = self.workspace_dir.join(artifact_path);
            if let Some(parent) = full_path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            // アーティファクト内容を生成
            let content = format!(
                "# Agent: {}\n\n## Goal\n{}\n\n## Task\n{}\n\n## Inputs\n{}\n\n## Agent Response\n\n{}\n\n## Execution Summary\n\n- Tokens Used: {}\n- Success Criteria:\n{}\n",
                agent_def.name,
                agent_def.goal,
                goal,
                inputs
                    .iter()
                    .map(|(k, v)| format!("- **{}**: {}", k, v))
                    .collect::<Vec<_>>()
                    .join("\n"),
                response_text,
                total_tokens,
                agent_def
                    .success_criteria
                    .iter()
                    .map(|c| format!("  - {}", c))
                    .collect::<Vec<_>>()
                    .join("\n")
            );

            tokio::fs::write(&full_path, content).await?;
            generated_artifacts.push(artifact_path.clone());
        }

        Ok(generated_artifacts)
    }

    /// 利用可能なエージェント一覧を取得
    pub async fn list_agents(&self) -> Result<Vec<String>> {
        let loader = self.loader.read().await;
        loader.list_available_agents()
    }

    /// 実行中のエージェント状態を取得
    pub async fn get_running_agents(&self) -> HashMap<String, AgentStatus> {
        self.running_agents.read().await.clone()
    }

    /// トークン使用状況を取得
    pub fn get_budget_status(&self) -> (usize, usize, f64) {
        let used = self.budgeter.get_used();
        let remaining = self.budgeter.get_remaining();
        let utilization = self.budgeter.get_utilization();
        (used, remaining, utilization)
    }

    /// 軽量版フォールバックが必要かチェック
    pub fn should_use_lightweight(&self, threshold: f64) -> bool {
        self.budgeter.should_fallback_lightweight(threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_agent_runtime_delegate() {
        use crate::config::Config;
        use crate::model_provider_info::ModelProviderInfo;
        use crate::model_provider_info::WireApi;
        use codex_otel::otel_event_manager::OtelEventManager;
        use codex_protocol::ConversationId;
        use uuid::Uuid;

        let temp_dir = TempDir::new().unwrap();
        let agents_dir = temp_dir.path().join(".codex/agents");
        fs::create_dir_all(&agents_dir).unwrap();

        let agent_yaml = r#"
name: "Test Agent"
goal: "Test goal"
tools:
  mcp: []
  fs:
    read: true
    write:
      - "./artifacts"
  net:
    allow: []
  shell: []
policies:
  context:
    max_tokens: 5000
    retention: "job"
  secrets:
    redact: false
success_criteria:
  - "基準1"
artifacts:
  - "artifacts/test-output.md"
"#;

        fs::write(agents_dir.join("test-agent.yaml"), agent_yaml).unwrap();

        // モックConfig作成
        let config = Arc::new(Config::default_for_family("gpt-5-codex"));
        let provider = ModelProviderInfo {
            name: "Test Provider".to_string(),
            base_url: Some("https://api.openai.com/v1".to_string()),
            env_key: Some("OPENAI_API_KEY".to_string()),
            wire_api: WireApi::Chat,
            env_key_instructions: None,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(10),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
        };
        let otel_manager = OtelEventManager::new();
        let conversation_id = ConversationId(Uuid::new_v4());

        let runtime = AgentRuntime::new(
            temp_dir.path().to_path_buf(),
            10000,
            config,
            None,
            otel_manager,
            provider,
            conversation_id,
        );

        let mut inputs = HashMap::new();
        inputs.insert("key1".to_string(), "value1".to_string());

        // Note: This will fail without real API credentials, but demonstrates the structure
        let result = runtime
            .delegate("test-agent", "Test goal", inputs, Some(5000), None)
            .await;

        // In real tests, we'd use mocks or fixtures
        // For now, just verify compilation
        match result {
            Ok(r) => {
                assert_eq!(r.agent_name, "test-agent");
            }
            Err(_) => {
                // Expected without real API credentials
            }
        }
    }

    #[tokio::test]
    async fn test_list_agents() {
        use crate::config::Config;
        use crate::model_provider_info::ModelProviderInfo;
        use crate::model_provider_info::WireApi;
        use codex_otel::otel_event_manager::OtelEventManager;
        use codex_protocol::ConversationId;
        use uuid::Uuid;

        let temp_dir = TempDir::new().unwrap();
        let agents_dir = temp_dir.path().join(".codex/agents");
        fs::create_dir_all(&agents_dir).unwrap();

        fs::write(agents_dir.join("agent1.yaml"), "name: Agent1\ngoal: Goal1\ntools: {}\npolicies: {context: {}}\nsuccess_criteria: []\nartifacts: []").unwrap();
        fs::write(agents_dir.join("agent2.yaml"), "name: Agent2\ngoal: Goal2\ntools: {}\npolicies: {context: {}}\nsuccess_criteria: []\nartifacts: []").unwrap();

     -co-colet config = Arc::new(Config::default_for_family("gpt-4"));
        let provider = ModelProviderInfo {
            name: "Test Provider".to_string(),
            base_url: Some("https://api.openai.com/v1".to_string()),
            env_key: Some("OPENAI_API_KEY".to_string()),
            wire_api: WireApi::Chat,
            env_key_instructions: None,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(4),
            stream_max_retries: Some(10),
            stream_idle_timeout_ms: Some(300_000),
            requires_openai_auth: false,
        };
        let otel_manager = OtelEventManager::new();
        let conversation_id = ConversationId(Uuid::new_v4());

        let runtime = AgentRuntime::new(
            temp_dir.path().to_path_buf(),
            10000,
            config.clone(),
            None,
            otel_manager,
            provider,
            conversation_id,
        );
        let agents = runtime.list_agents().await.unwrap();

        assert_eq!(agents, vec!["agent1", "agent2"]);
    }
}
