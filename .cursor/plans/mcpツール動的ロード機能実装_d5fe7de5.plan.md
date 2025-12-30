---
name: MCPツール動的ロード機能実装
overview: MCPツールを実行時に動的にロード・アンロードできる包括的なシステムを実装します。ファイル監視、APIエンドポイント、CLIコマンドの3つの方法をサポートし、プラグイン形式でのMCPツール追加とホットリロード機能を含みます。さらに、トークン節約機能として、必要なツールのみを選択的にロードし、使用頻度に基づく自動アンロード、ツール説明の圧縮、プロンプトへの選択的追加を実装します。
todos:
  - id: dynamic_loader_core
    content: DynamicMcpLoaderのコア実装（add_server, remove_server, reload_server）
    status: completed
  - id: connection_manager_extend
    content: McpConnectionManagerに動的ロード用メソッドを追加
    status: completed
  - id: file_watcher
    content: McpFileWatcherの実装（設定ファイル監視）
    status: completed
  - id: plugin_loader
    content: McpPluginLoaderの実装（プラグインディレクトリスキャン）
    status: completed
  - id: api_server
    content: McpApiServerの実装（REST APIエンドポイント）
    status: completed
  - id: cli_commands
    content: CLIコマンドの拡張（mcp_cmd.rsに動的ロードコマンド追加）
    status: completed
  - id: config_integration
    content: Config構造体への動的ロード設定追加
    status: completed
  - id: codex_integration
    content: Codex構造体へのDynamicMcpLoader統合
    status: completed
  - id: tests
    content: ユニットテスト・統合テストの実装
    status: completed
  - id: documentation
    content: プラグイン開発ガイドとAPI仕様書の作成
    status: completed
  - id: token_optimizer
    content: McpTokenOptimizerの実装（ツール使用頻度追跡、自動アンロード、説明圧縮）
    status: completed
  - id: selective_loading
    content: 選択的ツールロード機能（タスクに必要なツールのみをロード）
    status: completed
  - id: prompt_integration
    content: プロンプト構築時のツール選択的追加（必要なツールのみをプロンプトに含める）
    status: completed
---

# MCPツール動的ロード機能実装計画

## アーキテクチャ概要

現在のMCP実装は起動時に設定ファイル（`config.toml`、`.codex/mcp-servers.yaml`）から読み込む静的ロード方式です。これを拡張して、実行時にMCPサーバーを追加・削除・更新できる動的ロード機能を実装します。

## 実装コンポーネント

### 1. 動的ロードマネージャー (`DynamicMcpLoader`)

**ファイル**: `codex-rs/core/src/mcp_dynamic_loader.rs` (新規作成)

**責務**:

- MCPサーバーの動的追加・削除・更新
- プラグインディレクトリの監視
- 設定ファイルのホットリロード
- APIエンドポイント経由の操作

**主要メソッド**:

```rust
pub struct DynamicMcpLoader {
    connection_manager: Arc<McpConnectionManager>,
    plugin_dir: PathBuf,
    file_watcher: Option<FileWatcher>,
    api_server: Option<ApiServer>,
}

impl DynamicMcpLoader {
    pub async fn add_server(&self, config: McpServerConfig) -> Result<String>;
    pub async fn remove_server(&self, server_name: &str) -> Result<()>;
    pub async fn reload_server(&self, server_name: &str) -> Result<()>;
    pub async fn list_servers(&self) -> Vec<String>;
    pub async fn start_file_watcher(&mut self) -> Result<()>;
    pub async fn start_api_server(&mut self, port: u16) -> Result<()>;
}
```

### 2. ファイルシステム監視 (`McpFileWatcher`)

**ファイル**: `codex-rs/core/src/mcp_file_watcher.rs` (新規作成)

**責務**:

- 設定ファイル（`.codex/mcp-servers.yaml`）の変更監視
- プラグインディレクトリ（`.codex/mcp-plugins/`）の監視
- 変更検出時の自動リロード

**実装**:

- `notify`クレートを使用（既存の依存関係を確認）
- または`tokio::fs`とポーリング方式

### 3. プラグインシステム (`McpPluginLoader`)

**ファイル**: `codex-rs/core/src/mcp_plugin_loader.rs` (新規作成)

**責務**:

- プラグインディレクトリからMCPサーバー設定を自動検出
- プラグインメタデータ（`plugin.toml`）の読み込み
- プラグインの有効化・無効化

