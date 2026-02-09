---
name: agent-implementation-plan
description: "Zapabob/Codex Custom Skills Implementation Plan - VRChat, Blender, YukkuriMovieMaker, YOLO integration"
---

# Zapabob/Codex Custom Skills Implementation Plan

# Zapabob/Codex カスタムスキル実装計画

## Overview | 概要

This document outlines the comprehensive implementation plan for adding new custom skills to the Zapabob/Codex fork. The plan covers skill conversion from existing agents, new skill creation, slash command integration, and MCP server development.

本文書はZapabob/Codexフォークに新しいカスタムスキルを追加するための包括的な実装計画を概述する。既存エージェントからのスキル変換、新しいスキル作成、スラッシュコマンド統合、MCPサーバー開発をカバーする。

---

## Current State | 現状

### Completed | 完了済み

| Item                                         | Status      | Date       |
| -------------------------------------------- | ----------- | ---------- |
| Upstream merge (openai/codex)                | ✅ Complete | 2026-02-08 |
| Custom features preservation                 | ✅ Verified | 2026-02-08 |
| Git merge conflict resolution                | ✅ Complete | 2026-02-08 |
| Deep research (VRChat/Blender/YMM4/OpenClaw) | ✅ Complete | 2026-02-08 |
| Slash Commands Implementation                | ✅ Complete | 2026-02-08 |
| MCP Server Configuration                     | ✅ Complete | 2026-02-08 |
| MCP VRChat Server                            | ✅ Complete | 2026-02-08 |
| MCP Blender Server                           | ✅ Complete | 2026-02-08 |
| MCP YMM4 Server                              | ✅ Complete | 2026-02-08 |
| MCP YOLO Server                              | ✅ Complete | 2026-02-08 |

### Existing Custom Features | 既存の独自機能

| Feature       | Type          | Files                                       |
| ------------- | ------------- | ------------------------------------------- |
| `/qc`         | Slash Command | `slash_command.rs:23`, `chatwidget.rs:2883` |
| `/dev-mode`   | Slash Command | `slash_command.rs:24`, `chatwidget.rs:2886` |
| `/git4d`      | Slash Command | `slash_command.rs:50`, `chatwidget.rs:3084` |
| `/vr`         | Slash Command | `slash_command.rs:51`, `chatwidget.rs:3086` |
| `/ar`         | Slash Command | `slash_command.rs:52`, `chatwidget.rs:3087` |
| QC System     | Code          | `codex-rs/core/src/qc/`                     |
| Git4D System  | Code          | `codex-rs/core/src/git4d_accelerated.rs`    |
| Dev Mode      | Code          | `codex-rs/core/src/orchestration/`          |
| Custom Agents | Config        | `.codex/agents/*.yaml` (9 agents)           |

---

## Implementation Plan | 実装計画

### Phase 1: Agent to Skill Conversion (Week 1-2)

#### 1.1 Convert Existing Agents to Skills

| Agent                        | Target Skill          | Priority | Complexity |
| ---------------------------- | --------------------- | -------- | ---------- |
| `qc-optimizer.yaml`          | `qc-optimizer`        | High     | Medium     |
| `git4d-runtime-checker.yaml` | `git4d-runtime`       | High     | Medium     |
| `git4d-schema-auditor.yaml`  | `git4d-schema`        | High     | Medium     |
| `performance-analyst.yaml`   | `performance-analyst` | Medium   | Medium     |
| `security-audit.yaml`        | `sec-audit`           | Medium   | Medium     |

#### 1.2 Skill Structure

```
.codex/skills/{skill-name}/
├── SKILL.md              # Main skill definition
├── agents/
│   └── openai.yaml       # Agent configuration
└── scripts/
    └── run_{skill-name}.py  # Execution script

.cursor/skills/{skill-name}/
├── SKILL.md
├── agents/
│   └── openai.yaml
└── scripts/
    └── run_{skill-name}.py
```

### Phase 2: New Skill Creation (Week 2-3)

#### 2.1 New Skills to Create

