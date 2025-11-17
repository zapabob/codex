<!-- d1e7680c-cc20-47da-a63c-d4adaa1434c2 e8bc46b2-e168-416a-bc8f-d96eb10b6634 -->
# MCP統合並列開発QC管理システム実装計画

## 1. MCP統合拡張（GeminiCLI/ClaudeCode）

### 1.1 MCPサーバー統合

**対象ファイル**:

- `codex-rs/core/src/agents/mcp_codex_integration.rs` - 既存MCP統合
- `codex-rs/gemini-cli-mcp-server/src/main.rs` - GeminiCLI MCPサーバー
- 新規: `codex-rs/claude-code-mcp-server/src/main.rs` - ClaudeCode MCPサーバー

**実装内容**:

- GeminiCLI MCPサーバーの完全統合（GUI ↔ CLI一対一対応）
- ClaudeCode MCPサーバーの新規実装
- MCPプロトコル経由でのコマンド実行
- リアルタイム進捗通知（SSE/WebSocket）

### 1.2 GUI ↔ CLI一対一対応

**対象ファイル**:

- `codex-rs/tauri-gui/src-tauri/src/mcp_bridge.rs` - MCPブリッジ（新規）
- `codex-rs/tauri-gui/src/components/mcp/MCPAgentPanel.tsx` - エージェント管理パネル（新規）

**実装内容**:

- 各MCPサーバー（Codex、GeminiCLI、ClaudeCode）への接続管理
- 接続状態の可視化（接続中/切断/エラー）
- コマンド送信とレスポンス受信
- エラーハンドリングとリトライ機能

## 2. 動的リソース管理（コア数*2上限）

### 2.1 リソース管理システム

**対象ファイル**:

- `codex-rs/core/src/orchestration/resource_manager.rs` - リソース管理（新規）
- `codex-rs/tauri-gui/src-tauri/src/resource_manager.rs` - Tauriコマンド（新規）

**実装内容**:

- CPUコア数の自動検出
- 動的リソース上限計算（コア数 × 2）
- 実行中タスクのリソース使用量追跡
- リソース不足時のキューイング

### 2.2 GUIリソース制御

**対象ファイル**:

- `codex-rs/tauri-gui/src/components/orchestration/ResourceControl.tsx` - リソース制御パネル（新規）

**実装内容**:

- 現在のリソース使用状況表示（使用中/上限）
- エージェント数の増減ボタン（+/-）
- 数値入力による直接指定
- リアルタイム更新（2秒間隔）

## 3. Git Worktree並列開発

### 3.1 並列Worktree管理

**対象ファイル**:

- `codex-rs/core/src/orchestration/worktree_manager.rs` - 既存Worktree管理（拡張）
- `codex-rs/core/src/orchestration/parallel_worktree.rs` - 並列Worktree実行（新規）

**実装内容**:

- 複数エージェント用Worktreeの同時作成
- 各Worktreeでの独立した開発
- Worktree間の競合検出と解決
- 最適コードの自動選出とマージ

### 3.2 コード品質評価システム

**対象ファイル**:

- `codex-rs/core/src/quality/vulnerability_checker.rs` - 脆弱性チェック（新規）
- `codex-rs/core/src/quality/statistical_analyzer.rs` - 統計分析（新規）
- `codex-rs/core/src/quality/quantum_optimizer.rs` - 量子最適化（新規）

**実装内容**:

- **脆弱性チェック**: CVEスキャン、セキュリティパターン検出
- **統計分析**: 
- 分散分析（ANOVA）によるコード品質評価
- アルゴリズム複雑度分析
- 型定義の一貫性チェック
- 警告の統計的有意性検定
- **量子最適化**: 
- 複数Worktreeの結果を量子アルゴリズムで最適化
- レビューAIによる合議システム
- 最適コードの自動選出

## 4. 品質工程管理GUI

### 4.1 ガントチャート

**対象ファイル**:

- `codex-rs/tauri-gui/src/components/project/GanttChart.tsx` - ガントチャート（新規）

**実装内容**:

- dhtmlx-ganttまたはreact-gantt-chart使用
- エージェントタスクの時系列表示
- 依存関係表示
- 進捗状況可視化
- ドラッグ&ドロップでタスク調整

### 4.2 カンバンボード

**対象ファイル**:

- `codex-rs/tauri-gui/src/components/project/KanbanBoard.tsx` - カンバンボード（新規）

**実装内容**:

- react-beautiful-dnd使用
- カラム: To Do, In Progress, Review, Done
- カード表示: タスク名、担当エージェント、期限、優先度
- ドラッグ&ドロップでステータス変更
- フィルター・ソート機能

