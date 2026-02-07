---
name: qc-optimizer
description: "Before merging to main, orchestrate full-stack QC: run unit/abnormal/comprehensive tests, apply statistical + quantum-inspired optimization heuristics, and log merge decisions."
---

# Qc-Optimizer Agent Skill

## Overview

Before merging to main, orchestrate full-stack QC: run unit/abnormal/comprehensive tests, apply statistical + quantum-inspired optimization heuristics, and log merge decisions.

## Capabilities

- Qc-Optimizer-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools

- `codex_read_file`
- `codex_grep`
- `codex_codebase_search`
- `grep`
- `read_file`
- `codebase_search`

### File System Access

- **Read**: Full codebase access
- **Write**: Limited to ./\_doc, ./docs, ./artifacts

### Network Access

- https://docs.rs/*
- https://doc.rust-lang.org/*
- https://developer.mozilla.org/*

### Shell Commands

- `cargo`
- `npm`
- `pnpm`
- `just`
- `git`
- `node`

## Usage Examples

### Basic Usage

```
codex $qc-optimizer "Perform qc-optimizer analysis on this codebase"
```

### Advanced Usage

```
codex $qc-optimizer "Review and improve the qc-optimizer implementation"
```

## Output Format

The qc-optimizer agent provides:

- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
