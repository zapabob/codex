---
name: dependency-analyst
description: Perform deep dependency analysis, manifest auditing, and supply-chain risk assessment across language ecosystems.
---

# Dependency-Analyst Agent Skill

## Overview

Perform deep dependency analysis, manifest auditing, and supply-chain risk assessment across language ecosystems.

## Capabilities

- Dependency-Analyst-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `codex_read_file`
- `codex_grep`
- `codex_codebase_search`
- `read_file`
- `grep`
- `codebase_search`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./artifacts, ./dependency-reports
### Network Access
- https://crates.io/*
- https://docs.rs/*
- https://registry.npmjs.org/*
- https://pypi.org/pypi/*
### Shell Commands
- `cargo`
- `npm`
- `pnpm`
- `pip`
- `poetry`
- `uv`
- `jq`

## Usage Examples

### Basic Usage
```
codex $dependency-analyst "Perform dependency-analyst analysis on this codebase"
```

### Advanced Usage
```
codex $dependency-analyst "Review and improve the dependency-analyst implementation"
```

## Output Format

The dependency-analyst agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
