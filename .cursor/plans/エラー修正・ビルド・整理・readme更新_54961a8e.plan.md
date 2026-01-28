---
name: エラー修正・ビルド・整理・README更新
overview: ビルドエラーを機能を損なわず修正し、警告0で差分高速ビルドを実行。プロセスキルとバイナリインストール後、ファイルを整理し、採用担当者向けに日英併記のREADMEを作成して公式リポジトリとの独自性を強調する。
todos:
  - id: fix_build_errors
    content: ビルドエラー修正（型エラー、条件付きコンパイル、Sendトレイト）
    status: completed
  - id: fix_warnings
    content: 警告0達成（unsafe関数、未使用変数）
    status: in_progress
  - id: incremental_build
    content: 差分高速ビルド実行と検証
    status: pending
  - id: run_tests
    content: テスト実行とClippy・フォーマットチェック
    status: pending
  - id: install_binary
    content: プロセスキルとバイナリインストール
    status: pending
  - id: organize_files
    content: ファイル整理（削除なし、フォルダーに整理）
    status: pending
  - id: update_readme
    content: README更新（日英併記、独自性強調、採用担当者向け）
    status: pending
isProject: false
---

# エラー修正・ビルド・整理・README更新計画

## 現状分析

### 主要なビルドエラー

1. **型エラー**: `git4d_accelerated.rs:472` - `branches`イテレータの型不一致
2. **条件付きコンパイル**: `cuda_accelerator`フィールドへのアクセス（`#[cfg]`保護が必要）
3. **Sendトレイト**: `superior_git4d_visualizer.rs`の非同期関数での`git2::Commit`使用
4. **型アノテーション**: 複数箇所で型推論失敗

### 独自機能（公式リポジトリとの差異）

- Git4D VR/AR可視化（CUDA加速対応）
- Deep Research拡張
- マルチエージェントオーケストレーション
- ClaudeCowork統合
- セキュリティ機能（マルウェア検知、ランサムウェア対策）
- Windows 11 25H2 MCP対応

## 実装手順

### Phase 1: ビルドエラー修正（機能保持）

#### 1.1 `git4d_accelerated.rs`の型エラー修正

- **ファイル**: `codex-rs/core/src/git4d_accelerated.rs`
- **問題**: `branches`イテレータが`Result`を返すが、タプルとして扱っている
- **修正**: `for (branch, _) in branches` → `for branch_result in branches` に変更し、`Result`を適切に処理

#### 1.2 `cuda_accelerator`フィールドアクセス修正

- **ファイル**: `codex-rs/core/src/git4d_accelerated.rs` (596, 631, 640行目)
- **問題**: 条件付きコンパイルされたフィールドへの直接アクセス
- **修正**: `#[cfg(all(feature = "custom-features", feature = "cuda"))]`ブロックで保護

#### 1.3 `Send`トレイト問題修正

- **ファイル**: `codex-rs/core/src/superior_git4d_visualizer.rs`
- **問題**: `git2::Commit`が`Send`を実装していない
- **修正**: コミット情報を同期的に抽出してから非同期処理（既に実装済みの可能性あり）

#### 1.4 型アノテーション追加

- **ファイル**: `codex-rs/core/src/git4d_accelerated.rs:479`
- **修正**: `commits: Vec<Oid>`を明示的に指定（ユーザーが既に修正済み）

### Phase 2: 警告0達成

#### 2.1 Rust 2024 unsafe関数警告

- **ファイル**: `codex-rs/windows-sandbox-rs/src/*.rs`
- **問題**: `unsafe fn`内でのunsafe関数呼び出しに`unsafe`ブロックが必要
- **修正**: 各unsafe関数呼び出しを`unsafe { ... }`で囲む

#### 2.2 未使用変数警告

- **修正**: 未使用変数に`_`プレフィックスを追加、または削除

### Phase 3: 差分高速ビルド実行

#### 3.1 ビルドスクリプト実行

- **コマンド**: `cd codex-rs && py -3 build_with_progress.py`
- **確認**: エラー0件、警告数を記録

### Phase 4: テスト・リンター・フォーマット

#### 4.1 テスト実行

- **コマンド**: `cargo test --workspace --features custom-features`

#### 4.2 Clippy実行

- **コマンド**: `cargo clippy --workspace --features custom-features -- -D warnings`

#### 4.3 フォーマットチェック

- **コマンド**: `cargo fmt --all -- --check`

### Phase 5: プロセスキルとバイナリインストール

#### 5.1 プロセスキル

- **スクリプト**: `scripts/install_with_kill.ps1`
- **実行**: `.\scripts\install_with_kill.ps1 -SourcePath "codex-rs\target\release\codex.exe" -TargetPath "$env:USERPROFILE\.cargo\bin\codex.exe" -Force`

#### 5.2 動作確認

- **コマンド**: `codex --version`

### Phase 6: ファイル整理（削除なし）

#### 6.1 エラーログファイル整理

- **対象**: `codex-rs/*.txt` (build_errors.txt, clippy_warnings.txt等)
- **アクション**: `codex-rs/logs/`ディレクトリを作成して移動
- **保持**: すべてのファイルを保持（削除しない）

#### 6.2 一時ファイル整理

- **対象**: ビルドアーティファクト以外の一時ファイル
- **アクション**: 適切なサブディレクトリに整理

#### 6.3 ドキュメント整理

- **対象**: `_docs/`, `docs/`内の実装ログ
- **アクション**: 日付順・機能別に整理

### Phase 7: README更新（日英併記・独自性強調）

#### 7.1 構造

- 既存のREADME.mdをベースに拡張
- 日英併記セクションを維持
- 採用担当者向けセクションを追加

#### 7.2 追加コンテンツ

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

#### 7.3 強調ポイント

- Git4D VR/AR可視化（公式にはない）
- CUDA加速サポート
- マルチエージェント並列実行（2.6倍高速化）
- Deep Research拡張
- Windows 11 25H2対応
- セキュリティ機能（マルウェア検知、ランサムウェア対策）
- ClaudeCowork統合

#### 7.4 採用担当者向け要素

- 技術スタックの明確化
- パフォーマンス指標
- エンタープライズ対応機能
- コミュニティ貢献の実績
- ロードマップと将来性

## 完了基準

- ビルドエラー0件
- Clippy警告0件
- テスト全通過
- フォーマットチェック通過
- バイナリインストール成功
- ファイル整理完了（削除なし）
- README更新完了（日英併記、独自性強調）

## 注意事項

- 機能を損なわない修正のみ実施
- 既存の独自機能はすべて保持
- ファイルは削除せず、整理のみ
- READMEは採用担当者が理解しやすい構成

