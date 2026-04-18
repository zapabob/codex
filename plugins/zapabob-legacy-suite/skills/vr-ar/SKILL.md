---
name: vr-ar
description: Carry VR and AR forward as an optional plugin capability with graceful no-device fallback.
---

# VR or AR

Use this skill when the user asks for the fork's old VR or AR capabilities and we need to keep the experience aligned with the official Codex app and plugin architecture.

Guidelines:

- Present VR or AR as optional plugin-provided capability, not a first-party GUI requirement.
- If WebXR, device access, or graphics support is unavailable, fall back to non-immersive descriptions and standard app-server flows.
- Keep device-specific logic isolated from the main product story.
- Route computer control and OS integration requests toward future official Codex App Windows support rather than reviving fork-only surfaces.
