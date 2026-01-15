#!/usr/bin/env python3
"""
Pre-merge QA Hook - Automatic Quality Assurance Before Git Merge
Blocks merges with critical QA issues and generates detailed review reports
"""

import os
import sys
import json
import subprocess
import tempfile
import smtplib
import requests
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from pathlib import Path
from typing import Dict, List, Any, Optional, Tuple
from dataclasses import dataclass
from datetime import datetime
import logging

# Import QA components
try:
    from qa_engineer.scripts.run_qa_engineer import QAAnalyzer, QAReport
except ImportError:
    # Fallback import
    sys.path.append(str(Path(__file__).parent / "codex-supervisor"))
    from run_qa_engineer import QAAnalyzer, QAReport

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class NotificationConfig:
    slack_webhook_url: Optional[str] = None
    discord_webhook_url: Optional[str] = None
    email_smtp_server: Optional[str] = None
    email_smtp_port: int = 587
    email_username: Optional[str] = None
    email_password: Optional[str] = None
    email_from: Optional[str] = None
    email_to: List[str] = None
    notify_on_success: bool = True
    notify_on_failure: bool = True
    notify_on_warning: bool = True

    def __post_init__(self):
        if self.email_to is None:
            self.email_to = []

@dataclass
class MergeQAConfig:
    block_on_critical: bool = True
    block_on_high: bool = False
    require_minimum_score: float = 7.0
    max_qa_time: int = 300  # 5 minutes
    generate_diff_report: bool = True
    notification_config: Optional[NotificationConfig] = None

    def __post_init__(self):
        if self.notification_config is None:
            self.notification_config = NotificationConfig()

@dataclass
class MergeContext:
    source_branch: str
    target_branch: str
    merge_commit: str
    author: str
    changed_files: List[Path]
    diff_stats: Dict[str, int]

