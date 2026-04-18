---
name: deepresearch
description: Continue the fork's DeepResearch workflow through the official Codex plugin surface instead of the retired GUI.
---

# DeepResearch

Use this skill when the user wants the old DeepResearch experience but the implementation should follow the official Codex app and plugin model.

Guidelines:

- Prefer `plugin/list`, `plugin/read`, and plugin mentions over any legacy GUI route.
- Keep the existing DeepResearch backend logic where it still adds value, but route user-facing workflow through the plugin surface.
- Preserve citation-oriented research behavior.
- If the environment does not expose optional research providers, degrade to the available official browsing and app-server capabilities instead of failing.

Do not revive:

- `gui-x`
- custom browser shells
- virtual OS affordances
