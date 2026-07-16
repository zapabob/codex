#!/usr/bin/env python3
"""
VRChat Dev Skill Runner
Executes VRChat development tasks using the vrchat-dev agent.
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from codex_cli import CodexCLI


def main():
    """Run VRChat development task."""
    cli = CodexCLI()

    task = (
        " ".join(sys.argv[1:]) if len(sys.argv) > 1 else "Help with VRChat development"
    )

    result = cli.run_agent(
        agent_name="vrchat-dev-agent",
        task=task,
        context={
            "skill": "vrchat-dev",
            "technology": "VRChat SDK3",
            "features": ["UdonSharp", "modularavatar", "PhysBones"],
        },
    )

    print(result)
    return 0 if result.success else 1


if __name__ == "__main__":
    sys.exit(main())
