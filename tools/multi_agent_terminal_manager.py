#!/usr/bin/env python3
"""
Multi-Agent Terminal Manager
マルチエージェントシステムでのターミナル管理と協調制御
"""

import os
import sys
import json
import asyncio
import subprocess
import threading
import time
import signal
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple
from dataclasses import dataclass, asdict
from enum import Enum
import logging
import tempfile
import socket

# Import conflict prevention engine
try:
    from conflict_prevention_engine import ConflictPreventionEngine, AgentTerminal
except ImportError:
    print("Warning: ConflictPreventionEngine not available")
    ConflictPreventionEngine = None
    AgentTerminal = None

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class AgentRole(Enum):
    CODE_REVIEWER = "code-reviewer"
    TEST_RUNNER = "test-runner"
    SECURITY_AUDITOR = "security-auditor"
    PERFORMANCE_ANALYZER = "performance-analyzer"
    DOCUMENTATION_WRITER = "documentation-writer"
    ARCHITECT = "architect"
    QA_ENGINEER = "qa-engineer"

class AgentStatus(Enum):
    IDLE = "idle"
    WORKING = "working"
    WAITING = "waiting"
    COMPLETED = "completed"
    ERROR = "error"

@dataclass
class AgentProcess:
    agent_id: str
    role: AgentRole
    terminal: Optional[AgentTerminal]
    status: AgentStatus
    working_directory: Path
    assigned_task: Optional[str]
    start_time: Optional[float]
    end_time: Optional[float]
    results: Dict[str, Any]

@dataclass
class CoordinationTask:
    task_id: str
    description: str
    assigned_agents: List[str]
    dependencies: List[str]
    status: str
    created_at: float
    completed_at: Optional[float]

