---
name: code-reviewer
description: Advanced code analysis agent that performs comprehensive code reviews focusing on security, performance, maintainability, and best practices. Generates actionable feedback with severity levels and automated fix suggestions.
---

# Code Reviewer Agent Skill

## Overview

Advanced code analysis agent that performs comprehensive code reviews focusing on security, performance, maintainability, and best practices. Generates actionable feedback with severity levels and automated fix suggestions.

## Capabilities

- **Security Analysis**: Vulnerability detection, unsafe patterns, authentication issues
- **Performance Review**: Bottleneck identification, optimization opportunities, resource usage analysis
- **Code Quality**: Maintainability assessment, complexity analysis, duplication detection
- **Best Practices**: Language-specific conventions, framework compliance, testing coverage
- **Type Safety**: Type checking, null safety, generic usage validation

## Tools Required

### MCP Tools
- `codex_read_file` - Source code reading and analysis
- `codex_grep` - Pattern-based code searching
- `codex_codebase_search` - Semantic code search across project
- `grep` - Text pattern matching
- `read_file` - File content access
- `codebase_search` - Project-wide search

### File System Access
- **Read**: Full codebase access for analysis
- **Write**: Limited to `./artifacts`, `./code-review-reports`, `./review-results`

### Network Access
- `https://docs.rs/*` - Rust documentation and crate info
- `https://doc.rust-lang.org/*` - Official Rust documentation
- `https://docs.python.org/*` - Python standard library docs
- `https://developer.mozilla.org/*` - Web standards and APIs
- `https://cwe.mitre.org/*` - Common Weakness Enumeration database

### Shell Commands
- `cargo check` - Rust compilation checking
- `cargo clippy` - Rust linting
- `npm audit` - Node.js security audit
- `python -m py_compile` - Python syntax checking
- `rustfmt --check` - Code formatting validation

## Usage Examples

### Basic Code Review
```bash
codex $code-reviewer "Review the authentication module for security issues"
```

### Comprehensive Analysis
```bash
codex $code-reviewer "Perform full code review: security, performance, and maintainability analysis of the entire codebase"
```

### Language-Specific Review
```bash
codex $code-reviewer "Review Rust code for unsafe patterns, lifetime issues, and performance bottlenecks"
codex $code-reviewer "Review Python code for type hints, error handling, and best practices"
```

### CI/CD Integration
```bash
codex $code-reviewer "Analyze pull request changes for breaking changes and quality regressions"
```

## Output Format

### Review Report Structure
```
## Code Review Summary

### Severity Breakdown
- 🔴 Critical: X issues
- 🟠 High: X issues
- 🟡 Medium: X issues
- 🟢 Low: X issues

### Categories
#### Security Vulnerabilities
- [ISSUE-001] SQL Injection vulnerability in user input handling
  - Location: `src/auth/login.rs:45`
  - Severity: Critical
  - Recommendation: Use parameterized queries

#### Performance Issues
- [ISSUE-002] Inefficient database query in hot path
  - Location: `src/api/user_handler.rs:123`
  - Severity: High
  - Impact: N+1 query problem
  - Fix: Implement batch loading

#### Code Quality
- [ISSUE-003] Missing error handling for file operations
  - Location: `src/utils/file_ops.rs:67`
  - Severity: Medium
  - Recommendation: Add proper error propagation

### Automated Fixes Available
- [FIX-001] Add input sanitization
- [FIX-002] Implement query optimization
- [FIX-003] Add error handling wrapper
```

### JSON Output (for CI/CD)
```json
{
  "summary": {
    "total_issues": 15,
    "critical": 2,
    "high": 4,
    "medium": 6,
    "low": 3,
    "auto_fixable": 8
  },
  "issues": [
    {
      "id": "ISSUE-001",
      "category": "security",
      "severity": "critical",
      "file": "src/auth/login.rs",
      "line": 45,
      "description": "SQL Injection vulnerability",
      "recommendation": "Use parameterized queries",
      "auto_fix_available": true
    }
  ],
  "metrics": {
    "complexity_score": 4.2,
    "test_coverage_estimate": 85,
    "maintainability_index": 72
  }
}
```

## Progressive Disclosure

### Level 1: Quick Scan
Basic syntax, security vulnerabilities, critical issues only.

### Level 2: Standard Review
Full analysis including performance, maintainability, best practices.

### Level 3: Deep Analysis
Advanced patterns, architectural issues, long-term maintainability.

## Configuration

### Review Rules Customization
```json
{
  "rules": {
    "security": {
      "enabled": true,
      "severity_threshold": "medium"
    },
    "performance": {
      "enabled": true,
      "complexity_threshold": 10
    },
    "maintainability": {
      "enabled": true,
      "min_test_coverage": 80
    }
  },
  "languages": ["rust", "python", "typescript"],
  "output_format": "markdown"
}
```

## Integration Points

### GitHub Actions
```yaml
- name: Code Review
  uses: openai/codex-code-review@v1
  with:
    paths: 'src/'
    fail_on: 'critical'
    output_format: 'sarif'
```

### VS Code Extension
```json
{
  "contributes": {
    "commands": [
      {
        "command": "codex.codeReview",
        "title": "Run Code Review"
      }
    ]
  }
}
```

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
- [OWASP Code Review Guide](https://owasp.org/www-pdf-archive/OWASP_Code_Review_Guide_v2.pdf)
- [Microsoft SDL](https://www.microsoft.com/en-us/security/blog/2019/05/14/sdl-migration/)
- [Rust Security Guidelines](https://rustsec.org/)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-code-reviewer-skill`
**Version**: 2.1.0
**Compatibility**: Codex v2.9.0+