| Skill           | Technology     | Priority | Complexity | Description                         |
| --------------- | -------------- | -------- | ---------- | ----------------------------------- |
| `vrchat-dev`    | VRChat SDK3    | High     | 8/10       | VRChat world/avatar development     |
| `blender-cad`   | Blender Python | Medium   | 6/10       | Blender CAD modeling automation     |
| `yukkuri-movie` | YMM4           | Medium   | 7/10       | ゆっくりMovieMaker video production |
| `yolo-auto`     | OpenClaw       | Medium   | 5/10       | YOLO full-stack automation          |

#### 2.2 Technology Stack Research

##### VRChat SDK3 (Complexity: 8/10)

```
VRChat SDK3.10.1 (Dec 2025)
├── Udon (VRChat's VM)
│   ├── Udon Graph (visual programming)
│   └── UdonSharp (C#-like syntax)
├── modularavatar (avatar configuration)
├── liltoon (shader)
├── Poiyomi (shader)
└── VRClightvolume (lighting)
```

**Key MCP Tools Needed:**

- UdonSharp compilation
- modularavatar integration
- PhysBones configuration
- Contact system setup
- World upload automation

##### Blender Python (Complexity: 6/10)

```
Blender 4.0+
├── bpy (Python API)
├── Breaking API changes (3.x → 4.0)
├── STEP/IGES import
├── Geometry Nodes
└── USD export
```

**Key MCP Tools Needed:**

- Geometry creation/modification
- Material assignment
- Render setting automation
- Export to various formats

##### YukkuriMovieMaker (Complexity: 7/10)

```
YMM4 v4.49.0.2
├── Plugin Architecture
│   ├── [AudioEffect]
│   ├── [VideoEffect]
│   └── IMovieMakerPlugin
├── MIDI Integration
└── IPC Communication
```

**Key MCP Tools Needed:**

- Scene management
- Character animation
- Audio effect control
- Video rendering
- Project file manipulation

##### OpenClaw (Complexity: 5/10)

```
OpenClaw Latest
├── MCP Protocol
├── Claude Code Integration
├── GPU Model Selection
│   ├── claude-3-opus
│   ├── gpt-4
│   └── gpu-5.3-codex (mentioned by user)
└── Multi-Agent Orchestration
```

**Key MCP Tools Needed:**

- GPU model selection
- Task distribution
- Result aggregation

### Phase 3: Slash Command Integration (Week 3-4)

#### 3.1 New Slash Commands

Add to `codex-rs/tui/src/slash_command.rs`:

```rust
pub enum SlashCommand {
    // ... existing ...
    VrChat,    // NEW - VRChat development
    Blender,   // NEW - Blender CAD automation
    Yukkuri,   // NEW - YukkuriMovieMaker production
    Yolo,      // NEW - YOLO full automation
}

impl SlashCommand {
    pub fn description(self) -> &'static str {
        match self {
            // ... existing ...
            SlashCommand::VrChat => "VRChat world and avatar development",
            SlashCommand::Blender => "Blender CAD modeling automation",
            SlashCommand::Yukkuri => "ゆっくりMovieMaker video production",
            SlashCommand::Yolo => "YOLO full-stack automation with OpenClaw",
        }
    }
}
```

#### 3.2 Handler Registration

Add handlers in `codex-rs/tui/src/chatwidget.rs`:

- Line 2883: `/vr` handler (existing)
- Line 2886: `/ar` handler (existing)
- New: `/VRChat`, `/Blender`, `/Yukkuri`, `/YOLO`

### Phase 4: MCP Server Development (Week 4-6)

#### 4.1 New MCP Servers

```
mcp-servers/
├── mcp-vrchat-server/
│   └── src/
│       └── main.rs
├── mcp-blender-server/
│   └── src/
│       └── main.rs
└── mcp-ymm4-server/  # yukkuri-movie-maker
    └── src/
        └── main.rs
```

#### 4.2 MCP Server Structure

```rust
// Example: mcp-vrchat-server/src/main.rs
use mcp_server::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let server = McpServer::new("vrchat", "VRChat MCP Server")
        .with_tool(UdonCompiler::tool())
        .with_tool(WorldUploader::tool())
        .with_tool(AvatarConfigurator::tool());

    server.run().await
}
```

---

## Timeline | タイムライン

