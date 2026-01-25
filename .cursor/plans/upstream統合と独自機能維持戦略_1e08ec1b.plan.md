---
name: upstream統合と独自機能維持戦略
overview: upstream/mainの最新機能・バグフィクス・脆弱性改善を取り込みつつ、独自機能を条件付きコンパイルで保護し、公式リポジトリとの整合性を維持する
todos:
  - id: backup_branch
    content: 現在のブランチをバックアップ（安全のため）
    status: completed
  - id: fetch_upstream
    content: upstream/mainの最新版を取得
    status: completed
  - id: analyze_diff
    content: upstream/mainとの差分を分析し、コンフリクト予測
    status: completed
  - id: merge_upstream
    content: upstream/mainをマージ（コンフリクト解決時に独自機能を保護）
    status: completed
  - id: verify_build
    content: ビルド確認（custom-featuresフラグ付き）
    status: completed
  - id: verify_types
    content: 型チェック確認
    status: completed
  - id: verify_tests
    content: テスト実行と独自機能の動作確認
    status: completed
  - id: update_docs
    content: 実装ログを更新
    status: completed
isProject: false
---

# upstream/main統合と独自機能維持戦略

## 現状分析

### upstream/mainの最新コミット（12コミット）

- `b332482eb` - Mark collab as beta (#9834)
- `58450ba2a` - Use collaboration mode masks without mutating base settings (#9806)
- `24230c066` - Revert "fix: libcc link" (#9841)
- `18acec09d` - Ask for cwd choice when resuming session from different cwd (#9731)
- `182000999` - Raise welcome animation breakpoint to 37 rows (#9778)
- `652f08e98` - Revert "fix: musl build" (#9840)
- `279c9534a` - Prevent backspace from removing a text element when the cursor is at the element's left edge (#9630)
- `e2bd9311c` - fix(windows-sandbox): remove request files after read (#9316) - **セキュリティ修正**
- `2efcdf406` - fix: musl build (#9820) - **バグフィクス**
- `365160836` - fix: libcc link (#9819) - **バグフィクス**
- `83775f4df` - feat: ephemeral threads (#9765) - **新機能**
- `515ac2cd1` - feat: add thread spawn source for collab tools (#9769) - **新機能**

### 独自機能の保護状況

- `#[cfg(feature = "custom-features")]`で条件付きコンパイル済み
- `codex-rs/core/Cargo.toml`に`custom-features`フラグ定義済み
- `codex-rs/cli/Cargo.toml`で`custom-features`を有効化済み

## マージ戦略

### 原則

1. **upstream/mainの変更を優先**: バグフィクス、セキュリティ修正、新機能は取り込む
2. **独自機能は削除しない**: 公式に同等機能がない限り、条件付きコンパイルで保護
3. **整合性を維持**: コンフリクト時は公式の変更を採用し、独自機能は別モジュールで保持

### 保護すべき独自機能

1. **cowork CLI統合**: `codex-rs/core/src/cowork_integration.rs`
2. **git可視化VR/AR**: `codex-rs/core/src/git4d_accelerated.rs`, `codex-rs/core/src/vr_ar_integration.rs`
3. **/planコマンド**: `codex-rs/cli/src/plan_commands.rs`
4. **QCエージェント**: `codex-rs/core/src/orchestration/qc_merger.rs`
5. **git worktree並列開発**: `codex-rs/core/src/orchestration/worktree_manager.rs`
6. **orchestration/ディレクトリ**: 全体を条件付きコンパイルで保護

### マージ手順

#### Phase 1: 準備

1. 現在のブランチをバックアップ
2. `git fetch upstream`で最新を取得
3. コンフリクト予測のため差分を確認

#### Phase 2: マージ実行

1. `git merge upstream/main`を実行
2. コンフリクト解決:

- **公式の変更を優先**（コア機能、バグフィクス、セキュリティ修正）
- **独自機能は保持**（`#[cfg(feature = "custom-features")]`で保護）
- **両立可能な場合は統合**

#### Phase 3: 検証

1. ビルド確認: `cargo build --features custom-features`
2. 型チェック: `cargo check --features custom-features`
3. テスト実行: `cargo test --features custom-features`
4. 独自機能の動作確認

## 実装詳細

### 1. Cargo.tomlの更新

- `codex-rs/core/Cargo.toml`: `custom-features`フラグを維持
- `codex-rs/cli/Cargo.toml`: `custom-features`の有効化を維持

### 2. lib.rsの更新

- 独自機能モジュールを`#[cfg(feature = "custom-features")]`で保護
- upstream/mainの変更を取り込みつつ、独自機能のエクスポートを維持

### 3. main.rsの更新

- 独自機能コマンドを`#[cfg(feature = "custom-features")]`で保護
- upstream/mainのCLI変更を取り込みつつ、独自機能コマンドを維持

### 4. orchestration/ディレクトリの保護

- 全体を条件付きコンパイルで保護
- upstream/mainにorchestration/が存在しない場合、完全に保持

## リスクと対策

### リスク1: コンフリクトの大量発生

**対策**: 段階的なマージ、各機能ごとに独立してテスト

### リスク2: 独自機能の動作不良

**対策**: マージ後に各独自機能の統合テストを実行

### リスク3: API互換性の問題

**対策**: 型定義の整合性を確認、必要に応じてアダプター層を追加

## 成功基準

1. upstream/mainの最新機能・バグフィクス・セキュリティ修正が取り込まれている
2. すべての独自機能が`custom-features`フラグで保護されている
3. ビルドが成功し、型エラーがない
4. 独自機能が正常に動作している
5. 公式リポジトリとの整合性が維持されている