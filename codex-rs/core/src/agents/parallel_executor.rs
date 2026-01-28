//! 並列非同期サブエージェント実行エグゼキューター
//!
//! エラーハンドリング、進捗追跡、リソース制限を備えた並列実行機能を提供

use super::types::AgentResult;
use super::types::AgentStatus;
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use tracing::error;
use tracing::info;
use tracing::warn;

/// 進捗イベント
#[derive(Debug, Clone)]
pub enum ProgressEvent {
    /// エージェント開始
    AgentStarted {
        agent_name: String,
    },
    /// エージェント進捗更新
    AgentProgress {
        agent_name: String,
        progress: f64, // 0.0-1.0
        message: Option<String>,
    },
    /// エージェント完了
    AgentCompleted {
        agent_name: String,
        success: bool,
        duration_secs: f64,
    },
    /// 全体進捗更新
    OverallProgress {
        completed: usize,
        total: usize,
        progress: f64, // 0.0-1.0
    },
}

/// 並列実行設定
#[derive(Debug, Clone)]
pub struct ParallelExecutorConfig {
    /// 最大同時実行数
    pub max_concurrent: usize,
    /// タイムアウト（None = タイムアウトなし）
    pub timeout: Option<Duration>,
    /// 進捗通知チャンネル（None = 進捗通知なし）
    pub progress_tx: Option<mpsc::UnboundedSender<ProgressEvent>>,
    /// メモリ制限（MB、None = 制限なし）
    pub memory_limit_mb: Option<usize>,
}

impl Default for ParallelExecutorConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 4,
            timeout: Some(Duration::from_secs(300)), // 5分
            progress_tx: None,
            memory_limit_mb: None,
        }
    }
}

/// 並列実行エグゼキューター
pub struct ParallelExecutor {
    config: ParallelExecutorConfig,
}

/// エージェントタスク
pub struct AgentTask {
    pub agent_name: String,
    pub executor: Box<
        dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<AgentResult>> + Send>>
            + Send
            + Sync,
    >,
}

impl ParallelExecutor {
    /// 新しいエグゼキューターを作成
    pub fn new(config: ParallelExecutorConfig) -> Self {
        Self { config }
    }

    /// デフォルト設定でエグゼキューターを作成
    pub fn with_defaults() -> Self {
        Self::new(ParallelExecutorConfig::default())
    }

