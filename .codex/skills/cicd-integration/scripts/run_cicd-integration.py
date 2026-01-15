#!/usr/bin/env python3
"""
CI/CD Integration Agent - Multi-Platform Pipeline Orchestration
Generates and manages CI/CD pipelines for GitHub Actions, GitLab CI, Jenkins, etc.
"""

import os
import sys
import json
import yaml
from pathlib import Path
from typing import Dict, List, Any, Optional
from dataclasses import dataclass
from datetime import datetime

@dataclass
class CICDPipeline:
    name: str
    platform: str
    stages: List[str]
    quality_gates: List[str]
    environments: List[str]

@dataclass
class CICDConfig:
    platforms: List[str]
    project_name: str
    project_type: str  # rust, nodejs, python, etc.
    environments: List[str]
    quality_gates: List[str]
    notifications: Dict[str, Any]

class CICDGenerator:
    """Multi-platform CI/CD pipeline generator"""

    def __init__(self, config: CICDConfig):
        self.config = config

    def generate_pipelines(self) -> Dict[str, Any]:
        """Generate CI/CD pipelines for all configured platforms"""

        pipelines = {}

        if "github_actions" in self.config.platforms:
            pipelines["github_actions"] = self._generate_github_actions()

        if "gitlab_ci" in self.config.platforms:
            pipelines["gitlab_ci"] = self._generate_gitlab_ci()

        if "jenkins" in self.config.platforms:
            pipelines["jenkins"] = self._generate_jenkins()

        if "circleci" in self.config.platforms:
            pipelines["circleci"] = self._generate_circleci()

        if "azure_devops" in self.config.platforms:
            pipelines["azure_devops"] = self._generate_azure_devops()

        return pipelines

    def _generate_github_actions(self) -> Dict[str, Any]:
        """Generate GitHub Actions workflows"""

        workflows = {}

        # Main CI workflow
        workflows["ci.yml"] = self._create_github_ci_workflow()

        # QA workflow
        workflows["qa.yml"] = self._create_github_qa_workflow()

        # Deploy workflow
        workflows["deploy.yml"] = self._create_github_deploy_workflow()

        # Dependabot
        workflows["dependabot.yml"] = self._create_github_dependabot()

        return {
            "platform": "github_actions",
            "workflows": workflows,
            "directory": ".github/workflows"
        }

    def _create_github_ci_workflow(self) -> str:
        """Create GitHub Actions CI workflow"""

        workflow = f"""name: CI Pipeline

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]
    types: [opened, synchronize, reopened, ready_for_review]

concurrency:
  group: ci-${{{{ github.ref }}}}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1

jobs:
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust: [stable, beta]

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Install Rust
      uses: dtolnay/rust-toolchain@master
      with:
        toolchain: ${{matrix.rust}}

    - name: Cache dependencies
      uses: Swatinem/rust-cache@v2

    - name: Run tests
      run: cargo test --verbose --all-features

    - name: Run doctests
      run: cargo test --doc

    - name: Check formatting
      run: cargo fmt --all -- --check

    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings

    - name: Upload coverage
      uses: codecov/codecov-action@v3
      with:
        file: ./lcov.info
        flags: rust
        name: Rust Coverage

  security:
    name: Security Scan
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Run cargo audit
      uses: actions-rs/cargo@v1
      with:
        command: audit

    - name: Run cargo outdated
      run: cargo outdated --exit-code 1 || true

  qa:
    name: Quality Assurance
    runs-on: ubuntu-latest
    needs: test

    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      with:
        fetch-depth: 0

    - name: Setup Python
      uses: actions/setup-python@v4
      with:
        python-version: '3.9'

    - name: Install QA dependencies
      run: |
        pip install -r tools/codex-supervisor/requirements.txt
        # Install Codex QA Skills
        mkdir -p .codex/skills/qa-engineer/scripts
        cp tools/codex-supervisor/* .codex/skills/qa-engineer/ 2>/dev/null || true

    - name: Run QA Analysis
      id: qa
      run: |
        python tools/premerge_qa_hook.py origin/main origin/${{ github.head_ref || github.ref_name }}

        # Set output for next steps
        if [ -f "merge-qa-results.json" ]; then
          QA_PASSED=$(python -c "
          import json
          with open('merge-qa-results.json') as f:
              data = json.load(f)
          print('true' if data.get('merge_allowed', False) else 'false')
          ")
          echo "qa_passed=$QA_PASSED" >> $GITHUB_OUTPUT
        else
          echo "qa_passed=false" >> $GITHUB_OUTPUT
        fi

    - name: Upload QA Results
      uses: actions/upload-artifact@v3
      with:
        name: qa-results-${{{{ github.run_id }}}}
        path: |
          merge-qa-reports/
          merge-qa-results.json

  deploy-staging:
    name: Deploy to Staging
    runs-on: ubuntu-latest
    needs: [test, qa]
    if: github.ref == 'refs/heads/develop' && needs.qa.outputs.qa_passed == 'true'

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Build release
      run: cargo build --release

    - name: Deploy to staging
      run: |
        echo "🚀 Deploying to staging environment..."
        # Add your staging deployment commands here

  deploy-production:
    name: Deploy to Production
    runs-on: ubuntu-latest
    needs: [test, qa]
    if: github.ref == 'refs/heads/main' && needs.qa.outputs.qa_passed == 'true'
    environment: production

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Build release
      run: cargo build --release --all-features

    - name: Run integration tests
      run: cargo test --test integration

    - name: Deploy to production
      run: |
        echo "🚀 Deploying to production environment..."
        # Add your production deployment commands here

  notify:
    name: Send Notifications
    runs-on: ubuntu-latest
    needs: [test, qa]
    if: always()

    steps:
    - name: Send Slack notification
      uses: 8398a7/action-slack@v3
      with:
        status: ${{{{ job.status }}}}
        text: |
          🔬 CI Pipeline Results for ${{ github.repository }}
          Status: ${{{{ job.status }}}}
          Branch: ${{{{ github.ref_name }}}}
          <${{{{ github.server_url }}}}/${{{{ github.repository }}}}/actions/runs/${{{{ github.run_id }}}}|View Details>
      env:
        SLACK_WEBHOOK_URL: ${{{{ secrets.SLACK_WEBHOOK_URL }}}}
      continue-on-error: true
"""

        return workflow

    def _create_github_qa_workflow(self) -> str:
        """Create GitHub Actions QA workflow"""

        workflow = """name: QA Analysis

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

jobs:
  qa-analysis:
    name: Comprehensive QA Analysis
    runs-on: ubuntu-latest
    if: github.event.pull_request.draft == false

    strategy:
      matrix:
        python-version: [3.8, 3.9, '3.10']

    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      with:
        fetch-depth: 0
        token: ${{ secrets.GITHUB_TOKEN }}

    - name: Set up Python ${{ matrix.python-version }}
      uses: actions/setup-python@v4
      with:
        python-version: ${{ matrix.python-version }}

    - name: Cache pip dependencies
      uses: actions/cache@v3
      with:
        path: ~/.cache/pip
        key: ${{ runner.os }}-pip-${{ matrix.python-version }}-${{ hashFiles('**/requirements*.txt') }}

    - name: Install dependencies
      run: |
        python -m pip install --upgrade pip
        pip install -r tools/codex-supervisor/requirements.txt
        pip install pytest pytest-cov pytest-xdist mypy black isort flake8 bandit safety

    - name: Run QA Engineering Analysis
      id: qa-analysis
      run: |
        echo "🔬 Running Comprehensive QA Analysis..."

        # Get branch information
        if [ "${{ github.event_name }}" = "pull_request" ]; then
          SOURCE_BRANCH="${{ github.head_ref }}"
          TARGET_BRANCH="${{ github.base_ref }}"
        else
          SOURCE_BRANCH="${{ github.ref_name }}"
          TARGET_BRANCH="main"
        fi

        echo "Source branch: $SOURCE_BRANCH"
        echo "Target branch: $TARGET_BRANCH"

        # Run QA analysis
        python tools/premerge_qa_hook.py "$SOURCE_BRANCH" "$TARGET_BRANCH"
        QA_EXIT_CODE=$?

        echo "qa_exit_code=$QA_EXIT_CODE" >> $GITHUB_OUTPUT

        # Always generate report regardless of QA result
        if [ -f "merge-qa-results.json" ]; then
          echo "qa_report_exists=true" >> $GITHUB_OUTPUT
        else
          echo "qa_report_exists=false" >> $GITHUB_OUTPUT
        fi

    - name: Upload QA Report
      if: steps.qa-analysis.outputs.qa_report_exists == 'true'
      uses: actions/upload-artifact@v3
      with:
        name: qa-analysis-report-${{ matrix.python-version }}
        path: |
          merge-qa-reports/
          merge-qa-results.json
        retention-days: 30

    - name: Send Slack Notification
      if: always() && github.event_name == 'pull_request'
      uses: 8398a7/action-slack@v3
      with:
        status: ${{ job.status }}
        text: |
          🔬 QA Analysis Results for PR #${{ github.event.number }}
          Status: ${{ job.status }}
          <${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}|View Details>
        channel: '#qa-notifications'
      env:
        SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK_URL }}
      continue-on-error: true

    - name: Comment PR with QA Results
      if: always() && github.event_name == 'pull_request' && steps.qa-analysis.outputs.qa_report_exists == 'true'
      uses: actions/github-script@v6
      with:
        script: |
          const fs = require('fs');
          const path = require('path');

          // Read QA results
          let qaResults = {};
          try {
            const qaFile = fs.readFileSync('merge-qa-results.json', 'utf8');
            qaResults = JSON.parse(qaFile);
          } catch (error) {
            console.log('Could not read QA results file');
            return;
          }

          const qa = qaResults.qa_report;
          const evaluation = qaResults.evaluation;

          // Create comment body
          let comment = `## 🔬 QA Engineering Analysis Results

**Status:** ${evaluation.block_reasons.length > 0 ? '❌ Issues Found' : '✅ Passed'}

### 📊 Quality Scores
| Category | Score |
|----------|-------|
| Algorithmic Complexity | ${qa.metrics.algorithmic_complexity} |
| Quantum Optimization | ${qa.metrics.quantum_optimization} |
| Software Engineering | ${qa.metrics.software_engineering} |
| Code Quality | ${qa.metrics.code_quality} |
| Performance | ${qa.metrics.performance} |
| Security | ${qa.metrics.security} |

### 📈 Issues Summary
- **Total Issues:** ${qa.issues.length}
- **Critical:** ${qa.issues.filter(i => i.severity === 'CRITICAL').length}
- **High:** ${qa.issues.filter(i => i.severity === 'HIGH').length}

`;

          if (evaluation.block_reasons.length > 0) {
            comment += `### 🚫 Blocking Issues
${evaluation.block_reasons.map(reason => `- ${reason}`).join('\\n')}

`;
          }

          comment += `
### 🔗 Full Report
📄 [View Detailed Report](${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }})

---
*Generated by Codex QA Engineering System*`;

          // Post comment
          github.rest.issues.createComment({
            issue_number: context.issue.number,
            owner: context.repo.owner,
            repo: context.repo.repo,
            body: comment
          });
"""

        return workflow

    def _create_github_deploy_workflow(self) -> str:
        """Create GitHub Actions deployment workflow"""
        return """name: Deploy

on:
  push:
    branches: [ main ]
  workflow_dispatch:
    inputs:
      environment:
        description: 'Target environment'
        required: true
        default: 'staging'
        type: choice
        options:
        - staging
        - production

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  build-and-push:
    name: Build and Push Docker Image
    runs-on: ubuntu-latest

    permissions:
      contents: read
      packages: write

    steps:
    - name: Checkout repository
      uses: actions/checkout@v4

    - name: Log in to Container Registry
      uses: docker/login-action@v2
      with:
        registry: ${{ env.REGISTRY }}
        username: ${{ github.actor }}
        password: ${{ secrets.GITHUB_TOKEN }}

    - name: Extract metadata
      id: meta
      uses: docker/metadata-action@v4
      with:
        images: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}
        tags: |
          type=ref,event=branch
          type=ref,event=pr
          type=sha,prefix={{branch}}-
          type=raw,value=latest,enable={{is_default_branch}}

    - name: Build and push Docker image
      uses: docker/build-push-action@v4
      with:
        context: .
        push: true
        tags: ${{ steps.meta.outputs.tags }}
        labels: ${{ steps.meta.outputs.labels }}
        cache-from: type=gha
        cache-to: type=gha,mode=max

  deploy-staging:
    name: Deploy to Staging
    runs-on: ubuntu-latest
    needs: build-and-push
    if: github.ref == 'refs/heads/main' || github.event.inputs.environment == 'staging'

    environment: staging

    steps:
    - name: Deploy to staging
      run: |
        echo "🚀 Deploying ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest to staging"
        # Add your staging deployment commands here

  deploy-production:
    name: Deploy to Production
    runs-on: ubuntu-latest
    needs: [build-and-push, deploy-staging]
    if: github.event.inputs.environment == 'production'
    environment: production

    steps:
    - name: Deploy to production
      run: |
        echo "🚀 Deploying ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}:latest to production"
        # Add your production deployment commands here
"""

    def _create_github_dependabot(self) -> str:
        """Create GitHub Dependabot configuration"""
        return """version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10
    reviewers:
      - "zapabob"
    assignees:
      - "zapabob"
    commit-message:
      prefix: "deps"
      prefix-development: "deps-dev"
      include: "scope"

  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5
    reviewers:
      - "zapabob"
    assignees:
      - "zapabob"
    commit-message:
      prefix: "deps"
      prefix-development: "deps-dev"
      include: "scope"

  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5
    reviewers:
      - "zapabob"
    assignees:
      - "zapabob"
"""

    def _generate_gitlab_ci(self) -> Dict[str, Any]:
        """Generate GitLab CI configuration"""

        gitlab_ci = """stages:
  - test
  - qa
  - build
  - deploy

variables:
  CARGO_HOME: $CI_PROJECT_DIR/.cargo
  RUST_BACKTRACE: 1
  QA_TIMEOUT: "300"

cache:
  key: ${{ checksum "Cargo.lock" }}
  paths:
    - .cargo/
    - target/

test:
  stage: test
  image: rust:latest
  before_script:
    - rustc --version
    - cargo --version
  script:
    - cargo test --verbose --all-features
    - cargo test --doc
  coverage: '/(?i)total.*? (100(?:\\.0+)?\\%|\\d+(?:\\.\\d+)?\\%)$/'
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: coverage.xml
    expire_in: 1 week

lint:
  stage: test
  image: rust:latest
  script:
    - cargo fmt --all -- --check
    - cargo clippy --all-targets --all-features -- -D warnings

security:
  stage: test
  image: rust:latest
  script:
    - cargo install cargo-audit
    - cargo audit

qa:
  stage: qa
  image: python:3.9
  dependencies:
    - test
  before_script:
    - pip install -r tools/codex-supervisor/requirements.txt
  script:
    - python tools/premerge_qa_hook.py origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME origin/$CI_MERGE_REQUEST_SOURCE_BRANCH_NAME
  artifacts:
    reports:
      coverage_report:
        coverage_format: cobertura
        path: qa-coverage.xml
    paths:
      - merge-qa-reports/
      - merge-qa-results.json
    expire_in: 1 week
  only:
    - merge_requests

qa_gate:
  stage: qa
  image: python:3.9
  script:
    - |
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
  dependencies:
    - qa
  only:
    - merge_requests

build:
  stage: build
  image: rust:latest
  script:
    - cargo build --release --all-features
  artifacts:
    paths:
      - target/release/
    expire_in: 1 week
  only:
    - main
    - develop

deploy_staging:
  stage: deploy
  script:
    - echo "🚀 Deploying to staging..."
    # Add your staging deployment commands
  dependencies:
    - build
    - qa_gate
  only:
    - develop
  environment:
    name: staging
    url: https://staging.example.com

deploy_production:
  stage: deploy
  script:
    - echo "🚀 Deploying to production..."
    # Add your production deployment commands
  dependencies:
    - build
    - qa_gate
  only:
    - main
  environment:
    name: production
    url: https://example.com
  when: manual
"""

        return {
            "platform": "gitlab_ci",
            "config_file": ".gitlab-ci.yml",
            "content": gitlab_ci
        }

    def _generate_jenkins(self) -> Dict[str, Any]:
        """Generate Jenkins pipeline"""

        jenkinsfile = """pipeline {
    agent any

    environment {
        CARGO_HOME = '${env.WORKSPACE}/.cargo'
        RUST_BACKTRACE = '1'
        QA_TIMEOUT = '300'
    }

    options {
        timeout(time: 30, unit: 'MINUTES')
        disableConcurrentBuilds()
        buildDiscarder(logRotator(numToKeepStr: '10'))
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Setup') {
            steps {
                sh '''
                    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                    source ~/.cargo/env
                    rustc --version
                    cargo --version
                '''
            }
        }

        stage('Test') {
            steps {
                sh '''
                    source ~/.cargo/env
                    cargo test --verbose --all-features
                    cargo test --doc
                '''
            }
            post {
                always {
                    publishCoverage adapters: [coberturaAdapter('coverage.xml')]
                }
            }
        }

        stage('Lint') {
            steps {
                sh '''
                    source ~/.cargo/env
                    cargo fmt --all -- --check
                    cargo clippy --all-targets --all-features -- -D warnings
                '''
            }
        }

        stage('Security') {
            steps {
                sh '''
                    source ~/.cargo/env
                    cargo install cargo-audit
                    cargo audit
                '''
            }
        }

        stage('QA Analysis') {
            when {
                anyOf {
                    branch 'main'
                    branch 'develop'
                    changeRequest()
                }
            }
            steps {
                sh '''
                    python3 -m venv qa_env
                    source qa_env/bin/activate
                    pip install -r tools/codex-supervisor/requirements.txt

                    if [ -n "$CHANGE_TARGET" ]; then
                        python tools/premerge_qa_hook.py origin/$CHANGE_TARGET origin/$BRANCH_NAME
                    else
                        python tools/premerge_qa_hook.py origin/main origin/$BRANCH_NAME
                    fi
                '''
            }
            post {
                always {
                    archiveArtifacts artifacts: 'merge-qa-reports/**,merge-qa-results.json', allowEmptyArchive: true
                }
            }
        }

        stage('QA Gate') {
            when {
                changeRequest()
            }
            steps {
                script {
                    def qaPassed = sh(
                        script: '''
                            python3 -c "
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

                    if (qaPassed != 'true') {
                        error("❌ QA gates failed - merge blocked")
                    }

                    echo "✅ QA gates passed"
                }
            }
        }

        stage('Build') {
            steps {
                sh '''
                    source ~/.cargo/env
                    cargo build --release --all-features
                '''
            }
            post {
                always {
                    archiveArtifacts artifacts: 'target/release/**', allowEmptyArchive: true
                }
            }
        }

        stage('Deploy Staging') {
            when {
                branch 'develop'
            }
            steps {
                sh '''
                    echo "🚀 Deploying to staging environment..."
                    # Add your staging deployment commands here
                '''
            }
        }

        stage('Deploy Production') {
            when {
                branch 'main'
            }
            steps {
                input message: 'Deploy to production?', ok: 'Deploy'
                sh '''
                    echo "🚀 Deploying to production environment..."
                    # Add your production deployment commands here
                '''
            }
        }
    }

    post {
        always {
            sh '''
                # Cleanup
                rm -rf qa_env
                cargo clean
            '''

            // Send notifications
            script {
                def status = currentBuild.currentResult ?: 'SUCCESS'
                def color = status == 'SUCCESS' ? 'good' : 'danger'

                slackSend(
                    color: color,
                    message: """
                    🔬 Jenkins Pipeline Results
                    Job: ${env.JOB_NAME} #${env.BUILD_NUMBER}
                    Status: ${status}
                    Branch: ${env.BRANCH_NAME}
                    Duration: ${currentBuild.durationString}
                    <${env.BUILD_URL}|View Build>
                    """.stripIndent()
                )
            }
        }

        success {
            echo "✅ Pipeline completed successfully"
        }

        failure {
            echo "❌ Pipeline failed"
        }

        unstable {
            echo "⚠️ Pipeline completed with warnings"
        }
    }
}
"""

        return {
            "platform": "jenkins",
            "config_file": "Jenkinsfile",
            "content": jenkinsfile
        }

    def _generate_circleci(self) -> Dict[str, Any]:
        """Generate CircleCI configuration"""

        circleci_config = """version: 2.1

executors:
  rust-executor:
    docker:
      - image: cimg/rust:1.70
    working_directory: ~/repo

  python-executor:
    docker:
      - image: cimg/python:3.9
    working_directory: ~/repo

commands:
  setup_rust:
    steps:
      - run:
          name: Install Rust
          command: |
            rustup install stable
            rustup default stable
            rustc --version
            cargo --version

  setup_qa:
    steps:
      - run:
          name: Setup QA Environment
          command: |
            python3 -m venv qa_env
            source qa_env/bin/activate
            pip install -r tools/codex-supervisor/requirements.txt

workflows:
  ci:
    jobs:
      - test:
          filters:
            branches:
              only:
                - main
                - develop
      - qa:
          requires:
            - test
          filters:
            branches:
              only:
                - main
                - develop
      - deploy-staging:
          requires:
            - qa
          filters:
            branches:
              only:
                - develop
      - deploy-production:
          requires:
            - qa
          filters:
            branches:
              only:
                - main

jobs:
  test:
    executor: rust-executor
    steps:
      - checkout

      - setup_rust

      - restore_cache:
          keys:
            - cargo-cache-{{ checksum "Cargo.lock" }}
            - cargo-cache-

      - run:
          name: Run Tests
          command: cargo test --verbose --all-features

      - run:
          name: Run Doctests
          command: cargo test --doc

      - run:
          name: Check Formatting
          command: cargo fmt --all -- --check

      - run:
          name: Run Clippy
          command: cargo clippy --all-targets --all-features -- -D warnings

      - save_cache:
          key: cargo-cache-{{ checksum "Cargo.lock" }}
          paths:
            - ~/.cargo/registry
            - target/debug

      - store_test_results:
          path: test-results.xml

  qa:
    executor: python-executor
    steps:
      - checkout

      - setup_qa

      - run:
          name: Run QA Analysis
          command: |
            source qa_env/bin/activate
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

  deploy-staging:
    executor: rust-executor
    steps:
      - checkout

      - setup_rust

      - run:
          name: Build Release
          command: cargo build --release

      - run:
          name: Deploy to Staging
          command: |
            echo "🚀 Deploying to staging environment..."
            # Add your staging deployment commands here

  deploy-production:
    executor: rust-executor
    steps:
      - checkout

      - setup_rust

      - run:
          name: Build Release
          command: cargo build --release --all-features

      - run:
          name: Run Integration Tests
          command: cargo test --test integration

      - run:
          name: Deploy to Production
          command: |
            echo "🚀 Deploying to production environment..."
            # Add your production deployment commands here
"""

        return {
            "platform": "circleci",
            "config_file": ".circleci/config.yml",
            "content": circleci_config
        }

    def _generate_azure_devops(self) -> Dict[str, Any]:
        """Generate Azure DevOps pipeline"""

        azure_pipelines = """trigger:
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
  CARGO_HOME: $(Pipeline.Workspace)/.cargo
  RUST_BACKTRACE: 1
  QA_TIMEOUT: 300

stages:
- stage: Test
  jobs:
  - job: UnitTests
    steps:
    - task: UsePythonVersion@0
      inputs:
        versionSpec: '3.9'

    - script: |
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
        rustc --version
        cargo --version
      displayName: 'Install Rust'

    - task: Cache@2
      inputs:
        key: 'cargo | "$(Agent.OS)" | Cargo.lock'
        path: $(CARGO_HOME)/registry
      displayName: 'Cache Cargo Registry'

    - script: |
        source ~/.cargo/env
        cargo test --verbose --all-features
        cargo test --doc
      displayName: 'Run Tests'

    - script: |
        source ~/.cargo/env
        cargo fmt --all -- --check
        cargo clippy --all-targets --all-features -- -D warnings
      displayName: 'Run Linting'

    - task: PublishTestResults@2
      condition: succeededOrFailed()
      inputs:
        testResultsFiles: 'test-results.xml'
        testRunTitle: 'Unit Tests'

- stage: QA
  dependsOn: Test
  jobs:
  - job: QualityAnalysis
    steps:
    - task: UsePythonVersion@0
      inputs:
        versionSpec: '3.9'

    - script: |
        python -m pip install --upgrade pip
        pip install -r tools/codex-supervisor/requirements.txt
        pip install pytest pytest-cov mypy black bandit safety
      displayName: 'Install QA Dependencies'

    - script: |
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

- stage: Build
  dependsOn: QA
  jobs:
  - job: BuildRelease
    steps:
    - script: |
        source ~/.cargo/env
        cargo build --release --all-features
      displayName: 'Build Release'

    - task: PublishBuildArtifacts@1
      inputs:
        pathtoPublish: 'target/release'
        artifactName: 'release-artifacts'

- stage: Deploy
  dependsOn: Build
  jobs:
  - job: DeployStaging
    condition: eq(variables['Build.SourceBranch'], 'refs/heads/develop')
    steps:
    - script: |
        echo "🚀 Deploying to staging environment..."
        # Add your staging deployment commands here

  - job: DeployProduction
    condition: eq(variables['Build.SourceBranch'], 'refs/heads/main')
    steps:
    - script: |
        echo "🚀 Deploying to production environment..."
        # Add your production deployment commands here
"""

        return {
            "platform": "azure_devops",
            "config_file": "azure-pipelines.yml",
            "content": azure_pipelines
        }

