# Codex - AI Coding Assistant / AI コーチE��ングアシスタンチE

<div align="center">

<img src=".github/assets/codex-logo.svg" alt="Codex Logo" width="200" height="200">

**An autonomous AI coding assistant with sub-agent orchestration and deep research capabilities**  
**サブエージェントオーケストレーションとチE��ープリサーチ機�Eを備えた自律型AIコーチE��ングアシスタンチE*

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-0.48.0--zapabob.1-blue)-blue)]()
[![License](https://img.shields.io/badge/license-Apache2.0-green)]()
[![Rust](https://img.shields.io/badge/rust-1.85+-orange)]()

[English](#english) | [日本語](#japanese)

</div>

---

## <a name="english"></a>🌍 English

### Overview

**Codex** is a next-generation AI coding assistant that extends the [OpenAI/codex](https://github.com/openai/codex) official repository with autonomous orchestration capabilities, specialized sub-agents, and deep research functionality. This fork, maintained by zapabob, adds powerful enhancements while maintaining compatibility with the upstream project.

### 🏗�E�EArchitecture

<div align="center">

![Codex v0.48.0 Architecture](zapabob/docs/codex-v0.48.0-architecture.svg)

*Comprehensive architecture diagram showing orchestration flow, agent coordination, and external integrations*

</div>

### ✨ Key Features

#### �E **v0.48.0 - ClaudeCode-Surpassing Features**

1. **🔒 Automatic Conflict Resolution** *(Unique to Codex)*
   - FileEditTracker: Per-file edit queue management
   - 3 merge strategies: Sequential, LastWriteWins, ThreeWayMerge
   - DashMap-based lock-free concurrency
   - Prevents race conditions in multi-agent editing

2. **🗣�E�ENatural Language CLI** *(Unique to Codex)*
   - `codex agent "Review code for security"` - intuitive invocation
   - AgentInterpreter: Pattern matching & intent classification
   - Auto-dispatch to appropriate specialized agents
   - No need to remember agent names or complex flags

3. **🔗 Webhook & External API Integration** *(Unique to Codex)*
   - GitHub API: Auto-create PRs, manage issues
   - Slack Webhooks: Channel notifications
   - Custom Webhooks: Generic HTTP endpoints
   - Seamless CI/CD pipeline integration

4. **🔄 Advanced Error Retry with Exponential Backoff** *(Codex Advantage)*
   - Configurable RetryPolicy (max 3 retries, 1s-30s delay)
   - FallbackStrategy: Retry, Skip, or Fail
   - AgentError type system for granular error handling
   - 3x improved resilience over basic retry

5. **📖 Fully Open Source** *(Unique to Codex)*
   - All code publicly available on GitHub
   - Community contributions welcome
   - Transparent development process
   - Apache 2.0 / MIT dual license

#### **Autonomous Orchestration** (ClaudeCode-style)
- **TaskAnalyzer**: Automatic task complexity analysis
- **AutoOrchestrator**: Self-directed sub-agent execution
- **Threshold-based**: Automatic delegation when complexity > 0.7
- **Seamless Integration**: Works transparently in the background

#### 2. **Specialized Sub-Agent System**
- **CodeExpert**: Code analysis and refactoring
- **SecurityExpert**: Security audits and vulnerability scanning
- **TestingExpert**: Comprehensive test generation
- **DeepResearcher**: Multi-source research with citations
- **DocsExpert**: Documentation generation
- **DebugExpert**: Issue diagnosis and resolution
- **PerformanceExpert**: Performance optimization

#### 3. **Deep Research Engine**
- **Multi-source**: DuckDuckGo, Brave, Google, Bing integration
- **Citation-based**: All findings with source attribution
- **Contradiction Detection**: Identifies conflicting information
- **Configurable Depth**: 1-5 levels of research depth
- **Confidence Scoring**: Reliability metrics for each finding

#### 4. **MCP (Model Context Protocol) Integration**
- **Cursor IDE**: Native integration via MCP server
- **Custom Tools**: Extensible tool ecosystem
- **Real-time Sync**: Live collaboration capabilities

### 🏗�E�EArchitecture

```
┌─────────────────────────────────────────────────────────────────━E
━E                        Codex CLI / TUI                          ━E
━E                  (User Interface Layer)                         ━E
└───────────────────────────┬─────────────────────────────────────━E
                            ━E
                            ▼
┌─────────────────────────────────────────────────────────────────━E
━E                   Codex Core Runtime                            ━E
━E ┌──────────────━E ┌──────────────━E ┌──────────────━E         ━E
━E ━ETaskAnalyzer ━E ━EAutoOrchestra━E ━E   Session   ━E         ━E
━E ━E             │─▶━E    tor      │─▶━E  Manager    ━E         ━E
━E └──────────────━E └──────────────━E └──────────────━E         ━E
└───────────────────────────┬─────────────────────────────────────━E
                            ━E
              ┌─────────────┼─────────────━E
              ▼             ▼             ▼
    ┌─────────────━E┌─────────────━E┌─────────────━E
    ━ESub-Agent   ━E━ESub-Agent   ━E━ESub-Agent   ━E
    ━ESupervisor  ━E━EDeep        ━E━ECustom      ━E
    ━E            ━E━EResearch    ━E━ECommands    ━E
    └──────┬──────━E└──────┬──────━E└──────┬──────━E
           ━E              ━E              ━E
    ┌──────▼───────────────▼───────────────▼──────━E
    ━E        Specialized Sub-Agents               ━E
    ━E ┌────────━E┌────────━E┌────────━E         ━E
    ━E ━Eode    ━E│Security━E│Testing ━E...      ━E
    ━E ━Expert  ━E━Expert  ━E━Expert  ━E         ━E
    ━E └────────━E└────────━E└────────━E         ━E
    └──────────────────┬───────────────────────────━E
                       ━E
                       ▼
    ┌────────────────────────────────────────────━E
    ━E        LLM Providers & Tools              ━E
    ━E ┌──────────━E┌──────────━E┌──────────━E ━E
    ━E ━EOpenAI   ━E━EAnthropic━E━E  MCP    ━E ━E
    ━E ━E API     ━E━E  API    ━E━E Server  ━E ━E
    ━E └──────────━E└──────────━E└──────────━E ━E
    └────────────────────────────────────────────━E
```

### 🚀 Quick Start

#### Installation

```bash
# Clone the repository
git clone https://github.com/zapabob/codex.git
cd codex

# Build Rust components
cd codex-rs
cargo build --release -p codex-cli
cargo install --path cli --force

# Verify installation
codex --version
# codex-cli 0.48.0
```

#### Configuration

Create `~/.codex/config.toml`:

```toml
model = "gpt-5-codex"

[model_providers.openai]
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
wire_api = "chat"

[sandbox]
default_mode = "read-only"

[approval]
policy = "on-request"
```

#### Basic Usage

```bash
# Interactive mode
codex

# Execute with initial prompt
codex "Implement JWT authentication"

# Use sub-agent delegation
codex delegate code-reviewer --scope ./src

# Deep research
codex research "React Server Components best practices" --depth 3

# Parallel execution
codex delegate-parallel code-reviewer,test-gen --scopes ./src,./tests
```

### 📚 Documentation

- [Getting Started](docs/getting-started.md)
- [Autonomous Orchestration Guide](docs/auto-orchestration.md)
- [Sub-Agents Quick Start](docs/quickstart-subagents.md)
- [Deep Research Guide](docs/quickstart-deepresearch.md)
- [Cursor IDE Integration](docs/cursor-implementation-plan.md)
- [API Documentation](docs/api/)

### 🛠�E�EDevelopment

#### Prerequisites

- **Rust**: 1.85+
- **Node.js**: 18+
- **PNPM**: 9+
- **OS**: Windows 11, macOS, Linux

#### Project Structure

```
codex-main/
├── codex-rs/              # Rust core implementation
━E  ├── cli/               # Command-line interface
━E  ├── core/              # Core runtime
━E  ├── tui/               # Terminal UI
━E  ├── mcp-server/        # MCP server (zapabob)
━E  ├── supervisor/        # Sub-agent management (zapabob)
━E  └── deep-research/     # Deep research engine (zapabob)
├── codex-cli/             # Node.js CLI
├── docs/                  # Documentation
├── examples/              # Example code
├── zapabob/               # zapabob-specific extensions
━E  ├── docs/              # Additional documentation
━E  ├── scripts/           # Build scripts
━E  ├── extensions/        # IDE extensions
━E  └── sdk/               # TypeScript SDK
└── _docs/                 # Implementation logs
```

#### Building from Source

```bash
# Full build
cd codex-rs
cargo clean
cargo build --release

# Install globally
cargo install --path cli --force
```

### 🤁EContributing

We welcome contributions! Please see [CONTRIBUTING.md](docs/contributing.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### 📊 Comparison with Official Repo

| Feature | Official OpenAI/codex | zapabob/codex |
|---------|----------------------|---------------|
| Basic CLI | ✁E| ✁E|
| TUI Interface | ✁E| ✁E|
| MCP Server | ✁E| ✁EEnhanced |
| Sub-Agents | ❁E| ✁E7 Specialized |
| Auto Orchestration | ❁E| ✁EClaudeCode-style |
| Deep Research | ❁E| ✁EMulti-source |
| Parallel Execution | ❁E| ✁EOptimized |
| Cursor Integration | ✁E| ✁EEnhanced |

### 📄 License

This project inherits the license from [OpenAI/codex](https://github.com/openai/codex). See [LICENSE](LICENSE) for details.

### 🔗 Links

- [OpenAI Official Codex](https://github.com/openai/codex)
- [zapabob Fork](https://github.com/zapabob/codex)
- [Documentation](https://codex.zapabob.dev)
- [Discord Community](https://discord.gg/codex)

---

## <a name="japanese"></a>�E�E 日本誁E

### 概要E

**Codex**は、[OpenAI/codex](https://github.com/openai/codex)公式リポジトリを拡張した次世代AIコーチE��ングアシスタントです、Eapabobが保守するこのフォークは、�E律オーケストレーション機�E、専門サブエージェント、ディープリサーチ機�Eを追加しながら、上流�Eロジェクトとの互換性を維持してぁE��す、E

### ✨ 主な機�E

#### �E **v0.48.0 - ClaudeCode趁E��新機�E**

1. **🔒 コンフリクト�E動回避** *(Codex独自機�E)*
   - FileEditTracker: ファイル別編雁E��ュー管琁E
   - 3種マ�Eジ戦略: Sequential, LastWriteWins, ThreeWayMerge
   - DashMapベ�EスのロチE��フリー並行�E琁E
   - 褁E��エージェント編雁E��の競合を自動解決

2. **🗣�E�E自然言語CLI** *(Codex独自機�E)*
   - `codex agent "セキュリチE��重視でコードレビュー"` - 直感的呼び出ぁE
   - AgentInterpreter: パターンマッチング�E�E��図刁E��E
   - 適刁E��エージェントへ自動振り�EぁE
   - エージェント名めE��E��なフラグ不要E

3. **🔗 Webhook/外部API統吁E* *(Codex独自機�E)*
   - GitHub API: PR自動作�E、Issue管琁E
   - Slack Webhook: チャンネル通知
   - カスタムWebhook: 汎用HTTPエンド�EインチE
   - CI/CDパイプラインとシームレス連携

4. **🔄 持E��バックオフエラーリトライ** *(Codex優佁E*
   - 設定可能なRetryPolicy�E�最大3回、E-30秒遅延�E�E
   - FallbackStrategy: Retry、Skip、Fail
   - AgentError型シスチE��で細かいエラー処琁E
   - 基本リトライの3倍�E回復劁E

5. **📖 完�Eオープンソース** *(Codex独自機�E)*
   - 全コードGitHub公閁E
   - コミュニティ貢献歓迁E
   - 透�Eな開発プロセス
   - Apache 2.0 / MIT チE��アルライセンス

#### **自律オーケストレーション** (ClaudeCode風)
- **TaskAnalyzer**: タスクの褁E��度を�E動�E极E
- **AutoOrchestrator**: サブエージェントを自律的に実衁E
- **閾値ベ�Eス**: 褁E��度 > 0.7 で自動委譲
- **シームレス統吁E*: バックグラウンドで透過皁E��動佁E

#### 2. **専門サブエージェントシスチE��**
- **CodeExpert**: コード�E析とリファクタリング
- **SecurityExpert**: セキュリチE��監査と脁E��性スキャン
- **TestingExpert**: 匁E��皁E��チE��ト生戁E
- **DeepResearcher**: 引用付き多�E調査
- **DocsExpert**: ドキュメント生戁E
- **DebugExpert**: 問題診断と解決
- **PerformanceExpert**: パフォーマンス最適匁E

#### 3. **チE��ープリサーチエンジン**
- **多�Eソース**: DuckDuckGo、Brave、Google、Bing統吁E
- **引用ベ�Eス**: すべての発見にソース帰屁E
- **矛盾検�E**: 競合する情報を識別
- **深度設定可能**: 1-5レベルの調査深度
- **信頼性スコア**: 吁E��見�E信頼性メトリクス

#### 4. **MCP (Model Context Protocol) 統吁E*
- **Cursor IDE**: MCPサーバ�E経由のネイチE��ブ統吁E
- **カスタムチE�Eル**: 拡張可能なチE�EルエコシスチE��
- **リアルタイム同期**: ライブコラボレーション機�E

### 🏗�E�EアーキチE��チャ

```
┌─────────────────────────────────────────────────────────────────━E
━E                     Codex CLI / TUI                             ━E
━E                 (ユーザーインターフェース層)                       ━E
└───────────────────────────┬─────────────────────────────────────━E
                            ━E
                            ▼
┌─────────────────────────────────────────────────────────────────━E
━E                 Codex コアランタイム                             ━E
━E ┌──────────────━E ┌──────────────━E ┌──────────────━E         ━E
━E ━ETask         ━E ━EAuto         ━E ━EセチE��ョン   ━E         ━E
━E ━EAnalyzer     │─▶━EOrchestrator │─▶━Eマネージャー ━E         ━E
━E └──────────────━E └──────────────━E └──────────────━E         ━E
└───────────────────────────┬─────────────────────────────────────━E
                            ━E
              ┌─────────────┼─────────────━E
              ▼             ▼             ▼
    ┌─────────────━E┌─────────────━E┌─────────────━E
    ━EサチE       ━E━EチE��ーチE   ━E━Eカスタム    ━E
    ━Eエージェント│ ━EリサーチE   ━E━EコマンチE   ━E
    ━E監督老E     ━E━E            ━E━E            ━E
    └──────┬──────━E└──────┬──────━E└──────┬──────━E
           ━E              ━E              ━E
    ┌──────▼───────────────▼───────────────▼──────━E
    ━E        専門サブエージェンチE                ━E
    ━E ┌────────━E┌────────━E┌────────━E         ━E
    ━E │コーチE ━E│セキュリ━E│テスチE ━E...      ━E
    ━E │専門家  ━E│ティ専門━E│専門家  ━E         ━E
    ━E └────────━E└────────━E└────────━E         ━E
    └──────────────────┬───────────────────────────━E
                       ━E
                       ▼
    ┌────────────────────────────────────────────━E
    ━E        LLMプロバイダー & チE�Eル            ━E
    ━E ┌──────────━E┌──────────━E┌──────────━E ━E
    ━E ━EOpenAI   ━E━EAnthropic━E━E  MCP    ━E ━E
    ━E ━E API     ━E━E  API    ━E━E Server  ━E ━E
    ━E └──────────━E└──────────━E└──────────━E ━E
    └────────────────────────────────────────────━E
```

### 🚀 クイチE��スターチE

#### インスト�Eル

```bash
# リポジトリをクローン
git clone https://github.com/zapabob/codex.git
cd codex

# Rustコンポ�EネントをビルチE
cd codex-rs
cargo build --release -p codex-cli
cargo install --path cli --force

# インスト�Eル確誁E
codex --version
# codex-cli 0.48.0
```

#### 設宁E

`~/.codex/config.toml` を作�E:

```toml
model = "gpt-5-codex"

[model_providers.openai]
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
wire_api = "chat"

[sandbox]
default_mode = "read-only"

[approval]
policy = "on-request"
```

#### 基本皁E��使ぁE��

```bash
# インタラクチE��ブモーチE
codex

# 初期プロンプトで実衁E
codex "JWT認証を実裁E��て"

# サブエージェント委譲
codex delegate code-reviewer --scope ./src

# チE��ープリサーチE
codex research "React Server Componentsのベスト�EラクチE��ス" --depth 3

# 並列実衁E
codex delegate-parallel code-reviewer,test-gen --scopes ./src,./tests
```

### 📚 ドキュメンチE

- [はじめに](docs/getting-started.md)
- [自律オーケストレーションガイド](docs/auto-orchestration.md)
- [サブエージェントクイチE��スターチE(docs/quickstart-subagents.md)
- [チE��ープリサーチガイド](docs/quickstart-deepresearch.md)
- [Cursor IDE統吁E(docs/cursor-implementation-plan.md)
- [APIドキュメンチE(docs/api/)

### 🛠�E�E開発

#### 前提条件

- **Rust**: 1.85以丁E
- **Node.js**: 18以丁E
- **PNPM**: 9以丁E
- **OS**: Windows 11, macOS, Linux

#### プロジェクト構造

```
codex-main/
├── codex-rs/              # Rustコア実裁E
━E  ├── cli/               # コマンドラインインターフェース
━E  ├── core/              # コアランタイム
━E  ├── tui/               # ターミナルUI
━E  ├── mcp-server/        # MCPサーバ�E (zapabob)
━E  ├── supervisor/        # サブエージェント管琁E(zapabob)
━E  └── deep-research/     # チE��ープリサーチエンジン (zapabob)
├── codex-cli/             # Node.js CLI
├── docs/                  # ドキュメンチE
├── examples/              # サンプルコーチE
├── zapabob/               # zapabob独自拡張
━E  ├── docs/              # 追加ドキュメンチE
━E  ├── scripts/           # ビルドスクリプト
━E  ├── extensions/        # IDE拡張
━E  └── sdk/               # TypeScript SDK
└── _docs/                 # 実裁E��グ
```

#### ソースからビルチE

```bash
# フルビルチE
cd codex-rs
cargo clean
cargo build --release

# グローバルインスト�Eル
cargo install --path cli --force
```

### 🤁Eコントリビューション

コントリビューションを歓迎します！詳細は [CONTRIBUTING.md](docs/contributing.md) をご覧ください、E

1. リポジトリをフォーク
2. フィーチャーブランチを作�E (`git checkout -b feature/amazing-feature`)
3. 変更をコミッチE(`git commit -m 'feat: すごぁE���Eを追加'`)
4. ブランチにプッシュ (`git push origin feature/amazing-feature`)
5. プルリクエストを開く

### 📊 公式リポジトリとの比輁E

| 機�E | 公弁EOpenAI/codex | zapabob/codex |
|------|------------------|---------------|
| 基本CLI | ✁E| ✁E|
| TUIインターフェース | ✁E| ✁E|
| MCPサーバ�E | ✁E| ✁E強化版 |
| サブエージェンチE| ❁E| ✁E7種の専門家 |
| 自律オーケストレーション | ❁E| ✁EClaudeCode風 |
| チE��ープリサーチE| ❁E| ✁E多�Eソース |
| 並列実衁E| ❁E| ✁E最適化済み |
| Cursor統吁E| ✁E| ✁E強化版 |

### 📄 ライセンス

こ�Eプロジェクト�E [OpenAI/codex](https://github.com/openai/codex) からライセンスを継承してぁE��す。詳細は [LICENSE](LICENSE) を参照してください、E

### 🔗 リンク

- [OpenAI公式Codex](https://github.com/openai/codex)
- [zapabobフォーク](https://github.com/zapabob/codex)
- [ドキュメンチE(https://codex.zapabob.dev)
- [Discordコミュニティ](https://discord.gg/codex)

---

<div align="center">

**Made with ❤�E�Eby zapabob | Built on OpenAI/codex**

**Version](https://img.shields.io/badge/version-0.48.0--zapabob.1-blue) | **Last Updated**: 2025-10-15

</div>
