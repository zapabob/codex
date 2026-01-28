---
name: GUI構造の一本化
overview: gui/にprism-webとtauri-guiの機能を統合し、Rustバックエンド（codex-rs/gui）を統一APIとして使用する。Supabase依存を削除し、全機能をRustバックエンドに統合する。
todos:
  - id: extract-common-components
    content: 共通コンポーネントの抽出（可視化、VR/AR）
    status: completed
  - id: unify-api-client
    content: 共通APIクライアントの統合（Plan管理、認証）
    status: completed
  - id: implement-auth-api
    content: Rustバックエンドに認証APIを実装（Supabase代替）
    status: completed
  - id: implement-plans-api
    content: RustバックエンドにPlan管理APIを実装
    status: completed
  - id: implement-vr-api
    content: RustバックエンドにVR/AR APIを実装
    status: completed
  - id: migrate-plans-page
    content: Plan管理ページをgui/に移行
    status: completed
  - id: migrate-auth-page
    content: 認証ページをgui/に移行（Supabase削除）
    status: completed
  - id: integrate-visualization
    content: 可視化ページを統合
    status: completed
  - id: integrate-vr-pages
    content: VR/ARページを統合
    status: in_progress
  - id: update-dependencies
    content: 依存関係の整理（Supabase削除）
    status: pending
  - id: migrate-database
    content: SupabaseデータをSQLiteに移行
    status: pending
  - id: update-tests
    content: テストの更新と追加
    status: pending
  - id: update-docs
    content: ドキュメントの更新
    status: pending
  - id: create-implementation-log
    content: 実装ログを_docs/に保存
    status: pending
isProject: false
---

# GUI構造の一本化計画

## 現状分析

### 現在のGUI実装

