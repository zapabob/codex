"""
Codex Supervisor - Official OpenAI Codex Agents SDK Orchestrator

This package provides MCP-centric multi-agent workflow orchestration
for OpenAI Codex, implementing official Agents SDK patterns.
"""

__version__ = "1.0.0"
__author__ = "Zapabob"
__description__ = "Official OpenAI Codex Agents SDK Supervisor"

from .supervisor import CodexSupervisor, TaskStatus, AgentRole
from .mcp_bridge import CodexMCPBridge, create_mcp_bridge

__all__ = [
    "CodexSupervisor",
    "CodexMCPBridge",
    "TaskStatus",
    "AgentRole",
    "create_mcp_bridge",
]