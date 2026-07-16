#!/usr/bin/env python3
"""
Git4D Schema Skill Runner
Executes Git4D schema validation tasks.
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from codex_cli import CodexCLI


def main():
    """Run Git4D schema audit task."""
    cli = CodexCLI()

    task = (
        " ".join(sys.argv[1:])
        if len(sys.argv) > 1
        else "Validate Git4D schema definitions"
    )

    result = cli.run_agent(
        agent_name="git4d-schema-agent",
        task=task,
        context={
            "skill": "git4d-schema",
            "technology": "Git4D",
            "features": [
                "Schema Validation",
                "Configuration Audit",
                "Pipeline Integrity",
            ],
        },
    )

    print(result)
    return 0 if result.success else 1


if __name__ == "__main__":
    sys.exit(main())