1. **gui/** (Next.js)
  - HTTP API (`http://localhost:8787`) で `codex-rs/gui` (Rust) と通信
  - ダッシュボード、エージェント管理、セキュリティ、可視化
  - 依存: React 19.2.4, Next.js 14, Tailwind CSS
2. **prism-web/** (Next.js)
  - Supabase認証・データ保存
  - CLIを直接実行（`exec`）して通信
  - Plan管理、VR/AR機能、3D/4D可視化
  - 依存: Supabase, React 19.2.4, Next.js 14
3. **codex-rs/tauri-gui/** (Tauri)
  - Tauri APIでRustバックエンドと通信
  - VR/AR機能、デスクトップ統合、システムトレイ
  - 依存: Tauri 2.0, React 19.2.4, Vite

### 重複機能

- 3D/4D Git可視化（gui/, prism-web/, tauri-gui/）
- VR/AR機能（prism-web/, tauri-gui/）
- Plan管理（gui/, prism-web/）
- 認証（prism-web/のみSupabase）

## 統合方針

### 目標構造

```
gui/ (統合GUI)
├── src/
│   ├── app/                    # Next.js App Router
│   │   ├── (auth)/            # 認証（Rustバックエンド）
│   │   ├── (dashboard)/       # ダッシュボード
│   │   ├── plans/             # Plan管理（prism-webから移行）
│   │   ├── visualization/     # 3D/4D可視化（統合）
│   │   └── vr/                # VR/AR機能（統合）
│   ├── components/
│   │   ├── shared/            # 共通コンポーネント
│   │   ├── visualization/     # 可視化コンポーネント（統合）
│   │   └── vr/                # VR/ARコンポーネント（統合）
│   └── lib/
│       ├── api/
│       │   └── client.ts      # 統一APIクライアント
│       └── stores/             # 状態管理（Zustand）
│
codex-rs/gui/ (Rustバックエンド)
├── src/
│   ├── api/
│   │   ├── auth.rs            # 認証API（Supabase代替）
│   │   ├── plans.rs           # Plan管理API
│   │   ├── visualization.rs   # 可視化API
│   │   └── vr.rs              # VR/AR API
│   └── main.rs                # 統合エントリーポイント
```

## 実装フェーズ

### Phase 1: 共通ライブラリの抽出と統合

#### 1.1 共通コンポーネントの抽出

**対象ファイル**:

- `prism-web/components/visualizations/` → `gui/src/components/visualization/`
- `prism-web/lib/visualization/` → `gui/src/lib/visualization/`
- `prism-web/lib/xr/` → `gui/src/lib/xr/`
- `tauri-gui/src/components/` → `gui/src/components/vr/`（該当するもの）

**作業内容**:

- 重複する可視化コンポーネントを統合
- VR/AR機能を統合
- 共通型定義を `gui/src/lib/types/` に統合

#### 1.2 共通APIクライアントの統合

**対象ファイル**:

- `gui/src/lib/api/client.ts` - 既存のAPIクライアントを拡張
- `prism-web/lib/api/plans.ts` - Plan APIをRustバックエンド経由に変更

**作業内容**:

- `CodexAPIClient` にPlan管理メソッドを追加
- Supabase依存を削除し、RustバックエンドAPIに置き換え
- CLI直接実行（`exec`）をRustバックエンドAPI経由に変更

### Phase 2: Rustバックエンドの拡張

#### 2.1 認証APIの実装

**対象ファイル**: `codex-rs/gui/src/api/auth.rs` (新規作成)

**実装内容**:

- JWT認証の実装（Supabase代替）
- セッション管理
- ユーザー管理（SQLiteまたは既存DB）

**APIエンドポイント**:

- `POST /api/auth/login` - ログイン
- `POST /api/auth/logout` - ログアウト
- `GET /api/auth/session` - セッション確認
- `POST /api/auth/register` - ユーザー登録（オプション）

#### 2.2 Plan管理APIの実装

**対象ファイル**: `codex-rs/gui/src/api/plans.rs` (新規作成)

**実装内容**:

- Plan CRUD操作
- Plan承認/却下
- Plan実行
- Planエクスポート

**APIエンドポイント**:

- `GET /api/plans` - Plan一覧取得
- `POST /api/plans` - Plan作成
- `GET /api/plans/{id}` - Plan詳細取得
- `POST /api/plans/{id}/approve` - Plan承認
- `POST /api/plans/{id}/reject` - Plan却下
- `POST /api/plans/{id}/execute` - Plan実行
- `GET /api/plans/{id}/export` - Planエクスポート

**実装方法**:

- `codex Plan` コマンドを内部で実行
- 結果をJSON形式で返却

#### 2.3 可視化APIの拡張

**対象ファイル**: `codex-rs/gui/src/api/visualization.rs` (既存を拡張)

**実装内容**:

- Git4D可視化データの生成
- 3D/4D可視化用のAPIエンドポイント追加

#### 2.4 VR/AR APIの実装

**対象ファイル**: `codex-rs/gui/src/api/vr.rs` (新規作成)

**実装内容**:

- WebXRセッション管理
- ハンドトラッキング
- 空間オーディオ

### Phase 3: フロントエンドの統合

#### 3.1 Plan管理ページの移行

**対象ファイル**:

- `prism-web/app/(dashboard)/plans/page.tsx` → `gui/src/app/plans/page.tsx`
- `prism-web/lib/api/plans.ts` の機能を `gui/src/lib/api/client.ts` に統合

**作業内容**:

- Supabase依存を削除
- RustバックエンドAPIを使用するように変更
- UIコンポーネントを `gui/` のスタイルに統一

#### 3.2 認証ページの移行

**対象ファイル**:

- `prism-web/app/(auth)/login/page.tsx` → `gui/src/app/(auth)/login/page.tsx`
- `prism-web/lib/auth/context.tsx` → `gui/src/lib/context/AuthContext.tsx`

**作業内容**:

- Supabase認証を削除
- Rustバックエンド認証APIを使用
- JWTトークン管理

#### 3.3 可視化ページの統合

**対象ファイル**:

- `prism-web/app/(dashboard)/visualization/page.tsx` → `gui/src/app/visualization/page.tsx`
- `gui/src/app/git4d/page.tsx` と統合

**作業内容**:

- 重複する可視化機能を統合
- 統一されたUI/UX

#### 3.4 VR/ARページの統合

**対象ファイル**:

- `prism-web/app/(vr)/git-vr/page.tsx` → `gui/src/app/vr/page.tsx`
- `tauri-gui/src/pages/` のVR機能を統合

**作業内容**:

- WebXR機能の統合
- ハンドトラッキング統合
- 空間オーディオ統合

### Phase 4: 依存関係の整理

#### 4.1 package.jsonの統合

**対象ファイル**: `gui/package.json`

**作業内容**:

- `prism-web/package.json` から必要な依存関係を追加
- 重複する依存関係を整理
- Supabase関連の依存関係を削除

**追加する依存関係**:

- `@react-three/xr` (VR/AR)
- `zustand` (状態管理、既に存在する可能性)

**削除する依存関係**:

- `@supabase/supabase-js`
- `@supabase/auth-helpers-nextjs`
- `@supabase/auth-ui-react`

#### 4.2 環境変数の整理

**対象ファイル**: `gui/.env.example`

**作業内容**:

- Supabase関連の環境変数を削除
- Rustバックエンド用の環境変数を追加

**削除**:

- `NEXT_PUBLIC_SUPABASE_URL`
- `NEXT_PUBLIC_SUPABASE_ANON_KEY`
- `ENCRYPTION_SECRET`

**追加**:

- `CODEX_GUI_PORT` (デフォルト: 8787)
- `CODEX_GUI_CLI_PATH` (デフォルト: codex)

### Phase 5: データベース移行

#### 5.1 Supabaseデータの移行

**対象**: Planデータ、ユーザーデータ（該当する場合）

**作業内容**:

- Supabaseからデータをエクスポート
- RustバックエンドのSQLiteにインポート
- データ整合性の確認

#### 5.2 SQLiteスキーマの拡張

**対象ファイル**: `codex-rs/gui/src/db.rs` (既存を拡張)

**作業内容**:

- Planテーブルの追加
- ユーザーテーブルの追加（認証用）
- セッションテーブルの追加

### Phase 6: テストと検証

#### 6.1 統合テスト

**対象**:

- Plan管理機能のテスト
- 認証機能のテスト
- 可視化機能のテスト
- VR/AR機能のテスト

#### 6.2 E2Eテスト

**対象ファイル**: `gui/tests/`

**作業内容**:

- Playwrightテストの更新
- 新機能のE2Eテスト追加

### Phase 7: ドキュメント更新

#### 7.1 READMEの更新

**対象ファイル**:

- `gui/README.md`
- `codex-rs/gui/README.md`

**作業内容**:

- 統合後のアーキテクチャ説明
- セットアップ手順の更新
- APIドキュメントの更新

#### 7.2 実装ログの作成

**対象ファイル**: `_docs/yyyy-mm-dd_GUI構造一本化{worktreename}.md`

**作業内容**:

- 実装内容の記録
- 移行手順の記録
- 既知の問題と解決策

## 移行手順

### Step 1: バックアップ

```powershell
# 現在のブランチをバックアップ
git checkout -b backup-before-gui-unification
git push origin backup-before-gui-unification
```

### Step 2: Rustバックエンドの拡張

1. `codex-rs/gui/src/api/auth.rs` を作成
2. `codex-rs/gui/src/api/plans.rs` を作成
3. `codex-rs/gui/src/api/vr.rs` を作成
4. `codex-rs/gui/src/main.rs` にルートを追加

### Step 3: フロントエンドの統合

1. `prism-web/` から `gui/` にコンポーネントを移行
2. APIクライアントを更新
3. 依存関係を整理

### Step 4: テストと検証

1. 各機能の動作確認
2. 統合テストの実行
3. E2Eテストの実行

## 注意事項

- 段階的な移行を推奨（一度に全部変更するとリスクが高い）
- 各フェーズで動作確認を実施
- Supabase依存の完全削除前に、Rustバックエンドの動作確認
- データ移行は慎重に実施
- ロールバック計画を準備

## 期待される成果

- ✅ 単一の統合GUI (`gui/`)
- ✅ Rustバックエンドによる統一API
- ✅ Supabase依存の完全削除
- ✅ 重複コードの削減
- ✅ 保守性の向上
- ✅ 統一されたUI/UX

