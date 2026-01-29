# Technical Architecture | 技術アーキテクチャ

## Overview | 概要

This document provides a deep dive into the technical architecture of the Codex project, designed for technical reviewers and recruiters evaluating software engineering capabilities.

このドキュメントは、ソフトウェアエンジニアリング能力を評価する技術レビュアーおよび採用担当者向けに、Codexプロジェクトの技術アーキテクチャを詳細に説明します。

---

## System Architecture | システムアーキテクチャ

```
┌─────────────────────────────────────────────────────────────────┐
│                      User Interface Layer                        │
├─────────────────┬─────────────────┬─────────────────────────────┤
│    CLI (Rust)   │   TUI (Ratatui) │    GUI (Electron/React)     │
└────────┬────────┴────────┬────────┴──────────────┬──────────────┘
         │                 │                       │
         └────────────────┬┴───────────────────────┘
                         │
         ┌───────────────▼───────────────┐
         │     Core Orchestration Engine  │
         │         (codex-core)           │
         ├────────────────────────────────┤
         │ • Parallel Executor            │
         │ • Worktree Manager             │
         │ • A2A Communication            │
         │ • QC Evaluator                 │
         └───────────────┬───────────────┘
                         │
    ┌────────────────────┼────────────────────┐
    │                    │                    │
    ▼                    ▼                    ▼
┌────────┐        ┌────────────┐      ┌─────────────┐
│Security│        │   Model    │      │    MCP      │
│Sandbox │        │   Router   │      │   Server    │
└────┬───┘        └─────┬──────┘      └──────┬──────┘
     │                  │                    │
     ▼                  ▼                    ▼
┌─────────┐    ┌─────────────────┐   ┌─────────────┐
│ Win32   │    │ OpenAI/Anthropic│   │ External    │
│ ACL/SDK │    │ Google Gemini   │   │ Tools       │
└─────────┘    └─────────────────┘   └─────────────┘
```

---

## Key Components | 主要コンポーネント

### 1. Parallel Sub-Agent Executor | 並列サブエージェント実行器

**File**: `codex-rs/core/src/agents/parallel_executor.rs`

```rust
pub struct ParallelExecutor {
    agents: Vec<Agent>,
    orchestrator: Arc<Mutex<ParallelOrchestrator>>,
    task_queue: mpsc::Sender<Task>,
}

impl ParallelExecutor {
    pub async fn execute_parallel(&self, tasks: Vec<Task>) -> Vec<Result<Output>> {
        // Spawn concurrent agent tasks
        let handles: Vec<_> = tasks
            .into_iter()
            .map(|task| tokio::spawn(self.execute_single(task)))
            .collect();

        futures::future::join_all(handles).await
    }
}
```

**Technical Highlights | 技術的ハイライト**:

- Tokio-based async runtime for efficient concurrency
- Work-stealing scheduler for load balancing
- Graceful cancellation with structured concurrency

---

### 2. Windows Security Sandbox | Windowsセキュリティサンドボックス

**Directory**: `codex-rs/windows-sandbox-rs/`

This component provides secure process isolation on Windows through:

| Component          | Purpose                 | Win32 API                                        |
| ------------------ | ----------------------- | ------------------------------------------------ |
| `acl.rs`           | Access Control Lists    | `SetEntriesInAclW`, `SetNamedSecurityInfoW`      |
| `token.rs`         | Token Restriction       | `CreateRestrictedToken`, `AdjustTokenPrivileges` |
| `sandbox_users.rs` | Sandbox User Management | `NetUserAdd`, `NetLocalGroupAddMembers`          |
| `firewall.rs`      | Network Isolation       | `INetFwPolicy2`, Firewall COM API                |

**Security Flow | セキュリティフロー**:

```
1. Create restricted token (remove admin privileges)
2. Create sandbox user account
3. Set restrictive ACLs on workspace
4. Apply network firewall rules
5. Spawn process with restricted token
```

---

### 3. Git Worktree Manager | Git Worktree管理

**File**: `codex-rs/core/src/orchestration/worktree_manager.rs`

```rust
pub struct WorktreeManager {
    repo_path: PathBuf,
    active_worktrees: HashMap<String, WorktreeInfo>,
    conflict_prevention: ConflictPrevention,
}

impl WorktreeManager {
    /// Create parallel development branches with conflict detection
    pub async fn create_parallel_worktree(&mut self, branch: &str) -> Result<Worktree> {
        self.conflict_prevention.check_conflicts(branch)?;
        let worktree = git2::Repository::open(&self.repo_path)?
            .worktree(branch, &options)?;
        self.active_worktrees.insert(branch.to_string(), worktree.info());
        Ok(worktree)
    }
}
```

---

### 4. A2A Communication Protocol | A2A通信プロトコル

**File**: `codex-rs/core/src/a2a_communication.rs`

Agent-to-Agent communication using JSON-RPC over Unix sockets or named pipes:

```json
{
  "jsonrpc": "2.0",
  "method": "delegate_task",
  "params": {
    "agent_id": "sub-agent-001",
    "task": {
      "type": "code_review",
      "files": ["src/main.rs"],
      "context": {...}
    }
  },
  "id": 1
}
```

---

## Code Quality Metrics | コード品質メトリクス

| Metric             | Value                  | Tool              |
| ------------------ | ---------------------- | ----------------- |
| Lines of Rust Code | ~50,000                | `tokei`           |
| Test Coverage      | 65%+                   | `cargo-tarpaulin` |
| Clippy Warnings    | 0                      | `cargo clippy`    |
| Unsafe Blocks      | ~15 (Windows FFI only) | Manual audit      |

---

## Build & Release Pipeline | ビルド・リリースパイプライン

```yaml
# Simplified CI/CD flow
trigger: [push, pull_request]

jobs:
  - lint_build (6 targets: Linux/macOS/Windows × x86_64/aarch64)
  - tests (5 platforms)
  - security_scan (CodeQL, cargo-deny)
  - release (GitHub Releases with tar.gz)
```

---

## Future Roadmap | 今後のロードマップ

1. **GPU Acceleration** - CUDA/ROCm integration for local LLM inference
2. **Kubernetes Orchestration** - Container-based agent deployment
3. **WebAssembly Sandbox** - Cross-platform secure execution
4. **Multi-tenant Support** - Enterprise deployment features

---

_Last Updated: 2026-01-29 | v2.12.1_
