---
name: blender-cad
description: "Blender CAD modeling automation: STEP/IGES import, Geometry Nodes, parametric design, multi-format export."
---

# Blender CAD Agent Skill

## Overview

Blender Python API を使用したCADモデリング自動化スキル。
STEP/IGES インポート、Geometry Nodes、マテリアル管理、レンダリング、マルチフォーマットエクスポートに対応。

## Capabilities

### CADインポート・変換
- **STEP/IGES/OBJ/FBX** インポート (FreeCAD経由またはBlender直接)
- **STL** → Blender メッシュ変換
- **CAD → Blender** ワークフロー自動化

### モデリング
- **Geometry Nodes**: パラメトリックデザイン、プロシージャル生成
- **Boolean Operations**: Union / Difference / Intersect
- **Subdivision Surface**: 高品質スムーシング
- **Array Modifier**: 繰り返しパターン生成
- **Screw / Spin**: 回転体モデリング

### マテリアル & レンダリング
- **PBR マテリアル**: Metallic / Roughness / Normal Map 設定
- **HDRI 照明**: 高品質ライティング
- **Cycles / EEVEE**: フォトリアルとリアルタイムレンダリング
- **AOV (Arbitrary Output Variable)**: パスレンダリング
- **RTX 対応**: CUDA/OptiX レンダリング加速

### エクスポート
- **FBX**: Unity/Unreal Engine向け
- **OBJ / PLY**: 汎用3Dフォーマット
- **USD / USDZ**: Apple AR / Pixar USD
- **glTF 2.0**: Web 3D / AR 向け
- **STL**: 3Dプリント用

## Blender Python API 例

### 基本的なメッシュ操作

```python
import bpy
import bmesh

def create_parametric_box(width=1.0, height=1.0, depth=1.0, name="Box"):
    """パラメトリックボックス作成"""
    mesh = bpy.data.meshes.new(name)
    obj = bpy.data.objects.new(name, mesh)
    bpy.context.collection.objects.link(obj)
    
    bm = bmesh.new()
    bmesh.ops.create_cube(bm, size=1)
    
    # スケールを適用
    bmesh.ops.scale(bm, vec=(width, depth, height), verts=bm.verts)
    
    bm.to_mesh(mesh)
    bm.free()
    
    return obj
```

### Geometry Nodes セットアップ

```python
def setup_geometry_nodes(obj, pattern_count=5):
    """Geometry Nodes でパターン生成"""
    # Geometry Nodes モディファイアを追加
    modifier = obj.modifiers.new("GeometryNodes", 'NODES')
    node_group = bpy.data.node_groups.new("PatternNodes", 'GeometryNodeTree')
    modifier.node_group = node_group
    
    nodes = node_group.nodes
    links = node_group.links
    
    # 入力・出力
    group_in = nodes.new('NodeGroupInput')
    group_out = nodes.new('NodeGroupOutput')
    
    # Array Instance ノード
    array_node = nodes.new('GeometryNodeRepeatOutput')
    
    links.new(group_in.outputs[0], array_node.inputs[0])
    links.new(array_node.outputs[0], group_out.inputs[0])
```

### STEPファイルインポート (FreeCAD連携)

```python
import subprocess
import os

def import_step_via_freecad(step_path: str, output_obj: str) -> bool:
    """FreeCADを使ってSTEPをOBJに変換してからBlenderでインポート"""
    # FreeCAD Python でSTEP→OBJ変換
    freecad_script = f"""
import FreeCAD
import Part
import Mesh

doc = FreeCAD.newDocument()
shape = Part.Shape()
shape.read("{step_path}")
Part.show(shape)

# メッシュ化
mesh = Mesh.Mesh()
Mesh.export([doc.Objects[0]], "{output_obj}", 0.1)
FreeCAD.closeDocument(doc.Name)
"""
    with open("_temp_fc_import.py", "w") as f:
        f.write(freecad_script)
    
    result = subprocess.run(["freecadcmd", "_temp_fc_import.py"], capture_output=True)
    
    if os.path.exists(output_obj):
        bpy.ops.import_scene.obj(filepath=output_obj)
        return True
    return False
```

### マテリアル自動設定

```python
def apply_pbr_material(obj, color=(0.8, 0.8, 0.8, 1.0), 
                        metallic=0.0, roughness=0.5, name="PBR_Mat"):
    """PBRマテリアルを自動設定"""
    mat = bpy.data.materials.new(name=name)
    mat.use_nodes = True
    
    nodes = mat.node_tree.nodes
    nodes.clear()
    
    # Principled BSDF
    bsdf = nodes.new('ShaderNodeBsdfPrincipled')
    bsdf.inputs['Base Color'].default_value = color
    bsdf.inputs['Metallic'].default_value = metallic
    bsdf.inputs['Roughness'].default_value = roughness
    
    # Material Output
    output = nodes.new('ShaderNodeOutputMaterial')
    mat.node_tree.links.new(bsdf.outputs['BSDF'], output.inputs['Surface'])
    
    # オブジェクトに適用
    if obj.data.materials:
        obj.data.materials[0] = mat
    else:
        obj.data.materials.append(mat)
    
    return mat
```

### RTX Cycles レンダリング

```python
def setup_cycles_rtx_render(output_path: str, samples=128, 
                              width=1920, height=1080):
    """RTX/CUDA Cycles レンダリング設定"""
    scene = bpy.context.scene
    scene.render.engine = 'CYCLES'
    scene.cycles.device = 'GPU'
    scene.cycles.samples = samples
    
    # CUDA/OptiX 選択
    prefs = bpy.context.preferences.addons['cycles'].preferences
    prefs.compute_device_type = 'OPTIX'  # RTX3080はOptiX推奨
    prefs.get_devices()
    
    for device in prefs.devices:
        device.use = True
    
    # 解像度
    scene.render.resolution_x = width
    scene.render.resolution_y = height
    scene.render.filepath = output_path
    
    bpy.ops.render.render(write_still=True)
```

## Usage

```bash
# STEPファイルからBlenderシーン作成
codex $blender-cad "gear.step をインポートしてPBRマテリアルを設定してレンダリング"

# パラメトリックCADモデル
codex $blender-cad "Geometry NodesでM3ネジのパラメトリックモデルを作成"

# アニメーションレンダリング
codex $blender-cad "機械部品の分解アニメーションを作成してMP4でエクスポート"

# VRChat向け最適化
codex $blender-cad "モデルを VRChat用に最適化 (ポリゴン削減・アトラス化・FBXエクスポート)"
```

## Workflow

1. **インポート**: STEP/IGES/STL/FBX を読み込み
2. **クリーンアップ**: N-Gon修正・法線修正・頂点マージ
3. **リトポロジー**: ゲーム用・3Dプリント用に最適化
4. **マテリアル**: PBR/NPRマテリアル設定
5. **UV展開**: Smart UV / Unwrap
6. **レンダリング**: Cycles (RTX) / EEVEE
7. **エクスポート**: 用途別フォーマット出力

## References

- [Blender Python API](https://docs.blender.org/api/current/)
- [Geometry Nodes](https://docs.blender.org/manual/en/latest/modeling/geometry_nodes/)
- [Blender Cycles GPU](https://docs.blender.org/manual/en/latest/render/cycles/gpu_rendering.html)
- [FreeCAD Python Scripting](https://wiki.freecad.org/Python_scripting_tutorial)

---

**Version**: 2.0.0  
**Blender Target**: 4.x  
**GPU**: CUDA 12 / RTX 3080 (OptiX)  
**Compatibility**: Cursor IDE + Codex
