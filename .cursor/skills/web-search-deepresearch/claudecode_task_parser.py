#!/usr/bin/env python3
"""
ClaudeCode-Style Task Parser for Web Search Deepresearch 2.1
Provides natural language task understanding and autonomous execution capabilities.
"""

import re
import json
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, field
from enum import Enum
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class TaskType(Enum):
    RESEARCH = "research"
    CODE_GENERATION = "code_generation"
    CODE_REVIEW = "code_review"
    TESTING = "testing"
    DEPLOYMENT = "deployment"
    OPTIMIZATION = "optimization"
    ANALYSIS = "analysis"
    INTEGRATION = "integration"

class ComplexityLevel(Enum):
    SIMPLE = "simple"
    MODERATE = "moderate"
    COMPLEX = "complex"
    EXPERT = "expert"

@dataclass
class TaskSpecification:
    """ClaudeCode-style task specification with natural language understanding"""
    raw_input: str
    task_type: TaskType
    intent: str
    context: Dict[str, Any]
    requirements: List[str]
    success_criteria: List[str]
    complexity: ComplexityLevel
    estimated_effort: str
    suggested_approach: str
    dependencies: List[str] = field(default_factory=list)
    constraints: Dict[str, Any] = field(default_factory=dict)
    metadata: Dict[str, Any] = field(default_factory=dict)

@dataclass
class ExecutionPlan:
    """Autonomous execution plan with ClaudeCode intelligence"""
    task: TaskSpecification
    steps: List[Dict[str, Any]]
    estimated_duration: str
    risk_assessment: Dict[str, Any]
    fallback_strategies: List[Dict[str, Any]]
    success_metrics: List[str]

