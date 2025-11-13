# README更新 - zapabob拡張機能優先

**日時**: 2025-11-13 23:15:00  
**タスク**: README.mdを日英表記で更新し、zapabob独自機能を優先して記載

---

## 🎯 実装概要

README.mdを日英表記で更新し、zapabob独自機能を優先して記載しました。また、リモートへのコミットをzapabob/codex優先にする設定を明確にしました。

---

## 📋 実施内容

### 1. 日英表記の追加

- すべての主要セクションに英語と日本語の両方を記載
- 重要な情報を両言語で提供

### 2. zapabob拡張機能の優先記載

#### 追加したセクション

**🚀 zapabob Extended Features / zapabob拡張機能**

- **Git History Cleanup Script / Git履歴クリーンアップスクリプト**
  - 場所: `scripts/fix-invalid-paths-fast-export-streaming.py`
  - 機能:
    - ストリーミング処理
    - バイナリデータ処理
    - 進捗表示（tqdm）
    - 詳細なログ記録
    - Windowsエンコーディング対応
    - 自動バックアップブランチ作成

### 3. リモート優先設定の明確化

**⭐ Priority: zapabob Repository First / 優先: zapabobリポジトリ優先**

```markdown
**This repository prioritizes zapabob/codex as the primary remote for commits.**  
**このリポジトリは、コミットのプライマリリモートとしてzapabob/codexを優先します。**

# Primary remote (優先リモート)
origin: https://github.com/zapabob/codex.git

# Upstream (公式リポジトリ)
upstream: https://github.com/openai/codex.git
```

**Commit Strategy / コミット戦略:**
- ✅ **zapabob/codex (origin)** - Primary development and feature commits
- ✅ **zapabob/codex (origin)** - 主要な開発と機能コミット
- 📥 **openai/codex (upstream)** - Upstream synchronization (when needed)
- 📥 **openai/codex (upstream)** - 上流同期（必要に応じて）

### 4. リポジトリ情報セクションの追加

**Repository Information / リポジトリ情報**

- リモート設定の詳細
- コミット戦略の明確化
- プライマリリモートの優先度設定

---

## 🔧 変更内容

### 追加されたセクション

1. **🚀 zapabob Extended Features / zapabob拡張機能**
   - 拡張機能の紹介
   - Git履歴クリーンアップスクリプトの詳細

2. **Repository Information / リポジトリ情報**
   - リモート設定
   - コミット戦略
   - 優先度の明確化

3. **Acknowledgments / 謝辞**
   - ベースリポジトリのクレジット
   - 拡張機能の説明

### 更新されたセクション

- タイトル: "Codex CLI - zapabob Extended Edition"
- 説明文: zapabob独自機能を強調
- すべての主要セクションに日英表記を追加

---

## ✅ 完了事項

- [x] README.mdを日英表記で更新
- [x] zapabob独自機能を優先して記載
- [x] リモートコミット優先設定を追加
- [x] リポジトリ情報セクションを追加
- [x] コミットとプッシュ

---

## 🔗 関連リンク

- README.md: `README.md`
- Git履歴クリーンアップスクリプト: `scripts/fix-invalid-paths-fast-export-streaming.py`
- リモート設定: `.git/config`

---

**最終更新**: 2025-11-13 23:15:00  
**状態**: ✅ README更新完了

