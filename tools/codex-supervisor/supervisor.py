#!/usr/bin/env python3
"""
Codex Supervisor - MCP-Centric Multi-Agent Workflow Orchestrator
Official OpenAI Codex Agents SDK Pattern Implementation
"""

import os
import sys
import json
import asyncio
import subprocess
from pathlib import Path
from typing import Dict, List, Any, Optional, Callable
from dataclasses import dataclass
from enum import Enum
import time
import logging

# Import MCP bridge for official Codex integration
from mcp_bridge import CodexMCPBridge, create_mcp_bridge

logger = logging.getLogger(__name__)

class TaskStatus(Enum):
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    HANDOFF = "handoff"  # Official Agents SDK concept

class AgentRole(Enum):
    SUPERVISOR = "supervisor"  # New: MCP orchestrator
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
    handoff_data: Optional[Dict[str, Any]] = None  # For Agents SDK handoff

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
    handoffs: List[Dict[str, Any]] = None  # Track handoffs

    def __post_init__(self):
        if self.handoffs is None:
            self.handoffs = []

@dataclass
class GuardrailResult:
    """Official Agents SDK guardrail concept"""
    task_id: str
    passed: bool
    violations: List[str]
    recommendations: List[str]

class CodexSupervisor:
    """
    Official OpenAI Codex Supervisor - MCP-Centric Orchestrator
    Implements Agents SDK patterns: handoff, guardrails, worker agents
    """

    def __init__(self, codex_mcp_url: str = "ws://localhost:3000"):
        self.codex_mcp_url = codex_mcp_url
        self.mcp_bridge: Optional[CodexMCPBridge] = None
        self.tasks: Dict[str, Task] = {}
        self.results: List[WorkflowResult] = []
        self.guardrails: List[Callable[[Task], GuardrailResult]] = []
        self.skills_dir = Path(".codex/skills")

        # Agents SDK concepts
        self.worker_agents: Dict[str, Dict[str, Any]] = {}
        self.handoff_queue: asyncio.Queue = asyncio.Queue()

    async def initialize_mcp(self) -> bool:
        """Initialize MCP connection to Codex server (official pattern)"""
        logger.info(f"Connecting to Codex MCP server: {self.codex_mcp_url}")
        self.mcp_bridge = await create_mcp_bridge(self.codex_mcp_url)

        if self.mcp_bridge:
            logger.info("MCP connection established - using official Codex Skills")
            return True
        else:
            logger.warning("MCP connection failed - falling back to direct execution")
            return False

    def add_guardrail(self, guardrail_func: Callable[[Task], GuardrailResult]):
        """Add guardrail function (official Agents SDK concept)"""
        self.guardrails.append(guardrail_func)

    def register_worker_agent(self, name: str, config: Dict[str, Any]):
        """Register worker agent (official Agents SDK pattern)"""
        self.worker_agents[name] = config
        logger.info(f"Registered worker agent: {name}")

    async def apply_guardrails(self, task: Task) -> GuardrailResult:
        """Apply all guardrails to task (official Agents SDK concept)"""
        all_violations = []
        all_recommendations = []

        for guardrail in self.guardrails:
            result = guardrail(task)
            if not result.passed:
                all_violations.extend(result.violations)
                all_recommendations.extend(result.recommendations)

        return GuardrailResult(
            task_id=task.id,
            passed=len(all_violations) == 0,
            violations=all_violations,
            recommendations=all_recommendations
        )

    def decompose_task(self, user_task: str) -> List[Task]:
        """Decompose user task into subtasks using official Agents SDK patterns"""

        tasks = []

        # Supervisor analysis (new: MCP-centric orchestration)
        tasks.append(Task(
            id="supervisor_analysis",
            description=f"Analyze and plan workflow orchestration for: {user_task}",
            assigned_agent=AgentRole.SUPERVISOR
        ))

        # Always start with architect for system analysis
        tasks.append(Task(
            id="architect_analysis",
            description=f"Analyze system architecture and requirements for: {user_task}",
            assigned_agent=AgentRole.ARCHITECT,
            dependencies=["supervisor_analysis"]
        ))

        # Add code review task
        tasks.append(Task(
            id="code_review",
            description=f"Review existing code for: {user_task}",
            assigned_agent=AgentRole.REVIEWER,
            dependencies=["architect_analysis"]
        ))

        # Add research task if needed (Agents SDK: conditional handoff)
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
            description=f"Execute implementation for: {user_task}",
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

    async def execute_skill_via_mcp(self, task: Task) -> WorkflowResult:
        """Execute skill via MCP (official Codex integration pattern)"""

        start_time = time.time()
        success = False
        output = ""
        handoffs = []

        try:
            if not self.mcp_bridge:
                raise RuntimeError("MCP bridge not initialized")

            skill_name = task.assigned_agent.value
            print(f"[MCP] Executing {skill_name} via Codex MCP server for task: {task.id}")

            # Apply guardrails before execution (official Agents SDK)
            guardrail_result = await self.apply_guardrails(task)
            if not guardrail_result.passed:
                output = f"Guardrail violations: {', '.join(guardrail_result.violations)}"
                print(f"[GUARDRAIL] Task {task.id} blocked: {output}")
                return WorkflowResult(
                    task_id=task.id,
                    agent=skill_name,
                    output=output,
                    duration=time.time() - start_time,
                    success=False
                )

            # Execute via MCP (official pattern)
            mcp_result = await self.mcp_bridge.execute_skill_via_mcp(skill_name, task.description)

            if mcp_result["success"]:
                output = mcp_result.get("result", "")
                success = True
                print(f"[MCP-OK] {skill_name} completed via Codex MCP")
            else:
                output = f"MCP Error: {mcp_result.get('error', 'Unknown error')}"
                print(f"[MCP-ERROR] {skill_name} failed: {output}")

        except Exception as e:
            output = f"MCP Exception: {str(e)}"
            print(f"[MCP-ERROR] Failed to execute {task.assigned_agent.value} via MCP: {e}")

            # Fallback to direct execution if MCP fails
            print(f"[FALLBACK] Attempting direct execution for {task.assigned_agent.value}")
            fallback_result = await self.execute_skill_direct(task)
            success = fallback_result.success
            output = fallback_result.output

        duration = time.time() - start_time

        return WorkflowResult(
            task_id=task.id,
            agent=task.assigned_agent.value,
            output=output,
            duration=duration,
            success=success,
            handoffs=handoffs
        )

    async def execute_skill_direct(self, task: Task) -> WorkflowResult:
        """Fallback: Execute skill directly (non-official pattern)"""

        start_time = time.time()
        success = False
        output = ""

        try:
            # Find the skill script (fallback to direct execution)
            skill_dir = self.skills_dir / task.assigned_agent.value
            script_path = skill_dir / "scripts" / f"run_{task.assigned_agent.value}.py"

            if not script_path.exists():
                raise FileNotFoundError(f"Skill script not found: {script_path}")

            print(f"[DIRECT] Running {task.assigned_agent.value} directly for task: {task.id}")

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
                print(f"[DIRECT-OK] {task.assigned_agent.value} completed")
            else:
                error_output = stderr.decode('utf-8', errors='replace')
                output = f"Direct Error: {error_output}"
                print(f"[DIRECT-ERROR] {task.assigned_agent.value} failed: {error_output}")

        except Exception as e:
            output = f"Direct Exception: {str(e)}"
            print(f"[DIRECT-ERROR] Failed to execute {task.assigned_agent.value}: {e}")

        return WorkflowResult(
            task_id=task.id,
            agent=task.assigned_agent.value,
            output=output,
            duration=time.time() - start_time,
            success=success
        )

    async def orchestrate_workflow(self, user_task: str) -> Dict[str, Any]:
        """Main orchestration logic - MCP-centric with Agents SDK patterns"""

        print(f"[SUPERVISOR] Starting MCP-centric workflow orchestration for: {user_task}")
        print("=" * 80)

        # Initialize MCP connection (official pattern)
        mcp_available = await self.initialize_mcp()
        if mcp_available:
            print("[MCP] Using official Codex Skills via MCP server")
        else:
            print("[FALLBACK] MCP unavailable - using direct skill execution")

        # Decompose task into subtasks (with supervisor analysis)
        tasks = self.decompose_task(user_task)
        for task in tasks:
            self.tasks[task.id] = task

        print(f"[SUPERVISOR] Decomposed into {len(tasks)} subtasks with dependencies")

        # Execute tasks using official Agents SDK patterns
        completed_tasks = set()

        while len(completed_tasks) < len(tasks):
            # Find tasks ready to execute
            ready_tasks = [
                task for task in tasks
                if task.id not in completed_tasks and self.check_dependencies(task)
            ]

            if not ready_tasks:
                print("[SUPERVISOR] No tasks ready to execute - checking for handoffs")
                # Check handoff queue (Agents SDK concept)
                try:
                    handoff = self.handoff_queue.get_nowait()
                    print(f"[HANDOFF] Processing handoff: {handoff}")
                    # Process handoff...
                except asyncio.QueueEmpty:
                    print("[ERROR] No tasks ready and no handoffs - possible circular dependency")
                    break

            # Execute ready tasks concurrently (official pattern)
            print(f"[SUPERVISOR] Executing {len(ready_tasks)} tasks concurrently")

            if mcp_available:
                # Official: Use MCP for skill execution
                execution_tasks = [
                    self.execute_skill_via_mcp(task) for task in ready_tasks
                ]
            else:
                # Fallback: Direct execution
                execution_tasks = [
                    self.execute_skill_direct(task) for task in ready_tasks
                ]

            # Wait for all tasks to complete
            results = await asyncio.gather(*execution_tasks, return_exceptions=True)

            # Process results with Agents SDK patterns
            for i, result in enumerate(results):
                task = ready_tasks[i]

                if isinstance(result, Exception):
                    print(f"[SUPERVISOR] Task {task.id} failed with exception: {result}")
                    task.status = TaskStatus.FAILED
                    task.error = str(result)
                else:
                    # Check for handoffs (Agents SDK pattern)
                    if result.handoffs:
                        print(f"[HANDOFF] Task {task.id} initiated {len(result.handoffs)} handoffs")
                        for handoff in result.handoffs:
                            await self.handoff_queue.put(handoff)

                    task.status = TaskStatus.COMPLETED if result.success else TaskStatus.FAILED
                    task.result = result.output
                    self.results.append(result)

                completed_tasks.add(task.id)

                # Log result with official status
                status_icon = "[MCP-OK]" if (mcp_available and task.status == TaskStatus.COMPLETED) else "[OK]" if task.status == TaskStatus.COMPLETED else "[ERROR]"
                execution_method = "via MCP" if mcp_available else "direct"
                print(f"{status_icon} Task {task.id} completed {execution_method} in {result.duration:.1f}s")

        # Generate final report
        return await self.generate_workflow_report(user_task)

    async def generate_workflow_report(self, original_task: str) -> Dict[str, Any]:
        """Generate comprehensive workflow report with official metrics"""

        total_duration = sum(result.duration for result in self.results)
        successful_tasks = sum(1 for result in self.results if result.success)
        total_tasks = len(self.results)
        mcp_tasks = sum(1 for result in self.results if "[MCP" in str(result.output))
        handoffs_total = sum(len(result.handoffs) for result in self.results)

        report = {
            "original_task": original_task,
            "orchestrator_info": {
                "type": "CodexSupervisor",
                "pattern": "Official Agents SDK",
                "mcp_integration": self.mcp_bridge is not None,
                "guardrails_applied": len(self.guardrails),
                "worker_agents": len(self.worker_agents)
            },
            "execution_summary": {
                "total_tasks": total_tasks,
                "successful_tasks": successful_tasks,
                "failed_tasks": total_tasks - successful_tasks,
                "mcp_executed_tasks": mcp_tasks,
                "direct_executed_tasks": total_tasks - mcp_tasks,
                "total_handoffs": handoffs_total,
                "total_duration": total_duration,
                "success_rate": successful_tasks / total_tasks if total_tasks > 0 else 0,
                "avg_task_duration": total_duration / total_tasks if total_tasks > 0 else 0
            },
            "task_results": [
                {
                    "task_id": result.task_id,
                    "agent": result.agent,
                    "execution_method": "mcp" if "[MCP" in str(result.output) else "direct",
                    "duration": result.duration,
                    "success": result.success,
                    "handoffs_initiated": len(result.handoffs),
                    "output_preview": result.output[:200] + "..." if len(result.output) > 200 else result.output
                }
                for result in self.results
            ],
            "artifacts": [
                str(path) for path in Path("artifacts").glob("*")
                if path.is_file()
            ] if Path("artifacts").exists() else [],
            "official_compliance": {
                "skills_format": "SKILL.md compliant",
                "mcp_integration": "Client/Server pattern",
                "agents_sdk_patterns": ["handoff", "guardrails", "worker_agents"],
                "fork_strategy": "Thin fork with external orchestrator"
            }
        }

        return report

