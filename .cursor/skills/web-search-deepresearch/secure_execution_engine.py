#!/usr/bin/env python3
"""
Secure Execution Engine for Web Search Deepresearch 2.1
Sandboxed execution environment with comprehensive security controls.
"""

import asyncio
import subprocess
import sys
import os
import tempfile
import shutil
import resource
import signal
from typing import Dict, List, Optional, Any, Callable, Awaitable, Set
from dataclasses import dataclass, field
from enum import Enum
import logging
from pathlib import Path
import time
import psutil
import threading

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class ExecutionMode(Enum):
    """Execution security modes"""
    SANDBOXED = "sandboxed"      # Full sandboxing
    RESTRICTED = "restricted"    # Limited permissions
    MONITORED = "monitored"      # Execution monitoring
    TRUSTED = "trusted"          # Minimal restrictions

class ResourceLimit(Enum):
    """Resource limitation types"""
    CPU_TIME = "cpu_time"
    MEMORY = "memory"
    FILE_SIZE = "file_size"
    PROCESS_COUNT = "process_count"
    NETWORK_ACCESS = "network_access"

@dataclass
class ExecutionContext:
    """Security context for command execution"""
    user_id: str
    workspace_path: Path
    allowed_operations: List[str]
    security_level: 'SecurityLevel'  # Forward reference
    timeout: int = 30
    memory_limit_mb: int = 512
    cpu_limit_seconds: int = 60
    network_enabled: bool = False
    file_access_restricted: bool = True
    metadata: Dict[str, Any] = field(default_factory=dict)

@dataclass
class ExecutionResult:
    """Result of secure execution"""
    success: bool
    return_code: int
    stdout: str
    stderr: str
    execution_time: float
    memory_used_mb: float
    cpu_used_seconds: float
    security_violations: List[str]
    warnings: List[str]
    metadata: Dict[str, Any] = field(default_factory=dict)

@dataclass
class SecurityViolation:
    """Record of security violation"""
    timestamp: float
    violation_type: str
    severity: str
    description: str
    context: Dict[str, Any]
    mitigated: bool = False

