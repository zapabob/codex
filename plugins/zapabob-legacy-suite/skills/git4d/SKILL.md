---
name: git4d
description: Carry Git4D forward as an optional plugin capability with a text-first fallback.
---

# Git4D

Use this skill for Git4D-style repository exploration after the GUI migration.

Guidelines:

- Prefer official app-server and plugin discovery flows before any bespoke visualization path.
- Prefer `git4d/capabilities/read`, `git4d/session/start`, `git4d/session/list`, and `git4d/session/watch` on `codex app-server`.
- Use the lightweight GUI bridge only as a compatibility adapter when app-server is unavailable.
- Treat 3D or immersive rendering as optional. If the runtime does not support it, provide the same insight through repository summaries, graph descriptions, or structured data.
- Keep backend-only Git4D value when it does not conflict with upstream APIs.

Do not depend on:

- `codex-gui-x`
- legacy WebXR-only routes as a primary integration surface
- retired virtual OS windows