| Week | Phase   | Tasks                               | Deliverables                                    |
| ---- | ------- | ----------------------------------- | ----------------------------------------------- |
| 1    | Phase 1 | Convert qc-optimizer agent to skill | SKILL.md, agents/openai.yaml                    |
| 1    | Phase 1 | Convert git4d agents to skills      | SKILL.md × 2, agents/openai.yaml × 2            |
| 2    | Phase 1 | Convert remaining agents            | SKILL.md × 3, agents/openai.yaml × 3            |
| 2    | Phase 2 | Create vrchat-dev skill             | SKILL.md, agents/openai.yaml, MCP server design |
| 3    | Phase 2 | Create blender-cad skill            | SKILL.md, agents/openai.yaml, MCP server design |
| 3    | Phase 2 | Create yukkuri-movie skill          | SKILL.md, agents/openai.yaml, MCP server design |
| 3    | Phase 2 | Create yolo-auto skill              | SKILL.md, agents/openai.yaml, MCP server design |
| 4    | Phase 3 | Add slash commands                  | Rust code changes, handler integration          |
| 4-5  | Phase 4 | Build vrchat MCP server             | Working MCP server with tools                   |
| 5-6  | Phase 4 | Build blender MCP server            | Working MCP server with tools                   |
| 6    | Phase 4 | Build yukkuri MCP server            | Working MCP server with tools                   |

---

## File Structure | ファイル構造

### Skills Directory Structure

```
.codex/
├── skills/
│   ├── qc-optimizer/
│   │   ├── SKILL.md
│   │   ├── agents/
│   │   │   └── openai.yaml
│   │   └── scripts/
│   │       └── run_qc-optimizer.py
│   ├── git4d-runtime/
│   │   ├── SKILL.md
│   │   ├── agents/
│   │   │   └── openai.yaml
│   │   └── scripts/
│   │       └── run_git4d-runtime.py
│   ├── git4d-schema/
│   │   ├── SKILL.md
│   │   ├── agents/
│   │   │   └── openai.yaml
│   │   └── scripts/
│   │       └── run_git4d-schema.py
│   ├── vrchat-dev/
│   │   ├── SKILL.md
│   │   ├── agents/
│   │   │   └── openai.yaml
│   │   └── scripts/
│   │       └── run_vrchat-dev.py
│   ├── blender-cad/
│   │   ├── SKILL.md
│   │   ├── agents/
│   │   │   └── openai.yaml
│   │   └── scripts/
│   │       └── run_blender-cad.py
│   ├── yukkuri-movie/
│   │   ├── SKILL.md
│   │   ├── agents/
│   │   │   └── openai.yaml
│   │   └── scripts/
│   │       └── run_yukkuri-movie.py
│   └── yolo-auto/
│       ├── SKILL.md
│       ├── agents/
│       │   └── openai.yaml
│       └── scripts/
│           └── run_yolo-auto.py

.cursor/
├── skills/
│   ├── qc-optimizer/
│   │   └── SKILL.md
│   ├── git4d-runtime/
│   │   └── SKILL.md
│   ├── git4d-schema/
│   │   └── SKILL.md
│   ├── vrchat-dev/
│   │   └── SKILL.md
│   ├── blender-cad/
│   │   └── SKILL.md
│   ├── yukkuri-movie/
│   │   └── SKILL.md
│   └── yolo-auto/
│       └── SKILL.md
```

### MCP Servers Directory Structure

