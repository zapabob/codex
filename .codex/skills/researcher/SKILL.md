---
name: researcher
description: Deep research with multi-source validation, citation, and comprehensive reporting
---

# Researcher Agent Skill

## Overview

Deep research with multi-source validation, citation, and comprehensive reporting

## Capabilities

- Researcher-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `web_search`
- `read_file`
- `grep`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./artifacts, ./research-reports
### Network Access
- https://*

## Usage Examples

### Basic Usage
```
codex $researcher "Perform researcher analysis on this codebase"
```

### Advanced Usage
```
codex $researcher "Review and improve the researcher implementation"
```

## Output Format

The researcher agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