class MultiAgentTerminalManager:
    """マルチエージェントシステムでのターミナル管理と協調制御"""

    def __init__(self, repo_path: str, max_terminals: int = 5):
        self.repo_path = Path(repo_path)
        self.max_terminals = max_terminals
        self.active_agents: Dict[str, AgentProcess] = {}
        self.coordination_tasks: Dict[str, CoordinationTask] = {}
        self.conflict_engine = ConflictPreventionEngine(repo_path) if ConflictPreventionEngine else None

        # Communication setup
        self.coordination_socket = None
        self.agent_sockets: Dict[str, socket.socket] = {}

        # Setup coordination server
        self._setup_coordination_server()

    def _setup_coordination_server(self):
        """エージェント間通信用のサーバーをセットアップ"""
        try:
            self.coordination_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.coordination_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            self.coordination_socket.bind(('localhost', 0))  # Auto-assign port
            self.coordination_socket.listen(10)

            port = self.coordination_socket.getsockname()[1]
            logger.info(f"Coordination server started on port {port}")

            # Start coordination thread
            coordination_thread = threading.Thread(target=self._handle_coordination, daemon=True)
            coordination_thread.start()

        except Exception as e:
            logger.error(f"Failed to setup coordination server: {e}")

    def _handle_coordination(self):
        """エージェント間通信のハンドリング"""
        while True:
            try:
                client_socket, addr = self.coordination_socket.accept()
                logger.info(f"Agent connected from {addr}")

                # Handle agent communication in separate thread
                agent_thread = threading.Thread(
                    target=self._handle_agent_communication,
                    args=(client_socket,),
                    daemon=True
                )
                agent_thread.start()

            except Exception as e:
                logger.error(f"Coordination error: {e}")
                break

    def _handle_agent_communication(self, client_socket: socket.socket):
        """個別のエージェント通信を処理"""
        try:
            # Receive agent identification
            data = client_socket.recv(1024).decode()
            agent_info = json.loads(data)

            agent_id = agent_info.get('agent_id')
            if agent_id:
                self.agent_sockets[agent_id] = client_socket
                logger.info(f"Agent {agent_id} registered for communication")

                # Handle ongoing communication
                while True:
                    try:
                        data = client_socket.recv(1024)
                        if not data:
                            break

                        message = json.loads(data.decode())
                        self._process_agent_message(agent_id, message)

                    except Exception as e:
                        logger.error(f"Error handling message from {agent_id}: {e}")
                        break

        except Exception as e:
            logger.error(f"Agent communication error: {e}")
        finally:
            client_socket.close()

    def _process_agent_message(self, agent_id: str, message: Dict[str, Any]):
        """エージェントからのメッセージを処理"""
        message_type = message.get('type')

        if message_type == 'task_completed':
            self._handle_task_completion(agent_id, message)
        elif message_type == 'conflict_detected':
            self._handle_conflict_detection(agent_id, message)
        elif message_type == 'assistance_requested':
            self._handle_assistance_request(agent_id, message)
        elif message_type == 'status_update':
            self._handle_status_update(agent_id, message)

    def _handle_task_completion(self, agent_id: str, message: Dict[str, Any]):
        """タスク完了を処理"""
        task_id = message.get('task_id')
        results = message.get('results', {})

        if agent_id in self.active_agents:
            agent = self.active_agents[agent_id]
            agent.status = AgentStatus.COMPLETED
            agent.end_time = time.time()
            agent.results = results

            logger.info(f"Agent {agent_id} completed task {task_id}")

            # Update coordination task
            if task_id in self.coordination_tasks:
                task = self.coordination_tasks[task_id]
                task.completed_at = time.time()

                # Check if all dependent tasks are complete
                self._check_task_dependencies(task_id)

    def _handle_conflict_detection(self, agent_id: str, message: Dict[str, Any]):
        """コンフリクト検知を処理"""
        conflict_info = message.get('conflict', {})

        logger.warning(f"Conflict detected by agent {agent_id}: {conflict_info}")

        # Notify other agents about the conflict
        self._broadcast_conflict(conflict_info)

        # Launch conflict resolution agent if needed
        if conflict_info.get('severity', 'low') in ['high', 'critical']:
            self.launch_conflict_resolution_agent(conflict_info)

    def _handle_assistance_request(self, agent_id: str, message: Dict[str, Any]):
        """支援リクエストを処理"""
        assistance_type = message.get('assistance_type')
        context = message.get('context', {})

        logger.info(f"Agent {agent_id} requested assistance: {assistance_type}")

        # Find suitable agent to provide assistance
        helper_agent = self._find_helper_agent(assistance_type, context)
        if helper_agent:
            self._assign_assistance_task(helper_agent, agent_id, assistance_type, context)

    def _handle_status_update(self, agent_id: str, message: Dict[str, Any]):
        """ステータス更新を処理"""
        status = message.get('status')
        details = message.get('details', {})

        if agent_id in self.active_agents:
            agent = self.active_agents[agent_id]
            agent.status = AgentStatus(status) if status else agent.status

            logger.info(f"Agent {agent_id} status: {agent.status.value}")

    def _broadcast_conflict(self, conflict_info: Dict[str, Any]):
        """コンフリクト情報を全エージェントにブロードキャスト"""
        message = {
            'type': 'conflict_alert',
            'conflict': conflict_info,
            'timestamp': time.time()
        }

        for agent_id, sock in self.agent_sockets.items():
            try:
                sock.send(json.dumps(message).encode())
            except Exception as e:
                logger.error(f"Failed to send conflict alert to {agent_id}: {e}")

    def launch_agent(self, agent_role: AgentRole, task_description: str,
                    working_dir: Optional[Path] = None) -> Optional[str]:
        """指定された役割のエージェントを起動"""
        if len(self.active_agents) >= self.max_terminals:
            logger.warning("Maximum terminal limit reached")
            return None

        agent_id = f"{agent_role.value}_{int(time.time())}"
        working_directory = working_dir or self.repo_path

        # Launch terminal for agent
        if self.conflict_engine:
            terminal = self.conflict_engine.launch_agent_terminal(agent_id, working_directory)
        else:
            # Fallback terminal launch
            terminal = self._launch_terminal_fallback(agent_id, working_directory)

        if not terminal:
            logger.error(f"Failed to launch terminal for agent {agent_id}")
            return None

        # Create agent process
        agent = AgentProcess(
            agent_id=agent_id,
            role=agent_role,
            terminal=terminal,
            status=AgentStatus.IDLE,
            working_directory=working_directory,
            assigned_task=task_description,
            start_time=time.time(),
            end_time=None,
            results={}
        )

        self.active_agents[agent_id] = agent

        # Send initial task to agent
        self._send_task_to_agent(agent_id, task_description)

        logger.info(f"Launched agent {agent_id} with role {agent_role.value}")
        return agent_id

    def _launch_terminal_fallback(self, agent_id: str, working_dir: Path) -> Optional[AgentTerminal]:
        """ConflictPreventionEngineが利用できない場合のフォールバック"""
        try:
            if os.name == 'nt':  # Windows
                process = subprocess.Popen(
                    ['cmd.exe', '/c', 'start', 'cmd.exe'],
                    cwd=str(working_dir),
                    creationflags=subprocess.CREATE_NEW_CONSOLE
                )
            else:  # Unix-like
                process = subprocess.Popen(
                    ['x-terminal-emulator', '-e', 'bash'],
                    cwd=str(working_dir)
                )

            return AgentTerminal(
                agent_id=agent_id,
                terminal_id=f"fallback_{agent_id}",
                working_directory=working_dir,
                process=process,
                status="active",
                created_at=datetime.now(),
                last_activity=datetime.now()
            )

        except Exception as e:
            logger.error(f"Fallback terminal launch failed: {e}")
            return None

    def launch_conflict_resolution_agent(self, conflict_info: Dict[str, Any]):
        """コンフリクト解決専門エージェントを起動"""
        task_description = f"""
Resolve merge conflict in {conflict_info.get('file_path', 'unknown file')}

Conflict Details:
- Type: {conflict_info.get('conflict_type', 'unknown')}
- Risk Level: {conflict_info.get('risk_level', 'unknown')}
- Lines: {conflict_info.get('predicted_lines', [])}
- Reason: {conflict_info.get('reason', 'unknown')}

Please analyze the conflict and provide resolution recommendations.
"""

        agent_id = self.launch_agent(AgentRole.CODE_REVIEWER, task_description)
        if agent_id:
            logger.info(f"Launched conflict resolution agent: {agent_id}")

    def _send_task_to_agent(self, agent_id: str, task: str):
        """エージェントにタスクを送信"""
        if agent_id in self.agent_sockets:
            try:
                message = {
                    'type': 'task_assignment',
                    'task': task,
                    'timestamp': time.time()
                }
                self.agent_sockets[agent_id].send(json.dumps(message).encode())
            except Exception as e:
                logger.error(f"Failed to send task to agent {agent_id}: {e}")

    def _find_helper_agent(self, assistance_type: str, context: Dict[str, Any]) -> Optional[str]:
        """支援を提供できるエージェントを探す"""
        suitable_roles = {
            'code_review': [AgentRole.CODE_REVIEWER],
            'testing': [AgentRole.TEST_RUNNER],
            'security': [AgentRole.SECURITY_AUDITOR],
            'performance': [AgentRole.PERFORMANCE_ANALYZER],
            'documentation': [AgentRole.DOCUMENTATION_WRITER],
            'architecture': [AgentRole.ARCHITECT]
        }

        target_roles = suitable_roles.get(assistance_type, [AgentRole.CODE_REVIEWER])

        # Find available agent with suitable role
        for agent_id, agent in self.active_agents.items():
            if agent.status == AgentStatus.IDLE and agent.role in target_roles:
                return agent_id

        return None

    def _assign_assistance_task(self, helper_agent: str, requesting_agent: str,
                               assistance_type: str, context: Dict[str, Any]):
        """支援タスクを割り当て"""
        task_description = f"""
Provide assistance to agent {requesting_agent}

Assistance Type: {assistance_type}
Context: {json.dumps(context, indent=2)}

Please help resolve the issue and provide recommendations.
"""

        self._send_task_to_agent(helper_agent, task_description)

    def create_coordination_task(self, description: str, assigned_agents: List[str],
                                dependencies: List[str] = None) -> str:
        """協調タスクを作成"""
        task_id = f"task_{int(time.time())}"
        dependencies = dependencies or []

        task = CoordinationTask(
            task_id=task_id,
            description=description,
            assigned_agents=assigned_agents,
            dependencies=dependencies,
            status="pending",
            created_at=time.time(),
            completed_at=None
        )

        self.coordination_tasks[task_id] = task

        # Launch agents for this task
        for agent_id in assigned_agents:
            if agent_id in self.active_agents:
                agent = self.active_agents[agent_id]
                agent.assigned_task = description
                agent.status = AgentStatus.WORKING

        logger.info(f"Created coordination task {task_id} with {len(assigned_agents)} agents")
        return task_id

    def _check_task_dependencies(self, completed_task_id: str):
        """タスク依存関係をチェック"""
        for task_id, task in self.coordination_tasks.items():
            if completed_task_id in task.dependencies:
                # Check if all dependencies are complete
                all_complete = all(
                    dep_id in self.coordination_tasks and
                    self.coordination_tasks[dep_id].completed_at is not None
                    for dep_id in task.dependencies
                )

                if all_complete and task.status == "pending":
                    task.status = "ready"
                    logger.info(f"Task {task_id} is now ready (all dependencies completed)")

    def get_system_status(self) -> Dict[str, Any]:
        """システム全体のステータスを取得"""
        return {
            "active_agents": len(self.active_agents),
            "active_terminals": len([a for a in self.active_agents.values() if a.terminal]),
            "coordination_tasks": len(self.coordination_tasks),
            "pending_tasks": len([t for t in self.coordination_tasks.values() if t.status == "pending"]),
            "completed_tasks": len([t for t in self.coordination_tasks.values() if t.completed_at]),
            "agent_status": {
                agent_id: {
                    "role": agent.role.value,
                    "status": agent.status.value,
                    "task": agent.assigned_task,
                    "uptime": time.time() - (agent.start_time or time.time())
                }
                for agent_id, agent in self.active_agents.items()
            }
        }

    def shutdown(self):
        """システムをシャットダウン"""
        logger.info("Shutting down multi-agent terminal manager...")

        # Close coordination socket
        if self.coordination_socket:
            self.coordination_socket.close()

        # Close agent sockets
        for sock in self.agent_sockets.values():
            try:
                sock.close()
            except:
                pass

        # Cleanup conflict engine
        if self.conflict_engine:
            self.conflict_engine.cleanup_terminals()

        logger.info("Multi-agent terminal manager shutdown complete")

