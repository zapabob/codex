# CI/CD Integration Guide - Automated QA in DevOps Pipeline

## Overview

Comprehensive guide for integrating Codex QA Engineering system into various CI/CD platforms. Enable automated quality assurance at every stage of your development pipeline with mathematical optimization, security analysis, and performance monitoring.

## Supported CI/CD Platforms

### 1. GitHub Actions (Primary)

#### Basic Setup
```yaml
name: QA Pipeline
on: [push, pull_request]

jobs:
  qa-analysis:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0

    - name: Setup QA Environment
      run: |
        pip install -r tools/codex-supervisor/requirements.txt
        codex $skill-install https://github.com/zapabob/codex-qa-engineer-skill

    - name: Run QA Analysis
      run: python tools/premerge_qa_hook.py ${{ github.head_ref }} ${{ github.base_ref }}

    - name: Upload QA Report
      uses: actions/upload-artifact@v3
      with:
        name: qa-report
        path: merge-qa-reports/
```

#### Advanced Configuration
```yaml
name: Advanced QA Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    types: [opened, synchronize, ready_for_review]

env:
  QA_LEVEL: comprehensive
  QA_TIMEOUT: 600
  QA_MINIMUM_SCORE: 7.5

jobs:
  quality-gate:
    runs-on: ubuntu-latest
    outputs:
      qa-score: ${{ steps.qa.outputs.score }}
      can-merge: ${{ steps.qa.outputs.can_merge }}

    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0

    - name: Setup Python
      uses: actions/setup-python@v4
      with:
        python-version: '3.9'

    - name: Install Dependencies
      run: |
        pip install -r tools/codex-supervisor/requirements.txt
        pip install pytest pytest-cov mypy black

    - name: Security Scan
      run: |
        python -m pip install bandit safety
        bandit -r . -f json -o security-report.json || true
        safety check --json > safety-report.json || true

    - name: Code Quality Checks
      run: |
        python -m black --check --diff .
        python -m mypy . --ignore-missing-imports || true
        python -m pytest --cov=. --cov-report=xml --cov-fail-under=85

    - name: Run QA Engineering Analysis
      id: qa
      run: |
        echo "🔬 Running Comprehensive QA Analysis..."

        # Run QA analysis
        python tools/premerge_qa_hook.py origin/${{ github.base_ref }} origin/${{ github.head_ref }}

        # Extract QA score and merge status
        if [ -f "merge-qa-results.json" ]; then
          QA_SCORE=$(python -c "
          import json
          with open('merge-qa-results.json') as f:
              data = json.load(f)
          print(data['qa_report']['metrics']['code_quality'].replace('+', '').replace('-', ''))
          ")
          CAN_MERGE=$(python -c "
          import json
          with open('merge-qa-results.json') as f:
              data = json.load(f)
          print('true' if data['merge_allowed'] else 'false')
          ")
        else
          QA_SCORE="5.0"
          CAN_MERGE="false"
        fi

        echo "score=$QA_SCORE" >> $GITHUB_OUTPUT
        echo "can_merge=$CAN_MERGE" >> $GITHUB_OUTPUT

    - name: Upload QA Artifacts
      uses: actions/upload-artifact@v3
      with:
        name: qa-analysis-${{ github.run_id }}
        path: |
          merge-qa-reports/
          merge-qa-results.json
          security-report.json
          safety-report.json
          coverage.xml
          .coverage

  merge-gate:
    runs-on: ubuntu-latest
    needs: quality-gate
    if: github.event_name == 'pull_request'

    steps:
    - name: Check QA Gate Status
      run: |
        if [ "${{ needs.quality-gate.outputs.can-merge }}" != "true" ]; then
          echo "❌ QA gates failed - merge blocked"
          echo "QA Score: ${{ needs.quality-gate.outputs.qa-score }}"
          echo "Check the QA analysis artifacts for details"
          exit 1
        else
          echo "✅ QA gates passed - merge approved"
        fi

  deploy:
    runs-on: ubuntu-latest
    needs: [quality-gate, merge-gate]
    if: github.ref == 'refs/heads/main' && needs.quality-gate.outputs.can-merge == 'true'

    steps:
    - name: Deploy to Production
      run: |
        echo "🚀 Deploying to production..."
        # Add your deployment commands here
```

### 2. GitLab CI/CD

