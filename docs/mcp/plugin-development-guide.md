# MCP Plugin Development Guide

## Overview

This guide explains how to develop and deploy MCP server plugins for Codex's dynamic loading system.

## Plugin Structure

A plugin is a directory containing:

```
.codex/mcp-plugins/
└── my-plugin/
    ├── plugin.toml      # Plugin metadata
    └── server.toml       # MCP server configuration
```

## Plugin Metadata (`plugin.toml`)

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "Description of what this plugin does"
author = "Your Name"
enabled = true
```

## Server Configuration (`server.toml`)

The `server.toml` file follows the standard MCP server configuration format:

```toml
# For stdio transport
command = "node"
args = ["path/to/server.js"]
env = { API_KEY = "${MY_API_KEY}" }
enabled = true

# For streamable HTTP transport
# url = "https://example.com/mcp"
# bearer_token_env_var = "BEARER_TOKEN"
```

## Example Plugin

### GitHub Integration Plugin

**`.codex/mcp-plugins/github-plugin/plugin.toml`**:
```toml
[plugin]
name = "github-plugin"
version = "1.0.0"
description = "GitHub API integration for repository management"
author = "zapabob"
enabled = true
```

**`.codex/mcp-plugins/github-plugin/server.toml`**:
```toml
command = "npx"
args = ["-y", "@modelcontextprotocol/server-github"]
env = { GITHUB_PERSONAL_ACCESS_TOKEN = "${GITHUB_TOKEN}" }
enabled = true
startup_timeout_sec = 30.0
tool_timeout_sec = 60.0
```

## Plugin Loading

Plugins are automatically discovered and loaded when:

1. Codex starts (if `mcp_dynamic_loading.enabled = true`)
2. A plugin is added to the plugin directory
3. File watcher detects changes (if `mcp_dynamic_loading.watch_config_file = true`)

## Plugin Management

### Enable/Disable Plugin

Edit `plugin.toml`:
```toml
[plugin]
enabled = false  # Disable plugin
```

### Reload Plugin

After modifying `server.toml`, the plugin will be automatically reloaded if file watching is enabled.

## Best Practices

1. **Versioning**: Use semantic versioning for plugin versions
2. **Documentation**: Include clear descriptions in `plugin.toml`
3. **Error Handling**: Ensure your MCP server handles errors gracefully
4. **Environment Variables**: Use `${VAR_NAME}` syntax for secrets
5. **Testing**: Test plugins in isolation before deploying

## Troubleshooting

### Plugin Not Loading

- Check `plugin.toml` syntax
- Verify `server.toml` configuration
- Check Codex logs for errors
- Ensure `enabled = true` in `plugin.toml`

### Plugin Errors

- Check MCP server logs
- Verify environment variables are set
- Ensure command path is correct
- Check network connectivity (for HTTP transports)
