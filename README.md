# Codex — Enterprise AI Engineering Platform

<div align="center">

[![Version](https://img.shields.io/badge/version-v2.17.0-blue)](https://github.com/zapabob/codex/releases/tag/v2.17.0)
[![Rust](https://img.shields.io/badge/rust-1.93.0%20%7C%202024%20edition-orange)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5.0-blue)](https://www.typescriptlang.org/)
[![Next.js](https://img.shields.io/badge/next.js-14-black)](https://nextjs.org/)
[![CUDA](https://img.shields.io/badge/CUDA-12.0%2B-76B900?logo=nvidia)](./docs/cuda/)
[![MCP](https://img.shields.io/badge/MCP-1.0-blueviolet)](./docs/mcp/)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](./LICENSE)
[![CI](https://img.shields.io/badge/CI-passing-success)](https://github.com/zapabob/codex/actions)

**Production-grade autonomous AI coding assistant with multi-agent orchestration, WebXR visualization, and zero-trust sandboxing.**

[🇺🇸 English](#-english) | [🇯🇵 日本語](#-japanese) | [📖 Docs](./docs/) | [🚀 Quick Start](#quick-start) | [🔖 Releases](https://github.com/zapabob/codex/releases)

</div>

---

## 🇺🇸 English

### What is Codex?

Codex is a **next-generation AI engineering platform** forked from [OpenAI/codex](https://github.com/openai/codex) and extended with enterprise features. It turns AI from a simple chat assistant into a **self-organizing, multi-agent development environment** capable of autonomously designing, coding, reviewing, and testing software.

> Built in Rust 2024 for maximum performance and safety. Ships with a full-featured Next.js GUI, WebSocket CLI bridge, VR/AR Git visualization, and a pluggable MCP (Model Context Protocol) server ecosystem.

---

### Why Codex? (Recruiter TL;DR)

| Dimension | What We Ship |
|-----------|-------------|
| **Architecture** | 69-crate Rust workspace · Zero-copy async · Tokio runtime |
| **AI/ML** | OpenAI Responses API · Multi-agent swarms · CUDA-accelerated search |
| **Security** | Zero-trust sandbox (Win32 Job Objects / Linux Landlock / macOS Seatbelt) |
| **Frontend** | Next.js 14 · MUI 7 · Three.js WebXR · Real-time WebSocket |
| **DevOps** | GitHub Actions CI · Bazel · cargo-nextest · Playwright E2E |
| **Research** | Deep-research engine (DuckDuckGo + Gemini + Brave multi-source) |

---

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    User Interface Layer                          │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────────┐ │
│  │  TUI (Ratatui)│  │ Next.js GUI  │  │ MCP Client / VSCode   │ │
│  │  Slash cmds  │  │ Port 1919    │  │ Extension             │ │
│  └──────┬───────┘  └──────┬───────┘  └─────────┬─────────────┘ │
│         │ stdin/stdout    │ WebSocket           │ JSON-RPC      │
└─────────┼─────────────────┼─────────────────────┼───────────────┘
          ▼                 ▼                     ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Core Runtime (Rust)                           │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────────────────┐  │
│  │ App Server  │  │ Core Engine  │  │  MCP Server            │  │
│  │ Port 8787   │  │ (codex-core) │  │  (codex-mcp-server)    │  │
│  └──────┬──────┘  └──────┬───────┘  └─────────┬──────────────┘  │
│         │                │                     │                 │
│  ┌──────▼──────────────────────────────────────▼──────────────┐  │
│  │              Orchestration & Agent Runtime                  │  │
│  │  Planner → Assigner → Executor (parallel) → Aggregator     │  │
│  └──────────────────────────┬────────────────────────────────┘  │
└─────────────────────────────┼───────────────────────────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Execution Layer                               │
│  ┌───────────────┐  ┌──────────────┐  ┌───────────────────────┐ │
│  │ Sandboxed Exec │  │ Tool Routing │  │ OpenAI Backend Client │ │
│  │ (platform ACL) │  │ (shell/file) │  │ (SSE streaming)       │ │
│  └───────────────┘  └──────────────┘  └───────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

### Key Features

#### 🔒 Zero-Trust Security Sandbox
Three-tier sandbox system with platform-native isolation:
- **Windows**: Win32 Job Objects + restricted token ACLs
- **Linux**: Landlock LSM + seccomp-bpf syscall filtering
- **macOS**: Seatbelt (sandbox-exec) profiles
- Three modes: `read-only` (default) · `workspace-write` · `danger-full-access`

#### 🤖 Multi-Agent Orchestration (zapabob Extension)
```
codex delegate-parallel --agents backend,qa,frontend --task "..."
```
- **Planner** decomposes tasks into subtasks
- **Assigner** routes to specialized sub-agents
- **Executor** runs agents in parallel with Git worktrees
- **Aggregator** merges results with conflict detection

#### 🔬 Deep Research Engine
Multi-source intelligent research with:
- DuckDuckGo + Gemini + Brave Search APIs
- Citation tracking and fact verification
- Recursive depth-first exploration
- Academic paper synthesis (arXiv, Scholar)

#### 🎮 VR/AR Git Visualization (Git4D)
- **Three.js + WebXR** immersive repository visualization
- Commit graph rendered in 3D/4D space
- Meta Quest 2/3 hand tracking support
- Cyberpunk-themed shader effects

#### 🛠️ Slash Commands (TUI)
| Command | Agent | Capability |
|---------|-------|-----------|
| `/VRChat` | vrchat-dev | Unity C#, Udon, Modular Avatar, liltoon |
| `/Blender-CAD` | blender-cad | STEP/IGES import, Geometry Nodes, RTX |
| `/Yukkuri-Movie` | yukkuri-movie | YMM4, VOICEVOX, video automation |
| `/DeepResearch` | web-search-deepresearch | Multi-source research |

#### 🧩 MCP (Model Context Protocol) Ecosystem
- Built-in MCP server for tool exposure
- Dynamic MCP loader (load servers at runtime)
- OAuth callback URL support (v2.17.0 new)
- Configurable approval policies per tool

---

### What's New in v2.17.0

**Upstream OpenAI sync (163 commits):**
- MCP OAuth callback URL configuration
- Network proxy support (`permissions.network.proxy`)
- Fine-grained rejection approval policies
- Configurable agent spawn depth limit
- Disk-backed apps tool cache (faster startup)
- Security: CVE-2026-24842 pnpm fix

**zapabob Extensions:**
- GUI version unified: 2.14.1 → 2.17.0
- Playwright E2E: 20/24 tests passing
- 0 Rust compiler warnings (clean build)
- VRChat/Blender/YMM4 slash commands
- Enhanced skill system (YAML agent definitions)

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

#### Run CLI (Interactive TUI)
```bash
cd codex-rs
cargo run --bin codex
```

#### Run GUI
```bash
cd gui
npm install
npm run dev        # http://localhost:1919
# In another terminal:
cargo run --bin codex-gui  # WebSocket backend on :8787
```

#### Autonomous Agent Mode
```bash
# Single agent exec
codex exec "Add unit tests for the auth module"

# Multi-agent parallel delegation
codex delegate-parallel \
  --agents "code-reviewer,test-gen,sec-audit" \
  --task "Review PR #42 comprehensively"
```

#### Build from Source
```powershell
# Windows fast incremental build
cd codex-rs
.\ultra-fast-build-install.ps1
```

```bash
# Linux/macOS
cd codex-rs
cargo install --path cli
```

---

### Technology Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| **Core Language** | Rust | 1.93.0 (2024 edition) |
| **Async Runtime** | Tokio | 1.42+ |
| **TUI Framework** | Ratatui | latest |
| **AI Protocol** | OpenAI Responses API | v2 |
| **Tool Protocol** | MCP (Model Context Protocol) | 1.0 |
| **Frontend** | Next.js + React | 14 + latest |
| **UI Components** | MUI (Material UI) | 7 |
| **3D Graphics** | Three.js + @react-three | 0.159+ |
| **WebXR** | @react-three/xr | 6.2 |
| **Testing** | cargo-nextest + Playwright | latest |
| **Build** | Cargo + Bazel | workspace |
| **GPU** | CUDA | 12.0+ (RTX 3080) |

---

### Repository Structure

```
codex-main/
├── codex-rs/           # Rust workspace (69 crates)
│   ├── cli/            # Binary entry point
│   ├── core/           # Orchestration engine
│   ├── tui/            # Terminal UI (Ratatui)
│   ├── app-server/     # WebSocket server for GUI
│   ├── mcp-server/     # MCP server implementation
│   ├── deep-research/  # Multi-source research engine
│   ├── supervisor/     # Multi-agent supervisor
│   └── gui/            # Rust GUI backend (port 8787)
├── gui/                # Next.js frontend (port 1919)
│   ├── src/app/        # 21 pages (dashboard, VR, tasks, etc.)
│   ├── src/components/ # 60+ React components
│   └── tests/          # Playwright E2E tests
├── codex-cli/          # Node.js CLI wrapper
├── .codex/agents/      # YAML agent definitions
├── .cursor/skills/     # Cursor IDE skill definitions
├── docs/               # Technical documentation
└── _docs/              # Implementation logs
```

---

### Performance

- **Build time**: ~3min incremental (6-core sccache)
- **Cold start**: <2s (disk-cached apps tool)
- **Agent spawn**: <100ms per sub-agent
- **CUDA search**: 3.7x faster than CPU baseline
- **GUI build**: 21 pages in ~3.5min

---

### CI/CD

```yaml
# GitHub Actions matrix:
- Rust: linux-x64, macos-arm64, windows-x64
- Checks: clippy (0 warnings), fmt, nextest, cargo-deny
- Frontend: Playwright E2E (chromium)
- Security: cargo audit, pnpm audit
```

---

## 🇯🇵 日本語

### Codexとは

Codexは[OpenAI/codex](https://github.com/openai/codex)をフォークし、エンタープライズ向け機能を大幅に拡張した**次世代AIエンジニアリングプラットフォーム**です。AIを単なるチャットアシスタントから、コード設計・実装・レビュー・テストを自律的に行う**マルチエージェント開発環境**へと進化させます。

### アーキテクチャ特徴

- **Rust 2024エディション**：メモリ安全性と超高性能並行処理
- **ゼロトラストサンドボックス**：Win32 Job Objects / Linux Landlock / macOS Seatbelt
- **マルチエージェントオーケストレーション**：Planner→Assigner→Executor→Aggregator
- **ディープリサーチエンジン**：複数情報源からの自律的調査
- **WebXR Gitビジュアライゼーション**：3D/VRでリポジトリを体験
- **スラッシュコマンド**：/VRChat、/Blender-CAD、/Yukkuri-Movie等

### バージョン v2.17.0 の変更点

**公式OpenAIからの取り込み（163コミット）:**
- MCP OAuth コールバックURL設定対応
- ネットワークプロキシ設定（`permissions.network.proxy`）
- 拒否承認ポリシーの細粒度化
- エージェント生成深度の設定可能化
- ディスクキャッシュによる起動高速化
- セキュリティパッチ: CVE-2026-24842 (pnpm)

**zapabob独自機能:**
- GUIバージョン統一: 2.14.1 → 2.17.0
- Playwright E2Eテスト: 20/24 パス
- Rustコンパイラ警告ゼロを達成
- TUIスラッシュコマンド強化
- YAML定義エージェントシステム

### セットアップ

```bash
# 環境変数設定
export OPENAI_API_KEY="sk-..."

# CLI実行
cd codex-rs && cargo run --bin codex

# GUI起動
cd gui && npm install && npm run dev
# WebSocketバックエンド: cargo run --bin codex-gui
```

### サンドボックスモード

| モード | 説明 | 用途 |
|--------|------|------|
| `read-only` | ファイル読み取りのみ（デフォルト） | 安全な分析 |
| `workspace-write` | ワークスペース内書き込み | 通常開発 |
| `danger-full-access` | フルアクセス（明示的許可必要） | 統合テスト |

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

- Issues: [GitHub Issues](https://github.com/zapabob/codex/issues)
- Security: See [SECURITY.md](./SECURITY.md)

## License

Apache 2.0 — See [LICENSE](./LICENSE) for details.

---

<div align="center">

Built with ❤️ by zapabob | Powered by OpenAI Codex Platform

**[⬆ Back to top](#codex--enterprise-ai-engineering-platform)**

</div>
