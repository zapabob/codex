---
name: performance-scout
description: Provide quick performance triage from logs, benchmarks, and runtime counters.
---

# Performance-Scout Agent Skill

## Overview

Provide quick performance triage from logs, benchmarks, and runtime counters.

## Capabilities

- Performance-Scout-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `read_file`
- `grep`
- `codebase_search`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./artifacts, ./performance-reports
### Network Access
- https://doc.rust-lang.org/*
### Shell Commands
- `hyperfine`
- `python`

## Usage Examples

### Basic Usage
```
codex $performance-scout "Perform performance-scout analysis on this codebase"
```

### Advanced Usage
```
codex $performance-scout "Review and improve the performance-scout implementation"
```

## Output Format

The performance-scout agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
