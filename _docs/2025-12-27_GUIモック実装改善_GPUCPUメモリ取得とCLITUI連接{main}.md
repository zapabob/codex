# GUIモック実装改善・GPU/CPU/メモリ取得とCLI/TUI連接

**日時**: 2025-12-27
**ワークツリー**: main
**タスク**: GUIのモック実装を改善し、GPU/CPU/メモリの実パラメータを取得してCLI/TUIと連接

---

## 🎯 実装内容

### 1. SystemMetrics型の拡張

**ファイル**: `gui/src/lib/types/index.ts`

- GPU情報を追加:
  - `gpuUsage`: GPU使用率
  - `gpuMemoryUsed`: GPUメモリ使用量
  - `gpuMemoryTotal`: GPUメモリ総量
  - `gpuMemoryUsage`: GPUメモリ使用率
  - `gpuTemperature`: GPU温度
  - `gpuName`: GPU名
  - `gpuVendor`: GPUベンダー（nvidia/amd/intel/unknown）

### 2. ResourceMonitorコンポーネントの改善

**ファイル**: `gui/src/components/virtual-os/ResourceMonitor.tsx`

**変更点**:
- モックデータ生成を削除
- 実システム情報をAPIから取得するように変更
- WebSocketでリアルタイム更新を実装
- GPU情報の表示を追加
- システム概要セクションにGPUカードを追加

**実装詳細**:
- `CodexAPIClient`を使用してシステム情報を取得
- WebSocket接続で2秒ごとにリアルタイム更新
- フォールバック: API失敗時はCLI経由で取得を試行
- 環境別メトリクスは実システム情報を基に計算

### 3. APIクライアントの改善

**ファイル**: `gui/src/lib/api/client.ts`

**変更点**:
- `getSystemMetrics()`メソッドを改善
- バックエンドAPIレスポンスのマッピングを追加
- GPU情報の取得とマッピングを実装
- CLI/TUI経由のフォールバック機能を追加
- `executeCodexCommand()`メソッドを追加（CLI/TUI連接用）

### 4. Mock Serverの改善

**ファイル**: `gui-tests/mock-server.js`

**変更点**:
- GPU情報取得を詳細化
- GPUメモリ使用量、総量、使用率を取得
- GPU温度を取得
- GPU名とベンダーを判定
- WebSocketで送信するフォールバックデータにもGPU情報を追加

**実装詳細**:
- `systeminformation`ライブラリの`graphics()`を使用
- GPUベンダーを名前から自動判定（NVIDIA/AMD/Intel）
- メモリ使用率を計算

### 5. CLI/TUIとの連接

**実装方法**:
- 既存の`DualBridge`を活用
- `CodexContext`経由でCLI/TUIと通信
- APIクライアントに`executeCodexCommand()`メソッドを追加
- フォールバック機能: API失敗時はCLI経由で取得

---

## 📊 取得できる情報

### CPU
- 使用率（%）
- リアルタイム更新

### メモリ
- 使用率（%）
- 使用量/総量
- リアルタイム更新

### GPU
- 使用率（%）
- メモリ使用量（MB）
- メモリ総量（MB）
- メモリ使用率（%）
- 温度（°C）
- GPU名
- ベンダー（NVIDIA/AMD/Intel）

### ディスク
- 使用率（%）
- リアルタイム更新

### その他
- アクティブプロセス数
- システムアップタイム

---

## 🔌 接続方法

### 1. API経由（推奨）
- エンドポイント: `http://localhost:8787/api/system/metrics`
- ポーリング間隔: 5秒
- WebSocket: `ws://localhost:8787`（2秒ごとに更新）

### 2. CLI/TUI経由（フォールバック）
- `CodexAPIClient.executeCodexCommand(['system', 'metrics', '--json'])`
- DualBridge経由でCLI/TUIと通信

---

## ✅ 完了したタスク

1. ✅ SystemMetrics型にGPU情報を追加
2. ✅ ResourceMonitorコンポーネントを実システム情報取得に変更
3. ✅ APIクライアントにGPU情報取得を追加
4. ✅ CLI/TUIとの連接を実装（codexコマンド経由）
5. ✅ WebSocketでリアルタイム更新を実装

---

## 🎉 改善結果

- **モック実装から実システム情報取得へ**: ResourceMonitorが実際のシステム情報を表示
- **GPU情報の表示**: GPU使用率、メモリ、温度などの詳細情報を取得・表示
- **リアルタイム更新**: WebSocketで2秒ごとに自動更新
- **CLI/TUI連接**: API失敗時はCLI/TUI経由で取得を試行
- **フォールバック機能**: 複数の取得方法で確実に情報を取得

---

## 📝 次のステップ（オプション）

1. RustバックエンドのGPU情報取得を改善（現在は簡易実装）
2. ネットワーク情報の詳細取得
3. ディスク使用量の詳細取得（複数ドライブ対応）
4. プロセス別のリソース使用量表示
5. リソース使用履歴のグラフ表示

---

## 🔧 技術スタック

- **フロントエンド**: React, TypeScript, Next.js
- **バックエンド**: Node.js (mock-server.js), Rust (codex-rs)
- **システム情報取得**: systeminformation (Node.js), sysinfo (Rust)
- **リアルタイム通信**: WebSocket
- **CLI/TUI連接**: DualBridge, WebSocket RPC

---

完了！
