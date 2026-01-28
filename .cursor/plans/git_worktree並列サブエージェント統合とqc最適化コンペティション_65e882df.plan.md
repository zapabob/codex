---
name: Git Worktree並列サブエージェント統合とQC最適化コンペティション
overview: git worktreeと並列非同期サブエージェント実装による同一リポジトリでのコンフリクト予防、既存の予防戦略とA2A通信の統合、QCエージェントによる数理最適化・量子最適化・QC管理を用いたコンペティション式実装選出、クリーンビルドとバイナリ上書きインストールを実現する包括的な計画
todos:
  - id: integrated-competition
    content: 統合コンペティション実行システムの作成（worktree、並列実行、A2A通信、QC最適化の統合）
    status: completed
  - id: conflict-a2a-integration
    content: コンフリクト予防とA2A通信の統合（コンフリクト予測、予防的調整、リアルタイム監視）
    status: completed
  - id: qc-optimization-evaluator
    content: QC最適化による評価システムの統合（数理最適化、量子最適化、QC管理）
    status: completed
  - id: competition-selection-logging
    content: コンペティション式選出とログ記録の強化（スコアリング、選出、詳細ログ）
    status: completed
  - id: clean-build-install
    content: クリーンビルドとバイナリ上書きインストール（既存スクリプトの改善）
    status: in_progress
isProject: false
---

# Git Worktree並列サブエージェント統合とQC最適化コンペティション

## 現状分析

### 既存実装の確認

**Git Worktree管理**:

- `codex-rs/core/src/orchestration/worktree_manager.rs`: WorktreeManager実装済み
- `codex-rs/utils/git/src/worktree.rs`: 汎用worktree管理実装済み
- 機能: worktree作成、削除、リスト、マージ

**並列非同期サブエージェント**:

- `codex-rs/core/src/agents/parallel_executor.rs`: 新規作成済み（エラーハンドリング、進捗追跡、リソース制限）
- `codex-rs/core/src/agents/runtime.rs`: delegate_parallelメソッド改善済み
- `codex-rs/core/src/orchestration/parallel_execution.rs`: ParallelOrchestrator実装済み

**A2A通信**:

- `codex-rs/core/src/a2a_communication.rs`: A2ACommunicationManager実装済み
- 機能: メッセージング、協調、タスク委譲、信頼管理

**QCエージェント**:

- `codex-rs/core/src/qc/agent.rs`: QcAgent実装済み
- `codex-rs/core/src/qc/quantum.rs`: 量子最適化（QAOA、VQE）実装済み
- `codex-rs/core/src/qc/mathematical.rs`: 数理最適化（線形計画法、凸最適化）実装済み
- `codex-rs/core/src/qc/agent_coordination.rs`: AgentCoordinator実装済み

**コンペティション機能**:

- `codex-rs/core/src/agents/competition.rs`: CompetitionRunner実装済み
- `codex-rs/core/src/orchestration/qc_merger.rs`: QcMerger実装済み（最良worktree選択）
- `codex-rs/core/src/orchestration/qc_logger.rs`: QcLogger実装済み（ログ記録）

**コンフリクト予防**:

- `tools/conflict_prevention_engine.py`: ConflictPreventionEngine実装済み

## 実装タスク

### Task 1: 統合コンペティション実行システムの作成

**目的**: git worktree、並列非同期サブエージェント、A2A通信、QC最適化を統合したコンペティション実行システム

**実装ファイル**: `codex-rs/core/src/orchestration/integrated_competition.rs` (新規作成)

**機能要件**:

1. **Worktree作成と並列実行**
  - 各エージェントにworktreeを作成
  - 並列非同期で実行
  - A2A通信でエージェント間協調
2. **QC最適化による評価**
  - 数理最適化によるリソース配分評価
  - 量子最適化による品質評価
  - QC管理による総合評価
3. **コンペティション式選出**
  - 各worktreeの結果をQC分析
  - スコアリング（テスト、パフォーマンス、シンプリシティ）
  - 最良実装の自動選出
4. **ログ記録**
  - 各worktreeの実行ログ
  - QC分析結果の記録
  - 選出理由の記録

**実装内容**:

```rust
// codex-rs/core/src/orchestration/integrated_competition.rs
pub struct IntegratedCompetitionRunner {
    worktree_manager: WorktreeManager,
    parallel_executor: ParallelExecutor,
    a2a_manager: Arc<A2ACommunicationManager>,
    qc_agent: Arc<QcAgent>,
    qc_merger: QcMerger,
    qc_logger: QcLogger,
    conflict_prevention: ConflictPreventionEngine,
}

impl IntegratedCompetitionRunner {
    pub async fn run_competition(
        &self,
        task: CompetitionTask,
    ) -> Result<CompetitionResult> {
        // 1. Worktree作成（コンフリクト予防）
        let worktrees = self.create_worktrees_for_agents(&task.agents).await?;
        
        // 2. A2A通信でエージェント間協調
        self.setup_a2a_coordination(&worktrees).await?;
        
        // 3. 並列実行（各worktreeで独立実行）
        let results = self.execute_parallel_in_worktrees(worktrees, &task).await?;
        
        // 4. QC最適化による評価
        let qc_scores = self.evaluate_with_qc_optimization(&results).await?;
        
        // 5. コンペティション式選出
        let winner = self.select_best_implementation(&results, &qc_scores).await?;
        
        // 6. ログ記録
        self.log_competition_results(&results, &qc_scores, &winner).await?;
        
        // 7. 最良実装のマージ
        self.merge_winner_worktree(&winner).await?;
        
        Ok(CompetitionResult { winner, scores: qc_scores, logs: ... })
    }
}
```

