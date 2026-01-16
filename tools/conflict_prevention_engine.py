#!/usr/bin/env python3
"""
Conflict Prevention Engine - AI-Powered Merge Conflict Prediction & Prevention
マルチエージェントでのターミナル起動と差分ベースのコンフリクト予防システム
"""

import os
import sys
import json
import asyncio
import subprocess
import tempfile
import threading
import time
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple, Set
from dataclasses import dataclass, asdict
from enum import Enum
import logging
from datetime import datetime
import hashlib
import re

# External dependencies
try:
    import openai
    import git
    import aiofiles
    from git import Repo
except ImportError:
    print("Installing required dependencies...")
    subprocess.run([sys.executable, "-m", "pip", "install", "openai", "GitPython", "aiofiles"], check=True)
    import openai
    import git
    import aiofiles
    from git import Repo

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class ConflictRisk(Enum):
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

class ConflictType(Enum):
    TEXTUAL = "textual"  # Line-based conflicts
    STRUCTURAL = "structural"  # AST-level conflicts
    SEMANTIC = "semantic"  # Logic/behavior conflicts
    DEPENDENCY = "dependency"  # Import/dependency conflicts

@dataclass
class ConflictPrediction:
    file_path: str
    conflict_type: ConflictType
    risk_level: ConflictRisk
    confidence: float
    predicted_lines: List[int]
    reason: str
    suggested_resolution: str
    ai_analysis: Dict[str, Any]

@dataclass
class AgentTerminal:
    agent_id: str
    terminal_id: str
    working_directory: Path
    process: Optional[subprocess.Popen]
    status: str
    created_at: datetime
    last_activity: datetime

@dataclass
class MergeAnalysis:
    source_branch: str
    target_branch: str
    changed_files: List[str]
    conflict_predictions: List[ConflictPrediction]
    risk_assessment: Dict[str, Any]
    ai_recommendations: List[str]
    agent_terminals: List[AgentTerminal]

