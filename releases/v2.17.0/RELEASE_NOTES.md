## Codex v2.17.0 - zapabob Extended Edition

### What's New

#### Upstream OpenAI Features (merged from openai/codex)
- **MCP OAuth callback URL** - Configurable OAuth callback for MCP servers
- **Network proxy config** - `permissions.network proxy` configuration support
- **Reject approval policy** - Fine-grained rejection policies for agent actions
- **Configurable agent spawn depth** - Control sub-agent recursion depth
- **Configurable write_stdin timeout** - Tunable stdin write timeout
- **Shell snapshot failure reason** - Better error reporting for shell failures
- **Apps tool cache** - Disk-backed cache for faster startup
- **CVE-2026-24842 fix** - pnpm security update

#### zapabob Extensions
- **Multi-Agent System** - Planner/Assigner/Executor/Aggregator coordination
- **Deep Research** - DuckDuckGo/Gemini/Brave multi-source research engine
- **Slash Commands** - /VRChat, /Blender-CAD, /Yukkuri-Movie, /DeepResearch
- **VR/AR Integration** - WebXR-based Git visualization in 3D/VR
- **Supervisor System** - Agent lifecycle management with SQLite persistence
- **Git4D** - 4D time-aware Git visualization

#### GUI v2.17.0 (unified version)
- Version bump: 2.14.1 -> 2.17.0 (aligned with Rust workspace)
- 21-page Next.js app with MUI 7, Three.js, WebXR
- Playwright E2E tests: 20/24 passing (4 skipped: need Rust backend)
- CLI-GUI WebSocket bridge verified (Rust port 8787 <-> Next.js port 1919)

### Compiler Status
- **Rust**: 0 warnings, 0 errors (all compiler warnings eliminated)
- **GUI**: Build successful, 21 static pages generated

### Installation
See [README.md](https://github.com/zapabob/codex) for setup instructions.

### Requirements
- Rust 1.93.0+
- Node.js 20+
- OPENAI_API_KEY
