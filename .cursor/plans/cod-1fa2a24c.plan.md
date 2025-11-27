<!-- 1fa2a24c-271c-4a0d-81f8-29e9b344b8ab 9c4de1db-7952-4786-bbcf-9bf60388201a -->
# Restore Tonic TLS Feature Coverage

1. **Audit Current TLS Feature Flags**  

- Review `tonic` dependency declarations in [`codex-rs/Cargo.toml`](codex-rs/Cargo.toml) and [`codex-rs/otel/Cargo.toml`](codex-rs/otel/Cargo.toml) to confirm `tls-webpki-roots` is absent and that `tls-roots` is only an alias for the native roots feature.

2. **Reintroduce WebPKI Support**  

- Update both files so `tonic` is built with `transport`, `tls-native-roots`, and `tls-webpki-roots` (keeping `tls` if needed for shared TLS code) to restore compatibility with WebPKI-signed cert chains.

3. **Validate Dependency Graph**  

- Run `cargo metadata` (or `cargo check -p codex-core`) from `codex-rs` to ensure the workspace resolves successfully with the restored feature set and no new warnings surface.

### To-dos

- [ ] CLI/TUI/GUI統合テストの実行
- [ ] 安定版リリース準備の完了
- [ ] 継続的なテスト自動化の実装
- [ ] 未解決型参照58件の解消と型定義整備
- [ ] 警告0に向けたlint対応
- [ ] CLI/TUI/GUI統合テストの実行
- [ ] 安定版リリース準備の完了
- [ ] 未解決型参照58件の解消と型定義整備
- [ ] Confirm tonic features missing webpki support
- [ ] Re-add tls-webpki-roots in workspace + otel crates
- [ ] Run cargo metadata/check to ensure deps resolve