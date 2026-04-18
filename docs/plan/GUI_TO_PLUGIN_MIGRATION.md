# GUI To Plugin Migration

## Why

This fork no longer treats the custom GUI as a first-party product surface. Official Codex app and app-server flows are now authoritative, and fork-only UX value is carried through plugins.

## Replacement Map

- legacy GUI launcher: `codex gui-x`
  replacement: `codex app` or `codex app-server` plus plugin discovery

- DeepResearch GUI panels
  replacement: `zapabob-legacy-suite` plugin skill and mention flow

- Git4D visualization pages
  replacement: plugin capability with text-first fallback

- VR or AR pages
  replacement: plugin capability with no-device and no-WebXR fallback

- virtual OS and computer-control panels
  replacement: none in-tree; defer to future official Codex App platform work

## Repo-Local Plugin

Tracked files:

- [`.agents/plugins/marketplace.json`](/C:/Users/downl/Desktop/codex-main/.agents/plugins/marketplace.json)
- [`plugins/zapabob-legacy-suite/.codex-plugin/plugin.json`](/C:/Users/downl/Desktop/codex-main/plugins/zapabob-legacy-suite/.codex-plugin/plugin.json)

Plugin mention path:

- `plugin://zapabob-legacy-suite@zapabob-repo-local`

## Parity Criteria

Before deleting legacy GUI trees, confirm:

- `plugin/list` discovers the repo-local plugin
- `plugin/read` exposes the migration bundle metadata and skills
- `plugin/install` can install from the repo-local marketplace
- mention-based invocation works for the plugin
- DeepResearch remains available
- Git4D and VR or AR degrade cleanly when the runtime lacks optional support

## Non-Goals

This migration does not preserve:

- custom OS emulation
- fork-only computer-operation shells
- GUI-only transport layers that bypass official plugin or app-server seams
