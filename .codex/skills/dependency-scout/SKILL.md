---
name: dependency-scout
description: Lightweight dependency insight agent for quick manifest scans, transitive surfacing, and license notes.
---

# Dependency-Scout Agent Skill

## Overview

Lightweight dependency insight agent for quick manifest scans, transitive surfacing, and license notes.

## Capabilities

- Dependency-Scout-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `read_file`
- `grep`
- `codebase_search`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./artifacts, ./dependency-reports
### Network Access
- https://crates.io/*
- https://registry.npmjs.org/*
### Shell Commands
- `jq`

## Usage Examples

### Basic Usage
```
codex $dependency-scout "Perform dependency-scout analysis on this codebase"
```

### Advanced Usage
```
codex $dependency-scout "Review and improve the dependency-scout implementation"
```

## Output Format

The dependency-scout agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
