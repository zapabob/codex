#!/usr/bin/env python3
"""
Git4D Runtime Skill Runner
Executes Git4D runtime environment validation tasks.
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from codex_cli import CodexCLI


def main():
    """Run Git4D runtime check task."""
    cli = CodexCLI()

    task = (
        " ".join(sys.argv[1:])
        if len(sys.argv) > 1
        else "Check runtime environment for GPU development"
    )

    result = cli.run_agent(
        agent_name="git4d-runtime-agent",
        task=task,
        context={
            "skill": "git4d-runtime",
            "technology": "Git4D",
            "features": ["CUDA Detection", "SSE Detection", "GPU Verification"],
        },
    )

    print(result)
    return 0 if result.success else 1


if __name__ == "__main__":
    sys.exit(main())
