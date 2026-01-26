---
name: Git4D可視化へのVirtualDesktop対応統合
overview: Git4D可視化機能（Git4DVisualization、Git4DWebXRFramework）にVirtualDesktop検出と最適化を統合し、Rust側のデバイス検出ロジックにもVirtualDesktopプラットフォームを追加する
todos:
  - id: git4d_vd_gui_visualization
    content: Git4DVisualizationコンポーネントにVirtualDesktop検出と最適化を統合
    status: completed
  - id: git4d_vd_gui_webxr
    content: Git4DWebXRFrameworkコンポーネントにVirtualDesktop最適化を統合
    status: completed
  - id: git4d_vd_rust_detection
    content: Rust側git4d_accelerated.rsでVirtualDesktopプラットフォーム検出を追加
    status: completed
  - id: git4d_vd_api_endpoint
    content: GUI側APIエンドポイントでVirtualDesktopモードを認識できるように拡張
    status: completed
  - id: git4d_vd_page_integration
    content: git4d/page.tsxでVirtualDesktop検出を統合
    status: completed
isProject: false
---

# Git4D可視化へのVirtualDesktop対応統合計画

## 現状分析

### 既存実装

- `codex-rs/tauri-gui/src/utils/virtualdesktop-optimizer.ts`: VirtualDesktop検出と最適化機能は実装済み
- `codex-rs/core/src/vr_ar_integration.rs`: `XRPlatform::VirtualDesktop`は定義済みだが、Git4D可視化での検出に未統合
- `gui/src/components/visualization/Git4DVisualization.tsx`: VirtualDesktop対応なし
- `gui/src/components/visualization/Git4DWebXRFramework.tsx`: VirtualDesktop対応なし

### 問題点

1. Git4D可視化コンポーネントでVirtualDesktop検出が行われていない
2. VirtualDesktop最適化（品質プリセット、レンダリング調整）が適用されていない
3. Rust側のデバイス検出でVirtualDesktopプラットフォームが考慮されていない

## 実装計画

### Phase 1: GUI側Git4DVisualizationコンポーネントへの統合

**ファイル**: `gui/src/components/visualization/Git4DVisualization.tsx`

1. VirtualDesktopOptimizerのインポートと初期化

   - `virtualdesktop-optimizer.ts`から`useVirtualDesktopOptimizer`フックをインポート
   - コンポーネント内でVirtualDesktop検出を実行

2. VirtualDesktop検出時の最適化適用

   - 検出時に自動的に品質プリセットを適用
   - レンダリング解像度、FPS、LOD設定を調整

3. レンダリング設定の動的調整

   - Three.jsレンダラーの解像度をVirtualDesktopプリセットに合わせて調整
   - アニメーションループのFPS制限を適用

### Phase 2: GUI側Git4DWebXRFrameworkコンポーネントへの統合

**ファイル**: `gui/src/components/visualization/Git4DWebXRFramework.tsx`

1. VirtualDesktopOptimizerの統合

   - WebXRセッション開始前にVirtualDesktop検出を実行
   - VR/ARモード時にVirtualDesktop最適化を適用

2. WebXRセッション設定の最適化

   - VirtualDesktop検出時はストリーミング最適化を有効化
   - レンダリング品質を自動調整

3. パフォーマンスモニタリング

   - FPS測定とVirtualDesktopプリセットの動的調整

### Phase 3: Rust側デバイス検出ロジックの拡張

**ファイル**: `codex-rs/core/src/git4d_accelerated.rs`

1. VirtualDesktopプラットフォーム検出の追加

   - `check_vr_ar_device_availability`関数を拡張
   - VRモード時にVirtualDesktopを優先的に検出
   - UserAgentや環境変数からVirtualDesktop接続を判定

2. プラットフォーム判定ロジックの改善
   ```rust
   // VRモード時のプラットフォーム検出順序
   // 1. VirtualDesktop (Quest経由のストリーミング)
   // 2. WebXR (ブラウザベース)
   // 3. Oculus Quest (直接接続)
   ```


### Phase 4: GUI側APIエンドポイントの拡張

**ファイル**: `codex-rs/gui/src/main.rs`

1. VirtualDesktopモードの認識

   - `Git4DLaunchRequest`に`virtualDesktop: bool`フィールドを追加（オプション）
   - リクエスト時にVirtualDesktop検出結果を送信

2. レスポンスの拡張

   - `Git4DLaunchResponse`に検出されたプラットフォーム情報を追加

### Phase 5: git4d/page.tsxでの統合

**ファイル**: `gui/src/app/git4d/page.tsx`

1. VirtualDesktop検出の追加

   - ページ読み込み時にVirtualDesktop検出を実行
   - 検出結果をAPIリクエストに含める

2. ユーザーへの通知

   - VirtualDesktop検出時に最適化が適用されたことを表示

## 実装詳細

### VirtualDesktop検出ロジック

```typescript
// Git4DVisualization.tsx内
const { preset, isVD, changePreset } = useVirtualDesktopOptimizer();

useEffect(() => {
  if (isVD && (vrMode || arMode)) {
    // VirtualDesktop検出時は自動的に最適化プリセットを適用
    console.log('VirtualDesktop detected - applying optimizations');
    // レンダリング設定を調整
    if (rendererRef.current) {
      const scale = preset.renderScale;
      // 解像度調整
    }
  }
}, [isVD, vrMode, arMode, preset]);
```

### Rust側VirtualDesktop検出

```rust
// git4d_accelerated.rs内
// VirtualDesktop検出のヒューリスティック
fn detect_virtual_desktop() -> bool {
    // 1. 環境変数チェック
    if std::env::var("VIRTUAL_DESKTOP_STREAMER").is_ok() {
        return true;
    }
    
    // 2. プロセスチェック（Windows）
    #[cfg(windows)]
    {
        use std::process::Command;
        let output = Command::new("tasklist")
            .arg("/FI")
            .arg("IMAGENAME eq VirtualDesktop.Streamer.exe")
            .output();
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("VirtualDesktop.Streamer.exe") {
                return true;
            }
        }
    }
    
    false
}
```

## テスト計画

1. **VirtualDesktop検出テスト**

   - VirtualDesktop接続時の自動検出確認
   - 最適化プリセットの適用確認

2. **レンダリング品質テスト**

   - 異なる品質プリセットでのFPS測定
   - ストリーミング時のレイテンシ測定

3. **統合テスト**

   - VRモード起動時のVirtualDesktop検出と最適化
   - デスクトップモードとの切り替え

## 注意事項

- VirtualDesktop検出はヒューリスティックベース（完全確実ではない）
- 品質プリセットはユーザーが手動で変更可能にする
- パフォーマンス低下時は自動的に低品質プリセットに切り替え
- Rust側の検出はWindows環境を優先（他のOSでも動作するが精度が低い可能性）

## 完了確認

- [ ] Git4DVisualizationコンポーネントにVirtualDesktop最適化を統合
- [ ] Git4DWebXRFrameworkコンポーネントにVirtualDesktop最適化を統合
- [ ] Rust側デバイス検出にVirtualDesktopプラットフォームを追加
- [ ] GUI側APIエンドポイントでVirtualDesktopモードを認識
- [ ] git4d/page.tsxでVirtualDesktop検出を統合
- [ ] テスト完了