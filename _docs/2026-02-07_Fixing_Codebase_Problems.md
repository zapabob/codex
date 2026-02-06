# 2026-02-07 機能修正・整理ログ

## 修正内容

### 1. Rust (codex-tui) の未使用インポート削除

- `codex-rs/tui/src/history_cell/tests.rs` において、コンパイル警告の原因となっていた以下の未使用インポートを削除しました。
  - `mcp_types::ResourceLink`
  - `mcp_types::TextContent`

### 2. VS Code/Windsurf 拡張機能の `activationEvents` 整理

- 拡張機能の `package.json` において、`contributes.commands` から自動生成されるため不要な `onCommand` activationEvents を削除しました。
  - `extensions/package.json`
  - `extensions/windsurf-extension/package.json`

### 3. GitHub Workflow のシークレット参照修正

- Workflow リンターの警告を解消するため、シークレットを直接 `with` ブロックで参照するのではなく、ジョブまたはステップレベルの `env` 経由で参照するように修正しました。
  - `.github/workflows/issue-labeler.yml`
  - `.github/workflows/rust-release.yml`

## 検証結果

- `cargo check -p codex-tui`: 正常終了。未使用インポート削除後もコンパイルに問題がないことを確認。
