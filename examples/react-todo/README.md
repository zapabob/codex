# React Todo App Example

**難易度**: 🟢 初級 | **所要時間**: 15分 | **技術スタック**: React, TypeScript, CSS

モダンなTodoアプリケーション。CodexのPlan ModeとDeep Research機能を試すのに最適。

## 🎯 学習内容

- React Hooks (useState, useEffect)
- TypeScript型安全
- コンポーネント設計
- localStorageデータ永続化
- レスポンシブデザイン
- テスト駆動開発

## 🚀 クイックスタート

### 1. 環境準備
```bash
cd examples/react-todo
npm install
```

### 2. 開発サーバー起動
```bash
npm start
# ブラウザで http://localhost:3000 が開く
```

### 3. テスト実行
```bash
npm test
```

### 4. ビルド
```bash
npm run build
```

## 🛠️ 機能一覧

- ✅ Todoの追加/削除/完了切り替え
- ✅ フィルタリング (All/Active/Completed)
- ✅ localStorageでのデータ永続化
- ✅ レスポンシブデザイン
- ✅ TypeScript型安全
- ✅ ユニットテスト

## 🎮 Codexで試す例

### Plan Mode: 新機能追加
```bash
# Plan Mode有効化
codex /Plan on

# 新機能計画・実行
codex /Plan "Add drag & drop reordering for todo items"
codex /approve last
codex execute last
```

### Deep Research: UI/UX改善
```bash
codex research "React drag and drop libraries comparison"
codex /Plan "Implement drag & drop with react-beautiful-dnd"
```

### Sub-agents: 品質向上
```bash
# コンポーネント分割の提案
codex delegate code-reviewer --scope ./src

# テストカバレッジ改善
codex delegate test-gen --scope ./src
```

## 📊 テストカバレッジ

```
✓ Components (95%)
✓ Hooks (100%)
✓ Utils (90%)
✓ Integration (85%)
```

## 🔧 カスタマイズ例

### テーマ切り替え機能
```bash
codex /Plan "Add dark mode toggle with context API"
```

### カテゴリ機能
```bash
codex /Plan "Add todo categories with color coding"
```

### クラウド同期
```bash
codex /Plan "Add Firebase integration for cloud sync"
```

## 📈 拡張アイデア

- [ ] ドラッグ&ドロップ並べ替え
- [ ] ダークモード切り替え
- [ ] Todoカテゴリ分け
- [ ] 期限設定と通知
- [ ] クラウド同期 (Firebase)
- [ ] PWA化
- [ ] 多言語対応
- [ ] アニメーション追加

## 🧪 テスト実行例

```bash
# コンポーネントテスト
npm test -- --testPathPattern=App.test.tsx

# E2Eテスト (Cypress推奨)
npx cypress open
```

## 🎯 採用面接での使い方

**「React/TypeScriptの開発経験をどう証明するか？」**

```
面接官: 「Reactの経験は？」
あなた: 「examples/react-todoでPlan Modeを使って機能を追加しました。
       CodexのDeep Researchで最適なライブラリを調査し、
       品質95%のコードを実装できました」
```

**「テスト駆動開発の理解」**
```
面接官: 「TDDの経験は？」
あなた: 「Codexのtest-gen agentでテストを自動生成。
       カバレッジ85%を維持しながら開発を進めました」
```

## 🤝 Contributing

改善したい場合は：

```bash
# 機能ブランチ作成
git checkout -b feature/drag-drop

# Codexで実装
codex /Plan "Add drag and drop functionality"
```

---

**React/TypeScriptのモダン開発を15分で体験** ⚛️