#!/usr/bin/env python3
"""
Test script for Codex Agents SDK Orchestrator
"""

import asyncio
import sys
from pathlib import Path
from supervisor import CodexOrchestrator

async def test_basic_workflow():
    """Test basic workflow execution without MCP server"""

    print("[TEST] Testing basic workflow execution...")

    # Create orchestrator (will fallback to direct execution)
    orchestrator = CodexOrchestrator()

    # Test task decomposition
    tasks = orchestrator.decompose_task("Create a simple user registration system")

    print(f"[INFO] Task decomposed into {len(tasks)} subtasks:")
    for task in tasks:
        deps = ", ".join(task.dependencies) if task.dependencies else "none"
        print(f"  - {task.id}: {task.description[:50]}... (deps: {deps})")

    # Test architect skill execution (without MCP)
    print("\n[TEST] Testing architect skill execution...")

    # Manually execute architect skill
    architect_task = next(task for task in tasks if task.assigned_agent.value == "architect")

    try:
        result = await orchestrator.execute_skill(architect_task)

        print(f"[RESULT] Success: {result.success}")
        print(f"[RESULT] Duration: {result.duration:.2f}s")
        print(f"[RESULT] Output preview: {result.output[:100]}...")

        # Check if artifacts were created
        artifacts_dir = Path("artifacts")
        if artifacts_dir.exists():
            artifacts = list(artifacts_dir.glob("*"))
            print(f"[ARTIFACTS] Created {len(artifacts)} files:")
            for artifact in artifacts:
                print(f"  - {artifact.name}")

    except Exception as e:
        print(f"[ERROR] Architect skill execution failed: {e}")

async def test_mcp_bridge():
    """Test MCP bridge connection (if server available)"""

    print("\n[TEST] Testing MCP bridge connection...")

    try:
        from mcp_bridge import create_mcp_bridge

        bridge = await create_mcp_bridge()

        if bridge:
            print("[OK] MCP bridge connected successfully")

            # Test tool listing
            tools = await bridge.list_tools()
            print(f"[INFO] Available MCP tools: {len(tools)}")

            if tools:
                print("Sample tools:")
                for tool in tools[:3]:
                    print(f"  - {tool.get('name', 'unknown')}: {tool.get('description', '')[:50]}...")

            # Test resource listing
            resources = await bridge.list_resources()
            print(f"[INFO] Available MCP resources: {len(resources)}")

            await bridge.disconnect()
            print("[OK] MCP bridge test completed")

        else:
            print("[WARN] MCP server not available - bridge test skipped")

    except ImportError:
        print("[WARN] MCP bridge dependencies not available - test skipped")
    except Exception as e:
        print(f"[ERROR] MCP bridge test failed: {e}")

async def test_skill_execution():
    """Test individual skill execution"""

    print("\n[TEST] Testing individual skill execution...")

    # Test skills that should exist
    test_skills = ["architect", "code-reviewer", "executor"]

    for skill_name in test_skills:
        skill_dir = Path(".codex/skills") / skill_name
        script_path = skill_dir / "scripts" / f"run_{skill_name}.py"

        if script_path.exists():
            print(f"[INFO] Testing {skill_name} skill...")

            try:
                process = await asyncio.create_subprocess_exec(
                    sys.executable, str(script_path),
                    stdout=asyncio.subprocess.PIPE,
                    stderr=asyncio.subprocess.PIPE,
                    cwd=Path.cwd()
                )

                stdout, stderr = await process.communicate()
                success = process.returncode == 0

                output = stdout.decode('utf-8', errors='replace') if success else stderr.decode('utf-8', errors='replace')

                print(f"  [RESULT] Success: {success}")
                print(f"  [RESULT] Output: {output[:100]}...")

            except Exception as e:
                print(f"  [ERROR] Failed to execute {skill_name}: {e}")
        else:
            print(f"[WARN] {skill_name} skill script not found: {script_path}")

def check_prerequisites():
    """Check if prerequisites are met"""

    print("[CHECK] Checking prerequisites...")

    # Check Python version
    python_version = sys.version_info
    print(f"[INFO] Python version: {python_version.major}.{python_version.minor}.{python_version.micro}")

    # Check skills directory
    skills_dir = Path(".codex/skills")
    if skills_dir.exists():
        skill_count = len(list(skills_dir.glob("*/SKILL.md")))
        print(f"[INFO] Skills directory: {skill_count} skills found")

        # List available skills
        skills = [d.name for d in skills_dir.glob("*") if d.is_dir()]
        print(f"[INFO] Available skills: {', '.join(skills)}")
    else:
        print("[WARN] Skills directory not found: .codex/skills/")

    # Check orchestrator files
    required_files = [
        "supervisor.py",
        "mcp_bridge.py",
        "README.md"
    ]

    missing_files = []
    for file in required_files:
        if not Path(file).exists():
            missing_files.append(file)

    if missing_files:
        print(f"[ERROR] Missing required files: {', '.join(missing_files)}")
        return False
    else:
        print("[OK] All required files present")
        return True

async def main():
    """Main test function"""

    print("[TEST] Codex Agents SDK Orchestrator - Test Suite")
    print("=" * 60)

    # Check prerequisites
    if not check_prerequisites():
        print("[ERROR] Prerequisites not met - exiting")
        sys.exit(1)

    # Run tests
    await test_basic_workflow()
    await test_mcp_bridge()
    await test_skill_execution()

    print("\n" + "=" * 60)
    print("[OK] Test suite completed!")
    print("\nNext steps:")
    print("1. Start MCP server: codex mcp-server --port 3000")
    print("2. Run full workflow: python supervisor.py \"your task\"")
    print("3. Check results: cat artifacts/workflow_report.json")

if __name__ == "__main__":
    asyncio.run(main())