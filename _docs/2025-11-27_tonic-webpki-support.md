# Tonic WebPKI Feature Restoration

**日時**: 2025-11-27 17:50:11

---

## 概要

- `tonic` 依存関係から外れていた `tls-webpki-roots` を復活させ、WebPKI署名証明書チェーンを再び検証可能にした。
- `tls-roots` を維持しつつ `transport`/`tls` 系機能を整理し、CLI/TUI/GUI すべてのTLS経路で互換性を確保。

## 変更ファイル

- `codex-rs/Cargo.toml`
- `codex-rs/otel/Cargo.toml`

## 検証

- `cargo metadata --manifest-path codex-rs/core/Cargo.toml`

## 所見

- WebPKIルートを再度有効化しても依存グラフに循環は発生せず、差分ビルドも即完了。
- 今後 `tonic` を更新する際はTLS関連featureのalias仕様を再確認すること。

