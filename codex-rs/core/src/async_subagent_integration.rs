// 非同期サブエージェント統合モジュール
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::Duration;

use crate::protocol::{Event, EventMsg};
use crate::codex::Session;

use codex_supervisor::{
    AsyncSubAgent, AsyncSubAgentManager, AsyncSubAgentNotification, 
    AgentType, NotificationType, Inbox,
    ThinkingProcessManager, TokenTracker, TokenLimit, TokenAllocationStrategy,
    AutonomousDispatcher, ThinkingStepBuilder, ThinkingStepType
};

/// 非同期サブエージェント統合システム
#[derive(Debug)]
pub struct AsyncSubAgentIntegration {
    pub manager: AsyncSubAgentManager,
    pub global_inbox: Inbox,
    pub notification_tx: mpsc::UnboundedSender<AsyncSubAgentNotification>,
    pub notification_rx: mpsc::UnboundedReceiver<AsyncSubAgentNotification>,
    pub thinking_manager: Arc<tokio::sync::RwLock<ThinkingProcessManager>>,
    pub token_tracker: Arc<TokenTracker>,
    pub dispatcher: Arc<tokio::sync::RwLock<AutonomousDispatcher>>,
}

impl AsyncSubAgentIntegration {
    pub fn new() -> Self {
        let mut manager = AsyncSubAgentManager::new();
        let (notification_tx, notification_rx) = mpsc::unbounded_channel();
        
        // 思考プロセス管理、トークン追跡、自律的ディスパッチャーを初期化
        let thinking_manager = Arc::new(tokio::sync::RwLock::new(ThinkingProcessManager::new()));
        let token_tracker = Arc::new(TokenTracker::new(
            TokenLimit::default(),
            TokenAllocationStrategy::Dynamic,
        ));
        let dispatcher = Arc::new(tokio::sync::RwLock::new(AutonomousDispatcher::new()));
        
        // デフォルトエージェントを登録
        let _code_expert_id = manager.register_agent(AgentType::CodeExpert);
        let _security_expert_id = manager.register_agent(AgentType::SecurityExpert);
        let _testing_expert_id = manager.register_agent(AgentType::TestingExpert);
        let _docs_expert_id = manager.register_agent(AgentType::DocsExpert);
        let _deep_researcher_id = manager.register_agent(AgentType::DeepResearcher);
        let _debug_expert_id = manager.register_agent(AgentType::DebugExpert);
        let _performance_expert_id = manager.register_agent(AgentType::PerformanceExpert);
        let _general_id = manager.register_agent(AgentType::General);

        Self {
            manager,
            global_inbox: Inbox::new(1000),
            notification_tx,
            notification_rx,
            thinking_manager,
            token_tracker,
            dispatcher,
        }
    }

