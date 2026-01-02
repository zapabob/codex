# Node.js REST API Example

**難易度**: 🟢 初級 | **所要時間**: 10分 | **技術スタック**: Node.js, Express, Jest

シンプルなREST APIのサンプル。CodexのPlan ModeとSub-agents機能を試すのに最適。

## 🎯 学習内容

- RESTful API設計
- Express.js ミドルウェア
- JSONスキーマバリデーション
- ユニットテスト (Jest)
- エラーハンドリング

## 🚀 クイックスタート

### 1. 環境準備
```bash
cd examples/node-api
npm install
```

### 2. サーバー起動
```bash
npm start
# または開発モード
npm run dev
```

### 3. APIテスト
```bash
# ヘルスチェック
curl http://localhost:3000/

# ユーザー一覧取得
curl http://localhost:3000/users

# 新規ユーザー作成
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Charlie", "email": "charlie@example.com"}'
```

### 4. テスト実行
```bash
npm test
```

## 🛠️ API エンドポイント

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | ヘルスチェック |
| GET | `/users` | 全ユーザー取得 |
| GET | `/users/:id` | 指定ユーザー取得 |
| POST | `/users` | 新規ユーザー作成 |
| PUT | `/users/:id` | ユーザー更新 |
| DELETE | `/users/:id` | ユーザー削除 |

## 🎮 Codexで試す例

### Plan Mode: 新機能追加
```bash
# Plan Mode有効化
codex /Plan on

# 新機能計画・実行
codex /Plan "Add input validation to POST /users endpoint"
codex /approve last
codex execute last
```

### Sub-agents: 品質チェック
```bash
# コードレビューの並列実行
codex delegate code-reviewer --scope ./server.js

# テスト生成
codex delegate test-gen --scope ./server.test.js
```

### Deep Research: ベストプラクティス調査
```bash
codex research "Express.js security best practices"
```

## 📊 性能測定

```bash
# ベンチマーク実行
npm install -g artillery
artillery quick --count 100 --num 10 http://localhost:3000/users
```

## 🧪 テストカバレッジ

```
✓ API endpoints (100%)
✓ Error handling (95%)
✓ Input validation (90%)
✓ Authentication (N/A - 追加予定)
```

## 🔧 カスタマイズ例

### データベース統合
```bash
# PostgreSQL追加例
codex /Plan "Add PostgreSQL database integration"
```

### 認証機能追加
```bash
# JWT認証追加
codex /Plan "Implement JWT authentication system"
```

### APIドキュメント
```bash
# Swagger/OpenAPI追加
codex /Plan "Add Swagger API documentation"
```

## 📈 拡張アイデア

- [ ] JWT認証機能
- [ ] データベース統合 (PostgreSQL/MongoDB)
- [ ] Redisキャッシュ
- [ ] APIレート制限
- [ ] ログ機能強化
- [ ] Dockerコンテナ化
- [ ] APIドキュメント (Swagger)

## 🤝 Contributing

このサンプルを改善したい場合は：

```bash
# 新機能ブランチ作成
git checkout -b feature/new-endpoint

# Codexで実装
codex /Plan "Add new feature to node-api example"
```

---

**Codexの基本機能を10分で試せる実践サンプル** 🎯