#!/usr/bin/env python3
"""
MCP Bridge for Codex Agents SDK Integration
"""

import asyncio
import json
import websockets
from typing import Dict, Any, Optional, List
import logging
from dataclasses import dataclass
from enum import Enum
from pathlib import Path

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class MCPMessageType(Enum):
    REQUEST = "request"
    RESPONSE = "response"
    NOTIFICATION = "notification"

@dataclass
class MCPMessage:
    type: MCPMessageType
    method: str
    params: Optional[Dict[str, Any]] = None
    id: Optional[str] = None
    result: Optional[Any] = None
    error: Optional[Dict[str, Any]] = None

class CodexMCPBridge:
    """Bridge between Codex MCP server and Agents SDK orchestrator"""

    def __init__(self, mcp_url: str = "ws://localhost:3000"):
        self.mcp_url = mcp_url
        self.websocket: Optional[websockets.WebSocketServerProtocol] = None
        self.message_id = 0
        self.pending_requests: Dict[str, asyncio.Future] = {}

    async def connect(self) -> bool:
        """Connect to Codex MCP server"""
        try:
            logger.info(f"Connecting to MCP server: {self.mcp_url}")
            self.websocket = await websockets.connect(self.mcp_url)
            logger.info("Connected to MCP server")

            # Start message handler
            asyncio.create_task(self._message_handler())

            # Initialize connection
            await self._initialize()

            return True

        except Exception as e:
            logger.error(f"Failed to connect to MCP server: {e}")
            return False

    async def disconnect(self):
        """Disconnect from MCP server"""
        if self.websocket:
            await self.websocket.close()
            logger.info("Disconnected from MCP server")

    def _get_next_id(self) -> str:
        """Generate next message ID"""
        self.message_id += 1
        return str(self.message_id)

    async def _initialize(self):
        """Initialize MCP connection"""
        await self.send_request("initialize", {
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {},
                "resources": {},
                "prompts": {}
            },
            "clientInfo": {
                "name": "codex-agents-sdk-orchestrator",
                "version": "1.0.0"
            }
        })

    async def _message_handler(self):
        """Handle incoming MCP messages"""
        try:
            async for message in self.websocket:
                try:
                    data = json.loads(message)
                    await self._handle_message(data)
                except json.JSONDecodeError as e:
                    logger.error(f"Invalid JSON received: {e}")
        except websockets.exceptions.ConnectionClosed:
            logger.info("MCP connection closed")
        except Exception as e:
            logger.error(f"Message handler error: {e}")

    async def _handle_message(self, data: Dict[str, Any]):
        """Handle incoming MCP message"""
        msg_type = data.get("type")
        msg_id = data.get("id")

        if msg_type == "response" and msg_id in self.pending_requests:
            # Resolve pending request
            future = self.pending_requests.pop(msg_id)
            if "result" in data:
                future.set_result(data["result"])
            elif "error" in data:
                future.set_exception(Exception(data["error"]))
            else:
                future.set_result(None)

        elif msg_type == "notification":
            # Handle notifications
            await self._handle_notification(data)

        else:
            logger.debug(f"Received message: {data}")

    async def _handle_notification(self, notification: Dict[str, Any]):
        """Handle MCP notifications"""
        method = notification.get("method", "")
        params = notification.get("params", {})

        if method == "tools/list":
            logger.info("Received tools list update")
        elif method.startswith("resources/"):
            logger.info(f"Resource update: {method}")
        else:
            logger.debug(f"Notification: {method}")

    async def send_request(self, method: str, params: Optional[Dict[str, Any]] = None) -> Any:
        """Send MCP request and wait for response"""
        msg_id = self._get_next_id()

        message = {
            "type": "request",
            "id": msg_id,
            "method": method
        }

        if params:
            message["params"] = params

        # Create future for response
        future = asyncio.Future()
        self.pending_requests[msg_id] = future

        # Send message
        await self.websocket.send(json.dumps(message))

        # Wait for response with timeout
        try:
            result = await asyncio.wait_for(future, timeout=30.0)
            return result
        except asyncio.TimeoutError:
            logger.error(f"Request timeout: {method}")
            self.pending_requests.pop(msg_id, None)
            raise
        except Exception as e:
            logger.error(f"Request failed: {e}")
            raise

    async def list_tools(self) -> List[Dict[str, Any]]:
        """List available MCP tools"""
        try:
            result = await self.send_request("tools/list")
            return result.get("tools", [])
        except Exception as e:
            logger.error(f"Failed to list tools: {e}")
            return []

    async def call_tool(self, tool_name: str, arguments: Dict[str, Any]) -> Any:
        """Call an MCP tool"""
        try:
            result = await self.send_request("tools/call", {
                "name": tool_name,
                "arguments": arguments
            })
            return result
        except Exception as e:
            logger.error(f"Failed to call tool {tool_name}: {e}")
            raise

    async def list_resources(self) -> List[Dict[str, Any]]:
        """List available MCP resources"""
        try:
            result = await self.send_request("resources/list")
            return result.get("resources", [])
        except Exception as e:
            logger.error(f"Failed to list resources: {e}")
            return []

    async def read_resource(self, uri: str) -> str:
        """Read MCP resource"""
        try:
            result = await self.send_request("resources/read", {"uri": uri})
            return result.get("contents", [{}])[0].get("text", "")
        except Exception as e:
            logger.error(f"Failed to read resource {uri}: {e}")
            raise

    async def execute_skill_via_mcp(self, skill_name: str, task_description: str) -> Dict[str, Any]:
        """Execute a skill through MCP interface"""

        logger.info(f"Executing skill '{skill_name}' via MCP for task: {task_description}")

        # Check available tools
        tools = await self.list_tools()

        # Find skill-related tools
        skill_tools = [tool for tool in tools if skill_name.lower() in tool.get("name", "").lower()]

        if not skill_tools:
            # Fallback: try to execute skill directly
            logger.warning(f"No MCP tools found for skill '{skill_name}', falling back to direct execution")
            return await self._execute_skill_directly(skill_name, task_description)

        # Use first available tool
        tool = skill_tools[0]
        tool_name = tool["name"]

        logger.info(f"Using MCP tool: {tool_name}")

        # Call the tool
        result = await self.call_tool(tool_name, {
            "task": task_description,
            "skill": skill_name
        })

        return {
            "success": True,
            "tool_used": tool_name,
            "result": result,
            "method": "mcp"
        }

    async def _execute_skill_directly(self, skill_name: str, task_description: str) -> Dict[str, Any]:
        """Fallback: Execute skill directly via subprocess"""
        import subprocess
        import sys

        try:
            # Find skill script
            skill_dir = Path(".codex/skills") / skill_name
            script_path = skill_dir / "scripts" / f"run_{skill_name}.py"

            if not script_path.exists():
                raise FileNotFoundError(f"Skill script not found: {script_path}")

            # Execute script
            process = await asyncio.create_subprocess_exec(
                sys.executable, str(script_path),
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                cwd=Path.cwd()
            )

            stdout, stderr = await process.communicate()

            success = process.returncode == 0
            output = stdout.decode('utf-8') if success else stderr.decode('utf-8')

            return {
                "success": success,
                "output": output,
                "method": "direct"
            }

        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "method": "direct"
            }

