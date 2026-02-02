---
name: python-reviewer
description: Python蟆ら畑繧ｳ繝ｼ繝峨Ξ繝薙Η繝ｼ繝ｻ蝙九ヲ繝ｳ繝医・Django/FastAPI繝吶せ繝医・繝ｩ繧ｯ繝・ぅ繧ｹ
---

# Python-Reviewer Agent Skill

## Overview

Python蟆ら畑繧ｳ繝ｼ繝峨Ξ繝薙Η繝ｼ繝ｻ蝙九ヲ繝ｳ繝医・Django/FastAPI繝吶せ繝医・繝ｩ繧ｯ繝・ぅ繧ｹ

## Capabilities

- Python-Reviewer-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `code_indexer`
- `ast_analyzer`
- `python_language_server`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./artifacts, ./review-comments
### Shell Commands
- `python`
- `python3`
- `pip`
- `pylint`
- `black`
- `mypy`
- `flake8`
- `bandit`
- `pytest`
- `isort`
- `git`

## Usage Examples

### Basic Usage
```
codex $python-reviewer "Perform python-reviewer analysis on this codebase"
```

### Advanced Usage
```
codex $python-reviewer "Review and improve the python-reviewer implementation"
```

## Output Format

The python-reviewer agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