async def demonstrate_multi_agent_system():
    """マルチエージェントシステムの実演"""
    print("🚀 Multi-Agent Terminal Manager Demonstration")
    print("=" * 60)

    # Initialize manager
    manager = MultiAgentTerminalManager(".")

    try:
        # Launch multiple agents
        print("🎯 Launching agents...")

        # Code reviewer agent
        reviewer_id = manager.launch_agent(
            AgentRole.CODE_REVIEWER,
            "Review code changes for best practices and potential issues"
        )

        # Test runner agent
        tester_id = manager.launch_agent(
            AgentRole.TEST_RUNNER,
            "Run automated tests and validate functionality"
        )

        # Security auditor
        security_id = manager.launch_agent(
            AgentRole.SECURITY_AUDITOR,
            "Audit code for security vulnerabilities and compliance"
        )

        print(f"✅ Launched agents: {reviewer_id}, {tester_id}, {security_id}")

        # Create coordination task
        task_id = manager.create_coordination_task(
            "Perform comprehensive code review and testing for merge request",
            [reviewer_id, tester_id, security_id] if all([reviewer_id, tester_id, security_id]) else []
        )

        print(f"📋 Created coordination task: {task_id}")

        # Simulate some work
        await asyncio.sleep(2)

        # Get system status
        status = manager.get_system_status()
        print("📊 System Status:")
        print(json.dumps(status, indent=2, default=str))

        # Wait for tasks to complete (in real scenario)
        print("⏳ Waiting for agents to complete tasks...")
        await asyncio.sleep(5)

        # Final status
        final_status = manager.get_system_status()
        print("🏁 Final System Status:")
        print(json.dumps(final_status, indent=2, default=str))

    finally:
        manager.shutdown()