### 4.3 品質工程管理表

**対象ファイル**:

- `codex-rs/tauri-gui/src/components/project/ProjectManagementTable.tsx` - 工程管理表（新規）

**実装内容**:

- ag-gridまたはreact-table使用
- タスク一覧表示
- ソート・フィルター・検索
- エクスポート機能（CSV、Excel）
- バッチ操作

### 4.4 QCコントロールチャート

**対象ファイル**:

- `codex-rs/tauri-gui/src/components/quality/QCControlCharts.tsx` - QC管理図（新規）

**実装内容**:

- X-bar chart（平均値管理図）
- R chart（範囲管理図）
- p-chart（不良率管理図）
- 管理限界線の自動計算
- 異常値の検出とアラート

### 4.5 統計分析チャート

**対象ファイル**:

- `codex-rs/tauri-gui/src/components/quality/StatisticalAnalysisCharts.tsx` - 統計分析チャート（新規）

**実装内容**:

- ガウス分布チャート（ヒストグラム + 正規分布曲線）
- 分散分析チャート（F統計量、p値表示）
- ボックスプロット
- 散布図（相関分析）
- 統計的有意性の可視化

## 5. 統合ダッシュボード

### 5.1 並列開発ダッシュボード

**対象ファイル**:

- `codex-rs/tauri-gui/src/pages/ParallelDevelopmentDashboard.tsx` - 並列開発ダッシュボード（新規）

**実装内容**:

- ガントチャート、カンバン、工程管理表の統合表示
- タブ切り替えまたは分割画面
- リアルタイム更新（WebSocket/SSE）
- カスタマイズ可能なレイアウト

### 5.2 リアルタイムログ表示

**対象ファイル**:

- `codex-rs/tauri-gui/src/components/orchestration/RealtimeLogViewer.tsx` - リアルタイムログ（新規）

**実装内容**:

- 各エージェントの実行ログをリアルタイム表示
- ログレベル別フィルタリング（INFO/WARN/ERROR）
- 検索機能
- 自動スクロール
- ログエクスポート機能

### 5.3 ボタン操作UI

**対象ファイル**:

- `codex-rs/tauri-gui/src/components/orchestration/ControlPanel.tsx` - 制御パネル（拡張）

**実装内容**:

- エージェント起動/停止ボタン
- リソース増減ボタン（+/-）
- 並列実行開始/停止ボタン
- Worktree作成/削除ボタン
- 最適コード選出ボタン

## 6. 量子最適化レビューシステム

### 6.1 レビューAI合議システム

**対象ファイル**:

- `codex-rs/core/src/quality/review_consensus.rs` - レビュー合議（新規）

**実装内容**:

- 複数レビューAI（Codex、GeminiCLI、ClaudeCode）による評価
- 量子最適化アルゴリズムによる最適コード選出
- 評価スコアの重み付け
- 合議結果の可視化

### 6.2 評価指標

**実装内容**:

- アルゴリズム効率性
- 脆弱性スコア
- 型定義の一貫性
- 警告の統計的有意性
- コード品質総合スコア

## 実装順序

1. MCP統合拡張（GeminiCLI/ClaudeCode）
2. 動的リソース管理システム
3. Git Worktree並列開発拡張
4. コード品質評価システム（脆弱性、統計分析）
5. 品質工程管理GUI（ガント、カンバン、工程管理表）
6. QCコントロールチャート
7. 統計分析チャート
8. 量子最適化レビューシステム
9. 統合ダッシュボード
10. リアルタイムログ表示

## 検証項目

- MCPサーバーが正しく接続されるか
- リソース上限（コア数×2）が正しく適用されるか
- エージェント数の増減が正常に動作するか
- Git Worktreeが並列で作成・管理されるか
- コード品質評価が正しく実行されるか
- 統計分析が統計的に有意な結果を返すか
- 量子最適化が最適コードを選出できるか
- GUIがリアルタイムで更新されるか
- ガントチャート、カンバン、工程管理表が正しく表示されるか

### To-dos

- [ ] GeminiCLI/ClaudeCode MCP統合（GUI ↔ CLI一対一対応）
- [ ] 動的リソース管理（コア数*2上限、増減ボタン）
- [ ] Git Worktree並列開発拡張
- [ ] コード品質評価システム（脆弱性、統計分析、量子最適化）
- [ ] 品質工程管理GUI（ガント、カンバン、工程管理表）
- [ ] QCコントロールチャート実装
- [ ] 統計分析チャート実装
- [ ] 量子最適化レビューシステム（合議、最適コード選出）
- [ ] 統合ダッシュボード（リアルタイム更新）
- [ ] リアルタイムログ表示UI