class ClaudeCodeTaskParser:
    """
    Natural language task parser inspired by ClaudeCode's understanding capabilities.
    """

    def __init__(self):
        self.intent_patterns = {
            TaskType.RESEARCH: [
                r'research', r'investigate', r'explore', r'analyze', r'study',
                r'find out', r'discover', r'examine', r'survey'
            ],
            TaskType.CODE_GENERATION: [
                r'create', r'build', r'implement', r'develop', r'write',
                r'generate', r'make', r'construct', r'produce'
            ],
            TaskType.CODE_REVIEW: [
                r'review', r'check', r'validate', r'inspect', r'examine',
                r'assess', r'evaluate', r'audit'
            ],
            TaskType.TESTING: [
                r'test', r'verify', r'validate', r'check', r'confirm',
                r'ensure', r'prove'
            ],
            TaskType.DEPLOYMENT: [
                r'deploy', r'release', r'publish', r'launch', r'distribute'
            ],
            TaskType.OPTIMIZATION: [
                r'optimize', r'improve', r'enhance', r'speed up', r'refactor',
                r'performance', r'efficiency'
            ],
            TaskType.ANALYSIS: [
                r'analyze', r'assess', r'evaluate', r'measure', r'benchmark'
            ],
            TaskType.INTEGRATION: [
                r'integrate', r'connect', r'combine', r'merge', r'unify'
            ]
        }

        self.complexity_indicators = {
            ComplexityLevel.SIMPLE: ['simple', 'basic', 'straightforward', 'easy'],
            ComplexityLevel.MODERATE: ['moderate', 'intermediate', 'standard', 'typical'],
            ComplexityLevel.COMPLEX: ['complex', 'advanced', 'sophisticated', 'challenging'],
            ComplexityLevel.EXPERT: ['expert', 'cutting-edge', 'innovative', 'research-level']
        }

    def parse_task(self, user_input: str) -> TaskSpecification:
        """
        Parse natural language input into structured task specification.
        ClaudeCode-style understanding.
        """
        logger.info(f"Parsing task: {user_input[:100]}...")

        # Analyze intent
        task_type = self._analyze_intent(user_input)

        # Extract context
        context = self._extract_context(user_input)

        # Decompose requirements
        requirements = self._decompose_requirements(user_input)

        # Identify success criteria
        success_criteria = self._identify_success_criteria(user_input, requirements)

        # Assess complexity
        complexity = self._assess_complexity(user_input, requirements)

        # Estimate effort
        estimated_effort = self._estimate_effort(complexity, requirements)

        # Suggest approach
        suggested_approach = self._suggest_approach(task_type, complexity, context)

        # Extract dependencies
        dependencies = self._extract_dependencies(user_input, requirements)

        # Identify constraints
        constraints = self._identify_constraints(user_input)

        return TaskSpecification(
            raw_input=user_input,
            task_type=task_type,
            intent=self._extract_main_intent(user_input),
            context=context,
            requirements=requirements,
            success_criteria=success_criteria,
            complexity=complexity,
            estimated_effort=estimated_effort,
            suggested_approach=suggested_approach,
            dependencies=dependencies,
            constraints=constraints,
            metadata=self._generate_metadata(user_input, task_type)
        )

    def _analyze_intent(self, text: str) -> TaskType:
        """Analyze the primary intent of the user's request."""
        text_lower = text.lower()

        # Score each task type
        scores = {}
        for task_type, patterns in self.intent_patterns.items():
            score = 0
            for pattern in patterns:
                if re.search(r'\b' + pattern + r'\b', text_lower):
                    score += 1
            scores[task_type] = score

        # Return highest scoring task type
        return max(scores, key=scores.get)

    def _extract_main_intent(self, text: str) -> str:
        """Extract the main intent as a human-readable string."""
        task_type = self._analyze_intent(text)

        intent_templates = {
            TaskType.RESEARCH: "Research and analysis task",
            TaskType.CODE_GENERATION: "Code implementation task",
            TaskType.CODE_REVIEW: "Code review and validation task",
            TaskType.TESTING: "Testing and verification task",
            TaskType.DEPLOYMENT: "Deployment and release task",
            TaskType.OPTIMIZATION: "Optimization and improvement task",
            TaskType.ANALYSIS: "Analysis and assessment task",
            TaskType.INTEGRATION: "Integration and connection task"
        }

        return intent_templates.get(task_type, "General task")

    def _extract_context(self, text: str) -> Dict[str, Any]:
        """Extract relevant context information from the input."""
        context = {
            'technologies': self._extract_technologies(text),
            'domain': self._extract_domain(text),
            'stakeholders': self._extract_stakeholders(text),
            'timeline': self._extract_timeline(text),
            'priority': self._extract_priority(text)
        }

        return context

    def _extract_technologies(self, text: str) -> List[str]:
        """Extract mentioned technologies and tools."""
        technologies = []
        tech_patterns = [
            r'\b(React|Vue|Angular|Svelte|Next\.js|Nuxt)\b',
            r'\b(Node\.js|Python|Rust|Go|Java|TypeScript|JavaScript)\b',
            r'\b(Docker|Kubernetes|AWS|GCP|Azure)\b',
            r'\b(PostgreSQL|MongoDB|Redis|MySQL)\b',
            r'\b(Linux|Windows|macOS)\b'
        ]

        for pattern in tech_patterns:
            matches = re.findall(pattern, text, re.IGNORECASE)
            technologies.extend(matches)

        return list(set(technologies))

    def _extract_domain(self, text: str) -> Optional[str]:
        """Extract the problem domain or industry."""
        domains = {
            'web': ['website', 'web app', 'frontend', 'backend', 'full-stack'],
            'mobile': ['mobile', 'iOS', 'Android', 'React Native', 'Flutter'],
            'data': ['data science', 'machine learning', 'AI', 'analytics'],
            'devops': ['CI/CD', 'deployment', 'infrastructure', 'monitoring'],
            'security': ['security', 'authentication', 'encryption', 'privacy']
        }

        text_lower = text.lower()
        for domain, keywords in domains.items():
            if any(keyword in text_lower for keyword in keywords):
                return domain

        return None

    def _extract_stakeholders(self, text: str) -> List[str]:
        """Extract mentioned stakeholders or user types."""
        stakeholders = []
        stakeholder_patterns = [
            r'\b(users?|customers?|clients?)\b',
            r'\b(developers?|engineers?)\b',
            r'\b(administrators?|admins?)\b',
            r'\b(managers?|leads?)\b'
        ]

        for pattern in stakeholder_patterns:
            matches = re.findall(pattern, text, re.IGNORECASE)
            stakeholders.extend(matches)

        return list(set(stakeholders))

    def _extract_timeline(self, text: str) -> Optional[str]:
        """Extract timeline requirements."""
        timeline_patterns = [
            r'\b(today|tomorrow|this week|next week)\b',
            r'\b(\d+)\s*(days?|weeks?|months?)\b',
            r'\b(ASAP|immediately|urgent)\b'
        ]

        for pattern in timeline_patterns:
            match = re.search(pattern, text, re.IGNORECASE)
            if match:
                return match.group(0)

        return None

    def _extract_priority(self, text: str) -> Optional[str]:
        """Extract priority level."""
        if re.search(r'\b(high|urgent|critical|important)\b', text, re.IGNORECASE):
            return 'high'
        elif re.search(r'\b(medium|normal|standard)\b', text, re.IGNORECASE):
            return 'medium'
        elif re.search(r'\b(low|minor|optional)\b', text, re.IGNORECASE):
            return 'low'

        return 'medium'  # default

    def _decompose_requirements(self, text: str) -> List[str]:
        """Break down the task into specific requirements."""
        requirements = []

        # Split by common separators
        sentences = re.split(r'[.!?]+', text)
        sentences = [s.strip() for s in sentences if s.strip()]

        # Extract actionable items
        for sentence in sentences:
            if self._is_requirement(sentence):
                requirements.append(sentence)

        # If no clear requirements found, create based on task type
        if not requirements:
            task_type = self._analyze_intent(text)
            requirements = self._generate_default_requirements(task_type, text)

        return requirements

    def _is_requirement(self, sentence: str) -> bool:
        """Check if a sentence represents a requirement."""
        requirement_indicators = [
            'should', 'must', 'need to', 'have to', 'required',
            'implement', 'create', 'build', 'add', 'include',
            'ensure', 'make sure', 'provide', 'support'
        ]

        sentence_lower = sentence.lower()
        return any(indicator in sentence_lower for indicator in requirement_indicators)

    def _generate_default_requirements(self, task_type: TaskType, text: str) -> List[str]:
        """Generate default requirements based on task type."""
        base_requirements = {
            TaskType.CODE_GENERATION: [
                "Implement the requested functionality",
                "Follow coding best practices",
                "Include proper error handling",
                "Add necessary documentation"
            ],
            TaskType.RESEARCH: [
                "Gather comprehensive information",
                "Analyze multiple sources",
                "Validate findings",
                "Provide actionable insights"
            ],
            TaskType.TESTING: [
                "Create comprehensive test suite",
                "Test all functionality",
                "Validate edge cases",
                "Ensure reliability"
            ]
        }

        return base_requirements.get(task_type, ["Complete the requested task"])

    def _identify_success_criteria(self, text: str, requirements: List[str]) -> List[str]:
        """Identify measurable success criteria."""
        criteria = []

        # Extract explicit success criteria
        success_patterns = [
            r'should (be|have|work|function)',
            r'must (be|have|work|function)',
            r'needs? to (be|have|work|function)'
        ]

        for req in requirements:
            for pattern in success_patterns:
                matches = re.findall(pattern, req, re.IGNORECASE)
                criteria.extend([f"{req} - {match}" for match in matches])

        # Generate default criteria if none found
        if not criteria:
            criteria = [
                "Task completes without errors",
                "All requirements are met",
                "Code/functions as expected"
            ]

        return criteria

    def _assess_complexity(self, text: str, requirements: List[str]) -> ComplexityLevel:
        """Assess the complexity level of the task."""
        complexity_score = 0

        # Length and detail indicators
        if len(text) > 500:
            complexity_score += 2
        elif len(text) > 200:
            complexity_score += 1

        # Number of requirements
        if len(requirements) > 5:
            complexity_score += 2
        elif len(requirements) > 2:
            complexity_score += 1

        # Technical complexity indicators
        complex_terms = ['microservices', 'distributed', 'real-time', 'machine learning',
                        'blockchain', 'cryptography', 'parallel processing']
        for term in complex_terms:
            if term.lower() in text.lower():
                complexity_score += 1

        # Map score to complexity level
        if complexity_score >= 5:
            return ComplexityLevel.EXPERT
        elif complexity_score >= 3:
            return ComplexityLevel.COMPLEX
        elif complexity_score >= 1:
            return ComplexityLevel.MODERATE
        else:
            return ComplexityLevel.SIMPLE

    def _estimate_effort(self, complexity: ComplexityLevel, requirements: List[str]) -> str:
        """Estimate the effort required for the task."""
        base_effort = {
            ComplexityLevel.SIMPLE: "2-4 hours",
            ComplexityLevel.MODERATE: "1-2 days",
            ComplexityLevel.COMPLEX: "3-5 days",
            ComplexityLevel.EXPERT: "1-2 weeks"
        }

        # Adjust based on number of requirements
        multiplier = min(len(requirements) / 3, 2.0)

        effort = base_effort[complexity]
        if multiplier > 1:
            # Increase effort estimate
            if 'hours' in effort:
                hours = int(effort.split('-')[0]) * multiplier
                effort = f"{int(hours)}-{int(hours*1.5)} hours"
            elif 'days' in effort:
                days = int(effort.split('-')[0]) * multiplier
                effort = f"{int(days)}-{int(days*1.5)} days"
            elif 'weeks' in effort:
                weeks = int(effort.split('-')[0]) * multiplier
                effort = f"{int(weeks)}-{int(weeks*1.5)} weeks"

        return effort

    def _suggest_approach(self, task_type: TaskType, complexity: ComplexityLevel,
                         context: Dict[str, Any]) -> str:
        """Suggest the best approach for the task."""
        approaches = {
            TaskType.CODE_GENERATION: {
                ComplexityLevel.SIMPLE: "Direct implementation with standard patterns",
                ComplexityLevel.MODERATE: "Modular design with TDD approach",
                ComplexityLevel.COMPLEX: "Architecture planning followed by iterative development",
                ComplexityLevel.EXPERT: "Research-driven approach with prototyping"
            },
            TaskType.RESEARCH: {
                ComplexityLevel.SIMPLE: "Targeted search with 2-3 sources",
                ComplexityLevel.MODERATE: "Comprehensive search across multiple sources",
                ComplexityLevel.COMPLEX: "Deep analysis with expert consultation",
                ComplexityLevel.EXPERT: "Multi-disciplinary research with validation"
            }
        }

        return approaches.get(task_type, {}).get(complexity, "Standard approach")

    def _extract_dependencies(self, text: str, requirements: List[str]) -> List[str]:
        """Extract task dependencies."""
        dependencies = []

        # Look for prerequisite indicators
        dep_patterns = [
            r'(?:before|after|depends on|requires?) (.+?)[.,]',
            r'(?:prerequisite|dependency)[:-]?\s*(.+?)[.,]'
        ]

        for pattern in dep_patterns:
            matches = re.findall(pattern, text, re.IGNORECASE)
            dependencies.extend(matches)

        return dependencies

    def _identify_constraints(self, text: str) -> Dict[str, Any]:
        """Identify task constraints."""
        constraints = {}

        # Time constraints
        if timeline := self._extract_timeline(text):
            constraints['timeline'] = timeline

        # Resource constraints
        if 'limited resources' in text.lower() or 'budget' in text.lower():
            constraints['resources'] = 'limited'

        # Technology constraints
        if tech := self._extract_technologies(text):
            constraints['technologies'] = tech

        return constraints

    def _generate_metadata(self, text: str, task_type: TaskType) -> Dict[str, Any]:
        """Generate additional metadata for the task."""
        return {
            'input_length': len(text),
            'task_type_confidence': 'high',  # Simplified
            'processing_timestamp': json.dumps(None),  # Would be actual timestamp
            'language': 'en',  # Could be detected
            'contains_code': bool(re.search(r'```|function|class|import', text)),
            'urgency_level': 'medium'  # Could be analyzed
        }

