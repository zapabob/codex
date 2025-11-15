# VRモード実機テストガイド

**日時**: 2025-11-15  
**対象デバイス**: Meta Quest 2/3/Pro, SteamVR, HTC Vive

---

## 🎯 テスト準備

### 1. 前提条件

- ✅ Meta Quest 2/3/Pro または SteamVR対応デバイス
- ✅ PCとQuestが同じWi-Fiネットワークに接続
- ✅ 開発者モード有効化（Questの場合）
- ✅ Node.js 18+ インストール済み

### 2. PCのIPアドレス確認

```powershell
# PowerShellでIPアドレス確認
Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.InterfaceAlias -notlike '*Loopback*' -and $_.IPAddress -notlike '169.254.*'} | Select-Object IPAddress, InterfaceAlias

# または
ipconfig | findstr "IPv4"
```

**例**: `192.168.1.100` がPCのIPアドレス

---

## 🚀 テスト手順

### 方法1: 開発サーバーでテスト（推奨）

#### ステップ1: 依存関係インストール

```powershell
cd codex-rs/tauri-gui
npm install
```

#### ステップ2: 開発サーバー起動

```powershell
# Vite開発サーバー起動（ポート1420）
npm run dev

# または、ホストを0.0.0.0にバインド（他のデバイスからアクセス可能）
npm run dev -- --host 0.0.0.0
```

**出力例**:
```
  VITE v5.4.6  ready in 500 ms

  ➜  Local:   http://localhost:1420/
  ➜  Network: http://192.168.1.100:1420/
```

#### ステップ3: Questでアクセス

1. **Quest内でブラウザ起動**
   - Quest 2/3: 内蔵ブラウザ
   - Quest Pro: 内蔵ブラウザ

2. **URL入力**
   ```
   http://192.168.1.100:1420/git-vr
   ```
   （`192.168.1.100`を実際のPCのIPアドレスに置き換え）

3. **VRモードに入る**
   - ページが読み込まれたら「Enter VR」ボタンをクリック
   - WebXR APIが自動的にVRセッションを開始

---

### 方法2: ビルド済みアプリでテスト

#### ステップ1: Tauriアプリビルド

```powershell
cd codex-rs/tauri-gui
npm run tauri:build
```

#### ステップ2: アプリ起動

```powershell
# ビルド済みアプリ起動
.\src-tauri\target\release\codex-tauri.exe
```

#### ステップ3: VRモード起動

1. アプリ内で「🎮 Git VR/AR」ページに移動
2. リポジトリを選択
3. 「Enter VR」ボタンをクリック

---

## 🎮 操作方法

### Meta Quest 2

- **左スティック**: 移動（前後左右）
- **右スティック**: 回転（左右）
- **トリガー**: ノード選択
- **グリップ**: タイムラインスクラブ
- **Aボタン**: メニュー表示
- **Bボタン**: 戻る

### Meta Quest 3/Pro

- **ハンドトラッキング**: ピンチ→選択
- **手のひら**: メニュー表示
- **コントローラー**: Quest 2と同じ操作

### SteamVR / HTC Vive

- **コントローラー**: 標準SteamVR操作
- **トリガー**: 選択
- **タッチパッド**: 移動

---

## 🔍 テスト項目

### 基本機能

- [ ] VRモードに入れる
- [ ] コミットノードが3D空間に表示される
- [ ] コントローラー/ハンドトラッキングで操作できる
- [ ] ノード選択が動作する
- [ ] タイムライン操作が動作する

### パフォーマンス

- [ ] フレームレート: Quest 2で90fps、Quest 3で120fps
- [ ] レイテンシ: <20ms
- [ ] メモリ使用量: 適切な範囲内

### デバイス固有機能

#### Quest 3/Pro
- [ ] ハンドトラッキングが動作する
- [ ] カラーパススルーが動作する（ARモード）
- [ ] アイトラッキングが動作する（Quest Proのみ）

#### SteamVR
- [ ] SteamVR統合が動作する
- [ ] コントローラー入力が正しく認識される

---

## 🐛 トラブルシューティング

### 問題1: VRモードに入れない

**原因**: WebXR APIがサポートされていない、または開発者モードが無効

**解決策**:
1. Questで開発者モードを有効化
2. ブラウザで `chrome://flags` を開き、WebXRを有効化
3. HTTPS接続を使用（localhost以外の場合）

### 問題2: ページが読み込まれない

**原因**: ファイアウォールがポートをブロックしている

**解決策**:
```powershell
# Windowsファイアウォールでポート1420を開放
New-NetFirewallRule -DisplayName "Codex VR Dev Server" -Direction Inbound -LocalPort 1420 -Protocol TCP -Action Allow
```

### 問題3: フレームレートが低い

**原因**: GPU性能不足、またはネットワーク帯域幅不足

**解決策**:
1. GPUドライバーを最新版に更新
2. Wi-Fi 6ルーターを使用（Questの場合）
3. Virtual Desktopのビットレートを調整

### 問題4: ハンドトラッキングが動作しない

**原因**: Quest 3/Proでハンドトラッキングが無効

**解決策**:
1. Quest設定でハンドトラッキングを有効化
2. アプリ内でハンドトラッキングを有効化

---

## 📊 テスト結果記録

### テスト環境

- **デバイス**: [Quest 2 / Quest 3 / Quest Pro / SteamVR]
- **PC IP**: [192.168.x.x]
- **OS**: Windows 11
- **GPU**: [RTX 3080]
- **ブラウザ**: [Quest Browser / Chrome]

### テスト結果

| 項目 | 結果 | 備考 |
|------|------|------|
| VRモード起動 | ✅ / ❌ | |
| フレームレート | [fps] | |
| レイテンシ | [ms] | |
| ハンドトラッキング | ✅ / ❌ | Quest 3/Proのみ |
| コントローラー操作 | ✅ / ❌ | |
| ノード選択 | ✅ / ❌ | |
| タイムライン操作 | ✅ / ❌ | |

### 発見した問題

1. [問題の説明]
2. [再現手順]
3. [期待される動作]

---

## 🔗 関連リンク

- [WebXR API Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebXR_Device_API)
- [Babylon.js VR Guide](https://doc.babylonjs.com/divingDeeper/cameras/webVRCamera)
- [Quest Developer Documentation](https://developer.oculus.com/)

---

**テスト完了後**: 結果を `_docs/2025-11-15_VR実機テスト結果.md` に記録してください。

