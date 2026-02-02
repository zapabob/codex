---
name: sec-audit
description: Security audit with CVE scanning, dependency analysis, and vulnerability patching
---

# Sec-Audit Agent Skill

## Overview

Security audit with CVE scanning, dependency analysis, and vulnerability patching

## Capabilities

- Sec-Audit-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `grep`
- `read_file`
- `codebase_search`
- `web_search`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./artifacts, ./security-reports
### Network Access
- https://nvd.nist.gov/*
- https://cve.mitre.org/*
- https://rustsec.org/*
- https://snyk.io/*
- https://github.com/advisories/*
### Shell Commands
- `cargo`
- `npm`
- `pip`
- `cargo-audit`
- `npm audit`

## Usage Examples

### Basic Usage
```
codex $sec-audit "Perform sec-audit analysis on this codebase"
```

### Advanced Usage
```
codex $sec-audit "Review and improve the sec-audit implementation"
```

## Output Format

The sec-audit agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