class PreMergeQAHook:
    """Git pre-merge hook that runs comprehensive QA analysis"""

    def __init__(self, config: MergeQAConfig):
        self.config = config
        self.project_root = Path.cwd()

    def run_pre_merge_qa(self, source_branch: str, target_branch: str) -> Tuple[bool, Dict[str, Any]]:
        """
        Run pre-merge QA analysis

        Returns:
            (allow_merge: bool, results: dict)
        """

        logger.info(f"Starting pre-merge QA for {source_branch} → {target_branch}")

        # Gather merge context
        merge_context = self._gather_merge_context(source_branch, target_branch)

        # Run QA analysis on the merge
        qa_report = self._run_merge_qa_analysis(merge_context)

        # Evaluate merge criteria
        merge_allowed, evaluation = self._evaluate_merge_criteria(qa_report, merge_context)

        # Generate reports
        self._generate_merge_reports(qa_report, merge_context, evaluation)

        # Send notifications if configured
        if not merge_allowed or self.config.notify_channels:
            self._send_notifications(qa_report, merge_context, evaluation)

        results = {
            "merge_allowed": merge_allowed,
            "qa_report": qa_report,
            "merge_context": merge_context,
            "evaluation": evaluation,
            "timestamp": datetime.now().isoformat()
        }

        logger.info(f"Pre-merge QA completed. Merge {'ALLOWED' if merge_allowed else 'BLOCKED'}")

        return merge_allowed, results

    def _gather_merge_context(self, source_branch: str, target_branch: str) -> MergeContext:
        """Gather information about the merge operation"""

        # Get merge commit info
        try:
            result = subprocess.run(
                ["git", "rev-parse", "HEAD"],
                capture_output=True, text=True, cwd=self.project_root
            )
            merge_commit = result.stdout.strip()
        except Exception:
            merge_commit = "unknown"

        # Get author info
        try:
            result = subprocess.run(
                ["git", "log", "-1", "--format=%an <%ae>", merge_commit],
                capture_output=True, text=True, cwd=self.project_root
            )
            author = result.stdout.strip()
        except Exception:
            author = "unknown"

        # Get changed files
        try:
            result = subprocess.run(
                ["git", "diff", "--name-only", f"{target_branch}..{source_branch}"],
                capture_output=True, text=True, cwd=self.project_root
            )
            changed_files = [Path(f) for f in result.stdout.strip().split('\n') if f]
        except Exception:
            changed_files = []

        # Get diff stats
        try:
            result = subprocess.run(
                ["git", "diff", "--stat", f"{target_branch}..{source_branch}"],
                capture_output=True, text=True, cwd=self.project_root
            )
            # Parse diff stat (simplified)
            diff_stats = self._parse_diff_stats(result.stdout)
        except Exception:
            diff_stats = {"files_changed": len(changed_files), "insertions": 0, "deletions": 0}

        return MergeContext(
            source_branch=source_branch,
            target_branch=target_branch,
            merge_commit=merge_commit,
            author=author,
            changed_files=changed_files,
            diff_stats=diff_stats
        )

    def _parse_diff_stats(self, diff_output: str) -> Dict[str, int]:
        """Parse git diff --stat output"""
        stats = {"files_changed": 0, "insertions": 0, "deletions": 0}

        lines = diff_output.strip().split('\n')
        if len(lines) >= 2:
            # Last line contains summary
            summary_line = lines[-1]
            # Parse "X files changed, Y insertions(+), Z deletions(-)"
            import re
            match = re.search(r'(\d+) files? changed(?:, (\d+) insertions?\(\+\))?(?:, (\d+) deletions?\(-\))?', summary_line)
            if match:
                stats["files_changed"] = int(match.group(1) or 0)
                stats["insertions"] = int(match.group(2) or 0)
                stats["deletions"] = int(match.group(3) or 0)

        return stats

    def _run_merge_qa_analysis(self, merge_context: MergeContext) -> QAReport:
        """Run QA analysis on the merge changes"""

        logger.info("Running QA analysis on merge changes...")

        # Create temporary directory with merge changes for analysis
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_path = Path(temp_dir)

            # Copy project structure
            self._copy_project_for_analysis(self.project_root, temp_path, merge_context.changed_files)

            # Run QA analysis
            analyzer = QAAnalyzer(temp_path)
            report = analyzer.generate_report()

            logger.info(f"QA analysis completed: {len(report.issues)} issues found")

            return report

    def _copy_project_for_analysis(self, source: Path, dest: Path, changed_files: List[Path]):
        """Copy project files for QA analysis, focusing on changed files"""

        # Copy essential project structure
        essential_files = [
            "Cargo.toml", "package.json", "pyproject.toml", "requirements.txt",
            "setup.py", "Makefile", "CMakeLists.txt", ".codex/"
        ]

        for essential in essential_files:
            src_path = source / essential
            if src_path.exists():
                dest_path = dest / essential
                if src_path.is_file():
                    dest_path.parent.mkdir(parents=True, exist_ok=True)
                    import shutil
                    shutil.copy2(src_path, dest_path)
                else:
                    import shutil
                    shutil.copytree(src_path, dest_path, dirs_exist_ok=True)

        # Copy changed files
        for changed_file in changed_files:
            src_path = source / changed_file
            dest_path = dest / changed_file

            if src_path.exists():
                dest_path.parent.mkdir(parents=True, exist_ok=True)
                import shutil
                shutil.copy2(src_path, dest_path)

    def _evaluate_merge_criteria(self, qa_report: QAReport, merge_context: MergeContext) -> Dict[str, Any]:
        """Evaluate whether the merge should be allowed based on QA results"""

        evaluation = {
            "block_reasons": [],
            "warnings": [],
            "recommendations": [],
            "risk_level": "low",
            "merge_confidence": 1.0
        }

        # Check critical issues
        critical_issues = [i for i in qa_report.issues if i.severity.name == "CRITICAL"]
        high_issues = [i for i in qa_report.issues if i.severity.name == "HIGH"]

        if self.config.block_on_critical and critical_issues:
            evaluation["block_reasons"].append(
                f"Critical QA issues found: {len(critical_issues)} issues must be resolved"
            )
            evaluation["risk_level"] = "critical"
            evaluation["merge_confidence"] = 0.0

        elif self.config.block_on_high and high_issues:
            evaluation["block_reasons"].append(
                f"High-priority QA issues found: {len(high_issues)} issues should be addressed"
            )
            evaluation["risk_level"] = "high"
            evaluation["merge_confidence"] = 0.3

        # Check quality scores
        quality_scores = [
            qa_report.metrics.algorithmic_complexity,
            qa_report.metrics.quantum_optimization,
            qa_report.metrics.software_engineering,
            qa_report.metrics.code_quality,
            qa_report.metrics.performance,
            qa_report.metrics.security
        ]

        # Convert letter grades to numeric scores
        grade_to_score = {
            "A+": 10, "A": 9, "A-": 8, "B+": 7, "B": 6, "B-": 5,
            "C+": 4, "C": 3, "C-": 2, "D": 1, "N/A": 5
        }

        numeric_scores = []
        for grade in quality_scores:
            if isinstance(grade, str) and grade in grade_to_score:
                numeric_scores.append(grade_to_score[grade])
            else:
                numeric_scores.append(5)  # Default

        avg_score = sum(numeric_scores) / len(numeric_scores) if numeric_scores else 0

        if avg_score < self.config.require_minimum_score:
            evaluation["block_reasons"].append(
                ".2f"
            )
            evaluation["merge_confidence"] = min(evaluation["merge_confidence"], avg_score / 10)

        # Check diff size for risk assessment
        if merge_context.diff_stats["files_changed"] > 50:
            evaluation["warnings"].append("Large merge: >50 files changed - increased risk")
            evaluation["merge_confidence"] *= 0.8

        if merge_context.diff_stats["insertions"] + merge_context.diff_stats["deletions"] > 1000:
            evaluation["warnings"].append("Large code changes: >1000 lines - thorough review recommended")
            evaluation["merge_confidence"] *= 0.9

        # Generate recommendations
        if critical_issues:
            evaluation["recommendations"].append("Address all critical QA issues before merging")
        if high_issues:
            evaluation["recommendations"].append("Consider addressing high-priority issues")
        if avg_score < 8.0:
            evaluation["recommendations"].append("Improve code quality scores before merging")

        return evaluation

    def _generate_merge_reports(self, qa_report: QAReport, merge_context: MergeContext, evaluation: Dict[str, Any]):
        """Generate comprehensive merge QA reports"""

        # Create reports directory
        reports_dir = self.project_root / "merge-qa-reports"
        reports_dir.mkdir(exist_ok=True)

        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")

        # Generate JSON report
        json_report = {
            "merge_info": {
                "source_branch": merge_context.source_branch,
                "target_branch": merge_context.target_branch,
                "merge_commit": merge_context.merge_commit,
                "author": merge_context.author,
                "timestamp": datetime.now().isoformat()
            },
            "diff_stats": merge_context.diff_stats,
            "qa_results": {
                "quality_scores": {
                    "algorithmic_complexity": qa_report.metrics.algorithmic_complexity,
                    "quantum_optimization": qa_report.metrics.quantum_optimization,
                    "software_engineering": qa_report.metrics.software_engineering,
                    "code_quality": qa_report.metrics.code_quality,
                    "performance": qa_report.metrics.performance,
                    "security": qa_report.metrics.security
                },
                "issues_summary": {
                    "total": len(qa_report.issues),
                    "critical": len([i for i in qa_report.issues if i.severity.name == "CRITICAL"]),
                    "high": len([i for i in qa_report.issues if i.severity.name == "HIGH"]),
                    "medium": len([i for i in qa_report.issues if i.severity.name == "MEDIUM"]),
                    "low": len([i for i in qa_report.issues if i.severity.name == "LOW"])
                },
                "integration_status": qa_report.integration_status
            },
            "evaluation": evaluation,
            "changed_files": [str(f) for f in merge_context.changed_files]
        }

        json_path = reports_dir / f"merge_qa_report_{timestamp}.json"
        with open(json_path, 'w', encoding='utf-8') as f:
            json.dump(json_report, f, indent=2, ensure_ascii=False)

        # Generate human-readable markdown report
        md_report = self._generate_markdown_report(qa_report, merge_context, evaluation, timestamp)
        md_path = reports_dir / f"merge_qa_report_{timestamp}.md"
        with open(md_path, 'w', encoding='utf-8') as f:
            f.write(md_report)

        logger.info(f"Merge QA reports generated: {json_path}, {md_path}")

    def _generate_markdown_report(self, qa_report: QAReport, merge_context: MergeContext,
                                evaluation: Dict[str, Any], timestamp: str) -> str:
        """Generate human-readable markdown report"""

        report = f"""# Pre-Merge QA Review Report

**Generated:** {datetime.now().strftime("%Y-%m-%d %H:%M:%S")}
**Report ID:** {timestamp}

## Merge Information

- **Source Branch:** {merge_context.source_branch}
- **Target Branch:** {merge_context.target_branch}
- **Merge Commit:** {merge_context.merge_commit[:8]}
- **Author:** {merge_context.author}

## Diff Statistics

- **Files Changed:** {merge_context.diff_stats["files_changed"]}
- **Insertions:** {merge_context.diff_stats["insertions"]}
- **Deletions:** {merge_context.diff_stats["deletions"]}

## Quality Assessment

### Quality Scores
| Category | Score |
|----------|-------|
| Algorithmic Complexity | {qa_report.metrics.algorithmic_complexity} |
| Quantum Optimization | {qa_report.metrics.quantum_optimization} |
| Software Engineering | {qa_report.metrics.software_engineering} |
| Code Quality | {qa_report.metrics.code_quality} |
| Performance | {qa_report.metrics.performance} |
| Security | {qa_report.metrics.security} |

### Issues Summary
- **Total Issues:** {len(qa_report.issues)}
- **Critical:** {len([i for i in qa_report.issues if i.severity.name == "CRITICAL"])}
- **High:** {len([i for i in qa_report.issues if i.severity.name == "HIGH"])}
- **Medium:** {len([i for i in qa_report.issues if i.severity.name == "MEDIUM"])}
- **Low:** {len([i for i in qa_report.issues if i.severity.name == "LOW"])}

## Merge Evaluation

### Risk Level: {evaluation["risk_level"].upper()}
### Merge Confidence: {evaluation["merge_confidence"]:.1%}

"""

        if evaluation["block_reasons"]:
            report += "### 🚫 Block Reasons\n"
            for reason in evaluation["block_reasons"]:
                report += f"- {reason}\n"
            report += "\n"

        if evaluation["warnings"]:
            report += "### ⚠️ Warnings\n"
            for warning in evaluation["warnings"]:
                report += f"- {warning}\n"
            report += "\n"

        if evaluation["recommendations"]:
            report += "### 💡 Recommendations\n"
            for rec in evaluation["recommendations"]:
                report += f"- {rec}\n"
            report += "\n"

        # Add critical issues if any
        critical_issues = [i for i in qa_report.issues if i.severity.name == "CRITICAL"]
        if critical_issues:
            report += "## Critical Issues\n\n"
            for issue in critical_issues[:10]:  # Show first 10
                report += f"### {issue.title}\n"
                report += f"- **Severity:** {issue.severity.name}\n"
                report += f"- **Location:** {issue.location}\n"
                report += f"- **Impact Score:** {issue.impact_score}\n"
                report += f"- **Description:** {issue.description}\n"
                report += f"- **Recommendation:** {issue.recommendation}\n\n"

        # Add changed files
        if merge_context.changed_files:
            report += "## Changed Files\n\n"
            for file in merge_context.changed_files[:20]:  # Show first 20
                report += f"- {file}\n"
            if len(merge_context.changed_files) > 20:
                report += f"- ... and {len(merge_context.changed_files) - 20} more files\n"

        return report

    def _send_notifications(self, qa_report: QAReport, merge_context: MergeContext, evaluation: Dict[str, Any]):
        """Send notifications about merge QA results"""

        config = self.config.notification_config

        # Determine if we should send notification
        should_notify = False
        if evaluation["block_reasons"] and config.notify_on_failure:
            should_notify = True
        elif evaluation["warnings"] and config.notify_on_warning:
            should_notify = True
        elif config.notify_on_success:
            should_notify = True

        if not should_notify:
            return

        # Send notifications to configured channels
        if config.slack_webhook_url:
            try:
                self._send_slack_notification(qa_report, merge_context, evaluation)
            except Exception as e:
                logger.error(f"Failed to send Slack notification: {e}")

        if config.discord_webhook_url:
            try:
                self._send_discord_notification(qa_report, merge_context, evaluation)
            except Exception as e:
                logger.error(f"Failed to send Discord notification: {e}")

        if config.email_smtp_server and config.email_to:
            try:
                self._send_email_notification(qa_report, merge_context, evaluation)
            except Exception as e:
                logger.error(f"Failed to send email notification: {e}")

    def _send_slack_notification(self, qa_report: QAReport, merge_context: MergeContext, evaluation: Dict[str, Any]):
        """Send Slack notification via webhook"""

        config = self.config.notification_config

        # Determine status and color
        if evaluation["block_reasons"]:
            status = "❌ FAILED"
            color = "danger"
        elif evaluation["warnings"]:
            status = "⚠️ WARNING"
            color = "warning"
        else:
            status = "✅ PASSED"
            color = "good"

        # Create Slack message
        message = {
            "attachments": [
                {
                    "color": color,
                    "title": f"🔬 Pre-Merge QA Analysis: {status}",
                    "fields": [
                        {
                            "title": "Merge Details",
                            "value": f"*{merge_context.source_branch}* → *{merge_context.target_branch}*\nAuthor: {merge_context.author}",
                            "short": True
                        },
                        {
                            "title": "Quality Scores",
                            "value": f"Algorithmic: {qa_report.metrics.algorithmic_complexity}\nQuantum: {qa_report.metrics.quantum_optimization}\nEngineering: {qa_report.metrics.software_engineering}\nQuality: {qa_report.metrics.code_quality}",
                            "short": True
                        },
                        {
                            "title": "Issues Summary",
                            "value": f"Total: {len(qa_report.issues)}\nCritical: {len([i for i in qa_report.issues if i.severity.name == 'CRITICAL'])}\nHigh: {len([i for i in qa_report.issues if i.severity.name == 'HIGH'])}\nWarnings: {len(evaluation['warnings'])}",
                            "short": True
                        },
                        {
                            "title": "Merge Status",
                            "value": f"Confidence: {evaluation['merge_confidence']:.1%}\nRisk Level: {evaluation['risk_level']}",
                            "short": True
                        }
                    ],
                    "footer": "Codex QA Engineering System",
                    "ts": datetime.now().timestamp()
                }
            ]
        }

        # Add blocking reasons if any
        if evaluation["block_reasons"]:
            message["attachments"][0]["fields"].append({
                "title": "🚫 Blocking Issues",
                "value": "\n".join(f"• {reason}" for reason in evaluation["block_reasons"][:3]),
                "short": False
            })

        # Add recommendations
        if evaluation["recommendations"]:
            message["attachments"][0]["fields"].append({
                "title": "💡 Key Recommendations",
                "value": "\n".join(f"• {rec}" for rec in evaluation["recommendations"][:3]),
                "short": False
            })

        # Send to Slack
        response = requests.post(config.slack_webhook_url, json=message)
        response.raise_for_status()

        logger.info("Slack notification sent successfully")

    def _send_discord_notification(self, qa_report: QAReport, merge_context: MergeContext, evaluation: Dict[str, Any]):
        """Send Discord notification via webhook"""

        config = self.config.notification_config

        # Determine status emoji
        if evaluation["block_reasons"]:
            status_emoji = "❌"
            color = 15158332  # Red
        elif evaluation["warnings"]:
            status_emoji = "⚠️"
            color = 16776960  # Yellow
        else:
            status_emoji = "✅"
            color = 3066993   # Green

        # Create Discord embed
        embed = {
            "title": f"{status_emoji} Pre-Merge QA Analysis Results",
            "description": f"**{merge_context.source_branch}** → **{merge_context.target_branch}**",
            "color": color,
            "fields": [
                {
                    "name": "👤 Author",
                    "value": merge_context.author,
                    "inline": True
                },
                {
                    "name": "📊 Quality Scores",
                    "value": f"""Algorithmic: `{qa_report.metrics.algorithmic_complexity}`
Quantum: `{qa_report.metrics.quantum_optimization}`
Engineering: `{qa_report.metrics.software_engineering}`
Quality: `{qa_report.metrics.code_quality}`""",
                    "inline": True
                },
                {
                    "name": "🔍 Issues Summary",
                    "value": f"""Total: `{len(qa_report.issues)}`
Critical: `{len([i for i in qa_report.issues if i.severity.name == 'CRITICAL'])}`
High: `{len([i for i in qa_report.issues if i.severity.name == 'HIGH'])}`
Warnings: `{len(evaluation['warnings'])}`""",
                    "inline": True
                },
                {
                    "name": "🎯 Merge Confidence",
                    "value": f"`{evaluation['merge_confidence']:.1%}` ({evaluation['risk_level']} risk)",
                    "inline": True
                }
            ],
            "timestamp": datetime.now().isoformat(),
            "footer": {
                "text": "Codex QA Engineering System"
            }
        }

        # Add blocking issues
        if evaluation["block_reasons"]:
            embed["fields"].append({
                "name": "🚫 Blocking Issues",
                "value": "\n".join(f"• {reason}" for reason in evaluation["block_reasons"][:3]),
                "inline": False
            })

        # Add key recommendations
        if evaluation["recommendations"]:
            embed["fields"].append({
                "name": "💡 Recommendations",
                "value": "\n".join(f"• {rec}" for rec in evaluation["recommendations"][:3]),
                "inline": False
            })

        # Create Discord message
        message = {
            "embeds": [embed]
        }

        # Send to Discord
        response = requests.post(config.discord_webhook_url, json=message)
        response.raise_for_status()

        logger.info("Discord notification sent successfully")

    def _send_email_notification(self, qa_report: QAReport, merge_context: MergeContext, evaluation: Dict[str, Any]):
        """Send email notification via SMTP"""

        config = self.config.notification_config

        # Determine subject and status
        if evaluation["block_reasons"]:
            subject = f"❌ QA BLOCKED: Merge {merge_context.source_branch} → {merge_context.target_branch}"
            status_emoji = "❌"
        elif evaluation["warnings"]:
            subject = f"⚠️ QA WARNING: Merge {merge_context.source_branch} → {merge_context.target_branch}"
            status_emoji = "⚠️"
        else:
            subject = f"✅ QA PASSED: Merge {merge_context.source_branch} → {merge_context.target_branch}"
            status_emoji = "✅"

        # Create email content
        html_content = f"""
        <html>
        <head>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 20px; }}
                .header {{ background-color: #f8f9fa; padding: 20px; border-radius: 5px; margin-bottom: 20px; }}
                .scores {{ display: flex; flex-wrap: wrap; gap: 20px; margin: 20px 0; }}
                .score-card {{ background: #fff; border: 1px solid #dee2e6; padding: 15px; border-radius: 5px; min-width: 200px; }}
                .issues {{ margin: 20px 0; }}
                .issue-block {{ background: #f8d7da; border: 1px solid #f5c6cb; color: #721c24; padding: 15px; border-radius: 5px; margin: 10px 0; }}
                .issue-warn {{ background: #fff3cd; border: 1px solid #ffeaa7; color: #856404; padding: 15px; border-radius: 5px; margin: 10px 0; }}
                .footer {{ margin-top: 30px; padding-top: 20px; border-top: 1px solid #dee2e6; color: #6c757d; }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>{status_emoji} Pre-Merge QA Analysis Results</h1>
                <p><strong>Merge:</strong> {merge_context.source_branch} → {merge_context.target_branch}</p>
                <p><strong>Author:</strong> {merge_context.author}</p>
                <p><strong>Generated:</strong> {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</p>
            </div>

            <div class="scores">
                <div class="score-card">
                    <h3>Algorithmic Complexity</h3>
                    <div style="font-size: 24px; font-weight: bold;">{qa_report.metrics.algorithmic_complexity}</div>
                </div>
                <div class="score-card">
                    <h3>Quantum Optimization</h3>
                    <div style="font-size: 24px; font-weight: bold;">{qa_report.metrics.quantum_optimization}</div>
                </div>
                <div class="score-card">
                    <h3>Software Engineering</h3>
                    <div style="font-size: 24px; font-weight: bold;">{qa_report.metrics.software_engineering}</div>
                </div>
                <div class="score-card">
                    <h3>Code Quality</h3>
                    <div style="font-size: 24px; font-weight: bold;">{qa_report.metrics.code_quality}</div>
                </div>
            </div>

            <div class="issues">
                <h2>Issues Summary</h2>
                <ul>
                    <li><strong>Total Issues:</strong> {len(qa_report.issues)}</li>
                    <li><strong>Critical:</strong> {len([i for i in qa_report.issues if i.severity.name == 'CRITICAL'])}</li>
                    <li><strong>High:</strong> {len([i for i in qa_report.issues if i.severity.name == 'HIGH'])}</li>
                    <li><strong>Medium:</strong> {len([i for i in qa_report.issues if i.severity.name == 'MEDIUM'])}</li>
                    <li><strong>Low:</strong> {len([i for i in qa_report.issues if i.severity.name == 'LOW'])}</li>
                    <li><strong>Warnings:</strong> {len(evaluation['warnings'])}</li>
                </ul>
            </div>

            {"".join(f'''
            <div class="issue-block">
                <h3>🚫 Blocking Issues</h3>
                <ul>
                    {"".join(f"<li>{reason}</li>" for reason in evaluation["block_reasons"])}
                </ul>
            </div>
            ''' if evaluation["block_reasons"] else "")}

            {"".join(f'''
            <div class="issue-warn">
                <h3>⚠️ Warnings</h3>
                <ul>
                    {"".join(f"<li>{warning}</li>" for warning in evaluation["warnings"])}
                </ul>
            </div>
            ''' if evaluation["warnings"] else "")}

            {"".join(f'''
            <div class="issue-warn">
                <h3>💡 Recommendations</h3>
                <ul>
                    {"".join(f"<li>{rec}</li>" for rec in evaluation["recommendations"])}
                </ul>
            </div>
            ''' if evaluation["recommendations"] else "")}

            <div class="footer">
                <p>
                    <strong>Merge Confidence:</strong> {evaluation['merge_confidence']:.1%}<br>
                    <strong>Risk Level:</strong> {evaluation['risk_level']}
                </p>
                <p><em>Generated by Codex QA Engineering System</em></p>
                <p><small>Please review the detailed QA report for complete analysis.</small></p>
            </div>
        </body>
        </html>
        """

        # Create email message
        msg = MIMEMultipart('alternative')
        msg['Subject'] = subject
        msg['From'] = config.email_from
        msg['To'] = ', '.join(config.email_to)

        # Attach HTML content
        html_part = MIMEText(html_content, 'html')
        msg.attach(html_part)

        # Send email
        try:
            server = smtplib.SMTP(config.email_smtp_server, config.email_smtp_port)
            server.starttls()
            server.login(config.email_username, config.email_password)
            server.sendmail(config.email_from, config.email_to, msg.as_string())
            server.quit()

            logger.info(f"Email notification sent to {len(config.email_to)} recipients")

        except Exception as e:
            logger.error(f"Failed to send email: {e}")
            raise

