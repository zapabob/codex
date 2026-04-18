# Repository Structure

Last updated: 2026-04-17

## Top Level

```text
codex-main/
|- codex-rs/                  Rust workspace for CLI, TUI, app-server, protocol, plugins, and retained backend logic
|- codex-cli/                 Node-facing CLI packaging and wrappers
|- .agents/plugins/           Tracked repo-local marketplace definitions
|- plugins/                   Repo-local plugin bundles
|- scripts/                   Upstream sync, validation, maintenance, and packaging scripts
|- docs/                      Product, migration, and plan documentation
|- _docs/                     Implementation logs and deletion rationale
|- gui/                       Legacy GUI tree in migration mode
|- codex-gui-x/               Legacy GUI tree in migration mode
|- extensions/                Editor and visualization extensions under review for retirement or migration
```

## Official-First Runtime

The authoritative runtime story is:

- `codex-rs/cli`: CLI entrypoints, including `codex`, `codex app-server`, and deprecated `gui-x` migration guidance
- `codex-rs/tui`: terminal interface
- `codex-rs/app-server`: rich-client protocol server
- `codex-rs/app-server-protocol`: generated protocol types and schemas
- `codex-rs/protocol`: shared protocol contracts
- `codex-rs/core/src/plugins`: plugin discovery, marketplace loading, manifest parsing, mentions, and install state

## Repo-Local Plugin Layout

The repo-local plugin migration path is tracked in-tree:

```text
.agents/plugins/
`- marketplace.json           Repo-local marketplace entrypoint

plugins/
`- zapabob-legacy-suite/
   |- .codex-plugin/plugin.json
   |- .mcp.json
   |- .app.json
   `- skills/
      |- deepresearch/
      |- git4d/
      `- vr-ar/
```

Use this bundle when the fork's old DeepResearch, Git4D, or VR or AR features need to remain available without reviving the legacy GUI as a first-party product surface.

## Migration-Mode Paths

These trees are being retired or migrated and should not receive new product-facing features:

- `gui/`
- `codex-gui-x/`
- `codex-rs/gui`
- `codex-rs/tauri-gui`

These paths are expected to move toward one of two outcomes:

- `plugin-migrate`: user-facing behavior is rehomed into plugin and app-server seams
- `retire-after-parity`: the feature is removed after parity is proven elsewhere

## Deprecated Surfaces

The following fork-only directions are deprecated:

- custom virtual OS flows
- custom computer-operation surfaces
- fork-specific OS-control entrypoints

When a user asks for those capabilities, point them toward official Codex App evolution instead of extending the deprecated paths.
