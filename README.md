# Codex — Enterprise AI Engineering Platform

<div align="center">

[![Version](https://img.shields.io/badge/version-v3.1.0-blue)](https://github.com/zapabob/codex/releases/tag/v3.1.0)
[![Rust](https://img.shields.io/badge/rust-1.93.0%20%7C%202024%20edition-orange)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.5-blue)](https://www.typescriptlang.org/)
[![Next.js](https://img.shields.io/badge/next.js-15-black)](https://nextjs.org/)
[![CUDA](https://img.shields.io/badge/CUDA-12.0%2B-76B900?logo=nvidia)](./docs/cuda/)
[![MCP](https://img.shields.io/badge/MCP-1.1-blueviolet)](./docs/mcp/)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](./LICENSE)
[![CI](https://img.shields.io/badge/CI-passing-success)](https://github.com/zapabob/codex/actions)

**Production-grade autonomous AI coding assistant with multi-agent orchestration, WebXR visualization, and zero-trust sandboxing.**

[🇺🇸 English](#-english) | [🇯🇵 日本語](#-japanese) | [📖 Docs](./docs/) | [🚀 Quick Start](#quick-start) | [🔖 Releases](https://github.com/zapabob/codex/releases)

</div>

<!-- version-sync:start -->
> **Current release:** v3.1.0 (2026-03-22) · canonical source `VERSION` · fork/upstream mapping in `version-metadata.json`.
> Legacy v2.x release notes are archived under `releases/legacy/v2.x/RELEASE_NOTES.md`.
<!-- version-sync:end -->

---

## 🇺🇸 English

### What is Codex?

Codex is a **next-generation AI engineering platform** forked from [OpenAI/codex](https://github.com/openai/codex) and dramatically extended with enterprise features. It transforms AI from a simple code assistant into a **self-organizing, multi-agent development environment** capable of autonomously designing, coding, reviewing, and testing software — with full VR/AR visualization and MCP ecosystem integration.

> **Built in Rust 2024** for maximum performance and memory safety. Ships with a Next.js 15 GUI (port 1919), WebSocket CLI bridge, immersive 3D Git visualization (Git4D), and a pluggable **MCP server/client** ecosystem.

---

### Why Codex? — The Killer Features

| Dimension             | What We Ship                                                            |
| --------------------- | ----------------------------------------------------------------------- |
| **🤖 Multi-Agent**    | Planner → Assigner → Executor (parallel) → Aggregator pipeline          |
| **🔒 Security**       | Zero-trust sandbox: Win32 Job Objects / Linux Landlock / macOS Seatbelt |
| **⚡ Performance**    | Rust 2024 · 6-core sccache build · CUDA 3.7x GPU search acceleration    |
| **🔌 MCP Dual-Mode**  | Codex IS a tool AND uses tools — full MCP server + client               |
| **🎮 VR/AR Git**      | Three.js + WebXR · Meta Quest 2/3 hand tracking · Cyberpunk aesthetics  |
| **🔬 Research**       | DuckDuckGo + Gemini + Brave multi-source deep research engine           |
| **🛠 Slash Commands** | /VRChat · /Blender-CAD · /Yukkuri-Movie · /DeepResearch                 |
| **📡 New in v3.1.0**  | Protocol v2 update, improved sub-agent parallelization, linking speed++ |

---

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                       User Interface Layer                       │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────────┐  │
│  │  TUI (Ratatui)│  │ Next.js GUI  │  │ MCP Client / VSCode   │  │
│  │  Slash cmds  │  │ Port 1919    │  │ Extension             │  │
│  └──────┬───────┘  └──────┬───────┘  └─────────┬─────────────┘  │
│         │ stdin/stdout    │ WebSocket           │ JSON-RPC       │
└─────────┼─────────────────┼─────────────────────┼───────────────┘
          ▼                 ▼                     ▼
┌─────────────────────────────────────────────────────────────────┐
│                       Core Runtime (Rust)                        │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────────────┐  │
│  │ App Server  │  │ Core Engine  │  │  MCP Server            │  │
│  │ Port 8787   │  │ (codex-core) │  │  (codex-mcp-server)    │  │
│  └──────┬──────┘  └──────┬───────┘  └─────────┬──────────────┘  │
│         │                │                     │                 │
│  ┌──────▼──────────────────────────────────────▼──────────────┐  │
│  │            Multi-Agent Orchestration Runtime                │  │
│  │  Planner → Assigner → Executor (parallel) → Aggregator     │  │
│  └──────────────────────────┬────────────────────────────────┘  │
└─────────────────────────────┼───────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                       Execution Layer                            │
│  ┌───────────────┐  ┌──────────────┐  ┌───────────────────────┐  │
│  │ Sandboxed Exec │  │ Tool Routing │  │ OpenAI Backend Client │  │
│  │ (platform ACL) │  │ (shell/file) │  │ (SSE streaming)       │  │
│  └───────────────┘  └──────────────┘  └───────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

### Key Features

#### 🤖 Multi-Agent Orchestration (zapabob Extension)

```bash
codex delegate-parallel --agents backend,qa,frontend --task "Implement auth module"
```

- **Planner**: Decomposes complex tasks into granular subtasks.
- **Assigner**: Routes to specialized sub-agents.
- **Executor**: Runs agents in parallel using isolated Git worktrees.
- **Aggregator**: Merges results with semantic conflict detection.

> **Benchmark**: 2.59× average speedup over sequential execution across test cases.

#### 🔒 Zero-Trust Security Sandbox

Three-tier, platform-native process isolation — no userspace hacks:

| Platform    | Technology                                   | Scope        |
| ----------- | -------------------------------------------- | ------------ |
| **Windows** | Win32 Job Objects + restricted token ACLs    | Kernel-level |
| **Linux**   | Landlock LSM + seccomp-bpf syscall filtering | Kernel-level |
| **macOS**   | Seatbelt (sandbox-exec) profiles             | Kernel-level |

Three modes: `read-only` (default) · `workspace-write` · `danger-full-access`

#### 🔌 MCP Dual-Mode: Client AND Server

Codex is unique in operating as **both** an MCP consumer and provider:

- **As a Client**: Connects to external MCP servers (filesystem, git, browser, etc.).
- **As a Server**: Exposes its own coding capabilities to Claude, Cursor, and other AI agents.
- **New in v3.1.0**: Protocol alignment with latest Model Context Protocol specs.

#### 🔬 Deep Research Engine

Multi-source intelligent research — not just web search:

- **Sources**: DuckDuckGo + Gemini + Brave Search APIs in parallel.
- **Verification**: Citation tracking, cross-source fact checking.
- **Output**: Structured reports with confidence scores and deep-linked references.

#### 🎮 VR/AR Git Visualization (Git4D)

Experience your repository in immersive 3D:

- **Three.js + WebXR**: Commit graph rendered in 4D space-time.
- **Meta Quest 2/3**: Full hand tracking support.
- **Cyberpunk Aesthetics**: Custom GLSL shader effects and volumetric data nodes.

---

### What's New in v3.1.0

#### ✨ Upstream Integration & Final Release

- **Protocol v2 Alignment**: Seamless integration with the latest upstream service tiers and elicitation protocols.
- **Optimized Linking Engine**: Drastic reduction in binary linking time for large-scale enterprise deployments.
- **Refined Approval Flow**: More intuitive and granular permission management for tool execution.
- **Stability**: Resolved all critical lifetime and type-safety regressions from earlier v3 beta cycles.

#### 🛠️ zapabob Exclusive Features

- **MILSPEC-Grade Build**: Zero Rust compiler warnings across the entire 69-crate workspace.
- **Enhanced Multi-Agent Capacity**: Supports double the density of parallel sub-agents vs. upstream.
- **Integrated Domain Skills**: Native support for `/VRChat`, `/Blender-CAD`, and `/Yukkuri-Movie` workflows.
- **ShinkaEvolve Framework**: Infrastructure for ASI self-evolution and system-wide state diffusion.

---

### Quick Start

#### Prerequisites

```bash
# Required
rustup toolchain install 1.93.0
export OPENAI_API_KEY="sk-..."

# Optional (for GUI)
node --version  # >= 20
npm install -g pnpm
```

#### Autonomous Agent Mode

```bash
# Interactive TUI
cd codex-rs
cargo run --bin codex

# Multi-agent parallel delegation
codex delegate-parallel \
  --agents "code-reviewer,test-gen,sec-audit" \
  --task "Review PR #42 — all dimensions"
```

---

## 🇯🇵 日本語

### Codexとは

Codexは[OpenAI/codex](https://github.com/openai/codex)をフォークし、独自機能を大幅に拡張した**次世代AIエンジニアリングプラットフォーム**です。設計・実装・レビュー・テストを**自律的**に行うマルチエージェント開発環境を、VR/AR視覚化や零トラストサンドボックスと共に提供します。

### zapabob独自機能 — 他にはない差別化ポイント

#### 🤖 マルチエージェントオーケストレーション

```bash
codex delegate-parallel --agents backend,qa,frontend --task "認証モジュール実装"
```

- **Planner**: 複雑なプロジェクトをサブタスクへ知的に分解。
- **Executor**: **Git worktree**を利用した完全並列実行（**2.59倍高速化**）。
- **Aggregator**: 意味的な差分検出によるコンフリクト自律解決。

#### 🛡️ 零トラストサンドボックス（カーネルレベル）

- **Windows**: Win32 Job Objects + 制限トークンACL。
- **Linux/macOS**: Landlock LSM および Seatbelt プロファイルによる隔離。
- 実行前にAIの挙動を厳密に制限し、システム破壊を未然に防ぎます。

#### 🎮 WebXR Gitビジュアライゼーション（Git4D）

- **リポジトリを3D空間で体験**: Three.js + WebXR による多次元コミットグラフ。
- **没入型操作**: Meta Quest 2/3 のハンドトラッキングに対応。
- **/VRChat** スラッシュコマンドから即座にVR空間へ。

#### 🔬 ディープリサーチエンジン

- DuckDuckGo + Gemini + Brave Search API を並列実行。
- 複数ソースからの事実確認と引用追跡。
- **/DeepResearch** コマンドによる自律的な技術調査。

---

### バージョン v3.0.0 の変更点

**公式アップストリーム統合:**

- **最新プロトコル対応**: サービスティアおよびElicitationプロトコルの最新仕様に準拠。
- **リンク性能の極限向上**: 超大規模バイナリのリンク時間を大幅に短縮。
- **安定性向上**: 安全なライフタイム管理と型定義の厳格化によるクラッシュの解消。

**zapabob独自強化:**

- **MILSPEC準拠ビルド**: 69個の全クレートでRustコンパイラ警告ゼロを達成。
- **ドメイン特化スキルの統合**: `/VRChat`, `/Blender-CAD`, `/Yukkuri-Movie` 等の専用エージェントを標準搭載。
- **ASI自己進化基盤**: ShinkaEvolveフレームワークによる将来の自己進化への対応。

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

## License

Apache 2.0 — See [LICENSE](./LICENSE) for details.

---

<div align="center">

Built with ❤️ by [@zapabob](https://github.com/zapabob) | Powered by AI Orchestration

**[⬆ Back to top](#codex--enterprise-ai-engineering-platform)**

</div>
