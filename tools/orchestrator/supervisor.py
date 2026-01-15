#!/usr/bin/env python3
"""
Codex Agents SDK Orchestrator - Supervisor for Multi-Agent Workflows
"""

import os
import sys
import json
import asyncio
import subprocess
from pathlib import Path
from typing import Dict, List, Any, Optional
from dataclasses import dataclass
from enum import Enum
import time

class TaskStatus(Enum):
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"

class AgentRole(Enum):
    ARCHITECT = "architect"
    REVIEWER = "code-reviewer"
    EXECUTOR = "executor"
    RESEARCHER = "researcher"
    TESTER = "test-gen"

@dataclass
class Task:
    id: str
    description: str
    assigned_agent: AgentRole
    status: TaskStatus = TaskStatus.PENDING
    dependencies: List[str] = None
    result: Optional[str] = None
    error: Optional[str] = None

    def __post_init__(self):
        if self.dependencies is None:
            self.dependencies = []

@dataclass
class WorkflowResult:
    task_id: str
    agent: str
    output: str
    duration: float
    success: bool

class CodexOrchestrator:
    """Supervisor orchestrator for multi-agent workflows"""

    def __init__(self, codex_mcp_url: str = "http://localhost:3000"):
        self.codex_mcp_url = codex_mcp_url
        self.tasks: Dict[str, Task] = {}
        self.results: List[WorkflowResult] = []
        self.skills_dir = Path(".codex/skills")

    def decompose_task(self, user_task: str) -> List[Task]:
        """Decompose user task into subtasks for different agents"""

        tasks = []

        # Always start with architect for system analysis
        tasks.append(Task(
            id="architect_analysis",
            description=f"Analyze the system architecture and requirements for: {user_task}",
            assigned_agent=AgentRole.ARCHITECT
        ))

        # Add code review task
        tasks.append(Task(
            id="code_review",
            description=f"Review existing code for: {user_task}",
            assigned_agent=AgentRole.REVIEWER,
            dependencies=["architect_analysis"]
        ))

        # Add research task if needed
        if any(keyword in user_task.lower() for keyword in ["research", "investigate", "explore", "analyze"]):
            tasks.append(Task(
                id="research",
                description=f"Research and gather information for: {user_task}",
                assigned_agent=AgentRole.RESEARCHER,
                dependencies=["architect_analysis"]
            ))

        # Add testing task
        tasks.append(Task(
            id="testing",
            description=f"Generate and run tests for: {user_task}",
            assigned_agent=AgentRole.TESTER,
            dependencies=["code_review"]
        ))

        # Add execution task
        tasks.append(Task(
            id="execution",
            description=f"Execute the implementation for: {user_task}",
            assigned_agent=AgentRole.EXECUTOR,
            dependencies=["code_review", "testing"]
        ))

        return tasks

    def check_dependencies(self, task: Task) -> bool:
        """Check if task dependencies are satisfied"""
        for dep_id in task.dependencies:
            if dep_id not in self.tasks:
                return False
            if self.tasks[dep_id].status != TaskStatus.COMPLETED:
                return False
        return True

    async def execute_skill(self, task: Task) -> WorkflowResult:
        """Execute a skill for the given task"""

        start_time = time.time()
        success = False
        output = ""

        try:
            # Find the skill script
            skill_dir = self.skills_dir / task.assigned_agent.value
            script_path = skill_dir / "scripts" / f"run_{task.assigned_agent.value}.py"

            if not script_path.exists():
                raise FileNotFoundError(f"Skill script not found: {script_path}")

            # Execute the skill script
            print(f"[EXEC] Running {task.assigned_agent.value} for task: {task.id}")

            process = await asyncio.create_subprocess_exec(
                sys.executable, str(script_path),
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                cwd=Path.cwd()
            )

            stdout, stderr = await process.communicate()

            if process.returncode == 0:
                output = stdout.decode('utf-8', errors='replace')
                success = True
                print(f"[OK] {task.assigned_agent.value} completed successfully")
            else:
                error_output = stderr.decode('utf-8', errors='replace')
                output = f"Error: {error_output}"
                print(f"[ERROR] {task.assigned_agent.value} failed: {error_output}")

        except Exception as e:
            output = f"Exception: {str(e)}"
            print(f"[ERROR] Failed to execute {task.assigned_agent.value}: {e}")

        duration = time.time() - start_time

        return WorkflowResult(
            task_id=task.id,
            agent=task.assigned_agent.value,
            output=output,
            duration=duration,
            success=success
        )

    async def orchestrate_workflow(self, user_task: str) -> Dict[str, Any]:
        """Main orchestration logic"""

        print(f"[START] Starting workflow orchestration for: {user_task}")
        print("=" * 60)

        # Decompose task into subtasks
        tasks = self.decompose_task(user_task)
        for task in tasks:
            self.tasks[task.id] = task

        print(f"[INFO] Decomposed into {len(tasks)} subtasks")

        # Execute tasks in dependency order
        completed_tasks = set()

        while len(completed_tasks) < len(tasks):
            # Find tasks ready to execute
            ready_tasks = [
                task for task in tasks
                if task.id not in completed_tasks and self.check_dependencies(task)
            ]

            if not ready_tasks:
                print("[ERROR] No tasks ready to execute - possible circular dependency")
                break

            # Execute ready tasks concurrently
            print(f"[EXEC] Executing {len(ready_tasks)} tasks concurrently")

            execution_tasks = [
                self.execute_skill(task) for task in ready_tasks
            ]

            # Wait for all tasks to complete
            results = await asyncio.gather(*execution_tasks, return_exceptions=True)

            # Process results
            for i, result in enumerate(results):
                task = ready_tasks[i]

                if isinstance(result, Exception):
                    print(f"[ERROR] Task {task.id} failed with exception: {result}")
                    task.status = TaskStatus.FAILED
                    task.error = str(result)
                else:
                    task.status = TaskStatus.COMPLETED if result.success else TaskStatus.FAILED
                    task.result = result.output
                    self.results.append(result)

                completed_tasks.add(task.id)

                # Log result
                status_icon = "[OK]" if task.status == TaskStatus.COMPLETED else "[ERROR]"
                print(f"{status_icon} Task {task.id} completed in {result.duration:.1f}s")

        # Generate final report
        return self.generate_workflow_report(user_task)

    def generate_workflow_report(self, original_task: str) -> Dict[str, Any]:
        """Generate comprehensive workflow report"""

        total_duration = sum(result.duration for result in self.results)
        successful_tasks = sum(1 for result in self.results if result.success)
        total_tasks = len(self.results)

        report = {
            "original_task": original_task,
            "execution_summary": {
                "total_tasks": total_tasks,
                "successful_tasks": successful_tasks,
                "failed_tasks": total_tasks - successful_tasks,
                "total_duration": total_duration,
                "success_rate": successful_tasks / total_tasks if total_tasks > 0 else 0
            },
            "task_results": [
                {
                    "task_id": result.task_id,
                    "agent": result.agent,
                    "duration": result.duration,
                    "success": result.success,
                    "output_preview": result.output[:200] + "..." if len(result.output) > 200 else result.output
                }
                for result in self.results
            ],
            "artifacts": [
                str(path) for path in Path("artifacts").glob("*")
                if path.is_file()
            ] if Path("artifacts").exists() else []
        }

        return report

async def main():
    """Main entry point"""

    if len(sys.argv) < 2:
        print("Usage: python supervisor.py \"task description\"")
        sys.exit(1)

    user_task = sys.argv[1]

    # Initialize orchestrator
    orchestrator = CodexOrchestrator()

    # Execute workflow
    try:
        report = await orchestrator.orchestrate_workflow(user_task)

        # Save report
        report_path = Path("artifacts/workflow_report.json")
        report_path.parent.mkdir(exist_ok=True)

        with open(report_path, 'w', encoding='utf-8') as f:
            json.dump(report, f, indent=2, ensure_ascii=False)

        print("\n" + "=" * 60)
        print("[SUCCESS] Workflow completed!")
        print(f"[SAVE] Report saved to: {report_path}")
        print(".1f")
        print(f"[RATE] Success rate: {report['execution_summary']['success_rate']:.1%}")

        # Print artifacts
        if report['artifacts']:
            print("[ARTIFACTS] Generated files:")
            for artifact in report['artifacts']:
                print(f"  - {artifact}")

    except KeyboardInterrupt:
        print("\n[STOP] Workflow interrupted by user")
    except Exception as e:
        print(f"\n[ERROR] Workflow failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    asyncio.run(main())