---
name: Git4D VR/AR機能のスラッシュコマンド統合
overview: git 4D可視化VR/AR機能をスラッシュコマンドからcoworkGUI経由で呼び出せるようにする。`/git4d`、`/vr`、`/ar`コマンドを追加し、GUI統合ブリッジ経由でGit4DVisualizationコンポーネントを起動する。
todos:
  - id: add_slash_commands
    content: スラッシュコマンド定義の追加（/git4d, /vr, /ar）
    status: pending
  - id: implement_command_handlers
    content: コマンドハンドラーの実装（chatwidget.rs）
    status: pending
  - id: extend_cowork_integration
    content: coworkGUI統合の拡張（Git4D可視化起動機能）
    status: pending
  - id: add_gui_api_endpoint
    content: GUI側APIエンドポイントの追加（/api/visualization/git4d）
    status: pending
  - id: integrate_gui_component
    content: GUI側コンポーネント統合（Git4DVisualization）
    status: pending
  - id: integrate_rust_git4d
    content: Rust側Git4D機能との統合（launch_for_gui）
    status: pending
  - id: error_handling
    content: エラーハンドリングとユーザーフィードバック
    status: pending
  - id: testing_verification
    content: テストと検証（全機能の動作確認）
    status: pending
isProject: false
---

# Git4D VR/AR機能のスラッシュコマンド統合計画

## 現状分析

### 既存実装

- **Git 4D VR/AR機能**: 
  - Rust側: `codex-rs/core/src/git4d_accelerated.rs`, `superior_git4d_visualizer.rs`, `vr_ar_integration.rs`
  - GUI側: `gui/src/components/visualization/Git4DVisualization.tsx`, `Git4DWebXRFramework.tsx`
- **coworkGUI統合**: `codex-rs/core/src/cowork_integration.rs`, `qc/gui_integration.rs`
- **スラッシュコマンド**: `codex-rs/tui/src/slash_command.rs`, `chatwidget.rs`の`dispatch_command`

### 実装要件

- スラッシュコマンド `/git4d`, `/vr`, `/ar` を追加
- coworkGUI経由でGit4D可視化を起動
- GUI側でGit4DVisualizationコンポーネントを表示

## 実装フェーズ

### Phase 1: スラッシュコマンド定義の追加

**ファイル**: `codex-rs/tui/src/slash_command.rs`

1. `SlashCommand` enumに以下を追加:

   - `Git4d` - Git 4D可視化の起動
   - `Vr` - VRモードでの可視化
   - `Ar` - ARモードでの可視化

2. `description()`メソッドに説明を追加:

   - `Git4d`: "launch Git 4D visualization with VR/AR support"
   - `Vr`: "launch Git 4D visualization in VR mode"
   - `Ar`: "launch Git 4D visualization in AR mode"

3. `available_during_task()`でタスク中でも利用可能に設定

### Phase 2: コマンドハンドラーの実装

**ファイル**: `codex-rs/tui/src/chatwidget.rs`

1. `dispatch_command()`メソッドに以下を追加:
   ```rust
   SlashCommand::Git4d | SlashCommand::Vr | SlashCommand::Ar => {
       let mode = match cmd {
           SlashCommand::Vr => "vr",
           SlashCommand::Ar => "ar",
           _ => "desktop",
       };
       self.launch_git4d_visualization(mode);
   }
   ```

2. `launch_git4d_visualization()`メソッドを実装:

   - coworkGUI統合を使用してGit4D可視化を起動
   - `CoworkIntegrationManager`または`GUICLIBridge`を使用
   - モード（vr/ar/desktop）を引数として渡す

### Phase 3: coworkGUI統合の拡張

**ファイル**: `codex-rs/core/src/cowork_integration.rs`

1. `CoworkFeature` enumに`Git4DVisualization`を追加

2. `launch_git4d_visualization()`メソッドを追加:

   - GUI側のAPIエンドポイントを呼び出す
   - またはPythonスクリプト経由でGUIを起動
   - モードパラメータを渡す

**ファイル**: `codex-rs/core/src/qc/gui_integration.rs`

