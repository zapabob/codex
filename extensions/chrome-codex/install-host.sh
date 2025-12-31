#!/usr/bin/env bash
set -euo pipefail

BROWSER="chrome"
if [[ "${1:-}" == "--edge" ]]; then
  BROWSER="edge"
  shift
fi

if [[ $# -lt 1 ]]; then
  echo "Usage: install-host.sh [--edge] <EXTENSION_ID> [HOST_PATH]" >&2
  exit 1
fi

EXTENSION_ID="$1"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
HOST_PATH="${2:-$REPO_ROOT/codex-rs/target/release/codex-chrome-host}"

if [[ ! -f "$HOST_PATH" ]]; then
  echo "Host binary not found: $HOST_PATH" >&2
  exit 1
fi

if [[ "$(uname)" == "Darwin" ]]; then
  TEMPLATE="$SCRIPT_DIR/native-messaging-host-macos.json"
  if [[ "$BROWSER" == "edge" ]]; then
    TARGET_DIR="$HOME/Library/Application Support/Microsoft Edge/NativeMessagingHosts"
  else
    TARGET_DIR="$HOME/Library/Application Support/Google/Chrome/NativeMessagingHosts"
  fi
else
  TEMPLATE="$SCRIPT_DIR/native-messaging-host-linux.json"
  if [[ "$BROWSER" == "edge" ]]; then
    TARGET_DIR="$HOME/.config/microsoft-edge/NativeMessagingHosts"
  else
    TARGET_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"
  fi
fi

if [[ ! -f "$TEMPLATE" ]]; then
  echo "Template not found: $TEMPLATE" >&2
  exit 1
fi

mkdir -p "$TARGET_DIR"
OUTPUT_PATH="$TARGET_DIR/com.codex.chrome.json"

sed \
  -e "s|__HOST_PATH__|$HOST_PATH|g" \
  -e "s|__EXTENSION_ID__|$EXTENSION_ID|g" \
  "$TEMPLATE" > "$OUTPUT_PATH"

chmod 644 "$OUTPUT_PATH"

echo "Registered native host at $OUTPUT_PATH"
