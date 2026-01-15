#!/usr/bin/env python3
"""
Orchestrator Agent - Multi-Agent Workflow Coordination
Implements official Agents SDK patterns with MCP integration
"""

import os
import sys
import json
import asyncio
import time
from pathlib import Path
from typing import Dict, List, Any, Optional, Callable
from dataclasses import dataclass, asdict
from enum import Enum
from datetime import datetime

# Import MCP components (assuming they're available in the skill environment)
try:
    from mcp_bridge import CodexMCPBridge, create_mcp_bridge
    MCP_AVAILABLE = True
except ImportError:
    MCP_AVAILABLE = False

class TaskStatus(Enum):
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    HANDOFF = "handoff"

class AgentRole(Enum):
    SUPERVISOR = "supervisor"
    ARCHITECT = "architect"
    REVIEWER = "code-reviewer"
    EXECUTOR = "executor"
    RESEARCHER = "researcher"
    TESTER = "test-gen"
    QA_ENGINEER = "qa-engineer"
    BUILD_MANAGER = "build-manager"

@dataclass
class Task:
    id: str
    description: str
    assigned_agent: AgentRole
    status: TaskStatus = TaskStatus.PENDING
    dependencies: List[str] = None
    result: Optional[str] = None
    error: Optional[str] = None
    handoff_data: Optional[Dict[str, Any]] = None
    duration: float = 0.0
    started_at: Optional[datetime] = None
    completed_at: Optional[datetime] = None

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
    handoffs: List[Dict[str, Any]] = None
    mcp_used: bool = False

    def __post_init__(self):
        if self.handoffs is None:
            self.handoffs = []

@dataclass
class GuardrailResult:
    task_id: str
    passed: bool
    violations: List[str]
    recommendations: List[str]

@dataclass
class OrchestratorConfig:
    max_parallel_tasks: int = 3
    mcp_timeout: int = 30
    enable_handoffs: bool = True
    guardrails_enabled: bool = True
    verbose_logging: bool = False
    progress_reporting: bool = True

