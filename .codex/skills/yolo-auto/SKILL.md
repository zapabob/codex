---
name: yolo-auto
description: "YOLO (You Only Live Once) full-stack automation skill with OpenClaw MCP bridge and GPU model selection."
---

# YOLO Auto Agent Skill

## Overview

YOLO (You Only Live Once) full-stack automation skill with OpenClaw MCP bridge and GPU model selection. Enables autonomous multi-agent orchestration with intelligent task distribution across different GPU models.

## Capabilities

- **GPU Model Selection**: Select optimal GPU model (claude-3-opus, gpt-4, gpu-5.3-codex)
- **Multi-Agent Orchestration**: Coordinate multiple agents for complex tasks
- **OpenClaw Integration**: Use OpenClaw MCP bridge for tool access
- **Task Distribution**: Distribute work across agents intelligently
- **Result Aggregation**: Collect and synthesize results from multiple sources
- **Autonomous Execution**: Execute complex workflows with minimal input

## Tools Required

### MCP Tools

- `yolo_select_gpu` - Select GPU model for task
- `yolo_distribute_task` - Distribute task to agents
- `yolo_aggregate_results` - Aggregate results from agents
- `yolo_execute_workflow` - Execute multi-step workflow
- `yolo_monitor_progress` - Monitor workflow progress
- `openclaw_execute` - Execute via OpenClaw bridge

### File System Access

- **Read**: Full project access
- **Write**: `./yolo-workflows/`, `./artifacts/`, `./results/`

### Network Access

- **None required** - Local OpenClaw operations

### Shell Commands

- `openclaw` - OpenClaw CLI
- `python` - Python for orchestration scripts
- `git` - Version control

## Usage Examples

### Basic Workflow

```bash
codex $yolo-auto "Execute full-stack development workflow"
```

### GPU Selection

```bash
codex $yolo-auto "Select claude-3-opus for complex reasoning task"
```

### Multi-Agent Task

```bash
codex $yolo-auto "Distribute tasks: 3 agents for frontend, 2 for backend"
```

## YOLO Architecture

```
┌─────────────────────────────────────────┐
│           YOLO Orchestrator              │
├─────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    │
│  │ GPU Router  │───▶│ Task Queue   │    │
│  └─────────────┘    └─────────────┘    │
│         │                  │            │
│         ▼                  ▼            │
│  ┌─────────────┐    ┌─────────────┐    │
│  │ Agent Pool  │◀───│ Work Stealer │    │
│  │ - claude-3  │    └─────────────┘    │
│  │ - gpt-4     │                        │
│  │ - gpu-5.3   │                        │
│  └─────────────┘                        │
│         │                                │
│         ▼                                │
│  ┌─────────────┐                        │
│  │ Result       │                        │
│  │ Aggregator  │                        │
│  └─────────────┘                        │
└─────────────────────────────────────────┘
```

## GPU Models

| Model         | Strengths                  | Use Cases                |
| ------------- | -------------------------- | ------------------------ |
| claude-3-opus | Complex reasoning, coding  | Architecture, planning   |
| gpt-4         | Fast iteration, creativity | Prototyping, exploration |
| gpu-5.3-codex | Local processing           | Security, privacy tasks  |

## OpenClaw Integration

OpenClaw provides MCP bridge for external tools:

```json
{
  "mcpServers": {
    "openclaw": {
      "command": "openclaw",
      "args": ["serve"]
    }
  }
}
```

## Workflow Definition

```yaml
name: full-stack-web
stages:
  - name: frontend
    agents: 2
    gpu: claude-3-opus
    tasks:
      - Create React components
      - Implement state management
  - name: backend
    agents: 2
    gpu: gpt-4
    tasks:
      - Design API schema
      - Implement endpoints
  - name: integration
    agents: 1
    gpu: claude-3-opus
    tasks:
      - Connect frontend to backend
      - Test end-to-end
```

## Output Format

The yolo-auto agent provides:

- Workflow execution reports
- Agent task distributions
- GPU utilization metrics
- Result aggregations
- Performance statistics

## References

- [OpenClaw GitHub](https://github.com/anomalyco/openclaw)
- [MCP Protocol](https://modelcontextprotocol.io/)
- [Claude Code](https://claude.com/code)
- [Multi-Agent Systems](https://arxiv.org/abs/2308.02790)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-yolo-auto-skill`
**Version**: 1.0.0
**Compatibility**: Codex v2.14.0+
