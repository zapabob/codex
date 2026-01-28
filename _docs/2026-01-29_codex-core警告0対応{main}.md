## 実装ログ（2026-01-29）

### 目的
- `codex-core` の **警告を 0** にする
- `windows-sandbox-rs` の `unused_unsafe`（ネストした `unsafe`）を解消する

### 実施内容

#### windows-sandbox-rs: unused_unsafe 解消
- `windows-sandbox-rs/src/token.rs`
  - `OpenProcessToken(unsafe { GetCurrentProcess() }, ...)` の **内側の `unsafe` を除去**（外側で unsafe 扱いのため）。
- `windows-sandbox-rs/src/lib.rs`
  - `unsafe { convert_string_sid_to_sid(...) }` の **不要な `unsafe` を除去**（関数自体が unsafe ではないため）。

#### codex-core: unused / dead_code / unused_variables の解消
- `core/src/superior_git4d_visualizer.rs`
  - 未使用 import を削除
  - `unused_mut` / `unused_variables` を修正（`mut` 削除、`_vertices` 等）
  - まだ使われていないフィールドは `#[allow(dead_code)]` を付与（将来利用予定のため）
- `core/src/orchestration/integrated_competition.rs`
  - `repo_root` 未使用引数を `_repo_root` に変更
- `core/src/git4d_accelerated.rs`
  - `branch_name` 未使用変数を `_branch_name` に変更
- `core/src/codex.rs`
  - 現状未使用の関数群に `#[allow(dead_code)]` を付与（機能フラグ/導線の都合で将来使用前提）
- `core/src/error.rs`
  - `UsageLimitReachedError.rate_limits` に `#[allow(dead_code)]` を付与
- `core/src/models_manager/model_family.rs`
  - 未使用の override 系メソッドに `#[allow(dead_code)]` を付与
- `core/src/cuda_accelerator.rs`
  - 未使用の `GIT4D_KERNELS` に `#[allow(dead_code)]` を付与
- `core/src/vr_ar_integration.rs`
  - 未使用フィールドに `#[allow(dead_code)]` を付与

### 検証
- `cargo check -p codex-core --features custom-features` を実行し、**warnings 0** を確認。