class ConflictPreventionEngine:
    """AI-Powered Conflict Prevention & Terminal Management for Multi-Agent Systems"""

    def __init__(self, repo_path: str, openai_api_key: Optional[str] = None):
        self.repo_path = Path(repo_path)
        self.repo = Repo(repo_path)
        self.openai_api_key = openai_api_key or os.getenv("OPENAI_API_KEY")

        if self.openai_api_key:
            openai.api_key = self.openai_api_key

        self.agent_terminals: Dict[str, AgentTerminal] = {}
        self.active_processes: Set[subprocess.Popen] = set()

        # Conflict patterns for different file types
        self.conflict_patterns = self._load_conflict_patterns()

    def _load_conflict_patterns(self) -> Dict[str, List[str]]:
        """Load conflict patterns for different file types"""
        return {
            ".rs": [
                r"use\s+crate::",  # Rust module imports
                r"impl\s+\w+",     # Trait implementations
                r"fn\s+\w+",       # Function definitions
                r"struct\s+\w+",   # Struct definitions
                r"enum\s+\w+",     # Enum definitions
            ],
            ".py": [
                r"class\s+\w+",    # Python classes
                r"def\s+\w+",      # Function definitions
                r"import\s+",      # Import statements
                r"from\s+\w+",     # From imports
            ],
            ".js": [
                r"function\s+\w+", # Function declarations
                r"class\s+\w+",    # Class definitions
                r"import\s+",      # Import statements
                r"export\s+",      # Export statements
            ],
            ".ts": [
                r"interface\s+\w+", # TypeScript interfaces
                r"type\s+\w+",      # Type definitions
                r"class\s+\w+",     # Class definitions
            ],
            ".json": [
                r'"[^"]+"\s*:',    # JSON keys
            ],
            ".toml": [
                r"^\s*\[",         # TOML sections
                r"^\s*\w+\s*=",    # TOML keys
            ],
            ".md": [
                r"^#+\s+",         # Markdown headers
                r"^\s*-\s+",       # List items
                r"^\s*\d+\.\s+",   # Numbered lists
            ]
        }

    async def analyze_merge_conflicts(self, source_branch: str, target_branch: str) -> MergeAnalysis:
        """Analyze potential merge conflicts using AI and diff analysis"""
        logger.info(f"Analyzing merge conflicts: {source_branch} -> {target_branch}")

        # Get changed files
        changed_files = self._get_changed_files(source_branch, target_branch)

        # Analyze each file for potential conflicts
        conflict_predictions = []
        for file_path in changed_files:
            if self._should_analyze_file(file_path):
                predictions = await self._analyze_file_conflicts(file_path, source_branch, target_branch)
                conflict_predictions.extend(predictions)

        # Risk assessment
        risk_assessment = self._assess_merge_risk(conflict_predictions, changed_files)

        # AI recommendations
        ai_recommendations = await self._generate_ai_recommendations(conflict_predictions, risk_assessment)

        return MergeAnalysis(
            source_branch=source_branch,
            target_branch=target_branch,
            changed_files=changed_files,
            conflict_predictions=conflict_predictions,
            risk_assessment=risk_assessment,
            ai_recommendations=ai_recommendations,
            agent_terminals=[]
        )

    def _get_changed_files(self, source_branch: str, target_branch: str) -> List[str]:
        """Get list of files changed between branches"""
        try:
            # Get merge base
            merge_base = self.repo.git.merge_base(source_branch, target_branch)

            # Get changed files
            diff = self.repo.git.diff(merge_base, source_branch, name_only=True)
            return diff.split('\n') if diff else []

        except Exception as e:
            logger.error(f"Failed to get changed files: {e}")
            return []

    def _should_analyze_file(self, file_path: str) -> bool:
        """Determine if file should be analyzed for conflicts"""
        # Skip binary files, lock files, and generated files
        skip_patterns = [
            '.lock', '.exe', '.dll', '.so', '.dylib',  # Binaries
            'node_modules/', 'target/', '__pycache__/',  # Dependencies
            '.git/', 'dist/', 'build/',  # Generated
            '*.log', '*.tmp', '*.cache'  # Temporary
        ]

        for pattern in skip_patterns:
            if pattern in file_path:
                return False

        return True

    async def _analyze_file_conflicts(self, file_path: str, source_branch: str, target_branch: str) -> List[ConflictPrediction]:
        """Analyze a single file for potential conflicts"""
        predictions = []

        try:
            # Get file contents from both branches
            source_content = self._get_file_content(file_path, source_branch)
            target_content = self._get_file_content(file_path, target_branch)
            base_content = self._get_file_content(file_path, self.repo.git.merge_base(source_branch, target_branch))

            if not source_content or not target_content:
                return predictions

            # Basic diff analysis
            basic_predictions = self._analyze_basic_conflicts(file_path, source_content, target_content, base_content)
            predictions.extend(basic_predictions)

            # AI-powered analysis
            if self.openai_api_key:
                ai_predictions = await self._analyze_with_ai(file_path, source_content, target_content, base_content)
                predictions.extend(ai_predictions)

        except Exception as e:
            logger.error(f"Failed to analyze file {file_path}: {e}")

        return predictions

    def _get_file_content(self, file_path: str, ref: str) -> Optional[str]:
        """Get file content at specific git reference"""
        try:
            return self.repo.git.show(f"{ref}:{file_path}")
        except:
            return None

    def _analyze_basic_conflicts(self, file_path: str, source: str, target: str, base: str) -> List[ConflictPrediction]:
        """Basic conflict analysis using diff and pattern matching"""
        predictions = []

        # Get file extension and patterns
        ext = Path(file_path).suffix
        patterns = self.conflict_patterns.get(ext, [])

        source_lines = source.split('\n')
        target_lines = target.split('\n')
        base_lines = base.split('\n') if base else []

        # Analyze overlapping changes
        source_changes = self._find_changed_lines(base_lines, source_lines)
        target_changes = self._find_changed_lines(base_lines, target_lines)

        overlapping_lines = source_changes.intersection(target_changes)

        if overlapping_lines:
            # Check for pattern conflicts
            for line_num in overlapping_lines:
                if line_num < len(source_lines) and line_num < len(target_lines):
                    source_line = source_lines[line_num]
                    target_line = target_lines[line_num]

                    # Check if both branches changed the same structural element
                    for pattern in patterns:
                        if re.search(pattern, source_line) and re.search(pattern, target_line):
                            predictions.append(ConflictPrediction(
                                file_path=file_path,
                                conflict_type=ConflictType.STRUCTURAL,
                                risk_level=ConflictRisk.HIGH,
                                confidence=0.8,
                                predicted_lines=[line_num],
                                reason=f"Both branches modified structural element matching pattern: {pattern}",
                                suggested_resolution="Review both implementations and choose appropriate merge strategy",
                                ai_analysis={}
                            ))
                            break

        return predictions

    def _find_changed_lines(self, base_lines: List[str], new_lines: List[str]) -> Set[int]:
        """Find line numbers that changed between versions"""
        changed_lines = set()

        # Simple diff analysis
        max_lines = max(len(base_lines), len(new_lines))

        for i in range(max_lines):
            base_line = base_lines[i] if i < len(base_lines) else ""
            new_line = new_lines[i] if i < len(new_lines) else ""

            if base_line != new_line:
                changed_lines.add(i)

        return changed_lines

    async def _analyze_with_ai(self, file_path: str, source: str, target: str, base: str) -> List[ConflictPrediction]:
        """AI-powered conflict analysis using OpenAI"""
        if not self.openai_api_key:
            return []

        try:
            # Prepare prompt for AI analysis
            ext = Path(file_path).suffix

            prompt = f"""
Analyze the following code changes for potential merge conflicts. Consider:
1. Structural conflicts (functions, classes, imports)
2. Semantic conflicts (logic changes that might conflict)
3. Dependency conflicts (imports, references)

File: {file_path}
Language: {ext}

Base version:
``` {ext[1:] if ext else 'text'}
{base[:2000]}
```

Source branch changes:
``` {ext[1:] if ext else 'text'}
{source[:2000]}
```

Target branch changes:
``` {ext[1:] if ext else 'text'}
{target[:2000]}
```

Provide analysis in JSON format:
{{
  "conflict_predictions": [
    {{
      "type": "textual|structural|semantic|dependency",
      "risk": "low|medium|high|critical",
      "confidence": 0.0-1.0,
      "lines": [line_numbers],
      "reason": "explanation",
      "resolution": "suggested fix"
    }}
  ]
}}
"""

            response = await openai.ChatCompletion.acreate(
                model="gpt-4-turbo-preview",
                messages=[{"role": "user", "content": prompt}],
                temperature=0.1,
                max_tokens=2000
            )

            analysis = json.loads(response.choices[0].message.content)

            predictions = []
            for pred in analysis.get("conflict_predictions", []):
                predictions.append(ConflictPrediction(
                    file_path=file_path,
                    conflict_type=ConflictType(pred["type"]),
                    risk_level=ConflictRisk(pred["risk"]),
                    confidence=float(pred["confidence"]),
                    predicted_lines=pred.get("lines", []),
                    reason=pred["reason"],
                    suggested_resolution=pred["resolution"],
                    ai_analysis=pred
                ))

            return predictions

        except Exception as e:
            logger.error(f"AI analysis failed for {file_path}: {e}")
            return []

    def _assess_merge_risk(self, predictions: List[ConflictPrediction], changed_files: List[str]) -> Dict[str, Any]:
        """Assess overall merge risk"""
        total_files = len(changed_files)
        high_risk_predictions = [p for p in predictions if p.risk_level in [ConflictRisk.HIGH, ConflictRisk.CRITICAL]]
        structural_conflicts = [p for p in predictions if p.conflict_type == ConflictType.STRUCTURAL]

        risk_score = len(high_risk_predictions) / max(total_files, 1)
        structural_risk = len(structural_conflicts) / max(total_files, 1)

        overall_risk = "low"
        if risk_score > 0.3 or structural_risk > 0.2:
            overall_risk = "high"
        elif risk_score > 0.1 or structural_risk > 0.1:
            overall_risk = "medium"

        return {
            "overall_risk": overall_risk,
            "risk_score": risk_score,
            "structural_risk": structural_risk,
            "total_predictions": len(predictions),
            "high_risk_predictions": len(high_risk_predictions),
            "files_analyzed": len(changed_files)
        }

    async def _generate_ai_recommendations(self, predictions: List[ConflictPrediction],
                                         risk_assessment: Dict[str, Any]) -> List[str]:
        """Generate AI-powered recommendations for merge strategy"""
        if not self.openai_api_key or not predictions:
            return ["Review all predicted conflicts manually"]

        try:
            prompt = f"""
Based on the following merge conflict analysis, provide specific recommendations:

Risk Assessment: {json.dumps(risk_assessment, indent=2)}

Conflict Predictions: {len(predictions)} conflicts found
{chr(10).join([f"- {p.file_path}: {p.conflict_type.value} ({p.risk_level.value}) - {p.reason}" for p in predictions[:10]])}

Provide 3-5 specific recommendations for handling this merge safely.
"""

            response = await openai.ChatCompletion.acreate(
                model="gpt-4-turbo-preview",
                messages=[{"role": "user", "content": prompt}],
                temperature=0.3,
                max_tokens=1000
            )

            recommendations = response.choices[0].message.content.strip().split('\n')
            return [rec.strip('- ').strip() for rec in recommendations if rec.strip()]

        except Exception as e:
            logger.error(f"AI recommendations failed: {e}")
            return ["Review all predicted conflicts manually"]

    def launch_agent_terminal(self, agent_id: str, working_dir: Optional[Path] = None) -> AgentTerminal:
        """Launch a terminal for multi-agent coordination"""
        terminal_id = f"{agent_id}_{int(time.time())}"
        working_directory = working_dir or self.repo_path

        # Create terminal info
        terminal = AgentTerminal(
            agent_id=agent_id,
            terminal_id=terminal_id,
            working_directory=working_directory,
            process=None,
            status="launching",
            created_at=datetime.now(),
            last_activity=datetime.now()
        )

        self.agent_terminals[terminal_id] = terminal

        # Launch terminal process (background)
        try:
            if os.name == 'nt':  # Windows
                process = subprocess.Popen(
                    ['cmd.exe', '/c', 'start', 'cmd.exe'],
                    cwd=str(working_directory),
                    creationflags=subprocess.CREATE_NEW_CONSOLE
                )
            else:  # Unix-like
                process = subprocess.Popen(
                    ['x-terminal-emulator', '-e', 'bash'],
                    cwd=str(working_directory)
                )

            terminal.process = process
            terminal.status = "active"
            self.active_processes.add(process)

            logger.info(f"Launched terminal {terminal_id} for agent {agent_id}")

        except Exception as e:
            logger.error(f"Failed to launch terminal for agent {agent_id}: {e}")
            terminal.status = "failed"

        return terminal

    def coordinate_agents(self, analysis: MergeAnalysis) -> Dict[str, Any]:
        """Coordinate multiple agents for conflict resolution"""
        coordination_plan = {
            "agents_needed": [],
            "terminal_assignments": {},
            "conflict_resolution_strategy": {},
            "communication_channels": []
        }

        # Determine which agents are needed based on conflict types
        agent_requirements = self._determine_agent_requirements(analysis.conflict_predictions)

        for agent_type, needed in agent_requirements.items():
            if needed:
                coordination_plan["agents_needed"].append(agent_type)

                # Launch terminal for agent
                terminal = self.launch_agent_terminal(agent_type)
                coordination_plan["terminal_assignments"][agent_type] = terminal.terminal_id

        # Create conflict resolution strategy
        coordination_plan["conflict_resolution_strategy"] = self._create_resolution_strategy(analysis)

        # Setup communication channels
        coordination_plan["communication_channels"] = self._setup_agent_communication()

        return coordination_plan

    def _determine_agent_requirements(self, predictions: List[ConflictPrediction]) -> Dict[str, bool]:
        """Determine which agents are needed for conflict resolution"""
        requirements = {
            "code-reviewer": False,
            "test-runner": False,
            "security-auditor": False,
            "performance-analyzer": False,
            "documentation-writer": False
        }

        for prediction in predictions:
            if prediction.conflict_type == ConflictType.SEMANTIC:
                requirements["code-reviewer"] = True
            elif prediction.conflict_type == ConflictType.DEPENDENCY:
                requirements["test-runner"] = True
            elif "security" in prediction.reason.lower():
                requirements["security-auditor"] = True
            elif "performance" in prediction.reason.lower():
                requirements["performance-analyzer"] = True

        return requirements

    def _create_resolution_strategy(self, analysis: MergeAnalysis) -> Dict[str, Any]:
        """Create conflict resolution strategy"""
        strategy = {
            "phases": [
                "conflict_prediction_review",
                "agent_coordination",
                "parallel_resolution",
                "integration_testing",
                "final_validation"
            ],
            "conflict_priorities": {},
            "agent_assignments": {},
            "fallback_strategies": []
        }

        # Prioritize conflicts by risk level
        high_risk = [p for p in analysis.conflict_predictions if p.risk_level == ConflictRisk.HIGH]
        critical_risk = [p for p in analysis.conflict_predictions if p.risk_level == ConflictRisk.CRITICAL]

        strategy["conflict_priorities"] = {
            "critical": len(critical_risk),
            "high": len(high_risk),
            "total": len(analysis.conflict_predictions)
        }

        return strategy

    def _setup_agent_communication(self) -> List[Dict[str, Any]]:
        """Setup communication channels between agents"""
        channels = []

        # Setup inter-agent communication
        for terminal_id, terminal in self.agent_terminals.items():
            channels.append({
                "terminal_id": terminal_id,
                "agent_id": terminal.agent_id,
                "status": terminal.status,
                "working_directory": str(terminal.working_directory)
            })

        return channels

    def cleanup_terminals(self):
        """Clean up all agent terminals"""
        for terminal in self.agent_terminals.values():
            if terminal.process and terminal.process.poll() is None:
                try:
                    terminal.process.terminate()
                    terminal.process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    terminal.process.kill()

        self.agent_terminals.clear()
        self.active_processes.clear()

