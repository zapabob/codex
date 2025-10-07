# SubAgent & DeepResearch 本番環境テスト

## 📅 テスト実施日時
2025-10-08 03:15 JST

## 🎯 テスト目的
実際の本番環境でSubAgentとDeepResearch機能の動作を確認

## 🔒 セキュリティ設定

### サンドボックス設定
```bash
# 読み取り専用モードでテスト
export CODEX_SANDBOX_MODE="readonly"
```

### テストディレクトリ
```
C:\Users\downl\Desktop\codex-main\codex-main\test-workspace\
```

## 🧪 テストケース

### テスト1: CodeExpert - 単純な関数生成
**目的**: CodeExpertエージェントの基本動作確認
**タスク**: "Create a Rust function to calculate fibonacci numbers"

### テスト2: SecurityExpert - セキュリティ監査
**目的**: SecurityExpertの脆弱性検出能力確認
**タスク**: "Review this code for security issues: SQL query with string concatenation"

### テスト3: DeepResearch - 技術調査
**目的**: DeepResearch機能の情報収集能力確認
**タスク**: "Research Rust async programming best practices"

### テスト4: マルチエージェント協調
**目的**: 複数エージェントの協調動作確認
**タスク**: "Build a simple REST API with security review and tests"

## 📊 期待される結果
- ✅ 各エージェントが正しく起動
- ✅ タスクが適切に分解される
- ✅ 結果が構造化されて返される
- ✅ セキュリティチェックが機能する

