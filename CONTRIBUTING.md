# Contributing to Codex (zapabob fork)

Thank you for your interest in contributing to Codex! This document provides guidelines for contributing to the zapabob fork while maintaining compatibility with the upstream OpenAI/codex repository.

## 🌟 Contribution Philosophy

### 1. Dual Focus

- **Upstream Compatibility**: Maintain compatibility with OpenAI/codex
- **zapabob Extensions**: Enhance with unique features

### 2. Quality Standards

- Follow existing code style and conventions
- Add tests for new features
- Update documentation
- Ensure Apache 2.0 license compliance

## 🔄 Repository Structure

### Upstream (OpenAI/codex)

```
Public repository: https://github.com/openai/codex
```

### zapabob Fork

```
Fork repository: https://github.com/zapabob/codex
Branch strategy:
- main: zapabob独自 + 公式統合
- upstream-sync: 公式リポジトリ同期用
- feature/zapabob-*: 独自機能開発
```

## 🚀 Getting Started

### 1. Fork and Clone

```bash
# Fork the zapabob repository on GitHub
git clone https://github.com/YOUR_USERNAME/codex.git
cd codex

# Add upstream remote
git remote add upstream https://github.com/openai/codex.git
git fetch upstream
```

### 2. Set Up Development Environment

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
cd codex-rs
cargo build

# Run tests
cargo test
```

### 3. Create a Branch

```bash
# For zapabob-specific features
git checkout -b feature/zapabob-your-feature

# For upstream compatibility fixes
git checkout -b fix/upstream-compatibility
```

## 📝 Contribution Types

### 1. zapabob-Specific Features

**Examples**: Gemini CLI integration, Marisa voice notifications

#### Guidelines

- Place code in `zapabob/` or `codex-rs/{module}-zapabob/` directories
- Add `// zapabob:` comments to mark custom code
- Update `CHANGELOG.md` under "zapabob独自機能"
- Add comprehensive documentation

#### Example

```rust
// zapabob: 独自実装 - Gemini CLI MCP統合
pub struct GeminiSearchProvider {
    model: String,
    use_mcp: bool,
}
```

### 2. Upstream Compatibility

**Examples**: Bug fixes, performance improvements

#### Guidelines

- Keep changes minimal and focused
- Consider contributing to upstream
- Mark compatibility code clearly
- Test against upstream main branch

#### Example

```rust
// From OpenAI/codex upstream
// Compatible with upstream v0.48.0
pub struct OriginalComponent {
    // ...
}
```

### 3. Documentation

**Examples**: README updates, guides, tutorials

#### Guidelines

- Support both English and Japanese
- Update both language sections
- Add examples and use cases
- Keep formatting consistent

## 🔧 Development Guidelines

### Code Style

#### Rust

```rust
// Use standard Rust formatting
cargo fmt

// Check with Clippy
cargo clippy -- -D warnings

// Follow naming conventions
// zapabob-specific: suffix with _zapabob or place in zapabob/ module
```

#### PowerShell

```powershell
# Use verb-noun naming
# Add license header
# Follow Microsoft style guide
```

#### Bash

```bash
#!/bin/bash
# Use POSIX compatibility when possible
# Add license header
# Follow Google Shell Style Guide
```

### Testing

#### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zapabob_feature() {
        // Test zapabob-specific features
    }
}
```

#### Integration Tests

```rust
// codex-rs/tests/integration_test.rs
#[test]
fn test_upstream_compatibility() {
    // Test compatibility with upstream
}
```

### Documentation

#### Code Comments

````rust
/// zapabob: Gemini CLI MCP統合
///
/// # Examples
///
/// ```
/// let provider = GeminiSearchProvider::new(None);
/// let results = provider.search("query").await?;
/// ```
pub struct GeminiSearchProvider {
    // ...
}
````

#### Documentation Files

```markdown
# Place in \_docs/ directory

# Use clear headings

# Include examples

# Support Japanese and English
```

## 📋 Commit Messages

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature (zapabob-specific or upstream)
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code formatting
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance

### Examples

```bash
# zapabob-specific feature
feat(gemini-cli): Add MCP server integration

Implement Gemini CLI MCP server with OAuth 2.0 authentication
and Google Search Grounding support.

zapabob: Independent feature
```

```bash
# Upstream compatibility
fix(core): Fix compatibility with upstream v0.48.0

Update API calls to match upstream interface changes.

Upstream: Compatible with OpenAI/codex v0.48.0
```

## 🔍 Pull Request Process

### 1. Before Submitting

- [ ] Code follows style guidelines
- [ ] Tests pass locally
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] License headers are present

### 2. PR Template

```markdown
## Description

Brief description of changes

## Type of Change

- [ ] zapabob-specific feature
- [ ] Upstream compatibility
- [ ] Bug fix
- [ ] Documentation

## Testing

How to test these changes

## Checklist

- [ ] Tests pass
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] License headers present
```

### 3. Review Process

1. Automated checks run
2. Maintainer review
3. Community feedback
4. Approval and merge

## 🏗️ Architecture Guidelines

### Directory Structure

```
codex/
├── codex-rs/                    # Rust core
│   ├── gemini-cli-mcp-server/  # zapabob: Gemini CLI
│   ├── deep-research/          # Extended with Gemini
│   └── core/                   # Upstream + extensions
├── zapabob/                     # zapabob-specific
│   ├── scripts/                # Custom scripts
│   ├── docs/                   # Custom docs
│   └── assets/                 # Custom assets
├── _docs/                       # Implementation logs
└── .github/                     # GitHub workflows
```

### Module Organization

- Keep zapabob-specific code separate
- Use clear module boundaries
- Document interfaces
- Maintain backward compatibility

## 📄 License

### Apache 2.0 License

All contributions must be licensed under Apache 2.0.

### License Header

Add to all new files:

```rust
// Copyright 2025 zapabob
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
```

## 🤝 Code of Conduct

### Be Respectful

- Respect all contributors
- Be constructive in feedback
- Welcome newcomers

### Be Professional

- Keep discussions technical
- Avoid personal attacks
- Focus on the code

### Be Collaborative

- Share knowledge
- Help others learn
- Celebrate successes

## 📞 Contact

### GitHub Issues

- Bug reports
- Feature requests
- Questions

### Discussions

- General discussions
- Ideas
- Community support

### Maintainer

- GitHub: [@zapabob](https://github.com/zapabob)
- Issues: https://github.com/zapabob/codex/issues

## 🎯 Contribution Areas

### High Priority

- [ ] Upstream compatibility maintenance
- [ ] Security fixes
- [ ] Documentation improvements
- [ ] Test coverage

### Medium Priority

- [ ] New zapabob features
- [ ] Performance optimizations
- [ ] UI/UX improvements

### Low Priority

- [ ] Code style improvements
- [ ] Minor refactoring
- [ ] Non-critical features

## 📚 Resources

### Documentation

- [README.md](README.md)
- [CHANGELOG.md](CHANGELOG.md)
- [公式リポジトリとの整合性管理](_docs/公式リポジトリとの整合性管理.md)

### External

- [OpenAI/codex](https://github.com/openai/codex)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0)

---

**Thank you for contributing to Codex!** 🎉

Your contributions help make this project better for everyone while maintaining harmony with the upstream OpenAI/codex project.
