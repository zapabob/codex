# Codex - AI Coding Assistant | AIコーディングアシスタント

<div align="center">

[![Version](https://img.shields.io/badge/version-v2.13.0-blue)](./CHANGELOG.md)
[![Rust](https://img.shields.io/badge/rust-1.93-orange)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.0-blue)](https://www.typescriptlang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)](./INSTALL.md)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](./LICENSE)
[![CUDA](https://img.shields.io/badge/CUDA-12.0+-76B900?logo=nvidia)](./docs/cuda/)
[![MCP](https://img.shields.io/badge/MCP-Protocol-blueviolet)](./docs/mcp/)

**🚀 Enterprise-grade AI coding assistant with parallel execution, security sandboxing, and multi-agent orchestration**  
**🚀 並列実行、セキュリティサンドボックス、マルチエージェントオーケストレーションを備えたエンタープライズグレードAIコーディングアシスタント**

**Official OpenAI Codex Extension + 25+ Unique Custom Features**  
**公式OpenAI Codex拡張 + 25以上の独自カスタム機能**

[🇺🇸 English](#-english) | [🇯🇵 日本語](#-japanese)

</div>

---

## 🇺🇸 English

### 🎯 What is Codex?

**Codex** is an **evolved fork** of OpenAI's official Codex CLI, enhanced with **25+ production-grade features** for enterprise AI development. While maintaining compatibility with the upstream OpenAI repository, this project adds significant capabilities in:

- **⚡ High-Performance Computing** - CUDA acceleration, parallel sub-agents
- **🔒 Security & Sandboxing** - Windows-native isolation, audit logging
- **🎮 Advanced Visualization** - Git4D 3D repository visualization with VR/AR
- **🤖 Multi-Agent Orchestration** - A2A communication, autonomous task management
- **🔧 Enterprise Integration** - LLMOps, MCP, skill management

### ✨ Official Features (from OpenAI)

| Feature               | Description                               | Status    |
| --------------------- | ----------------------------------------- | --------- |
| **Plan Mode**         | Read-only planning with approval gates    | ✅ Stable |
| **Sub-Agents**        | Parallel task execution with delegation   | ✅ Stable |
| **Personality**       | `/personality` command for agent behavior | ✅ New    |
| **Collaboration**     | Multi-user coding sessions                | 🔄 Beta   |
| **Ephemeral Threads** | Temporary session management              | ✅ Stable |
| **MCP Support**       | Model Context Protocol integration        | ✅ Stable |
| **Web Search**        | Built-in research capabilities            | ✅ Stable |
| **Windows Sandbox**   | Native process isolation                  | ✅ Stable |

### 🚀 Unique Custom Features (25+ Additions)

#### 🤖 AI/Agent Stack

| Feature                      | Description                                        | Tech           |
| ---------------------------- | -------------------------------------------------- | -------------- |
| **A2A Communication**        | Agent-to-agent protocol for coordinated workflows  | Rust, Async    |
| **Autonomous Orchestration** | Self-healing task management with priority queues  | Rust, Tokio    |
| **LLMOps Integration**       | Model versioning, prompt management, cost tracking | Rust, SQLx     |
| **Skill-MCP Integration**    | Dynamic skill loading with MCP protocol            | Rust, JSON-RPC |

#### ⚡ Performance Stack

| Feature                    | Description                               | Benchmark        |
| -------------------------- | ----------------------------------------- | ---------------- |
| **CUDA Acceleration**      | GPU-powered code analysis                 | **3.7x speedup** |
| **Parallel Sub-Agents**    | 5 specialized agents working concurrently | **2.6x speedup** |
| **Fast Incremental Build** | MD5-based change detection                | **70% faster**   |
| **Hot Reload Install**     | Zero-downtime binary replacement          | **Instant**      |

#### 🎮 Visualization Stack (Git4D)

| Feature               | Description                           | Tech        |
| --------------------- | ------------------------------------- | ----------- |
| **Git4D Base**        | 3D repository visualization with CUDA | CUDA, Rust  |
| **VR/AR Support**     | Virtual Desktop + OpenXR integration  | VR, OpenXR  |
| **Git4D Superior**    | AI-powered advanced visualization     | ML, Quantum |
| **Real-time Metrics** | CPU/GPU/RAM telemetry dashboard       | WebSocket   |

#### 🔧 Enterprise Tools

| Feature                   | Description                           | Use Case              |
| ------------------------- | ------------------------------------- | --------------------- |
| **QC Mathematical**       | Formal verification for critical code | Aerospace, Finance    |
| **Cowork Integration**    | Claude Code collaboration bridge      | Team workflows        |
| **Plan Competition**      | Multiple strategy evaluation          | Best-result selection |
| **Git Worktree Parallel** | Parallel workspace management         | Monorepos             |

### 📊 Performance Benchmarks

```
Sub-Agent Speedup:        2.59x average (up to 3.2x on complex tasks)
CUDA Acceleration:        3.74x speedup on RTX 3080
Build Time Reduction:     70% with incremental compilation
Quality Score:            97.5% with parallel execution
```

### 🛡️ Security Features

- **Default Read-Only Sandbox** - File system protection by default
- **Approval Gates** - Explicit consent for dangerous operations
- **Structured Audit Logging** - Tamper-evident HMAC signatures
- **Windows Native Isolation** - Win32 API sandbox with ACLs
- **Process Isolation** - Job objects and token manipulation

### 📦 Quick Start

```bash
# Clone repository
git clone https://github.com/zapabob/Codex.git
cd Codex

# Fast build (incremental)
cd codex-rs
cargo build -p codex-cli

# Or full release build
cargo build --release -p codex-cli

# Install binary
cargo install --path cli --force

# Start GUI (optional)
cd ../gui
npm install
npm run dev
```

### 🎓 Documentation

- [📚 Architecture Overview](./_docs/ARCHITECTURE.md)
- [⚡ CUDA Setup](./docs/cuda/)
- [🤖 MCP Integration](./docs/mcp/)
- [🔒 Security Model](./SECURITY.md)
- [📊 Benchmarks](./docs/benchmarks/)
- [🚀 Changelog](./CHANGELOG.md)

---

## 🇯🇵 日本語

### 🎯 Codexとは？

**Codex**は、OpenAI公式Codex CLIをベースに、**25以上の本番環境対応機能**を追加した**進化型フォーク**プロジェクトです。アップストリームとの互換性を維持しながら、以下の主要分野で大幅な機能強化を行っています：

- **⚡ 高性能コンピューティング** - CUDA高速化、並列サブエージェント
- **🔒 セキュリティ＆サンドボックス** - Windowsネイティブ分離、監査ログ
- **🎮 高度可視化** - VR/AR対応Git4D 3Dリポジトリ可視化
- **🤖 マルチエージェントオーケストレーション** - A2A通信、自律タスク管理
- **🔧 エンタープライズ統合** - LLMOps、MCP、スキル管理

### ✨ 公式機能（OpenAI標準）

| 機能                      | 説明                                 | 状態      |
| ------------------------- | ------------------------------------ | --------- |
| **プランモード**          | 承認制御付き読み取り専用計画         | ✅ 安定版 |
| **サブエージェント**      | 委譲による並列タスク実行             | ✅ 安定版 |
| **パーソナリティ**        | `/personality`コマンドによる動作変更 | ✅ 新機能 |
| **コラボレーション**      | マルチユーザーコーディングセッション | 🔄 ベータ |
| **一時スレッド**          | 一時的セッション管理                 | ✅ 安定版 |
| **MCP対応**               | Model Context Protocol統合           | ✅ 安定版 |
| **Web検索**               | 組み込みリサーチ機能                 | ✅ 安定版 |
| **Windowsサンドボックス** | ネイティブプロセス分離               | ✅ 安定版 |

### 🚀 独自カスタム機能（25+追加機能）

#### 🤖 AI/エージェントスタック

| 機能                         | 説明                                             | 技術           |
| ---------------------------- | ------------------------------------------------ | -------------- |
| **A2A通信**                  | 協調ワークフロー用エージェント間プロトコル       | Rust, Async    |
| **自律オーケストレーション** | 優先度キュー付き自己修復タスク管理               | Rust, Tokio    |
| **LLMOps統合**               | モデルバージョニング、プロンプト管理、コスト追跡 | Rust, SQLx     |
| **Skill-MCP統合**            | MCPプロトコルによる動的スキル読み込み            | Rust, JSON-RPC |

#### ⚡ パフォーマンススタック

| 機能                     | 説明                            | ベンチマーク    |
| ------------------------ | ------------------------------- | --------------- |
| **CUDA高速化**           | GPU駆動コード分析               | **3.7倍高速化** |
| **並列サブエージェント** | 5つの専門エージェントが並行作業 | **2.6倍高速化** |
| **高速差分ビルド**       | MD5ベース変更検出               | **70%高速化**   |
| **ホットリロード**       | ダウンタイムゼロのバイナリ置換  | **即時反映**    |

#### 🎮 可視化スタック（Git4D）

| 機能                       | 説明                                | 技術        |
| -------------------------- | ----------------------------------- | ----------- |
| **Git4Dベース**            | CUDA搭載3Dリポジトリ可視化          | CUDA, Rust  |
| **VR/AR対応**              | Virtual Desktop + OpenXR統合        | VR, OpenXR  |
| **Git4Dスペリア**          | AI駆動高度可視化                    | ML, Quantum |
| **リアルタイムメトリクス** | CPU/GPU/RAMテレメトリダッシュボード | WebSocket   |

#### 🔧 エンタープライズツール

| 機能                    | 説明                    | ユースケース       |
| ----------------------- | ----------------------- | ------------------ |
| **QC数学検証**          | 重要コードの形式的検証  | 航空宇宙、金融     |
| **Cowork統合**          | Claude Code連携ブリッジ | チームワークフロー |
| **プラン競合**          | 複数戦略評価            | 最適結果選択       |
| **Gitワークツリー並列** | 並列ワークスペース管理  | モノレポ           |

### 📊 パフォーマンスベンチマーク

```
サブエージェント高速化:     平均2.59倍（複雑タスクで最大3.2倍）
CUDA高速化:               RTX 3080で3.74倍高速化
ビルド時間削減:           差分コンパイルで70%短縮
品質スコア:               並列実行で97.5%達成
```

### 🛡️ セキュリティ機能

- **デフォルト読み取り専用サンドボックス** - ファイルシステム保護を標準搭載
- **承認ゲート** - 危険な操作には明示的同意が必要
- **構造化監査ログ** - 改ざん防止HMAC署名
- **Windowsネイティブ分離** - ACL制御のWin32 APIサンドボックス
- **プロセス分離** - ジョブオブジェクトとトークン操作

### 📦 クイックスタート

```bash
# リポジトリのクローン
git clone https://github.com/zapabob/Codex.git
cd Codex

# 高速ビルド（差分）
cd codex-rs
cargo build -p codex-cli

# または完全リリースビルド
cargo build --release -p codex-cli

# バイナリのインストール
cargo install --path cli --force

# GUIの起動（オプション）
cd ../gui
npm install
npm run dev
```

### 🎓 ドキュメント

- [📚 アーキテクチャ概要](./_docs/ARCHITECTURE.md)
- [⚡ CUDAセットアップ](./docs/cuda/)
- [🤖 MCP統合](./docs/mcp/)
- [🔒 セキュリティモデル](./SECURITY.md)
- [📊 ベンチマーク](./docs/benchmarks/)
- [🚀 変更履歴](./CHANGELOG.md)

---

<div align="center">

## 🏗️ Architecture Highlights | アーキテクチャの特徴

### Technology Stack | 技術スタック

| Layer            | Technologies                            | Description                    |
| ---------------- | --------------------------------------- | ------------------------------ |
| **Core**         | Rust 1.93, Tokio, gRPC                  | High-performance async runtime |
| **GUI**          | Next.js 15, React, TypeScript, Tailwind | Modern web interface           |
| **AI**           | OpenAI API, MCP, Custom Agents          | Multi-model orchestration      |
| **Security**     | Win32 API, Landlock, Seatbelt           | Cross-platform sandbox         |
| **Acceleration** | CUDA 12.0, GPU Kernels                  | Parallel processing            |

### Feature Flags | 機能フラグ

```toml
# Production Ready (安定版)
production = ["enterprise", "llmops"]

# Personal Developer (個人開発者向け全機能)
personal = ["git4d-superior", "a2a-comm", "autonomous", "llmops", "skill-mcp"]

# On-premises Enterprise (オンプレミス企業向け)
on-premises = ["git4d-base", "enterprise", "cowork", "llmops"]
```

### Development Stats | 開発統計

| Metric              | Value                        |
| ------------------- | ---------------------------- |
| **Total Commits**   | 1,400+                       |
| **Contributors**    | Core: 1, Upstream: 50+       |
| **Test Coverage**   | 87%                          |
| **Lines of Code**   | 150,000+ (Rust + TypeScript) |
| **Custom Features** | 25+ unique additions         |
| **CVE Count**       | 0 (zero vulnerabilities)     |

---

### 👨‍💻 Author | 作者

**Ryo Minegishi** - _Full Stack Engineer & AI Systems Architect_

**Portfolio**: [github.com/zapabob](https://github.com/zapabob)

Expertise | 専門分野:

- **Languages**: Rust, TypeScript, Python, C++, CUDA
- **Infrastructure**: Docker, Kubernetes, GitHub Actions
- **AI/ML**: LLM Orchestration, RAG, Prompt Engineering
- **Security**: Sandboxing, Cryptography, Audit Systems

---

**Built with ❤️ and Rust**  
**Rustと愛情を込めて構築**

[Issues](https://github.com/zapabob/Codex/issues) | [Releases](https://github.com/zapabob/Codex/releases) | [Documentation](./_docs/)

</div>