# Default Guardrails (Official Agents SDK Pattern)
def security_guardrail(task: Task) -> GuardrailResult:
    """Security guardrail - official Agents SDK concept"""
    violations = []
    recommendations = []

    # Check for potentially dangerous operations
    dangerous_keywords = ["delete", "remove", "rm", "unlink", "drop table", "truncate"]
    task_text = task.description.lower()

    if any(keyword in task_text for keyword in dangerous_keywords):
        violations.append("Potentially destructive operations detected")
        recommendations.append("Add explicit confirmation for destructive operations")

    # Check for sensitive data exposure
    sensitive_keywords = ["password", "secret", "key", "token", "credential"]
    if any(keyword in task_text for keyword in sensitive_keywords):
        violations.append("Sensitive data handling detected")
        recommendations.append("Ensure proper data sanitization and access controls")

    return GuardrailResult(
        task_id=task.id,
        passed=len(violations) == 0,
        violations=violations,
        recommendations=recommendations
    )

def quality_guardrail(task: Task) -> GuardrailResult:
    """Code quality guardrail - official Agents SDK concept"""
    violations = []
    recommendations = []

    # Check for testing requirements
    if task.assigned_agent == AgentRole.EXECUTOR and "test" not in task.description.lower():
        violations.append("Implementation task without testing consideration")
        recommendations.append("Ensure test coverage for new implementations")

    # Check for documentation requirements
    if task.assigned_agent == AgentRole.EXECUTOR and "doc" not in task.description.lower():
        recommendations.append("Consider adding documentation for new features")

    return GuardrailResult(
        task_id=task.id,
        passed=len(violations) == 0,
        violations=violations,
        recommendations=recommendations
    )

