# 🚀 SubAgent & DeepResearch デモコマンド集

## 📅 作成日時
2025-10-08 03:25 JST

## 🎯 本番環境テスト用コマンド

以下のコマンドを順番に実行して、SubAgentとDeepResearch機能をテストします。

---

## テスト1: CodeExpert - 基本的な関数生成 ⚡

### コマンド
```powershell
codex exec "Create a Rust function to calculate the nth Fibonacci number with input validation, error handling using Result type, documentation comments, and unit tests"
```

### 期待される動作
- CodeExpertエージェントが起動
- 完全なRust関数が生成される
- Result型でエラーハンドリング
- ユニットテスト付き

### 所要時間目安
約30-60秒

---

## テスト2: SecurityExpert - セキュリティ監査 🔒

### コマンド
```powershell
codex exec "Review the file test-workspace\sample_code.rs for security vulnerabilities. Focus on SQL injection risks, hardcoded secrets, and error handling issues. Provide specific fixes for each vulnerability found."
```

### 期待される動作
- SecurityExpertエージェントが起動
- 3つの脆弱性を検出:
  1. SQL Injection（文字列連結）
  2. ハードコードされたAPI_KEY
  3. unwrapの使用
- 具体的な修正案を提示

### 所要時間目安
約45-90秒

---

## テスト3: TestingExpert - テスト生成 🧪

### コマンド
```powershell
codex exec "Generate comprehensive unit tests for a binary search function in Rust. Include edge cases like empty array, single element, element not found, first element, last element, and middle element. Use assert_eq and descriptive test names."
```

### 期待される動作
- TestingExpertエージェントが起動
- 包括的なテストスイートが生成される
- 全エッジケースをカバー
- 説明的なテスト名

### 所要時間目安
約40-70秒

---

## テスト4: DeepResearch - 技術調査 🔬

### コマンド
```powershell
codex exec "Research Rust async programming best practices. Compare Tokio vs async-std, explain common pitfalls, and provide performance considerations. Include at least 3 reliable sources and practical recommendations."
```

### 期待される動作
- DeepResearcherエージェントが起動
- 複数ソースから情報収集
- 構造化されたレポート生成
- 実用的な推奨事項

### 所要時間目安
約60-120秒

---

## テスト5: DocsExpert - API仕様書生成 📚

### コマンド
```powershell
codex exec "Generate API documentation for a simple REST API with the following endpoints: GET /users, POST /users, GET /users/:id, PUT /users/:id, DELETE /users/:id. Include request/response examples, error codes, and authentication requirements."
```

### 期待される動作
- DocsExpertエージェントが起動
- 完全なAPI仕様書が生成される
- サンプルリクエスト/レスポンス付き
- OpenAPI準拠

### 所要時間目安
約50-90秒

---

## テスト6: マルチエージェント協調 🤖🤖🤖

### コマンド
```powershell
codex exec "Create a simple user login system with the following requirements:

1. Research best practices for authentication (DeepResearcher)
2. Implement JWT-based login function in Rust (CodeExpert)
3. Review the implementation for security issues (SecurityExpert)
4. Generate unit tests for the login function (TestingExpert)
5. Create API documentation (DocsExpert)

The login function should accept username and password, validate them, and return a JWT token on success."
```

### 期待される動作
1. Coordinatorが5つのフェーズに分解
2. DeepResearcher → 認証ベストプラクティスのリサーチ
3. CodeExpert → JWT実装
4. SecurityExpert → セキュリティレビュー
5. TestingExpert → テスト生成
6. DocsExpert → ドキュメント生成
7. Reviewer → 統合レビュー

### 所要時間目安
約2-4分

---

## 🎯 クイックテスト（すぐ試せる）

### 最小テスト（10秒）
```powershell
codex exec "Create a simple hello world function in Rust"
```

### セキュリティチェック（30秒）
```powershell
codex exec "Is this code safe? let query = format!(\"SELECT * FROM users WHERE id = '{}'\", user_input);"
```

### 簡単なリサーチ（45秒）
```powershell
codex exec "What are the differences between String and &str in Rust?"
```

---

## 📊 テスト実行ログテンプレート

各テスト実行後、以下の情報を記録してください：

```markdown
### テスト[番号]: [テスト名]
**実施日時**: YYYY-MM-DD HH:MM:SS
**所要時間**: XX秒
**ステータス**: ✅成功 / ❌失敗 / ⚠️部分成功

**生成された出力**:
[ここに出力を記録]

**評価**:
- 機能性: [1-5]/5
- 品質: [1-5]/5
- 有用性: [1-5]/5

**備考**:
[気づいた点、改善提案など]
```

---

## ⚙️ 実行前の確認

### 環境確認
```powershell
# バージョン確認
codex --version

# テストディレクトリ確認
Test-Path "test-workspace"

# サンプルコード確認
Test-Path "test-workspace\sample_code.rs"
```

### セキュリティ設定（オプション）
```powershell
# 読み取り専用モード
$env:CODEX_SANDBOX_MODE = "readonly"
```

---

## 🔧 トラブルシューティング

### コマンドが動かない場合
```powershell
# パス確認
where.exe codex

# 再インストール
cd C:\Users\downl\Desktop\codex-main\codex-main\codex-rs
cargo install --path cli --force
```

### 応答が遅い場合
- タスクを分割
- より具体的な指示を与える
- システムリソースを確認

---

**作成者**: AI Assistant  
**更新日**: 2025-10-08  
**バージョン**: 1.0

**すべてのコマンドをコピペして実行できます！** 📋✨

