#!/usr/bin/env python3
"""
Progress Report Generator
Generates comprehensive project progress reports with metrics and insights.
"""

import argparse
import json
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, Any, List
import statistics

class ProgressReportGenerator:
    """Generates project progress reports"""

    def __init__(self):
        self.report_templates = {
            'weekly': self._weekly_report,
            'monthly': self._monthly_report,
            'milestone': self._milestone_report,
            'executive': self._executive_summary
        }

    def generate_report(self, report_type: str, project_data: Dict[str, Any]) -> Dict[str, Any]:
        """Generate a progress report"""
        if report_type not in self.report_templates:
            raise ValueError(f"Unknown report type: {report_type}")

        template_func = self.report_templates[report_type]
        return template_func(project_data)

    def _weekly_report(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Generate weekly progress report"""
        return {
            'report_type': 'Weekly Progress Report',
            'period': data.get('period', f"Week of {datetime.now().strftime('%Y-%m-%d')}"),
            'project_name': data.get('project_name', 'Unknown Project'),
            'sections': {
                'accomplishments': self._format_accomplishments(data.get('accomplishments', [])),
                'planned_work': self._format_planned_work(data.get('planned_work', [])),
                'blockers': self._format_blockers(data.get('blockers', [])),
                'risks': self._format_risks(data.get('risks', [])),
                'metrics': self._calculate_metrics(data),
                'next_week_focus': data.get('next_week_focus', [])
            },
            'generated_at': datetime.now().isoformat()
        }

    def _monthly_report(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Generate monthly progress report"""
        return {
            'report_type': 'Monthly Progress Report',
            'period': data.get('period', f"{datetime.now().strftime('%B %Y')}"),
            'project_name': data.get('project_name', 'Unknown Project'),
            'sections': {
                'executive_summary': data.get('executive_summary', ''),
                'key_achievements': self._format_achievements(data.get('achievements', [])),
                'challenges_overcome': data.get('challenges_overcome', []),
                'budget_status': self._calculate_budget_status(data),
                'schedule_status': self._calculate_schedule_status(data),
                'quality_metrics': self._calculate_quality_metrics(data),
                'team_performance': self._calculate_team_performance(data),
                'next_month_priorities': data.get('next_month_priorities', [])
            },
            'generated_at': datetime.now().isoformat()
        }

    def _milestone_report(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Generate milestone completion report"""
        return {
            'report_type': 'Milestone Completion Report',
            'milestone_name': data.get('milestone_name', 'Unknown Milestone'),
            'project_name': data.get('project_name', 'Unknown Project'),
            'completion_date': data.get('completion_date', datetime.now().isoformat()),
            'sections': {
                'milestone_objectives': data.get('objectives', []),
                'deliverables': self._format_deliverables(data.get('deliverables', [])),
                'quality_assurance': self._format_quality_checks(data.get('quality_checks', [])),
                'lessons_learned': data.get('lessons_learned', []),
                'next_milestone_prep': data.get('next_milestone_prep', []),
                'celebrations': data.get('celebrations', [])
            },
            'generated_at': datetime.now().isoformat()
        }

    def _executive_summary(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Generate executive summary report"""
        return {
            'report_type': 'Executive Summary',
            'period': data.get('period', f"Q{((datetime.now().month - 1) // 3) + 1} {datetime.now().year}"),
            'project_name': data.get('project_name', 'Unknown Project'),
            'sections': {
                'overall_status': self._determine_overall_status(data),
                'key_highlights': data.get('key_highlights', []),
                'critical_issues': data.get('critical_issues', []),
                'financial_summary': self._generate_financial_summary(data),
                'schedule_forecast': self._generate_schedule_forecast(data),
                'recommendations': data.get('recommendations', [])
            },
            'generated_at': datetime.now().isoformat()
        }

    def _format_accomplishments(self, accomplishments: List[str]) -> List[str]:
        """Format accomplishments list"""
        return [f"✅ {item}" for item in accomplishments]

    def _format_planned_work(self, planned_work: List[str]) -> List[str]:
        """Format planned work list"""
        return [f"🎯 {item}" for item in planned_work]

    def _format_blockers(self, blockers: List[str]) -> List[str]:
        """Format blockers list"""
        return [f"🚧 {item}" for item in blockers]

    def _format_risks(self, risks: List[Dict[str, Any]]) -> List[str]:
        """Format risks list"""
        formatted = []
        for risk in risks:
            status = risk.get('status', 'monitoring')
            emoji = {'critical': '🔴', 'high': '🟠', 'medium': '🟡', 'low': '🟢'}.get(
                risk.get('severity', 'medium'), '🟢')
            formatted.append(f"{emoji} {risk.get('description', 'Unknown risk')} - {status}")
        return formatted

    def _calculate_metrics(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Calculate project metrics"""
        tasks_completed = len(data.get('accomplishments', []))
        tasks_planned = len(data.get('planned_work', []))
        blockers_count = len(data.get('blockers', []))

        completion_rate = (tasks_completed / max(tasks_planned, 1)) * 100

        return {
            'tasks_completed': tasks_completed,
            'tasks_planned': tasks_planned,
            'completion_rate': f"{completion_rate:.1f}%",
            'active_blockers': blockers_count,
            'velocity_trend': self._calculate_velocity_trend(data)
        }

    def _calculate_velocity_trend(self, data: Dict[str, Any]) -> str:
        """Calculate team velocity trend"""
        # Simplified velocity calculation
        recent_velocity = data.get('recent_velocity', [10, 12, 8, 15])
        if len(recent_velocity) < 2:
            return "stable"

        avg_recent = statistics.mean(recent_velocity[-3:]) if len(recent_velocity) >= 3 else statistics.mean(recent_velocity)
        avg_overall = statistics.mean(recent_velocity)

        if avg_recent > avg_overall * 1.1:
            return "increasing"
        elif avg_recent < avg_overall * 0.9:
            return "decreasing"
        else:
            return "stable"

    def _format_achievements(self, achievements: List[str]) -> List[str]:
        """Format achievements list"""
        return [f"🏆 {item}" for item in achievements]

    def _calculate_budget_status(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Calculate budget status"""
        budget_used = data.get('budget_used', 75000)
        budget_total = data.get('budget_total', 100000)
        utilization = (budget_used / budget_total) * 100

        return {
            'budget_used': f"${budget_used:,.0f}",
            'budget_remaining': f"${budget_total - budget_used:,.0f}",
            'utilization_percentage': f"{utilization:.1f}%",
            'status': 'on_track' if utilization <= 85 else 'at_risk' if utilization <= 95 else 'over_budget'
        }

    def _calculate_schedule_status(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Calculate schedule status"""
        days_completed = data.get('days_completed', 15)
        days_planned = data.get('days_planned', 20)
        progress_percentage = (days_completed / days_planned) * 100

        return {
            'days_completed': days_completed,
            'days_planned': days_planned,
            'progress_percentage': f"{progress_percentage:.1f}%",
            'status': 'ahead' if progress_percentage > 105 else 'on_track' if progress_percentage >= 95 else 'behind'
        }

    def _calculate_quality_metrics(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Calculate quality metrics"""
        test_coverage = data.get('test_coverage', 85.5)
        defect_density = data.get('defect_density', 0.8)
        code_quality_score = data.get('code_quality_score', 8.2)

        return {
            'test_coverage': f"{test_coverage:.1f}%",
            'defect_density': f"{defect_density:.1f} defects/KLOC",
            'code_quality_score': f"{code_quality_score:.1f}/10",
            'overall_quality': 'excellent' if code_quality_score >= 9 else 'good' if code_quality_score >= 7 else 'needs_improvement'
        }

    def _calculate_team_performance(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Calculate team performance metrics"""
        team_velocity = data.get('team_velocity', 45)
        sprint_success_rate = data.get('sprint_success_rate', 92.5)

        return {
            'average_velocity': f"{team_velocity} story points",
            'sprint_success_rate': f"{sprint_success_rate:.1f}%",
            'productivity_trend': 'improving' if sprint_success_rate > 90 else 'stable' if sprint_success_rate > 80 else 'needs_attention'
        }

    def _format_deliverables(self, deliverables: List[Dict[str, Any]]) -> List[str]:
        """Format deliverables list"""
        formatted = []
        for deliverable in deliverables:
            status = deliverable.get('status', 'pending')
            emoji = {'completed': '✅', 'in_progress': '🔄', 'pending': '⏳', 'blocked': '🚫'}.get(status, '❓')
            formatted.append(f"{emoji} {deliverable.get('name', 'Unknown')} - {deliverable.get('description', '')}")
        return formatted

    def _format_quality_checks(self, quality_checks: List[Dict[str, Any]]) -> List[str]:
        """Format quality checks list"""
        formatted = []
        for check in quality_checks:
            status = check.get('status', 'pending')
            emoji = {'passed': '✅', 'failed': '❌', 'pending': '⏳', 'na': '➖'}.get(status, '❓')
            formatted.append(f"{emoji} {check.get('name', 'Unknown check')} - {check.get('result', '')}")
        return formatted

    def _determine_overall_status(self, data: Dict[str, Any]) -> str:
        """Determine overall project status"""
        schedule_status = data.get('schedule_status', 'on_track')
        budget_status = data.get('budget_status', 'on_track')
        quality_status = data.get('quality_status', 'good')

        if all(status in ['ahead', 'on_track', 'excellent', 'good'] for status in [schedule_status, budget_status, quality_status]):
            return '🟢 On Track - All systems green'
        elif any(status in ['behind', 'over_budget', 'needs_improvement'] for status in [schedule_status, budget_status, quality_status]):
            return '🟠 At Risk - Requires attention'
        else:
            return '🔴 Critical - Immediate action required'

    def _generate_financial_summary(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Generate financial summary"""
        budget_used = data.get('budget_used', 75000)
        budget_total = data.get('budget_total', 100000)
        forecasted_total = data.get('forecasted_total', 105000)

        return {
            'budget_utilized': f"${budget_used:,.0f}",
            'budget_remaining': f"${budget_total - budget_used:,.0f}",
            'forecasted_final_cost': f"${forecasted_total:,.0f}",
            'variance': f"${forecasted_total - budget_total:,.0f}",
            'burn_rate': f"${data.get('monthly_burn', 15000):,.0f}/month"
        }

    def _generate_schedule_forecast(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Generate schedule forecast"""
        completion_percentage = data.get('completion_percentage', 75)
        days_remaining = data.get('days_remaining', 45)
        forecasted_completion = datetime.now() + timedelta(days=days_remaining)

        return {
            'current_completion': f"{completion_percentage:.1f}%",
            'estimated_completion_date': forecasted_completion.strftime('%Y-%m-%d'),
            'days_remaining': days_remaining,
            'schedule_variance': f"{data.get('schedule_variance_days', 3)} days",
            'critical_path_status': data.get('critical_path_status', 'on_track')
        }

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(description='Generate project progress reports')
    parser.add_argument('report_type', choices=['weekly', 'monthly', 'milestone', 'executive'],
                       help='Type of progress report')
    parser.add_argument('--project-name', default='Sample Project', help='Name of the project')
    parser.add_argument('--data', type=Path, help='JSON file with project data')
    parser.add_argument('--output', '-o', type=Path, help='Output file path')

    args = parser.parse_args()

    # Load project data
    if args.data and args.data.exists():
        with open(args.data, 'r', encoding='utf-8') as f:
            project_data = json.load(f)
    else:
        # Use sample data
        project_data = {
            'project_name': args.project_name,
            'accomplishments': [
                'Completed user authentication module',
                'Implemented responsive dashboard UI',
                'Set up CI/CD pipeline'
            ],
            'planned_work': [
                'Implement payment processing',
                'Add user profile management',
                'Write API documentation'
            ],
            'blockers': [
                'Third-party API integration delayed'
            ],
            'risks': [
                {'description': 'API rate limiting issues', 'severity': 'medium', 'status': 'monitoring'},
                {'description': 'Team member vacation', 'severity': 'low', 'status': 'mitigated'}
            ]
        }

    generator = ProgressReportGenerator()
    report = generator.generate_report(args.report_type, project_data)

    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            json.dump(report, f, indent=2, ensure_ascii=False)
        print(f"Report saved to {args.output}")
    else:
        print(json.dumps(report, indent=2, ensure_ascii=False))

if __name__ == '__main__':
    main()