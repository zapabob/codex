---
name: vrchat-dev
description: "VRChat world and avatar development skill with UdonSharp, modularavatar, and PhysBones support. Automates VRChat SDK3 development workflows."
---

# VRChat Dev Agent Skill

## Overview

VRChat world and avatar development skill with UdonSharp, modularavatar, and PhysBones support. Automates VRChat SDK3 development workflows including world building, avatar configuration, and upload automation.

## Capabilities

- **UdonSharp Development**: Write, compile, and optimize Udon/UdonSharp scripts for VRChat worlds
- **World Building**: Create VRChat worlds with dynamic content, interactive objects, and optimization
- **Avatar Configuration**: Configure avatars with modularavatar, PhysBones, and contact systems
- **Shader Integration**: Work with popular shaders like liltoon and Poiyomi
- **World Upload**: Automate world upload and version management
- **Performance Optimization**: Optimize worlds for various performance tiers

## Tools Required

### MCP Tools

- `vrchat_compile_udon` - Compile UdonSharp scripts
- `vrchat_upload_world` - Upload world to VRChat
- `vrchat_upload_avatar` - Upload avatar to VRChat
- `vrchat_configure_avatar` - Configure avatar with modularavatar
- `vrchat_setup_physbones` - Setup PhysBones configuration
- `vrchat_create_contact` - Create contact systems
- `vrchat_optimize_world` - Optimize world performance

### File System Access

- **Read**: Full project access
- **Write**: `./vrchat-projects/`, `./artifacts/`, `*.cs`, `*.asset`

### Network Access

- `https://api.vrchat.cloud/` - VRChat API
- `https://vrchat.com/` - VRChat website

### Shell Commands

- `dotnet` - .NET SDK for UdonSharp
- `git` - Version control
- `node` - Node.js for tools

## Usage Examples

### Basic World Development

```bash
codex $vrchat-dev "Create a new VRChat world with interactive objects"
```

### Avatar Configuration

```bash
codex $vrchat-dev "Configure avatar with modularavatar and PhysBones"
```

### UdonSharp Scripting

```bash
codex $vrchat-dev "Write UdonSharp script for door interaction system"
```

## VRChat Project Structure

```
vrchat-project/
├── Assets/
│   ├── Scripts/
│   │   └── Udon/
│   │       ├── MainController.cs
│   │       └── InteractiveObjects/
│   ├── Shaders/
│   ├── Materials/
│   └── Prefabs/
├── Packages/
│   ├── com.vrchat.sdk3/
│   ├── com.modularavatar/
│   └── com.liltoon/
├── ProjectSettings/
└── Unity/
```

## Performance Tiers

VRChat supports multiple performance tiers:

| Tier   | Description   | Requirements                   |
| ------ | ------------- | ------------------------------ |
| Low    | Minimum specs | Simple geometry, basic shaders |
| Medium | Balanced      | Moderate complexity            |
| High   | High-end      | Complex shaders, particles     |
| Ultra  | Maximum       | Full features enabled          |

## Output Format

The vrchat-dev agent provides:

- UdonSharp code with compilation
- World configuration files
- Avatar setup scripts
- Performance optimization recommendations
- Upload-ready packages

## References

- [VRChat SDK3 Documentation](https://docs.vrchat.com/docs/sdk3)
- [UdonSharp Documentation](https://udosharp-docs.vercel.app/)
- [modularavatar](https://modularavatar.nadena.dev/)
- [PhysBones](https://docs.vrchat.com/docs/physbones)
- [VRChat Performance Stats](https://docs.vrchat.com/docs/performance-stats)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-vrchat-dev-skill`
**Version**: 1.0.0
**Compatibility**: Codex v2.14.0+
