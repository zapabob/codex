# Codex-RS: The Next-Gen AI Coding Agent / 次世代AIコーディングエージェント

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tokio](https://img.shields.io/badge/async-Tokio-green.svg)](https://tokio.rs/)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui-yellow.svg)](https://ratatui.rs/)

> A high-performance, modular AI coding assistant designed for speed, safety, and extensibility.
>
> 高速性、安全性、拡張性を追求した、Rust製ハイパフォーマンスAIコーディングアシスタント。

---

## 🚀 Vision / ビジョン

Codex-RS redefines the AI coding experience by moving away from heavy, dependency-laden environments to a **single, native binary**. Built on Rust, it offers unparalleled performance, memory safety, and a robust architecture capable of handling complex engineering tasks.

Codex-RSは、依存関係の重い環境から脱却し、**単一のネイティブバイナリ**としてAIコーディング体験を再定義します。Rustで構築されており、圧倒的なパフォーマンス、メモリ安全性、そして複雑なエンジニアリングタスクを処理できる堅牢なアーキテクチャを提供します。

---

## ✨ Unique Features / 独自機能と特徴

### 1. Dual Interface: CLI & TUI

**Professional-grade tools for every workflow.**

- **Headless CLI**: Automate tasks via `codex exec`. Pipe inputs, script workflows, and integrate into CI/CD pipelines.
- **Immersive TUI**: A rich, terminal-based user interface built with **Ratatui**. Features syntax highlighting, multi-tab management, and real-time feedback without leaving your keyboard.

**あらゆるワークフローに対応するプロフェッショナルツール**

- **ヘッドレスCLI**: `codex exec`によるタスク自動化。入力のパイプ処理、ワークフローのスクリプト化、CI/CDパイプラインへの統合が可能です。
- **没入型TUI**: **Ratatui**で構築されたリッチな端末ユーザーインターフェース。シンタックスハイライト、マルチタブ管理、リアルタイムフィードバックを、キーボードから手を離すことなく提供します。

### 2. Model Context Protocol (MCP) Native

**Seamless ecosystem integration.**

- **Client & Server**: Codex is not just a tool; it's a platform. It functions as both an MCP Client (connecting to external tools) and an MCP Server (serving its capabilities to other agents).
- **Chrome & Git Integration**: specialized bridges for browser automation and version control operations.

**シームレスなエコシステム統合**

- **クライアント & サーバー**: Codexは単なるツールではなく、プラットフォームです。MCPクライアント（外部ツールへの接続）としても、MCPサーバー（他のエージェントへの機能提供）としても機能します。
- **Chrome & Git統合**: ブラウザ自動化やバージョン管理操作のための専用ブリッジを搭載。

### 3. Granular Sandboxing / 堅牢なサンドボックス

**Safety first execution.**

- Supports strict execution policies on macOS (Seatbelt), Linux (Landlock), and Windows.
- Configurable modes: `read-only`, `workspace-write`, and `danger-full-access`.

**安全第一の実行環境**

- macOS (Seatbelt)、Linux (Landlock)、Windowsでの厳格な実行ポリシーをサポート。
- `read-only`、`workspace-write`、`danger-full-access`など、設定可能なモードを提供。

### 4. Advanced DevOps Integration / 高度なDevOps統合

**Built for speed.**

- **sccache Support**: Optimized for 6-core parallel builds, drastically reducing compilation time.
- **Atomic Updates**: Smart installation scripts (`fast_build.ps1`) ensuring zero-downtime updates during development.

**スピードを追求**

- **sccacheサポート**: 6コア並列ビルドに最適化されており、コンパイル時間を大幅に短縮。
- **アトミック更新**: スマートなインストールスクリプト（`fast_build.ps1`）により、開発中のゼロダウンタイム更新を実現。

---

## 🏗️ Architecture / アーキテクチャ

The project follows a clean, modular workspace structure:
プロジェクトは、クリーンでモジュール化されたワークスペース構造を採用しています:

| Module / モジュール | Description / 説明                                                                                                                                                        |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`core/`**         | The brain of Codex. Contains business logic, LLM integration, and context management. <br> Codexの頭脳。ビジネスロジック、LLM統合、コンテキスト管理を含みます。           |
| **`tui/`**          | The frontend. A sophisticated terminal UI driven by modern async Rust patterns. <br> フロントエンド。最新の非同期Rustパターンで駆動する洗練されたターミナルUI。           |
| **`cli/`**          | The entry point. Handles argument parsing, subcommands, and tool orchestration. <br> エントリーポイント。引数解析、サブコマンド、ツールオーケストレーションを処理します。 |
| **`exec/`**         | The automation engine. Runs non-interactive tasks and scripts. <br> 自動化エンジン。非対話型タスクやスクリプトを実行します。                                              |
| **`mcp-*/`**        | Modular bridges for the Model Context Protocol ecosystem. <br> Model Context Protocolエコシステムのためのモジュール式ブリッジ。                                           |

---

## 🛠️ Getting Started / クイックスタート

### Prerequisites / 前提条件

- Rust (latest stable)
- Node.js (for some optional integrations)
- `sccache` (recommended for fast builds)

### Fast Build & Install / 高速ビルドとインストール

For developers who value speed, utilize our optimized PowerShell workflow:
スピードを重視する開発者向けに、最適化されたPowerShellワークフローを提供しています:

```powershell
# Optimized for 6-core parallel processing
# 6コア並列処理に最適化
.\fast_build.ps1
```

### Standard Usage / 基本的な使い方

```bash
# Start the TUI / TUIを起動
codex

# Run a quick command / クイックコマンド実行
codex exec "Analyze this project structure"

# Start as MCP Server / MCPサーバーとして起動
codex mcp-server
```

---

## 📄 Documentation / ドキュメント

- **[Getting Started / 入門](../docs/getting-started.md)**: First-time setup and walkthrough.
- **[Configuration / 設定](../docs/config.md)**: Deep dive into `config.toml`.
- **[Architecture Guide / 設計ガイド](../docs/architecture.md)**: Internal design details for contributors.

---

> **Note to Recruiters / 採用担当者様へ**
>
> This project demonstrates mastery of:
>
> - **System Programming**: Low-level resource management and cross-platform compatibility.
> - **Async Concurrency**: Complex Tokio runtimes and actor-like patterns.
> - **Modern AI/LLM Application Design**: RAG, tool use, and agentic workflows.
> - **Production-Grade Engineering**: CI/CD, testing, and documentation standards.
>
> 本プロジェクトは、以下の領域における高い技術力を示しています:
>
> - **システムプログラミング**: 低レイヤーのリソース管理とクロスプラットフォーム互換性。
> - **非同期並行処理**: 複雑なTokioランタイムとアクターモデル的パターン。
> - **最新AI/LLMアプリケーション設計**: RAG、ツール利用、エージェンティックワークフロー。
> - **プロダクショングレードのエンジニアリング**: CI/CD、テスト、ドキュメンテーション標準。

---

_Built with ❤️ in Rust._