#### .gitlab-ci.yml Configuration
```yaml
stages:
  - test
  - qa
  - deploy

variables:
  QA_TIMEOUT: "600"
  QA_LEVEL: "comprehensive"

qa_analysis:
  stage: qa
  image: python:3.9
  before_script:
    - pip install -r tools/codex-supervisor/requirements.txt
    - pip install pytest pytest-cov mypy black bandit safety
  script:
    - echo "🔬 Running QA Engineering Analysis..."
    - python tools/premerge_qa_hook.py origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME origin/$CI_MERGE_REQUEST_SOURCE_BRANCH_NAME
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: coverage.xml
    paths:
      - merge-qa-reports/
      - merge-qa-results.json
    expire_in: 1 week
  coverage: '/(?i)total.*? (100(?:\.0+)?\%|[1-9]?\d(?:\.\d+)?\%)$/'
  only:
    - merge_requests

qa_gate:
  stage: qa
  image: python:3.9
  script:
    - |
      if [ -f "merge-qa-results.json" ]; then
        python -c "
        import json
        with open('merge-qa-results.json') as f:
          data = json.load(f)
        if not data.get('merge_allowed', False):
          print('❌ QA gates failed')
          exit(1)
        print('✅ QA gates passed')
        "
      else
        echo "❌ No QA results found"
        exit 1
  dependencies:
    - qa_analysis
  only:
    - merge_requests

deploy_staging:
  stage: deploy
  script:
    - echo "🚀 Deploying to staging..."
  dependencies:
    - qa_gate
  only:
    - develop

deploy_production:
  stage: deploy
  script:
    - echo "🚀 Deploying to production..."
  dependencies:
    - qa_gate
  only:
    - main
  when: manual
```

### 3. Jenkins Pipeline

#### Jenkinsfile Configuration
```groovy
pipeline {
    agent any

    environment {
        QA_TIMEOUT = '600'
        QA_LEVEL = 'comprehensive'
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Setup QA Environment') {
            steps {
                sh '''
                    python3 -m venv qa_env
                    source qa_env/bin/activate
                    pip install -r tools/codex-supervisor/requirements.txt
                    pip install pytest pytest-cov mypy black bandit safety
                '''
            }
        }

        stage('Security & Quality Checks') {
            parallel {
                stage('Security Scan') {
                    steps {
                        sh '''
                            source qa_env/bin/activate
                            bandit -r . -f json -o security-report.json || true
                            safety check --json > safety-report.json || true
                        '''
                    }
                }

                stage('Code Quality') {
                    steps {
                        sh '''
                            source qa_env/bin/activate
                            black --check --diff . || true
                            mypy . --ignore-missing-imports || true
                        '''
                    }
                }

                stage('Unit Tests') {
                    steps {
                        sh '''
                            source qa_env/bin/activate
                            pytest --cov=. --cov-report=xml --cov-report=html --cov-fail-under=85
                        '''
                    }
                }
            }
        }

        stage('QA Engineering Analysis') {
            steps {
                script {
                    if (env.CHANGE_TARGET) {
                        sh """
                            source qa_env/bin/activate
                            echo "🔬 Running QA Analysis: \${CHANGE_BRANCH} → \${CHANGE_TARGET}"
                            python tools/premerge_qa_hook.py origin/\${CHANGE_TARGET} origin/\${CHANGE_BRANCH}
                        """
                    } else {
                        sh '''
                            source qa_env/bin/activate
                            echo "🔬 Running QA Analysis: main branch"
                            python tools/premerge_qa_hook.py origin/main origin/main
                        '''
                    }
                }
            }
        }

        stage('QA Gate Check') {
            steps {
                script {
                    def qaPassed = sh(
                        script: '''
                            source qa_env/bin/activate
                            python -c "
                            import json
                            try:
                                with open('merge-qa-results.json') as f:
                                    data = json.load(f)
                                print('true' if data.get('merge_allowed', False) else 'false')
                            except:
                                print('false')
                            "
                        ''',
                        returnStdout: true
                    ).trim()

                    if (qaPassed == 'false') {
                        error("❌ QA gates failed - merge blocked")
                    }

                    echo "✅ QA gates passed"
                }
            }
        }

        stage('Deploy') {
            when {
                anyOf {
                    branch 'main'
                    branch 'master'
                }
            }
            steps {
                echo "🚀 Deploying to production..."
                // Add deployment commands
            }
        }
    }

    post {
        always {
            archiveArtifacts artifacts: 'merge-qa-reports/**,merge-qa-results.json,security-report.json,safety-report.json,coverage.xml', allowEmptyArchive: true
            publishCoverage adapters: [coberturaAdapter('coverage.xml')]
        }

        success {
            script {
                if (env.CHANGE_ID) {
                    // Notify PR success
                    echo "✅ QA Pipeline completed successfully"
                }
            }
        }

        failure {
            script {
                if (env.CHANGE_ID) {
                    // Notify PR failure
                    echo "❌ QA Pipeline failed"
                }
            }
        }
    }
}
```

