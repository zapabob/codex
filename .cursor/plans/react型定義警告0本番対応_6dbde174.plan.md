---
name: React型定義警告0本番対応
overview: React最新版への更新（CVE2026対応）、型定義の完全化、警告0、未使用関数削除、本番環境対応、Rust高速差分ビルドのエラー修正を実施
todos:
  - id: react-update
    content: React 19.2.4以上に更新（全プロジェクト）
    status: completed
  - id: eslint-strict
    content: ESLint設定をerrorレベルに変更（全プロジェクト）
    status: completed
  - id: tsconfig-strict
    content: TypeScript設定をstrict化（全プロジェクト）
    status: completed
  - id: any-type-elimination
    content: any型を段階的に排除（型定義ファイル作成→置換）
    status: completed
  - id: ts-warnings-fix
    content: TypeScript/ESLint警告を全て修正
    status: in_progress
  - id: unused-functions-ts
    content: TypeScript未使用関数を削除
    status: pending
  - id: rust-tui-errors
    content: codex-tuiのビルドエラー修正（重複モジュール、未解決インポート）
    status: completed
  - id: rust-unused-code
    content: Rust未使用コード削除（変数、関数、インポート）
    status: completed
  - id: rust-warnings-fix
    content: Rust Clippy警告を全て修正
    status: pending
  - id: production-optimization
    content: 本番環境最適化確認（ビルド設定、環境変数、パフォーマンス）
    status: completed
  - id: incremental-build
    content: Rust高速差分ビルド設定確認・最適化
    status: completed
isProject: false
---

# React型定義・警告0・本番環境対応計画

## 現状分析

### Reactプロジェクト

