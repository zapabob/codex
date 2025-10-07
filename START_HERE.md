# 🚀 START HERE - Codex本番環境テスト開始ガイド

## ⚡ クイックスタート（3ステップ）

### ステップ1: 新しいPowerShellを開く
```powershell
# 新しいPowerShellウィンドウを開いて以下を実行:
cd C:\Users\downl\Desktop\codex-main\codex-main
```

### ステップ2: Codexの動作確認
```powershell
codex --version
```
**期待される出力**: `codex-cli 0.0.0`

### ステップ3: 最初のテストを実行！
```powershell
codex exec "Create a simple hello world function in Rust"
```

---

## 🎯 推奨テスト順序

### レベル1: 超簡単（各10-30秒）

#### テスト1-A: 最小テスト
```powershell
codex exec "Create a hello world function in Rust"
```

#### テスト1-B: 簡単な質問
```powershell
codex exec "What is the difference between String and &str in Rust?"
```

#### テスト1-C: セキュリティチェック
```powershell
codex exec "Is this code safe? let query = format!(\"SELECT * FROM users WHERE id = '{}'\", user_input);"
```

---

### レベル2: 基本テスト（各30-60秒）

#### テスト2-A: CodeExpert - 関数生成
```powershell
codex exec "Create a Rust function to calculate factorial with error handling using Result type and unit tests"
```

#### テスト2-B: SecurityExpert - 脆弱性検出
```powershell
codex exec "Review the file test-workspace\sample_code.rs for security vulnerabilities. Identify SQL injection risks, hardcoded secrets, and error handling issues."
```

#### テスト2-C: TestingExpert - テスト生成
```powershell
codex exec "Generate unit tests for a binary search function in Rust with edge cases"
```

---

### レベル3: 高度なテスト（各60-120秒）

#### テスト3-A: DeepResearch - 技術調査
```powershell
codex exec "Research Rust async programming best practices. Compare Tokio vs async-std and provide recommendations."
```

#### テスト3-B: マルチエージェント協調
```powershell
codex exec "Create a user authentication system:
- Research JWT best practices (DeepResearcher)
- Implement JWT token generation in Rust (CodeExpert)
- Review for security issues (SecurityExpert)
- Generate unit tests (TestingExpert)
- Write API documentation (DocsExpert)"
```

---

## 📋 テスト実行チェックリスト

### 事前準備
- [ ] PowerShellを管理者権限で開いた
- [ ] `codex --version` で動作確認
- [ ] `cd C:\Users\downl\Desktop\codex-main\codex-main` でディレクトリ移動

### レベル1テスト
- [ ] テスト1-A: Hello World実行
- [ ] テスト1-B: 簡単な質問実行
- [ ] テスト1-C: セキュリティチェック実行

### レベル2テスト
- [ ] テスト2-A: CodeExpert実行
- [ ] テスト2-B: SecurityExpert実行
- [ ] テスト2-C: TestingExpert実行

### レベル3テスト
- [ ] テスト3-A: DeepResearch実行
- [ ] テスト3-B: マルチエージェント実行

---

## 🔒 セキュリティ設定（オプション）

### 読み取り専用モード
```powershell
$env:CODEX_SANDBOX_MODE = "readonly"
codex exec "your command"
```

### 通常モード
```powershell
# 環境変数を設定しない状態で実行
codex exec "your command"
```

---

## 📊 結果の記録方法

### 自動ログ
各テスト実行後、以下のファイルに結果を追記してください：
- `test-results/実行ログ.md`

### 手動記録テンプレート
```markdown
### テスト[番号]: [名前]
**実施時刻**: HH:MM:SS
**コマンド**: `codex exec "..."`
**所要時間**: XX秒
**結果**: ✅成功 / ❌失敗

**生成された出力**:
```
[ここにコピペ]
```

**評価**:
- 機能性: [1-5]/5
- 品質: [1-5]/5
- 有用性: [1-5]/5

**備考**: [気づいた点]
```

---

## ⚠️ よくある問題と解決方法

### 問題1: `codex: command not found`
**解決策**:
```powershell
# パス確認
where.exe codex

# 再インストール
cd C:\Users\downl\Desktop\codex-main\codex-main\codex-rs
cargo install --path cli --force
```

### 問題2: 応答が遅い
**解決策**:
- タスクを分割する
- より具体的な指示を与える
- 別のテストを試す

### 問題3: エラーが発生
**解決策**:
```powershell
# ログ確認
$env:RUST_LOG="debug"
codex exec "your command"

# ログリセット
Remove-Item Env:\RUST_LOG
```

---

## 📚 詳細ドキュメント

### メインガイド
1. **demo_commands.md** - コピペ用コマンド集
2. **_docs/2025-10-08_本番環境テストガイド.md** - 完全ガイド
3. **独自機能使用ガイド.md** - 詳細な使い方

### 参考ファイル
- `test-workspace/sample_code.rs` - セキュリティテスト用サンプル
- `test-results/` - テスト結果保存先

---

## 🎯 成功の基準

### 各テストの合格基準

#### 機能性
- ✅ タスクが正しく完了する
- ✅ 期待した形式で出力される
- ✅ エラーが発生しない

#### 品質
- ✅ 生成されたコードが動作する
- ✅ ベストプラクティスに従っている
- ✅ 実用的である

#### SubAgent機能
- ✅ 適切なエージェントが起動
- ✅ 専門性が発揮される
- ✅ 高品質な出力

#### DeepResearch機能
- ✅ 複数ソースから情報収集
- ✅ 構造化されたレポート
- ✅ 実用的な推奨事項

---

## 🚀 今すぐ始める！

### 超簡単スタート（コピペするだけ）

```powershell
# Step 1: ディレクトリ移動
cd C:\Users\downl\Desktop\codex-main\codex-main

# Step 2: 動作確認
codex --version

# Step 3: 最初のテスト実行！
codex exec "Create a hello world function in Rust"
```

**これだけでOK！** 🎉

---

## 📞 サポート

### 困ったら
1. `demo_commands.md` を開く
2. `_docs/2025-10-08_本番環境テストガイド.md` を確認
3. `独自機能使用ガイド.md` で詳細確認

### GitHub
- https://github.com/zapabob/codex
- Issues で質問可能

---

**作成日時**: 2025-10-08 03:25 JST  
**バージョン**: codex-cli 0.0.0  
**ステータス**: ✅ Ready to Start

**さあ、今すぐ始めよう！** 🚀✨

---

## 🎊 最後に

このガイドに従えば、誰でも簡単にCodexのSubAgentとDeepResearch機能をテストできます。

**重要**: 最初は簡単なテストから始めてください！

**推奨順序**:
1. レベル1のテスト（3つ）
2. レベル2のテスト（3つ）
3. レベル3のテスト（2つ）

**頑張って！最高のAI開発体験を！** 💪🎉