**プラグイン構造**:

```
.codex/mcp-plugins/
├── github-plugin/
│   ├── plugin.toml      # プラグインメタデータ
│   └── server.toml      # MCPサーバー設定
└── serena-plugin/
    ├── plugin.toml
    └── server.toml
```

### 4. APIエンドポイント (`McpApiServer`)

**ファイル**: `codex-rs/core/src/mcp_api_server.rs` (新規作成)

**責務**:

- REST API経由でMCPサーバーを操作
- 認証・認可（オプション）

**エンドポイント**:

- `POST /api/mcp/servers` - サーバー追加
- `DELETE /api/mcp/servers/{name}` - サーバー削除
- `PUT /api/mcp/servers/{name}/reload` - サーバーリロード
- `GET /api/mcp/servers` - サーバー一覧
- `GET /api/mcp/servers/{name}/tools` - ツール一覧

### 5. CLIコマンド拡張

**ファイル**: `codex-rs/cli/src/mcp_cmd.rs` (既存ファイルを拡張)

**追加コマンド**:

```rust
// 既存のmcp_cmd.rsに追加
pub async fn handle_dynamic_load(
    cmd: McpDynamicCommand,
    loader: Arc<DynamicMcpLoader>,
) -> Result<()> {
    match cmd {
        McpDynamicCommand::Add { config_path } => {
            // 設定ファイルから読み込んで追加
        }
        McpDynamicCommand::Remove { name } => {
            // サーバーを削除
        }
        McpDynamicCommand::Reload { name } => {
            // サーバーをリロード
        }
        McpDynamicCommand::List => {
            // サーバー一覧を表示
        }
        McpDynamicCommand::Watch => {
            // ファイル監視を開始
        }
    }
}
```

## 実装手順

### Phase 1: コア動的ロード機能

1. **`DynamicMcpLoader`の実装**

   - `McpConnectionManager`への参照を保持
   - `add_server`、`remove_server`、`reload_server`メソッドを実装
   - サーバー状態管理（`HashMap<String, ServerState>`）

2. **`McpConnectionManager`の拡張**

   - 既存の`initialize`メソッドを拡張
   - `add_server_dynamic`、`remove_server_dynamic`メソッドを追加
   - サーバー接続のライフサイクル管理を改善

### Phase 2: ファイル監視機能

1. **`McpFileWatcher`の実装**

   - `notify`クレートを使用（またはポーリング方式）
   - 設定ファイル変更の検出
   - 変更時の自動リロード

2. **設定ファイルの統合**

   - `.codex/mcp-servers.yaml`の変更監視
   - プラグインディレクトリ（`.codex/mcp-plugins/`）の監視

### Phase 3: プラグインシステム

1. **プラグイン構造の定義**

   - `plugin.toml`スキーマ定義
   - プラグインメタデータ構造体

2. **`McpPluginLoader`の実装**

   - プラグインディレクトリのスキャン
   - プラグインの自動ロード
   - 有効化・無効化機能

### Phase 4: APIエンドポイント

1. **`McpApiServer`の実装**

   - HTTPサーバー（`axum`または`warp`を使用）
   - REST APIエンドポイント
   - エラーハンドリング

2. **認証・認可**（オプション）

   - APIキー認証
   - ローカルホストのみ許可

### Phase 5: CLIコマンド拡張

1. **`mcp_cmd.rs`の拡張**

   - `McpDynamicCommand` enumの追加
   - 各コマンドのハンドラー実装

2. **コマンドライン引数の追加**

   - `codex mcp add --config <path>`
   - `codex mcp remove --name <name>`
   - `codex mcp reload --name <name>`
   - `codex mcp list`
   - `codex mcp watch`

## データ構造

### プラグインメタデータ (`plugin.toml`)

```toml
[plugin]
name = "github-plugin"
version = "1.0.0"
description = "GitHub integration MCP server"
author = "zapabob"
enabled = true

[server]
# MCPサーバー設定（既存のMcpServerConfig形式）
command = "node"
args = ["dist/github-mcp-server.js"]
env = { GITHUB_TOKEN = "${GITHUB_TOKEN}" }
```

### サーバー状態管理

