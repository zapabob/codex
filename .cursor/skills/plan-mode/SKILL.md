---
name: plan-mode
description: Create comprehensive project plans with task decomposition, execution strategies, and progress tracking. Use when planning complex projects, breaking down large tasks, or managing multi-step workflows with dependencies and timelines.
---

# Plan Mode

This skill enables Cursor/ClaudeCode-style advanced project planning and execution management. It transforms complex projects into structured, actionable plans with intelligent task decomposition, dependency mapping, and progress tracking.

## Core Planning Workflow

### 1. Project Analysis
When planning any complex project:

1. **Scope Assessment**: Understand the full scope and objectives
2. **Stakeholder Identification**: Identify all parties involved and their roles
3. **Constraint Analysis**: Determine time, budget, resource, and quality constraints
4. **Risk Assessment**: Identify potential blockers and mitigation strategies

### 2. Task Decomposition
Break down projects into manageable tasks:

**Technique**: Use the "divide and conquer" approach:
- Start with the main objective
- Break it into 3-7 major phases
- Decompose each phase into specific, actionable tasks
- Ensure each task is completable in 1-3 days

**Task Template**:
```markdown
## [Task Name]
**Priority**: [High/Medium/Low]
**Estimated Time**: [X hours/days]
**Dependencies**: [List of prerequisite tasks]
**Success Criteria**: [Measurable completion indicators]
**Owner**: [Responsible person/team]
```

### 3. Dependency Mapping
Map task relationships and execution order:

**Types of Dependencies**:
- **Finish-to-Start**: Task B can't start until Task A finishes
- **Start-to-Start**: Task B can't start until Task A starts
- **Finish-to-Finish**: Task B can't finish until Task A finishes
- **Start-to-Finish**: Task B can't finish until Task A starts

**Visualization**:
```
Task A ──► Task B ──► Task C
   │           │
   └─────► Task D ─────┘
```

### 4. Timeline Planning
Create realistic timelines with buffers:

**Buffer Guidelines**:
- Add 20-30% buffer for unexpected issues
- Include time for code reviews and testing
- Account for team availability and holidays
- Build in milestones for progress checkpoints

### 5. Risk Management
Proactively identify and mitigate risks:

**Risk Categories**:
- **Technical Risks**: Technology unfamiliarity, integration challenges
- **Resource Risks**: Team capacity, skill gaps, tool availability
- **Schedule Risks**: Underestimation, dependency delays
- **Quality Risks**: Requirements changes, testing gaps

## Planning Methodologies

### Agile Sprint Planning
For iterative development:

```markdown
## Sprint Planning Template

**Sprint Goal**: [Clear, measurable objective]

**Capacity**: [Available team hours]
**Selected Stories**: [Prioritized backlog items]

**Sprint Backlog**:
- [ ] Story 1: [Description] ([Story Points])
- [ ] Story 2: [Description] ([Story Points])
- [ ] Story 3: [Description] ([Story Points])

**Definition of Done**:
- Code written and unit tested
- Code reviewed and approved
- Acceptance criteria met
- Documentation updated
```

### Waterfall Phase Planning
For sequential, predictable projects:

```markdown
## Phase Gate Template

**Phase 1: Requirements** [Date Range]
- [ ] Requirements gathering complete
- [ ] Requirements document approved
- [ ] Acceptance criteria defined
- [ ] Phase gate review passed

**Phase 2: Design** [Date Range]
- [ ] Technical design complete
- [ ] Architecture review passed
- [ ] Design document approved
- [ ] Phase gate review passed

**Phase 3: Implementation** [Date Range]
- [ ] Code development complete
- [ ] Unit testing passed
- [ ] Integration testing passed
- [ ] Phase gate review passed
```

### Hybrid Planning
Combine methodologies for optimal results:

**When to Use Hybrid**:
- Large projects with some predictable elements
- Teams transitioning from Waterfall to Agile
- Projects with regulatory or compliance requirements
- Mixed teams with different working styles

## Execution Management

### Daily Progress Tracking
Maintain momentum with structured check-ins:

**Daily Standup Format**:
```markdown
**Yesterday**: What was completed
**Today**: What will be worked on
**Blockers**: Any obstacles or impediments
**Help Needed**: Support required from others
```

### Milestone Management
Track progress against major deliverables:

**Milestone Template**:
```markdown
## Milestone: [Name]
**Due Date**: [Date]
**Deliverables**:
- [ ] Deliverable 1: [Description]
- [ ] Deliverable 2: [Description]
- [ ] Deliverable 3: [Description]

**Success Criteria**:
- [ ] All deliverables completed
- [ ] Quality standards met
- [ ] Stakeholder approval received
- [ ] Documentation updated
```

### Issue Resolution
Handle problems systematically:

**Issue Resolution Process**:
1. **Identify**: Clearly define the problem
2. **Assess Impact**: Determine scope and severity
3. **Develop Solutions**: Brainstorm potential fixes
4. **Select Approach**: Choose best solution with trade-offs
5. **Implement**: Execute the chosen solution
6. **Verify**: Confirm the fix works
7. **Document**: Record the resolution for future reference

## Communication and Reporting

### Progress Reporting
Keep stakeholders informed with structured updates:

**Weekly Status Report Template**:
```markdown
## Weekly Status Report - Week [Number]

### Accomplishments This Week
- [ ] Completed task 1
- [ ] Completed task 2
- [ ] Milestone achieved: [Description]

### Planned for Next Week
- [ ] Task 1: [Description]
- [ ] Task 2: [Description]
- [ ] Milestone target: [Description]

### Risks and Issues
- [ ] Risk 1: [Description] - Mitigation: [Plan]
- [ ] Issue 1: [Description] - Resolution: [Status]

### Key Metrics
- Tasks Completed: [X/Y]
- Sprint Velocity: [Points]
- Quality Metrics: [Coverage %, Defect Rate]
```

### Stakeholder Management
Communicate effectively with different audiences:

**Communication Guidelines**:
- **Executives**: Focus on business impact and high-level progress
- **Team Members**: Provide detailed technical updates and blockers
- **Customers**: Emphasize features delivered and timelines
- **Vendors**: Highlight dependencies and coordination needs

## Quality Assurance

### Code Quality Standards
Maintain high code quality throughout:

**Quality Checklist**:
- [ ] Unit tests written and passing
- [ ] Code reviewed by peers
- [ ] Static analysis clean
- [ ] Security scan passed
- [ ] Performance requirements met
- [ ] Documentation updated

### Testing Strategy
Ensure comprehensive test coverage:

**Testing Pyramid**:
```
End-to-End Tests (Few)
    ↕
Integration Tests (Some)
    ↕
Unit Tests (Many)
```

### Continuous Integration
Automate quality checks:

**CI Pipeline Requirements**:
- Automated test execution
- Code quality analysis
- Security vulnerability scanning
- Performance regression testing
- Deployment automation

## Risk Mitigation Strategies

### Proactive Risk Management
Identify and address risks before they become issues:

**Risk Register Template**:
```markdown
| Risk | Probability | Impact | Mitigation | Owner | Status |
|------|-------------|--------|------------|-------|--------|
| Technology X unfamiliarity | Medium | High | Training + Expert consultation | [Name] | Monitoring |
| Third-party API dependency | Low | Medium | Develop fallback + Monitor status | [Name] | Mitigated |
```

### Contingency Planning
Prepare for worst-case scenarios:

**Contingency Plan Elements**:
- **Trigger Conditions**: When to activate the plan
- **Response Actions**: Specific steps to take
- **Resource Requirements**: Additional resources needed
- **Communication Plan**: Who to notify and how
- **Recovery Timeline**: Expected time to normal operations

## Tools and Automation

### Project Management Tools
Leverage tools for efficiency:

**Recommended Tools**:
- **Jira/Linear**: Issue tracking and sprint management
- **GitHub Projects**: Lightweight project tracking
- **Notion**: Documentation and knowledge management
- **Miro/Figma**: Visual planning and brainstorming

### Automation Opportunities
Streamline repetitive tasks:

**Automation Candidates**:
- Code quality checks (linting, formatting)
- Test execution and reporting
- Deployment pipelines
- Progress report generation
- Risk monitoring alerts

## Success Metrics

### Project Success Indicators
Measure project health and success:

**Key Metrics**:
- **On-Time Delivery**: Percentage of milestones met on schedule
- **Budget Adherence**: Actual vs. planned budget utilization
- **Quality Metrics**: Defect rates, test coverage, user satisfaction
- **Team Productivity**: Velocity, throughput, efficiency measures
- **Stakeholder Satisfaction**: Regular feedback and satisfaction scores

### Continuous Improvement
Learn from each project:

**Retrospective Questions**:
- What went well and should be repeated?
- What could be improved?
- What surprised us?
- What did we learn?
- What should we change for next time?

## Scaling for Large Projects

### Program Management
Coordinate multiple related projects:

**Program Management Elements**:
- **Program Charter**: Overall objectives and scope
- **Project Interdependencies**: How projects affect each other
- **Resource Pooling**: Shared resources across projects
- **Integrated Planning**: Coordinated timelines and milestones

### Enterprise Considerations
Scale planning for large organizations:

**Enterprise Planning Factors**:
- **Governance Requirements**: Compliance and approval processes
- **Stakeholder Management**: Large and diverse stakeholder groups
- **Change Management**: Managing organizational change
- **Portfolio Optimization**: Balancing competing priorities