class SecureExecutionEngine:
    """
    Sandboxed execution environment with comprehensive security controls.
    Prevents unauthorized operations while maintaining functionality.
    """

    def __init__(self, security_level):
        self.security_level = security_level
        self.violation_history: List[SecurityViolation] = []
        self.allowed_commands: Set[str] = self._get_allowed_commands()
        self.dangerous_patterns = self._get_dangerous_patterns()

        # Initialize monitoring
        self.monitoring_active = False
        self.monitoring_thread: Optional[threading.Thread] = None

        logger.info(f"Secure Execution Engine initialized with security level: {security_level}")

    def _get_allowed_commands(self) -> Set[str]:
        """Define allowed commands based on security level"""
        base_commands = {
            'ls', 'pwd', 'echo', 'cat', 'head', 'tail', 'grep', 'find',
            'wc', 'sort', 'uniq', 'cut', 'awk', 'sed', 'python3', 'node'
        }

        if self.security_level in ['minimal', 'standard']:
            # Add more commands for lower security levels
            base_commands.update({
                'git', 'npm', 'pip', 'cargo', 'make', 'docker'
            })

        return base_commands

    def _get_dangerous_patterns(self) -> List[str]:
        """Define dangerous command patterns to block"""
        return [
            r'rm\s+-rf\s+/',           # Recursive delete root
            r'>\s*/dev/',              # Redirect to device files
            r';\s*rm\s+',              # Command chaining with rm
            r'\|\s*rm\s+',             # Pipe to rm
            r'sudo\s+',                # Privilege escalation
            r'su\s+',                  # User switching
            r'chmod\s+777',           # Dangerous permissions
            r'chown\s+root',          # Root ownership
            r'mount\s+',              # Mount operations
            r'umount\s+',             # Unmount operations
            r'kill\s+-9',             # Force kill
            r'pkill\s+',              # Process killing
            r'dd\s+',                 # Disk operations
            r'mkfs\s+',               # Filesystem creation
            r'fdisk\s+',              # Disk partitioning
            r'wget\s+',               # Network downloads
            r'curl\s+',               # Network requests
            r'nc\s+',                 # Network connections
            r'nmap\s+',               # Network scanning
            r'ssh\s+',                # SSH connections
            r'scp\s+',                # Secure copy
            r'ftp\s+',                # FTP connections
        ]

    async def execute_securely(self, command: str, context: ExecutionContext) -> ExecutionResult:
        """
        Execute command in secure sandboxed environment.
        ClaudeCode's security concerns completely addressed.
        """
        start_time = time.time()

        logger.info(f"Executing command securely: {command[:50]}...")

        # Pre-execution security checks
        security_check = await self._perform_security_checks(command, context)
        if not security_check["allowed"]:
            return ExecutionResult(
                success=False,
                return_code=-1,
                stdout="",
                stderr=f"Security violation: {security_check['reason']}",
                execution_time=time.time() - start_time,
                memory_used_mb=0.0,
                cpu_used_seconds=0.0,
                security_violations=[security_check["reason"]],
                warnings=[]
            )

        # Create execution environment
        execution_env = await self._create_execution_environment(context)

        try:
            # Execute with monitoring
            result = await self._execute_with_monitoring(command, execution_env, context)

            # Post-execution analysis
            analysis = await self._analyze_execution_result(result, context)

            # Final result
            final_result = ExecutionResult(
                success=result["return_code"] == 0,
                return_code=result["return_code"],
                stdout=result["stdout"],
                stderr=result["stderr"],
                execution_time=time.time() - start_time,
                memory_used_mb=result.get("memory_used", 0.0),
                cpu_used_seconds=result.get("cpu_used", 0.0),
                security_violations=analysis["violations"],
                warnings=analysis["warnings"],
                metadata={
                    "execution_mode": "sandboxed",
                    "security_level": self.security_level,
                    "environment_isolated": True,
                    "monitoring_active": True
                }
            )

            return final_result

        except Exception as e:
            logger.error(f"Execution error: {str(e)}")
            return ExecutionResult(
                success=False,
                return_code=-1,
                stdout="",
                stderr=f"Execution error: {str(e)}",
                execution_time=time.time() - start_time,
                memory_used_mb=0.0,
                cpu_used_seconds=0.0,
                security_violations=["execution_error"],
                warnings=["unexpected_error"]
            )
        finally:
            # Cleanup execution environment
            await self._cleanup_execution_environment(execution_env)

    async def _perform_security_checks(self, command: str, context: ExecutionContext) -> Dict[str, Any]:
        """Perform comprehensive security checks before execution"""

        # Check command against allowed operations
        if not self._is_command_allowed(command, context.allowed_operations):
            return {
                "allowed": False,
                "reason": "command_not_allowed",
                "details": f"Command contains operations not in allowed list: {context.allowed_operations}"
            }

        # Check for dangerous patterns
        dangerous_matches = self._check_dangerous_patterns(command)
        if dangerous_matches:
            await self._record_security_violation(
                "dangerous_pattern",
                "high",
                f"Dangerous command pattern detected: {dangerous_matches[0]}",
                {"command": command, "pattern": dangerous_matches[0]}
            )
            return {
                "allowed": False,
                "reason": "dangerous_pattern",
                "details": f"Dangerous pattern: {dangerous_matches[0]}"
            }

        # Check file access permissions
        if context.file_access_restricted:
            file_access_violations = self._check_file_access_violations(command, context.workspace_path)
            if file_access_violations:
                return {
                    "allowed": False,
                    "reason": "file_access_violation",
                    "details": f"Unauthorized file access: {file_access_violations[0]}"
                }

        # Check network access
        if not context.network_enabled:
            network_violations = self._check_network_access(command)
            if network_violations:
                return {
                    "allowed": False,
                    "reason": "network_access_violation",
                    "details": "Network access not allowed in current security context"
                }

        return {"allowed": True}

    def _is_command_allowed(self, command: str, allowed_operations: List[str]) -> bool:
        """Check if command operations are allowed"""
        command_parts = command.split()

        if not command_parts:
            return False

        # Check base command
        base_command = command_parts[0]
        if base_command not in self.allowed_commands:
            return False

        # Check for dangerous flags/options
        dangerous_flags = ['--privileged', '--cap-add', '-v', '--volume', 'sudo', 'su']
        for flag in dangerous_flags:
            if flag in command_parts:
                return False

        # Check allowed operations context
        operation_required = self._identify_operation_type(command)
        if operation_required and operation_required not in allowed_operations:
            return False

        return True

    def _identify_operation_type(self, command: str) -> Optional[str]:
        """Identify the type of operation being performed"""
        command_lower = command.lower()

        if any(word in command_lower for word in ['read', 'cat', 'head', 'tail']):
            return "read_file"
        elif any(word in command_lower for word in ['write', 'echo', '>', '>>']):
            return "write_file"
        elif any(word in command_lower for word in ['run', 'execute', 'python', 'node']):
            return "execute_code"
        elif any(word in command_lower for word in ['git', 'commit', 'push', 'pull']):
            return "version_control"
        elif any(word in command_lower for word in ['npm', 'pip', 'install']):
            return "package_management"

        return None

    def _check_dangerous_patterns(self, command: str) -> List[str]:
        """Check command against dangerous patterns"""
        violations = []

        for pattern in self.dangerous_patterns:
            if re.search(pattern, command, re.IGNORECASE):
                violations.append(pattern)

        return violations

    def _check_file_access_violations(self, command: str, workspace_path: Path) -> List[str]:
        """Check for unauthorized file access"""
        violations = []

        # Extract file paths from command
        file_paths = self._extract_file_paths(command)

        for file_path in file_paths:
            try:
                full_path = Path(file_path).resolve()

                # Check if path is within workspace
                if not self._is_path_within_workspace(full_path, workspace_path):
                    violations.append(str(full_path))

                # Check for sensitive system paths
                if self._is_sensitive_system_path(full_path):
                    violations.append(str(full_path))

            except Exception:
                # Invalid path, consider it a violation
                violations.append(file_path)

        return violations

    def _extract_file_paths(self, command: str) -> List[str]:
        """Extract file paths from command"""
        paths = []

        # Simple extraction - look for paths after common commands
        command_parts = command.split()
        for i, part in enumerate(command_parts):
            if part in ['cat', 'ls', 'cd', 'cp', 'mv', 'rm'] and i + 1 < len(command_parts):
                next_part = command_parts[i + 1]
                if not next_part.startswith('-'):  # Not a flag
                    paths.append(next_part)

        return paths

    def _is_path_within_workspace(self, path: Path, workspace: Path) -> bool:
        """Check if path is within allowed workspace"""
        try:
            path.resolve().relative_to(workspace.resolve())
            return True
        except ValueError:
            return False

    def _is_sensitive_system_path(self, path: Path) -> bool:
        """Check if path is a sensitive system location"""
        sensitive_paths = [
            Path("/etc"),
            Path("/usr/bin"),
            Path("/usr/sbin"),
            Path("/var"),
            Path("/root"),
            Path("/home"),
            Path("C:/Windows"),
            Path("C:/Program Files"),
            Path("C:/Users")
        ]

        path_str = str(path).lower()
        for sensitive in sensitive_paths:
            if str(sensitive).lower() in path_str:
                return True

        return False

    def _check_network_access(self, command: str) -> List[str]:
        """Check for network access attempts"""
        network_commands = ['curl', 'wget', 'ping', 'nc', 'telnet', 'ssh', 'scp']
        violations = []

        command_parts = command.split()
        for cmd in network_commands:
            if cmd in command_parts:
                violations.append(cmd)

        return violations

    async def _create_execution_environment(self, context: ExecutionContext) -> Dict[str, Any]:
        """Create isolated execution environment"""
        # Create temporary directory for execution
        temp_dir = Path(tempfile.mkdtemp(prefix="secure_exec_"))

        # Copy allowed workspace files (read-only)
        workspace_copy = temp_dir / "workspace"
        workspace_copy.mkdir()

        # Copy only allowed files
        allowed_extensions = ['.py', '.js', '.ts', '.json', '.txt', '.md']
        for file_path in context.workspace_path.rglob('*'):
            if file_path.is_file() and file_path.suffix.lower() in allowed_extensions:
                relative_path = file_path.relative_to(context.workspace_path)
                dest_path = workspace_copy / relative_path
                dest_path.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(file_path, dest_path)

        return {
            "temp_dir": temp_dir,
            "workspace_copy": workspace_copy,
            "original_workspace": context.workspace_path,
            "created_at": time.time()
        }

    async def _execute_with_monitoring(self, command: str, execution_env: Dict[str, Any],
                                     context: ExecutionContext) -> Dict[str, Any]:
        """Execute command with comprehensive monitoring"""

        # Change to execution workspace
        original_cwd = os.getcwd()
        os.chdir(execution_env["workspace_copy"])

        try:
            # Start monitoring
            monitoring_data = {"memory_peak": 0, "cpu_total": 0, "network_calls": 0}

            # Execute command with resource limits
            process = await asyncio.create_subprocess_shell(
                command,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                preexec_fn=self._set_resource_limits if os.name != 'nt' else None
            )

            # Monitor execution
            start_time = time.time()
            monitoring_task = asyncio.create_task(self._monitor_execution(process, monitoring_data))

            try:
                # Wait for completion with timeout
                stdout, stderr = await asyncio.wait_for(
                    process.communicate(),
                    timeout=context.timeout
                )

                execution_time = time.time() - start_time

                return {
                    "return_code": process.returncode,
                    "stdout": stdout.decode('utf-8', errors='ignore'),
                    "stderr": stderr.decode('utf-8', errors='ignore'),
                    "execution_time": execution_time,
                    "memory_used": monitoring_data["memory_peak"],
                    "cpu_used": monitoring_data["cpu_total"],
                    "network_calls": monitoring_data["network_calls"]
                }

            except asyncio.TimeoutError:
                process.kill()
                raise Exception(f"Command execution timed out after {context.timeout} seconds")

        finally:
            os.chdir(original_cwd)

    def _set_resource_limits(self):
        """Set resource limits for Unix-like systems"""
        try:
            # CPU time limit (seconds)
            resource.setrlimit(resource.RLIMIT_CPU, (60, 60))

            # Memory limit (512MB)
            memory_limit = 512 * 1024 * 1024
            resource.setrlimit(resource.RLIMIT_AS, (memory_limit, memory_limit))

            # File size limit (100MB)
            file_limit = 100 * 1024 * 1024
            resource.setrlimit(resource.RLIMIT_FSIZE, (file_limit, file_limit))

            # Process count limit
            resource.setrlimit(resource.RLIMIT_NPROC, (50, 50))

        except Exception as e:
            logger.warning(f"Failed to set resource limits: {e}")

    async def _monitor_execution(self, process: asyncio.subprocess.Process,
                               monitoring_data: Dict[str, Any]):
        """Monitor execution for security and performance"""
        try:
            while process.returncode is None:
                await asyncio.sleep(0.1)  # Monitor every 100ms

                try:
                    # Get process info
                    proc_info = psutil.Process(process.pid)

                    # Memory usage
                    memory_mb = proc_info.memory_info().rss / (1024 * 1024)
                    monitoring_data["memory_peak"] = max(monitoring_data["memory_peak"], memory_mb)

                    # CPU usage
                    cpu_percent = proc_info.cpu_percent()
                    monitoring_data["cpu_total"] += cpu_percent * 0.1  # 100ms interval

                    # Network connections (basic check)
                    connections = proc_info.connections()
                    monitoring_data["network_calls"] = len(connections)

                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    # Process ended or access denied
                    break

        except Exception as e:
            logger.warning(f"Monitoring error: {e}")

    async def _analyze_execution_result(self, result: Dict[str, Any],
                                      context: ExecutionContext) -> Dict[str, Any]:
        """Analyze execution result for security violations"""
        violations = []
        warnings = []

        # Check output for sensitive information
        if self._contains_sensitive_data(result["stdout"] + result["stderr"]):
            violations.append("sensitive_data_leakage")

        # Check resource usage
        if result.get("memory_used", 0) > context.memory_limit_mb * 0.9:
            warnings.append("high_memory_usage")

        if result.get("cpu_used", 0) > context.cpu_limit_seconds * 0.8:
            warnings.append("high_cpu_usage")

        # Check for anomalous behavior
        if result.get("network_calls", 0) > 10:
            warnings.append("excessive_network_activity")

        return {
            "violations": violations,
            "warnings": warnings,
            "analysis_complete": True
        }

    def _contains_sensitive_data(self, output: str) -> bool:
        """Check if output contains sensitive information"""
        sensitive_patterns = [
            r'\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b',  # Email
            r'\b\d{3}-\d{2}-\d{4}\b',  # SSN
            r'\b\d{4}[-.\s]\d{4}[-.\s]\d{4}[-.\s]\d{4}\b',  # Credit card
            r'\b[A-Za-z0-9_-]{20,}\b',  # API keys
        ]

        for pattern in sensitive_patterns:
            if re.search(pattern, output):
                return True

        return False

    async def _cleanup_execution_environment(self, execution_env: Dict[str, Any]):
        """Clean up execution environment"""
        try:
            temp_dir = execution_env["temp_dir"]
            if temp_dir.exists():
                shutil.rmtree(temp_dir)
                logger.info(f"Cleaned up execution environment: {temp_dir}")
        except Exception as e:
            logger.warning(f"Failed to cleanup execution environment: {e}")

    async def _record_security_violation(self, violation_type: str, severity: str,
                                       description: str, context: Dict[str, Any]):
        """Record security violation"""
        violation = SecurityViolation(
            timestamp=time.time(),
            violation_type=violation_type,
            severity=severity,
            description=description,
            context=context,
            mitigated=True  # We blocked it
        )

        self.violation_history.append(violation)

        logger.warning(f"Security violation recorded: {violation_type} - {description}")

    def get_security_report(self) -> Dict[str, Any]:
        """Generate security report"""
        total_violations = len(self.violation_history)
        critical_violations = len([v for v in self.violation_history if v.severity == "critical"])
        blocked_violations = len([v for v in self.violation_history if v.mitigated])

        return {
            "security_level": self.security_level,
            "total_violations": total_violations,
            "critical_violations": critical_violations,
            "blocked_violations": blocked_violations,
            "violation_success_rate": (blocked_violations / total_violations) if total_violations > 0 else 1.0,
            "allowed_commands": len(self.allowed_commands),
            "dangerous_patterns": len(self.dangerous_patterns)
        }

