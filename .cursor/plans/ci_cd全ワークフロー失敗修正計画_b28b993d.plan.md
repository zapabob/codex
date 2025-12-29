---
name: CI/CD全ワークフロー失敗修正計画
overview: zapabob/codexのCI/CDワークフロー（cargo-deny、ci、AI Kernel Modules CI、Codespell、Sub-Agent & Deep Research CI、rust-ci、sdk）の失敗を修正し、npm公開準備を完了させる。
todos:
  - id: verify_ghost_commits_commit
    content: ghost_commits.rsのBTreeSet修正がコミット・プッシュされているか確認
    status: completed
  - id: analyze_ci_logs
    content: 各ワークフローのCIログを確認して具体的なエラーメッセージを取得（rust-ci、cargo-deny、ci、Sub-Agent CI、Codespell、AI Kernel Modules CI、sdk）
    status: completed
  - id: fix_hashset_errors
    content: CIでエラーになっているHashSet使用箇所をBTreeSetに変更（優先度順）
    status: completed
  - id: fix_ci_workflow
    content: ciワークフローのエラーを修正（フォーマット、npmステージング、MCPテスト）
    status: completed
  - id: fix_cargo_deny
    content: cargo-denyワークフローの依存関係・ライセンス問題を解決
    status: completed
  - id: fix_subagent_ci
    content: Sub-Agent & Deep Research CIのビルド・テストエラーを修正
    status: completed
  - id: fix_codespell
    content: Codespellワークフローのスペルミスを修正
    status: completed
  - id: fix_kernel_ci
    content: AI Kernel Modules CIのビルドエラーを修正
    status: completed
  - id: fix_sdk_workflow
    content: sdkワークフローのTypeScriptビルド・Lint・テストエラーを修正
    status: completed
  - id: verify_locally
    content: ローカルでcargo clippyとcargo fmt --checkを実行して検証
    status: completed
  - id: commit_and_push
    content: 修正をコミット・プッシュしてCIの再実行を確認
    status: in_progress
---

# CI/CD全ワークフロー失敗修正計画

## 現状分析

画像から確認できる失敗ワークフロー：

- `cargo-deny` #221: 失敗（1m 4s）
- `ci` #721: 失敗（31s）
- `AI Kernel Modules CI` #330: 失敗（36s）
- `rust-ci` #721: 進行中
- `Codespell` #717: 失敗（32s）
- `Sub-Agent & Deep Research CI` #384: 失敗（2m 55s）
- `sdk` #723: 進行中

すべて「feat: Prepare for npm publication of @zapabob/codex」コミット（ca11aed）で実行。

## 問題の特定と修正方針

### 1. HashSet/BTreeSet問題の確認と修正

**現状**:

- `codex-rs/utils/git/src/ghost_commits.rs`はローカルで`BTreeSet`に修正済み
- コードベース全体で193箇所の`HashSet`使用が存在
- `clippy.toml`で`HashSet`が禁止されている

**修正方針**:

- CIで実際にエラーになっている箇所を優先的に修正
- `ghost_commits.rs`の修正がコミット・プッシュされているか確認
- `rust-ci`のClippyチェックでエラーになる`HashSet`使用箇所を特定して修正

**対象ファイル（優先度順）**:

1. `codex-rs/utils/git/src/ghost_commits.rs` - 既に修正済みだが、コミット確認が必要
2. CIでエラーになっている他のファイル（エラーログから特定）

### 2. cargo-denyワークフローの修正

**ファイル**: [.github/workflows/cargo-deny.yml](.github/workflows/cargo-deny.yml)

**確認事項**:

- `cargo-deny`が依存関係やライセンスの問題を検出している可能性
- `deny.toml`の設定を確認
- エラーログから具体的な問題を特定

**修正手順**:

1. CIログで具体的なエラーを確認
2. 依存関係の問題なら`deny.toml`を更新
3. ライセンス問題なら`allow`リストに追加

### 3. ciワークフローの修正

**ファイル**: [.github/workflows/ci.yml](.github/workflows/ci.yml)

**確認事項**:

- npmパッケージのステージング処理（`stage_npm_packages.py`）
- READMEのASCIIチェック
- Prettierフォーマットチェック
- MCP統合テスト

**修正手順**:

1. 各ステップのエラーログを確認
2. フォーマット問題なら`pnpm run format:fix`を実行
3. スクリプトエラーなら修正

### 4. rust-ciワークフローの修正

**ファイル**: [.github/workflows/rust-ci.yml](.github/workflows/rust-ci.yml)

**確認事項**:

- `cargo clippy -- -D warnings`で`HashSet`エラーが発生している可能性
- `cargo fmt --check`でフォーマットエラー
- `cargo check`でコンパイルエラー

**修正手順**:

1. Clippyエラーで`HashSet`使用箇所を特定
2. 優先度の高い箇所から`BTreeSet`に変更
3. フォーマットエラーがあれば`cargo fmt`を実行

### 5. Sub-Agent & Deep Research CIの修正

**ファイル**: [.github/workflows/subagent-ci.yml](.github/workflows/subagent-ci.yml)

**確認事項**:

- Rustビルド・テストエラー
- Clippy/Rustfmtチェック
- ドキュメント検証

**修正手順**:

1. ビルドエラーを確認
2. テスト失敗の原因を特定
3. 必要に応じてテストコードを修正

### 6. Codespellワークフローの修正

**ファイル**: [.github/workflows/codespell.yml](.github/workflows/codespell.yml)

**確認事項**:

- スペルチェックエラー
- 辞書ファイルの更新が必要な可能性

**修正手順**:

1. エラーログでスペルミスを確認
2. 辞書に追加するか、コードを修正

### 7. AI Kernel Modules CIの修正

**ファイル**: [.github/workflows/kernel-ci.yml](.github/workflows/kernel-ci.yml)

**確認事項**:

- カーネルモジュール関連のビルドエラー
- テスト失敗

**修正手順**:

1. エラーログを確認
2. ビルド設定やテストコードを修正

### 8. sdkワークフローの修正

**ファイル**: [.github/workflows/sdk.yml](.github/workflows/sdk.yml)

**確認事項**:

- TypeScript SDKのビルドエラー
- Lintエラー
- テスト失敗

**修正手順**:

1. ビルドエラーを確認
2. Lintルール違反を修正
3. テスト失敗の原因を特定

## 実装手順

### Phase 1: 現状確認とエラー特定

1. 各ワークフローのCIログを確認して具体的なエラーメッセージを取得
2. `ghost_commits.rs`の修正がコミット・プッシュされているか確認
3. Gitの状態を確認（未コミットの変更があるか）

### Phase 2: 優先度の高い修正

1. **rust-ci**: Clippyでエラーになる`HashSet`使用箇所を`BTreeSet`に変更

- エラーログから対象ファイルを特定
- 各ファイルで`use std::collections::HashSet;` → `use std::collections::BTreeSet;`
- 型アノテーションとインスタンス作成を修正

2. **ci**: フォーマットエラーがあれば修正
3. **cargo-deny**: 依存関係・ライセンス問題を解決

### Phase 3: その他のワークフロー修正

1. **Sub-Agent CI**: ビルド・テストエラーを修正
2. **Codespell**: スペルミスを修正
3. **AI Kernel Modules CI**: ビルドエラーを修正
4. **sdk**: TypeScript SDKのエラーを修正

### Phase 4: 検証とコミット

1. ローカルで`cargo clippy --all-features --tests -- -D warnings`を実行
2. ローカルで`cargo fmt --check`を実行
3. 修正をコミット・プッシュ
4. CIの再実行を確認

## 注意事項

- `HashSet`→`BTreeSet`の変更は、順序が重要な場合は動作に影響する可能性がある
- `BTreeSet`は`HashSet`より挿入・検索が遅いが、順序が保証される
- テストコードの`HashSet`使用も修正が必要（`clippy.toml`の設定により）
- CI環境のキャッシュが古い可能性があるため、必要に応じてキャッシュをクリア

## 成功基準

- すべてのCI/CDワークフローが成功する
- `cargo clippy`で警告なし
- `cargo fmt --check`がパスする
- npmパッケージのステージングが成功する