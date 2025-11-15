# Virtual Desktop経由VRモード実機テストガイド

**日時**: 2025-11-15  
**対象**: Meta Quest 2/3/Pro + Virtual Desktop

---

## 🎯 前提条件

### 必要なもの

- ✅ Meta Quest 2/3/Pro
- ✅ Virtual Desktop（Questストアで購入済み）
- ✅ Virtual Desktop Streamer（PC側アプリ、無料）
- ✅ PCとQuestが同じWi-Fiネットワークに接続
- ✅ 5GHz Wi-Fi推奨（低レイテンシのため）

---

## 🚀 セットアップ手順

### ステップ1: Virtual Desktop Streamerインストール

1. **Virtual Desktop公式サイトからダウンロード**
   - URL: https://www.vrdesktop.net/
   - 「Download Streamer」をクリック
   - Windows版をダウンロード・インストール

2. **Streamer設定**
   - Streamerを起動
   - Questのユーザー名を入力
   - 「Allow remote connections」を有効化
   - ファイアウォール許可を確認

### ステップ2: QuestでVirtual Desktop起動

1. **Quest内でVirtual Desktopアプリを起動**
   - QuestホームからVirtual Desktopを選択
   - PCが表示されることを確認

2. **接続確認**
   - Streamerが「Connected」と表示されていることを確認
   - レイテンシが<30msであることを確認

### ステップ3: Codex Tauriアプリビルド

```powershell
# codex-rs/tauri-guiディレクトリに移動
cd codex-rs/tauri-gui

# 依存関係インストール（初回のみ）
npm install

# Tauriアプリビルド（Release）
npm run tauri:build
```

**ビルド出力場所**:
```
codex-rs/tauri-gui/src-tauri/target/release/codex-tauri.exe
```

### ステップ4: Codexアプリ起動

```powershell
# ビルド済みアプリ起動
.\src-tauri\target\release\codex-tauri.exe

# または、インストール済みの場合
codex-tauri
```

---

## 🥽 VRモード起動手順

### 方法1: Virtual Desktop経由（推奨）

1. **QuestでVirtual Desktopを起動**
   - Questホーム → Virtual Desktop
   - PCのデスクトップが表示される

2. **Codexアプリを起動**
   - Virtual Desktop内でCodexアプリを起動
   - または、PC側でCodexアプリを起動（Virtual Desktop経由で表示される）

3. **VRモードに入る**
   - Codexアプリ内で「🎮 Git VR/AR」ページに移動
   - リポジトリを選択
   - 「Enter VR」ボタンをクリック
   - WebXR APIがVRセッションを開始

4. **VR空間で操作**
   - Questコントローラーで操作
   - または、ハンドトラッキング（Quest 3/Pro）

### 方法2: Virtual DesktopのVR環境で直接起動

1. **Virtual DesktopのVR環境に入る**
   - Virtual Desktop起動 → 「Enter VR」ボタン
   - PCのデスクトップがVR空間に表示される

2. **CodexアプリをVR空間で起動**
   - VR空間内のデスクトップでCodexアプリを起動
   - アプリがVR空間内のウィンドウとして表示される

3. **VRモードに入る**
   - Codexアプリ内で「Enter VR」ボタンをクリック
   - ネイティブVRモードに切り替わる

---

## 🎮 操作方法

### Quest 2（コントローラー）

- **左スティック**: 移動（前後左右）
- **右スティック**: 回転（左右）
- **トリガー**: ノード選択
- **グリップ**: タイムラインスクラブ
- **Aボタン**: メニュー表示
- **Bボタン**: 戻る
- **メニューボタン**: Virtual Desktopメニュー

### Quest 3/Pro（ハンドトラッキング）

- **ピンチ**: ノード選択
- **手のひら**: メニュー表示
- **コントローラー**: Quest 2と同じ操作

### Virtual Desktop操作

- **左メニューボタン**: Virtual Desktop設定
- **右メニューボタン**: デスクトップ操作
- **グリップ**: ウィンドウ移動
- **トリガー**: クリック

---

## ⚙️ Virtual Desktop最適化設定

### Streamer設定（PC側）

1. **Streamerを開く**
2. **Settings** → **Graphics**
   - **VR Graphics Quality**: High（RTX 3080推奨）
   - **VR Frame Rate**: 90Hz（Quest 2） / 120Hz（Quest 3）
   - **VR Bitrate**: 100-150 Mbps
   - **Sliced Encoding**: ON（低レイテンシ）
   - **Video Buffering**: OFF

3. **Settings** → **Advanced**
   - **Automatically adjust bitrate**: ON
   - **Use HEVC codec**: ON（H.264より高品質）
   - **Increase color vibrance**: ON（視覚品質向上）

### Quest側設定

1. **Virtual Desktop内で設定を開く**
2. **Graphics**
   - **Refresh Rate**: 90Hz（Quest 2） / 120Hz（Quest 3）
   - **VR Graphics Quality**: High
   - **VR Bitrate**: 100-150 Mbps

