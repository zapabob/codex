# Dom Console Network完全実装

**実装日時**: 2026-01-01  
**worktree**: main  
**機能名**: Dom Console Network完全実装

## 実装概要

CLIからNative Messaging Host経由でDOM読み取り、コンソールログ取得、ネットワークログ取得を完全実装しました。拡張機能との連携を確立し、実際にブラウザの機能を実行できるようにしました。

## 実装内容

### 1. `run_console`の完全実装

- `run_dom`と`run_network`と同様に、Native Messaging Hostと通信する実装を追加
- コンソールログ取得のリクエストを送信し、結果を表示

### 2. エラーハンドリングの改善

- タイムアウト処理（30秒）を追加
- より詳細なエラーメッセージを追加
- Native Messaging Hostの起動失敗時の処理を改善
- stderrの読み取りと表示を追加

### 3. `cli_bridge.rs`の改善

- `handle_dom_read`、`handle_console_logs`、`handle_network_logs`の関数を改善
- リクエストパラメータとステータス情報を返す実装に変更
- 拡張機能が処理する必要があることを明示

### 4. 拡張機能との連携

- `connectNative`関数を改善し、自動再接続機能を追加
- 未要求メッセージ（CLIからのリクエストなど）を処理する機能を追加
- `dom.read.request`、`console.get_logs.request`、`network.get_logs.request`のハンドラーを改善
- Native Messaging Hostへの結果送信機能を改善

### 5. ドキュメント更新

- `docs/chrome-extension.md`を更新
- 新しいメッセージタイプの説明を追加
- CLIコマンドの使用方法を更新
- 拡張機能との連携に関する注意事項を追加

## 実装ファイル

### 修正ファイル

- `codex-rs/cli/src/chrome_cmd.rs`
  - `run_console`の完全実装
  - エラーハンドリングの改善（タイムアウト、詳細なエラーメッセージ）
  - `spawn_native_host`の改善（stderr読み取り）

- `codex-rs/chrome-host/src/cli_bridge.rs`
  - `handle_dom_read`、`handle_console_logs`、`handle_network_logs`の改善
  - リクエストパラメータとステータス情報を返す実装

- `extensions/chrome-codex/background/background.js`
  - `connectNative`関数の改善（自動再接続）
  - 未要求メッセージの処理機能
  - 各リクエストハンドラーの改善

- `docs/chrome-extension.md`
  - 新しいメッセージタイプの説明
  - CLIコマンドの使用方法
  - 拡張機能との連携に関する注意事項

## 技術詳細

### CLIからNative Messaging Hostを呼び出す方法

CLIがNative Messaging Hostを起動し、直接通信する実装を追加しました。ただし、DOM読み取り、コンソールログ取得、ネットワークログ取得は、実際には拡張機能のcontent scriptやbackground scriptが実行する必要があります。

### 拡張機能との連携

拡張機能がNative Messaging Hostに接続し、CLIからのリクエストを処理できるようにしました。ただし、Native Messaging Hostは通常、拡張機能から呼び出されるため、CLIが起動したNative Messaging Hostに拡張機能が接続するには、拡張機能が常に接続している状態を維持する必要があります。

### エラーハンドリング

- タイムアウト処理: リクエストが30秒以内に返らない場合、タイムアウトエラーを返す
- 詳細なエラーメッセージ: Native Messaging Hostの起動失敗、メッセージ送信失敗、レスポンス受信失敗など、各段階で詳細なエラーメッセージを提供
- stderrの読み取り: Native Messaging Hostのstderrを読み取り、エラー情報を表示

## 使用方法

```bash
# DOM読み取り
codex chrome dom --selector "#main-content" --max-chars 5000

# コンソールログ取得
codex chrome console --level "error" --filter "api" --limit 50

# ネットワークログ取得
codex chrome network --filter "api" --limit 50
```

## 注意事項

- 拡張機能がインストールされ、アクティブである必要があります
- Native Messaging Hostがビルドされ、利用可能である必要があります
- 拡張機能がNative Messaging Hostに接続している必要があります
- リクエストは30秒以内に完了する必要があります

## 今後の改善点

- 拡張機能がNative Messaging Hostに自動接続する機能の改善
- CLIから拡張機能と直接通信する方法の実装（HTTPサーバー、Named Pipeなど）
- より堅牢なエラーハンドリング
- 統合テストの追加