def create_execution_plan(task: TaskSpecification) -> ExecutionPlan:
    """
    Create an autonomous execution plan based on the task specification.
    ClaudeCode-style intelligent planning.
    """
    logger.info(f"Creating execution plan for: {task.intent}")

    # Generate execution steps based on task type
    steps = _generate_execution_steps(task)

    # Estimate duration
    estimated_duration = _estimate_duration(task, steps)

    # Risk assessment
    risk_assessment = _assess_risks(task, steps)

    # Fallback strategies
    fallback_strategies = _generate_fallbacks(task, risk_assessment)

    # Success metrics
    success_metrics = _define_success_metrics(task)

    return ExecutionPlan(
        task=task,
        steps=steps,
        estimated_duration=estimated_duration,
        risk_assessment=risk_assessment,
        fallback_strategies=fallback_strategies,
        success_metrics=success_metrics
    )

def _generate_execution_steps(task: TaskSpecification) -> List[Dict[str, Any]]:
    """Generate detailed execution steps."""
    steps = []

    if task.task_type == TaskType.RESEARCH:
        steps = [
            {
                'phase': 'planning',
                'description': 'Define research scope and objectives',
                'duration': '1 hour',
                'automated': True
            },
            {
                'phase': 'data_collection',
                'description': 'Gather information from multiple sources',
                'duration': '4-6 hours',
                'automated': True
            },
            {
                'phase': 'analysis',
                'description': 'Analyze and synthesize findings',
                'duration': '2-3 hours',
                'automated': True
            },
            {
                'phase': 'validation',
                'description': 'Validate conclusions and recommendations',
                'duration': '1-2 hours',
                'automated': True
            }
        ]
    elif task.task_type == TaskType.CODE_GENERATION:
        steps = [
            {
                'phase': 'analysis',
                'description': 'Analyze requirements and design solution',
                'duration': '1-2 hours',
                'automated': True
            },
            {
                'phase': 'implementation',
                'description': 'Write the actual code',
                'duration': f"{task.estimated_effort}",
                'automated': True
            },
            {
                'phase': 'testing',
                'description': 'Create and run comprehensive tests',
                'duration': '2-4 hours',
                'automated': True
            },
            {
                'phase': 'documentation',
                'description': 'Generate documentation and comments',
                'duration': '1 hour',
                'automated': True
            }
        ]

    return steps

