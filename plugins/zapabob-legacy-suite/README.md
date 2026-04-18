# Zapabob Legacy Suite

This repo-local plugin keeps the fork's unique value on top of the official Codex app and plugin model.

Included migration surfaces:

- `deepresearch`: official-plugin-facing DeepResearch guidance
- `git4d`: Git4D workflow guidance with graceful non-visual fallback
- `vr-ar`: VR and AR workflow guidance with no-device and no-WebXR fallback
- `legacy-suite-fallbacks`: plugin-local MCP server for DeepResearch briefing, Git4D repository summaries, and VR or AR capability reports

Not carried forward:

- Legacy `gui-x`
- Virtual OS shells
- Fork-only computer or OS control surfaces

Bundled CodexApp integrations:

- GitHub connector for Git4D and repository-first workflows
- Hugging Face connector for DeepResearch model, dataset, and paper lookup
- Vercel connector for browser-delivered visualization or prototype follow-through

Use `plugin/list`, `plugin/read`, and `plugin/install` from `codex app-server`, or mention the plugin as `plugin://zapabob-legacy-suite@zapabob-repo-local`.
