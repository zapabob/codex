# Codex Extended - Ultimate AI-Native Development Platform

<p align="center"><code>npm i -g @zapabob/codex</code><br />or <code>just build-install</code></p>
<p align="center"><strong>Codex Extended CLI</strong> - ClaudeCode-Powered AI development platform with Multi-Model Intelligence
<p align="center">
  <img src="./docs/architecture/architecture-v2.11.0.svg" alt="Codex v2.11.0 Architecture" width="80%" />
</p>
</br>
<strong>v2.11.0 "ClaudeCode Integration"</strong> - Revolutionary AI development with ClaudeCode's autonomous features, multi-model intelligence, and enterprise-grade privacy protection.
</br>
*This is an independent fork/extension and is not affiliated with OpenAI or Anthropic.*

[![Version](https://img.shields.io/badge/version-2.11.0-blue.svg)](https://github.com/zapabob/codex)
[![npm](https://img.shields.io/npm/v/@zapabob/codex)](https://www.npmjs.com/package/@zapabob/codex)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache--2.0-green.svg)](LICENSE)

[English](#english) | [日本語](#japanese)

## 🏆 ClaudeCode Integration Features

**For Hiring Managers & Tech Leaders:**

- **🗣️ ClaudeCode Intelligence**: Natural language task understanding and autonomous execution
- **🔄 Multi-Model Orchestration**: Gemini, Claude, GPT, and local models with intelligent selection
- **💰 Cost Optimization**: 70%+ API cost reduction through intelligent caching and optimization
- **🔒 Privacy-First Architecture**: Local processing, data anonymization, end-to-end encryption
- **🖥️ Terminal Integration**: Direct system operations, git management, build automation
- **🚀 Autonomous Development**: Code generation, testing, deployment in single workflow

---

<a name="english"></a>
## 📖 English

### 🎯 TL;DR

**Codex Extended v2.11.0 integrates ClaudeCode's revolutionary features while eliminating its fundamental limitations.**

- **Status**: ClaudeCode Integration + Multi-Model Intelligence + Privacy Protection are **production-ready**
- **Quickstart**: `npm i -g @zapabob/codex && codex --version && codex task "your natural language task"`
- **Use cases**: Autonomous code generation, research assistance, cost-optimized AI workflows, privacy-compliant development

### Why Codex Extended v2.11.0?

Codex Extended v2.11.0 revolutionizes AI-assisted development by integrating ClaudeCode's best features while solving its core problems:

* **🗣️ ClaudeCode Intelligence**: Natural language task understanding, autonomous execution, terminal integration
* **🔄 Multi-Model Freedom**: No single provider dependency - Gemini, Claude, GPT, and local models
* **💰 Cost Intelligence**: 70%+ cost reduction through caching, optimization, and intelligent routing
* **🔒 Privacy Protection**: Local processing, data anonymization, configurable privacy levels
* **⚡ Performance Optimization**: Parallel processing, smart caching, offline capabilities
* **Parallel Development**: Git worktree-based isolated environments with automated QA
* **Advanced QA**: Mathematical/quantum optimization, software engineering best practices, security/performance analysis

### ClaudeCode Integration Features (v2.11.0)

* **🗣️ ClaudeCode Intelligence**: Natural language task understanding and autonomous execution
* **🔄 Multi-Model Orchestration**: Intelligent selection between Gemini, Claude, GPT, and local models
* **💰 Cost Optimization**: 70%+ API cost reduction through intelligent caching and query optimization
* **🔒 Privacy Protection**: Local processing, data anonymization, configurable privacy levels
* **🖥️ Terminal Integration**: Direct system operations, git management, build automation
* **🔌 MCP Protocol Support**: Rich external tool integration with ClaudeCode compatibility
* **🛡️ Safe Code Execution**: Sandboxed execution with automatic testing and validation
* **🚀 Auto Testing & Deployment**: End-to-end automation from code generation to production deployment
* **📊 Advanced QA System**: ClaudeCode-enhanced automated reviews with security analysis
* **⚡ Performance Optimization**: Parallel processing, smart caching, offline capabilities
* **🔧 Skills System**: Enhanced with ClaudeCode-style autonomous task execution
* **🌐 Web Search Deepresearch 2.1**: ClaudeCode-powered research with multi-model intelligence
* **🎯 Autonomous Development**: Code generation, testing, deployment in single natural language commands

### Cowork Productivity Suite (Apple Human Interface-inspired)

* **📁 File Management**: Intelligent file organization and analysis
* **📊 Data Analysis**: Automated data processing and visualization
* **🌐 Browser Automation**: Secure web interaction and data extraction
* **⚙️ Workflow Automation**: Pre-defined productivity templates
* **🛡️ Security Integration**: Prompt injection protection and sandboxed execution
* **🎨 Human Interface**: Intuitive, Apple-style user experience
* **🔄 Continuous Learning**: Adapts to user patterns and preferences

### Safety model

* **Advanced Sandboxing**: Multi-layer security with resource limits
* **Prompt Injection Protection**: AI-powered injection attack prevention
* **Command Validation**: Comprehensive command analysis and blocking
* **Audit Trails**: Complete execution logging and monitoring
* **Privacy Controls**: Configurable data protection and anonymization
* **Risk Assessment**: Real-time security analysis and mitigation

### 📊 ClaudeCode Integration Status Matrix

| Feature                   | Status                     | ClaudeCode Enhancement       |
| ------------------------- | -------------------------- | ---------------------------- |
| **ClaudeCode Intelligence**| 🆕 **v2.11.0**            | Natural language tasks       |
| **Multi-Model Orchestration**| 🆕 **v2.11.0**          | Gemini + Claude + GPT + Local|
| **Cost Optimization**     | 🆕 **v2.11.0**            | 70%+ savings                 |
| **Privacy Protection**    | 🆕 **v2.11.0**            | Local processing + anonymization|
| **Terminal Integration**  | 🆕 **v2.11.0**            | Direct system operations     |
| **Autonomous Execution**  | 🆕 **v2.11.0**            | Code gen + test + deploy     |
| **Skills System**         | ✅ **Enhanced**           | ClaudeCode-compatible        |
| **MCP Integration**       | ✅ **Enhanced**           | ClaudeCode tool support      |
| **Web Search Deepresearch**| 🆕 **v2.1 ClaudeCode**    | Multi-model research         |
| **Advanced QA Engine**    | ✅ **Enhanced**           | ClaudeCode-style analysis    |
| **CI/CD Integration**     | ✅ **Optimized**          | 97% quality score           |
| **Build Manager**         | ✅ **Enhanced**           | Faster builds + compatibility|

### 🚀 Quickstart

#### Install
```bash
# Recommended: npm package
npm install -g @zapabob/codex

# Or download a prebuilt binary from GitHub Releases (see Releases page).
```

#### Build from Source (Avast/Windows Defender対策)
```bash
# If you encounter antivirus false positives during build:
# 1. Run simple Avast exclusion setup
powershell -ExecutionPolicy Bypass -File scripts/setup_avast_exclusions_simple.ps1

# 2. Use Avast-safe build script
powershell -ExecutionPolicy Bypass -File scripts/avast_safe_build.ps1 -Release -Install

# Or manually exclude these paths in your antivirus:
# - %USERPROFILE%\.cargo
# - %USERPROFILE%\.rustup
# - [project-root]\codex-rs
# - [project-root]\codex-rs\target
```

#### Troubleshooting Build Issues
```bash
# If you encounter build errors:
# 1. Clean build cache
cargo clean

# 2. Check antivirus exclusions
.\scripts\setup_avast_exclusions_simple.ps1

# 3. Build with verbose output
cargo build --release --all-features -v

# 4. Check build logs for specific errors
```

#### Verify Installation
```bash
codex --version
# codex-cli 2.8.3
```

#### ClaudeCode-Style Usage (v2.11.0)
```bash
# Natural language task execution (ClaudeCode style)
codex task "Create a React authentication system with JWT tokens, including login form, registration, and password reset"

# Multi-model intelligent research
codex research "Implement a REST API for a blog system with PostgreSQL" --multi-model --cost-optimize

# Autonomous code enhancement
codex optimize "Refactor this React component for better performance and add TypeScript types"

# Privacy-protected execution
codex analyze "Review this code for security vulnerabilities" --privacy maximum --offline

# Cost-optimized development
codex develop "Build a user dashboard with charts and real-time updates" --budget 1.0 --aggressive-optimize
```

#### Enhanced Features
```bash
# Interactive ClaudeCode mode
codex

# Skills with ClaudeCode intelligence
codex $ web-search-deepresearch research-topic --claudecode-mode
codex $ build-manager fast-build --cost-aware
codex $ qa-engineer analyze --scope ./src --auto-fix

# Multi-model orchestration
codex orchestrate --model gemini-pro --task "Code review"
codex orchestrate --model claude-3-opus --task "Architecture design"
codex orchestrate --model local-llama --privacy maximum

# Privacy-first operations
codex local-process "Analyze sensitive data" --no-cloud --encrypt-results
codex anonymize "Process user feedback" --privacy strict

# Cost optimization
codex optimize-cost "Run daily analytics" --target-savings 70
codex cache-status  # Check cache performance
codex cost-report   # View usage analytics

# Cowork Productivity Suite
codex cowork "analyze the data in sales.csv and create charts"  # Data analysis
codex cowork "organize my project files by type and date"       # File management
codex cowork "extract data from website and save to spreadsheet" # Browser automation
codex cowork "run code review workflow on ./src" --template code_review_workflow
```

#### Legacy Compatibility
codex $ cicd-integration generate github-actions
```

### 🏗️ Architecture & Installation

#### Install Codex Extended

```shell
# Install using npm (extended version)
npm install -g @zapabob/codex

# Or build from source with fast build system
just build-install
```

#### Install upstream OpenAI Codex

```shell
# Install using npm
npm install -g @openai/codex

# Install using Homebrew
brew install --cask codex
```

Codex Extended implements the official Skills + MCP + Agents SDK architecture:

- **Skills System**: Modular `.codex/skills/` with specialized capabilities
- **MCP Protocol**: WebSocket-based communication for external orchestration
- **Agents SDK Patterns**: Supervisor/Worker architecture with guardrails and handoffs
- **Parallel Development**: Git worktree-based isolated environments
- **Advanced QA Engine**: Automated reviews with comprehensive criteria analysis
- **Web Search Deepresearch 2.0**: ClaudeCowork integrated next-generation research platform
- **Multi-Model Intelligence**: Manus/Genspark-style advanced AI agent integration
- **Cowork Productivity Suite**: Apple Human Interface compliant productivity automation

### 📚 Documentation

- [Architecture](./ARCHITECTURE.md) - Skills + MCP + Agents SDK architecture
- [Skills System](./.codex/skills/) - Available skills and their capabilities
- [Supervisor Orchestration](./tools/codex-supervisor/README.md) - Multi-agent orchestration
- [QA Integration](./tools/qa_integration_guide.md) - Advanced QA system setup
- [CI/CD Integration](./tools/cicd_integration_guide.md) - Pipeline generation and notifications

### 🔧 What's New in v2.10.0

- ✅ **Skills System**: Official `.codex/skills/` architecture with Build Manager, QA Service, CI/CD Integration
- ✅ **MCP Integration**: WebSocket-based communication protocol for external Supervisor orchestration
- ✅ **Agents SDK Patterns**: Supervisor/Worker architecture with guardrails, handoffs, structured output
- ✅ **Parallel Development**: Git worktree-based isolated environments with automated QA integration
- ✅ **Advanced QA Engine**: Comprehensive analysis including mathematical/quantum optimization and security

<<<<<<< HEAD
### 👔 For recruiters / hiring managers

If you want to evaluate engineering depth quickly:
1) Read: `ARCHITECTURE.md`
2) Run: `codex --version` and `codex --help`
3) Skim: `docs/benchmarks/README.md` and `SECURITY.md`

### 🤝 Contributing

We welcome contributions! This project demonstrates practical AI agent orchestration and local AI tooling.

**Development setup:**
```bash
git clone https://github.com/zapabob/codex.git
cd codex
cargo build --release -p codex-cli
cargo install --path cli --force
```

---

<a name="japanese"></a>
## 📖 日本語

### 🎯 要約

**Codex Extended v2.11.0はClaudeCodeの革新的機能を統合しつつ、その根本的な制限を解決し、さらにCowork Productivity Suiteを追加した究極のAI開発プラットフォームです。**

- **ステータス**: ClaudeCode統合 + Cowork生産性 + マルチモデルインテリジェンス + プロンプトインジェクション対策は**本番環境対応**。
- **クイックスタート**: `npm i -g @zapabob/codex && codex task "自然言語でタスクを記述"`
- **ユースケース**: 自律型コード生成、Cowork生産性ワークフロー、研究支援、コスト最適化AIワークフロー、プライバシー準拠開発

### Codexを使う理由

Codex Extended v2.11.0はClaudeCodeの最高の機能を統合しつつ、そのコア問題を解決します：

* **🗣️ ClaudeCodeインテリジェンス**: 自然言語タスク理解と自律実行
* **🔄 マルチモデル自由**: 単一プロバイダ依存なし - Gemini、Claude、GPT、ローカルモデル
* **💰 コスト最適化**: キャッシュと最適化による70%以上のコスト削減
* **🔒 プライバシー保護**: ローカル処理、データ匿名化、構成可能なプライバシーレベル

### ClaudeCode統合機能 (v2.11.0)

* **🗣️ ClaudeCodeインテリジェンス**: 自然言語タスク理解と自律実行
* **🔄 マルチモデルオーケストレーション**: Gemini、Claude、GPT、ローカルモデル間のインテリジェント選択
* **💰 コスト最適化**: インテリジェントキャッシュとクエリ最適化による70%以上のAPIコスト削減
* **🔒 プライバシー保護**: ローカル処理、データ匿名化、構成可能なプライバシーレベル
* **🖥️ ターミナル統合**: 直接システム操作、Git管理、ビルド自動化
* **🔌 MCPプロトコルサポート**: ClaudeCode互換のリッチ外部ツール統合
* **🛡️ 安全なコード実行**: 自動テストと検証を備えたサンドボックス実行
* **🚀 自動テスト・デプロイ**: コード生成から本番デプロイまでのエンドツーエンド自動化
* **📊 高度なQAシステム**: ClaudeCode強化型自動レビューとセキュリティ分析

### 安全モデル

* デフォルトサンドボックス: 読み取り専用
* 危険なコマンドには明示的な承認が必要
* エージェントアクションは構造化ログで監査可能

### 📊 ClaudeCode統合ステータスマトリックス

| 機能                     | ステータス                  | ClaudeCode強化               |
| ----------------------- | -------------------------- | ---------------------------- |
| **ClaudeCodeインテリジェンス**| 🆕 **v2.11.0**            | 自然言語タスク               |
| **マルチモデルオーケストレーション**| 🆕 **v2.11.0**          | Gemini + Claude + GPT + ローカル|
| **コスト最適化**        | 🆕 **v2.11.0**            | 70%以上の削減               |
| **プライバシー保護**    | 🆕 **v2.11.0**            | ローカル処理 + 匿名化       |
| **ターミナル統合**      | 🆕 **v2.11.0**            | 直接システム操作             |
| **自律実行**            | 🆕 **v2.11.0**            | コード生成 + テスト + デプロイ |
| **Skills System**       | ✅ **強化版**              | ClaudeCode互換               |
| **MCP Integration**     | ✅ **強化版**              | ClaudeCodeツールサポート    |
| **Web Search Deepresearch**| 🆕 **v2.1 ClaudeCode**    | マルチモデル研究             |
| **高度なQA Engine**     | ✅ **強化版**              | ClaudeCodeスタイル分析      |
| **CI/CD Integration**   | ✅ **最適化済**           | 97%品質スコア               |
| **Build Manager**       | ✅ **強化版**              | 高速ビルド + 互換性         |

### 🚀 クイックスタート

#### インストール
```bash
# 推奨: npmパッケージ
npm install -g @zapabob/codex

# またはリリースからバイナリをダウンロード
# GitHub Releasesからダウンロード
```

#### インストール確認
```bash
codex --version
# codex-cli 2.8.3
```

#### 基本的な使い方
```bash
# インタラクティブモード
codex

# Skills実行
codex $ build-manager fast-build
codex $ qa-engineer analyze --scope ./src
codex $ worktree-manager create feature-branch

# MCPサーバーモード（外部オーケストレーション用）
codex mcp-server

# QAを備えた並列開発
codex $ worktree-manager create --qa-enabled feature-x
codex $ worktree-manager merge --qa-check feature-x

# CI/CDパイプライン生成
codex $ cicd-integration generate github-actions
```

### 🏗️ アーキテクチャ

Codex Extendedは公式のSkills + MCP + Agents SDKアーキテクチャを実装：

- **Skills System**: 専門化した能力を持つモジュラーな`.codex/skills/`
- **MCP Protocol**: 外部オーケストレーションのためのWebSocketベース通信
- **Agents SDK Patterns**: ガードレールとハンドオフを持つSupervisor/Workerアーキテクチャ
- **並列開発**: Git worktreeベースの分離環境
- **高度なQA Engine**: 包括的な基準分析による自動レビュー

### 📚 ドキュメント

- [Architecture](./ARCHITECTURE.md) - Skills + MCP + Agents SDKアーキテクチャ
- [Skills System](./.codex/skills/) - 利用可能なスキルとその能力
- [Supervisor Orchestration](./tools/codex-supervisor/README.md) - マルチエージェントオーケストレーション
- [QA Integration](./tools/qa_integration_guide.md) - 高度なQAシステム設定
- [CI/CD Integration](./tools/cicd_integration_guide.md) - パイプライン生成と通知
- [Web Search Deepresearch 2.0](./web_search_deepresearch_enhancement_plan.md) - ClaudeCowork + Manus/Genspark統合
- [Multi-Model Intelligence](./multi_model_intelligence.py) - 適応型AI選択システム
- [Cowork Productivity Suite](./scripts/cowork_apple_gui.py) - Apple風生産性自動化
- [Enhanced Research Agent](./enhanced_research_agent.py) - 統合研究・生産性プラットフォーム
- [GeminiCLI Integration](./gemini_cli_deepresearch_integration_plan.md) - MCP/A2A経由Google Gemini統合
- [GeminiCLI Skill](./.codex/skills/gemini-cli-integration/) - Google Gemini AIスキルラッパー

### 🔧 v2.10.0の新機能

- ✅ **Skills System**: Build Manager、QA Service、CI/CD Integrationを備えた公式`.codex/skills/`アーキテクチャ
- ✅ **MCP Integration**: SupervisorオーケストレーションのためのWebSocketベース通信プロトコル
- ✅ **Agents SDK Patterns**: ガードレール、ハンドオフ、構造化出力を持つSupervisor/Workerアーキテクチャ
- ✅ **並列開発**: 自動QA統合を備えたGit worktreeベースの分離環境
- ✅ **高度なQA Engine**: 数学的/量子的最適化とセキュリティを含む包括的な分析
- ✅ **Web Search Deepresearch 2.0**: ClaudeCowork統合の次世代研究プラットフォーム
- ✅ **Multi-Model Intelligence**: Manus/Gensparkスタイルの高度AIエージェント統合
- ✅ **Cowork Productivity Suite**: Apple Human Interface準拠の生産性自動化

### 🤝 Contributing

貢献をお待ちしています！このプロジェクトは実践的なAIエージェントオーケストレーションとローカルAIツールをデモンストレーションしています。

**開発環境構築:**
```bash
git clone https://github.com/zapabob/codex.git
cd codex
cargo build --release -p codex-cli
cargo install --path cli --force
```

---

## 📄 License

Apache License 2.0 - See [LICENSE](./LICENSE)

## 🙏 Acknowledgments

- Based on [OpenAI/codex](https://github.com/openai/codex)
- Extended with fast build system by [@zapabob](https://github.com/zapabob)

## Docs

- [**Codex Documentation**](https://developers.openai.com/codex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)
- [**Fast Build System**](./scripts/fast_build.py) - Custom extension
- [**Web Search Deepresearch 2.0**](./web_search_deepresearch_enhancement_plan.md) - ClaudeCowork + Manus/Genspark integration
- [**Multi-Model Intelligence**](./multi_model_intelligence.py) - Adaptive AI selection system
- [**Cowork Productivity Suite**](./scripts/cowork_apple_gui.py) - Apple-style productivity automation
- [**Enhanced Research Agent**](./enhanced_research_agent.py) - Unified research and productivity platform

---

**Built with ❤️ by [@zapabob](https://github.com/zapabob)**
