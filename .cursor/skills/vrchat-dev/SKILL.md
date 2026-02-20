---
name: vrchat-dev
description: "VRChat world/avatar development with UdonSharp, modularavatar, liltoon, VRCFury, VRChatSDK3 latest spec."
---

# VRChat Dev Agent Skill

## Overview

VRChat SDK3を用いたワールド・アバター開発の完全自動化スキル。
UdonSharp、Modular Avatar、liltoon、VRCFury/VRCLightPlugin、PhysBones等の最新仕様に対応。

## Capabilities

### アバター開発
- **Modular Avatar (MA)**: MA Merge Armature / MA Bone Proxy / MA Parameters / MA Menu Installer による非破壊アバター改変
- **PhysBones**: コライダー設定、自然な揺れ物、髪・スカート・耳の設定
- **Contacts**: ContactSender / ContactReceiver によるインタラクション実装
- **FX Layer**: VRCParameterDriver、VRCAvatarDescriptor の設定
- **VRCFury / VRCLight Plugin**: アセット配布用ギミック構築

### シェーダー
- **liltoon**: 最新版 (v1.9+) 対応、ノーマルマップ、MatCap、Rim Light、Outline、Emission
- **Poiyomi Toon**: Pro/Patreon版機能対応
- **Sunao Shader**: シングルパス対応設定

### ワールド開発
- **UdonSharp (v2.x)**: C# ライクスクリプトでのインタラクティブオブジェクト実装
- **VRCStation**: 椅子・乗り物実装
- **VRCPickup**: 拾えるオブジェクト
- **Udon Network Sync**: 同期変数・RPC・PlayerModsによるマルチプレイヤー処理
- **VRCWorld**: リスポーン・タイムゾーン・コンテンツ設定
- **Audio Source**: 空間音響・音楽プレイヤー実装
- **Mirror**: 高品質ミラー実装
- **PostProcessing**: URP/PPv2 対応の視覚エフェクト

### ツール統合
- **VRChat Creator Companion (VCC)**: プロジェクト管理・パッケージ更新
- **Gesture Manager**: エディタ上でのアバターテスト
- **Avatar 3.0 Emulator**: FX レイヤーのデバッグ

## Latest SDK3 Spec (2025-2026)

```
VRChat SDK: 3.7.x+
UdonSharp: 2.x (C# subset)
Modular Avatar: 1.10.x+
liltoon: 1.9.x+
VRChatSDK Unity Version: 2022.3.22f1
```

### UdonSharp 2.x 例

```csharp
using UdonSharp;
using UnityEngine;
using VRC.SDKBase;
using VRC.Udon;

[UdonBehaviourSyncMode(BehaviourSyncMode.Manual)]
public class InteractiveButton : UdonSharpBehaviour
{
    [UdonSynced] private bool _isActive = false;
    
    public override void Interact()
    {
        if (!Networking.IsOwner(gameObject))
            Networking.SetOwner(Networking.LocalPlayer, gameObject);
        
        _isActive = !_isActive;
        RequestSerialization();
        OnDeserialization();
    }
    
    public override void OnDeserialization()
    {
        // 同期後の処理
        GetComponent<Renderer>().material.color = _isActive ? Color.green : Color.red;
    }
}
```

### Modular Avatar 例

```csharp
// MA Merge Armature でアーマチュアをマージ
// Inspector: ModularAvatarMergeArmature コンポーネント追加
// LockPhysBonesToParent: true (推奨)
// PrefixSuffix: アバターのプレフィックスに合わせる

// MA Menu Installer
// Controls: VRCExpressionsMenu.Control[] で動的メニュー構築
```

### liltoon 設定例

```csharp
// liltoon マテリアル主要プロパティ
// _MainTex: メインテクスチャ
// _Color: 基本色 (HDR対応)
// _NormalMap: 法線マップ (強度: _BumpScale)
// _ShadowStrengthMask: 影の強さマスク  
// _RimColor: リムライト色
// _EmissionColor: エミッションHDR
// _OutlineWidth: アウトライン幅 (0.0-1.0)
// _OutlineColor: アウトライン色
```

## Usage

```bash
# アバターギミック実装
codex $vrchat-dev "キャラクターにMA対応の羽ギミックを実装して。liltoonでエミッション対応"

# ワールド作成  
codex $vrchat-dev "UdonSharpでスコアボード付きのミニゲームワールドを実装して"

# シェーダー設定
codex $vrchat-dev "liltoon v1.9でトゥーンシェーディングマテリアルをセットアップして"

# アセット作成
codex $vrchat-dev "VRCFuryでギミック付きコスチュームパッケージを作成して"
```

## Workflow

1. **要件分析**: アバター/ワールドの仕様をヒアリング
2. **スケルトン確認**: アーマチュア構造・ボーン名を確認
3. **MA設定**: 非破壊改変パイプライン構築
4. **UdonSharp実装**: インタラクティブ機能コーディング
5. **シェーダー設定**: liltoon/Poiyomiマテリアル調整
6. **テスト**: Gesture Manager / Av3 Emulator でデバッグ
7. **アップロード**: VCC → Build & Publish

## References

- [VRChat SDK3 Docs](https://creators.vrchat.com/sdk/)
- [UdonSharp Documentation](https://udonsharp.docs.vrchat.com/)
- [Modular Avatar Docs](https://modular-avatar.nadena.dev/)
- [liltoon GitHub](https://github.com/lilxyzw/lilToon)
- [VRCFury Docs](https://vrcfury.com/docs)
- [VRCLight Plugin](https://github.com/Narazaka/VRCLightPlugin)
- [VCC (VRChat Creator Companion)](https://vcc.docs.vrchat.com/)

---

**Version**: 2.0.0  
**SDK Target**: VRChat SDK 3.7.x / Unity 2022.3.22f1  
**Compatibility**: Cursor IDE + Codex
