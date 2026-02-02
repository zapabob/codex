---
name: test-gen
description: Generate comprehensive test suites including unit, integration, and e2e tests
---

# Test-Gen Agent Skill

## Overview

Generate comprehensive test suites including unit, integration, and e2e tests

## Capabilities

- Test-Gen-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `grep`
- `read_file`
- `codebase_search`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./tests, ./__tests__, ./test, ./artifacts
### Network Access
- https://docs.rs/*
- https://jestjs.io/*
- https://vitest.dev/*
- https://docs.pytest.org/*
### Shell Commands
- `cargo`
- `npm`
- `python`
- `pytest`
- `jest`

## Usage Examples

### Basic Usage
```
codex $test-gen "Perform test-gen analysis on this codebase"
```

### Advanced Usage
```
codex $test-gen "Review and improve the test-gen implementation"
```

## Output Format

The test-gen agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
