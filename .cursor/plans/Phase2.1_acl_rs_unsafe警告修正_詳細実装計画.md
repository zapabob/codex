---
name: Phase 2.1 acl.rs unsafe警告修正詳細実装計画
overview: acl.rsファイルのRust 2024 unsafe警告を修正するための詳細な実装計画。パイロット修正でパターンを確認し、残りを一括修正する。
todos:
  - id: analyze_acl_rs
    content: acl.rsファイルの内容を確認し、警告箇所を特定
    status: in_progress
  - id: pilot_fix_5
    content: 最初の5箇所を手動修正してパターンを確認
    status: pending
  - id: document_patterns
    content: 確認した修正パターンを文書化
    status: pending
  - id: batch_fix_remaining
    content: 残り15箇所を一括修正
    status: pending
  - id: build_verify
    content: cargo checkでビルド検証（警告0確認）
    status: pending
isProject: false
---

# Phase 2.1: acl.rs unsafe警告修正詳細実装計画

## 📋 現状分析

### ファイル状況
- **ファイルパス**: `codex-rs/windows-sandbox-rs/src/acl.rs`
- **現在の状態**: ⚠️ ファイルが空（0行）の可能性
- **確認が必要**: ファイルの実際の内容を確認

### ビルドエラーログから特定された警告箇所

| 行番号 | 警告内容 | 修正パターン |
|--------|---------|------------|
| 283 | `fetch_dacl_handle(path)?` | パターン1: Result返却 |
| 286 | `dacl_mask_allows(...)` | パターン2: 条件分岐内 |
| 363 | `ensure_allow_mask_aces_with_inheritance_impl(...)` | パターン1: 関数呼び出し |
| 376 | `ensure_allow_mask_aces_with_inheritance(...)` | パターン1: 関数呼び出し |
| 390 | `ensure_allow_mask_aces(...)` | パターン1: 関数呼び出し |
| 416 | `dacl_has_write_allow_for_sid(...)` | パターン2: 条件分岐内 |
| 451 | `!dacl_has_write_allow_for_sid(...)` | パターン6: 複合式内 |
| 486 | `!dacl_has_write_deny_for_sid(...)` | パターン5: 否定演算子付き |

**推定総数**: 約20箇所（ビルドエラーログから確認できたのは8箇所）

## 🎯 実装戦略

### Phase 1: ファイル確認と分析（5分）

#### 1.1 ファイルの存在確認
```powershell
# ファイルサイズ確認
Get-Item codex-rs\windows-sandbox-rs\src\acl.rs | Select-Object Length

# ファイル内容確認（最初の50行）
Get-Content codex-rs\windows-sandbox-rs\src\acl.rs -TotalCount 50
```

#### 1.2 警告箇所の特定
- ビルドエラーログから行番号を抽出
- 該当行のコードを確認
- 修正パターンを分類

### Phase 2: パイロット修正（30分）

#### 2.1 最初の5箇所を手動修正

**修正対象（優先順位順）**:
1. 行283: `fetch_dacl_handle(path)?`
2. 行286: `dacl_mask_allows(...)`
3. 行363: `ensure_allow_mask_aces_with_inheritance_impl(...)`
4. 行390: `ensure_allow_mask_aces(...)`
5. 行416: `dacl_has_write_allow_for_sid(...)`

**修正例**:
```rust
// 修正前（行283）
let (p_dacl, p_sd) = fetch_dacl_handle(path)?;

// 修正後
let (p_dacl, p_sd) = unsafe { fetch_dacl_handle(path)? };
```

```rust
// 修正前（行286）
if dacl_mask_allows(p_dacl, &[*sid], allow_mask, true) {

// 修正後
if unsafe { dacl_mask_allows(p_dacl, &[*sid], allow_mask, true) } {
```

#### 2.2 パターンの確認と文書化
- 各修正箇所で使用したパターンを記録
- 一括修正に適用可能か確認
- 特殊ケースがあれば記録

### Phase 3: 一括修正（15分）

#### 3.1 残り15箇所の修正
- 確認したパターンで一括修正
- 各修正箇所を慎重に確認

#### 3.2 修正パターンの適用
- パターン1: `unsafe { function_call()? }`
- パターン2: `unsafe { function_call() }`
- パターン5: `!unsafe { function_call() }`
- パターン6: `variable = !unsafe { function_call() }`

### Phase 4: ビルド検証（10分）

#### 4.1 コンパイル確認
```powershell
cd codex-rs
cargo check --package windows-sandbox-rs
```

#### 4.2 警告確認
- 警告数が減少しているか確認
- 新しいエラーが発生していないか確認
- 警告0を目指す

## 📊 成功基準

- [ ] acl.rsのすべてのunsafe警告が修正された
- [ ] `cargo check`でコンパイルエラーがない
- [ ] `cargo clippy -- -D warnings`で警告0
- [ ] 既存の機能が損なわれていない

## ⚠️ リスク管理

### リスク1: ファイルが空または存在しない
- **対策**: git履歴から復元、または他のソースから確認
- **影響**: 高（作業開始前に解決が必要）

### リスク2: 修正パターンの誤適用
- **対策**: 各修正後にビルド検証を実施
- **影響**: 中（すぐに検出可能）

### リスク3: 複雑な式の修正漏れ
- **対策**: ビルドエラーログを完全に確認
- **影響**: 低（自動検出可能）

## 🚀 次のステップ

1. **即座に実行**: acl.rsファイルの内容確認
2. **ファイルが空の場合**: git履歴から復元、または別の方法で確認
3. **ファイル確認後**: Phase 2（パイロット修正）を開始

## 📝 進捗記録

- **開始時刻**: 2026-01-28
- **現在のPhase**: Phase 1（ファイル確認）
- **完了予定**: Phase 4完了後、Phase 2.2（token.rs）に移行

---

**作成日時**: 2026-01-28  
**Worktree**: main  
**ステータス**: Phase 1進行中（ファイル確認が必要）
