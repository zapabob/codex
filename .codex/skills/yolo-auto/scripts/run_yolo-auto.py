#!/usr/bin/env python3
"""
YOLO Auto Skill Runner
Executes YOLO full-stack automation workflows.
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from codex_cli import CodexCLI


def main():
    """Run YOLO automation task."""
    cli = CodexCLI()

    task = (
        " ".join(sys.argv[1:])
        if len(sys.argv) > 1
        else "Help with YOLO full-stack automation"
    )

    result = cli.run_agent(
        agent_name="yolo-auto-agent",
        task=task,
        context={
            "skill": "yolo-auto",
            "technology": "OpenClaw",
            "features": [
                "GPU Model Selection",
                "Multi-Agent Orchestration",
                "Workflow Automation",
            ],
        },
    )

    print(result)
    return 0 if result.success else 1


if __name__ == "__main__":
    sys.exit(main())
