# Repository Structure

Codex v2.17.0 — Enterprise AI Engineering Platform  
Last updated: 2026-02-20

## Top-Level Layout

```
codex-main/
│
├── codex-rs/           # 🦀 Rust workspace (69 crates) — Core platform
├── gui/                # ⚛️  Next.js 14 frontend GUI (port 1919)
├── codex-cli/          # 🖥️  Node.js CLI wrapper
├── codex-gui-x/        # 🌐 Extended GUI (Next.js 15 / React 19 / WebXR)
├── codex-supervisor/   # 🤖 Multi-agent supervisor (standalone)
│
├── docs/               # 📚 Technical documentation
├── _docs/              # 📝 Implementation logs (dev diary)
├── examples/           # 💡 Usage examples
├── extensions/         # 🔌 IDE extensions (VSCode, Windsurf, Codex-Viz)
│
├── mcp-servers/        # 🔧 MCP server implementations
├── prism-mcp-server/   # 🔭 Prism MCP integration server
├── shell-tool-mcp/     # 🐚 Shell tool MCP server
│
├── scripts/            # 🔨 Build & automation scripts
├── tools/              # 🛠️  Development tools
├── sdk/                # 📦 SDK (TypeScript client)
│
├── releases/           # 📦 Release archives (tar.gz, notes)
├── logs/               # 📊 Build & check logs
│   ├── build/          # Cargo/Next.js build outputs
│   ├── tui/            # TUI-specific logs
│   └── checks/         # Lint/audit outputs
├── archive/            # 🗃️  Archived files & temp scripts
│
├── portfolio/          # 🎯 Project portfolio documentation
├── social_announcements/ # 📣 Release announcements
├── qa-reports/         # ✅ QA test reports
│
├── .codex/             # 🤖 Codex agent definitions (YAML)
│   └── agents/         # Specialized agent configs
├── .cursor/            # 🖱️  Cursor IDE configuration
│   ├── skills/         # Agent skills (SKILL.md)
│   └── plans/          # Execution plans
│
├── README.md           # Project overview
├── ARCHITECTURE.md     # System architecture
├── STRUCTURE.md        # This file
├── CHANGELOG.md        # Version history
├── CONTRIBUTING.md     # Contribution guide
├── SECURITY.md         # Security policy
├── CLAUDE.md           # AI assistant guidance
└── AGENT.md            # Agent behavior spec
```

## Core Rust Workspace (`codex-rs/`)

The Rust workspace follows a layered architecture:

### Entry Points
| Crate | Binary | Purpose |
|-------|--------|---------|
| `cli` | `codex` | Main TUI/CLI entry point |
| `gui` | `codex-gui` | WebSocket backend for Next.js GUI |
| `mcp-server` | `codex-mcp-server` | MCP protocol server |

### Core Layer
| Crate | Purpose |
|-------|---------|
| `core` | Orchestration engine, tool routing, agent runtime |
| `backend-client` | OpenAI API HTTP client (SSE streaming) |
| `protocol` | Wire protocol types (CLI ↔ App Server) |
| `config` | Configuration types (`~/.codex/config.toml`) |
| `state` | SQLite-backed session persistence |

### UI Layer
| Crate | Purpose |
|-------|---------|
| `tui` | Terminal UI (Ratatui) with slash commands |
| `app-server` | WebSocket/SSE server for GUI |
| `app-server-protocol` | Schema definitions (v2 protocol) |

### Extended Features (zapabob)
| Crate | Purpose |
|-------|---------|
| `deep-research` | Multi-source research (DuckDuckGo/Gemini/Brave) |
| `supervisor` | Multi-agent supervisor (Planner/Assigner/Executor) |
| `otel` | OpenTelemetry tracing integration |

### Security/Platform
| Crate | Purpose |
|-------|---------|
| `windows-sandbox-rs` | Win32 Job Objects sandboxing |
| `linux-sandbox` | Landlock + seccomp-bpf |
| `process-hardening` | Cross-platform process hardening |
| `execpolicy` | Execution policy enforcement |

## GUI (`gui/`)

Next.js 14 application:

```
gui/
├── src/
│   ├── app/            # 21 pages (App Router)
│   │   ├── page.tsx    # Dashboard
│   │   ├── git4d/      # 4D Git visualization
│   │   ├── vr/         # VR/AR interface
│   │   ├── virtual-os/ # AI-powered virtual OS
│   │   ├── security/   # Security dashboard
│   │   ├── qc/         # Quality control
│   │   └── ...
│   ├── components/     # 60+ React components
│   │   ├── visualization/ # Three.js / WebXR
│   │   ├── ai-tools/   # AI orchestration UI
│   │   ├── virtual-os/ # Virtual OS components
│   │   └── ...
│   ├── lib/
│   │   ├── bridge/     # CLI-GUI bridge (WebSocket)
│   │   └── api/        # API client (JSON-RPC)
│   └── store/          # Zustand state management
└── tests/              # Playwright E2E tests
```

## Agent Definitions (`.codex/agents/`)

Specialized YAML-defined agents:

| Agent | Capability |
|-------|-----------|
| `code-reviewer` | Automated code review |
| `test-gen` | Test suite generation |
| `sec-audit` | Security vulnerability audit |
| `researcher` | Deep research and synthesis |
| `vrchat-dev` | VRChat SDK3 development |
| `blender-cad` | Blender Python automation |
| `yukkuri-movie` | YMM4/VOICEVOX automation |
| `web-search-deepresearch` | Multi-source web research |

## Documentation (`docs/`)

```
docs/
├── README.md           # Documentation index
├── agents/             # Agent system documentation
├── gui/                # GUI setup and API
├── plan/               # Planning system
├── research/           # Deep research docs
├── vr/                 # VR/AR documentation
├── git/                # Git integration
├── benchmarks/         # Performance benchmarks
└── zapabob/            # zapabob extension docs
```
