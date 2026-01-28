---
name: エラー修正・ビルド・整理・README更新（容量整理含む）
overview: ビルドエラー（kernel_driverモジュール、DeviceCopyトレイト）とRust 2024 unsafe警告（104箇所）を同時に修正し、警告0で差分高速ビルドを実行。プロセスキルとバイナリインストール後、Cドライブの容量整理を行い、採用担当者向けに日英併記のREADMEを拡張して公式リポジトリとの独自性を強調する。
todos:
  - id: fix_kernel_driver_error
    content: kernel_driverモジュール重複宣言を修正（windows_impl.rsから削除）
    status: in_progress
  - id: fix_devicecopy_error
    content: DeviceCopyトレイトインポート追加（cuda_impl.rs）
    status: pending
  - id: fix_unsafe_warnings_acl
    content: acl.rsのunsafe警告修正（約20箇所）
    status: pending
  - id: fix_unsafe_warnings_token
    content: token.rsのunsafe警告修正（約30箇所）
    status: pending
  - id: fix_unsafe_warnings_process
    content: process.rsのunsafe警告修正（約10箇所）
    status: pending
  - id: fix_unsafe_warnings_other
    content: その他ファイルのunsafe警告修正（約44箇所）
    status: pending
  - id: fix_unused_variables
    content: 未使用変数警告修正
    status: pending
  - id: incremental_build
    content: 差分高速ビルド実行と検証（エラー0、警告0確認）
    status: pending
  - id: run_tests
    content: テスト実行とClippy・フォーマットチェック
    status: pending
  - id: install_binary
    content: プロセスキルとバイナリインストール
    status: pending
  - id: cleanup_disk_space
    content: Cドライブ容量整理（一時ビルドディレクトリ削除、ログファイル整理）
    status: pending
  - id: update_readme
    content: README更新（日英併記、独自性強調、採用担当者向け）
    status: pending
isProject: false
---

# エラー修正・ビルド・整理・README更新計画（容量整理含む）

## 現状分析

### ビルドエラー（修正が必要）

1. **kernel_driverモジュールエラー**
  - **ファイル**: `codex-rs/windows-ai/src/windows_impl.rs:8`
  - **問題**: `pub mod kernel_driver;`が重複宣言（`lib.rs`で既に宣言済み）
  - **修正**: `windows_impl.rs`の8行目を削除
2. **DeviceCopyトレイトエラー**
  - **ファイル**: `codex-rs/cuda-runtime/src/cuda_impl.rs`
  - **問題**: `DeviceCopy`トレイトがインポートされていない
  - **修正**: `use cust::memory::DeviceCopy;`を追加

### Rust 2024 unsafe警告（104箇所）

- **対象ファイル**: `codex-rs/windows-sandbox-rs/src/*.rs`
- **問題**: `unsafe fn`内でunsafe関数呼び出しに明示的な`unsafe`ブロックが必要
- **修正**: 各unsafe関数呼び出しを`unsafe { ... }`で囲む

### 未使用変数警告

- 複数の未使用変数に`_`プレフィックスを追加

## 実装手順

### Phase 1: ビルドエラー修正

#### 1.1 kernel_driverモジュール修正

- **ファイル**: `codex-rs/windows-ai/src/windows_impl.rs`
- **修正**: 8行目の`pub mod kernel_driver;`を削除（`lib.rs`で既に宣言済み）

#### 1.2 DeviceCopyトレイト修正

- **ファイル**: `codex-rs/cuda-runtime/src/cuda_impl.rs`
- **修正**: ファイル先頭に`use cust::memory::DeviceCopy;`を追加

### Phase 2: Rust 2024 unsafe警告修正（104箇所）

#### 2.1 acl.rs修正（約20箇所）

- `fetch_dacl_handle`, `dacl_mask_allows`, `ensure_allow_mask_aces_with_inheritance_impl`等の呼び出しを`unsafe { ... }`で囲む

#### 2.2 token.rs修正（約30箇所）

- `world_sid`, `get_logon_sid_bytes`, `set_default_dacl`, `enable_single_privilege`等の呼び出しを`unsafe { ... }`で囲む

#### 2.3 process.rs修正（約10箇所）

