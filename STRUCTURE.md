# Repository Structure Guide / リポジトリ構造ガイド

**For AI tech recruiters**: This document provides a quick navigation guide to understand the codebase structure at a glance.

**AIテック企業採用担当者向け**: このドキュメントは、コードベース構造を一目で理解するためのクイックナビゲーションガイドです。

---

## 📁 Top-Level Organization / トップレベル構成

### Core Implementation / コア実装

| Directory | Purpose | Key Files | Status |
|-----------|---------|-----------|--------|
| `codex-rs/` | **Rust core** (CLI, TUI, orchestration, agents) | `core/src/plan/`, `core/src/agents/`, `core/src/qc/` | ✅ Production |
| `codex-cli/` | **Node.js CLI** (official OpenAI/codex compatible) | `bin/codex.js` | ✅ Production |
| `gui/` | **Web GUI** (Next.js, Plan management, visualization) | `src/app/`, `src/lib/xr/` | ✅ Production |
| `extensions/` | **IDE extensions** (VS Code, Chrome, etc.) | `vscode-codex/`, `codex-viz-web/` | ✅ Production |

### Documentation & Evidence / ドキュメントと証拠

| Directory | Purpose | Key Files | Status |
|-----------|---------|-----------|--------|
| `docs/` | **User guides, architecture, benchmarks** | `plan/README.md`, `benchmarks/`, `architecture/` | ✅ Complete |
| `portfolio/` | **Recruiter navigation** (see this first) | `README.md` (start here) | ✅ Complete |
| `_docs/` | **Implementation logs** (development history) | Daily logs with dates | ✅ Complete |
| `ARCHITECTURE.md` | **System architecture** (high-level design) | Complete system overview | ✅ Complete |

### Development Tools / 開発ツール

| Directory | Purpose | Key Files | Status |
|-----------|---------|-----------|--------|
| `scripts/` | **Build, test, deployment scripts** | `fast_build.py`, `build_and_install.py` | ✅ Active |
| `tools/` | **Development utilities** | QA integration, orchestrator | ✅ Active |
| `.codex/` | **Agent definitions** (YAML configs) | `agents/*.yaml`, `skills/` | ✅ Active |
| `.cursor/` | **Cursor IDE config** (MCP, rules) | `mcp.json`, `rules/` | ✅ Active |

### Examples & Testing / サンプルとテスト

| Directory | Purpose | Key Files | Status |
|-----------|---------|-----------|--------|
| `examples/` | **Production-ready samples** | `node-api/`, `react-todo/` | ✅ Complete |
| `gui-tests/` | **E2E tests** (Playwright) | `tests/gui-basic.spec.js` | ✅ Active |
| `tests/` | **Unit/integration tests** | Various test suites | ✅ Active |

### Specialized Components / 専門コンポーネント

| Directory | Purpose | Key Files | Status |
|-----------|---------|-----------|--------|
| `prism-web/` | **Prism web app** (separate product) | `app/`, `components/` | ✅ Complete |
| `prism-mcp-server/` | **Prism MCP integration** | `src/index.ts` | ✅ Complete |
| `shell-tool-mcp/` | **Shell tool MCP server** | `src/` | ✅ Complete |
| `sdk/` | **TypeScript SDK** | `typescript/` | ✅ Complete |
| `kernel-extensions/` | **Kernel modules** (Linux/Windows) | `linux/`, `windows/` | ✅ Experimental |

### Configuration & CI/CD / 設定とCI/CD

| Directory | Purpose | Key Files | Status |
|-----------|---------|-----------|--------|
| `.github/` | **GitHub Actions, templates** | `workflows/`, `ISSUE_TEMPLATE/` | ✅ Active |
| `.devcontainer/` | **Dev container config** | `devcontainer.json` | ✅ Active |
| `patches/` | **Dependency patches** | `*.patch` | ✅ Active |

---

## 🎯 Quick Navigation / クイックナビゲーション

### For Recruiters / 採用担当者向け

1. **Start here**: `portfolio/README.md` (5-minute overview)
2. **Architecture**: `ARCHITECTURE.md` (system design)
3. **Evidence**: `docs/benchmarks/` (performance proof)
4. **Examples**: `examples/README.md` (real-world demos)

### For Developers / 開発者向け

1. **Core code**: `codex-rs/` (Rust implementation)
2. **CLI**: `codex-cli/` (Node.js CLI)
3. **GUI**: `gui/` (Web interface)
4. **Scripts**: `scripts/` (build automation)

### For Contributors / コントリビュータ向け

1. **Contributing guide**: `CONTRIBUTING.md`
2. **Repository structure**: `.github/REPOSITORY_STRUCTURE.md`
3. **Agent definitions**: `.codex/agents/`
4. **Skills**: `.codex/skills/`

---

## 📊 File Count Summary / ファイル数サマリー

| Category | Count | Notes |
|----------|-------|-------|
| Rust source files | 1,139+ | `codex-rs/**/*.rs` |
| TypeScript/TSX | 100+ | `gui/`, `extensions/`, `prism-web/` |
| Documentation | 888+ | `docs/**/*.md` |
| Implementation logs | 15+ | `_docs/*.md` |
| Scripts | 150+ | `scripts/**/*.{ps1,py,sh}` |
| Tests | 200+ | Various test suites |

---

## 🔍 Finding Specific Features / 特定機能の探し方

### Plan Mode
- **Implementation**: `codex-rs/core/src/plan/`
- **Documentation**: `docs/plan/README.md`
- **CLI commands**: `codex-rs/cli/src/plan_commands.rs`

### Sub-Agents
- **Implementation**: `codex-rs/core/src/agents/`
- **Documentation**: `docs/agents/README.md`
- **Agent definitions**: `.codex/agents/*.yaml`

### QC Agent
- **Implementation**: `codex-rs/core/src/qc/`
- **Quantum optimization**: `codex-rs/core/src/qc/quantum.rs`
- **Mathematical optimization**: `codex-rs/core/src/qc/mathematical.rs`

### Deep Research
- **Implementation**: `codex-rs/deep-research/`
- **Documentation**: `docs/research/README.md`
- **MCP integration**: `codex-rs/deep-research/src/mcp_search_provider.rs`

### A2A Communication
- **Implementation**: `codex-rs/core/src/a2a_communication.rs`
- **Conflict prevention**: `codex-rs/core/src/orchestration/conflict_prevention.rs`

### Git Worktree
- **Implementation**: `codex-rs/core/src/orchestration/worktree_manager.rs`
- **Competition runner**: `codex-rs/core/src/orchestration/integrated_competition.rs`

### GUI & Visualization
- **Main GUI**: `gui/` (Next.js)
- **3D/4D Visualizer**: `extensions/codex-viz-web/`
- **Prism**: `prism-web/`

---

## 📚 Related Documents / 関連ドキュメント

- **Repository Structure**: `.github/REPOSITORY_STRUCTURE.md` (detailed)
- **Architecture**: `ARCHITECTURE.md` (system design)
- **Portfolio Guide**: `portfolio/README.md` (recruiter navigation)
- **Contributing**: `CONTRIBUTING.md` (how to contribute)

---

**Last Updated**: 2026-01-29  
**Maintained by**: [@zapabob](https://github.com/zapabob)
