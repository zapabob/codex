---
name: unity-reviewer
description: Unity C#蟆ら畑繧ｳ繝ｼ繝峨Ξ繝薙Η繝ｼ繝ｻ繝代ヵ繧ｩ繝ｼ繝槭Φ繧ｹ譛驕ｩ蛹悶・Unity繝吶せ繝医・繝ｩ繧ｯ繝・ぅ繧ｹ
---

# Unity-Reviewer Agent Skill

## Overview

Unity C#蟆ら畑繧ｳ繝ｼ繝峨Ξ繝薙Η繝ｼ繝ｻ繝代ヵ繧ｩ繝ｼ繝槭Φ繧ｹ譛驕ｩ蛹悶・Unity繝吶せ繝医・繝ｩ繧ｯ繝・ぅ繧ｹ

## Capabilities

- Unity-Reviewer-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

### MCP Tools
- `code_indexer`
- `ast_analyzer`
- `csharp_language_server`
- `unity_analyzer`
### File System Access
- **Read**: Full codebase access
- **Write**: Limited to ./artifacts, ./review-comments, ./Assets
### Shell Commands
- `dotnet`
- `msbuild`
- `csc`
- `git`

## Usage Examples

### Basic Usage
```
codex $unity-reviewer "Perform unity-reviewer analysis on this codebase"
```

### Advanced Usage
```
codex $unity-reviewer "Review and improve the unity-reviewer implementation"
```

## Output Format

The unity-reviewer agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
