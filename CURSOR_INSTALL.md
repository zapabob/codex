# ⚡ CursorIDE - Codex インストール超簡単ガイド

## 🎯 3ステップでCursorIDEに統合！

---

## ステップ1: Cursor設定を開く 📝

### 方法1: キーボードショートカット
```
Ctrl + ,
```

### 方法2: メニューから
```
File → Preferences → Settings
```

### 方法3: JSONファイル直接編集
```
Ctrl + Shift + P → "Preferences: Open User Settings (JSON)"
```

---

## ステップ2: Codex設定を追加 ✏️

### 設定ファイルに以下を追加:

```json
{
  "codex.executablePath": "C:\\Users\\downl\\.cargo\\bin\\codex.exe",
  "codex.enableDeepResearch": true,
  "codex.enableSubAgents": true,
  "codex.supervisorEnabled": true
}
```

### 完全な設定（推奨）:

`cursor-integration/settings.json` の内容をコピーして貼り付け

**または**:

```powershell
# 自動設定スクリプト（PowerShell）
$cursorSettings = "$env:APPDATA\Cursor\User\settings.json"

# バックアップ作成
if (Test-Path $cursorSettings) {
    Copy-Item $cursorSettings "$cursorSettings.backup"
}

# Codex設定を追加（既存設定がある場合はマージ）
$codexConfig = @{
    "codex.executablePath" = "C:\Users\downl\.cargo\bin\codex.exe"
    "codex.enableDeepResearch" = $true
    "codex.enableSubAgents" = $true
    "codex.supervisorEnabled" = $true
}

# 既存設定読み込み
if (Test-Path $cursorSettings) {
    $settings = Get-Content $cursorSettings -Raw | ConvertFrom-Json -AsHashtable
} else {
    $settings = @{}
}

# Codex設定をマージ
foreach ($key in $codexConfig.Keys) {
    $settings[$key] = $codexConfig[$key]
}

# 保存
$settings | ConvertTo-Json -Depth 10 | Set-Content $cursorSettings
Write-Host "✅ Cursor設定を更新しました！"
```

---

## ステップ3: CursorIDEを再起動 🔄

1. **CursorIDEを完全に閉じる**
   - ウィンドウを閉じる
   - タスクトレイのアイコンも閉じる（もしあれば）

2. **CursorIDEを再度開く**

3. **設定が読み込まれる**

---

## 🎯 動作確認

### Cursorターミナルで確認

**ターミナルを開く**: `` Ctrl + ` ``

**実行**:
```powershell
codex --version
```

**期待される出力**:
```
codex-cli 0.0.0
```

✅ これが表示されれば成功！

---

## 🚀 CursorでCodexを使う

### 使い方1: ターミナルから

```powershell
# Ctrl + ` でターミナルを開く
codex exec "Create a Rust function to validate email"
```

### 使い方2: エディタから

1. コードファイルを開く
2. `Ctrl + K` でCursor AIを起動
3. 「Use Codex」を選択
4. タスクを入力

### 使い方3: コンテキストメニュー

1. コードを選択
2. 右クリック
3. 「Ask Codex」を選択
4. タスクを入力

---

## 🎯 すぐ試せるテスト

### テスト1: Hello World（10秒）
```
Create a hello world function in Rust
```

### テスト2: セキュリティチェック（30秒）
```
Review this code for SQL injection: SELECT * FROM users WHERE id = '$user_input'
```

### テスト3: DeepResearch（60秒）
```
Research Rust async programming best practices
```

### テスト4: マルチエージェント（2分）
```
Build a REST API with security review and tests
```

---

## 📊 設定ファイルの場所

### Windows
```
設定ファイル:
%APPDATA%\Cursor\User\settings.json

フルパス:
C:\Users\downl\AppData\Roaming\Cursor\User\settings.json

バックアップ:
C:\Users\downl\AppData\Roaming\Cursor\User\settings.json.backup_YYYYMMDD_HHMMSS
```

### 参考設定ファイル
```
このリポジトリ:
cursor-integration/settings.json

推奨設定が全て含まれています
```

---

## 🔧 トラブルシューティング

### Q: Codexが見つからない

**A**: パスを確認
```powershell
where.exe codex
# → C:\Users\downl\.cargo\bin\codex.exe が表示されるべき

# 表示されない場合は再インストール
cd C:\Users\downl\Desktop\codex-main\codex-main\codex-rs
cargo install --path cli --force
```

### Q: 設定が反映されない

**A**: CursorIDEを完全再起動
1. 全てのCursorウィンドウを閉じる
2. タスクマネージャーでCursorプロセスが残っていないか確認
3. CursorIDEを再起動

### Q: SubAgent機能が動かない

**A**: テストを実行
```powershell
cd C:\Users\downl\Desktop\codex-main\codex-main\codex-rs
cargo test -p codex-supervisor --lib

# 24/24 passed と表示されればOK
```

---

## 🌟 Cursor統合の利点

### 1. シームレスな統合
- Cursorのエディタ内で直接Codex使用
- コンテキスト自動認識
- リアルタイム提案

### 2. 8種類のSubAgent
- **CodeExpert**: コード生成
- **SecurityExpert**: セキュリティ監査
- **TestingExpert**: テスト作成
- **DocsExpert**: ドキュメント生成
- **DeepResearcher**: 技術調査
- **DebugExpert**: デバッグ支援
- **PerformanceExpert**: 最適化
- **General**: 汎用タスク

### 3. DeepResearch
- 3階層深層分析
- 最大20ソース統合
- 学術論文対応

---

## ✅ インストール完了チェックリスト

### 事前準備
- [ ] codex グローバルインストール済み
- [ ] `codex --version` が動作

### 設定
- [ ] Cursor設定ファイルをバックアップ
- [ ] Codex設定を追加
- [ ] CursorIDEを再起動

### 動作確認
- [ ] Cursorターミナルで `codex --version` 成功
- [ ] Cursor AIでCodexが選択可能
- [ ] テストタスクが実行できる

---

## 🎉 完了！

**これでCursorIDEでCodexのSubAgent & DeepResearch機能が使えるで〜！** 🎊

### 今すぐ試す
```
1. CursorIDEを開く
2. Ctrl + ` でターミナルを開く
3. codex exec "Create a hello world function in Rust"
```

**最高の開発体験を楽しんでや〜！** 🚀✨

---

**作成日時**: 2025-10-08 03:35 JST  
**バージョン**: codex-cli 0.0.0  
**ステータス**: ✅ Ready to Install

**参考ドキュメント**:
- `cursor-integration/README.md` - 詳細ガイド
- `cursor-integration/settings.json` - 推奨設定

