---
name: yukkuri-movie
description: "ゆっくりMovieMaker (YMM4) video production skill with character animation, scene management, and MIDI integration."
---

# Yukkuri MovieMaker Agent Skill

## Overview

ゆっくりMovieMaker (YMM4) video production skill with character animation, scene management, and MIDI integration. Automates video production workflows for VTuber content creation using YukkuriMovieMaker.

## Capabilities

- **Character Animation**: Create and animate ゆっくり characters
- **Scene Management**: Manage scenes, timelines, and transitions
- **Audio Effects**: Configure audio effects and MIDI integration
- **Video Effects**: Apply video effects and filters
- **Project Management**: Create and organize YMM4 projects
- **Rendering**: Trigger and manage video rendering
- **Plugin Integration**: Work with YMM4 plugin architecture

## Tools Required

### MCP Tools

- `ymm4_create_project` - Create new YMM4 project
- `ymm4_add_scene` - Add scenes to project
- `ymm4_add_character` - Add ゆっくり characters
- `ymm4_animate_character` - Animate character movements
- `ymm4_configure_audio` - Configure audio effects
- `ymm4_configure_video` - Configure video effects
- `ymm4_render` - Trigger video rendering
- `ymm4_export_project` - Export project files

### File System Access

- **Read**: Full project access
- **Write**: `./ymm4-projects/`, `./artifacts/`, `*.ymmp`, `*.wav`

### Network Access

- **None required** - Local YMM4 operations

### Shell Commands

- `YukkuriMovieMaker.exe` - YMM4 installation
- `python` - Python for automation scripts
- `git` - Version control

## Usage Examples

### Basic Project Creation

```bash
codex $yukkuri-movie "Create new YMM4 project with default settings"
```

### Character Animation

```bash
codex $yukkuri-movie "Create animation for ゆっくりキャラ"
```

### Scene Management

```bash
codex $yukkuri-movie "Setup scene transitions and effects"
```

## YMM4 Project Structure

```
ymm4-project/
├── project/
│   └── project.ymmp
├── characters/
│   ├── character1/
│   │   ├── pose1.pose
│   │   ├── pose2.pose
│   │   └── motion.motion
│   └── character2/
├── backgrounds/
│   ├── bg1.png
│   └── bg2.jpg
├── audio/
│   ├── bgm/
│   ├── se/
│   └── voice/
├── effects/
│   ├── videoeffects/
│   └── audioeffects/
└── output/
    └── rendered_video.mp4
```

## YMM4 Plugin Architecture

YMM4 supports plugins via COM interface:

```csharp
// Example YMM4 Plugin Interface
public interface IMovieMakerPlugin
{
    void Initialize(IMovieMakerApi api);
    void OnSceneChanged(SceneChangedEventArgs args);
    void OnRenderProgress(RenderProgressEventArgs args);
}
```

### Plugin Types

| Type        | Description          | Extension Point     |
| ----------- | -------------------- | ------------------- |
| AudioEffect | Audio processing     | `[AudioEffect]`     |
| VideoEffect | Video processing     | `[VideoEffect]`     |
| Full Plugin | Complete integration | `IMovieMakerPlugin` |

## ゆっくりCharacter Setup

ゆっくり characters have:

- Multiple facial expressions
- Body movement ranges
- Lip sync capability
- Eye tracking
- Accessory support

## Output Format

The yukkuri-movie agent provides:

- YMM4 project configurations
- Character animation setups
- Scene transition scripts
- Render job configurations
- Export packages

## References

- [ゆっくりMovieMaker Official](https://imammura.channel.jp/)
- [YMM4 Plugin Documentation](https://booth.pm/ja/items/2666730)
- [ゆっくり素材配布所](https://sanabou.com/)
- [YMM4 API Reference](https://github.com/o8o8o8/Ymm4ApiDoc)

---

**Installation**: `$ codex $skill-install https://github.com/zapabob/codex-yukkuri-movie-skill`
**Version**: 1.0.0
**Compatibility**: Codex v2.14.0+
