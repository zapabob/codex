# Plan Mode Reference Guide

This document provides detailed reference information for advanced planning techniques and methodologies.

## Advanced Planning Techniques

### Critical Path Method (CPM)
Identify the longest path of dependent tasks:

**CPM Calculation**:
1. List all tasks with durations
2. Identify dependencies
3. Calculate earliest start/finish times
4. Calculate latest start/finish times
5. Identify critical path (tasks with zero slack)

**Example**:
```
Task A: 3 days
Task B: 2 days (depends on A)
Task C: 4 days (depends on A)
Task D: 1 day (depends on B,C)

Critical Path: A → C → D (8 days total)
```

### Program Evaluation and Review Technique (PERT)
Handle uncertainty in task duration estimates:

**PERT Formula**:
```
Expected Duration = (O + 4M + P) / 6
Where:
- O = Optimistic estimate
- M = Most likely estimate
- P = Pessimistic estimate
```

**Example**:
```
Task: Implement user authentication
- Optimistic: 2 days
- Most likely: 5 days
- Pessimistic: 12 days

Expected Duration: (2 + 4×5 + 12) / 6 = 6.33 days
```

### Monte Carlo Simulation
Model project uncertainty:

**Process**:
1. Define task duration ranges
2. Run thousands of simulations
3. Analyze probability distributions
4. Identify risk scenarios

**Benefits**:
- Quantify project risk
- Provide confidence intervals
- Support better decision making

## Agile Planning Frameworks

### Scrum
Structured framework for iterative development:

**Roles**:
- **Product Owner**: Defines requirements and priorities
- **Scrum Master**: Facilitates process and removes impediments
- **Development Team**: Self-organizing team that delivers increments

**Ceremonies**:
- **Sprint Planning**: Plan work for upcoming sprint
- **Daily Scrum**: Daily coordination and progress check
- **Sprint Review**: Demonstrate completed work
- **Sprint Retrospective**: Improve process and practices

### Kanban
Visualize workflow and limit work in progress:

**Core Principles**:
- Visualize the workflow
- Limit work in progress (WIP)
- Manage flow
- Make process policies explicit
- Use feedback loops to improve

**Kanban Board**:
```
Backlog | Ready | In Progress | Review | Done
--------|-------|-------------|--------|------
Item 1  | Item 4| Item 2      | Item 3 | Item 5
Item 6  |       |             |        |
```

### Extreme Programming (XP)
Focus on engineering excellence:

**Practices**:
- **Pair Programming**: Two developers work together
- **Test-Driven Development**: Write tests before code
- **Continuous Integration**: Frequent integration and testing
- **Refactoring**: Continuously improve code design
- **Simple Design**: Keep design as simple as possible

## Estimation Techniques

### Planning Poker
Consensus-based estimation:

**Process**:
1. Product Owner presents user story
2. Team discusses and asks questions
3. Each member selects a card (Fibonacci sequence)
4. Reveal cards simultaneously
5. Discuss discrepancies and re-vote if needed

**Card Values**: 0, 1, 2, 3, 5, 8, 13, 20, 40, 100, ?

### Three-Point Estimation
Account for uncertainty:

**Technique**:
- **Most Likely**: Expected duration
- **Optimistic**: Best case scenario
- **Pessimistic**: Worst case scenario

**Weighted Average**: (O + 4M + P) / 6

### Story Points vs Hours
Relative vs absolute estimation:

**Story Points**:
- Relative sizing (not time-based)
- Account for complexity, effort, uncertainty
- Fibonacci sequence: 1, 2, 3, 5, 8, 13, 21...
- Team calibrates points to velocity

**Hours**:
- Absolute time estimates
- Familiar for traditional project management
- Can be more precise for known tasks
- Susceptible to padding and optimism bias

## Risk Management Frameworks

### Risk Breakdown Structure (RBS)
Categorize risks hierarchically:

```
Project Risks
├── Technical Risks
│   ├── Technology Selection
│   ├── Integration Challenges
│   ├── Performance Issues
│   └── Security Vulnerabilities
├── Schedule Risks
│   ├── Resource Constraints
│   ├── Dependency Delays
│   └── Scope Changes
├── Budget Risks
│   ├── Cost Overruns
│   ├── Vendor Issues
│   └── Currency Fluctuations
└── Quality Risks
    ├── Requirements Issues
    ├── Testing Gaps
    └── User Acceptance Problems
```

### Risk Probability and Impact Matrix
Prioritize risks based on likelihood and consequences:

```
High Impact    | High Probability: Critical (Immediate action)
               | Low Probability: High (Monitor closely)
---------------|-------------------------------------------
Low Impact     | High Probability: Medium (Plan mitigation)
               | Low Probability: Low (Accept or ignore)
```

### Risk Response Strategies

**Avoid**: Change plan to eliminate threat
**Transfer**: Shift risk to third party (insurance, outsourcing)
**Mitigate**: Reduce probability or impact
**Accept**: Acknowledge risk and prepare contingency
**Exploit**: Take advantage of positive risk opportunity
**Enhance**: Increase probability/impact of opportunity
**Share**: Allocate ownership to third party
**Ignore**: No active management

