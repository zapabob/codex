---
name: ts-reviewer
description: TypeScript/JavaScript蟆ら畑繧ｳ繝ｼ繝峨Ξ繝薙Η繝ｼ繝ｻ蝙句ｮ牙・諤ｧ繝ｻReact/Next.js繝吶せ繝医・繝ｩ繧ｯ繝・ぅ繧ｹ
---

# Ts-Reviewer Agent Skill

## Overview

TypeScript/JavaScript蟆ら畑繧ｳ繝ｼ繝峨Ξ繝薙Η繝ｼ繝ｻ蝙句ｮ牙・諤ｧ繝ｻReact/Next.js繝吶せ繝医・繝ｩ繧ｯ繝・ぅ繧ｹ

## Capabilities

- Ts-Reviewer-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `code_indexer`
- `ast_analyzer`
- `typescript_language_server`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./artifacts, ./review-comments
### Shell Commands
- `npm`
- `npx`
- `eslint`
- `prettier`
- `tsc`
- `jest`
- `vitest`
- `git`

## Usage Examples

### Basic Usage
```
codex $ts-reviewer "Perform ts-reviewer analysis on this codebase"
```

### Advanced Usage
```
codex $ts-reviewer "Review and improve the ts-reviewer implementation"
```

## Output Format

The ts-reviewer agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