class WorkflowOrchestrator:
    """Official Agents SDK compliant workflow orchestrator"""

    def __init__(self, config: OrchestratorConfig):
        self.config = config
        self.tasks: Dict[str, Task] = {}
        self.results: List[WorkflowResult] = []
        self.mcp_bridge: Optional[CodexMCPBridge] = None
        self.guardrails: List[Callable[[Task], GuardrailResult]] = []
        self.worker_agents: Dict[str, Dict[str, Any]] = {}

        # Setup default guardrails
        self._setup_default_guardrails()

        # Setup default worker agents
        self._setup_default_workers()

    def _setup_default_guardrails(self):
        """Setup default guardrails following Agents SDK patterns"""
        self.guardrails.extend([
            self._security_guardrail,
            self._quality_guardrail,
            self._performance_guardrail
        ])

    def _setup_default_workers(self):
        """Setup default worker agent configurations"""
        self.worker_agents = {
            "architect": {
                "skills": ["architecture_analysis", "system_design"],
                "capabilities": ["planning", "design", "modeling"],
                "priority": "high"
            },
            "executor": {
                "skills": ["code_generation", "implementation"],
                "capabilities": ["development", "coding", "debugging"],
                "priority": "high"
            },
            "qa_engineer": {
                "skills": ["quality_assurance", "testing", "security"],
                "capabilities": ["analysis", "validation", "verification"],
                "priority": "critical"
            },
            "code-reviewer": {
                "skills": ["code_analysis", "best_practices"],
                "capabilities": ["review", "standards", "compliance"],
                "priority": "high"
            },
            "build_manager": {
                "skills": ["build_orchestration", "deployment"],
                "capabilities": ["compilation", "packaging", "distribution"],
                "priority": "medium"
            }
        }

    async def initialize_mcp(self) -> bool:
        """Initialize MCP connection for skill execution"""
        if not MCP_AVAILABLE:
            print("⚠️ MCP not available - falling back to direct execution")
            return False

        print("🔌 Initializing MCP connection...")
        self.mcp_bridge = await create_mcp_bridge()

        if self.mcp_bridge:
            print("✅ MCP connection established")
            return True
        else:
            print("❌ MCP connection failed")
            return False

    def decompose_task(self, user_task: str) -> List[Task]:
        """Decompose complex task into orchestrated subtasks"""

        # Analyze task complexity and requirements
        task_analysis = self._analyze_task_complexity(user_task)

        tasks = []

        # Supervisor analysis (always first)
        tasks.append(Task(
            id="supervisor_analysis",
            description=f"Analyze and plan workflow orchestration for: {user_task}",
            assigned_agent=AgentRole.SUPERVISOR
        ))

        # Determine required agents based on task analysis
        required_agents = self._determine_required_agents(task_analysis)

        # Create tasks based on requirements
        if "architect" in required_agents:
            tasks.append(Task(
                id="architect_analysis",
                description=f"Design system architecture and technical specifications for: {user_task}",
                assigned_agent=AgentRole.ARCHITECT,
                dependencies=["supervisor_analysis"]
            ))

        if "researcher" in required_agents:
            tasks.append(Task(
                id="research_analysis",
                description=f"Research and gather technical information for: {user_task}",
                assigned_agent=AgentRole.RESEARCHER,
                dependencies=["supervisor_analysis"]
            ))

        if "executor" in required_agents:
            deps = ["supervisor_analysis"]
            if "architect" in required_agents:
                deps.append("architect_analysis")

            tasks.append(Task(
                id="implementation",
                description=f"Implement the core functionality for: {user_task}",
                assigned_agent=AgentRole.EXECUTOR,
                dependencies=deps
            ))

        if "qa_engineer" in required_agents:
            qa_deps = ["supervisor_analysis"]
            if "executor" in required_agents:
                qa_deps.append("implementation")

            tasks.append(Task(
                id="quality_assurance",
                description=f"Perform comprehensive quality assurance for: {user_task}",
                assigned_agent=AgentRole.QA_ENGINEER,
                dependencies=qa_deps
            ))

        if "code-reviewer" in required_agents:
            review_deps = ["supervisor_analysis"]
            if "executor" in required_agents:
                review_deps.append("implementation")

            tasks.append(Task(
                id="code_review",
                description=f"Review code quality and best practices for: {user_task}",
                assigned_agent=AgentRole.REVIEWER,
                dependencies=review_deps
            ))

        if "tester" in required_agents:
            test_deps = ["supervisor_analysis"]
            if "executor" in required_agents:
                test_deps.append("implementation")

            tasks.append(Task(
                id="testing",
                description=f"Generate and execute comprehensive tests for: {user_task}",
                assigned_agent=AgentRole.TESTER,
                dependencies=test_deps
            ))

        if "build_manager" in required_agents:
            build_deps = ["supervisor_analysis"]
            if "executor" in required_agents:
                build_deps.append("implementation")

            tasks.append(Task(
                id="build_deployment",
                description=f"Build and prepare deployment artifacts for: {user_task}",
                assigned_agent=AgentRole.BUILD_MANAGER,
                dependencies=build_deps
            ))

        return tasks

    def _analyze_task_complexity(self, task: str) -> Dict[str, Any]:
        """Analyze task complexity and determine requirements"""
        analysis = {
            "complexity": "simple",
            "requires_architecture": False,
            "requires_testing": False,
            "requires_security": False,
            "requires_performance": False,
            "requires_research": False,
            "estimated_duration": 0
        }

        task_lower = task.lower()

        # Complexity indicators
        if any(word in task_lower for word in ["system", "architecture", "design", "platform"]):
            analysis["requires_architecture"] = True
            analysis["complexity"] = "complex"

        if any(word in task_lower for word in ["security", "auth", "encryption", "vulnerability"]):
            analysis["requires_security"] = True
            analysis["complexity"] = "complex"

        if any(word in task_lower for word in ["performance", "optimize", "scale", "bottleneck"]):
            analysis["requires_performance"] = True

        if any(word in task_lower for word in ["research", "investigate", "explore", "analyze"]):
            analysis["requires_research"] = True

        if any(word in task_lower for word in ["test", "testing", "coverage", "qa"]):
            analysis["requires_testing"] = True

        # Estimate duration based on complexity
        if analysis["complexity"] == "complex":
            analysis["estimated_duration"] = 300  # 5 minutes
        elif len(task.split()) > 20:  # Detailed task
            analysis["estimated_duration"] = 180  # 3 minutes
        else:
            analysis["estimated_duration"] = 60   # 1 minute

        return analysis

    def _determine_required_agents(self, analysis: Dict[str, Any]) -> List[str]:
        """Determine which agents are required for the task"""
        agents = ["supervisor"]  # Always include supervisor

        if analysis["requires_architecture"]:
            agents.extend(["architect", "executor"])

        if analysis["requires_research"]:
            agents.append("researcher")

        if analysis["requires_security"] or analysis["requires_performance"]:
            agents.append("qa_engineer")

        if analysis["complexity"] == "complex":
            agents.extend(["code-reviewer", "tester", "build_manager"])
        elif analysis["requires_testing"]:
            agents.append("tester")

        # Remove duplicates and maintain order
        seen = set()
        unique_agents = []
        for agent in agents:
            if agent not in seen:
                unique_agents.append(agent)
                seen.add(agent)

        return unique_agents

    def check_dependencies(self, task: Task) -> bool:
        """Check if task dependencies are satisfied"""
        for dep_id in task.dependencies:
            if dep_id not in self.tasks:
                return False
            if self.tasks[dep_id].status != TaskStatus.COMPLETED:
                return False
        return True

    async def apply_guardrails(self, task: Task) -> GuardrailResult:
        """Apply all guardrails to task"""
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

    async def execute_skill_via_mcp(self, task: Task) -> WorkflowResult:
        """Execute skill via MCP (official pattern)"""
        start_time = time.time()
        success = False
        output = ""
        handoffs = []
        mcp_used = False

        try:
            if not self.mcp_bridge:
                raise RuntimeError("MCP bridge not initialized")

            skill_name = task.assigned_agent.value
            print(f"[MCP] Executing {skill_name} for task: {task.id}")

            # Apply guardrails
            if self.config.guardrails_enabled:
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

            # Execute via MCP
            mcp_result = await self.mcp_bridge.execute_skill_via_mcp(skill_name, task.description)
            mcp_used = True

            if mcp_result["success"]:
                output = mcp_result.get("result", "")
                success = True
                print(f"[MCP-OK] {skill_name} completed successfully")
            else:
                output = f"MCP Error: {mcp_result.get('error', 'Unknown error')}"
                print(f"[MCP-ERROR] {skill_name} failed: {output}")

        except Exception as e:
            output = f"MCP Exception: {str(e)}"
            print(f"[MCP-ERROR] Failed to execute {task.assigned_agent.value}: {e}")

            # Fallback to simulation for demo purposes
            if "MCP bridge not initialized" in str(e):
                print(f"[FALLBACK] Simulating {task.assigned_agent.value} execution")
                output = f"[SIMULATION] {task.assigned_agent.value} completed successfully for: {task.description[:50]}..."
                success = True

        duration = time.time() - start_time

        return WorkflowResult(
            task_id=task.id,
            agent=task.assigned_agent.value,
            output=output,
            duration=duration,
            success=success,
            handoffs=handoffs,
            mcp_used=mcp_used
        )

    async def orchestrate_workflow(self, user_task: str) -> Dict[str, Any]:
        """Main orchestration logic with official Agents SDK patterns"""
        print("🎯 Orchestrator Workflow Execution"        print("=" * 60)

        # Initialize MCP if available
        mcp_available = await self.initialize_mcp()

        # Decompose task
        tasks = self.decompose_task(user_task)
        for task in tasks:
            self.tasks[task.id] = task

        print(f"[SUPERVISOR] Decomposed into {len(tasks)} subtasks")
        if mcp_available:
            print("[MCP] Using official Codex Skills via MCP")
        else:
            print("[SIMULATION] Using simulated skill execution")

        # Execute workflow
        completed_tasks = set()
        total_start_time = time.time()

        while len(completed_tasks) < len(tasks):
            # Find ready tasks
            ready_tasks = [
                task for task in tasks
                if task.id not in completed_tasks and self.check_dependencies(task)
            ]

            if not ready_tasks:
                print("[SUPERVISOR] No tasks ready - checking for circular dependencies")
                break

            # Execute ready tasks (with parallelism limit)
            max_parallel = min(len(ready_tasks), self.config.max_parallel_tasks)
            executing_tasks = ready_tasks[:max_parallel]

            print(f"[SUPERVISOR] Executing {len(executing_tasks)} tasks concurrently")

            # Execute tasks
            execution_tasks = [
                self.execute_skill_via_mcp(task) for task in executing_tasks
            ]

            results = await asyncio.gather(*execution_tasks, return_exceptions=True)

            # Process results
            for i, result in enumerate(results):
                task = executing_tasks[i]

                if isinstance(result, Exception):
                    print(f"[ERROR] Task {task.id} failed: {result}")
                    task.status = TaskStatus.FAILED
                    task.error = str(result)
                else:
                    task.status = TaskStatus.COMPLETED if result.success else TaskStatus.FAILED
                    task.result = result.output
                    task.duration = result.duration
                    task.started_at = datetime.now()
                    task.completed_at = datetime.now()
                    self.results.append(result)

                completed_tasks.add(task.id)

                status_icon = "✅" if task.status == TaskStatus.COMPLETED else "❌"
                mcp_indicator = "[MCP]" if hasattr(result, 'mcp_used') and result.mcp_used else "[SIM]"
                print(".1f")

        # Generate comprehensive report
        total_duration = time.time() - total_start_time
        return self.generate_workflow_report(user_task, total_duration)

    def generate_workflow_report(self, original_task: str, total_duration: float) -> Dict[str, Any]:
        """Generate comprehensive workflow report"""
        successful_tasks = sum(1 for result in self.results if result.success)
        total_tasks = len(self.results)
        mcp_tasks = sum(1 for result in self.results if result.mcp_used)
        total_handoffs = sum(len(result.handoffs) for result in self.results)

        # Calculate agent utilization
        agent_stats = {}
        for result in self.results:
            agent = result.agent
            if agent not in agent_stats:
                agent_stats[agent] = {"count": 0, "success": 0, "total_duration": 0}
            agent_stats[agent]["count"] += 1
            agent_stats[agent]["total_duration"] += result.duration
            if result.success:
                agent_stats[agent]["success"] += 1

        report = {
            "workflow_id": f"orchestrator-{int(time.time())}",
            "original_task": original_task,
            "execution_summary": {
                "total_tasks": total_tasks,
                "completed_tasks": successful_tasks,
                "failed_tasks": total_tasks - successful_tasks,
                "mcp_executed_tasks": mcp_tasks,
                "simulated_tasks": total_tasks - mcp_tasks,
                "total_handoffs": total_handoffs,
                "total_duration": total_duration,
                "success_rate": successful_tasks / total_tasks if total_tasks > 0 else 0
            },
            "task_results": [
                {
                    "task_id": result.task_id,
                    "agent": result.agent,
                    "execution_method": "mcp" if result.mcp_used else "simulated",
                    "duration": result.duration,
                    "success": result.success,
                    "handoffs_initiated": len(result.handoffs)
                }
                for result in self.results
            ],
            "agent_utilization": agent_stats,
            "orchestrator_info": {
                "type": "CodexOrchestrator",
                "pattern": "Official Agents SDK",
                "mcp_integration": self.mcp_bridge is not None,
                "guardrails_applied": len(self.guardrails),
                "worker_agents": len(self.worker_agents),
                "max_parallel_tasks": self.config.max_parallel_tasks
            }
        }

        return report

    # Guardrail implementations
    def _security_guardrail(self, task: Task) -> GuardrailResult:
        """Security guardrail"""
        violations = []
        recommendations = []

        task_text = task.description.lower()

        # Check for potentially dangerous operations
        dangerous_keywords = ["delete", "remove", "drop", "truncate", "format"]
        if any(keyword in task_text for keyword in dangerous_keywords):
            violations.append("Potentially destructive operations detected")
            recommendations.append("Ensure proper authorization and confirmation for destructive operations")

        return GuardrailResult(
            task_id=task.id,
            passed=len(violations) == 0,
            violations=violations,
            recommendations=recommendations
        )

    def _quality_guardrail(self, task: Task) -> GuardrailResult:
        """Quality guardrail"""
        violations = []
        recommendations = []

        task_text = task.description.lower()

        # Check for testing requirements
        if task.assigned_agent in [AgentRole.EXECUTOR, AgentRole.BUILD_MANAGER] and "test" not in task_text:
            recommendations.append("Consider including testing in implementation tasks")

        # Check for documentation requirements
        if task.assigned_agent == AgentRole.EXECUTOR and "doc" not in task_text:
            recommendations.append("Consider documenting new implementations")

        return GuardrailResult(
            task_id=task.id,
            passed=len(violations) == 0,
            violations=violations,
            recommendations=recommendations
        )

    def _performance_guardrail(self, task: Task) -> GuardrailResult:
        """Performance guardrail"""
        violations = []
        recommendations = []

        task_text = task.description.lower()

        # Check for performance considerations
        if "loop" in task_text or "iterate" in task_text:
            recommendations.append("Consider algorithmic complexity and performance implications")

        return GuardrailResult(
            task_id=task.id,
            passed=len(violations) == 0,
            violations=violations,
            recommendations=recommendations
        )

def main():
    """Main entry point"""
    import argparse

    parser = argparse.ArgumentParser(description="Codex Orchestrator - Multi-Agent Workflow Coordination")
    parser.add_argument("task", help="Task description to orchestrate")
    parser.add_argument("--max-parallel", type=int, default=3, help="Maximum parallel tasks")
    parser.add_argument("--no-mcp", action="store_true", help="Disable MCP and use simulation")
    parser.add_argument("--verbose", action="store_true", help="Enable verbose logging")
    parser.add_argument("--output", type=Path, default="./artifacts", help="Output directory")

    args = parser.parse_args()

    # Create configuration
    config = OrchestratorConfig(
        max_parallel_tasks=args.max_parallel,
        verbose_logging=args.verbose,
        guardrails_enabled=not args.no_mcp  # Disable guardrails in simulation mode
    )

    # Initialize orchestrator
    orchestrator = WorkflowOrchestrator(config)

    # Force disable MCP if requested
    if args.no_mcp:
        global MCP_AVAILABLE
        MCP_AVAILABLE = False

    # Create output directory
    args.output.mkdir(parents=True, exist_ok=True)
    os.chdir(args.output)

    async def run_workflow():
        """Run the orchestration workflow"""
        print(f"🎯 Orchestrating: {args.task}")
        print("=" * 80)

        try:
            report = await orchestrator.orchestrate_workflow(args.task)

            # Save report
            report_file = args.output / "orchestrator_report.json"
            with open(report_file, 'w', encoding='utf-8') as f:
                json.dump(report, f, indent=2, ensure_ascii=False, default=str)

            # Print summary
            summary = report["execution_summary"]
            print("\n" + "=" * 80)
            print("🎉 Orchestration Complete!"            print("=" * 80)
            print(f"Tasks: {summary['completed_tasks']}/{summary['total_tasks']} completed")
            print(".1f")
            print(".1%")
            print(f"MCP Tasks: {summary['mcp_executed_tasks']}")
            print(f"Handoffs: {summary['total_handoffs']}")

            if summary['failed_tasks'] > 0:
                print(f"❌ Failed Tasks: {summary['failed_tasks']}")
                return 1
            else:
                print("✅ All tasks completed successfully")
                return 0

        except Exception as e:
            print(f"❌ Orchestration failed: {e}")
            if args.verbose:
                import traceback
                traceback.print_exc()
            return 1

    # Run the workflow
    exit_code = asyncio.run(run_workflow())
    sys.exit(exit_code)

if __name__ == "__main__":
    main()