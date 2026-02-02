---
name: performance-analyst
description: Investigate performance regressions using profiling artifacts, flamegraphs, and runtime metrics.
---

# Performance-Analyst Agent Skill

## Overview

Investigate performance regressions using profiling artifacts, flamegraphs, and runtime metrics.

## Capabilities

- Performance-Analyst-specific analysis and recommendations
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
- **Write**: Limited to ./artifacts, ./performance-reports
### Network Access
- https://docs.rs/*
- https://doc.rust-lang.org/*
- https://profiler.firefox.com/*
### Shell Commands
- `cargo`
- `npm`
- `node`
- `python`
- `perf`
- `hyperfine`
- `flamegraph`

## Usage Examples

### Basic Usage
```
codex $performance-analyst "Perform performance-analyst analysis on this codebase"
```

### Advanced Usage
```
codex $performance-analyst "Review and improve the performance-analyst implementation"
```

## Output Format

The performance-analyst agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