```rust
struct ServerState {
    name: String,
    config: McpServerConfig,
    connection: Option<Arc<ManagedClient>>,
    status: ServerStatus,
    last_updated: SystemTime,
}

enum ServerStatus {
    Initializing,
    Running,
    Stopped,
    Error(String),
}
```

## 統合ポイント

### `Config`構造体の拡張

**ファイル**: `codex-rs/core/src/config/types.rs`

```rust
pub struct Config {
    // ... 既存フィールド
    pub mcp_dynamic_loading: McpDynamicLoadingConfig,
}

pub struct McpDynamicLoadingConfig {
    pub enabled: bool,
    pub plugin_dir: Option<PathBuf>,
    pub watch_config_file: bool,
    pub api_server_port: Option<u16>,
}
```

### `Codex`構造体への統合

**ファイル**: `codex-rs/core/src/codex.rs`

- `DynamicMcpLoader`のインスタンスを保持
- 初期化時に動的ロード機能を有効化（設定により）

## エラーハンドリング

- サーバー追加時の競合検出（同名サーバー）
- サーバー削除時の接続切断処理
- リロード時の状態遷移管理
- ファイル監視エラーの処理

## テスト戦略

1. **ユニットテスト**

   - `DynamicMcpLoader`の各メソッド
   - プラグインローダーのテスト

2. **統合テスト**

   - ファイル監視の動作確認
   - APIエンドポイントのテスト
   - CLIコマンドのテスト

3. **E2Eテスト**

   - 実行中のサーバーへの動的追加・削除
   - ホットリロードの動作確認

## セキュリティ考慮事項

- プラグインの検証（署名、チェックサム）
- APIエンドポイントの認証
- サンドボックス環境でのプラグイン実行
- 設定ファイルのパーミッション確認

## パフォーマンス考慮事項

- ファイル監視の効率化（debounce）
- サーバー追加・削除時の非同期処理
- プラグインディレクトリのスキャン最適化

## 依存関係の追加

- `notify` - ファイルシステム監視（既存を確認）
- `axum` または `warp` - HTTPサーバー（既存を確認）
- `serde_yaml` - YAML設定ファイル（既存）

## トークン節約機能

### 6. トークン最適化マネージャー (`McpTokenOptimizer`)

**ファイル**: `codex-rs/core/src/mcp_token_optimizer.rs` (新規作成)

**責務**:

- ツール使用頻度の追跡
- 使用頻度の低いツールの自動アンロード
- ツール説明の圧縮・要約
- トークン使用量の監視とレポート

**主要メソッド**:

```rust
pub struct McpTokenOptimizer {
    tool_usage_stats: Arc<Mutex<HashMap<String, ToolUsageStats>>>,
    token_tracker: Arc<TokenBudgetTracker>,
    auto_unload_threshold: Duration,
    min_usage_count: u64,
}

impl McpTokenOptimizer {
    pub async fn track_tool_usage(&self, tool_name: &str);
    pub async fn get_unused_tools(&self, threshold: Duration) -> Vec<String>;
    pub async fn auto_unload_unused(&self, loader: &DynamicMcpLoader) -> Result<()>;
    pub fn compress_tool_description(&self, tool: &Tool) -> String;
    pub fn estimate_tokens(&self, tools: &[Tool]) -> u64;
    pub fn select_relevant_tools(&self, task: &str, available_tools: &[Tool]) -> Vec<Tool>;
}
```

### 7. 選択的ツールロード (`SelectiveToolLoader`)

**ファイル**: `codex-rs/core/src/mcp_selective_loader.rs` (新規作成)

**責務**:

- タスク分析に基づくツール選択
- 必要なツールのみを動的にロード
- タスク完了後の自動アンロード

**実装**:

```rust
pub struct SelectiveToolLoader {
    optimizer: Arc<McpTokenOptimizer>,
    dynamic_loader: Arc<DynamicMcpLoader>,
    task_tool_mapping: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl SelectiveToolLoader {
    pub async fn load_for_task(&self, task_description: &str) -> Result<Vec<String>>;
    pub async fn unload_after_task(&self, task_id: &str) -> Result<()>;
    pub async fn analyze_task_requirements(&self, task: &str) -> Vec<String>;
}
```

### 8. プロンプト統合 (`McpPromptBuilder`)

**ファイル**: `codex-rs/core/src/mcp_prompt_builder.rs` (新規作成)

**責務**:

- プロンプト構築時に必要なツールのみを追加
- ツール説明の圧縮
- トークン予算に基づくツール選択

