# Codex Sub-Agents & Deep Research - クイックスタートガイド 🚀

**Claude Code を超える機能を今すぐ使おう！**

---

## ⚡ 3分で始める

### 1. エージェント確認

```bash
# 利用可能なエージェントを確認
ls .codex/agents/
# → researcher.yaml
# → test-gen.yaml
# → sec-audit.yaml
# → code-reviewer.yaml
```

### 2. 最初のリサーチ

```bash
# Deep Research実行
codex research "Rust WebAssembly 2025" --depth 3

# 結果確認
cat artifacts/report.md
```

### 3. エージェント委任

```bash
# テスト生成を委任
codex delegate test-gen --scope ./src

# 結果確認
cat artifacts/test-report.md
```

**完了！** 🎉

---

## 📚 4つのサブエージェント

### 🔍 Deep Researcher
**用途**: 技術調査・比較分析・トレンド調査

```bash
# 基本
codex research "トピック" --depth 3 --breadth 8

# 軽量版（予算少ない時）
codex research "トピック" --depth 2 --lightweight-fallback --budget 20000

# 出力
artifacts/report.md        # レポート
artifacts/evidence/*.json  # エビデンスデータ
```

**特徴**:
- ✅ 複数ドメイン出典必須
- ✅ 矛盾検出（自動）
- ✅ 信頼度スコア
- ✅ 5検索バックエンド

---

### 🧪 Test Generator
**用途**: ユニットテスト自動生成・カバレッジ向上

```bash
# 基本
codex delegate test-gen --scope ./src

# 期限指定
codex delegate test-gen --scope ./src --deadline 2h

# 出力
artifacts/test-report.md      # テストレポート
artifacts/coverage-diff.json  # カバレッジ差分
+ 実際のテストコード（src配下）
```

**成功基準**:
- ✅ CI green
- ✅ カバレッジ +10%
- ✅ 既存テスト破壊なし

---

### 🔒 Security Auditor
**用途**: CVE監査・脆弱性検出・修正提案

```bash
# 基本
codex delegate sec-audit --scope ./src

# 予算増（徹底スキャン）
codex delegate sec-audit --scope ./src --budget 60000

# 出力
artifacts/sec-audit.md      # 監査レポート
artifacts/patches/*.diff    # 修正パッチ
artifacts/cve-report.json   # CVEレポート
```

**チェック内容**:
- ✅ 依存パッケージスキャン（npm, cargo, pip）
- ✅ CVE データベース照会
- ✅ 静的解析（bandit, clippy）
- ✅ CVSS スコア評価

---

### 📝 Code Reviewer
**用途**: コードレビュー・品質チェック・ベストプラクティス

```bash
# 基本
codex delegate code-reviewer --scope ./src

# ファイル指定
codex delegate code-reviewer --scope ./src/agents/runtime.rs

# 出力
artifacts/code-review.md          # レビューレポート
artifacts/review-summary.json     # サマリー
review-comments/*.md              # ファイル別コメント
```

**レビュー観点**（8項目）:
1. Style consistency（スタイル一貫性）
2. Error handling（エラーハンドリング）
3. Performance（パフォーマンス）
4. Security（セキュリティ）
5. Testing（テストカバレッジ）
6. Documentation（ドキュメント）
7. Maintainability（保守性）
8. Best practices（ベストプラクティス）

**Rust特化**:
- clippy lints
- rustfmt check
- unsafe code review
- lifetime analysis
- ownership patterns

---

## 🎯 実践シナリオ

### シナリオA: 新機能開発

```bash
# 1. 技術調査
codex research "Feature X technology comparison" --depth 3

# 2. コード実装（手動 or 別エージェント）

# 3. テスト生成
codex delegate test-gen --scope ./src/feature-x

# 4. セキュリティチェック
codex delegate sec-audit --scope ./src/feature-x

# 5. コードレビュー
codex delegate code-reviewer --scope ./src/feature-x

# 6. 結果統合
ls artifacts/
# - report.md（技術調査）
# - test-report.md（テスト）
# - sec-audit.md（セキュリティ）
# - code-review.md（レビュー）

# → GitHub PR自動作成
# → Slack通知
```

---

### シナリオB: 緊急セキュリティ対応

