# Codex - Advanced AI Coding Assistant | 高度なAIコーディングアシスタント

<div align="center">

![Version](https://img.shields.io/badge/version-v2.12.1-blue)
![Rust](https://img.shields.io/badge/rust-1.93-orange)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/license-Apache%202.0-green)

**Enterprise-grade AI coding assistant with parallel execution, security sandboxing, and multi-agent orchestration**

**並列実行、セキュリティサンドボックス、マルチエージェントオーケストレーションを備えたエンタープライズグレードのAIコーディングアシスタント**

[English](#english) | [日本語](#japanese)

</div>

---

<a name="english"></a>

## 🇺🇸 English

### 🎯 Project Overview

This is an advanced fork of OpenAI's Codex project, enhanced with custom features for enterprise-grade AI development workflows. The project demonstrates expertise in:

- **Systems Programming (Rust)**: High-performance CLI/TUI applications with memory safety
- **Distributed Systems**: Parallel sub-agent execution and Git worktree management
- **Security Engineering**: Windows sandbox implementation with ACL/token management
- **AI/ML Integration**: Multi-model orchestration (OpenAI, Anthropic, Google Gemini)

### 🚀 Key Features (v2.12.1)

| Feature                               | Description                            | Technical Highlight                               |
| ------------------------------------- | -------------------------------------- | ------------------------------------------------- |
| **Parallel Sub-Agent Execution**      | Run multiple AI agents concurrently    | Async Rust with Tokio runtime                     |
| **Git Worktree Parallel Development** | Manage multiple workspaces efficiently | Conflict prevention & auto-merge                  |
| **Windows Security Sandbox**          | Isolated execution environment         | Win32 ACL, token restriction, firewall rules      |
| **A2A Communication**                 | Agent-to-Agent messaging protocol      | JSON-RPC based inter-agent coordination           |
| **QC Optimization Competition**       | Quality control evaluation system      | Parallel scoring with quantum-inspired algorithms |
| **MCP (Model Context Protocol)**      | Multi-model context management         | Unified API for multiple LLM providers            |

### 🏗️ Architecture

```
codex-rs/
├── cli/              # Command-line interface (Rust)
├── tui/              # Terminal UI with ratatui
├── core/             # Core orchestration engine
│   ├── agents/       # Parallel executor, runtime
│   ├── orchestration/# Worktree manager, QC evaluator
│   ├── qc/           # Quality control optimization
│   └── a2a_communication.rs
├── windows-sandbox-rs/  # Windows security sandbox
│   ├── acl.rs        # Access Control Lists
│   ├── token.rs      # Token restriction
│   └── sandbox_users.rs
├── mcp-server/       # Model Context Protocol server
└── protocol/         # Communication protocols
```

### 💻 Technical Skills Demonstrated

#### Rust Systems Programming

- Zero-cost abstractions with async/await
- Memory safety without garbage collection
- Cross-platform compilation (Windows/macOS/Linux)
- FFI with Windows sys-calls

#### DevOps & CI/CD

- GitHub Actions with 22+ workflow files
- Multi-platform build matrix (x86_64, aarch64)
- Automated release management
- Security scanning integration

#### Security Engineering

- Windows sandbox implementation from scratch
- ACL manipulation and token restriction
- Firewall rule automation
- DPAPI encryption for secrets

### 📦 Installation

```bash
# Clone the repository
git clone https://github.com/zapabob/Codex.git
cd Codex

# Build release binaries
cd codex-rs
cargo build --release

# Install to ~/.cargo/bin
cargo install --path cli
```

### 🔧 Configuration

# ~/.codex/config.toml

[model]
provider = "openai" # or "anthropic", "gemini"
model = "gpt-5.2codex"

[sandbox]
enabled = true
network_access = false

[parallel]
max_agents = 4

```

---

<a name="japanese"></a>

## 🇯🇵 日本語

### 🎯 プロジェクト概要

このプロジェクトはOpenAIのCodexプロジェクトの高度なフォークであり、エンタープライズグレードのAI開発ワークフロー向けにカスタム機能を追加しています。以下の専門性を実証しています：

- **システムプログラミング (Rust)**: メモリ安全性を備えた高性能CLI/TUIアプリケーション
- **分散システム**: 並列サブエージェント実行とGit worktree管理
- **セキュリティエンジニアリング**: ACL/トークン管理によるWindowsサンドボックス実装
- **AI/ML統合**: マルチモデルオーケストレーション (OpenAI、Anthropic、Google Gemini)

### 🚀 主要機能 (v2.12.1)

| 機能                                  | 説明                                   | 技術的ハイライト                                   |
| ------------------------------------- | -------------------------------------- | -------------------------------------------------- |
| **並列サブエージェント実行**          | 複数のAIエージェントを同時実行         | Tokioランタイムによる非同期Rust                    |
| **Git Worktree並列開発**              | 複数ワークスペースの効率的管理         | コンフリクト防止と自動マージ                       |
| **Windowsセキュリティサンドボックス** | 隔離された実行環境                     | Win32 ACL、トークン制限、ファイアウォールルール    |
| **A2A通信**                           | エージェント間メッセージングプロトコル | JSON-RPCベースのエージェント間調整                 |
| **QC最適化コンペティション**          | 品質管理評価システム                   | 量子インスパイアアルゴリズムによる並列スコアリング |
| **MCP (Model Context Protocol)**      | マルチモデルコンテキスト管理           | 複数LLMプロバイダー向け統一API                     |

### 🛠️ 技術スタック

```

言語: Rust 1.93, TypeScript, Python
ランタイム: Tokio (非同期), Ratatui (TUI)
プラットフォーム: Windows, macOS, Linux (x86_64, aarch64)
CI/CD: GitHub Actions (22ワークフロー)
セキュリティ: Windows Sandbox, ACL, DPAPI

```

### 📊 v2.12.1 リリースノート

#### 修正内容

1. **windows-sandbox-rs モジュール構造修正**
   - 二重モジュール宣言の問題を解決
   - `sandbox_users.rs`のインポートパス修正
   - ローカル`log_line`関数の実装

2. **Upstream統合**
   - OpenAI/codex公式リポジトリからの最新機能取り込み
   - セキュリティ更新とバグ修正の適用
   - 独自機能の完全保持

3. **CI/CD修正**
   - GitHub Actions @v6 → @v4 置換 (52箇所以上)
   - 壊れたバリデーションの削除

### 🎓 開発者について

このプロジェクトは以下のスキルセットを持つエンジニアによって開発されています：

- Rustによる高性能システムプログラミング
- Windowsカーネル/セキュリティAPI
- 分散システムとマルチエージェントアーキテクチャ
- CI/CDパイプライン設計と自動化
- AI/LLM統合とプロンプトエンジニアリング

### 📄 ライセンス

Apache License 2.0

---

<div align="center">

**Built with ❤️ and Rust**

[Issues](https://github.com/zapabob/Codex/issues) | [Releases](https://github.com/zapabob/Codex/releases) | [Documentation](./_docs/)

</div>
```