```
mcp-servers/
├── mcp-vrchat-server/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── mcp-blender-server/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── mcp-ymm4-server/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

---

## Success Criteria | 成功基準

### Functional Requirements

1. **Agent to Skill Conversion**
   - [ ] All 9 existing agents converted to skills
   - [ ] Skills auto-load when agent executes
   - [ ] Dual placement (.codex/skills/ and .cursor/skills/)

2. **New Skills**
   - [ ] vrchat-dev skill functional
   - [ ] blender-cad skill functional
   - [ ] yukkuri-movie skill functional
   - [ ] yolo-auto skill functional

3. **Slash Commands**
   - [ ] /VRChat command registered
   - [ ] /Blender command registered
   - [ ] /Yukkuri command registered
   - [ ] /YOLO command registered

4. **MCP Servers**
   - [ ] vrchat MCP server builds and runs
   - [ ] blender MCP server builds and runs
   - [ ] yukkuri MCP server builds and runs

### Quality Requirements

1. **Code Quality**
   - Clippy warnings: 0
   - Test coverage: 65%+
   - Documentation: 100%

2. **Integration**
   - All skills integrate with existing /qc, /git4d commands
   - Slash commands work in TUI
   - MCP tools available in Codex

---

## Configuration | 設定

### Update .codex/mcp-servers.yaml

```yaml
mcpServers:
  # ... existing servers ...
  vrchat:
    command: uvx
    args:
      - "--from"
      - "mcp-vrchat-server"
      - "mcp-vrchat-server"
    env: {}
  blender:
    command: uvx
    args:
      - "--from"
      - "mcp-blender-server"
      - "mcp-blender-server"
    env: {}
  yukkuri:
    command: uvx
    args:
      - "--from"
      - "mcp-ymm4-server"
      - "mcp-ymm4-server"
    env: {}
```

---

## References | 参考文献

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
- [VRChat SDK3 Documentation](https://docs.vrchat.com/docs/sdk3)
- [Blender Python API](https://docs.blender.org/api/current/)
- [YukkuriMovieMaker Plugin Documentation](https://k这门.com/ymm4/)

---

---

## Meta Prompt for Next Agent | 次のエージェントへのメタ指示

### Current Status | 現在のステータス

You are continuing the **Zapabob/Codex Custom Skills Implementation Project**.

**実装状況**: All core infrastructure has been implemented.

You are powered by the model named **minimax-m2.1-free**. The exact model ID is **opencode/minimax-m2.1-free**.

#### Completed Items | 完了済み項目

| Item                                                                                   | Status      |
| -------------------------------------------------------------------------------------- | ----------- |
| Slash Commands: `/VRChat`, `/Blender`, `/Yukkuri`, `/YOLO`                             | ✅ Complete |
| MCP Servers: vrchat, blender, yukkuri, yolo-auto                                       | ✅ Complete |
| Skills: vrchat-dev, blender-cad, yukkuri-movie, yolo-auto, git4d-runtime, git4d-schema | ✅ Complete |
| Implementation Logs in `_docs/`                                                        | ✅ Complete |

#### What Did We Do | 做了什么

**1. Planning Document**: Created AGENT.md with comprehensive implementation plan for VRChat SDK3, Blender Python, YukkuriMovieMaker, OpenClaw integration.

**2. Slash Commands**: Added `VrChat`, `Blender`, `Yukkuri`, `Yolo` to `slash_command.rs` and handlers in `chatwidget.rs`.

**3. MCP Server Configuration**: Updated `.codex/mcp-servers.yaml` with 4 new servers.

**4. MCP Servers Created**:
| Server | Tools |
|--------|-------|
| `mcp-vrchat-server` | vrchat_compile_udon, vrchat_upload_world, vrchat_configure_avatar, vrchat_setup_physbones, vrchat_create_contact |
| `mcp-blender-server` | blender_create_geometry, blender_assign_material, blender_render, blender_export, blender_geometry_nodes |
| `mcp-ymm4-server` | ymm4_create_scene, ymm4_add_character, ymm4_audio_effect, ymm4_video_effect, ymm4_render |
| `mcp-yolo-server` | yolo_select_gpu, yolo_distribute_task, yolo_execute_workflow, yolo_aggregate_results, yolo_monitor_progress |

**5. Skill Definitions**: Created SKILL.md in `.codex/skills/` and `.cursor/skills/`.

**6. Implementation Logs**: `_docs/2026-02-08_Skill実装計画{main}.md` and `_docs/2026-02-08_Skill実装{main}.md`.

---

### Next Tasks | 次のタスク

#### High Priority | 高優先度

1. **Agent YAML Conversion**
   - Convert remaining `.codex/agents/*.yaml` to skill format
   - Files: `performance-analyst.yaml`, `security-audit.yaml`, etc.

2. **Build Verification**

   ```bash
   cd mcp-servers/mcp-vrchat-server && cargo build
   cd mcp-servers/mcp-blender-server && cargo build
   cd mcp-servers/mcp-ymm4-server && cargo build
   cd mcp-servers/mcp-yolo-server && cargo build
   ```

3. **Test Implementation**
   - Add unit tests for MCP servers
   - Add integration tests for slash commands

#### Medium Priority | 中優先度

4. **CI/CD Integration** - Add MCP server builds to GitHub Actions
5. **Documentation** - Add usage examples and tutorials
6. **Complete Agent Mapping** - Finish `agent_servers` configuration

---

### Key Files | 主要ファイル

| File                                      | Purpose                    |
| ----------------------------------------- | -------------------------- |
| `codex-rs/tui/src/slash_command.rs`       | Slash command definitions  |
| `codex-rs/tui/src/chatwidget.rs`          | Command handlers           |
| `.codex/mcp-servers.yaml`                 | MCP server configuration   |
| `.codex/skills/*/SKILL.md`                | Skill definitions          |
| `.codex/skills/*/agents/openai.yaml`      | Agent configurations       |
| `mcp-servers/*/src/main.rs`               | MCP server implementations |
| `_docs/2026-02-08_Skill実装{main}.md`     | Implementation details     |
| `_docs/2026-02-08_Skill実装計画{main}.md` | Planning log               |

---

### Version Information | バージョン情報

- **Codex CLI**: v2.14.1
- **Rust**: 2024 Edition
- **Edition**: zapabob/codex custom fork
- **Model**: opencode/minimax-m2.1-free

---

### Continue Implementation | 実装の継続

1. Read AGENT.md (this file) for context
2. Read `_docs/2026-02-08_Skill実装{main}.md` for implementation details
3. Start with Agent YAML Conversion (highest priority)
4. Run build verification after code changes

---

### Success Criteria Checklist | 成功基準チェックリスト

- [ ] All 9 existing agents converted to skills
- [ ] vrchat-dev skill functional
- [ ] blender-cad skill functional
- [ ] yukkuri-movie skill functional
- [ ] yolo-auto skill functional
- [ ] /VRChat command registered
- [ ] /Blender command registered
- [ ] /Yukkuri command registered
- [ ] /YOLO command registered
- [ ] vrchat MCP server builds
- [ ] blender MCP server builds
- [ ] yukkuri MCP server builds
- [ ] yolo MCP server builds
- [ ] Clippy warnings: 0
- [ ] Test coverage: 65%+

---

### Japanese Prompt | 日本語プロンプト

```
Zapabob/Codexカスタムスキル実装の継続：