# Utility functions for orchestrator integration
async def create_mcp_bridge(mcp_url: str = "ws://localhost:3000") -> Optional[CodexMCPBridge]:
    """Create and connect MCP bridge"""
    bridge = CodexMCPBridge(mcp_url)

    if await bridge.connect():
        return bridge
    else:
        return None

async def execute_skill_with_fallback(skill_name: str, task: str, mcp_bridge: Optional[CodexMCPBridge] = None):
    """Execute skill with MCP bridge or direct fallback"""

    if mcp_bridge:
        try:
            result = await mcp_bridge.execute_skill_via_mcp(skill_name, task)
            if result["success"]:
                return result
        except Exception as e:
            logger.warning(f"MCP execution failed, falling back to direct: {e}")

    # Fallback to direct execution
    if mcp_bridge:
        return await mcp_bridge._execute_skill_directly(skill_name, task)
    else:
        # Create temporary bridge just for direct execution
        temp_bridge = CodexMCPBridge()
        return await temp_bridge._execute_skill_directly(skill_name, task)

if __name__ == "__main__":
    # Test MCP connection
    async def test():
        bridge = await create_mcp_bridge()
        if bridge:
            print("MCP bridge connected successfully")

            # Test tool listing
            tools = await bridge.list_tools()
            print(f"Available tools: {len(tools)}")

            for tool in tools[:5]:  # Show first 5
                print(f"  - {tool.get('name', 'unknown')}")

            await bridge.disconnect()
        else:
            print("Failed to connect to MCP server")

    asyncio.run(test())
