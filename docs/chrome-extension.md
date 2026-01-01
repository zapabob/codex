# Codex Chrome Extension (Native Messaging)

This document describes how to run the Codex Chrome/Edge extension with the native messaging host and the `/chrome` CLI command.

## Build the native host

From the repo root:

```bash
cargo build -p codex-chrome-host --release
```

The binary path will be:

- Windows: `codex-rs\target\release\codex-chrome-host.exe`
- macOS/Linux: `codex-rs/target/release/codex-chrome-host`

## Load the extension

1. Open Chrome or Edge.
2. Enable Developer Mode in `chrome://extensions` or `edge://extensions`.
3. Click **Load unpacked** and select `extensions/chrome-codex`.
4. Copy the extension ID shown in the extensions list.

## Register the native host

### Windows (Chrome or Edge)

```powershell
# Chrome
./extensions/chrome-codex/install-host.ps1 -ExtensionId <EXTENSION_ID>

# Edge
./extensions/chrome-codex/install-host.ps1 -ExtensionId <EXTENSION_ID> -Edge
```

### macOS / Linux

```bash
# Chrome
./extensions/chrome-codex/install-host.sh <EXTENSION_ID>

# Edge
./extensions/chrome-codex/install-host.sh --edge <EXTENSION_ID>
```

## Security and guardrails

- Natural language instructions are parsed into structured intents and validated.
- High-risk actions (posting, login, shopping) require explicit confirmation.
- Domain guards are enforced:
  - `post_social` only on `x.com` or `twitter.com`
  - `post_article` only on `note.com`, `qiita.com`, or `zenn.dev`
  - shopping intents only on `amazon.com`, `amazon.co.jp`, `mercari.com`, `mercari.jp`, or `auctions.yahoo.co.jp`
- No credential storage. Login automation is limited to click/field fill only.
- Code execution (`eval`) is exposed but should only be run after user confirmation.
- Image uploads require manual file selection in the browser.

## Message types (native host)

- `deep_research.request` - Run deep research query
- `nl_command.request` - Parse natural language command
- `dom.read.request` - Read DOM from active tab (requires extension)
- `console.get_logs.request` - Get console logs from active tab (requires extension)
- `network.get_logs.request` - Get network request logs from active tab (requires extension)
- `codegen.request` - Code generation (not yet implemented)
- `ping` - Connection test

Responses are returned as `*.response` payloads with `success` and `data` fields.

## CLI Usage Notes

When using CLI commands that require the extension (dom, console, network):

1. **Extension must be active**: The Chrome extension must be installed and active
2. **Native host connection**: The extension must be able to connect to the native messaging host
3. **Timeout**: Requests timeout after 30 seconds if no response is received
4. **Error handling**: Detailed error messages are provided if the extension is not connected or the request fails

The CLI spawns the native messaging host process and communicates with it directly. However, for DOM reading, console log retrieval, and network monitoring, the extension's content script and background script must be active to perform the actual operations.

## MCP Bridge Server (Experimental)

An MCP (Model Context Protocol) bridge server is available as an alternative communication method:

### Building the MCP Bridge Server

From the repo root:

```bash
cargo build -p codex-chrome-mcp-bridge --release
```

### Running the MCP Bridge Server

**stdio mode (for CLI):**
```bash
codex-chrome-mcp-bridge stdio
```

**HTTP mode (for extension):**
```bash
codex-chrome-mcp-bridge http 8788
```

### Using MCP Bridge

The CLI will automatically try to use the MCP bridge if available, falling back to native messaging host if the bridge is not found. The extension can connect to the MCP bridge via streamable HTTP for more robust communication.

## CLI Commands

The Codex CLI now supports Chrome extension integration commands:

```bash
# Parse natural language instruction
codex chrome parse --utterance "click the login button" --url "https://example.com"

# Run deep research
codex chrome research "Rust async best practices" --depth 3 --breadth 10

# Read DOM from active tab (requires extension)
# Note: Requires Chrome extension to be active and connected to native messaging host
codex chrome dom --selector "#main-content"

# Get console logs (requires extension)
# Note: Requires Chrome extension to be active and connected to native messaging host
codex chrome console --filter "error" --limit 50

# Monitor network requests (requires extension)
# Note: Requires Chrome extension to be active and connected to native messaging host
codex chrome network --filter "api" --limit 50
```

## Example CLI usage

Parse a natural language instruction manually:

```bash
echo '{"utterance":"click the login button"}' | codex chrome parse --json
```
