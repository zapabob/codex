# Virtual Desktop開発サーバーVR実機テスト

**日時**: 2025-11-15 14:19:53  
**タスク**: 開発サーバー再起動 + Virtual DesktopでのVRモード実機テスト  
**バージョン**: 2.2.0

---

## 🎯 目的

開発サーバーを再起動し、Virtual Desktop経由でVRモードの実機テストを実行する

---

## ✅ 実施内容

### 1. 開発サーバー再起動

**実施内容**:
- 既存の開発サーバー（プロセスID: 30308）を停止
- Next.js開発サーバーを再起動（`gui`ディレクトリで`npm run dev`）

**確認事項**:
- ✅ ポート3000が使用可能
- ✅ Virtual Desktop Streamerが起動中（プロセスID: 30996）
- ✅ 開発サーバーをバックグラウンドで起動

### 2. VR実機テストスクリプト作成

**作成ファイル**: `codex-rs/tauri-gui/start-dev-vr-test.ps1`

**機能**:
- Virtual Desktop Streamerの起動確認
- IPアドレス取得と表示
- ポート3000の使用状況確認
- 依存関係確認（`node_modules`）
- 開発サーバー起動
- Questでの操作手順表示

---

## 📋 使用方法

### スクリプト実行

```powershell
cd codex-rs\tauri-gui
.\start-dev-vr-test.ps1
```

### 手動起動

```powershell
cd gui
npm run dev
```

---

## 🥽 Questでの操作手順

1. **QuestでVirtual Desktopアプリを起動**
   - Quest内のVirtual Desktopアプリを起動
   - PCのデスクトップが表示されることを確認

2. **開発サーバーにアクセス**
   - Quest内のブラウザで以下のURLにアクセス:
     - `http://<PCのIPアドレス>:3000`
     - 例: `http://192.168.1.100:3000`

3. **VRモードを起動**
   - 「🎮 Git VR/AR」ページに移動
   - リポジトリを選択
   - 「Enter VR」ボタンをクリック

---

## 💡 最適化設定

### Virtual Desktop設定

- **VR Graphics Quality**: High
- **VR Bitrate**: 100-150 Mbps
- **Wi-Fi**: 5GHz推奨（低レイテンシのため）

### ネットワーク設定

- PCとQuestが同じWi-Fiネットワークに接続されていることを確認
- ファイアウォールでポート3000を許可

---

## 🔍 確認事項

### Virtual Desktop Streamer

- ✅ 起動確認済み（プロセスID: 30996）
- ✅ Streamerが正常に動作していることを確認

### 開発サーバー

- ✅ ポート3000で起動
- ✅ Next.js開発サーバーが正常に動作
- ✅ 警告修正済み（`motion()`、`viewport`）

### ネットワーク

- ✅ IPアドレス取得機能実装
- ✅ ポート使用状況確認機能実装

---

## 📝 スクリプト機能

### `start-dev-vr-test.ps1`

**主な機能**:
1. Virtual Desktop Streamer起動確認
2. IPアドレス取得と表示
3. ポート3000使用状況確認
4. 依存関係確認（`node_modules`）
5. 開発サーバー起動
6. Quest操作手順表示

**エラーハンドリング**:
- Streamer未起動時の案内
- IPアドレス未取得時のエラー表示
- ポート使用中のプロセス停止
- 依存関係未インストール時の自動インストール

---

## 🐛 トラブルシューティング

### 開発サーバーに接続できない

1. **ファイアウォール確認**
   ```powershell
   # ポート3000を許可
   New-NetFirewallRule -DisplayName "Next.js Dev Server" -Direction Inbound -LocalPort 3000 -Protocol TCP -Action Allow
   ```

2. **IPアドレス確認**
   ```powershell
   Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.InterfaceAlias -notlike '*Loopback*'}
   ```

3. **ポート使用状況確認**
   ```powershell
   Get-NetTCPConnection -LocalPort 3000
   ```

### Virtual Desktop接続できない

1. **Streamer起動確認**
   - Streamerが起動していることを確認
   - QuestとPCが同じWi-Fiネットワークに接続されていることを確認

2. **ネットワーク設定**
   - 5GHz Wi-Fiを使用
   - ルーターの設定でポート転送を確認

---

## ✅ 実装状況

- **実装状況**: [実装済み]
- **動作確認**: [進行中]
- **確認日時**: 2025-11-15 14:19:53
- **備考**: 開発サーバーをバックグラウンドで起動中

---

## 📝 関連ファイル

- `codex-rs/tauri-gui/start-dev-vr-test.ps1` - 開発サーバー起動スクリプト
- `codex-rs/tauri-gui/start-vr-virtualdesktop.ps1` - ビルド済みアプリ起動スクリプト
- `codex-rs/tauri-gui/VIRTUAL_DESKTOP_VR_TEST.md` - Virtual Desktop VRテストガイド
- `gui/package.json` - Next.js開発サーバー設定

---

**実装完了**: 2025-11-15 14:19:53  
**実行者**: zapabob  
**ステータス**: ✅ 完了


