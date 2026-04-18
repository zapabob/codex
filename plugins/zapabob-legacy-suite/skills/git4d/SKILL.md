---
name: git4d
description: Carry Git4D forward as an optional plugin capability with a text-first fallback.
---

# Git4D

Use this skill for Git4D-style repository exploration after the GUI migration.

Guidelines:

- Prefer official app-server and plugin discovery flows before any bespoke visualization path.
- When the lightweight GUI bridge is available, prefer its live launch, session, SSE, and capability routes before dropping to text-only summaries.
- Treat 3D or immersive rendering as optional. If the runtime does not support it, provide the same insight through repository summaries, graph descriptions, or structured data.
- Keep backend-only Git4D value when it does not conflict with upstream APIs.

Do not depend on:

- `codex-gui-x`
- legacy WebXR-only routes
- retired virtual OS windows