**実装**:

```rust
pub struct McpPromptBuilder {
    optimizer: Arc<McpTokenOptimizer>,
    token_budget: Option<u64>,
    max_tools_per_prompt: usize,
}

impl McpPromptBuilder {
    pub fn build_prompt_with_tools(
        &self,
        base_prompt: &str,
        available_tools: &[Tool],
        task_context: &str,
    ) -> Result<Prompt>;
    pub fn select_tools_for_prompt(&self, tools: &[Tool], context: &str) -> Vec<Tool>;
    pub fn compress_tool_descriptions(&self, tools: &[Tool]) -> Vec<String>;
}
```

## トークン節約戦略

### 1. 使用頻度ベースの自動アンロード

- ツールの使用頻度を追跡
- 一定期間（デフォルト: 1時間）使用されていないツールを自動アンロード
- 設定可能な閾値（`auto_unload_threshold`）

### 2. タスクベースの選択的ロード

- タスクの内容を分析して必要なツールを特定
- 必要なツールのみを動的にロード
- タスク完了後に自動アンロード

### 3. ツール説明の圧縮

- 長いツール説明を要約
- 必須情報のみを保持（名前、主要パラメータ、簡潔な説明）
- トークン数を50-70%削減

### 4. プロンプトへの選択的追加

- プロンプト構築時にタスクに関連するツールのみを追加
- トークン予算に基づくツール数の制限
- 優先度に基づくツール選択

### 5. トークン使用量の監視

- 各ツールの説明が消費するトークン数を追跡
- プロンプト全体のトークン数を監視
- レポート生成（どのツールがどのくらいトークンを消費しているか）

## 実装手順（トークン節約機能）

### Phase 6: トークン最適化

1. **`McpTokenOptimizer`の実装**

   - ツール使用頻度の追跡
   - 自動アンロードロジック
   - ツール説明の圧縮

2. **`SelectiveToolLoader`の実装**

   - タスク分析機能
   - 選択的ロード機能
   - タスク完了後の自動アンロード

### Phase 7: プロンプト統合

1. **`McpPromptBuilder`の実装**

   - プロンプト構築時のツール選択
   - トークン予算に基づく制限
   - ツール説明の圧縮

2. **既存プロンプト構築の統合**

   - `client_common.rs`の`Prompt`構造体への統合
   - `agents/runtime.rs`のプロンプト構築への統合

## データ構造（トークン節約）

### ツール使用統計

```rust
struct ToolUsageStats {
    tool_name: String,
    server_name: String,
    usage_count: u64,
    last_used: SystemTime,
    total_tokens_consumed: u64,
    average_tokens_per_call: f64,
}

struct ToolTokenEstimate {
    tool_name: String,
    description_tokens: u64,
    schema_tokens: u64,
    total_tokens: u64,
}
```

### 設定構造

```rust
pub struct McpTokenOptimizationConfig {
    pub enabled: bool,
    pub auto_unload_enabled: bool,
    pub auto_unload_threshold: Duration,
    pub min_usage_count: u64,
    pub compress_descriptions: bool,
    pub max_tools_per_prompt: usize,
    pub token_budget_per_prompt: Option<u64>,
}
```

## 統合ポイント（トークン節約）

### `Config`構造体の拡張

```rust
pub struct Config {
    // ... 既存フィールド
    pub mcp_dynamic_loading: McpDynamicLoadingConfig,
    pub mcp_token_optimization: McpTokenOptimizationConfig,
}
```

### `Prompt`構造体の拡張

**ファイル**: `codex-rs/core/src/client_common.rs`

- ツール選択ロジックの統合
- トークン予算チェックの追加

### `Codex`構造体への統合

**ファイル**: `codex-rs/core/src/codex.rs`

- `McpTokenOptimizer`のインスタンスを保持
- `SelectiveToolLoader`のインスタンスを保持
- プロンプト構築時の統合

## パフォーマンス考慮事項（トークン節約）

- ツール使用頻度の追跡は軽量な操作
- タスク分析はLLMを使用する場合、非同期で実行
- ツール説明の圧縮はキャッシュ可能
- プロンプト構築時のツール選択は高速（インメモリ操作）

## ドキュメント

- プラグイン開発ガイド
- API仕様書
- CLIコマンドリファレンス
- トークン節約ガイド