def install_git_hooks():
    """Install git hooks for pre-merge QA"""

    hooks_dir = Path(".git/hooks")
    hooks_dir.mkdir(exist_ok=True)

    # Create pre-merge-commit hook
    hook_content = '''#!/bin/bash
# Pre-merge QA Hook

# Get branch names
TARGET_BRANCH=${1:-main}
SOURCE_BRANCH=$(git rev-parse --abbrev-ref HEAD)

echo "Running pre-merge QA analysis..."
echo "Source: $SOURCE_BRANCH"
echo "Target: $TARGET_BRANCH"

# Run QA analysis
python3 tools/premerge_qa_hook.py "$SOURCE_BRANCH" "$TARGET_BRANCH"
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
    echo "❌ Pre-merge QA failed - merge blocked"
    echo "Check merge-qa-reports/ for detailed analysis"
    exit 1
else
    echo "✅ Pre-merge QA passed - proceeding with merge"
fi
'''

    hook_path = hooks_dir / "pre-merge-commit"
    with open(hook_path, 'w', encoding='utf-8') as f:
        f.write(hook_content)

    # Make executable
    os.chmod(hook_path, 0o755)

    # Create prepare-commit-msg hook for additional checks
    prepare_hook_content = '''#!/bin/bash
# Prepare commit message hook with QA context

COMMIT_MSG_FILE=$1
COMMIT_SOURCE=$2

# If this is a merge commit, add QA summary
if [ "$COMMIT_SOURCE" = "merge" ]; then
    # Find latest QA report
    QA_REPORT=$(find merge-qa-reports -name "merge_qa_report_*.json" -type f -printf '%T@ %p\n' 2>/dev/null | sort -n | tail -1 | cut -d' ' -f2-)

    if [ -n "$QA_REPORT" ]; then
        echo "" >> "$COMMIT_MSG_FILE"
        echo "QA Analysis: $(basename "$QA_REPORT")" >> "$COMMIT_MSG_FILE"
    fi
fi
'''

    prepare_hook_path = hooks_dir / "prepare-commit-msg"
    with open(prepare_hook_path, 'w', encoding='utf-8') as f:
        f.write(prepare_hook_content)

    os.chmod(prepare_hook_path, 0o755)

    print("Git hooks installed successfully")
    print("Hooks: pre-merge-commit, prepare-commit-msg")

def main():
    """Main entry point for pre-merge QA hook"""

    if len(sys.argv) > 1 and sys.argv[1] == "--install-hooks":
        install_git_hooks()
        return

    if len(sys.argv) < 3:
        print("Usage: python premerge_qa_hook.py <source_branch> <target_branch>")
        print("Or: python premerge_qa_hook.py --install-hooks")
        sys.exit(1)

    source_branch = sys.argv[1]
    target_branch = sys.argv[2]

    # Load configuration
    config = MergeQAConfig()

    # Initialize hook
    hook = PreMergeQAHook(config)

    # Run pre-merge QA
    merge_allowed, results = hook.run_pre_merge_qa(source_branch, target_branch)

    # Save results
    results_file = Path("merge-qa-results.json")
    with open(results_file, 'w', encoding='utf-8') as f:
        json.dump(results, f, indent=2, ensure_ascii=False, default=str)

    # Exit with appropriate code
    if merge_allowed:
        print("✅ Merge approved by QA")
        sys.exit(0)
    else:
        print("❌ Merge blocked by QA")
        print("See merge-qa-reports/ for detailed analysis")
        sys.exit(1)

if __name__ == "__main__":
    main()