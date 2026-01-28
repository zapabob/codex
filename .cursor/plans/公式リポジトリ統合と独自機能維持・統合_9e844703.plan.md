---
name: 公式リポジトリ統合と独自機能維持・統合
overview: 公式リポジトリ（upstream/main）の最新機能・バグフィクス・脆弱性更新を取り込みつつ、独自機能（git worktree並列開発、QC最適化コンペティション、A2A通信等）を維持。公式機能と独自機能が同等の場合は、公式機能に独自機能の利点を取り入れ、欠点を補完する形で統合する。
todos:
  - id: upstream-fetch-analysis
    content: Upstream最新取得と差分分析（コミット数、重要コミット、ファイル差分、コンフリクト予測）
    status: completed
  - id: feature-comparison
    content: 公式機能と独自機能の比較分析（並列実行、worktree管理、QC評価、A2A通信等）
    status: completed
  - id: merge-execution
    content: マージ実行とコンフリクト解決（公式優先、独自保持、統合戦略の適用）
    status: completed
  - id: integration-verification
    content: 統合後の検証（ビルド、テスト、独自機能の動作確認）
    status: in_progress
  - id: implementation-log
    content: 実装ログの作成（差分、比較結果、統合戦略、検証結果）
    status: pending
isProject: false
---

# 公式リポジトリ統合と独自機能維持・統合

## 現状確認

### 既存の独自機能（保護対象）

**Git Worktree並列開発**:

- `codex-rs/core/src/orchestration/worktree_manager.rs` - WorktreeManager
- `codex-rs/core/src/orchestration/integrated_competition.rs` - IntegratedCompetitionRunner
- `codex-rs/core/src/orchestration/conflict_prevention.rs` - コンフリクト予測

**並列非同期サブエージェント**:

- `codex-rs/core/src/agents/parallel_executor.rs` - ParallelExecutor
- `codex-rs/core/src/agents/runtime.rs` - delegate_parallel
- `codex-rs/core/src/orchestration/parallel_execution.rs` - ParallelOrchestrator

**A2A通信**:

- `codex-rs/core/src/a2a_communication.rs` - A2ACommunicationManager
- エージェント間メッセージング、協調、タスク委譲

**QC最適化コンペティション**:

- `codex-rs/core/src/qc/` - QcAgent、量子最適化、数理最適化
- `codex-rs/core/src/orchestration/qc_merger.rs` - QcMerger
- `codex-rs/core/src/orchestration/qc_optimization_evaluator.rs` - QC最適化評価
- `codex-rs/core/src/orchestration/qc_logger.rs` - QCログ記録

**保護方法**:

- `#[cfg(feature = "custom-features")]` で条件付きコンパイル
- `codex-rs/core/Cargo.toml` に `custom-features` フラグ定義済み
- `codex-rs/cli/Cargo.toml` で `custom-features` 有効化済み

## 実装フェーズ

### Phase 1: Upstream最新取得と差分分析

**実行スクリプト**: `scripts/upstream-merge.ps1`

**手順**:

1. `git fetch upstream` で最新を取得
2. `git log main..upstream/main --oneline` で差分コミット確認
3. 重要なコミットの特定:
  - セキュリティ修正（CVE、security fix）
  - バグフィクス（bug fix、fix）
  - 新機能（feat）
4. ファイル差分の確認: `git diff --stat main upstream/main`
5. マージコンフリクトの予測: `git merge-tree`

**出力確認項目**:

- upstream/mainより何コミット遅れているか
- セキュリティ修正の有無
- 脆弱性更新（CVE対応）の確認
- 公式の新機能追加の有無

### Phase 2: 公式機能と独自機能の比較分析

**実行スクリプト**: `scripts/compare-official-custom.ps1`（拡張版を作成）

**比較対象機能**:

1. **並列実行機能**
  - 公式: 基本的な並列実行（存在する場合）
  - 独自: ParallelExecutor（進捗追跡、リソース制限、エラーハンドリング強化）
  - 統合戦略: 公式の基本機能を採用し、独自の進捗追跡・リソース制限を追加
2. **Git Worktree管理**
  - 公式: 基本的なworktree管理（存在する場合）
  - 独自: WorktreeManager + コンフリクト予測 + A2A統合
  - 統合戦略: 公式の基本機能を採用し、独自のコンフリクト予測・A2A統合を追加