async def main():
    """Main entry point"""
    if len(sys.argv) < 3:
        print("Usage: python conflict_prevention_engine.py <source_branch> <target_branch>")
        print("Or: python conflict_prevention_engine.py launch-terminal <agent_id>")
        sys.exit(1)

    engine = ConflictPreventionEngine(".")

    if sys.argv[1] == "launch-terminal":
        agent_id = sys.argv[2]
        terminal = engine.launch_agent_terminal(agent_id)
        print(f"Launched terminal: {terminal.terminal_id}")
    else:
        source_branch = sys.argv[1]
        target_branch = sys.argv[2]

        print(f"🔍 Analyzing merge conflicts: {source_branch} -> {target_branch}")

        analysis = await engine.analyze_merge_conflicts(source_branch, target_branch)

        print("
📊 Analysis Results:"        print(f"Files changed: {len(analysis.changed_files)}")
        print(f"Predicted conflicts: {len(analysis.conflict_predictions)}")
        print(f"Overall risk: {analysis.risk_assessment['overall_risk']}")

        if analysis.conflict_predictions:
            print("
⚠️  Top Conflict Predictions:"            for i, pred in enumerate(analysis.conflict_predictions[:5]):
                print(f"{i+1}. {pred.file_path}: {pred.conflict_type.value} ({pred.risk_level.value})")
                print(f"   {pred.reason}")

        if analysis.ai_recommendations:
            print("
🤖 AI Recommendations:"            for rec in analysis.ai_recommendations:
                print(f"• {rec}")

        # Coordinate agents if conflicts detected
        if analysis.conflict_predictions:
            print("
🎯 Coordinating agents for conflict resolution..."            coordination = engine.coordinate_agents(analysis)

            print(f"Agents needed: {', '.join(coordination['agents_needed'])}")
            print(f"Terminals launched: {len(coordination['terminal_assignments'])}")

        # Save analysis results
        results_file = Path("merge-conflict-analysis.json")
        with open(results_file, 'w', encoding='utf-8') as f:
            json.dump({
                "analysis": asdict(analysis),
                "coordination": coordination if 'coordination' in locals() else {},
                "timestamp": datetime.now().isoformat()
            }, f, indent=2, default=str)

        print(f"\n💾 Analysis saved to: {results_file}")

if __name__ == "__main__":
    asyncio.run(main())