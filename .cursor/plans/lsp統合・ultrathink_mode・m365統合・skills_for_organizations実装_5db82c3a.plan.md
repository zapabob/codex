---
name: LSP統合・Ultrathink Mode・M365統合・Skills for Organizations実装
overview: 4つの主要機能（LSP統合、Ultrathink Mode、Microsoft 365統合、Skills for Organizations）を実装し、Codexの機能を拡張する
todos:
  - id: lsp-client
    content: LSPクライアント実装（rust-analyzer、TypeScript Server等との統合）
    status: completed
  - id: lsp-diagnostics
    content: LSP診断マネージャー実装（リアルタイム診断情報の収集と配信）
    status: completed
  - id: lsp-gui
    content: GUI統合（LSP診断情報のリアルタイム表示）
    status: completed
  - id: ultrathink-chain
    content: 推論チェーンエンジン実装（深い推論チェーンの実行）
    status: completed
  - id: ultrathink-extend
    content: 既存思考プロセスシステムの拡張（Ultrathink Mode対応）
    status: completed
  - id: ultrathink-cli
    content: CLI/TUI統合（codex ultrathinkコマンド追加）
    status: completed
  - id: m365-client
    content: Office 365 API クライアント実装（Microsoft Graph API統合）
    status: completed
  - id: m365-auth
    content: OAuth 2.0認証管理実装
    status: completed
  - id: m365-mcp
    content: Office 365 MCPサーバー実装（Word、Excel、PowerPoint、Outlook操作）
    status: completed
  - id: org-manager
    content: 組織管理システム実装（組織の作成・管理、メンバー管理）
    status: completed
  - id: org-skills
    content: Skills共有機能実装（組織内Skillsの共有・バージョン管理）
    status: completed
  - id: org-cli
    content: CLI統合（組織作成・参加・Skills共有コマンド）
    status: completed
---

# LSP統合・Ultrathink Mode・M365統合・Skills for Organizations実装計画

## 概要

Codexに以下の4つの主要機能を追加します：

1. **LSP統合（リアルタイム診断）**: 既存のLSPサーバー（rust-analyzer、TypeScript Server等）との統合
2. **Ultrathink Mode（高度推論）**: 深い推論チェーンを実行するモード
3. **Microsoft 365統合**: Office 365 API統合
4. **Skills for Organizations**: 組織単位でのSkills共有・管理機能

## 実装フェーズ

### Phase 1: LSP統合（リアルタイム診断）

#### 1.1 LSPクライアント実装

- **ファイル**: `codex-rs/core/src/lsp/client.rs` (新規)
- **機能**:
  - `lsp-types`クレートを使用したLSPクライアント実装
  - rust-analyzer、TypeScript Server、Python Language Server等の接続管理
  - リアルタイム診断情報の取得と配信
  - コード補完、ホバー情報、シンボル検索の統合

#### 1.2 LSP診断マネージャー

- **ファイル**: `codex-rs/core/src/lsp/diagnostics.rs` (新規)
- **機能**:
  - 診断情報の収集と管理
  - リアルタイム診断のWebSocket配信
  - 診断情報のキャッシュと更新

#### 1.3 GUI統合

- **ファイル**: `gui/src/lib/lsp/LspClient.ts` (新規)
- **ファイル**: `gui/src/components/LspDiagnostics.tsx` (新規)
- **機能**:
  - LSP診断情報のリアルタイム表示
  - エディタ統合（問題表示、クイックフィックス）
  - 診断フィルタリングとソート

#### 1.4 MCP統合

- **ファイル**: `codex-rs/mcp-server/src/lsp_tool_handler.rs` (新規)
- **機能**:
  - LSP診断情報をMCPツールとして公開
  - 診断情報の取得・更新コマンド

### Phase 2: Ultrathink Mode（高度推論）

#### 2.1 推論チェーンエンジン

- **ファイル**: `codex-rs/core/src/reasoning/chain.rs` (新規)
- **機能**:
  - 複数ステップの深い推論チェーン実行
  - 推論ステップ間の依存関係管理
  - 推論結果の検証と反証

#### 2.2 既存思考プロセスシステムの拡張

- **ファイル**: `codex-rs/supervisor/src/thinking_process.rs` (拡張)
- **機能**:
  - Ultrathink Mode用の思考ステップタイプ追加
  - 深い推論チェーンの記録と可視化
  - 推論の信頼度スコアリング

#### 2.3 Ultrathink Mode設定

- **ファイル**: `codex-rs/core/src/reasoning/config.rs` (新規)
- **機能**:
  - 推論チェーンの深さ設定
  - 推論タイムアウト設定
  - 推論リソース制限

#### 2.4 CLI/TUI統合

- **ファイル**: `codex-rs/cli/src/commands/ultrathink.rs` (新規)
- **機能**:
  - `codex ultrathink`コマンド追加
  - 推論プロセスのリアルタイム表示
  - 推論結果の可視化

