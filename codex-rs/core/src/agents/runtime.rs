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
}

impl AgentRuntime {
    /// 新しいランタイムを作成
    pub fn new(workspace_dir: PathBuf, total_budget: usize) -> Self {
        let loader = Arc::new(RwLock::new(AgentLoader::new(&workspace_dir)));
        let budgeter = Arc::new(TokenBudgeter::new(total_budget));

        Self {
            loader,
            budgeter,
            running_agents: Arc::new(RwLock::new(HashMap::new())),
            workspace_dir,
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

        let result = match self.execute_agent(&agent_def, goal, inputs, deadline).await {
            Ok(artifacts) => {
                let duration_secs = start_time.elapsed().as_secs_f64();
                let tokens_used = self.budgeter.get_agent_usage(agent_name);

                info!(
                    "Agent '{}' completed successfully in {:.2}s, used {} tokens",
                    agent_name, duration_secs, tokens_used
                );

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

        // TODO: 実際のエージェント実行ロジック
        // - コンテキスト作成
        // - システムプロンプト設定
        // - ツール権限チェック
        // - LLM呼び出し
        // - アーティファクト生成

        // モック実装：テスト用に仮のトークン消費とアーティファクト生成
        let tokens_to_consume = 1000; // 仮の値

        if !self
            .budgeter
            .try_consume(&agent_def.name, tokens_to_consume)?
        {
            anyhow::bail!("Token budget exceeded for agent '{}'", agent_def.name);
        }

        // アーティファクトディレクトリを作成
        let artifacts_dir = self.workspace_dir.join("artifacts");
        tokio::fs::create_dir_all(&artifacts_dir).await?;

        // 仮のアーティファクトを生成
        let mut generated_artifacts = Vec::new();
        for artifact_path in &agent_def.artifacts {
            let full_path = self.workspace_dir.join(artifact_path);
            if let Some(parent) = full_path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }

            let content = format!(
                "# Agent: {}\n\n## Goal\n{}\n\n## Inputs\n{:?}\n\n## Result\nMock result from agent execution.\n",
                agent_def.name, goal, inputs
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

        let runtime = AgentRuntime::new(temp_dir.path().to_path_buf(), 10000);

        let mut inputs = HashMap::new();
        inputs.insert("key1".to_string(), "value1".to_string());

        let result = runtime
            .delegate("test-agent", "Test goal", inputs, Some(5000), None)
            .await
            .unwrap();

        assert_eq!(result.agent_name, "test-agent");
        assert_eq!(result.status, AgentStatus::Completed);
        assert!(!result.artifacts.is_empty());
        assert!(result.tokens_used > 0);
    }

    #[tokio::test]
    async fn test_list_agents() {
        let temp_dir = TempDir::new().unwrap();
        let agents_dir = temp_dir.path().join(".codex/agents");
        fs::create_dir_all(&agents_dir).unwrap();

        fs::write(agents_dir.join("agent1.yaml"), "name: Agent1\ngoal: Goal1\ntools: {}\npolicies: {context: {}}\nsuccess_criteria: []\nartifacts: []").unwrap();
        fs::write(agents_dir.join("agent2.yaml"), "name: Agent2\ngoal: Goal2\ntools: {}\npolicies: {context: {}}\nsuccess_criteria: []\nartifacts: []").unwrap();

        let runtime = AgentRuntime::new(temp_dir.path().to_path_buf(), 10000);
        let agents = runtime.list_agents().await.unwrap();

        assert_eq!(agents, vec!["agent1", "agent2"]);
    }
}
