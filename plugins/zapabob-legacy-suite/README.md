# Zapabob Legacy Suite

This repo-local plugin keeps the fork's unique value on top of the official Codex app and plugin model.
It is now positioned as an official-surface bridge: when the lightweight Git4D GUI service is available, the plugin can point users at live launch, session, SSE, and capability routes instead of treating everything as text-only fallback.

Included migration surfaces:

- `deepresearch`: official-plugin-facing DeepResearch guidance
- `git4d`: Git4D workflow guidance with live bridge support plus graceful non-visual fallback
- `vr-ar`: VR and AR workflow guidance with live capability reporting plus no-device and no-WebXR fallback
- `legacy-suite-fallbacks`: plugin-local MCP server for DeepResearch briefing, Git4D bridge summaries, live session inspection, launch requests, and VR or AR capability reports

Not carried forward:

- Legacy `gui-x`
- Virtual OS shells
- Fork-only computer or OS control surfaces

Bundled CodexApp integrations:

- GitHub connector for Git4D and repository-first workflows
- Hugging Face connector for DeepResearch model, dataset, and paper lookup
- Vercel connector for browser-delivered visualization or prototype follow-through

When `codex-rs/gui` is running, the plugin can bridge into:

- `POST /api/visualization/git4d`
- `GET /api/visualization/git4d/sessions`
- `GET /api/visualization/git4d/capabilities/{mode}`
- `GET /api/visualization/git4d/{session_id}/events`

Set `CODEX_GUI_BASE_URL` if the service is not using the default `http://127.0.0.1:8787`.

Use `plugin/list`, `plugin/read`, and `plugin/install` from `codex app-server`, or mention the plugin as `plugin://zapabob-legacy-suite@zapabob-repo-local`.
