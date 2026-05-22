# Upstream Overlay Merge Report

- Baseline ref: `24faf49b2a70f8813e522afb0e701add9b15b0bd`
- Upstream ref: `upstream/main`
- Applied: `yes`
- Planned paths: **59**

## Actions

- `checkout-upstream`: **51**
- `merge-file`: **6**
- `skip`: **2**

## Conflicts

- `.github/workflows/ci.yml`
- `MODULE.bazel.lock`
- `codex-cli/scripts/build_npm_package.py`
- `codex-rs/Cargo.toml`

## Planned Paths

- `.github/workflows/ci.yml`: `merge-file` (upstream-first)
- `MODULE.bazel.lock`: `merge-file` (manual)
- `codex-cli/scripts/README.md`: `checkout-upstream` (upstream-first)
- `codex-cli/scripts/build_npm_package.py`: `merge-file` (upstream-first)
- `codex-cli/scripts/install_native_deps.py`: `skip` (upstream-first)
- `codex-rs/Cargo.lock`: `merge-file` (upstream-first)
- `codex-rs/Cargo.toml`: `merge-file` (upstream-first)
- `codex-rs/app-server-transport/src/transport/remote_control/tests.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/app-server-transport/src/transport/remote_control/websocket.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/app-server/src/lib.rs`: `merge-file` (upstream-plus-reinject)
- `codex-rs/app-server/tests/suite/v2/thread_list.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core-plugins/src/loader.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core-skills/src/loader.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core-skills/src/loader_tests.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core-skills/src/manager_tests.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core/src/context_manager/history.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core/src/hook_runtime.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core/src/tools/handlers/extension_tools.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core/src/tools/handlers/mcp.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core/src/tools/router_tests.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core/tests/suite/rmcp_client.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/core/tests/suite/subagent_notifications.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/ext/extension-api/src/lib.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/ext/goal/tests/goal_extension_backend.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/ext/memories/src/tests.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/schema/generated/permission-request.command.input.schema.json`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/schema/generated/post-compact.command.input.schema.json`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/schema/generated/post-tool-use.command.input.schema.json`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/schema/generated/pre-compact.command.input.schema.json`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/schema/generated/pre-tool-use.command.input.schema.json`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/schema/generated/user-prompt-submit.command.input.schema.json`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/engine/mod_tests.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/events/common.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/events/compact.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/events/permission_request.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/events/post_tool_use.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/events/pre_tool_use.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/events/user_prompt_submit.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/lib.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/hooks/src/schema.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/login/src/auth/manager.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/model-provider/src/amazon_bedrock/mantle.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/network-proxy/src/proxy.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/plugin/src/load_outcome.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/rmcp-client/src/bin/test_stdio_server.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/rollout/Cargo.toml`: `checkout-upstream` (upstream-first)
- `codex-rs/rollout/src/search.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tools/Cargo.toml`: `checkout-upstream` (upstream-first)
- `codex-rs/tools/src/json_schema.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tools/src/json_schema_tests.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tools/src/lib.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tools/src/tool_call.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tui/src/app.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tui/src/app/config_persistence.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tui/src/app/event_dispatch.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tui/src/app/tests.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/tui/src/config_update.rs`: `checkout-upstream` (upstream-first)
- `codex-rs/utils/plugins/src/lib.rs`: `checkout-upstream` (upstream-first)
- `scripts/stage_npm_packages.py`: `skip` (keep-fork)