    /// 並列実行を実行
    pub async fn execute_parallel(
        &self,
        tasks: Vec<AgentTask>,
    ) -> Result<Vec<AgentResult>> {
        let total = tasks.len();
        info!(
            "Starting parallel execution of {} agents (max_concurrent: {})",
            total, self.config.max_concurrent
        );

        // セマフォによる同時実行数制限
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrent));

        // 全体進捗通知
        if let Some(ref tx) = self.config.progress_tx {
            let _ = tx.send(ProgressEvent::OverallProgress {
                completed: 0,
                total,
                progress: 0.0,
            });
        }

        // 各タスクを並列実行
        let mut handles = Vec::new();
        for task in tasks {
            let sem = semaphore.clone();
            let progress_tx = self.config.progress_tx.clone();
            let timeout_duration = self.config.timeout;
            let agent_name = task.agent_name.clone();
            let agent_name_for_handle = agent_name.clone();

            // エージェント開始通知
            if let Some(ref tx) = progress_tx {
                let _ = tx.send(ProgressEvent::AgentStarted {
                    agent_name: agent_name.clone(),
                });
            }

            let handle = tokio::spawn(async move {
                // セマフォで同時実行数を制限
                let _permit = match sem.acquire().await {
                    Ok(p) => p,
                    Err(e) => {
                        error!("Failed to acquire semaphore: {}", e);
                        return Err(anyhow::anyhow!("Semaphore acquisition failed"));
                    }
                };

                let start_time = Instant::now();

                // 進捗通知用のクロージャ
                let send_progress = |progress: f64, message: Option<String>| {
                    if let Some(ref tx) = progress_tx {
                        let _ = tx.send(ProgressEvent::AgentProgress {
                            agent_name: agent_name.clone(),
                            progress,
                            message,
                        });
                    }
                };

                // タイムアウト付きで実行
                let result = if let Some(timeout_dur) = timeout_duration {
                    match timeout(timeout_dur, (task.executor)()).await {
                        Ok(Ok(result)) => {
                            send_progress(1.0, Some("Completed".to_string()));
                            Ok(result)
                        }
                        Ok(Err(e)) => {
                            error!("Agent '{}' failed: {}", agent_name, e);
                            send_progress(1.0, Some(format!("Failed: {}", e)));
                            Err(e)
                        }
                        Err(_) => {
                            let error_msg = format!("Agent '{}' timed out after {:?}", agent_name, timeout_dur);
                            error!("{}", error_msg);
                            send_progress(1.0, Some("Timed out".to_string()));
                            Err(anyhow::anyhow!(error_msg))
                        }
                    }
                } else {
                    match (task.executor)().await {
                        Ok(result) => {
                            send_progress(1.0, Some("Completed".to_string()));
                            Ok(result)
                        }
                        Err(e) => {
                            error!("Agent '{}' failed: {}", agent_name, e);
                            send_progress(1.0, Some(format!("Failed: {}", e)));
                            Err(e)
                        }
                    }
                };

                let duration_secs = start_time.elapsed().as_secs_f64();

                // 完了通知
                if let Some(ref tx) = progress_tx {
                    let success = result.is_ok();
                    let _ = tx.send(ProgressEvent::AgentCompleted {
                        agent_name: agent_name.clone(),
                        success,
                        duration_secs,
                    });
                }

                // エラーでも結果を返す（部分的な成功を許可）
                match result {
                    Ok(mut agent_result) => {
                        agent_result.duration_secs = duration_secs;
                        Ok(agent_result)
                    }
                    Err(e) => Ok(AgentResult {
                        agent_name: agent_name.clone(),
                        status: AgentStatus::Failed,
                        artifacts: vec![],
                        tokens_used: 0,
                        duration_secs,
                        error: Some(e.to_string()),
                    }),
                }
            });

            handles.push((agent_name_for_handle, handle));
        }

        // 結果を集約（エラーがあっても続行）
        let mut results = Vec::new();
        let mut completed_count = 0;

        for (agent_name, handle) in handles {
            match handle.await {
                Ok(Ok(result)) => {
                    if matches!(result.status, AgentStatus::Completed) {
                        info!("Agent '{}' completed successfully", agent_name);
                    } else {
                        warn!("Agent '{}' completed with status: {:?}", agent_name, result.status);
                    }
                    results.push(result);
                    completed_count += 1;
                }
                Ok(Err(e)) => {
                    error!("Agent '{}' execution error: {}", agent_name, e);
                    results.push(AgentResult {
                        agent_name: agent_name.clone(),
                        status: AgentStatus::Failed,
                        artifacts: vec![],
                        tokens_used: 0,
                        duration_secs: 0.0,
                        error: Some(e.to_string()),
                    });
                    completed_count += 1;
                }
                Err(e) => {
                    error!("Agent '{}' task panicked: {}", agent_name, e);
                    results.push(AgentResult {
                        agent_name: agent_name.clone(),
                        status: AgentStatus::Failed,
                        artifacts: vec![],
                        tokens_used: 0,
                        duration_secs: 0.0,
                        error: Some(format!("Task panicked: {e}")),
                    });
                    completed_count += 1;
                }
            }

            // 全体進捗通知
            if let Some(ref tx) = self.config.progress_tx {
                let progress = completed_count as f64 / total as f64;
                let _ = tx.send(ProgressEvent::OverallProgress {
                    completed: completed_count,
                    total,
                    progress,
                });
            }
        }

        let success_count = results
            .iter()
            .filter(|r| matches!(r.status, AgentStatus::Completed))
            .count();

        info!(
            "Parallel execution completed: {}/{} agents succeeded",
            success_count, total
        );

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_parallel_execution() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let config = ParallelExecutorConfig {
            max_concurrent: 2,
            timeout: Some(Duration::from_secs(10)),
            progress_tx: Some(tx),
            memory_limit_mb: None,
        };

        let executor = ParallelExecutor::new(config);

        let tasks = vec![
            AgentTask {
                agent_name: "agent1".to_string(),
                executor: Box::new(|| {
                    Box::pin(async move {
                        tokio::time::sleep(Duration::from_millis(100)).await;
                        Ok(AgentResult {
                            agent_name: "agent1".to_string(),
                            status: AgentStatus::Completed,
                            artifacts: vec![],
                            tokens_used: 100,
                            duration_secs: 0.1,
                            error: None,
                        })
                    })
                }),
            },
            AgentTask {
                agent_name: "agent2".to_string(),
                executor: Box::new(|| {
                    Box::pin(async move {
                        tokio::time::sleep(Duration::from_millis(50)).await;
                        Ok(AgentResult {
                            agent_name: "agent2".to_string(),
                            status: AgentStatus::Completed,
                            artifacts: vec![],
                            tokens_used: 50,
                            duration_secs: 0.05,
                            error: None,
                        })
                    })
                }),
            },
        ];

        let results = executor.execute_parallel(tasks).await.unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| matches!(r.status, AgentStatus::Completed)));

        // 進捗イベントの確認
        let mut events = Vec::new();
        while let Ok(event) = rx.try_recv() {
            events.push(event);
        }
        assert!(!events.is_empty());
    }
}