def save_pipeline_files(pipelines: Dict[str, Any], base_path: Path):
    """Save generated pipeline files to disk"""

    for platform, config in pipelines.items():
        if platform == "github_actions":
            # Create .github/workflows directory
            workflows_dir = base_path / ".github" / "workflows"
            workflows_dir.mkdir(parents=True, exist_ok=True)

            for filename, content in config["workflows"].items():
                file_path = workflows_dir / filename
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                print(f"✅ Created: {file_path}")

        elif "config_file" in config:
            file_path = base_path / config["config_file"]
            file_path.parent.mkdir(parents=True, exist_ok=True)

            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(config["content"])
            print(f"✅ Created: {file_path}")

def main():
    """Main entry point for CI/CD integration"""
    import argparse

    parser = argparse.ArgumentParser(description="CI/CD Integration - Multi-Platform Pipeline Generator")
    parser.add_argument("command", choices=["generate", "setup", "validate"])
    parser.add_argument("--platforms", nargs="+",
                       default=["github_actions"],
                       choices=["github_actions", "gitlab_ci", "jenkins", "circleci", "azure_devops"],
                       help="Target CI/CD platforms")
    parser.add_argument("--project-name", default="codex", help="Project name")
    parser.add_argument("--project-type", default="rust",
                       choices=["rust", "nodejs", "python", "generic"], help="Project type")
    parser.add_argument("--environments", nargs="+",
                       default=["development", "staging", "production"],
                       help="Target environments")
    parser.add_argument("--quality-gates", nargs="+",
                       default=["code_review", "qa_analysis", "security_scan"],
                       help="Quality gates to enforce")
    parser.add_argument("--output-dir", type=Path, default=".", help="Output directory")

    args = parser.parse_args()

    # Create configuration
    config = CICDConfig(
        platforms=args.platforms,
        project_name=args.project_name,
        project_type=args.project_type,
        environments=args.environments,
        quality_gates=args.quality_gates
    )

    if args.command == "generate":
        print("🚀 Generating CI/CD Pipelines...")
        print(f"Platforms: {', '.join(config.platforms)}")
        print(f"Project: {config.project_name} ({config.project_type})")
        print(f"Environments: {', '.join(config.environments)}")
        print(f"Quality Gates: {', '.join(config.quality_gates)}")
        print()

        generator = CICDGenerator(config)
        pipelines = generator.generate_pipelines()

        save_pipeline_files(pipelines, args.output_dir)

        print("\n🎉 CI/CD Pipeline Generation Complete!")
        print("Generated files:")
        for platform, config in pipelines.items():
            if platform == "github_actions":
                for filename in config["workflows"].keys():
                    print(f"  - .github/workflows/{filename}")
            elif "config_file" in config:
                print(f"  - {config['config_file']}")

        print("\nNext steps:")
        print("1. Review and customize generated pipeline files")
        print("2. Set up required secrets (API keys, deployment tokens)")
        print("3. Configure branch protection rules")
        print("4. Test pipelines with a sample PR")

    elif args.command == "setup":
        print("🔧 Setting up CI/CD integration...")

        # Generate pipelines
        generator = CICDGenerator(config)
        pipelines = generator.generate_pipelines()
        save_pipeline_files(pipelines, args.output_dir)

        # Create additional configuration files
        create_additional_configs(args.output_dir)

        print("✅ CI/CD setup complete!")

    elif args.command == "validate":
        print("🔍 Validating CI/CD configurations...")

        # Basic validation of generated files
        validation_results = validate_pipeline_configs(args.output_dir, config.platforms)

        if validation_results["valid"]:
            print("✅ All pipeline configurations are valid")
        else:
            print("❌ Validation failed:")
            for error in validation_results["errors"]:
                print(f"  - {error}")