3. **QC/品質評価**
  - 公式: 基本的な品質チェック（存在する場合）
  - 独自: QC最適化コンペティション（量子最適化、数理最適化、QC管理）
  - 統合戦略: 公式の基本機能を採用し、独自のQC最適化評価を追加
4. **A2A通信**
  - 公式: 存在しない可能性が高い
  - 独自: A2ACommunicationManager（完全な独自実装）
  - 統合戦略: 独自機能を完全保持

**比較結果の記録**:

- `_docs/2026-01-29_公式機能と独自機能の比較分析{main}.md` に記録
- 各機能の統合戦略を明確化

### Phase 3: マージ実行とコンフリクト解決

**作業ブランチ作成**:

```powershell
git checkout -b upstream-sync-$(Get-Date -Format 'yyyy-MM-dd')
```

**マージ実行**:

```powershell
git merge upstream/main --no-ff -m "feat: Merge upstream/main with custom features preservation"
```

**コンフリクト解決方針**:

1. **公式の変更を優先する場合**:
  - セキュリティ修正
  - バグフィクス
  - コア機能の改善
2. **独自機能を保持する場合**:
  - `#[cfg(feature = "custom-features")]` で保護されたモジュール
  - 公式に存在しない独自機能
  - 公式より優れている独自機能
3. **統合する場合**（同等機能）:
  - 公式の基本実装を採用
  - 独自機能の利点（進捗追跡、リソース制限、エラーハンドリング等）を公式実装に追加
  - 公式実装の欠点を独自機能で補完

**コンフリクト解決の具体例**:

- `codex-rs/core/src/agents/` - 公式版を優先、独自のParallelExecutor機能を統合
- `codex-rs/core/src/orchestration/` - 独自機能を完全保持（公式に存在しない）
- `codex-rs/core/src/qc/` - 独自機能を完全保持（公式に存在しない）
- `codex-rs/core/src/a2a_communication.rs` - 独自機能を完全保持（公式に存在しない）

### Phase 4: 統合後の検証

**ビルド検証**:

```powershell
cd codex-rs
cargo build --workspace --features custom-features
cargo check --workspace --features custom-features
cargo test --workspace --features custom-features
```

**独自機能の動作確認**:

- Git worktree並列開発の動作確認
- QC最適化コンペティションの動作確認
- A2A通信の動作確認
- 並列サブエージェント実行の動作確認

**検証項目**:

- ビルドが成功する
- テストが全て通過する
- 独自機能が正常に動作する
- 公式の新機能が正常に動作する
- 統合された機能（公式+独自）が正常に動作する

### Phase 5: 実装ログの作成

**ログファイル**: `_docs/2026-01-29_公式リポジトリ統合と独自機能維持・統合{main}.md`

**記録内容**:

- upstream/mainとの差分（コミット数、重要コミット）
- 公式機能と独自機能の比較結果
- 統合戦略の決定理由
- コンフリクト解決の詳細
- 統合後の検証結果
- 残存する課題や今後の改善点

## 保護対象ディレクトリ・モジュール

**完全保持（公式に存在しない）**:

- `.codex/` - 独自機能・スキル・設定
- `.cursor/` - Cursor設定・計画・ルール
- `codex-rs/core/src/orchestration/` - オーケストレーション（全体）
- `codex-rs/core/src/qc/` - QC機能（全体）
- `codex-rs/core/src/a2a_communication.rs` - A2A通信

**条件付き統合（公式に同等機能がある場合）**:

- `codex-rs/core/src/agents/` - 公式版を優先、独自機能の利点を統合
- `codex-rs/core/src/plan/` - 公式版を優先、独自機能の利点を統合

## リスクと対策

**リスク1**: 大量のマージコンフリクト

- **対策**: 段階的なマージ、各機能ごとに独立してテスト

**リスク2**: 独自機能の動作不良

- **対策**: マージ後に各独自機能の統合テストを実行

**リスク3**: API互換性の問題

- **対策**: 型定義の整合性を確認、必要に応じてアダプター層を追加

**リスク4**: 統合された機能の品質低下

- **対策**: QC最適化コンペティションで統合後の品質を評価

## 成功基準

1. upstream/mainの最新機能・バグフィクス・セキュリティ修正が取り込まれている
2. すべての独自機能が`custom-features`フラグで保護されている
3. 公式機能と独自機能が同等の場合、公式機能に独自機能の利点が統合されている
4. ビルドが成功し、型エラーがない
5. 独自機能が正常に動作している
6. 公式リポジトリとの整合性が維持されている