3. **Controllers**
   - **Controller Vibration**: ON
   - **Hand Tracking**: ON（Quest 3/Pro）

---

## 🔍 テスト項目

### 基本機能

- [ ] Virtual DesktopでPCデスクトップが表示される
- [ ] Codexアプリが起動できる
- [ ] VRモードに入れる
- [ ] コミットノードが3D空間に表示される
- [ ] コントローラー/ハンドトラッキングで操作できる
- [ ] ノード選択が動作する
- [ ] タイムライン操作が動作する

### パフォーマンス

- [ ] フレームレート: Quest 2で90fps、Quest 3で120fps
- [ ] レイテンシ: <30ms（Virtual Desktop経由）
- [ ] ビットレート: 100-150 Mbps維持
- [ ] メモリ使用量: 適切な範囲内

### Virtual Desktop固有

- [ ] ストリーミング品質: 高品質維持
- [ ] ネットワーク帯域幅: 十分な帯域幅確保
- [ ] コントローラー入力: 正しく認識される
- [ ] ハンドトラッキング: 動作する（Quest 3/Pro）

---

## 🐛 トラブルシューティング

### 問題1: Virtual Desktopで接続できない

**原因**: ファイアウォールがブロックしている、またはネットワーク設定

**解決策**:
1. WindowsファイアウォールでVirtual Desktop Streamerを許可
2. ルーターのUPnPを有効化
3. PCとQuestが同じWi-Fiネットワークに接続されていることを確認

### 問題2: レイテンシが高い（>50ms）

**原因**: ネットワーク帯域幅不足、またはWi-Fi設定

**解決策**:
1. 5GHz Wi-Fiを使用（2.4GHzは避ける）
2. Wi-Fi 6ルーターを使用（推奨）
3. PCを有線LANに接続
4. 他のデバイスの帯域幅使用を制限
5. Virtual Desktopのビットレートを下げる（100 Mbps → 80 Mbps）

### 問題3: フレームレートが低い

**原因**: GPU性能不足、または設定

**解決策**:
1. GPUドライバーを最新版に更新
2. Virtual DesktopのVR Graphics Qualityを下げる（High → Medium）
3. Codexアプリの描画品質を下げる
4. 他のアプリケーションを終了

### 問題4: VRモードに入れない

**原因**: WebXR APIがサポートされていない、または設定

**解決策**:
1. Questで開発者モードを有効化
2. Virtual Desktop内でブラウザを起動してWebXRをテスト
3. CodexアプリのVR設定を確認

### 問題5: コントローラー入力が認識されない

**原因**: Virtual Desktopの設定、またはアプリ側の設定

**解決策**:
1. Virtual DesktopのController設定を確認
2. CodexアプリのVR設定でコントローラー入力を有効化
3. Questのコントローラーを再ペアリング

---

## 📊 テスト結果記録

### テスト環境

- **デバイス**: [Quest 2 / Quest 3 / Quest Pro]
- **Virtual Desktop**: [バージョン]
- **Streamer**: [バージョン]
- **PC IP**: [192.168.x.x]
- **OS**: Windows 11
- **GPU**: [RTX 3080]
- **Wi-Fi**: [5GHz / Wi-Fi 6]

### テスト結果

| 項目 | 結果 | 備考 |
|------|------|------|
| Virtual Desktop接続 | ✅ / ❌ | |
| レイテンシ | [ms] | |
| ビットレート | [Mbps] | |
| フレームレート | [fps] | |
| VRモード起動 | ✅ / ❌ | |
| コントローラー操作 | ✅ / ❌ | |
| ハンドトラッキング | ✅ / ❌ | Quest 3/Proのみ |
| ノード選択 | ✅ / ❌ | |
| タイムライン操作 | ✅ / ❌ | |

### 発見した問題

1. [問題の説明]
2. [再現手順]
3. [期待される動作]

---

## 🔗 関連リンク

- [Virtual Desktop公式サイト](https://www.vrdesktop.net/)
- [Virtual Desktop設定ガイド](https://www.vrdesktop.net/setup/)
- [Quest Developer Documentation](https://developer.oculus.com/)

---

## 💡 最適化のヒント

### ネットワーク最適化

1. **Wi-Fi 6ルーター使用**
   - 最大帯域幅: 9.6 Gbps
   - 低レイテンシ: <10ms

2. **専用Wi-Fiネットワーク**
   - Quest専用の5GHzネットワークを作成
   - 他のデバイスの干渉を避ける

3. **PCを有線LANに接続**
   - Wi-Fi経由ではなく有線LANを使用
   - レイテンシをさらに削減

### GPU最適化

1. **NVIDIA設定**
   - 「NVIDIAコントロールパネル」→「3D設定」
   - 「最大プリレンダーフレーム数」: 1
   - 「垂直同期」: OFF

2. **Windows設定**
   - 「ゲームモード」を有効化
   - 「ハードウェア加速GPUスケジューリング」を有効化

---

**テスト完了後**: 結果を `_docs/2025-11-15_VirtualDesktop-VR実機テスト結果.md` に記録してください。

