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

