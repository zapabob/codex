#!/usr/bin/env python3
"""
Blender CAD Skill Runner
Executes Blender CAD modeling tasks using the blender-cad agent.
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from codex_cli import CodexCLI


def main():
    """Run Blender CAD task."""
    cli = CodexCLI()

    task = (
        " ".join(sys.argv[1:])
        if len(sys.argv) > 1
        else "Help with Blender CAD modeling"
    )

    result = cli.run_agent(
        agent_name="blender-cad-agent",
        task=task,
        context={
            "skill": "blender-cad",
            "technology": "Blender Python",
            "features": ["STEP/IGES Import", "Geometry Nodes", "USD Export"],
        },
    )

    print(result)
    return 0 if result.success else 1


if __name__ == "__main__":
    sys.exit(main())
