---
name: refactorer
description: Refactor code to improve maintainability, performance, and readability. Identify code smells, extract reusable components, optimize algorithms, and reduce technical debt.
---

# Refactorer Agent Skill

## Overview

Refactor code to improve maintainability, performance, and readability. Identify code smells, extract reusable components, optimize algorithms, and reduce technical debt.

## Capabilities

- Refactorer-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `codex_read_file`
- `codex_grep`
- `codex_codebase_search`
- `codex_write_file`
- `codex_search_replace`
- `grep`
- `read_file`
- `codebase_search`
- `write`
- `search_replace`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./src, ./lib, ./app, ./components, ./artifacts, ./refactoring-reports
### Network Access
- https://refactoring.guru/*
- https://martinfowler.com/*
- https://docs.rs/*
- https://doc.rust-lang.org/*
### Shell Commands
- `cargo`
- `npm`
- `rustfmt`
- `clippy`
- `prettier`
- `eslint`

## Usage Examples

### Basic Usage
```
codex $refactorer "Perform refactorer analysis on this codebase"
```

### Advanced Usage
```
codex $refactorer "Review and improve the refactorer implementation"
```

## Output Format

The refactorer agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