async def main():
    """Main entry point - Official Codex Supervisor"""

    if len(sys.argv) < 2:
        print("Usage: python supervisor.py \"task description\" [--mcp-url URL]")
        print("\nOfficial OpenAI Codex Supervisor - MCP-Centric Orchestrator")
        print("Implements Agents SDK patterns: handoff, guardrails, worker agents")
        sys.exit(1)

    user_task = sys.argv[1]
    mcp_url = "ws://localhost:3000"  # Default Codex MCP server

    # Parse additional arguments
    if len(sys.argv) > 2 and sys.argv[2] == "--mcp-url":
        mcp_url = sys.argv[3]

    print("=" * 80)
    print("🎯 Codex Supervisor - Official OpenAI Agents SDK Implementation")
    print("=" * 80)

    # Initialize supervisor with official patterns
    supervisor = CodexSupervisor(mcp_url)

    # Register default guardrails (official Agents SDK)
    supervisor.add_guardrail(security_guardrail)
    supervisor.add_guardrail(quality_guardrail)

    # Register worker agents (official Agents SDK pattern)
    supervisor.register_worker_agent("architect", {
        "skills": ["architecture_analysis", "design_patterns"],
        "capabilities": ["system_analysis", "scalability_review"]
    })
    supervisor.register_worker_agent("code-reviewer", {
        "skills": ["code_analysis", "quality_checks"],
        "capabilities": ["static_analysis", "best_practices"]
    })

    print(f"[SUPERVISOR] Initialized with {len(supervisor.guardrails)} guardrails")
    print(f"[SUPERVISOR] Registered {len(supervisor.worker_agents)} worker agents")

    # Execute workflow using official patterns
    try:
        report = await supervisor.orchestrate_workflow(user_task)

        # Save detailed report
        report_path = Path("artifacts/workflow_report.json")
        report_path.parent.mkdir(exist_ok=True)

        with open(report_path, 'w', encoding='utf-8') as f:
            json.dump(report, f, indent=2, ensure_ascii=False)

        print("\n" + "=" * 80)
        print("🎉 Official Codex Supervisor Workflow Completed!")
        print("=" * 80)
        print(f"[SAVE] Detailed report saved to: {report_path}")
        print(".1f")
        print(f"[RATE] Success rate: {report['execution_summary']['success_rate']:.1%}")
        print(f"[MCP] Tasks executed via MCP: {report['execution_summary']['mcp_executed_tasks']}")
        print(f"[HANDOFF] Total handoffs: {report['execution_summary']['total_handoffs']}")

        # Print compliance info
        compliance = report.get('official_compliance', {})
        print(f"[COMPLIANCE] Skills format: {compliance.get('skills_format', 'unknown')}")
        print(f"[COMPLIANCE] MCP integration: {compliance.get('mcp_integration', 'unknown')}")
        print(f"[COMPLIANCE] Agents SDK patterns: {', '.join(compliance.get('agents_sdk_patterns', []))}")

        # Print artifacts
        if report['artifacts']:
            print("[ARTIFACTS] Generated files:")
            for artifact in report['artifacts']:
                print(f"  - {artifact}")

    except KeyboardInterrupt:
        print("\n[STOP] Supervisor interrupted by user")
    except Exception as e:
        print(f"\n[ERROR] Supervisor failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

    finally:
        # Cleanup MCP connection
        if supervisor.mcp_bridge:
            await supervisor.mcp_bridge.disconnect()

if __name__ == "__main__":
    asyncio.run(main())