## Quality Management

### Capability Maturity Model Integration (CMMI)
Process improvement framework:

**Maturity Levels**:
1. **Initial**: Unpredictable, poorly controlled
2. **Managed**: Basic project management processes
3. **Defined**: Standard, consistent processes
4. **Quantitatively Managed**: Measured and controlled
5. **Optimizing**: Focus on continuous improvement

### Six Sigma
Quality management methodology:

**DMAIC Process**:
1. **Define**: Define problem and goals
2. **Measure**: Measure current performance
3. **Analyze**: Identify root causes
4. **Improve**: Implement solutions
5. **Control**: Maintain improvements

**DMADV Process** (Design):
1. **Define**: Define requirements
2. **Measure**: Measure CTQs
3. **Analyze**: Analyze alternatives
4. **Design**: Design optimal solution
5. **Verify**: Verify design performance

## Communication Planning

### Stakeholder Analysis
Identify and prioritize stakeholders:

**Power/Interest Grid**:
```
High Power     | High Interest: Manage Closely
               | Low Interest: Keep Satisfied
---------------|-------------------------------
Low Power      | High Interest: Keep Informed
               | Low Interest: Monitor
```

### Communication Plan Template

| Stakeholder | Information Needed | Frequency | Format | Responsible |
|-------------|-------------------|-----------|--------|-------------|
| Executive Team | High-level progress, risks | Weekly | Email/Slide deck | Project Manager |
| Development Team | Technical details, blockers | Daily | Standup/Issues | Scrum Master |
| QA Team | Requirements, defects | As needed | Issues/Meetings | QA Lead |
| Customers | Feature updates, timelines | Bi-weekly | Demo/Newsletter | Product Owner |

## Change Management

### Change Control Process
Manage scope changes systematically:

1. **Change Request**: Submit formal change request
2. **Impact Assessment**: Evaluate schedule, cost, quality impacts
3. **Approval Decision**: Review and approve/reject change
4. **Implementation**: Update plans and communicate changes
5. **Verification**: Confirm change implemented correctly

### Configuration Management
Control changes to project artifacts:

**Key Activities**:
- **Identification**: Uniquely identify configuration items
- **Control**: Manage changes through formal process
- **Auditing**: Verify conformance to specifications
- **Reporting**: Report status of configuration items

## Performance Measurement

### Earned Value Management (EVM)
Measure project performance objectively:

**Key Metrics**:
- **Planned Value (PV)**: Budgeted cost of work planned
- **Earned Value (EV)**: Budgeted cost of work performed
- **Actual Cost (AC)**: Actual cost incurred

**Performance Indicators**:
- **Schedule Variance (SV)**: EV - PV
- **Cost Variance (CV)**: EV - AC
- **Schedule Performance Index (SPI)**: EV / PV
- **Cost Performance Index (CPI)**: EV / AC

### Forecasting
Predict final project outcomes:

**Estimate at Completion (EAC)**:
- If future performance same as past: EAC = AC + (BAC - EV)
- If future performance at planned rate: EAC = AC + (BAC - EV) / CPI

**Estimate to Complete (ETC)**: EAC - AC

Where BAC = Budget at Completion

## Scaling Considerations

### Program Management
Manage multiple related projects:

**Program vs Project**:
- **Project**: Temporary endeavor for unique product/service
- **Program**: Group of related projects managed together
- **Portfolio**: Collection of programs/projects for strategic objectives

### Enterprise Project Management
Large-scale project management:

**Key Challenges**:
- **Complexity**: Multiple stakeholders, dependencies, constraints
- **Governance**: Compliance, standards, oversight
- **Culture**: Organizational change, alignment
- **Technology**: Tools, integration, automation

### Agile at Scale
Scaling agile methodologies:

**Frameworks**:
- **SAFe (Scaled Agile Framework)**: Comprehensive scaling framework
- **LeSS (Large-Scale Scrum)**: Minimal scaling approach
- **DaD (Disciplined Agile Delivery)**: Goal-driven delivery
- **Spotify Model**: Tribe/squad/squad organizational model

## Tools and Technologies

### Project Management Software
Modern tools for planning and tracking:

**Traditional PM Tools**:
- Microsoft Project: Comprehensive project planning
- Oracle Primavera: Enterprise project portfolio management
- Deltek Cobra: Earned value management

**Agile Tools**:
- Jira: Issue tracking and agile project management
- Azure DevOps: Integrated development and planning
- Rally Software: Enterprise agile planning

**Modern Tools**:
- Linear: Developer-focused issue tracking
- Notion: Flexible workspace and documentation
- Miro: Visual collaboration and planning
- Figma: Design and prototyping

### Automation and Integration

**CI/CD Integration**:
- Jenkins, GitLab CI, GitHub Actions
- Automated testing and deployment
- Quality gates and approvals

**API Integration**:
- RESTful APIs for tool integration
- Webhooks for event-driven automation
- Zapier/IFTTT for workflow automation

**AI-Powered Planning**:
- Predictive analytics for risk assessment
- Automated task estimation
- Intelligent resource allocation
- Natural language processing for requirements