### 4. CircleCI

#### .circleci/config.yml
```yaml
version: 2.1

executors:
  qa-executor:
    docker:
      - image: cimg/python:3.9
    working_directory: ~/repo

jobs:
  qa-analysis:
    executor: qa-executor
    steps:
      - checkout

      - restore_cache:
          keys:
            - qa-deps-{{ checksum "tools/codex-supervisor/requirements.txt" }}

      - run:
          name: Install QA Dependencies
          command: |
            pip install -r tools/codex-supervisor/requirements.txt
            pip install pytest pytest-cov mypy black bandit safety

      - save_cache:
          key: qa-deps-{{ checksum "tools/codex-supervisor/requirements.txt" }}
          paths:
            - ~/.cache/pip

      - run:
          name: Security Scan
          command: |
            bandit -r . -f json -o security-report.json || true
            safety check --json > safety-report.json || true

      - run:
          name: Code Quality Checks
          command: |
            black --check --diff . || true
            mypy . --ignore-missing-imports || true

      - run:
          name: Run Tests with Coverage
          command: |
            pytest --cov=. --cov-report=xml --cov-report=html --cov-fail-under=85

      - run:
          name: QA Engineering Analysis
          command: |
            if [ -n "$CIRCLE_PULL_REQUEST" ]; then
              # Extract branch names from PR URL
              PR_NUMBER=$(echo $CIRCLE_PULL_REQUEST | sed 's|.*/||')
              BASE_BRANCH=$(curl -s https://api.github.com/repos/$CIRCLE_PROJECT_USERNAME/$CIRCLE_PROJECT_REPONAME/pulls/$PR_NUMBER | jq -r '.base.ref')
              HEAD_BRANCH=$(curl -s https://api.github.com/repos/$CIRCLE_PROJECT_USERNAME/$CIRCLE_PROJECT_REPONAME/pulls/$PR_NUMBER | jq -r '.head.ref')
              python tools/premerge_qa_hook.py origin/$BASE_BRANCH origin/$HEAD_BRANCH
            else
              python tools/premerge_qa_hook.py origin/main origin/$CIRCLE_BRANCH
            fi

      - store_artifacts:
          path: merge-qa-reports/
          destination: qa-reports

      - store_artifacts:
          path: merge-qa-results.json
          destination: qa-results.json

      - store_artifacts:
          path: security-report.json
          destination: security-report.json

      - store_test_results:
          path: test-results

      - run:
          name: QA Gate Check
          command: |
            if [ -f "merge-qa-results.json" ]; then
              python -c "
              import json, sys
              with open('merge-qa-results.json') as f:
                  data = json.load(f)
              if not data.get('merge_allowed', False):
                  print('❌ QA gates failed')
                  sys.exit(1)
              print('✅ QA gates passed')
              "
            else
              echo "❌ No QA results found"
              exit 1

workflows:
  version: 2
  qa-workflow:
    jobs:
      - qa-analysis:
          filters:
            branches:
              only:
                - main
                - develop
                - /feature\/.*/

  nightly-qa:
    triggers:
      - schedule:
          cron: "0 2 * * *"  # Daily at 2 AM UTC
          filters:
            branches:
              only:
                - main
    jobs:
      - qa-analysis
```

### 5. Azure DevOps