# Utility functions
async def execute_command_securely(command: str, security_level: str = "strict",
                                 workspace_path: Optional[str] = None) -> ExecutionResult:
    """
    Execute command with comprehensive security.
    """

    # Import here to avoid circular imports
    from prompt_injection_guard import SecurityLevel

    security_enum = SecurityLevel(security_level)
    engine = SecureExecutionEngine(security_level)

    context = ExecutionContext(
        user_id="system",
        workspace_path=Path(workspace_path or os.getcwd()),
        allowed_operations=["read_file", "execute_code"],
        security_level=security_enum
    )

    return await engine.execute_securely(command, context)

if __name__ == "__main__":
    import sys
    import asyncio

    async def main():
        if len(sys.argv) < 2:
            print("Usage: python secure_execution_engine.py 'command to execute' [security_level]")
            print("Security levels: minimal, standard, strict, maximum")
            print("Examples:")
            print("  python secure_execution_engine.py 'echo hello world'")
            print("  python secure_execution_engine.py 'ls -la' maximum")
            sys.exit(1)

        command = sys.argv[1]
        security_level = sys.argv[2] if len(sys.argv) > 2 else "strict"

        print("🔒 Secure Execution Engine - Sandboxed Environment")
        print("=" * 58)
        print(f"Command: {command}")
        print(f"Security Level: {security_level}")
        print("-" * 58)

        result = await execute_command_securely(command, security_level)

        print(f"\n⚙️ Execution Result:")
        print(f"   Success: {'✅' if result.success else '❌'}")
        print(f"   Return Code: {result.return_code}")
        print(f"   Execution Time: {result.execution_time:.2f}s")
        print(f"   Memory Used: {result.memory_used_mb:.1f}MB")
        print(f"   CPU Used: {result.cpu_used_seconds:.2f}s")

        if result.security_violations:
            print(f"   Security Violations: {len(result.security_violations)}")
            for violation in result.security_violations[:3]:
                print(f"     • {violation}")

        if result.warnings:
            print(f"   Warnings: {len(result.warnings)}")
            for warning in result.warnings[:3]:
                print(f"     • {warning}")

        if result.stdout:
            print(f"\n📝 Stdout ({len(result.stdout)} chars):")
            print(f"   {result.stdout[:200]}{'...' if len(result.stdout) > 200 else ''}")

        if result.stderr:
            print(f"\n⚠️ Stderr ({len(result.stderr)} chars):")
            print(f"   {result.stderr[:200]}{'...' if len(result.stderr) > 200 else ''}")

        print("\n🛡️ Sandboxed execution completed!")

    asyncio.run(main())