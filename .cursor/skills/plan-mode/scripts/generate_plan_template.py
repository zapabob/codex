#!/usr/bin/env python3
"""
Plan Template Generator
Generates structured project plan templates based on project type and methodology.
"""

import argparse
import json
from datetime import datetime, timedelta
from pathlib import Path
from typing import Dict, Any, List

class PlanTemplateGenerator:
    """Generates project plan templates"""

    def __init__(self):
        self.templates = {
            'web-app': self._web_app_template,
            'mobile-app': self._mobile_app_template,
            'api-service': self._api_service_template,
            'data-migration': self._data_migration_template,
            'research-project': self._research_project_template,
        }

    def generate_template(self, project_type: str, project_name: str, methodology: str = 'agile') -> Dict[str, Any]:
        """Generate a project plan template"""
        if project_type not in self.templates:
            raise ValueError(f"Unknown project type: {project_type}")

        template_func = self.templates[project_type]
        return template_func(project_name, methodology)

    def _web_app_template(self, name: str, methodology: str) -> Dict[str, Any]:
        """Generate web application project template"""
        phases = [
            {
                'name': 'Foundation',
                'duration_weeks': 2,
                'tasks': [
                    'Set up development environment and CI/CD',
                    'Design system architecture',
                    'Implement authentication system',
                    'Create basic project structure'
                ],
                'milestones': ['Development environment ready', 'Architecture approved']
            },
            {
                'name': 'Core Features',
                'duration_weeks': 4,
                'tasks': [
                    'Implement user management',
                    'Build main application features',
                    'Create responsive UI components',
                    'Implement business logic'
                ],
                'milestones': ['Core features complete', 'UI/UX approved']
            },
            {
                'name': 'Integration & Testing',
                'duration_weeks': 2,
                'tasks': [
                    'Integrate third-party services',
                    'Write comprehensive tests',
                    'Performance optimization',
                    'Security audit'
                ],
                'milestones': ['All tests passing', 'Performance benchmarks met']
            },
            {
                'name': 'Deployment & Launch',
                'duration_weeks': 1,
                'tasks': [
                    'Production deployment setup',
                    'Final testing and validation',
                    'Documentation completion',
                    'Go-live preparation'
                ],
                'milestones': ['Successfully deployed', 'User acceptance']
            }
        ]

        return self._build_template(name, phases, methodology)

    def _mobile_app_template(self, name: str, methodology: str) -> Dict[str, Any]:
        """Generate mobile application project template"""
        phases = [
            {
                'name': 'Planning & Design',
                'duration_weeks': 2,
                'tasks': [
                    'Market research and competitor analysis',
                    'User persona development',
                    'UI/UX design and prototyping',
                    'Technical architecture design'
                ],
                'milestones': ['Design approved', 'Technical spec complete']
            },
            {
                'name': 'Core Development',
                'duration_weeks': 6,
                'tasks': [
                    'Set up development environment',
                    'Implement core features',
                    'Create native UI components',
                    'Backend API integration'
                ],
                'milestones': ['MVP features complete', 'App store guidelines met']
            },
            {
                'name': 'Testing & Polish',
                'duration_weeks': 2,
                'tasks': [
                    'Unit and integration testing',
                    'User acceptance testing',
                    'Performance optimization',
                    'Bug fixing and polishing'
                ],
                'milestones': ['All tests passing', 'Performance optimized']
            },
            {
                'name': 'Launch Preparation',
                'duration_weeks': 1,
                'tasks': [
                    'App store submission',
                    'Marketing materials preparation',
                    'Launch documentation',
                    'Support team training'
                ],
                'milestones': ['App store approved', 'Launch ready']
            }
        ]

        return self._build_template(name, phases, methodology)

    def _api_service_template(self, name: str, methodology: str) -> Dict[str, Any]:
        """Generate API service project template"""
        phases = [
            {
                'name': 'API Design',
                'duration_weeks': 1,
                'tasks': [
                    'Define API requirements',
                    'Design RESTful endpoints',
                    'Create OpenAPI specification',
                    'Set up API versioning strategy'
                ],
                'milestones': ['API spec approved', 'Endpoint design complete']
            },
            {
                'name': 'Implementation',
                'duration_weeks': 3,
                'tasks': [
                    'Set up project infrastructure',
                    'Implement core endpoints',
                    'Add authentication and authorization',
                    'Implement business logic'
                ],
                'milestones': ['Core API functional', 'Authentication working']
            },
            {
                'name': 'Quality Assurance',
                'duration_weeks': 2,
                'tasks': [
                    'Write comprehensive tests',
                    'API documentation',
                    'Performance testing',
                    'Security testing'
                ],
                'milestones': ['All tests passing', 'Documentation complete']
            },
            {
                'name': 'Deployment',
                'duration_weeks': 1,
                'tasks': [
                    'Production environment setup',
                    'API deployment and monitoring',
                    'Client SDK generation',
                    'Launch preparation'
                ],
                'milestones': ['API deployed', 'Clients can integrate']
            }
        ]

        return self._build_template(name, phases, methodology)

    def _data_migration_template(self, name: str, methodology: str) -> Dict[str, Any]:
        """Generate data migration project template"""
        phases = [
            {
                'name': 'Assessment',
                'duration_weeks': 1,
                'tasks': [
                    'Analyze source data structure',
                    'Assess data quality and volume',
                    'Identify transformation requirements',
                    'Plan migration strategy'
                ],
                'milestones': ['Data assessment complete', 'Migration strategy approved']
            },
            {
                'name': 'Design & Development',
                'duration_weeks': 2,
                'tasks': [
                    'Design target data model',
                    'Develop migration scripts',
                    'Create data validation rules',
                    'Set up monitoring and logging'
                ],
                'milestones': ['Migration scripts ready', 'Validation rules defined']
            },
            {
                'name': 'Testing',
                'duration_weeks': 1,
                'tasks': [
                    'Run migration tests',
                    'Validate data integrity',
                    'Performance testing',
                    'Error handling verification'
                ],
                'milestones': ['Tests passing', 'Data validation successful']
            },
            {
                'name': 'Execution & Go-live',
                'duration_weeks': 1,
                'tasks': [
                    'Execute production migration',
                    'Monitor migration progress',
                    'Verify data consistency',
                    'Rollback plan preparation'
                ],
                'milestones': ['Migration successful', 'Data consistency verified']
            }
        ]

        return self._build_template(name, phases, methodology)

    def _research_project_template(self, name: str, methodology: str) -> Dict[str, Any]:
        """Generate research project template"""
        phases = [
            {
                'name': 'Research Planning',
                'duration_weeks': 1,
                'tasks': [
                    'Define research objectives',
                    'Literature review planning',
                    'Methodology selection',
                    'Resource identification'
                ],
                'milestones': ['Research plan approved', 'Objectives defined']
            },
            {
                'name': 'Data Collection',
                'duration_weeks': 3,
                'tasks': [
                    'Implement data collection methods',
                    'Execute literature review',
                    'Gather experimental data',
                    'Data preprocessing and cleaning'
                ],
                'milestones': ['Data collection complete', 'Dataset ready for analysis']
            },
            {
                'name': 'Analysis & Findings',
                'duration_weeks': 2,
                'tasks': [
                    'Data analysis and modeling',
                    'Results interpretation',
                    'Statistical validation',
                    'Findings documentation'
                ],
                'milestones': ['Analysis complete', 'Key findings identified']
            },
            {
                'name': 'Reporting & Publication',
                'duration_weeks': 1,
                'tasks': [
                    'Research paper writing',
                    'Results visualization',
                    'Peer review preparation',
                    'Publication submission'
                ],
                'milestones': ['Paper submitted', 'Research complete']
            }
        ]

        return self._build_template(name, phases, methodology)

    def _build_template(self, name: str, phases: List[Dict], methodology: str) -> Dict[str, Any]:
        """Build complete project template"""
        start_date = datetime.now()
        current_date = start_date

        template_phases = []
        total_duration = 0

        for phase in phases:
            phase_start = current_date
            phase_end = current_date + timedelta(weeks=phase['duration_weeks'])
            current_date = phase_end
            total_duration += phase['duration_weeks']

            template_phases.append({
                'name': phase['name'],
                'duration_weeks': phase['duration_weeks'],
                'start_date': phase_start.strftime('%Y-%m-%d'),
                'end_date': phase_end.strftime('%Y-%m-%d'),
                'tasks': phase['tasks'],
                'milestones': phase['milestones']
            })

        return {
            'project_name': name,
            'methodology': methodology,
            'total_duration_weeks': total_duration,
            'start_date': start_date.strftime('%Y-%m-%d'),
            'end_date': current_date.strftime('%Y-%m-%d'),
            'phases': template_phases,
            'risks': self._generate_risks(),
            'success_metrics': self._generate_metrics()
        }

    def _generate_risks(self) -> List[Dict[str, Any]]:
        """Generate common project risks"""
        return [
            {
                'category': 'Technical',
                'risk': 'Technology complexity underestimated',
                'probability': 'Medium',
                'impact': 'High',
                'mitigation': 'Conduct technical spike early'
            },
            {
                'category': 'Schedule',
                'risk': 'Key resources unavailable',
                'probability': 'Low',
                'impact': 'High',
                'mitigation': 'Identify backup resources'
            },
            {
                'category': 'Scope',
                'risk': 'Requirements change during development',
                'probability': 'Medium',
                'impact': 'Medium',
                'mitigation': 'Use change control process'
            }
        ]

    def _generate_metrics(self) -> List[str]:
        """Generate success metrics"""
        return [
            'On-time delivery rate',
            'Budget adherence',
            'Quality metrics (defect rate, test coverage)',
            'Stakeholder satisfaction',
            'Team productivity metrics'
        ]

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(description='Generate project plan templates')
    parser.add_argument('project_type', choices=[
        'web-app', 'mobile-app', 'api-service', 'data-migration', 'research-project'
    ], help='Type of project')
    parser.add_argument('project_name', help='Name of the project')
    parser.add_argument('--methodology', choices=['agile', 'waterfall', 'kanban'],
                       default='agile', help='Project methodology')
    parser.add_argument('--output', '-o', type=Path, help='Output file path')

    args = parser.parse_args()

    generator = PlanTemplateGenerator()
    template = generator.generate_template(
        args.project_type,
        args.project_name,
        args.methodology
    )

    if args.output:
        with open(args.output, 'w', encoding='utf-8') as f:
            json.dump(template, f, indent=2, ensure_ascii=False)
        print(f"Template saved to {args.output}")
    else:
        print(json.dumps(template, indent=2, ensure_ascii=False))

if __name__ == '__main__':
    main()