    /// 非同期でサブエージェントにタスクを開始
    pub async fn start_subagent_task(&self, agent_type: AgentType, task: String) -> Result<String> {
        // エージェントIDを取得（簡略化のため、最初に見つかったエージェントを使用）
        let agent_ids = self.manager.get_all_agent_ids();
        let agent_id = agent_ids.iter()
            .find(|id| {
                if let Some(agent) = self.manager.get_agent(id) {
                    agent.agent_type == agent_type
                } else {
                    false
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Agent not found: {:?}", agent_type))?;

        // 非同期でタスクを開始
        self.manager.start_task_async(agent_id, task.clone()).await?;

        Ok(format!("Started task '{}' on {} agent", task, agent_type))
    }

    /// 受信トレイをチェックして未読通知を取得
    pub async fn check_inbox(&self) -> Vec<AsyncSubAgentNotification> {
        let mut all_notifications = Vec::new();
        
        // 各エージェントの受信トレイをチェック
        for agent_id in self.manager.get_all_agent_ids() {
            if let Some(agent) = self.manager.get_agent(&agent_id) {
                let notifications = agent.get_inbox().get_unread_notifications().await;
                all_notifications.extend(notifications);
            }
        }

        // グローバル受信トレイもチェック
        let global_notifications = self.global_inbox.get_unread_notifications().await;
        all_notifications.extend(global_notifications);

        // タイムスタンプでソート（新しい順）
        all_notifications.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        all_notifications
    }

    /// 通知を処理してイベントを生成
    pub async fn process_notifications(&self, session: &Arc<Session>) -> Result<()> {
        let notifications = self.check_inbox().await;
        
        for notification in notifications {
            let event_msg = match notification.notification_type {
                NotificationType::TaskCompleted => {
                    EventMsg::SubAgentTaskCompleted(
                        codex_protocol::protocol::SubAgentTaskCompletedEvent {
                            agent_type: notification.agent_type.to_string(),
                            content: notification.content,
                            timestamp: notification.timestamp,
                        }
                    )
                }
                NotificationType::TaskFailed => {
                    EventMsg::SubAgentTaskFailed(
                        codex_protocol::protocol::SubAgentTaskFailedEvent {
                            agent_type: notification.agent_type.to_string(),
                            error: notification.content,
                            timestamp: notification.timestamp,
                        }
                    )
                }
                NotificationType::ProgressUpdate => {
                    EventMsg::SubAgentProgressUpdate(
                        codex_protocol::protocol::SubAgentProgressUpdateEvent {
                            agent_type: notification.agent_type.to_string(),
                            progress: notification.content,
                            timestamp: notification.timestamp,
                        }
                    )
                }
                NotificationType::AgentMessage => {
                    EventMsg::SubAgentMessage(
                        codex_protocol::protocol::SubAgentMessageEvent {
                            agent_type: notification.agent_type.to_string(),
                            message: notification.content,
                            timestamp: notification.timestamp,
                        }
                    )
                }
                NotificationType::Error => {
                    EventMsg::SubAgentError(
                        codex_protocol::protocol::SubAgentErrorEvent {
                            agent_type: notification.agent_type.to_string(),
                            error: notification.content,
                            timestamp: notification.timestamp,
                        }
                    )
                }
                NotificationType::Info => {
                    EventMsg::SubAgentInfo(
                        codex_protocol::protocol::SubAgentInfoEvent {
                            agent_type: notification.agent_type.to_string(),
                            info: notification.content,
                            timestamp: notification.timestamp,
                        }
                    )
                }
            };

            let event = Event {
                id: notification.id,
                msg: event_msg,
            };

            session.send_event(event).await;
        }

        Ok(())
    }

    /// サブエージェントの状態を取得
    pub async fn get_agent_states(&self) -> Vec<(String, AgentType, String, f32)> {
        let mut states = Vec::new();
        
        for agent_id in self.manager.get_all_agent_ids() {
            if let Some(agent) = self.manager.get_agent(&agent_id) {
                let state = agent.get_state().await;
                states.push((
                    agent_id,
                    state.agent_type,
                    format!("{:?}", state.status),
                    state.progress,
                ));
            }
        }
        
        states
    }

    /// 非同期サブエージェント監視ループ
    pub async fn start_monitoring_loop(&self, session: Arc<Session>) -> Result<()> {
        let mut interval = tokio::time::interval(Duration::from_millis(1000)); // 1秒間隔
        
        loop {
            interval.tick().await;
            
            // 通知を処理
            if let Err(e) = self.process_notifications(&session).await {
                eprintln!("Error processing notifications: {}", e);
            }
        }
    }

    /// サブエージェントとの会話を開始
    pub async fn start_conversation_with_subagent(
        &self,
        agent_type: AgentType,
        message: String,
    ) -> Result<String> {
        // エージェントIDを取得
        let agent_ids = self.manager.get_all_agent_ids();
        let agent_id = agent_ids.iter()
            .find(|id| {
                if let Some(agent) = self.manager.get_agent(id) {
                    agent.agent_type == agent_type
                } else {
                    false
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Agent not found: {:?}", agent_type))?;

        // メッセージを送信
        if let Some(agent) = self.manager.get_agent(agent_id) {
            agent.send_notification(NotificationType::AgentMessage, message.clone()).await?;
            Ok(format!("Message sent to {}: {}", agent_type, message))
        } else {
            Err(anyhow::anyhow!("Agent not found"))
        }
    }

    /// サブエージェントを終了
    pub async fn terminate_subagent(&self, agent_type: AgentType) -> Result<String> {
        // エージェントIDを取得
        let agent_ids = self.manager.get_all_agent_ids();
        let agent_id = agent_ids.iter()
            .find(|id| {
                if let Some(agent) = self.manager.get_agent(id) {
                    agent.agent_type == agent_type
                } else {
                    false
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Agent not found: {:?}", agent_type))?;

        // 終了通知を送信
        if let Some(agent) = self.manager.get_agent(agent_id) {
            agent.send_notification(NotificationType::Info, "Terminating agent".to_string()).await?;
            Ok(format!("Terminated {} agent", agent_type))
        } else {
            Err(anyhow::anyhow!("Agent not found"))
        }
    }

    /// 自律的にサブエージェントを呼び出す（メインエージェントが自動判断）
    pub async fn auto_dispatch_task(&self, task: &str) -> Result<String> {
        // タスクを分類
        let mut dispatcher = self.dispatcher.write().await;
        let classification = dispatcher.classify_task(task);

        drop(dispatcher); // ロックを解放

        // 思考プロセスを記録
        let task_id = uuid::Uuid::new_v4().to_string();
        {
            let mut thinking_manager = self.thinking_manager.write().await;
            let process = thinking_manager.start_process(
                classification.recommended_agent.clone(),
                task_id.clone(),
                50,
            );

            // 分析ステップを追加
            let analysis_step = ThinkingStepBuilder::new(ThinkingStepType::ProblemAnalysis)
                .content(format!("Task classified as: {:?}", classification.recommended_agent))
                .confidence(classification.confidence)
                .reasoning(classification.reasoning.clone())
                .build();
            process.add_step(analysis_step);

            // 判断ステップを追加
            let decision_step = ThinkingStepBuilder::new(ThinkingStepType::Decision)
                .content(format!(
                    "Dispatching to {} (Confidence: {:.1}%)",
                    classification.recommended_agent,
                    classification.confidence * 100.0
                ))
                .confidence(classification.confidence)
                .reasoning(format!(
                    "Best match based on: {}. Alternatives: {:?}",
                    classification.reasoning, classification.alternative_agents
                ))
                .build();
            process.add_step(decision_step);
        }

        // エージェントにタスクを開始
        self.start_subagent_task(classification.recommended_agent.clone(), task.to_string())
            .await?;

        Ok(format!(
            "Auto-dispatched task to {} (Confidence: {:.1}%)\nReasoning: {}",
            classification.recommended_agent,
            classification.confidence * 100.0,
            classification.reasoning
        ))
    }

    /// 思考プロセスのサマリーを取得
    pub async fn get_thinking_summary(&self, task_id: &str) -> Option<String> {
        let thinking_manager = self.thinking_manager.read().await;
        thinking_manager
            .get_process(task_id)
            .map(|p| p.get_summary())
    }

    /// 全ての思考プロセスサマリーを取得
    pub async fn get_all_thinking_summaries(&self) -> String {
        let thinking_manager = self.thinking_manager.read().await;
        thinking_manager.get_all_summaries()
    }

    /// トークンレポートを生成
    pub async fn generate_token_report(&self) -> String {
        self.token_tracker.generate_report().await
    }

    /// トークン使用量を記録
    pub async fn record_token_usage(
        &self,
        agent_id: &str,
        task_id: String,
        task_description: String,
        prompt_tokens: u64,
        completion_tokens: u64,
    ) -> Result<()> {
        let usage = codex_supervisor::TokenUsage::new(prompt_tokens, completion_tokens);
        self.token_tracker
            .record_usage(agent_id, task_id, task_description, usage)
            .await?;
        Ok(())
    }
}

impl Default for AsyncSubAgentIntegration {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_async_subagent_integration() {
        let integration = AsyncSubAgentIntegration::new();
        assert!(integration.manager.agent_count() > 0);
    }

    #[tokio::test]
    async fn test_start_subagent_task() {
        let integration = AsyncSubAgentIntegration::new();
        let result = integration
            .start_subagent_task(AgentType::CodeExpert, "Test task".to_string())
            .await
            .unwrap();
        
        assert!(result.contains("CodeExpert"));
        assert!(result.contains("Test task"));
    }

    #[tokio::test]
    async fn test_check_inbox() {
        let integration = AsyncSubAgentIntegration::new();
        let notifications = integration.check_inbox().await;
        // 初期状態では通知はないはず
        assert_eq!(notifications.len(), 0);
    }

    #[tokio::test]
    async fn test_get_agent_states() {
        let integration = AsyncSubAgentIntegration::new();
        let states = integration.get_agent_states().await;
        
        // 8つのエージェントが登録されているはず
        assert_eq!(states.len(), 8);
    }

    #[tokio::test]
    async fn test_auto_dispatch() {
        let integration = AsyncSubAgentIntegration::new();

        // セキュリティタスクを自動ディスパッチ
        let result = integration
            .auto_dispatch_task("Review code for security vulnerabilities")
            .await
            .unwrap();

        assert!(result.contains("SecurityExpert"));
        assert!(result.contains("Confidence"));
    }

    #[tokio::test]
    async fn test_token_reporting() {
        let integration = AsyncSubAgentIntegration::new();

        // トークン使用量を記録
        integration
            .record_token_usage("agent-1", "task-1".to_string(), "Test task".to_string(), 100, 50)
            .await
            .unwrap();

        // レポート生成
        let report = integration.generate_token_report().await;
        assert!(report.contains("Token Usage Report"));
    }

    #[tokio::test]
    async fn test_thinking_process_summary() {
        let integration = AsyncSubAgentIntegration::new();

        // 全ての思考プロセスサマリーを取得
        let summary = integration.get_all_thinking_summaries().await;
        assert!(summary.contains("All Thinking Processes"));
    }
}