#### azure-pipelines.yml
```yaml
trigger:
  branches:
    include:
    - main
    - develop
  paths:
    exclude:
    - docs/
    - README.md

pr:
  branches:
    include:
    - main
    - develop

pool:
  vmImage: 'ubuntu-latest'

variables:
  QA_TIMEOUT: 600
  QA_LEVEL: comprehensive
  pythonVersion: '3.9'

stages:
- stage: Test
  jobs:
  - job: UnitTests
    steps:
    - task: UsePythonVersion@0
      inputs:
        versionSpec: '$(pythonVersion)'

    - script: |
        pip install pytest pytest-cov
        pytest --cov=. --cov-report=xml --cov-fail-under=85
      displayName: 'Run Unit Tests'

    - task: PublishTestResults@2
      condition: succeededOrFailed()
      inputs:
        testResultsFiles: 'test-results.xml'
        testRunTitle: 'Unit Tests'

    - task: PublishCodeCoverageResults@1
      inputs:
        codeCoverageTool: Cobertura
        summaryFileLocation: '$(System.DefaultWorkingDirectory)/coverage.xml'

- stage: QA
  dependsOn: Test
  jobs:
  - job: QualityAnalysis
    steps:
    - task: UsePythonVersion@0
      inputs:
        versionSpec: '$(pythonVersion)'

    - script: |
        pip install -r tools/codex-supervisor/requirements.txt
        pip install mypy black bandit safety
      displayName: 'Install QA Dependencies'

    - script: |
        echo "🔬 Running QA Engineering Analysis..."
        if [ -n "$(System.PullRequest.PullRequestId)" ]; then
          python tools/premerge_qa_hook.py origin/$(System.PullRequest.TargetBranch) origin/$(Build.SourceBranchName)
        else
          python tools/premerge_qa_hook.py origin/main origin/$(Build.SourceBranchName)
        fi
      displayName: 'Run QA Analysis'

    - task: PublishBuildArtifacts@1
      displayName: 'Publish QA Reports'
      inputs:
        pathtoPublish: 'merge-qa-reports'
        artifactName: 'qa-reports'

    - script: |
        if [ -f "merge-qa-results.json" ]; then
          python -c "
          import json, sys
          with open('merge-qa-results.json') as f:
              data = json.load(f)
          if not data.get('merge_allowed', False):
              print('##vso[task.logissue type=error]QA gates failed - merge blocked')
              sys.exit(1)
          print('##vso[task.logissue type=warning]QA gates passed')
          "
        else
          echo "##vso[task.logissue type=error]No QA results found"
          exit 1
      displayName: 'QA Gate Check'

- stage: Deploy
  dependsOn: QA
  condition: and(succeeded(), eq(variables['Build.SourceBranch'], 'refs/heads/main'))
  jobs:
  - job: DeployProduction
    steps:
    - script: |
        echo "🚀 Deploying to production..."
        # Add your deployment commands here
      displayName: 'Deploy to Production'
```

## Notification Integration

### Slack Integration
```yaml
# GitHub Actions
- name: Send Slack Notification
  uses: 8398a7/action-slack@v3
  if: always()
  with:
    status: ${{ job.status }}
    text: |
      🔬 QA Analysis Results for ${{ github.event.pull_request.title }}
      Status: ${{ job.status }}
      <${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}|View Details>
    channel: '#qa-notifications'
  env:
    SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK_URL }}
```

### Discord Integration
```yaml
# GitHub Actions
- name: Send Discord Notification
  uses: Ilshidur/action-discord@master
  if: always()
  with:
    args: |
      🔬 **QA Analysis Complete**
      **PR:** ${{ github.event.pull_request.title }}
      **Status:** ${{ job.status }}
      **Details:** <${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}>
    channel: 'qa-alerts'
  env:
    DISCORD_WEBHOOK: ${{ secrets.DISCORD_WEBHOOK }}
```

### Email Integration
```yaml
# GitHub Actions
- name: Send Email Notification
  if: always()
  run: |
    python tools/premerge_qa_hook.py \
      --email-smtp-server ${{ secrets.SMTP_SERVER }} \
      --email-smtp-port ${{ secrets.SMTP_PORT }} \
      --email-username ${{ secrets.SMTP_USERNAME }} \
      --email-password ${{ secrets.SMTP_PASSWORD }} \
      --email-from ${{ secrets.EMAIL_FROM }} \
      --email-to ${{ secrets.EMAIL_TO }} \
      --notify-on-failure \
      ${{ github.head_ref }} ${{ github.base_ref }}
```

## Quality Gates Configuration

### GitHub Branch Protection Rules
```json
{
  "required_status_checks": {
    "strict": true,
    "contexts": [
      "qa-analysis",
      "qa-gate-check"
    ]
  },
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": true
  },
  "restrictions": null,
  "allow_force_pushes": false,
  "allow_deletions": false
}
```

### Custom QA Gate Rules
```json
{
  "qa_gates": {
    "block_on_critical_issues": true,
    "block_on_high_issues": false,
    "require_minimum_score": 7.0,
    "max_qa_time": 300,
    "required_checks": [
      "algorithmic_complexity",
      "security",
      "performance"
    ]
  }
}
```

