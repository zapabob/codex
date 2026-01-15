# Ts-Reviewer Agent Skill

## Overview

TypeScript/JavaScript専用コードレビュー・型安全性・React/Next.jsベストプラクティス

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
