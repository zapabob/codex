# Changelog Archive — v2.x Release Line

This archive preserves the pre-v3 changelog entries that were previously published from the repository root.
For the current release line, see [`../../../CHANGELOG.md`](../../../CHANGELOG.md).

---

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.17.0] - 2026-02-20 - "Upstream Sync & API Refinements"

### 🚀 Major Features

**This release merges the latest upstream commits from openai/codex, incorporating new API changes, bug fixes, and security updates while preserving all custom zapabob extensions.**

### ✨ Added (from upstream)

- **MCP OAuth Support** - Enhanced MCP server authentication with OAuth flow
- **Permissions Proxy** - New permissions proxy layer for fine-grained access control
- **Auth Plan Support** - Session info now includes auth plan for better UX
- **AnimationEnabled API Change** - Frame scheduler replaces direct animations_enabled flag
- **Apps Tools Cache Context** - New cache context parameter for MCP client initialization
- **Model Catalog Integration** - ModelsManager and ThreadManager now accept optional model catalog

### 🔒 Security (from upstream)

- **CVE-2026-24842** - Addressed identified security vulnerability
- Dependency updates across Node.js ecosystem (pnpm audit fixes)

### 🔧 Fixed

- `find_model_info_for_slug` renamed to `model_info_from_slug` (API alignment)
- `CancelErr` struct pattern matching updated (was enum variant, now struct)
- `ToolRouter::from_config` app_tools parameter added
- `UnifiedExecProcessDetails` now includes `recent_chunks` field
- `spinner()` function signature simplified (removed `animations_enabled` parameter)
- History cell module structure aligned with upstream flat layout

### 🛠️ Custom Features Preserved (zapabob)

- Deep Research multi-source module (`codex-rs/deep-research/`)
- Supervisor agent lifecycle management (`codex-rs/supervisor/`)
- Remote image URLs support in UserMessage
- Git4D feature gates maintained
- Web search call animations support

---

## [2.16.0] - 2026-02-13 - "Upstream Sync & Security Hardening"

### 🚀 Major Features

**This release merges 226 upstream commits from openai/codex, incorporating the latest security fixes, bug fixes, and new features while preserving all custom zapabob extensions.**

### ✅ Added (from upstream)

- **Apps MCP Gateway** (`apps_mcp_gateway`) - New gateway for apps integration
- **Shell Tool MCP** - Patched zsh build pipeline for improved shell execution
- **Thread/List CWD** - Added `cwd` as optional field to thread/list API
- **Feature Flags Testing** - Verify enabled-by-default feature flags are stable
- **TurnContextItem Persistence** - Complete state persisted via canonical conversion
- **Approvals Scenarios** - More comprehensive approval workflow testing

### 🔒 Security (from upstream)

- **DNS Rebinding Fix** - Resolved DNS rebinding vulnerability in network proxy
- **Sandbox Bypass Fix** - Fixed sandbox bypass vulnerability
- **Exec Policy Path Confusion** - Resolved path confusion in exec policy
- **Case-Insensitivity Vulnerability** - Fixed case-insensitive matching exploit
- **Git Command Safety** - Removed git commands from dangerous command checks

### 🐛 Fixed (from upstream)

- **NUX Display** - Don't show NUX for upgrade-target models that are hidden
- **App Loading Logic** - Fixed app loading sequence
- **TUI Improvements** - Delta streaming, compaction events, approvals UI

### 📦 Preserved Custom Features (zapabob)

- `codex-gui-x/` - Custom GUI implementation
- `prism-mcp-server/`, `prism-web/` - Prism MCP integrations
- WebXR Git visualization with cyberpunk effects
- AI tool orchestration (task distribution, result integration)
- Bilingual README and GH Pages
- Fast build system and hot reload installation
- CI/CD customizations and release packaging

### 🔧 Technical Details

- **Merge Strategy**: `git merge upstream/main` with Python-automated conflict resolution
- **Conflicts Resolved**: 51 files (0 failures)
- **Upstream Commits Merged**: 226
- **Files Changed**: 594 files (+41,384 / -11,057 lines)
- **Module Restructuring**: `codex-rs/common/` merged into `codex-rs/utils/cli/`, hooks module restructured

### 📈 Dependencies

- pnpm 10.28.2, node >=22
- Various Cargo dependency updates (axum, clap, tokio, etc.)

---

## [2.9.0] - 2026-01-04 - "Fast Build & Hot Reload System"

### 🚀 Major Features

**This release introduces a complete build and deployment pipeline overhaul with hot reload capabilities and integrated release packaging.**