def _estimate_duration(task: TaskSpecification, steps: List[Dict[str, Any]]) -> str:
    """Estimate total duration for the execution plan."""
    total_hours = 0

    for step in steps:
        duration_str = step['duration']
        if 'hour' in duration_str:
            # Extract hour range and take average
            hours = re.findall(r'\d+', duration_str)
            if hours:
                total_hours += int(hours[0])
        elif 'day' in duration_str:
            days = re.findall(r'\d+', duration_str)
            if days:
                total_hours += int(days[0]) * 8  # Assume 8 hours per day

    if total_hours < 8:
        return f"{total_hours} hours"
    elif total_hours < 40:
        return f"{total_hours // 8} days"
    else:
        return f"{total_hours // 40} weeks"

def _assess_risks(task: TaskSpecification, steps: List[Dict[str, Any]]) -> Dict[str, Any]:
    """Assess risks for the execution plan."""
    risks = {
        'technical_risks': [],
        'timeline_risks': [],
        'resource_risks': [],
        'external_dependencies': []
    }

    # Technical complexity risks
    if task.complexity in [ComplexityLevel.COMPLEX, ComplexityLevel.EXPERT]:
        risks['technical_risks'].append('High technical complexity may cause delays')

    # Timeline risks
    if task.constraints.get('timeline') in ['ASAP', 'urgent', 'today']:
        risks['timeline_risks'].append('Tight timeline increases delivery risk')

    # External dependencies
    if 'external' in task.raw_input.lower() or 'third-party' in task.raw_input.lower():
        risks['external_dependencies'].append('External dependencies may cause delays')

    return risks