- `GetStdHandle`, `SetHandleInformation`, `GetLastError`等の呼び出しを`unsafe { ... }`で囲む

#### 2.4 その他のファイル修正

- `command_runner_win.rs`, `audit.rs`, `lib.rs`, `elevated_impl.rs`, `setup_orchestrator.rs`, `sandbox_users.rs`, `firewall.rs`, `dpapi.rs`, `winutil.rs`, `hide_users.rs`, `read_acl_mutex.rs`の各unsafe関数呼び出しを修正

### Phase 3: 未使用変数警告修正

- 未使用変数に`_`プレフィックスを追加、または削除

### Phase 4: 差分高速ビルド実行

#### 4.1 ビルドスクリプト実行

- **コマンド**: `cd codex-rs && py -3 build_with_progress.py`
- **確認**: エラー0件、警告0件を確認

### Phase 5: テスト・リンター・フォーマット

#### 5.1 テスト実行

- **コマンド**: `cargo test --workspace --features custom-features`

#### 5.2 Clippy実行

- **コマンド**: `cargo clippy --workspace --features custom-features -- -D warnings`

#### 5.3 フォーマットチェック

- **コマンド**: `cargo fmt --all -- --check`

### Phase 6: プロセスキルとバイナリインストール

#### 6.1 プロセスキル

- **スクリプト**: `scripts/install_with_kill.ps1`
- **実行**: `.\scripts\install_with_kill.ps1 -SourcePath "codex-rs\target\release\codex.exe" -TargetPath "$env:USERPROFILE\.cargo\bin\codex.exe" -Force`

#### 6.2 動作確認

- **コマンド**: `codex --version`

### Phase 7: Cドライブ容量整理

#### 7.1 ビルドアーティファクト整理

- **削除対象**: 
  - `codex-rs/target-test/` (0.33GB)
  - `codex-rs/target_fix_verify/` (0.09GB)
  - `codex-rs/target_install_verify/` (0.03GB)
- **保持**: `codex-rs/target/` (31.99GB) - 次回ビルドで再利用可能

#### 7.2 ログファイル整理

- **対象**: `codex-rs/*.txt` (約50ファイル)
- **アクション**: `codex-rs/logs/`ディレクトリを作成し、すべての`.txt`ログファイルを移動

#### 7.3 一時ビルドディレクトリ整理

- **削除対象**: `codex-rs/temp-gui-build/`

### Phase 8: README更新（日英併記・独自性強調）

#### 8.1 既存README.mdの拡張

- **ファイル**: `README.md`
- **方針**: 既存の日英併記構造を維持し、採用担当者向けセクションを追加

#### 8.2 追加コンテンツ

**English Section**:

- "Why Choose This Fork?" - 公式リポジトリとの明確な差異
- "Enterprise-Ready Features" - 本番環境対応機能
- "Performance Benchmarks" - 具体的な数値
- "Technical Differentiators" - 技術的な優位性

**Japanese Section**:

- 「なぜこのフォークを選ぶのか？」- 公式リポジトリとの明確な差異
- 「エンタープライズ対応機能」- 本番環境対応機能
- 「パフォーマンスベンチマーク」- 具体的な数値
- 「技術的優位性」- 技術的な差別化ポイント

#### 8.3 強調ポイント

- Git4D VR/AR可視化（公式にはない）
- CUDA加速サポート
- マルチエージェント並列実行（2.6倍高速化）
- Deep Research拡張
- Windows 11 25H2対応
- セキュリティ機能（マルウェア検知、ランサムウェア対策）
- ClaudeCowork統合

## 完了基準

- ビルドエラー0件
- Clippy警告0件
- テスト全通過
- フォーマットチェック通過
- バイナリインストール成功
- Cドライブ容量整理完了（約0.45GB削減 + ログファイル整理）
- README更新完了（日英併記、独自性強調）

## 注意事項

- 機能を損なわない修正のみ実施
- 既存の独自機能はすべて保持
- ファイルは削除せず、整理のみ（一時ビルドディレクトリは削除可）
- READMEは採用担当者が理解しやすい構成
- `target/`ディレクトリは保持（次回ビルドで再利用可能）

