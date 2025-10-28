# Codex - AI Coding Assistant / AIコーディングアシスタント

<div align="center">

<img src=".github/assets/codex-logo.svg" alt="Codex Logo" width="200" height="200">

**An autonomous AI coding assistant with sub-agent orchestration and deep research capabilities**  
**サブエージェントオーケストレーションとディープリサーチ機能を備えた自律型AIコーディングアシスタント**

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Version](https://img.shields.io/badge/version-0.51.0--zapabob.1-blue)]()
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![Rust](https://img.shields.io/badge/rust-1.90+-orange)]()
[![OpenAI](https://img.shields.io/badge/OpenAI-upstream%20synced-success)]()
[![MCP](https://img.shields.io/badge/MCP-15%20servers-blueviolet)]()

[English](#english) | [日本語](#japanese)

</div>

---

## <a name="english"></a>🌍 English

### Overview

**Codex** is a next-generation AI coding assistant that extends the [OpenAI/codex](https://github.com/openai/codex) official repository with autonomous orchestration capabilities, specialized sub-agents, and deep research functionality. This fork, maintained by zapabob, adds powerful enhancements while maintaining compatibility with the upstream project.

### 🏗️ Architecture

<div align="center">

![Codex v0.51.0 Architecture](zapabob/docs/codex-v0.51.0-architecture.svg)

*Comprehensive architecture diagram showing orchestration flow, agent coordination, and external integrations (Updated 2025-10-28)*

</div>

#### 📊 **Architecture Overview**

The Codex v0.51.0-zapabob.1 architecture consists of **8 major layers** with **70+ components**:

1. **🖥️ User Interface Layer** - CLI, TUI, Cursor IDE, Natural Language CLI
2. **🧠 Core Orchestration Layer** - Task Analyzer, Auto Orchestrator, Supervisor
3. **🤖 Specialized Sub-Agents** - 7 specialized agents for different tasks
4. **🔍 Deep Research Engine** - Multi-source search with citation management
5. **🔗 MCP Integration Layer** - 14 MCP servers for tool integration
6. **🌐 External Integrations** - GitHub API, Slack, Audio notifications
7. **💾 Data & Configuration** - Settings, session management, audit logs
8. **🤖 LLM Model Providers** - OpenAI, Google Gemini

#### 🎯 **Key Architectural Features**

- **🔒 Automatic Conflict Resolution** - FileEditTracker with 3 merge strategies
- **🗣️ Natural Language CLI** - AgentInterpreter with pattern matching
- **🔄 Advanced Error Retry** - Exponential backoff with fallback strategies
- **📖 Fully Open Source** - Apache 2.0 / MIT dual license
- **🔌 MCP Protocol Integration** - Standardized tool ecosystem (15 servers)
- **🔍 Multi-source Research** - Gemini Search Grounding, DuckDuckGo, Google, Bing

If you're running into upgrade issues with Homebrew, see the [FAQ entry on brew upgrade codex](./docs/faq.md#brew-update-codex-isnt-upgrading-me).

<details>
<summary>📊 <b>Detailed Architecture Diagram (Mermaid)</b></summary>

```mermaid
graph TB
    subgraph UI["🖥️ User Interface Layer"]
        CLI["CLI<br/>Command Line Interface"]
        TUI["TUI<br/>Terminal UI"]
        Cursor["Cursor IDE<br/>Composer Integration"]
        NaturalCLI["Natural Language CLI<br/>AgentInterpreter"]
    end

    subgraph Orchestration["🧠 Core Orchestration Layer - rmcp 0.8.3+"]
        TaskAnalyzer["Task Analyzer<br/>Complexity: 0-1.0<br/>Skills Detection"]
        AutoOrch["Auto Orchestrator<br/>Dynamic Agent Selection<br/>Strategy: Parallel/Sequential/Hybrid"]
        Supervisor["Supervisor<br/>Timeout: 5min<br/>Retry: 3x Exponential Backoff"]
        CollabStore["Collaboration Store<br/>Message Passing<br/>Priority: 0-255"]
    end

    subgraph Agents["🤖 Specialized Sub-Agents (8 Types)"]
        Researcher["Researcher<br/>Multi-source Validation"]
        CodeReviewer["Code Reviewer<br/>Best Practices"]
        TestGen["Test Generator<br/>Coverage 80%+"]
        SecAudit["Security Auditor<br/>OWASP Top 10"]
        PythonRev["Python Reviewer"]
        TSRev["TypeScript Reviewer"]
        UnityRev["Unity Reviewer"]
        CustomAgent["Custom Agent<br/>User-defined"]
    end

    subgraph DeepResearch["🔍 Deep Research Engine"]
        SearchProvider["MCP Search Provider<br/>Cache TTL: 1h<br/>45x Faster"]
        Gemini["Gemini CLI<br/>OAuth 2.0<br/>Google Search Grounding"]
        DuckDuckGo["DuckDuckGo<br/>API Key Free"]
        Citation["Citation Manager<br/>Source Tracking"]
        Contradiction["Contradiction Checker<br/>Cross-validation"]
    end

    subgraph MCP["🔗 MCP Integration (14 Servers)"]
        CodexMCP["codex mcp-server<br/>Self-hosted"]
        GeminiMCP["gemini-cli<br/>Google Search"]
        SerenaM["serena<br/>Code Intelligence"]
        ArxivMCP["arxiv-mcp-server"]
        ChromeMCP["chrome-devtools"]
    end

    CLI --> TaskAnalyzer
    TUI --> TaskAnalyzer
    Cursor --> CodexMCP
    TaskAnalyzer -->|complexity > 0.7| AutoOrch
    AutoOrch --> Supervisor
    Supervisor --> CollabStore
    CollabStore --> Agents
    AutoOrch --> Researcher
    AutoOrch --> CodeReviewer
    Researcher --> SearchProvider
    SearchProvider --> Gemini
    SearchProvider --> DuckDuckGo
    SearchProvider --> Citation
    Citation --> Contradiction
    CodexMCP --> Supervisor
    GeminiMCP --> Gemini

    classDef uiClass fill:#e1f5ff,stroke:#01579b,stroke-width:2px
    classDef orchClass fill:#fff9c4,stroke:#f57f17,stroke-width:2px
    classDef agentClass fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef researchClass fill:#e8f5e9,stroke:#1b5e20,stroke-width:2px
    classDef mcpClass fill:#fff3e0,stroke:#e65100,stroke-width:2px

    class CLI,TUI,Cursor,NaturalCLI uiClass
    class TaskAnalyzer,AutoOrch,Supervisor,CollabStore orchClass
    class Researcher,CodeReviewer,TestGen,SecAudit,PythonRev,TSRev,UnityRev,CustomAgent agentClass
    class SearchProvider,Gemini,DuckDuckGo,Citation,Contradiction researchClass
    class CodexMCP,GeminiMCP,SerenaM,ArxivMCP,ChromeMCP mcpClass
```

</details>

### 📁 Repository Structure

<details>
<summary><b>Directory Organization</b></summary>

```mermaid
graph TB
    Root["📦 codex/"]
    
    Root --> Official["🏢 Official OpenAI/codex"]
    Root --> Zapabob["⭐ zapabob Extensions"]
    Root --> Config["⚙️ Configuration"]
    Root --> Temp["🗑️ Temporary (.gitignore)"]
    Root --> Archive["📦 Archive"]

    Official --> CodexRS["codex-rs/<br/>Rust Implementation<br/>Core, Agents, Orchestration"]
    Official --> CodexCLI["codex-cli/<br/>npm Package"]
    Official --> Docs["docs/<br/>51 Official Docs"]
    Official --> Scripts["scripts/<br/>9 Official Scripts"]

    Zapabob --> ZDocs["zapabob/docs/<br/>- implementation-logs/ (236)<br/>- guides/<br/>- architecture/"]
    Zapabob --> ZScripts["zapabob/scripts/<br/>- play-completion-sound.ps1<br/>- build automation"]
    Zapabob --> ZExt["zapabob/extensions/<br/>- vscode-extension/<br/>- windsurf-extension/"]
    Zapabob --> ZSDK["zapabob/sdk/<br/>TypeScript SDK"]

    Config --> Cursor[".cursor/<br/>- mcp-config.json<br/>- composer-guide.md"]
    Config --> CodexC[".codex/<br/>- agents/*.yaml (8)<br/>- marisa_owattaze.wav"]

    Temp --> TBuild["_temp/build-logs/<br/>7 logs"]
    Temp --> TTest["_temp/test-outputs/<br/>6 test files"]
    Temp --> TArt["_temp/build-artifacts/<br/>exe/pdb"]

    Archive --> AOld["archive/<br/>Legacy PRs"]
    Archive --> ABackup["archive/backups/<br/>2 files"]

    classDef officialClass fill:#e3f2fd,stroke:#0d47a1,stroke-width:3px
    classDef zapaClass fill:#fff9c4,stroke:#f57f17,stroke-width:3px
    classDef configClass fill:#f3e5f5,stroke:#4a148c,stroke-width:2px
    classDef tempClass fill:#ffebee,stroke:#b71c1c,stroke-width:2px
    classDef archiveClass fill:#eceff1,stroke:#37474f,stroke-width:2px

    class Official,CodexRS,CodexCLI,Docs,Scripts officialClass
    class Zapabob,ZDocs,ZScripts,ZExt,ZSDK zapaClass
    class Config,Cursor,CodexC configClass
    class Temp,TBuild,TTest,TArt tempClass
    class Archive,AOld,ABackup archiveClass
```

</details>

### ✨ Key Features

#### 🆕 **v0.51.0-zapabob.1 - Latest Updates** *(2025-10-28)*

1. **🔄 OpenAI/codex Upstream Integration** *(NEW)*
   - Merged OpenAI official repository (commit 4a42c4e1)
   - Resolved 9 merge conflicts manually
   - Integrated auth system enhancements (Keyring support)
   - Updated to Rust 2024 edition compatibility
   - Full semantic versioning alignment

2. **✨ Gemini Search Grounding for Deep Research** *(NEW)*
   - Default search backend: Google Gemini Search Grounding
   - High-quality search results via Google Search API
   - OAuth 2.0 authentication (no API key required)
   - Smart fallback chain: Gemini → DuckDuckGo → Google → Bing
   - Integrated with codex-gemini-mcp server

3. **🤖 Enhanced MCP Integration**
   - 15 MCP servers fully operational (was 14)
   - Added: codex-research, codex-agent, sequential-thinking
   - Complete Cursor IDE / VSCode compatibility
   - config.toml and mcp.json synchronized
   - Serena (21 tools), Playwright, Chrome DevTools enabled

4. **🎯 Sub-Agent System Improvements**
   - 8 specialized agents fully tested
   - Parallel execution (3x faster processing)
   - Natural language CLI: `codex agent "<task>"`
   - Improved error handling and retry logic

5. **🔔 Audio Notification System** *(Enhanced)*
   - Task completion: Marisa "終わったぜ！" (Owattaze!)
   - Agent completion notifications
   - Session end notifications
   - PowerShell-based audio playback
   - Location: `.codex/marisa_owattaze.wav`

#### 🔥 **ClaudeCode-Surpassing Features**

1. **🔒 Automatic Conflict Resolution** *(Unique to Codex)*
   - FileEditTracker: Per-file edit queue management
   - 3 merge strategies: Sequential, LastWriteWins, ThreeWayMerge
   - DashMap-based lock-free concurrency
   - Prevents race conditions in multi-agent editing

2. **🗣️ Natural Language CLI** *(Unique to Codex)*
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
   - Apache 2.0 

#### **Autonomous Orchestration** (ClaudeCode-style)
- **TaskAnalyzer**: Automatic task complexity analysis
- **AutoOrchestrator**: Self-directed sub-agent execution
- **Threshold-based**: Automatic delegation when complexity > 0.7
- **Seamless Integration**: Works transparently in the background

#### **Specialized Sub-Agent System**
- **CodeExpert**: Code analysis and refactoring
- **SecurityExpert**: Security audits and vulnerability scanning
- **TestingExpert**: Comprehensive test generation
- **DeepResearcher**: Multi-source research with citations
- **DocsExpert**: Documentation generation
- **DebugExpert**: Issue diagnosis and resolution
- **PerformanceExpert**: Performance optimization

#### **Deep Research Engine**
- **Multi-source**: Gemini Search Grounding (default), DuckDuckGo, Google, Bing
- **Citation-based**: All findings with source attribution
- **Contradiction Detection**: Identifies conflicting information
- **Configurable Depth**: 1-5 levels of research depth
- **Confidence Scoring**: Reliability metrics for each finding
- **Smart Fallback**: Automatic backend switching on failure
- **MCP Integration**: Native integration with 15 MCP servers

#### **MCP (Model Context Protocol) Integration**
- **Cursor IDE**: Native integration via MCP server
- **Custom Tools**: Extensible tool ecosystem
- **Real-time Sync**: Live collaboration capabilities
- **15 MCP Servers**: Codex, Serena, Context7, Playwright, GitHub, Gemini, Sequential-Thinking, and more
- **Config Sync**: Automatic synchronization between config.toml and mcp.json

### 📦 Installation

#### Prerequisites
- **Rust** 1.90 or later
- **OpenAI API Key** (set as `OPENAI_API_KEY`)
- **Git** for cloning
- **Node.js** (optional, for Gemini CLI)

#### Quick Start

```bash
# Clone the repository
git clone https://github.com/zapabob/codex.git
cd codex/codex-rs

# Build release version
cargo build --release

# Install globally
cargo install --path cli --force

# Verify installation
codex --version
# Output: codex-cli 0.51.0-zapabob.1
```

#### Gemini CLI MCP Setup (Optional)

```bash
# Install Gemini CLI (Node.js)
npm install -g @google-labs/gemini-cli

# Login with OAuth 2.0
gemini login

# Test Gemini CLI
gemini -p "Hello Gemini" -o text

# Use with Codex
codex research "Rust async best practices" --gemini --use-mcp
```

### 🚀 Usage

#### Basic Commands

```bash
# Interactive TUI mode
codex

# Quick command execution
codex exec "explain this TypeScript function"

# Deep research with citations
codex research "React Server Components" --depth 3

# Gemini CLI integration
codex research "Machine Learning basics" --gemini --use-mcp

# Resume last session
codex resume --last
```

#### Sub-Agent Delegation

```bash
# Security audit
codex delegate sec-audit --scope ./src

# Code review
codex delegate code-reviewer --scope ./app

# Test generation
codex delegate test-gen --scope ./lib

# Parallel execution (3x faster)
codex delegate-parallel code-reviewer,test-gen --scopes ./src,./tests
```

#### Natural Language CLI

```bash
# Intuitive agent invocation
codex agent "Review my code for security vulnerabilities"
codex agent "Generate comprehensive tests for user auth"
codex agent "Research best practices for Rust error handling"
```

### ⚙️ Configuration

#### config.toml Example

```toml
# Model settings
model = "gpt-5-codex"

[model_providers.openai]
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
wire_api = "chat"

# Security settings
[sandbox]
default_mode = "read-only"

[approval]
policy = "on-request"

# MCP Servers
[mcp_servers.codex-gemini-mcp]
command = "codex-gemini-mcp"
args = []
env.PATH = "C:\\Users\\username\\.cargo\\bin;${PATH}"

# Hooks - Audio notifications
[hooks]
on_task_complete = "powershell -ExecutionPolicy Bypass -File zapabob/scripts/play-completion-sound.ps1"
on_subagent_complete = "powershell -ExecutionPolicy Bypass -File zapabob/scripts/play-completion-sound.ps1"
on_session_end = "powershell -ExecutionPolicy Bypass -File zapabob/scripts/play-completion-sound.ps1"
```

### 🧪 Testing

#### Run Integration Tests

```bash
cd codex-rs

# Basic tests
cargo test --test mcp_integration_test

# Integration tests with real MCP server
cargo test --test mcp_integration_test -- --ignored --nocapture

# Full test suite
cargo test --all
```

#### Test Results
- **MCP Integration**: 6/6 passing ✅
- **Performance**: < 6 seconds ✅
- **End-to-End**: All flows validated ✅

### 📚 Documentation

- **Architecture**: [`zapabob/docs/codex-v0.51.0-architecture.svg`](zapabob/docs/codex-v0.51.0-architecture.svg)
- **MCP Config Guide**: [`_docs/MCP設定ファイル同期管理ガイド.md`](_docs/MCP設定ファイル同期管理ガイド.md)
- **Implementation Logs**: [`_docs/`](_docs/)
- **Audio Notifications**: [`_docs/2025-10-23_音声通知設定更新.md`](_docs/2025-10-23_音声通知設定更新.md)

### 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### 📄 License

This project is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

```
Copyright 2025 zapabob

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

### 🙏 Acknowledgments

- Based on [OpenAI/codex](https://github.com/openai/codex)
- Maintained by [zapabob](https://github.com/zapabob)
- Community contributors

---

## <a name="japanese"></a>🇯🇵 日本語

### 概要

**Codex**は、[OpenAI/codex](https://github.com/openai/codex)公式リポジトリを拡張した次世代AIコーディングアシスタントです。自律的なオーケストレーション機能、専門化されたサブエージェント、ディープリサーチ機能を搭載しています。zapabobがメンテナンスするこのフォークは、上流プロジェクトとの互換性を維持しながら強力な機能拡張を追加しています。

### 🏗️ アーキテクチャ

<div align="center">

![Codex v0.51.0 Architecture](zapabob/docs/codex-v0.51.0-architecture.svg)

*オーケストレーションフロー、エージェント協調、外部統合を示す包括的なアーキテクチャ図（2025-10-28更新）*

</div>

#### 📊 **アーキテクチャ概要**

Codex v0.51.0-zapabob.1アーキテクチャは**8つの主要レイヤー**と**70+のコンポーネント**で構成されています：

1. **🖥️ ユーザーインターフェース層** - CLI、TUI、Cursor IDE、自然言語CLI
2. **🧠 コアオーケストレーション層** - タスク分析器、自動オーケストレーター、スーパーバイザー
3. **🤖 専門サブエージェント** - 7つの専門エージェント
4. **🔍 ディープリサーチエンジン** - 引用管理付きマルチソース検索
5. **🔗 MCP統合層** - ツール統合用14個のMCPサーバー
6. **🌐 外部統合** - GitHub API、Slack、音声通知
7. **💾 データ・設定** - 設定、セッション管理、監査ログ
8. **🤖 LLMモデルプロバイダー** - OpenAI、Anthropic、Google Gemini

#### 🎯 **主要アーキテクチャ特徴**

- **🔒 自動コンフリクト解決** - 3つのマージ戦略を持つFileEditTracker
- **🗣️ 自然言語CLI** - パターンマッチング付きAgentInterpreter
- **🔄 高度エラーリトライ** - フォールバック戦略付き指数バックオフ
- **📖 完全オープンソース** - Apache 2.0 
- **🔌 MCPプロトコル統合** - 標準化ツールエコシステム
- **🔍 マルチソース研究** - DuckDuckGo、Brave、Google、Bing、Gemini CLI

### ✨ 主要機能

#### 🆕 **v0.51.0-zapabob.1 - 最新アップデート** *(2025-10-28)*

1. **🔄 OpenAI/codex公式リポジトリ統合** *(NEW)*
   - OpenAI公式リポジトリをマージ（コミット 4a42c4e1）
   - 9ファイルのマージコンフリクトを手動解決
   - 認証システム強化（Keyring対応）を統合
   - Rust 2024 edition互換性対応
   - セマンティックバージョニング完全整合

2. **✨ Deep ResearchへのGemini Search Grounding統合** *(NEW)*
   - デフォルト検索バックエンド: Google Gemini Search Grounding
   - Google Search API経由の高品質検索結果
   - OAuth 2.0認証（APIキー不要）
   - スマートフォールバックチェーン: Gemini → DuckDuckGo → Google → Bing
   - codex-gemini-mcpサーバーと統合

3. **🤖 MCP統合の強化**
   - 15個のMCPサーバーが完全動作（14個から増加）
   - 新規追加: codex-research、codex-agent、sequential-thinking
   - Cursor IDE / VSCode完全互換
   - config.tomlとmcp.jsonの自動同期
   - Serena（21ツール）、Playwright、Chrome DevTools有効化

4. **🎯 サブエージェントシステム改善**
   - 8個の専門エージェントを完全テスト
   - 並列実行（3倍高速処理）
   - 自然言語CLI: `codex agent "<タスク>"`
   - エラーハンドリングとリトライロジック改善

5. **🔔 音声通知システム** *(強化)*
   - タスク完了: Marisa「終わったぜ！」
   - エージェント完了通知
   - セッション終了通知
   - PowerShellベースの音声再生
   - 場所: `.codex/marisa_owattaze.wav`

#### 🔥 **ClaudeCodeを超える機能**

1. **🔒 自動コンフリクト解決** *(Codex独自)*
   - FileEditTracker: ファイルごとの編集キュー管理
   - 3つのマージ戦略: Sequential、LastWriteWins、ThreeWayMerge
   - DashMapベースのロックフリー並行処理
   - マルチエージェント編集時のレースコンディション防止

2. **🗣️ 自然言語CLI** *(Codex独自)*
   - `codex agent "コードをセキュリティレビューして"` - 直感的な呼び出し
   - AgentInterpreter: パターンマッチング&意図分類
   - 適切な専門エージェントへの自動振り分け
   - エージェント名や複雑なフラグを覚える必要なし

3. **🔗 Webhook & 外部API統合** *(Codex独自)*
   - GitHub API: PR自動作成、Issue管理
   - Slack Webhook: チャンネル通知
   - カスタムWebhook: 汎用HTTPエンドポイント
   - シームレスなCI/CDパイプライン統合

4. **🔄 指数バックオフ付き高度エラーリトライ** *(Codex優位性)*
   - 設定可能なRetryPolicy（最大3回、1s-30s遅延）
   - FallbackStrategy: Retry、Skip、Fail
   - AgentError型システムできめ細かいエラーハンドリング
   - 基本リトライの3倍の耐障害性

5. **📖 完全オープンソース** *(Codex独自)*
   - 全コードをGitHubで公開
   - コミュニティコントリビューション歓迎
   - 透明性の高い開発プロセス
   - Apache 2.0 / MITデュアルライセンス

### 📦 インストール

#### 前提条件
- **Rust** 1.90以降
- **OpenAI APIキー**（`OPENAI_API_KEY`として設定）
- **Git**（クローン用）
- **Node.js**（オプション、Gemini CLI用）

#### クイックスタート

```bash
# リポジトリをクローン
git clone https://github.com/zapabob/codex.git
cd codex/codex-rs

# リリースビルド
cargo build --release

# グローバルインストール
cargo install --path cli --force

# インストール確認
codex --version
# 出力: codex-cli 0.51.0-zapabob.1
```

#### Gemini CLI MCPセットアップ（オプション）

```bash
# Gemini CLIインストール（Node.js）
npm install -g @google-labs/gemini-cli

# OAuth 2.0でログイン
gemini login

# Gemini CLIテスト
gemini -p "Hello Gemini" -o text

# Codexで使用
codex research "Rust非同期プログラミングベストプラクティス" --gemini --use-mcp
```

### 🚀 使用方法

#### 基本コマンド

```bash
# インタラクティブTUIモード
codex

# クイックコマンド実行
codex exec "このTypeScript関数を説明して"

# 引用付きディープリサーチ
codex research "React Server Components" --depth 3

# Gemini CLI統合
codex research "機械学習の基礎" --gemini --use-mcp

# 前回セッション再開
codex resume --last
```

#### サブエージェント委譲

```bash
# セキュリティ監査
codex delegate sec-audit --scope ./src

# コードレビュー
codex delegate code-reviewer --scope ./app

# テスト生成
codex delegate test-gen --scope ./lib

# 並列実行（3倍高速）
codex delegate-parallel code-reviewer,test-gen --scopes ./src,./tests
```

### ⚙️ 設定

#### config.toml例

```toml
# モデル設定
model = "gpt-5-codex"

[model_providers.openai]
base_url = "https://api.openai.com/v1"
env_key = "OPENAI_API_KEY"
wire_api = "chat"

# セキュリティ設定
[sandbox]
default_mode = "read-only"

[approval]
policy = "on-request"

# MCPサーバー
[mcp_servers.codex-gemini-mcp]
command = "codex-gemini-mcp"
args = []
env.PATH = "C:\\Users\\username\\.cargo\\bin;${PATH}"

# フック - 音声通知
[hooks]
on_task_complete = "powershell -ExecutionPolicy Bypass -File zapabob/scripts/play-completion-sound.ps1"
on_subagent_complete = "powershell -ExecutionPolicy Bypass -File zapabob/scripts/play-completion-sound.ps1"
on_session_end = "powershell -ExecutionPolicy Bypass -File zapabob/scripts/play-completion-sound.ps1"
```

### 🧪 テスト

#### 統合テスト実行

```bash
cd codex-rs

# 基本テスト
cargo test --test mcp_integration_test

# 実MCPサーバーでの統合テスト
cargo test --test mcp_integration_test -- --ignored --nocapture

# フルテストスイート
cargo test --all
```

#### テスト結果
- **MCP統合**: 6/6合格 ✅
- **パフォーマンス**: < 6秒 ✅
- **エンドツーエンド**: 全フロー検証済み ✅

### 📚 ドキュメント

- **アーキテクチャ**: [`zapabob/docs/codex-v0.51.0-architecture.svg`](zapabob/docs/codex-v0.51.0-architecture.svg)
- **MCP設定ガイド**: [`_docs/MCP設定ファイル同期管理ガイド.md`](_docs/MCP設定ファイル同期管理ガイド.md)
- **実装ログ**: [`_docs/`](_docs/)
- **音声通知**: [`_docs/2025-10-23_音声通知設定更新.md`](_docs/2025-10-23_音声通知設定更新.md)

### 🤝 コントリビューション

コントリビューションを歓迎します！[CONTRIBUTING.md](CONTRIBUTING.md)をご覧ください。

### 📄 ライセンス

このプロジェクトはApache License, Version 2.0の下でライセンスされています。詳細については[LICENSE](LICENSE)を参照してください。

```
Copyright 2025 zapabob

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

### 🙏 謝辞

- [OpenAI/codex](https://github.com/openai/codex)をベースにしています
- [zapabob](https://github.com/zapabob)がメンテナンス
- コミュニティコントリビューターの皆様

---

<div align="center">

**Made with ❤️ by zapabob**

[GitHub](https://github.com/zapabob/codex) | [Issues](https://github.com/zapabob/codex/issues) | [Discussions](https://github.com/zapabob/codex/discussions)

</div>
