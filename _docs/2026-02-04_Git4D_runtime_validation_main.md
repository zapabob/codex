# Git4D Runtime Validation (main)

- Date: 2026-02-04
- Worktree: main

## Subagent setup
- Added .codex/agents/git4d-runtime-checker.yaml
- Added .codex/agents/git4d-schema-auditor.yaml

## Runtime validation
- Set CODEX_GUI_DB_URL to sqlite://C:/Users/downl/Desktop/codex-main/_tmp/codex-gui.db
- Started codex-gui via cargo run -p codex-gui
- GET /api/health -> {"status":"ok"}
- POST /api/visualization/git4d (mode=desktop, repositoryPath=C:\\Users\\downl\\Desktop\\codex-main)
  -> sessionId=2479659f-9df1-4e8f-b6ed-b6979023f5e5, status=started, platform=Desktop
- GET /api/visualization/git4d/sessions -> session listed with status=Starting

## Runtime validation (VR/AR)
- GET /api/health -> {"status":"ok"}
- POST /api/visualization/git4d (mode=desktop, repositoryPath=C:\\Users\\downl\\Desktop\\codex-main)
  -> sessionId=56daadc0-85ad-4983-a5b8-a39785374b98, status=started, platform=Desktop
- POST /api/visualization/git4d (mode=vr, repositoryPath=C:\\Users\\downl\\Desktop\\codex-main)
  -> sessionId=f6933fbb-cc96-4577-86a7-81afca76765d, status=started, platform=WebXR, deviceName=WebXR
- POST /api/visualization/git4d (mode=ar, repositoryPath=C:\\Users\\downl\\Desktop\\codex-main)
  -> sessionId=9266c89b-53fc-44f2-b759-2dd7383caa2f, status=started, platform=WebXR, deviceName=WebXR
- GET /api/visualization/git4d/sessions -> 3 sessions listed (desktop/vr/ar)

## CUDA/GPU validation
- cargo check -p codex-core --features cuda: success
- cargo test -p codex-core --features cuda test_gpu_validation_suite_creation started but stalled; terminated cargo PIDs 1960/13152.
- Added quick-mode test (test_gpu_validation_suite_creation_quick) with reduced timeouts and benchmarks.
- cargo test -p codex-core --features cuda test_gpu_validation_suite_creation_quick started but stalled during compile; terminated cargo PIDs 13980/36908.

## GUI updates
- Git4DVisualization now launches backend session automatically and displays backend status, session, platform, and SSE event stream.
- Git4D page shows session ID chip when available.

## Cleanup
- Stopped codex-gui after validation
- Removed _tmp/codex-gui.db

## Next actions
- If VR/AR device is available, repeat launch with mode=vr/ar
- Validate CUDA visualization path on GPU host
- Consider exposing Git4D session status updates in GUI