- **gui/**: React 19.2.1、any型182件、ESLint警告設定
- **prism-web/**: React 19.2.1、any型65件、ESLint警告設定
- **extensions/codex-viz-web/frontend/**: React 19.2.1、any型29件
- **codex-rs/tauri-gui/**: React 19.2.1、strict設定あり

### Rustプロジェクト

- **codex-tui**: 177エラー、21警告（未使用インポート、未使用変数、未使用関数）
- **codex-core**: 多数の未使用関数・変数警告
- **app-server-protocol**: 未使用インポート警告

### セキュリティ

- React 19.2.1はCVE-2025-55182修正済みだが、追加CVE（CVE-2026-23864等）対応で19.2.4以上が必要

## 実装計画

### Phase 1: React最新版更新とCVE対応

#### 1.1 Reactバージョン更新

- **対象ファイル**: 
  - `gui/package.json`
  - `prism-web/package.json`
  - `extensions/codex-viz-web/frontend/package.json`
  - `codex-rs/tauri-gui/package.json`
- **更新内容**: React 19.2.1 → 19.2.4以上（最新安定版）
- **関連パッケージ**: `react-dom`, `@types/react`, `@types/react-dom`も同時更新

#### 1.2 依存関係の確認

- `npm audit`でセキュリティ脆弱性チェック
- `react-server-dom-`*パッケージのバージョン確認（Server Components使用時）

### Phase 2: TypeScript型定義の完全化

#### 2.1 ESLint設定の厳格化

- **対象ファイル**:
  - `gui/.eslintrc.json`
  - `prism-web/.eslintrc.json`
  - `extensions/codex-viz-web/frontend/.eslintrc.cjs`（作成）
- **変更内容**:
  ```json
  {
    "rules": {
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-unused-vars": "error",
      "react-hooks/exhaustive-deps": "error"
    }
  }
  ```

#### 2.2 TypeScript設定の強化

- **対象ファイル**: 全`tsconfig.json`
- **追加設定**:
  ```json
  {
    "compilerOptions": {
      "noImplicitAny": true,
      "strictNullChecks": true,
      "noUnusedLocals": true,
      "noUnusedParameters": true,
      "noImplicitReturns": true
    }
  }
  ```

#### 2.3 any型の段階的排除

- **優先度順**:
  1. `gui/src/lib/api/client.ts` - APIレスポンス型定義
  2. `gui/src/components/visualization/*.tsx` - 3D関連型定義
  3. `prism-web/lib/xr/webxr-manager.ts` - WebXR型定義
  4. `extensions/codex-viz-web/frontend/src/**/*.ts` - フロントエンド型定義
- **型定義作成**:
  - `gui/src/lib/types/api.ts` - API型定義集約
  - `gui/src/lib/types/webxr.ts` - WebXR型定義
  - `gui/src/lib/types/three.ts` - Three.js型定義

### Phase 3: 警告0達成

#### 3.1 TypeScript/ESLint警告の解消

- **手順**:
  1. `npm run lint`で全警告をリスト化
  2. `tsc --noEmit`で型エラー確認
  3. 警告を1つずつ修正（未使用変数削除、型定義追加）

#### 3.2 未使用関数の削除

- **検出方法**: ESLint/TypeScriptの未使用検出
- **対象**:
  - `gui/src/**/*.tsx` - 未使用コンポーネント
  - `prism-web/components/**/*.tsx` - 未使用コンポーネント
  - `extensions/codex-viz-web/frontend/src/**/*.ts` - 未使用ユーティリティ

### Phase 4: Rustビルドエラー修正と警告0

#### 4.1 codex-tuiエラー修正

- **対象ファイル**: `codex-rs/tui/src/lib.rs`
- **主な問題**:
  - 重複モジュール定義（`app_backtrack`）
  - 未解決インポート（`AppExitInfo`, `LegacyApp`）
  - 型不一致エラー

#### 4.2 未使用コードの削除

- **対象**:
  - `codex-rs/tui/src/ui.rs` - 未使用変数（`app`, `f`, `area`）
  - `codex-rs/tui/src/bottom_pane/mod.rs` - 未使用インポート
  - `codex-rs/core/src/unified_exec/async_watcher.rs` - 未使用関数群

#### 4.3 Clippy警告の解消

- **設定**: `codex-rs/Cargo.toml`の`[workspace.lints.clippy]`は既に厳格
- **対応**: 未使用変数は`_`プレフィックス、未使用関数は削除

### Phase 5: 本番環境対応

#### 5.1 ビルド最適化確認

- **Next.js設定**:
  - `prism-web/next.config.js` - 既に最適化済み（SWC minify、code splitting）
  - `gui/next.config.js` - 同様の設定確認・追加

#### 5.2 環境変数管理

- **確認項目**:
  - 本番環境変数の適切な設定
  - 機密情報のハードコード排除
  - `.env.example`の更新

#### 5.3 パフォーマンス最適化

- **React最適化**:
  - `React.memo`の適切な使用
  - `useMemo`/`useCallback`の最適化
  - コード分割の確認

### Phase 6: Rust高速差分ビルド

#### 6.1 インクリメンタルビルド設定

- **環境変数**: `CARGO_INCREMENTAL=1`（devビルド）
- **設定確認**: `codex-rs/Cargo.toml`の`[profile.dev]`

#### 6.2 ビルドエラー修正

- **優先順位**:
  1. `codex-tui`のコンパイルエラー修正
  2. 未使用コード削除による警告解消
  3. 型エラーの修正

#### 6.3 ビルドスクリプト最適化

- **確認**: `scripts/rust_incremental_build.ps1`の最適化
- **並列ビルド**: `CARGO_BUILD_JOBS`の適切な設定

## 実装順序

1. **React最新版更新**（Phase 1）
2. **Rustビルドエラー修正**（Phase 4.1-4.2） - ビルド可能にする
3. **TypeScript型定義強化**（Phase 2）
4. **警告解消**（Phase 3, 4.3）
5. **本番環境確認**（Phase 5）
6. **高速ビルド最適化**（Phase 6）

## 検証方法

### TypeScript/React

```powershell
# 各プロジェクトで実行
cd gui
npm run type-check  # 型エラー0確認
npm run lint        # ESLint警告0確認
npm run build       # 本番ビルド成功確認
```

### Rust

```powershell
cd codex-rs
cargo check --workspace  # エラー0確認
cargo clippy --workspace -- -D warnings  # 警告0確認
cargo build --workspace  # ビルド成功確認
```

## 注意事項

- `any`型の排除は段階的に実施（一度に全部変更すると破壊的）
- 未使用関数の削除前に、将来使用予定がないか確認
- Rustの`#[allow(dead_code)]`は削除前に使用予定を確認
- 本番環境での動作確認を各フェーズで実施

## 期待される成果

- React 19.2.4以上（CVE2026対応済み）
- TypeScript型エラー0
- ESLint警告0
- Rustコンパイル警告0
- 未使用関数0
- 本番環境対応完了
- 高速差分ビルド動作確認