### Task 2: コンフリクト予防とA2A通信の統合

**目的**: 既存のコンフリクト予防戦略とA2A通信を統合し、エージェント間でコンフリクト情報を共有

**実装ファイル**: 

- `codex-rs/core/src/orchestration/conflict_prevention.rs` (新規作成、Rust実装)
- `codex-rs/core/src/orchestration/integrated_competition.rs` (更新)

**統合内容**:

1. **コンフリクト予測**
  - 各worktreeでの変更を予測
  - コンフリクトリスクの評価
  - A2A通信でリスク情報を共有
2. **予防的調整**
  - エージェント間で変更範囲を調整
  - 重複ファイルの回避
  - 依存関係の管理
3. **リアルタイム監視**
  - 実行中のコンフリクト検出
  - A2A通信で即座に通知
  - 動的な調整

### Task 3: QC最適化による評価システムの統合

**目的**: 数理最適化、量子最適化、QC管理を統合した評価システム

**実装ファイル**: `codex-rs/core/src/orchestration/qc_optimization_evaluator.rs` (新規作成)

**評価内容**:

1. **数理最適化評価**
  - リソース配分の最適化
  - パフォーマンスボトルネックの特定
  - コストベネフィット分析
2. **量子最適化評価**
  - QAOAによる品質最適化
  - VQEによる固有値問題解決
  - 量子インスパイアされた最適化
3. **QC管理評価**
  - コード品質スコア
  - 可読性、保守性、パフォーマンス、セキュリティ
  - 総合評価スコア

### Task 4: コンペティション式選出とログ記録の強化

**目的**: コンペティション式で最良実装を選出し、詳細なログを記録

**実装ファイル**: 

- `codex-rs/core/src/orchestration/integrated_competition.rs` (更新)
- `codex-rs/core/src/orchestration/qc_logger.rs` (更新)

**選出ロジック**:

1. **スコアリング**
  - テスト結果スコア（50%）
  - パフォーマンススコア（30%）
  - シンプリシティスコア（20%）
  - QC最適化スコア（ボーナス）
2. **選出**
  - 総合スコアが最高のworktreeを選出
  - スコアが同点の場合はQC最適化スコアで決定
  - 選出理由を詳細に記録
3. **ログ記録**
  - 各worktreeの実行ログ
  - QC分析結果
  - スコアリング詳細
  - 選出理由
  - マージ結果

### Task 5: クリーンビルドとバイナリ上書きインストール

**目的**: 選出された最良実装をクリーンビルドし、バイナリを上書きインストール

**実装ファイル**: `scripts/competition-build-install.ps1` (新規作成)

**機能要件**:

1. **クリーンビルド**
  - `cargo clean`の実行
  - `cargo build --release -p codex-cli --features custom-features`
2. **進捗表示**
  - tqdm風の進捗表示
  - 残り時間・経過時間表示
3. **上書きインストール**
  - 実行中のプロセスを自動検出・終了
  - コピーアンドペーストで上書きインストール

## 実装順序

1. **Task 1**: 統合コンペティション実行システムの作成
2. **Task 2**: コンフリクト予防とA2A通信の統合
3. **Task 3**: QC最適化による評価システムの統合
4. **Task 4**: コンペティション式選出とログ記録の強化
5. **Task 5**: クリーンビルドとバイナリ上書きインストール

## 成功基準

### Task 1

- 統合コンペティション実行システムが正常に動作する
- Worktree作成と並列実行が機能する
- A2A通信でエージェント間協調が機能する

### Task 2

- コンフリクト予測が機能する
- A2A通信でリスク情報が共有される
- 予防的調整が機能する

### Task 3

- 数理最適化評価が機能する
- 量子最適化評価が機能する
- QC管理評価が機能する

### Task 4

- コンペティション式選出が機能する
- 詳細なログが記録される
- 選出理由が明確に記録される

### Task 5

- クリーンビルドが正常に実行される
- バイナリが正常にインストールされる
- 上書きインストールが機能する

## リスクと対策

**リスク1**: Worktree作成の失敗

- **対策**: フォールバックでメインディレクトリを使用、エラーハンドリング強化

**リスク2**: A2A通信の遅延

- **対策**: タイムアウト設定、非同期処理の最適化

**リスク3**: QC最適化の計算コスト

- **対策**: キャッシング、並列処理、必要に応じて簡易評価モード

**リスク4**: コンフリクト予測の精度

- **対策**: 複数の予測手法を組み合わせ、実際のマージで検証

