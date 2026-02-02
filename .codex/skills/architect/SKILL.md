---
name: architect
description: Analyze system architecture, design patterns, module boundaries, and scalability. Provide high-level design recommendations and architectural decision records (ADR).
---

# Architect Agent Skill

## Overview

Analyze system architecture, design patterns, module boundaries, and scalability. Provide high-level design recommendations and architectural decision records (ADR).

## Capabilities

- Architect-specific analysis and recommendations
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
- **Write**: Limited to ./artifacts, ./architecture-docs, ./adrs
### Network Access
- https://docs.rs/*
- https://doc.rust-lang.org/*
- https://developer.mozilla.org/*
- https://martinfowler.com/*
- https://c4model.com/*
### Shell Commands
- `cargo`
- `npm`
- `tree`
- `cloc`

## Usage Examples

### Basic Usage
```
codex $architect "Perform architect analysis on this codebase"
```

### Advanced Usage
```
codex $architect "Review and improve the architect implementation"
```

## Output Format

The architect agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
