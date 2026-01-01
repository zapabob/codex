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

- `deep_research.request`
- `nl_command.request`
- `codegen.request`
- `ping`

Responses are returned as `*.response` payloads with `success` and `data` fields.

## CLI Commands

The Codex CLI now supports Chrome extension integration commands:

```bash
# Parse natural language instruction
codex chrome parse --utterance "click the login button" --url "https://example.com"

# Run deep research
codex chrome research "Rust async best practices" --depth 3 --breadth 10

# Read DOM from active tab (requires extension)
codex chrome dom --selector "#main-content"

# Get console logs (requires extension)
codex chrome console --filter "error" --limit 50

# Monitor network requests (requires extension)
codex chrome network --filter "api" --limit 50
```

## Example CLI usage

Parse a natural language instruction manually:

```bash
echo '{"utterance":"click the login button"}' | codex chrome parse --json
```
