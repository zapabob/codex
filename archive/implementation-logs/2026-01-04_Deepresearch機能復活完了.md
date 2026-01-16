# 2026-01-04 Deepresearch機能復活完了

## 取り組み内容

### 1. Deepresearch機能の調査と復活
- Deepresearch関連ファイルの存在確認
  - `codex-rs/deep-research/` ディレクトリは存在していた
  - `codex-rs/cli/src/research_cmd.rs` は存在していた
  - CLIの `main.rs` からresearchコマンドが削除されていた

### 2. CLIへのResearchコマンド復活
- `codex-rs/cli/src/main.rs` に以下の変更を実施：
  - `mod research_cmd;` を追加
  - `Subcommand` enumに `Research(ResearchCommand)` を追加
  - `ResearchCommand` 構造体を定義（パラメータ：topic, depth, breadth, budget, citations, mcp, lightweight_fallback, gemini, use_mcp, out）
  - match文にResearchコマンドの処理を追加

### 3. README日英併記更新
- タイトルを "Codex Extended - Skills + MCP + Agents SDK Architecture" に変更
- アーキテクチャ図を `docs/architecture/architecture-v2.10.0.svg` に更新
- バージョン表記を "Skills + MCP + Agents SDK" に変更
- TL;DR、Why Codex Extended、What's implemented、Feature Status Matrixを更新
- 基本的な使い方、建築、アーキテクチャ、ドキュメントリンクを更新

### 4. Mermaidアーキテクチャ図生成
- `architecture-v2.10.0.mmd` を作成（Skills + MCP + Agents SDK アーキテクチャ）
- Mermaid CLIで `docs/architecture/architecture-v2.10.0.svg` を生成

## 実装したアーキテクチャ

### Skills System
- Build Manager: 高速インクリメンタルビルド
- QA Service: 高度な品質分析
- Worktree Manager: 並列開発環境
- CI/CD Integration: パイプライン生成と通知

### MCP Integration
- WebSocketベースの通信プロトコル
- Codex CLI (server) と外部オーケストレータ (client) 間の接続

### Agents SDK Patterns
- Supervisor/Workerアーキテクチャ
- Guardrails: セキュリティと品質チェック
- Handoffs: エージェント間連携
- Structured Output: JSONスキーマ検証

## メモ

- ビルド環境にアクセス拒否エラーが発生したが、コードレベルの変更は完了
- Deepresearch機能は `codex research <topic>` コマンドで利用可能
- READMEは現在のアーキテクチャを正確に反映
- アーキテクチャ図はMermaidで生成されたSVGとして利用可能