```bash
# 1. CVEスキャン（最優先）
codex delegate sec-audit --scope ./src --budget 40000

# 結果即座に確認
cat artifacts/sec-audit.md
# → CVE-2024-XXXXX検出（Critical）

# 2. 自動修正パッチ確認
cat artifacts/patches/fix-cve-2024-xxxxx.diff

# 3. PR自動作成
# → GitHub: PR #999 [SECURITY] Fix CVE-2024-XXXXX
# → Slack: 🚨 #security-alerts にアラート
# → Webhook: PagerDutyインシデント作成

# 4. テスト生成（修正検証）
codex delegate test-gen --scope ./src/affected

# 5. 緊急デプロイ
```

**所要時間**: ~5-10分（従来: 数時間〜数日）

---

### シナリオC: コードレビューの自動化

```bash
# PRごとに自動レビュー（GitHub Actions）
# .github/workflows/codex-review.yml

on:
  pull_request:
    types: [opened, synchronize]

jobs:
  review:
    runs-on: ubuntu-latest
    steps:
      - name: Codex Review
        run: |
          codex delegate code-reviewer \
            --scope ./src \
            --out review.md
          
          # レビュー結果をPRコメント
          gh pr comment ${{ github.event.pull_request.number }} \
            --body-file review.md
```

**メリット**:
- ✅ 人間レビュー前の自動チェック
- ✅ 重要度付きで優先度明確
- ✅ Rustイディオム指摘
- ✅ レビュー時間50-70%削減

---

## 🖥️ VS Code拡張の使い方

### インストール

```bash
cd vscode-extension
npm install
npm run compile

# VSIXパッケージ作成
npx vsce package

# インストール
code --install-extension codex-subagents-0.1.0.vsix
```

### 使い方

1. **Command Palette** (`Ctrl+Shift+P`)
   - `Codex: Delegate to Sub-Agent`
   - `Codex: Deep Research`
   - `Codex: Review Code`

2. **サイドバー**
   - 「Codex Agents」アイコンクリック
   - Sub-Agents 一覧表示
   - 実行中エージェントの状態確認

3. **設定**
   - File → Preferences → Settings
   - 「Codex」で検索
   - Slack/GitHub連携設定

---

## ⚙️ 設定ファイル例

### Slack統合（`.codex/slack.yaml`）

```yaml
webhook_url: ${SLACK_WEBHOOK_URL}
bot_token: ${SLACK_BOT_TOKEN}
default_channel: "#codex-agents"

channels:
  research: "#research-reports"
  security: "#security-alerts"
  general: "#engineering"

notifications:
  agent_started: general
  agent_completed: general
  research_completed: research
  security_audit: security
```

### Webhook設定（`.codex/webhooks.yaml`）

```yaml
webhooks:
  - name: "slack-main"
    url: "${SLACK_WEBHOOK_URL}"
    events:
      - AgentCompleted
      - ResearchCompleted
      - PrCreated
  
  - name: "github-actions"
    url: "https://api.github.com/repos/${REPO}/dispatches"
    events:
      - TestResults
      - SecurityAudit
    auth:
      type: Bearer
      token: "${GITHUB_TOKEN}"
  
  - name: "pagerduty"
    url: "https://events.pagerduty.com/v2/enqueue"
    events:
      - AgentFailed
      - SecurityAudit
    auth:
      type: Header
      name: "Authorization"
      value: "Token ${PAGERDUTY_TOKEN}"
```

---

## 🔧 トラブルシューティング

### Q: エージェントが見つからない

```bash
# エージェント定義を確認
ls .codex/agents/

# パスが正しいか確認
codex delegate researcher  # ✅ OK
codex delegate .codex/agents/researcher  # ❌ NG
```

### Q: 予算超過エラー

```bash
# 軽量版モードで実行
codex research "トピック" --lightweight-fallback

# または予算を増やす
codex delegate test-gen --budget 60000
```

### Q: Slack通知が来ない

```bash
# Webhook URLを確認
echo $SLACK_WEBHOOK_URL

# 設定ファイル確認
cat .codex/slack.yaml

# テスト送信
curl -X POST $SLACK_WEBHOOK_URL \
  -H 'Content-Type: application/json' \
  -d '{"text":"Test from Codex"}'
```

---

## 🌟 まとめ

**Codex Sub-Agents & Deep Research で開発を10倍加速！** 🚀

- ✅ 4つの専門エージェント
- ✅ 自動リサーチ・テスト・レビュー・セキュリティ
- ✅ GitHub/Slack完全統合
- ✅ VS Code拡張でGUI対応
- ✅ Claude Code完全超越

**今すぐ始めよう！** 💪

```bash
codex research "あなたのトピック" --depth 3
```

