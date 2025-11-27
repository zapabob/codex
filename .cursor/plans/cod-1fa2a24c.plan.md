<!-- 1fa2a24c-271c-4a0d-81f8-29e9b344b8ab 3f591ee8-ad87-4638-a97f-2d2bab12a865 -->
# Codex Core Compile Fix & Type Restoration Plan

1. **Restore Core Dependencies**  

- Re-enable required workspace crates in [codex-rs/Cargo.toml](codex-rs/Cargo.toml) (`codex-protocol`, `codex-rmcp-client`, `codex-otel`, other commented Codex crates) and ensure feature flags are configured for Windows-friendly builds.  
- Run `cargo metadata` (read-only) to confirm dependency graph resolves without conflicts.

2. **Revive Removed Modules**  

- For each `.bak` module under `core/src` (e.g., `client.rs.bak`, `client_common.rs.bak`, `model_provider_info.rs.bak`, `codex_conversation.rs.bak`), restore them to their original filenames, re-export in [core/src/lib.rs](codex-rs/core/src/lib.rs), and resolve any import drift introduced since their removal.

3. **Rewire AgentRuntime Types**  

- Revert `AgentRuntime` in [core/src/agents/runtime.rs](codex-rs/core/src/agents/runtime.rs) to use the proper types (`Config`, `AuthManager`, `OtelEventManager`, `ModelProviderInfo`, `CollaborationStore`, `ConversationId`, `ReasoningEffort`, etc.).  
- Reconnect helper logic for LLM streaming (`ModelClient`, `ResponseItem`, `ContentItem`, `ResponseEvent`) and audit logging (`log_audit_event`, `AuditEvent`, `AuditEventType`, `ExecutionStatus`).  
- Ensure dependent modules (plan executor, orchestration, async subagent integration) import the restored types correctly.

4. **Implement Missing Stubs with Best Practices**  

- Audit any interim placeholder logic added during prior fixes (e.g., mock responses, hard-coded strings). Replace with real implementations leveraging the restored modules, adding error handling and tracing aligned with existing `tracing` conventions.

5. **Lint & Warning Cleanup**  

- Run `just fmt` for formatting, then `just fix -p codex-core` to auto-apply Clippy suggestions.  
- Manually address remaining warnings (unused imports, missing docs) to achieve “warnings = 0” for `codex-core`.

6. **Deep Validation & Tests**  

- Execute `cargo check -p codex-core` followed by `cargo test -p codex-core` (or targeted suites) to confirm the resolved types compile and runtime tests pass.  
- Document the restored functionality and compiler-clean build in a new `_docs` entry (timestamped) summarizing the fixes, referencing the re-enabled modules and tests run.

### To-dos

- [ ] CLI/TUI/GUI統合テストの実行
- [ ] 安定版リリース準備の完了
- [ ] 継続的なテスト自動化の実装
- [ ] 未解決型参照58件の解消と型定義整備
- [ ] 警告0に向けたlint対応