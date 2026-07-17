#!/usr/bin/env python3
"""
Migrate existing YAML-based agents to official SKILL.md format
"""

import os
import yaml
from pathlib import Path
from typing import Dict, Any


def load_yaml_agent(agent_path: Path) -> Dict[str, Any]:
    """Load existing YAML agent configuration"""
    with open(agent_path, "r", encoding="utf-8") as f:
        return yaml.safe_load(f)


def convert_tools_to_markdown(tools: Dict[str, Any]) -> str:
    """Convert tools configuration to markdown format"""
    sections = []

    if "mcp" in tools:
        mcp_tools = tools["mcp"]
        if mcp_tools:
            sections.append("### MCP Tools")
            for tool in mcp_tools:
                sections.append(f"- `{tool}`")

    if "fs" in tools:
        fs_config = tools["fs"]
        sections.append("### File System Access")
        if fs_config.get("read"):
            sections.append("- **Read**: Full codebase access")
        if "write" in fs_config:
            write_paths = fs_config["write"]
            sections.append(f"- **Write**: Limited to {', '.join(write_paths)}")

    if "net" in tools:
        net_config = tools["net"]
        if "allow" in net_config:
            sections.append("### Network Access")
            for url in net_config["allow"]:
                sections.append(f"- {url}")

    if "shell" in tools:
        shell_config = tools["shell"]
        if "exec" in shell_config:
            sections.append("### Shell Commands")
            for cmd in shell_config["exec"]:
                sections.append(f"- `{cmd}`")

    return "\n".join(sections)


def create_skill_md(agent_name: str, agent_config: Dict[str, Any]) -> str:
    """Create SKILL.md content from agent configuration"""

    goal = agent_config.get("goal", f"Perform {agent_name} tasks")
    tools_md = convert_tools_to_markdown(agent_config.get("tools", {}))

    skill_md = f"""# {agent_name.title()} Agent Skill

## Overview

{goal}

## Capabilities

- {agent_name.title()}-specific analysis and recommendations
- Automated code review and improvements
- Best practices enforcement

## Tools Required

{tools_md}

## Usage Examples

### Basic Usage
```
codex ${agent_name} "Perform {agent_name} analysis on this codebase"
```

### Advanced Usage
```
codex ${agent_name} "Review and improve the {agent_name} implementation"
```

## Output Format

The {agent_name} agent provides:
- Detailed analysis reports
- Code improvement suggestions
- Best practices recommendations
- Automated fixes where applicable

## References

- [OpenAI Codex Documentation](https://developers.openai.com/codex)
- [Agent Skills Guide](https://developers.openai.com/codex/skills)
"""

    return skill_md


def migrate_agent(agent_name: str):
    """Migrate a single agent to SKILL.md format"""

    # Paths
    yaml_path = Path(".codex/agents") / f"{agent_name}.yaml"
    skill_dir = Path(".codex/skills") / agent_name
    skill_md_path = skill_dir / "SKILL.md"

    if not yaml_path.exists():
        print(f"[ERROR] YAML file not found: {yaml_path}")
        return False

    try:
        # Load YAML configuration
        agent_config = load_yaml_agent(yaml_path)

        # Create skill directory
        skill_dir.mkdir(parents=True, exist_ok=True)

        # Create SKILL.md
        skill_md_content = create_skill_md(agent_name, agent_config)
        with open(skill_md_path, "w", encoding="utf-8") as f:
            f.write(skill_md_content)

        # Create scripts directory structure
        scripts_dir = skill_dir / "scripts"
        scripts_dir.mkdir(exist_ok=True)

        # Create basic run script
        run_script = scripts_dir / f"run_{agent_name}.py"
        with open(run_script, "w", encoding="utf-8") as f:
            f.write(f'''#!/usr/bin/env python3
"""
{agent_name.title()} Agent - Specialized analysis and recommendations
"""

import os
import sys
from pathlib import Path

def run_{agent_name}_analysis():
    """Run {agent_name} analysis"""
    print("[SEARCH] {agent_name.title()} Agent: Performing specialized analysis...")

    # Implementation goes here
    print("[OK] Analysis completed")

if __name__ == "__main__":
    run_{agent_name}_analysis()
''')

        # Make script executable
        os.chmod(run_script, 0o755)

        print(f"[OK] Migrated {agent_name} to SKILL.md format")
        return True

    except Exception as e:
        print(f"[ERROR] Failed to migrate {agent_name}: {e}")
        return False


def main():
    """Main migration function"""

    agents_dir = Path(".codex/agents")

    if not agents_dir.exists():
        print("[ERROR] .codex/agents directory not found")
        return

    # Get all YAML files
    yaml_files = list(agents_dir.glob("*.yaml"))

    if not yaml_files:
        print("[ERROR] No YAML agent files found")
        return

    print(f"[MIGRATE] Found {len(yaml_files)} agent files to migrate")
    print("[DIR] Target: .codex/skills/ (official SKILL.md format)")

    success_count = 0
    for yaml_file in yaml_files:
        agent_name = yaml_file.stem
        if migrate_agent(agent_name):
            success_count += 1

    print(
        f"\n[OK] Migration completed: {success_count}/{len(yaml_files)} agents migrated"
    )

    if success_count > 0:
        print("\n[INFO] Next steps:")
        print("1. Review and customize the generated SKILL.md files")
        print("2. Implement the actual logic in scripts/run_*.py files")
        print("3. Add references and assets as needed")
        print('4. Test with: codex $agent_name "task description"')


if __name__ == "__main__":
    main()