def main():
    """メインエントリーポイント"""
    if len(sys.argv) > 1:
        if sys.argv[1] == "demo":
            asyncio.run(demonstrate_multi_agent_system())
        elif sys.argv[1] == "launch-agent":
            if len(sys.argv) < 3:
                print("Usage: python multi_agent_terminal_manager.py launch-agent <role>")
                sys.exit(1)

            role_name = sys.argv[2]
            try:
                role = AgentRole(role_name)
            except ValueError:
                print(f"Invalid role: {role_name}")
                print("Available roles:", [r.value for r in AgentRole])
                sys.exit(1)

            manager = MultiAgentTerminalManager(".")
            agent_id = manager.launch_agent(role, f"Agent for {role_name} role")
            if agent_id:
                print(f"Launched agent: {agent_id}")
                # Keep running to maintain terminals
                try:
                    while True:
                        time.sleep(1)
                except KeyboardInterrupt:
                    manager.shutdown()
            else:
                print("Failed to launch agent")
                sys.exit(1)
        else:
            print("Usage: python multi_agent_terminal_manager.py [demo|launch-agent <role>]")
    else:
        print("Multi-Agent Terminal Manager")
        print("Usage:")
        print("  python multi_agent_terminal_manager.py demo")
        print("  python multi_agent_terminal_manager.py launch-agent <role>")
        print("")
        print("Available roles:")
        for role in AgentRole:
            print(f"  - {role.value}")

if __name__ == "__main__":
    main()