#!/usr/bin/env python3
"""
Demo Multi-Agent Workflow
マルチエージェントでのターミナル起動とコンフリクト予防の実演
"""

import os
import sys
import asyncio
import json
import time
from pathlib import Path
from typing import Dict, List, Any

# Add tools directory to path
sys.path.insert(0, str(Path(__file__).parent))

try:
    from conflict_prevention_engine import ConflictPreventionEngine, AgentTerminal
    from multi_agent_terminal_manager import MultiAgentTerminalManager, AgentRole
    from worktree_manager import WorktreeManager
    from premerge_qa_hook import PreMergeQAHook
except ImportError as e:
    print(f"Import error: {e}")
    print("Please ensure all required modules are available")
    sys.exit(1)

class MultiAgentWorkflowDemo:
    """マルチエージェントワークフローの実演"""

    def __init__(self):
        self.repo_path = Path(".")
        self.conflict_engine = ConflictPreventionEngine(str(self.repo_path))
        self.terminal_manager = MultiAgentTerminalManager(str(self.repo_path))

    async def demonstrate_workflow(self):
        """ワークフローの実演"""
        print("🎬 Multi-Agent Workflow Demonstration")
        print("=" * 60)

        # Step 1: Launch agents
        print("1️⃣ Launching specialized agents...")
        await self._launch_agents()

        # Step 2: Create worktree for development
        print("2️⃣ Creating isolated development environment...")
        worktree_info = self._create_development_worktree()

        # Step 3: Analyze potential conflicts
        print("3️⃣ Analyzing potential merge conflicts...")
        conflict_analysis = await self._analyze_conflicts(worktree_info)

        # Step 4: Coordinate agent tasks
        print("4️⃣ Coordinating agent tasks...")
        coordination_result = self._coordinate_agent_tasks(conflict_analysis)

        # Step 5: Simulate merge process
        print("5️⃣ Simulating merge process with QA...")
        merge_result = await self._simulate_merge_process(worktree_info, conflict_analysis)

        # Step 6: Cleanup
        print("6️⃣ Cleaning up resources...")
        self._cleanup_resources()

        print("✅ Multi-agent workflow demonstration completed!")

        return {
            "agents_launched": len(self.terminal_manager.active_agents),
            "conflicts_analyzed": len(conflict_analysis.conflict_predictions) if conflict_analysis else 0,
            "merge_successful": merge_result.get("success", False),
            "coordination_tasks": len(self.terminal_manager.coordination_tasks)
        }

    async def _launch_agents(self):
        """各種エージェントを起動"""
        agents_to_launch = [
            (AgentRole.CODE_REVIEWER, "Review code changes for quality and best practices"),
            (AgentRole.TEST_RUNNER, "Execute automated tests and validate functionality"),
            (AgentRole.SECURITY_AUDITOR, "Audit code for security vulnerabilities"),
            (AgentRole.QA_ENGINEER, "Perform comprehensive quality analysis")
        ]

        launched_agents = []

        for role, description in agents_to_launch:
            agent_id = self.terminal_manager.launch_agent(role, description)
            if agent_id:
                launched_agents.append((role.value, agent_id))
                print(f"  ✅ Launched {role.value}: {agent_id}")
            else:
                print(f"  ❌ Failed to launch {role.value}")

        print(f"  📊 Total agents launched: {len(launched_agents)}")

        # Brief pause for agents to initialize
        await asyncio.sleep(1)

        return launched_agents

    def _create_development_worktree(self):
        """開発用のworktreeを作成"""
        try:
            worktree_manager = WorktreeManager(self.repo_path)

            # Create a feature branch worktree
            worktree_name = f"demo-feature-{int(time.time())}"

            success = worktree_manager.create_worktree(
                name=worktree_name,
                branch_name=f"feature/demo-{int(time.time())}",
                worktree_type="feature"
            )

            if success:
                print(f"  ✅ Created worktree: {worktree_name}")

                # Get worktree info
                worktree_info = worktree_manager.get_worktree_info(worktree_name)
                if worktree_info:
                    return worktree_info

            print(f"  ❌ Failed to create worktree: {worktree_name}")
            return None

        except Exception as e:
            print(f"  ❌ Worktree creation failed: {e}")
            return None

    async def _analyze_conflicts(self, worktree_info):
        """コンフリクト分析を実行"""
        if not worktree_info:
            print("  ⚠️ No worktree available for conflict analysis")
            return None

        try:
            # Analyze conflicts between worktree branch and main
            analysis = await self.conflict_engine.analyze_merge_conflicts(
                worktree_info.branch,
                "main"
            )

            print("  📊 Conflict Analysis Results:")
            print(f"    - Files changed: {len(analysis.changed_files)}")
            print(f"    - Predicted conflicts: {len(analysis.conflict_predictions)}")
            print(f"    - Risk level: {analysis.risk_assessment.get('overall_risk', 'unknown')}")

            if analysis.conflict_predictions:
                print("    - Top conflicts:")
                for i, pred in enumerate(analysis.conflict_predictions[:3]):
                    print(f"      {i+1}. {pred.file_path}: {pred.conflict_type.value} ({pred.risk_level.value})")

            return analysis

        except Exception as e:
            print(f"  ❌ Conflict analysis failed: {e}")
            return None

    def _coordinate_agent_tasks(self, conflict_analysis):
        """エージェントタスクを協調"""
        try:
            # Get active agents
            active_agent_ids = list(self.terminal_manager.active_agents.keys())

            if not active_agent_ids:
                print("  ⚠️ No active agents available")
                return {"tasks_created": 0}

            # Create coordination task
            task_description = f"""
Collaborative code review and testing for merge preparation.

Analysis Summary:
- Files changed: {len(conflict_analysis.changed_files) if conflict_analysis else 0}
- Predicted conflicts: {len(conflict_analysis.conflict_predictions) if conflict_analysis else 0}
- Risk level: {conflict_analysis.risk_assessment.get('overall_risk', 'unknown') if conflict_analysis else 'unknown'}

Agents should:
1. Review code changes for quality and best practices
2. Execute relevant tests
3. Identify and report any issues
4. Provide recommendations for merge preparation
"""

            task_id = self.terminal_manager.create_coordination_task(
                task_description,
                active_agent_ids,
                []  # No dependencies for this demo
            )

            print(f"  ✅ Created coordination task: {task_id}")
            print(f"  📋 Assigned agents: {len(active_agent_ids)}")

            return {
                "task_id": task_id,
                "agents_assigned": len(active_agent_ids),
                "task_description": task_description
            }

        except Exception as e:
            print(f"  ❌ Task coordination failed: {e}")
            return {"error": str(e)}

    async def _simulate_merge_process(self, worktree_info, conflict_analysis):
        """マージプロセスをシミュレート"""
        try:
            if not worktree_info:
                return {"success": False, "reason": "No worktree available"}

            # Initialize QA hook
            qa_hook = PreMergeQAHook()

            # Run pre-merge QA
            print("  🔍 Running pre-merge QA analysis...")
            merge_allowed, results = qa_hook.run_pre_merge_qa(
                worktree_info.branch,
                "main"
            )

            print(f"  📋 QA Result: {'ALLOWED' if merge_allowed else 'BLOCKED'}")

            if results.get("conflict_analysis"):
                analysis = results["conflict_analysis"]
                print(f"  ⚠️ Conflict predictions: {len(analysis.conflict_predictions)}")

            # Simulate merge if allowed
            if merge_allowed:
                print("  🔀 Simulating merge process...")

                # In a real scenario, this would perform the actual merge
                # For demo purposes, we just report success
                success = True
                reason = "QA checks passed, merge simulation successful"
            else:
                success = False
                reason = "QA checks failed, merge blocked"
                if results.get("evaluation", {}).get("block_reasons"):
                    reason += f": {results['evaluation']['block_reasons'][0]}"

            return {
                "success": success,
                "reason": reason,
                "qa_results": results
            }

        except Exception as e:
            print(f"  ❌ Merge simulation failed: {e}")
            return {"success": False, "reason": str(e)}

    def _cleanup_resources(self):
        """リソースをクリーンアップ"""
        try:
            print("  🧹 Cleaning up agent terminals...")
            self.terminal_manager.shutdown()

            print("  🧹 Cleaning up conflict engine resources...")
            self.conflict_engine.cleanup_terminals()

            print("  ✅ Cleanup completed")

        except Exception as e:
            print(f"  ⚠️ Cleanup warning: {e}")

    def get_workflow_summary(self):
        """ワークフロー実行結果のサマリー"""
        return {
            "terminal_manager_status": self.terminal_manager.get_system_status(),
            "conflict_engine_status": "active" if self.conflict_engine else "unavailable",
            "workflow_completed": True
        }

async def main():
    """メイン実行関数"""
    print("🚀 Starting Multi-Agent Workflow Demo")
    print("=" * 60)

    demo = MultiAgentWorkflowDemo()

    try:
        # Run the demonstration
        results = await demo.demonstrate_workflow()

        print("\n📊 Workflow Results Summary:")
        print(json.dumps(results, indent=2))

        print("\n🎉 Multi-Agent Workflow Demo completed successfully!")

    except KeyboardInterrupt:
        print("\n⚠️ Demo interrupted by user")
    except Exception as e:
        print(f"\n❌ Demo failed with error: {e}")
        import traceback
        traceback.print_exc()
    finally:
        demo._cleanup_resources()

if __name__ == "__main__":
    asyncio.run(main())