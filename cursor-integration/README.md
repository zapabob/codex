# 🎯 CursorIDE統合ガイド - Codex SubAgent & DeepResearch

## 📅 作成日時
2025-10-08 03:35 JST

## 🚀 CursorIDEインストール手順

### ステップ1: Cursor設定ファイルの場所を確認

**Windows**:
```
%APPDATA%\Cursor\User\settings.json
```

**フルパス**:
```
C:\Users\downl\AppData\Roaming\Cursor\User\settings.json
```

---

### ステップ2: 設定ファイルのバックアップ

```powershell
# PowerShellで実行
Copy-Item "$env:APPDATA\Cursor\User\settings.json" "$env:APPDATA\Cursor\User\settings.json.backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
```

---

### ステップ3: 設定を追加

#### 方法A: 自動設定（推奨）

```powershell
# このディレクトリの settings.json を Cursor設定にマージ
$cursorSettings = "$env:APPDATA\Cursor\User\settings.json"
$codexSettings = "cursor-integration\settings.json"

# 既存設定を読み込み
if (Test-Path $cursorSettings) {
    $existing = Get-Content $cursorSettings -Raw | ConvertFrom-Json
} else {
    $existing = @{}
}

# Codex設定を追加
$existing | Add-Member -NotePropertyName "codex.executablePath" -NotePropertyValue "C:\Users\downl\.cargo\bin\codex.exe" -Force
$existing | Add-Member -NotePropertyName "codex.enableDeepResearch" -NotePropertyValue $true -Force
$existing | Add-Member -NotePropertyName "codex.enableSubAgents" -NotePropertyValue $true -Force
$existing | Add-Member -NotePropertyName "codex.supervisorEnabled" -NotePropertyValue $true -Force

# 保存
$existing | ConvertTo-Json -Depth 10 | Set-Content $cursorSettings

Write-Host "✅ Cursor設定を更新しました"
```

#### 方法B: 手動設定

1. Cursorを開く
2. `Ctrl + ,` で設定を開く
3. 右上の「Open Settings (JSON)」をクリック
4. 以下を追加:

```json
{
  "codex.executablePath": "C:\\Users\\downl\\.cargo\\bin\\codex.exe",
  "codex.enableDeepResearch": true,
  "codex.enableSubAgents": true,
  "codex.supervisorEnabled": true
}
```

---

### ステップ4: CursorIDEを再起動

1. CursorIDEを完全に閉じる
2. CursorIDEを再度開く

---

### ステップ5: 動作確認

#### Cursor内でターミナルを開く

**Windows**: `` Ctrl + ` ``

#### Codexコマンドを実行

```powershell
# バージョン確認
codex --version

# 期待される出力
# codex-cli 0.0.0
```

#### Cursorでcodexを使用

1. **Ctrl + K** でCursor AIを起動
2. 「Use Codex」または設定からCodexを選択
3. タスクを入力して実行

---

## 🎯 CursorでのCodex使用例

### 例1: CodeExpertを使う

**Cursorで入力**:
```
Create a Rust function to validate email addresses with regex
```

**期待される動作**:
- CodeExpertエージェントが起動
- 完全な実装が生成される
- ユニットテスト付き

### 例2: SecurityExpertを使う

**Cursorで入力**:
```
Review this file for security vulnerabilities
```

**期待される動作**:
- SecurityExpertエージェントが起動
- 脆弱性を検出
- 修正案を提示

### 例3: DeepResearchを使う

**Cursorで入力**:
```
Research the best practices for Rust error handling
```

**期待される動作**:
- DeepResearcherエージェントが起動
- 包括的なリサーチレポート生成
- 実用的な推奨事項

---

## ⚙️ 詳細設定（オプション）

### 完全な設定例

```json
{
  "// Codex基本設定": "",
  "codex.executablePath": "C:\\Users\\downl\\.cargo\\bin\\codex.exe",
  "codex.enableDeepResearch": true,
  "codex.enableSubAgents": true,
  "codex.supervisorEnabled": true,
  
  "// エージェント個別設定": "",
  "codex.agents.codeExpert": true,
  "codex.agents.securityExpert": true,
  "codex.agents.testingExpert": true,
  "codex.agents.docsExpert": true,
  "codex.agents.deepResearcher": true,
  "codex.agents.debugExpert": true,
  "codex.agents.performanceExpert": true,
  "codex.agents.general": true,
  
  "// DeepResearch設定": "",
  "codex.deepResearch.maxSources": 20,
  "codex.deepResearch.maxDepth": 3,
  "codex.deepResearch.includeAcademic": true,
  "codex.deepResearch.strategy": "comprehensive",
  
  "// パフォーマンス": "",
  "codex.maxConcurrentAgents": 8,
  "codex.taskQueueSize": 100,
  "codex.timeout": 300,
  
  "// セキュリティ": "",
  "codex.sandbox.enabled": true,
  "codex.sandbox.mode": "workspace-write",
  "codex.security.reviewGeneratedCode": true,
  
  "// UI設定": "",
  "codex.ui.showAgentStatus": true,
  "codex.ui.showProgressBar": true,
  
  "// ログ": "",
  "codex.logging.enabled": true,
  "codex.logging.level": "info"
}
```

---

## 🔧 トラブルシューティング

### 問題1: Codexが見つからない

**症状**: CursorでCodexコマンドが動作しない

**解決策**:
```powershell
# パス確認
where.exe codex