def _generate_fallbacks(task: TaskSpecification, risks: Dict[str, Any]) -> List[Dict[str, Any]]:
    """Generate fallback strategies for high-risk scenarios."""
    fallbacks = []

    if risks['technical_risks']:
        fallbacks.append({
            'trigger': 'Technical blockers encountered',
            'action': 'Break down into smaller tasks and implement incrementally',
            'resources': 'Additional expert consultation'
        })

    if risks['timeline_risks']:
        fallbacks.append({
            'trigger': 'Timeline slippage',
            'action': 'Prioritize core functionality and defer nice-to-haves',
            'resources': 'Additional team members'
        })

    if risks['external_dependencies']:
        fallbacks.append({
            'trigger': 'External dependency issues',
            'action': 'Develop internal alternatives or local implementations',
            'resources': 'Alternative solutions research'
        })

    return fallbacks

def _define_success_metrics(task: TaskSpecification) -> List[str]:
    """Define measurable success metrics."""
    metrics = [
        "Task completes without critical errors",
        "All success criteria are met",
        f"Code quality meets {task.complexity.value} standards"
    ]

    if task.task_type == TaskType.CODE_GENERATION:
        metrics.extend([
            "Unit test coverage > 80%",
            "No critical security vulnerabilities",
            "Performance meets requirements"
        ])
    elif task.task_type == TaskType.RESEARCH:
        metrics.extend([
            "Findings validated across multiple sources",
            "Recommendations are actionable",
            "Conclusions are well-supported"
        ])

    return metrics

