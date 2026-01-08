# Codex - Extended Local AI Coding CLI with Advanced Build System

<p align="center"><code>npm i -g @zapabob/codex</code><br />or <code>just build-install</code></p>
<p align="center"><strong>Codex Extended CLI</strong> - OpenAI Codex with Fast Build & Hot Reload System
<p align="center">
  <img src="./.github/codex-cli-splash.png" alt="Codex CLI splash" width="80%" />
</p>
</br>
<strong>v2.9.0 "Fast Build & Hot Reload System"</strong> - Independent fork with enhanced development workflow.
</br>
*This is an independent fork/extension and is not affiliated with OpenAI.*

[![Version](https://img.shields.io/badge/version-2.9.0-blue.svg)](https://github.com/zapabob/codex)
[![npm](https://img.shields.io/npm/v/@zapabob/codex)](https://www.npmjs.com/package/@zapabob/codex)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache--2.0-green.svg)](LICENSE)

[English](#english) | [日本語](#japanese)

---

<a name="english"></a>
## 📖 English

### 🎯 TL;DR

**Codex Extended is OpenAI's Codex CLI with advanced build system and enhanced development workflow.**

- **Status**: CLI + Plan Mode + Sub-agents + Fast Build System are **stable**
- **Quickstart**: `npm i -g @zapabob/codex && codex --version && codex`
- **Use cases**: Task planning/execution, parallel reviews/tests, reproducible research, rapid development cycles

### Why Codex Extended?

Codex Extended enhances OpenAI's Codex with:

* Converting ambiguous tasks into executable plans (Plan mode)
* Delegating review/testing/security checks to parallel sub-agents
* Providing reproducible local research outputs with citations
* **Fast incremental builds with change detection** (70% faster)
* **Hot reload installation** with process management
* **Integrated release packaging** for all platforms

### What's extended compared to upstream OpenAI/codex?

* Plan mode execution workflow (create/approve/execute)
* Parallel sub-agent orchestration (delegate-parallel)
* Research workflow with citations and MCP integration
* Git analysis utilities for repository-level insights
* **Fast incremental build system with change detection**
* **Hot reload installation with process management**
* **Integrated release packaging for all platforms**

### Safety model

* Default sandbox: read-only
* Risky commands require explicit approval
* Agent actions can be audited via structured logs

### 📊 Feature Status Matrix

| Feature              | Status                     | Proof                        |
| -------------------- | -------------------------- | ---------------------------- |
| **Plan mode**        | ✅ **Stable**              | `docs/plan/README.md`        |
| **Sub-agents**       | ✅ **Stable**              | `docs/agents/README.md`      |
| **Deep research**    | ✅ **Stable**              | `docs/research/README.md`    |
| **Git analysis**     | ✅ **Stable**              | `docs/git/README.md`         |
| **GUI/Web interface**| 🧪 **Experimental**        | `docs/gui/README.md`         |
| **VR/AR support**    | 🧪 **Experimental**        | `docs/vr/README.md`          |
| **CUDA acceleration**| 🧪 **Experimental**        | `docs/benchmarks/cuda.md`    |

### 🚀 Quickstart

#### Install
```bash
# Recommended: npm package
npm install -g @zapabob/codex

# Or download a prebuilt binary from GitHub Releases (see Releases page).
```

#### Verify Installation
```bash
codex --version
# codex-cli 2.8.3
```

#### Basic Usage
```bash
# Interactive mode
codex

# Plan execution
codex plan create "Implement user authentication"
codex plan execute <plan-id>

# Sub-agent delegation
codex delegate code-reviewer --scope ./src
codex delegate-parallel code-reviewer,test-gen --scopes ./src,./tests

# Deep research
codex research "Rust async best practices"

# Git analysis
codex git-analyze commits
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

Codex extends OpenAI's Codex CLI with:

- **Plan Orchestrator**: Multi-step task planning and execution
- **Sub-Agent System**: Parallel AI agent execution (2.6x speedup)
- **Deep Research Engine**: MCP-integrated research with citations
- **Git Timeline Visualization**: 4D git history analysis
- **Security Sandbox**: Process isolation and permission management

### 📚 Documentation

- [Architecture](./ARCHITECTURE.md) - System design and components
- [Benchmarks](./docs/benchmarks.md) - Performance measurements
- [Security](./SECURITY.md) - Sandboxing and audit logging
- [Contributing](./CONTRIBUTING.md) - Development guidelines
- [Upstream](./UPSTREAM.md) - OpenAI Codex CLI documentation

### 🔧 What's New in v2.9.0

- ✅ **Fast Incremental Build**: MD5-based change detection with tqdm progress visualization
- ✅ **Hot Reload Installation**: Process-safe binary replacement with cross-platform support
- ✅ **Integrated Release Packaging**: Single tgz archive with all platform binaries
- ✅ **Development Workflow**: One-command build, test, and deploy pipeline

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

**CodexはPlan実行、並列サブエージェント、Git解析ユーティリティを拡張したローカルAIコーディングCLIです。**

- **ステータス**: CLI + Plan Mode + Sub-agentsは**安定版**。GUI/VR/CUDA高速化は**実験版**。
- **クイックスタート**: `npm i -g @zapabob/codex && codex --version && codex`
- **ユースケース**: タスク計画/実行、並列レビュー/テスト、再現性のある引用付きリサーチ、リポジトリレベル分析

### Codexを使う理由

Codexは日々のエンジニアリング作業のオーバーヘッドを削減します：

* 曖昧なタスクを実行可能な計画に変換（Planモード）
* レビュー/テスト/セキュリティチェックを並列サブエージェントに委任
* 引用付きの再現性のあるローカルリサーチ出力を提供

### 上流OpenAI/codexとの拡張点

* Planモード実行ワークフロー（作成/承認/実行）
* 並列サブエージェントオーケストレーション（delegate-parallel）
* 引用とMCP統合付きリサーチワークフロー
* リポジトリレベル洞察のためのGit解析ユーティリティ

### 安全モデル

* デフォルトサンドボックス: 読み取り専用
* 危険なコマンドには明示的な承認が必要
* エージェントアクションは構造化ログで監査可能

### 📊 機能ステータスマトリックス

| 機能                | ステータス                  | 証明                          |
| ------------------- | -------------------------- | ---------------------------- |
| **Plan mode**       | ✅ **安定版**              | `docs/plan/README.md`        |
| **Sub-agents**      | ✅ **安定版**              | `docs/guides/parallel-custom-agent.md` |
| **Deep research**   | ✅ **安定版**              | `docs/mcp/api-specification.md` |
| **Git analysis**    | ✅ **安定版**              | `docs/guides/cursor-integration.md` |
| **GUI/Webインターフェース** | 🧪 **実験版**        | `docs/guides/cursor-ide-setup.md` |
| **VR/AR対応**       | 🧪 **実験版**              | `docs/zapabob/AGENTS.md`     |
| **CUDA高速化**      | 🧪 **実験版**              | `CHANGELOG.md`               |

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

# Plan実行
codex plan create "Implement user authentication"
codex plan execute <plan-id>

# サブエージェント委任
codex delegate code-reviewer --scope ./src
codex delegate-parallel code-reviewer,test-gen --scopes ./src,./tests

# Deep research
codex research "Rust async best practices"

# Git解析
codex git-analyze commits
```

### 🏗️ アーキテクチャ

CodexはOpenAIのCodex CLIを以下で拡張：

- **Plan Orchestrator**: マルチステップタスクの計画・実行
- **Sub-Agent System**: 並列AIエージェント実行（2.6倍高速化）
- **Deep Research Engine**: MCP統合リサーチエンジン
- **Git Timeline Visualization**: 4D Git履歴解析
- **Security Sandbox**: プロセス分離と権限管理

### 📚 ドキュメント

- [Architecture](./ARCHITECTURE.md) - システム設計とコンポーネント
- [Benchmarks](./docs/benchmarks.md) - パフォーマンス測定
- [Security](./SECURITY.md) - サンドボックスと監査ログ
- [Contributing](./CONTRIBUTING.md) - 開発ガイドライン
- [Upstream](./UPSTREAM.md) - OpenAI Codex CLI ドキュメント

### 🔧 v2.9.0の新機能

- ✅ **高速インクリメンタルビルド**: MD5ベース変更検出とtqdm進捗可視化
- ✅ **ホットリロードインストール**: プロセス安全なバイナリ置換、クロスプラットフォーム対応
- ✅ **統合リリースパッケージ**: 全プラットフォームバイナリを含む単一tgzアーカイブ
- ✅ **開発ワークフロー**: ワンコマンドでのビルド・テスト・デプロイパイプライン

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

---

**Built with ❤️ by [@zapabob](https://github.com/zapabob)**
