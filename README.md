# Codex - Advanced AI Coding Assistant | 高度なAIコーディングアシスタント

<div align="center">

![Version](https://img.shields.io/badge/version-v2.13.0-blue)
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

**Codex** is an evolved fork of the original OpenAI Codex project, re-engineered for **enterprise-scale AI development**. This project serves as a comprehensive portfolio demonstrating advanced capabilities in systems programming, distributed architectures, and AI integration.

Designed to solve complex coding tasks autonomously, Codex leverages a **multi-agent architecture** to plan, execute, and verify software changes securely and efficiently.

### 🚀 Key Features (v2.13.0)

This release introduces significant enhancements to the GUI and system integration subsystems.

| Feature domain           | Capabilities                                                      | Technical Stack                                      |
| :----------------------- | :---------------------------------------------------------------- | :--------------------------------------------------- |
| **🤖 Autonomous Agents** | Parallel sub-agent execution for complex tasks                    | **Rust (Tokio)**, Async/Await, Actor Model           |
| **🛡️ Security Sandbox**  | Windows-native isolation for untrusted code execution             | **Win32 API**, ACLs, Token Manipulation, Job Objects |
| **🖥️ Modern GUI**        | Real-time dashboard with system metrics & CLI bridge              | **Next.js**, React, WebSocket, Tailwind CSS          |
| **⚡ Performance**       | High-concurrency I/O and low-latency IPC                          | **gRPC (Tonic)**, Named Pipes, Shared Memory         |
| **🔌 Interoperability**  | **Model Context Protocol (MCP)** support for universal LLM access | JSON-RPC, SSE (Server-Sent Events)                   |

### 🏗️ Engineering Highlights

#### 1. Robust Systems Programming (Rust)

- **Memory Safety**: Leveraged Rust's ownership model to ensure thread safety without garbage collection pauses.
- **Error Handling**: Comprehensive error propagation using `anyhow` and `thiserror` for resilient long-running processes.
- **Cross-Platform Abstractions**: Unified APIs for file system and process management across Windows, macOS, and Linux.

#### 2. Advanced GUI & Visualization

- **Real-Time Telemetry**: Implemented a Node.js backend (`gui/server.js`) pushing CPU/GPU/RAM metrics via WebSockets.
- **Interactive Dashboard**: Responsive React frontend allowing users to monitor agent status and intervene when necessary.
- **CLI Bridge**: Seamless integration to execute and pipe CLI commands directly from the web interface.

#### 3. Security Engineering

- **Windows Sandbox**: Engineered a custom sandbox using low-level Win32 APIs to restrict file system network access for AI-generated scripts.
- **Fine-Grained Permissions**: Implemented capability-based security ensuring agents operate with least privilege.

### 📦 Installation & Build

```bash
# Clone the repository
git clone https://github.com/zapabob/Codex.git
cd Codex

# Build Release Binaries (Rust)
cd codex-rs
cargo build --release

# Start GUI (Optional)
cd ../gui
npm install
node server.js
```

### 👨‍💻 Author / Developer

**Ryo Minegishi** - _Full Stack Engineer & AI Systems Architect_

Demonstrating expertise in:

- **Languages**: Rust, TypeScript, Python, C++
- **Cloud/Infra**: Docker, Kubernetes, GitHub Actions (CI/CD)
- **AI/ML**: LLM Orchestration, Prompt Engineering, RAG Pipelines

---

<a name="japanese"></a>

## 🇯🇵 日本語

### 🎯 プロジェクト概要

**Codex** は、OpenAIのCodexプロジェクトをベースに、**エンタープライズレベルのAI開発**に耐えうるよう再設計された高度なフォークプロジェクトです。

本プロジェクトは、システムプログラミング、分散アーキテクチャ、そしてAI統合における高度な技術力を実証するポートフォリオとして機能します。**マルチエージェント・アーキテクチャ**を採用し、ソフトウェア変更の計画・実行・検証を自律的かつセキュアに行います。

### 🚀 主な機能 (v2.13.0)

v2.13.0では、GUIとシステム統合サブシステムに大幅な強化が施されました。

| 機能ドメイン            | 機能詳細                                              | 技術スタック                                |
| :---------------------- | :---------------------------------------------------- | :------------------------------------------ |
| **🤖 自律エージェント** | 複雑なタスクを並列実行するサブエージェント群          | **Rust (Tokio)**, Async/Await, Actor Model  |
| **🛡️ セキュリティ**     | Windowsネイティブの隔離サンドボックス環境             | **Win32 API**, ACL制御, ジョブオブジェクト  |
| **🖥️ モダンGUI**        | リアルタイムメトリクスとCLI連携を備えたダッシュボード | **Next.js**, React, WebSocket, Tailwind CSS |
| **⚡ パフォーマンス**   | 高並行処理と低レイテンシIPC                           | **gRPC (Tonic)**, 名前付きパイプ            |
| **🔌 相互運用性**       | **Model Context Protocol (MCP)** による汎用LLM接続    | JSON-RPC, SSE                               |

### 🏗️ エンジニアリング・ハイライト

#### 1. 堅牢なシステムプログラミング (Rust)

- **メモリ安全性**: ガベージコレクションに頼らず、Rustの所有権モデルでスレッドセーフを確立。
- **エラー処理**: `anyhow` や `thiserror` を駆使した、長時間稼働に耐える堅牢なエラーハンドリング設計。

#### 2. 高度なGUIと可視化

- **リアルタイム・テレメトリ**: Node.jsバックエンドによるCPU/GPU/メモリ使用率のWebSocket配信。
- **インタラクティブ・ダッシュボード**: エージェントの状態監視と介入を可能にするReactフロントエンド。
- **CLIブリッジ**: Webインターフェースから直接CLIコマンドを実行・パイプする機能の実装。

#### 3. セキュリティエンジニアリング

- **Windowsサンドボックス**: Win32 APIを直接操作し、AI生成スクリプトのファイル・ネットワークアクセスを厳格に制限。
- **最小権限の原則**: ケーパビリティベースのセキュリティモデルによる権限管理。

### 📦 インストールとビルド

```bash
# リポジトリのクローン
git clone https://github.com/zapabob/Codex.git
cd Codex

# リリースビルド (Rust)
cd codex-rs
cargo build --release

# GUIの起動 (オプション)
cd ../gui
npm install
node server.js
```

### 👨‍💻 開発者 / Author

**Ryo Minegishi** - _Full Stack Engineer & AI Systems Architect_

スキルセット:

- **言語**: Rust, TypeScript, Python, C++
- **インフラ**: Docker, Kubernetes, GitHub Actions
- **AI/ML**: LLMオーケストレーション, RAGパイプライン, プロンプトエンジニアリング

---

<div align="center">

**Built with ❤️ and Rust**

[Issues](https://github.com/zapabob/Codex/issues) | [Releases](https://github.com/zapabob/Codex/releases) | [Documentation](./_docs/)

</div>
