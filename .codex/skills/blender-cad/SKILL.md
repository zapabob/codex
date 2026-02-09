---
name: blender-cad
description: "Blender CAD modeling automation skill with STEP/IGES import, Geometry Nodes, and USD export support."
---

# Blender CAD Agent Skill

## Overview

Blender CAD modeling automation skill with STEP/IGES import, Geometry Nodes, and USD export support. Automates Blender Python scripting workflows for CAD modeling, rendering, and export automation.

## Capabilities

- **CAD Import**: Import STEP, IGES, and other CAD formats
- **Geometry Nodes**: Create parametric designs with Geometry Nodes
- **Python Scripting**: Write bpy scripts for automation
- **Material Management**: Create and manage materials
- **Render Automation**: Configure and trigger renders
- **Export Workflows**: Export to various formats (FBX, OBJ, USD, glTF)
- **Procedural Modeling**: Generate complex geometries procedurally

## Tools Required

### MCP Tools

- `blender_import_step` - Import STEP files
- `blender_import_iges` - Import IGES files
- `blender_create_geometry` - Create geometry via Python
- `blender_geometry_nodes` - Apply Geometry Nodes
- `blender_assign_material` - Assign materials
- `blender_render` - Trigger rendering
- `blender_export` - Export to various formats
- `blender_run_script` - Execute bpy scripts

### File System Access

- **Read**: Full project access
- **Write**: `./blender-projects/`, `./artifacts/`, `*.blend`, `*.py`

### Network Access

- **None required** - Local Blender operations

### Shell Commands

- `blender` - Blender installation
- `python` - Python for bpy scripts
- `git` - Version control

## Usage Examples

### Basic CAD Import

```bash
codex $blender-cad "Import STEP file and create CAD model"
```

### Geometry Nodes Design

```bash
codex $blender-cad "Create parametric design with Geometry Nodes"
```

### Render Setup

```bash
codex $blender-cad "Configure render settings and trigger render"
```

## Blender Project Structure

```
blender-project/
├── source/
│   ├── step_files/
│   ├── iges_files/
│   └── import/
├── scripts/
│   ├── run_automation.py
│   ├── geometry_nodes.py
│   └── materials.py
├── materials/
│   ├── metal.matl
│   ├── plastic.matl
│   └── wood.matl
├── renders/
│   ├── viewport/
│   └── final/
└── exports/
    ├── fbx/
    ├── obj/
    └── usd/
```

## Blender Python API

Blender 4.0+ uses bpy (Blender Python) API:

```python
import bpy

# Create mesh
mesh = bpy.data.meshes.new("Cube")
obj = bpy.data.objects.new("Cube", mesh)
bpy.context.collection.objects.link(obj)

# Geometry Nodes
modifier = obj.modifiers.new("GeometryNodes", 'NODES')
modifier.node_group = bpy.data.node_groups["MyNodeGroup"]
```

## Export Formats

| Format | Use Case             | Notes              |
| ------ | -------------------- | ------------------ |
| FBX    | Game engines, VRChat | Supports materials |
| OBJ    | General purpose      | Simple geometry    |
| USD    | VFX pipelines        | Rich metadata      |
| glTF   | Web, AR/VR           | Compact, efficient |

## Output Format

The blender-cad agent provides:

- Python scripts for automation
- Geometry Nodes setups
- Material configurations
- Render outputs
- Export packages

## References

- [Blender Python API](https://docs.blender.org/api/current/)
- [Geometry Nodes Manual](https://docs.blender.org/manual/en/latest/modeling/geometry_nodes/)
- [STEP/IGES Import](https://docs.blender.org/manual/en/latest/files/import_export.html)
- [CAD Blender Addon](https://github.com/visiblink/cad-transforms)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-blender-cad-skill`
**Version**: 1.0.0
**Compatibility**: Codex v2.14.0+
