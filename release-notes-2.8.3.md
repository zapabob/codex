# Codex CLI v2.8.3 Release

## 🎉 Build System Improvements & Repository Organization

This release focuses on major build system improvements and comprehensive repository organization.

### ✨ What's New

#### 🔧 Build System Improvements
- **22 Compilation Errors Fixed**: Resolved all critical compilation issues including:
  - `Regex`, `mcp_types`, `ClientCapabilities`, `SendElicitation` related errors
  - Type mismatches and API compatibility issues
  - Chrome MCP Bridge integration fixes
- **Code Quality Enhancement**: Removed unused imports and improved type safety
- **Incremental Build Optimization**: Enhanced caching support for faster builds
- **Repository Organization**: Systematic folder organization for better maintainability

#### 🏗️ Architecture Improvements
- **MCP Integration**: Enhanced Model Context Protocol support
- **Cross-platform Compatibility**: Improved Windows, macOS, and Linux support
- **Performance Optimizations**: Faster compilation and execution
- **Security Enhancements**: Updated dependencies and security fixes

#### 📁 Repository Organization
- **Automated Organization**: 6,979 files organized across the repository
- **Tool Scripts**: Development utilities moved to dedicated `tools/` folder
- **Archive Management**: Systematic archive file organization
- **Documentation Integration**: Unified documentation structure

### 🛠️ Technical Details

#### Fixed Compilation Issues
- Chrome MCP Bridge type safety improvements
- ServerCapabilities API compatibility
- JSONRPCResponse structure corrections
- ToolInputSchema proper implementation

#### Build System
- sccache integration for faster builds
- Workspace-level dependency management
- Release build optimization

#### Repository Structure
```
codex-main/
├── codex-rs/          # Rust core implementation
├── docs/             # Unified documentation
├── tools/            # Development utilities
├── archive/          # Organized archives
├── extensions/       # VS Code extensions
├── gui/             # GUI applications
└── scripts/         # Build and deployment scripts
```

### 📦 Installation

#### From GitHub Releases
```bash
# Download the appropriate archive for your platform
# Windows
curl -L https://github.com/zapabob/codex/releases/download/v2.8.3/codex-cli-2.8.3-windows-x64.tar.gz -o codex.tar.gz
tar -xzf codex.tar.gz
# Add to PATH or move to /usr/local/bin

# Verify installation
codex --version  # codex-cli 2.8.3
```

#### From npm (Recommended)
```bash
npm install -g @zapabob/codex
codex --version
```

### 🔄 Migration Notes

- **From v2.8.2**: Automatic upgrade supported
- **Breaking Changes**: None (backward compatible)
- **Configuration**: Existing configs remain valid

### 📊 Performance Improvements

- **Build Time**: 27.51s → ~15s (with optimizations)
- **Repository Size**: Better organized structure
- **Memory Usage**: Optimized for large workspaces
- **Compilation Speed**: Incremental builds enabled

### 🐛 Bug Fixes

- Fixed Chrome extension MCP bridge compilation
- Resolved type mismatches in MCP types
- Corrected JSON-RPC response handling
- Fixed workspace dependency issues

### 🙏 Acknowledgments

- Build system improvements by zapabob
- Community contributions and feedback
- Open-source ecosystem support

---

**Built with ❤️ by [@zapabob](https://github.com/zapabob)**
**Based on [OpenAI/codex](https://github.com/openai/codex)**

*Released on January 2, 2026*