def create_additional_configs(output_dir: Path):
    """Create additional configuration files"""

    # Docker configuration
    dockerfile = """# Multi-stage Docker build for Rust application
FROM rust:1.70-slim as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src target/release/deps

COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/codex /usr/local/bin/codex

EXPOSE 3000
CMD ["codex"]
"""

    docker_compose = """version: '3.8'

services:
  codex:
    build: .
    ports:
      - "3000:3000"
    environment:
      - NODE_ENV=production
    volumes:
      - ./data:/app/data
    restart: unless-stopped

  qa-service:
    build:
      context: .
      dockerfile: Dockerfile.qa
    volumes:
      - ./:/workspace
    command: ["python", "tools/background_qa_service.py", "--auto-discover-worktrees"]
"""

    # Write configuration files
    with open(output_dir / "Dockerfile", 'w', encoding='utf-8') as f:
        f.write(dockerfile)

    with open(output_dir / "docker-compose.yml", 'w', encoding='utf-8') as f:
        f.write(docker_compose)

    print("✅ Created Docker configuration files")

def validate_pipeline_configs(output_dir: Path, platforms: List[str]) -> Dict[str, Any]:
    """Validate generated pipeline configurations"""

    results = {"valid": True, "errors": []}

    for platform in platforms:
        if platform == "github_actions":
            workflows_dir = output_dir / ".github" / "workflows"
            if workflows_dir.exists():
                for workflow_file in workflows_dir.glob("*.yml"):
                    try:
                        with open(workflow_file, 'r', encoding='utf-8') as f:
                            yaml.safe_load(f)
                    except Exception as e:
                        results["valid"] = False
                        results["errors"].append(f"Invalid YAML in {workflow_file}: {e}")
            else:
                results["valid"] = False
                results["errors"].append("GitHub Actions workflows directory not found")

        elif platform == "gitlab_ci":
            gitlab_file = output_dir / ".gitlab-ci.yml"
            if gitlab_file.exists():
                try:
                    with open(gitlab_file, 'r', encoding='utf-8') as f:
                        yaml.safe_load(f)
                except Exception as e:
                    results["valid"] = False
                    results["errors"].append(f"Invalid YAML in {gitlab_file}: {e}")
            else:
                results["valid"] = False
                results["errors"].append("GitLab CI configuration file not found")

        # Add validation for other platforms as needed

    return results

if __name__ == "__main__":
    main()