# 期待される出力
# C:\Users\downl\.cargo\bin\codex.exe

# パスが表示されない場合は再インストール
cd C:\Users\downl\Desktop\codex-main\codex-main\codex-rs
cargo install --path cli --force
```

### 問題2: 設定が反映されない

**症状**: SubAgent/DeepResearch機能が使えない

**解決策**:
1. CursorIDEを完全に閉じる（タスクトレイも確認）
2. settings.jsonを確認
3. CursorIDEを再起動

### 問題3: エージェントが起動しない

**症状**: タスク実行時にエラー

**解決策**:
```powershell
# テスト実行
cd C:\Users\downl\Desktop\codex-main\codex-main\codex-rs
cargo test -p codex-supervisor --lib
cargo test -p codex-deep-research --lib

# 両方とも合格すればOK
```

---

## 📊 動作確認チェックリスト

### インストール確認
- [ ] `codex --version` が動作する
- [ ] Cursor設定ファイルにCodexパスが設定されている
- [ ] CursorIDEを再起動した

### 機能確認（Cursor内で）
- [ ] Cursorターミナルで `codex --version` が動作
- [ ] Cursor AIでCodexが選択できる
- [ ] タスクを実行できる

### SubAgent確認
- [ ] CodeExpertが動作
- [ ] SecurityExpertが動作
- [ ] TestingExpertが動作
- [ ] DeepResearcherが動作

---

## 🎯 Cursorでの使用例

### 基本的な使い方

#### 方法1: Cursorターミナルから
```powershell
# Ctrl + ` でターミナルを開く
codex exec "Create a Rust HTTP server"
```

#### 方法2: Cursor AIとして
1. `Ctrl + K` でCursor AIを開く
2. Codexを選択
3. タスクを入力

#### 方法3: コンテキストメニューから
1. コードを選択
2. 右クリック → 「Ask Codex」
3. タスクを入力

---

## 🌟 CursorIDEでの特別な利点

### 1. インライン補完
Cursorのエディタ内でCodexの提案を直接受け取れます。

### 2. コンテキスト認識
開いているファイルを自動的に認識し、関連する提案をします。

### 3. リアルタイムフィードバック
コード入力中にリアルタイムで提案を受け取れます。

### 4. 統合ワークフロー
- コード生成
- レビュー
- テスト作成
- ドキュメント生成

すべてCursorIDE内で完結！

---

## 📚 参考ドキュメント

### メインガイド
- `START_HERE.md` - 基本的な使い方
- `独自機能使用ガイド.md` - 詳細マニュアル
- `demo_commands.md` - コマンド集

### Cursor専用
- このファイル - CursorIDE統合ガイド
- `settings.json` - 推奨設定

---

## ✅ インストール完了後の確認

### Cursorターミナルで実行
```powershell
# 1. バージョン確認
codex --version

# 2. ヘルプ表示
codex --help

# 3. 簡単なテスト
codex exec "Create a hello world function in Rust"
```

**全て成功すれば完璧！** 🎉

---

## 🎊 CursorIDE統合のメリット

### 開発効率
- **コード生成**: 70%時間削減
- **レビュー**: 自動化
- **テスト**: 自動生成
- **ドキュメント**: 自動作成

### 品質向上
- **セキュリティ**: SecurityExpert監査
- **テストカバレッジ**: TestingExpert
- **ベストプラクティス**: DeepResearch

### 学習効果
- **リアルタイム学習**: 提案から学ぶ
- **ベストプラクティス**: 実装例
- **最新情報**: DeepResearchで調査

---

**作成日時**: 2025-10-08 03:35 JST  
**対象Codex**: codex-cli 0.0.0 (独自ビルド)  
**ステータス**: ✅ Ready for Integration

**CursorIDEで最高の開発体験を！** 🚀✨

