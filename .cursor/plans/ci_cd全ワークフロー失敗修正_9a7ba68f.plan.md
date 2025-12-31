---
name: CI/CD全ワークフロー失敗修正
overview: すべてのCI/CDワークフローの失敗を修正する。主な原因は古いGitHub Actionsの使用、コンパイルエラー、テストの失敗、依存関係の問題など。
todos:
  - id: update_rust_clippy_workflow
    content: rust-clippy.ymlの古いGitHub Actionsを最新版に更新（actions/checkout@v4→v6、actions-rs/toolchain→dtolnay/rust-toolchain@1.90）
    status: completed
  - id: verify_compilation_errors
    content: cargo checkでコンパイルエラーを確認・修正（integration_web_search.rs、rate_limit.rs、cli/src/main.rsなど）
    status: completed
  - id: verify_lint_errors
    content: cargo clippyでLintエラーを確認・修正
    status: completed
  - id: verify_format_errors
    content: cargo fmt --checkでフォーマットエラーを確認・修正
    status: completed
  - id: fix_ci_mcp_integration
    content: ci.ymlのMCP統合テストのビルド失敗を適切にハンドリング
    status: completed
  - id: verify_cargo_deny
    content: cargo-denyワークフローの依存関係・ライセンス問題を確認・修正
    status: completed
  - id: verify_rust_ci
    content: rust-ci.ymlのRust toolchainバージョンがrust-toolchain.tomlと一致しているか確認
    status: completed
  - id: test_locally
    content: ローカルでCIと同等のコマンドを実行して検証（cargo check、cargo clippy、cargo fmt --check、cargo test）
    status: completed
---

# CI/CD全ワークフロー失敗修正計画

## 現状分析

すべてのCI/CDワークフローが失敗している原因として、以下が考えられます：

1. **古いGitHub Actionsの使用**: `rust-clippy.yml`で古いアクション（`actions-rs/toolchain@v1`、`actions/checkout@v4`）を使用
2. **コンパイルエラー**: Rust 2024 editionでのunsafe関数呼び出しや構文エラー
3. **テストの失敗**: 環境変数の設定やテストフィクスチャの問題
4. **依存関係の問題**: `cargo-deny`でのライセンスやセキュリティ問題
5. **ビルドエラー**: MCP統合テストでのビルド失敗

## 修正内容

### 1. rust-clippy.ymlの更新

**ファイル**: `.github/workflows/rust-clippy.yml`

**問題**: 古いGitHub Actionsを使用している（`actions-rs/toolchain@v1`、`actions/checkout@v4`）

**修正箇所**:

- Line 31: `actions/checkout@v4` → `actions/checkout@v6`
- Line 34: `actions-rs/toolchain@v1` → `dtolnay/rust-toolchain@1.90`（他のワークフローと統一）

**修正後**:

```yaml
- name: Checkout code
  uses: actions/checkout@v6

- name: Install Rust toolchain
  uses: dtolnay/rust-toolchain@1.90
  with:
    components: clippy
```

### 2. ci.ymlのMCP統合テスト修正

**ファイル**: `.github/workflows/ci.yml`

**問題**: MCP統合テストでビルドが失敗しても続行している（Line 118-120）

**修正箇所**: Line 114-120

**修正方法**: ビルド失敗を適切にハンドリングし、実際のエラーを報告するように変更

### 3. コンパイルエラーの確認と修正

**確認が必要なファイル**:

- `codex-rs/deep-research/tests/integration_web_search.rs` - `env::set_var`と`env::remove_var`がunsafeブロックで囲まれているか
- `codex-rs/orchestrator/src/rate_limit.rs` - 構文エラーがないか
- `codex-rs/cli/src/main.rs` - パターンマッチングで`..`が使用されているか

### 4. cargo-denyワークフローの確認

**ファイル**: `.github/workflows/cargo-deny.yml`

**問題**: 依存関係のライセンスやセキュリティ問題

**確認事項**:

- `codex-rs/.cargo/audit.toml`の設定を確認
- 新しい依存関係のライセンスを確認

### 5. rust-ci.ymlの確認

**ファイル**: `.github/workflows/rust-ci.yml`

**確認事項**:

- Rust toolchainのバージョンが`rust-toolchain.toml`と一致しているか（1.90.0）
- すべてのマトリックスビルドが正常に動作するか

### 6. テストの確認

**確認が必要なテスト**:

- MCP統合テスト（`ci.yml`の`mcp-integration-test`ジョブ）
- GUI統合テスト（`ci.yml`の`gui-tests`ジョブ）
- Rustのユニットテスト（`rust-ci.yml`の`tests`ジョブ）

## 実装手順

1. **rust-clippy.ymlの更新**: 古いアクションを最新版に更新
2. **コンパイルエラーの確認**: `cargo check`でコンパイルエラーを確認
3. **Lintエラーの確認**: `cargo clippy`でLintエラーを確認
4. **フォーマットの確認**: `cargo fmt --check`でフォーマットエラーを確認
5. **テストの実行**: `cargo test`でテストが正常に実行されるか確認
6. **CIワークフローの修正**: 各ワークフローの問題を修正
7. **ローカルでの検証**: 修正後、ローカルでCIと同等のコマンドを実行して検証

## 注意事項

- `rust-clippy.yml`は`actions-rs/toolchain`から`dtolnay/rust-toolchain`に変更する必要がある
- MCP統合テストのビルド失敗は適切にハンドリングする必要がある
- すべてのワークフローでRust toolchainのバージョンを統一（1.90.0）
- コンパイルエラーがある場合は、まずそれを修正してからCIを再実行