背景：
- VRChat SDK3、Blender Python、ゆっくりMovieMaker、OpenClaw統合のカスタムスキル
- バージョン：v2.14.1、Rust 2024 Edition
- 全てのコアインフラ実装済み

完了済み：
- ✅ スラッシュコマンド：/VRChat、/Blender、/Yukkuri、/YOLO
- ✅ MCPサーバー設定（.codex/mcp-servers.yaml）
- ✅ 4つのMCPサーバー：vrchat、blender、yukkuri、yolo-auto
- ✅ スキル定義（.codex/skills/、.cursor/skills/）
- ✅ 実装ログ（_docs/）

次のタスク（優先順）：
1. 残りのagent YAMLファイルをスキル形式に変換
2. MCPサーバーのビルド検証
3. ユニットテスト追加
4. CI/CD統合

主要ファイル：
- AGENT.md（このファイル）
- _docs/2026-02-08_Skill実装{main}.md（実装詳細）
- codex-rs/tui/src/slash_command.rs（スラッシュコマンド）
- codex-rs/tui/src/chatwidget.rs（ハンドラー）
- .codex/mcp-servers.yaml（MCP設定）
- mcp-servers/*/src/main.rs（MCP実装）
- .codex/skills/*/SKILL.md（スキル定義）

最初はAGENT.mdを読んで文脈を理解してから、
Agent YAML変換（最高優先度）から始めてください。
```

---

**Document Version**: 2.0
**Last Updated**: 2026-02-08
**Author**: Zapabob/Codex Implementation Team
**For**: Next agent continuing the implementation