# Main execution function
def parse_and_plan_claudecode_task(user_input: str) -> Tuple[TaskSpecification, ExecutionPlan]:
    """
    Complete ClaudeCode-style task parsing and planning pipeline.
    """
    logger.info("Starting ClaudeCode-style task analysis...")

    # Parse the natural language task
    parser = ClaudeCodeTaskParser()
    task_spec = parser.parse_task(user_input)

    # Create execution plan
    execution_plan = create_execution_plan(task_spec)

    logger.info(f"Task analysis complete: {task_spec.task_type.value} - {task_spec.complexity.value}")
    logger.info(f"Estimated effort: {task_spec.estimated_effort}")
    logger.info(f"Execution plan: {len(execution_plan.steps)} steps, {execution_plan.estimated_duration}")

    return task_spec, execution_plan

if __name__ == "__main__":
    import sys

    if len(sys.argv) < 2:
        print("Usage: python claudecode_task_parser.py 'your natural language task description'")
        sys.exit(1)

    user_task = sys.argv[1]
    task_spec, execution_plan = parse_and_plan_claudecode_task(user_task)

    # Output results
    print("\n" + "="*60)
    print("CLAUDECODE TASK ANALYSIS RESULTS")
    print("="*60)
    print(f"Task Type: {task_spec.task_type.value}")
    print(f"Intent: {task_spec.intent}")
    print(f"Complexity: {task_spec.complexity.value}")
    print(f"Estimated Effort: {task_spec.estimated_effort}")
    print(f"Suggested Approach: {task_spec.suggested_approach}")
    print(f"\nRequirements ({len(task_spec.requirements)}):")
    for i, req in enumerate(task_spec.requirements, 1):
        print(f"  {i}. {req}")
    print(f"\nSuccess Criteria ({len(task_spec.success_criteria)}):")
    for i, criteria in enumerate(task_spec.success_criteria, 1):
        print(f"  {i}. {criteria}")
    print(f"\nExecution Plan ({len(execution_plan.steps)} steps):")
    for i, step in enumerate(execution_plan.steps, 1):
        print(f"  {i}. {step['phase']}: {step['description']} ({step['duration']})")

    print(f"\nEstimated Total Duration: {execution_plan.estimated_duration}")
    print(f"Success Metrics: {len(execution_plan.success_metrics)} defined")