## Monitoring and Analytics

### QA Metrics Dashboard
```yaml
# GitHub Actions - Upload to dashboard
- name: Upload QA Metrics
  if: always()
  run: |
    # Upload metrics to your dashboard service
    curl -X POST ${{ secrets.METRICS_ENDPOINT }} \
      -H "Content-Type: application/json" \
      -d @merge-qa-results.json
```

### Trend Analysis
```yaml
# Weekly QA trends
- name: Generate QA Trends
  if: github.event_name == 'schedule'
  run: |
    python tools/qa_metrics_analyzer.py \
      --generate-trends \
      --period 30d \
      --output qa-trends-report.md
```

## Troubleshooting

### Common Issues

#### QA Analysis Timeout
```yaml
# Increase timeout
env:
  QA_TIMEOUT: 900  # 15 minutes

# Or optimize analysis scope
- name: Run Targeted QA
  run: |
    QA_LEVEL=standard python tools/premerge_qa_hook.py source target
```

#### False Positives in QA
```yaml
# Configure QA rules
- name: Custom QA Configuration
  run: |
    cat > qa-config.json << EOF
    {
      "exclude_patterns": ["test_*", "mock_*"],
      "custom_rules": {
        "max_complexity": 15,
        "allow_weak_crypto_in_tests": true
      }
    }
    EOF
    export QA_CONFIG_FILE=qa-config.json
```

#### Integration Conflicts
```yaml
# Run QA in isolation
- name: Isolated QA Analysis
  run: |
    git stash  # Stash local changes
    python tools/premerge_qa_hook.py origin/main origin/feature-branch
    git stash pop  # Restore changes
```

#### Performance Issues
```yaml
# Optimize for speed
- name: Fast QA Check
  run: |
    QA_LEVEL=basic QA_TIMEOUT=120 python tools/premerge_qa_hook.py source target

# Parallel execution
- name: Parallel QA
  run: |
    python tools/worktree_manager.py create qa-check qa-$(date +%s)
    python tools/worktree_manager.py qa qa-check &
    # Continue with other tasks
```

## Best Practices

### Pipeline Optimization
1. **Parallel Execution**: Run security scans and QA analysis in parallel
2. **Caching**: Cache dependencies and QA environments
3. **Incremental Analysis**: Only analyze changed files when possible
4. **Fail Fast**: Stop pipeline on critical issues immediately

### Quality Gate Strategy
1. **Multiple Levels**: Basic → Standard → Comprehensive
2. **Branch-specific Rules**: Stricter rules for main branch
3. **Escalation Paths**: Allow overrides with proper approval
4. **Feedback Loop**: Use QA results to improve development practices

### Monitoring and Improvement
1. **Track Metrics**: Monitor QA pass rates and analysis times
2. **False Positive Reduction**: Refine QA rules based on feedback
3. **Performance Tuning**: Optimize QA analysis for your codebase
4. **Team Training**: Use QA results to improve code quality awareness

## Integration Examples

### Monorepo Support
```yaml
# Analyze specific packages
- name: QA Analysis per Package
  run: |
    for package in packages/*; do
      if [ -d "$package" ]; then
        echo "🔬 Analyzing $package"
        cd "$package"
        python ../../tools/premerge_qa_hook.py origin/main origin/feature-branch
        cd -
      fi
    done
```

### Multi-language Support
```yaml
# Language-specific QA
- name: Python QA
  if: contains(github.event.pull_request.changed_files, '.py')
  run: python tools/qa_python_analyzer.py

- name: Rust QA
  if: contains(github.event.pull_request.changed_files, '.rs')
  run: python tools/qa_rust_analyzer.py
```

### Custom Integrations
```yaml
# Integrate with your existing tools
- name: Send to Jira
  run: |
    python tools/jira_integration.py \
      --qa-results merge-qa-results.json \
      --create-tickets

- name: Update Documentation
  run: |
    python tools/docs_updater.py \
      --qa-report merge-qa-reports/ \
      --update-api-docs
```

---

**Compatibility**: All major CI/CD platforms
**Languages**: Python, Rust, JavaScript/TypeScript, and extensible
**Integration**: Webhooks, APIs, notifications, dashboards

This guide provides comprehensive integration patterns for automated QA in your DevOps pipeline! 🚀