### Phase 3: Microsoft 365統合

#### 3.1 Office 365 API クライアント

- **ファイル**: `codex-rs/microsoft365/src/client.rs` (新規)
- **機能**:
  - Microsoft Graph API統合
  - OAuth 2.0認証フロー
  - Word、Excel、PowerPoint、Outlook API呼び出し

#### 3.2 Office 365 MCPサーバー

- **ファイル**: `codex-rs/mcp-server/src/microsoft365_tool_handler.rs` (新規)
- **機能**:
  - Word文書の読み取り・作成・編集
  - Excelスプレッドシートの操作
  - PowerPointプレゼンテーションの操作
  - Outlookメール・カレンダーの操作

#### 3.3 認証管理

- **ファイル**: `codex-rs/microsoft365/src/auth.rs` (新規)
- **機能**:
  - OAuth 2.0トークン管理
  - トークンリフレッシュ
  - 認証情報の安全な保存

#### 3.4 設定管理

- **ファイル**: `config.toml` (拡張)
- **機能**:
  - Microsoft 365統合設定セクション追加
  - クライアントID、テナントID設定

### Phase 4: Skills for Organizations

#### 4.1 組織管理システム

- **ファイル**: `codex-rs/core/src/organizations/mod.rs` (新規)
- **ファイル**: `codex-rs/core/src/organizations/manager.rs` (新規)
- **機能**:
  - 組織の作成・管理
  - 組織メンバーの管理
  - 組織レベルのSkills共有

#### 4.2 Skills共有機能

- **ファイル**: `codex-rs/core/src/skills/sharing.rs` (新規)
- **機能**:
  - 組織内Skillsの共有
  - Skillsのバージョン管理
  - Skillsの権限管理

#### 4.3 組織Skillsリポジトリ

- **ファイル**: `codex-rs/core/src/organizations/skills_repo.rs` (新規)
- **機能**:
  - 組織Skillsの保存・取得
  - Skillsの検索・フィルタリング
  - Skillsの使用統計

#### 4.4 CLI統合

- **ファイル**: `codex-rs/cli/src/commands/organization.rs` (新規)
- **機能**:
  - `codex org create` - 組織作成
  - `codex org join` - 組織参加
  - `codex org skills share` - Skills共有
  - `codex org skills list` - 組織Skills一覧

#### 4.5 データベーススキーマ

- **ファイル**: `codex-rs/core/src/organizations/schema.rs` (新規)
- **機能**:
  - 組織、メンバー、Skills共有のデータモデル
  - SQLite/PostgreSQL統合

## 技術スタック

### LSP統合

- `lsp-types` - LSPプロトコル型定義
- `tower-lsp` - LSPサーバー/クライアント実装
- WebSocket - リアルタイム診断配信

### Ultrathink Mode

- 既存の`thinking_process.rs`を拡張
- 推論チェーンエンジンの新規実装
- 推論結果の可視化

### Microsoft 365統合

- `reqwest` - HTTPクライアント
- `oauth2` - OAuth 2.0認証
- Microsoft Graph API SDK

### Skills for Organizations

- SQLite/PostgreSQL - データベース
- `diesel` または `sqlx` - ORM
- 既存のSkillsシステムを拡張

## 依存関係の追加

### Cargo.toml (codex-rs/core)

```toml
[dependencies]
lsp-types = "0.95"
tower-lsp = "0.20"
reqwest = { version = "0.11", features = ["json"] }
oauth2 = "4.4"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
```

## 設定ファイル拡張

### config.toml

```toml
[lsp]
enabled = true
servers = ["rust-analyzer", "typescript", "python"]
diagnostics_realtime = true

[ultrathink]
enabled = true
max_chain_depth = 10
timeout_seconds = 300

[microsoft365]
enabled = false
client_id = ""
tenant_id = ""
scopes = ["Files.ReadWrite", "Mail.ReadWrite"]

[organizations]
enabled = true
database_path = ".codex/organizations.db"
```

## テスト戦略

1. **LSP統合**: rust-analyzerとの統合テスト、診断情報の取得テスト
2. **Ultrathink Mode**: 推論チェーンの実行テスト、タイムアウトテスト
3. **Microsoft 365統合**: OAuth認証フローテスト、API呼び出しテスト
4. **Skills for Organizations**: 組織作成・参加テスト、Skills共有テスト

## 実装順序

1. Phase 1: LSP統合（既存の`lsp-types`依存を活用）
2. Phase 2: Ultrathink Mode（既存の思考プロセスシステムを拡張）
3. Phase 3: Microsoft 365統合（新規実装）
4. Phase 4: Skills for Organizations（既存のSkillsシステムを拡張）

## 注意事項

- LSP統合は既存の`lsp-types`クレートを活用
- Ultrathink Modeは既存の`thinking_process.rs`を拡張
- Microsoft 365統合はOAuth認証の実装が必要
- Skills for Organizationsはデータベーススキーマの設計が必要