### ✅ Added

- **Fast Incremental Build System (`scripts/fast_build.py`)**
  - MD5 hash-based change detection for intelligent rebuilds
  - Cargo incremental compilation optimization
  - Parallel build processing with CPU core utilization
  - tqdm-powered progress visualization
  - Build cache persistence (`.build_cache.pkl`)

- **Hot Reload Installation System (`scripts/build_and_install.py`)**
  - Cross-platform process detection and termination (psutil)
  - Atomic binary replacement with safety checks
  - Platform-specific installation (Windows/macOS/Linux)
  - Installation verification with version checking
  - PowerShell integration for Windows deployment

- **Integrated Release Packaging**
  - GitHub Actions workflow for cross-platform tgz packages
  - All-platform binaries in single downloadable archive
  - Automated install script generation (`install.sh`)
  - Comprehensive release documentation (`INSTALL.md`)
  - Release notes with installation instructions

- **Development Tools Enhancement**
  - `just fast-build` - Quick incremental builds
  - `just build-install` - Full pipeline execution
  - `just install-kill` - Direct binary replacement
  - Process-safe deployment with zero-downtime updates

### 🎯 Performance Improvements

- **Build Speed**: Up to 70% faster incremental builds with change detection
- **Deployment Time**: Instant hot reload with process management
- **Release Size**: Optimized binaries with integrated packaging
- **Developer Experience**: One-command build and deploy workflow

### 🔧 Technical Details

- **Incremental Compilation**: Leverages Cargo's incremental features
- **Process Management**: Safe termination with psutil cross-platform support
- **Package Distribution**: Unified tgz format for all target platforms
- **Cache Strategy**: Persistent build state with intelligent invalidation

### 📦 Distribution

- **Release Archive**: Single `codex-2.9.0.tgz` containing all platform binaries
- **Installation**: `./install.sh` for automatic platform detection and setup
- **Verification**: Built-in version checking and integrity validation

## [2.8.3] - 2026-01-03 - "Build System Improvements & Repository Organization"

### 🎯 Interview-Ready Release

**This release transforms Codex from a personal project into enterprise-ready tooling with comprehensive documentation, benchmarks, and security hardening.**

### ✅ Added

- **Interview-Focused Documentation Suite**
  - `docs/plan/README.md` - 5-minute Plan Mode quickstart guide
  - `docs/benchmarks/` - Performance measurement methodology (Sub-agents: 2.6x speedup, CUDA: 3.7x speedup)
  - `examples/` - Production-ready sample projects (Node.js API, React Todo App)
  - `SECURITY.md` - Detailed sandbox architecture and audit logging

- **Real-World Examples**
  - `examples/node-api/` - REST API with Jest testing (96% quality score)
  - `examples/react-todo/` - React + TypeScript app with localStorage persistence
  - Sample projects demonstrate Codex's Plan Mode and Sub-agent orchestration

- **Benchmark Infrastructure**
  - Sub-agent performance measurement achieving 2.59x average speedup
  - CUDA acceleration benchmarks with 3.74x GPU speedup
  - Quality metrics: Type safety (100%), Code style (98.2%), Test coverage (96.7%)

- **Security Hardening**
  - Process isolation with read-only default sandbox
  - Structured audit logging with HMAC signatures
  - Approval gates for risky operations (shell, network, package install)

### 🔧 Changed

- **README.md** - Complete rewrite for interview-readiness
  - Removed "production-ready" claims, replaced with "stable/experimental" status
  - Added "Why Codex?" and "Safety model" sections
  - Feature matrix now links to real documentation paths
  - Status: CLI + Plan Mode + Sub-agents marked as **stable**

- **Documentation Structure**
  - Moved from scattered docs to organized structure
  - Added proof links for all feature claims
  - Included adoption-focused use cases

### 🐛 Fixed

- Build system compilation errors (22 fixed)
- Type safety improvements throughout codebase
- Repository organization (6,979 files systematically organized)

### 📈 Performance

- **Sub-agent Speedup**: 2.59x average across test cases
- **CUDA Acceleration**: 3.74x speedup on RTX 3080
- **Quality Maintenance**: 97.5% average quality score with parallel execution
- **Build Performance**: sccache integration for faster incremental builds

### 🔒 Security

- Default sandbox: read-only mode
- Explicit approval required for file writes, shell commands, network access
- Comprehensive audit logging with tamper-evident signatures
- Zero-day vulnerability count: 0 (v2.8.3)

## [2.8.0] - 2025-12-15 - "CUDA Acceleration & Quality Assurance"
