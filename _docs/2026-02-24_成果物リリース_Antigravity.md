# 2026-02-24 成果物 v2.17.0 バイナリリリース

## 概要

成果物 v2.17.0 の Windows 用バイナリをパッケージングし、GitHub CLI (`gh`) を使用してリリースにアップロードしました。

## 実施内容

1.  **バージョン確認**: `package.json` および git タグから、現在のバージョンが `v2.17.0` であることを確認。
2.  **バイナリ取得**: `codex-rs/target/release/` にビルド済みの `codex.exe` および `codex-tui.exe` が存在することを確認。
3.  **パッケージング**: `tar -aczf` を使用して、上記バイナリを `codex-v2.17.0-windows-x64.tar.gz` に圧縮。
4.  **リリースアップロード**: `gh release upload v2.17.0` を実行し、既存の `v2.17.0` リリースにアセットを追加。
5.  **検証**: `gh release view` でアセットが正しくアップロードされていることを確認。

## 成果物

- `codex-v2.17.0-windows-x64.tar.gz` (Windows x64 用バイナリパック)

## 実装者

- Antigravity (AI Coding Assistant)