1. `GUICommandRequest`でGit4D可視化コマンドをサポート
2. GUI側へのリクエスト送信機能を実装

### Phase 4: GUI側APIエンドポイントの追加

**ファイル**: `codex-rs/gui/src/main.rs` または GUI側のAPIルーター

1. Git4D可視化起動用のAPIエンドポイントを追加:

   - `POST /api/visualization/git4d` - Git4D可視化の起動
   - パラメータ: `mode` (vr/ar/desktop), `repository_path` (optional)

2. エンドポイントハンドラーで:

   - Git4D可視化セッションを開始
   - セッションIDを返す
   - WebSocketまたはSSEでリアルタイム更新を提供

### Phase 5: GUI側コンポーネント統合

**ファイル**: `gui/src/components/visualization/Git4DVisualization.tsx`

1. APIエンドポイントから呼び出し可能にする
2. モードパラメータ（vr/ar/desktop）を受け取る
3. 既存の`Git4DVisualization`コンポーネントを活用

**ファイル**: `gui/src/lib/context/CodexContext.tsx` またはルーティング

1. Git4D可視化ページ/ルートを追加
2. スラッシュコマンドからの起動をサポート

### Phase 6: Rust側Git4D機能との統合

**ファイル**: `codex-rs/core/src/git4d_accelerated.rs`

1. GUIからの起動をサポートする関数を追加:
   ```rust
   pub async fn launch_for_gui(
       repository_path: PathBuf,
       mode: VisualizationMode,
   ) -> Result<Git4DVisualizationSession>
   ```

2. セッション管理機能を追加:

   - セッションIDの生成
   - セッション状態の管理
   - WebSocket/SSE経由でのデータ送信

### Phase 7: エラーハンドリングとユーザーフィードバック

1. エラーケースの処理:

   - リポジトリが見つからない場合
   - VR/ARデバイスが利用できない場合
   - GUIが起動していない場合

2. ユーザーへの通知:

   - 起動中のメッセージ表示
   - エラー時の適切なメッセージ
   - 成功時の確認

### Phase 8: テストと検証

1. スラッシュコマンドの動作確認
2. GUI統合の動作確認
3. VR/ARモードの動作確認（デバイスがある場合）
4. エラーハンドリングの確認

## 実装順序

1. **Phase 1**: スラッシュコマンド定義の追加
2. **Phase 2**: コマンドハンドラーの実装
3. **Phase 3**: coworkGUI統合の拡張
4. **Phase 4**: GUI側APIエンドポイントの追加
5. **Phase 5**: GUI側コンポーネント統合
6. **Phase 6**: Rust側Git4D機能との統合
7. **Phase 7**: エラーハンドリング
8. **Phase 8**: テストと検証

## 技術的考慮事項

### アーキテクチャ

- TUI (CLI) → coworkGUI統合 → GUI API → Reactコンポーネント
- 非同期通信（WebSocket/SSE）でリアルタイム更新
- セッション管理で複数の可視化を同時にサポート

### 依存関係

- 既存の`cowork_integration.rs`を拡張
- 既存の`Git4DVisualization.tsx`を再利用
- GUI側のAPIルーターに統合

### パフォーマンス

- 起動時間の最適化
- メモリ使用量の管理
- VR/ARモードでのレンダリング性能

## 成功基準

1. `/git4d`, `/vr`, `/ar`コマンドが正常に動作
2. coworkGUI経由でGit4D可視化が起動
3. GUI側でGit4DVisualizationコンポーネントが表示
4. VR/ARモードが正常に動作（デバイスがある場合）
5. エラーハンドリングが適切に機能
6. 既存機能に影響がない

## 参考ファイル

- スラッシュコマンド定義: `codex-rs/tui/src/slash_command.rs`
- コマンドハンドラー: `codex-rs/tui/src/chatwidget.rs`
- cowork統合: `codex-rs/core/src/cowork_integration.rs`
- GUI統合: `codex-rs/core/src/qc/gui_integration.rs`
- Git4D実装: `codex-rs/core/src/git4d_accelerated.rs`
- GUIコンポーネント: `gui/src/components/visualization/Git4DVisualization.tsx`