#!/usr/bin/env python3
"""
Yukkuri MovieMaker Skill Runner
Executes ゆっくりMovieMaker video production tasks.
"""

import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from codex_cli import CodexCLI


def main():
    """Run Yukkuri MovieMaker task."""
    cli = CodexCLI()

    task = (
        " ".join(sys.argv[1:])
        if len(sys.argv) > 1
        else "Help with ゆっくりMovieMaker video production"
    )

    result = cli.run_agent(
        agent_name="yukkuri-movie-agent",
        task=task,
        context={
            "skill": "yukkuri-movie",
            "technology": "YMM4",
            "features": ["Character Animation", "Scene Management", "MIDI Integration"],
        },
    )

    print(result)
    return 0 if result.success else 1


if __name__ == "__main__":
    sys.exit(main())
