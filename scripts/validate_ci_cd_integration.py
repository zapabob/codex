#!/usr/bin/env python3
"""
CI/CD統合検証スクリプト
コードベースとCI/CD設定の互換性を検証
"""

import os
import sys
import subprocess
import json
from pathlib import Path
from typing import Dict, List, Any, Tuple


class CICDEvaluator:
    """CI/CD設定評価クラス"""

    def __init__(self, project_root: Path):
        self.project_root = project_root
        self.github_workflows = project_root / ".github" / "workflows"
        self.codex_rs = project_root / "codex-rs"
        self.issues = []
        self.warnings = []
        self.recommendations = []

    def evaluate_all(self) -> Dict[str, Any]:
        """全評価実行"""
        print("Starting CI/CD integration validation...")

        self._check_workflow_files()
        self._validate_rust_toolchains()
        self._check_cargo_profiles()
        self._validate_dependencies()
        self._check_cache_configurations()
        self._validate_runner_compatibility()
        self._check_timeout_settings()

        return {
            "issues": self.issues,
            "warnings": self.warnings,
            "recommendations": self.recommendations,
            "score": self._calculate_score(),
        }

    def _check_workflow_files(self):
        """ワークフローファイルの検証"""
        required_workflows = ["ci.yml", "rust-ci.yml", "rust-release.yml"]

        for workflow in required_workflows:
            workflow_path = self.github_workflows / workflow
            if not workflow_path.exists():
                self.issues.append(f"Missing required workflow: {workflow}")
            else:
                self._validate_workflow_syntax(workflow_path)

    def _validate_workflow_syntax(self, workflow_path: Path):
        """ワークフロー構文検証"""
        try:
            import yaml

            with open(workflow_path, "r", encoding="utf-8") as f:
                workflow_data = yaml.safe_load(f)

            # 基本構造チェック
            if "jobs" not in workflow_data:
                self.issues.append(
                    f"Invalid workflow {workflow_path.name}: missing 'jobs' section"
                )

            # タイムアウト設定チェック
            for job_name, job_config in workflow_data.get("jobs", {}).items():
                if isinstance(job_config, dict):
                    timeout = job_config.get("timeout-minutes", 0)
                    if timeout > 60:
                        self.warnings.append(
                            f"Job '{job_name}' has long timeout: {timeout} minutes"
                        )

        except Exception as e:
            self.issues.append(f"Failed to parse workflow {workflow_path.name}: {e}")

    def _validate_rust_toolchains(self):
        """Rustツールチェーン検証"""
        rust_ci = self.github_workflows / "rust-ci.yml"
        if rust_ci.exists():
            # ツールチェーン設定チェック
            try:
                import yaml

                with open(rust_ci, "r", encoding="utf-8") as f:
                    workflow_data = yaml.safe_load(f)

                # dtolnay/rust-toolchain使用チェック
                found_toolchain = False
                for job in workflow_data.get("jobs", {}).values():
                    if isinstance(job, dict) and "steps" in job:
                        for step in job["steps"]:
                            if isinstance(step, dict) and "uses" in step:
                                if "dtolnay/rust-toolchain" in step["uses"]:
                                    found_toolchain = True
                                    break

                if not found_toolchain:
                    self.warnings.append("No official Rust toolchain action found")

            except Exception as e:
                self.issues.append(f"Failed to check toolchain config: {e}")

    def _check_cargo_profiles(self):
        """Cargoプロファイルチェック"""
        cargo_toml = self.codex_rs / "Cargo.toml"
        if cargo_toml.exists():
            try:
                import toml

                with open(cargo_toml, "r", encoding="utf-8") as f:
                    cargo_data = toml.load(f)

                profiles = cargo_data.get("profile", {})

                # CI最適化プロファイルチェック
                if "ci-release" not in profiles:
                    self.recommendations.append(
                        "Add ci-release profile for optimized CI builds"
                    )

                if "ci-test" not in profiles:
                    self.warnings.append("Missing ci-test profile for CI testing")

            except Exception as e:
                self.issues.append(f"Failed to parse Cargo.toml: {e}")

    def _validate_dependencies(self):
        """依存関係検証"""
        cargo_toml = self.codex_rs / "Cargo.toml"
        if cargo_toml.exists():
            try:
                import toml

                with open(cargo_toml, "r", encoding="utf-8") as f:
                    cargo_data = toml.load(f)

                workspace = cargo_data.get("workspace", {})
                members = workspace.get("members", [])

                if len(members) < 5:
                    self.warnings.append(
                        "Workspace has few members - consider consolidating"
                    )

                # 重複メンバーチェック
                if len(members) != len(set(members)):
                    self.issues.append("Duplicate workspace members found")

            except Exception as e:
                self.issues.append(f"Failed to validate dependencies: {e}")

    def _check_cache_configurations(self):
        """キャッシュ設定チェック"""
        rust_ci = self.github_workflows / "rust-ci.yml"
        if rust_ci.exists():
            try:
                import yaml

                with open(rust_ci, "r", encoding="utf-8") as f:
                    workflow_data = yaml.safe_load(f)

                # キャッシュアクション使用チェック
                cache_found = False
                for job in workflow_data.get("jobs", {}).values():
                    if isinstance(job, dict) and "steps" in job:
                        for step in job["steps"]:
                            if isinstance(step, dict) and "uses" in step:
                                if "actions/cache" in step["uses"]:
                                    cache_found = True
                                    break

                if not cache_found:
                    self.recommendations.append("Add caching to improve CI performance")

            except Exception as e:
                self.warnings.append(f"Failed to check cache config: {e}")

    def _validate_runner_compatibility(self):
        """ランナー互換性検証"""
        # Ubuntuバージョン統一チェック
        workflows = list(self.github_workflows.glob("*.yml"))
        ubuntu_versions = set()

        try:
            import yaml

            for workflow in workflows:
                with open(workflow, "r", encoding="utf-8") as f:
                    data = yaml.safe_load(f)

                for job in data.get("jobs", {}).values():
                    if isinstance(job, dict):
                        runs_on = job.get("runs-on", "")
                        if "ubuntu" in str(runs_on):
                            ubuntu_versions.add(str(runs_on))

            if len(ubuntu_versions) > 1:
                self.recommendations.append(
                    f"Consider standardizing Ubuntu versions: {ubuntu_versions}"
                )

        except Exception as e:
            self.warnings.append(f"Failed to check runner compatibility: {e}")

    def _check_timeout_settings(self):
        """タイムアウト設定チェック"""
        workflows = list(self.github_workflows.glob("*.yml"))

        try:
            import yaml

            for workflow in workflows:
                with open(workflow, "r", encoding="utf-8") as f:
                    data = yaml.safe_load(f)

                for job_name, job in data.get("jobs", {}).items():
                    if isinstance(job, dict):
                        timeout = job.get("timeout-minutes", 0)
                        if timeout > 45:
                            self.warnings.append(
                                f"Job '{job_name}' in {workflow.name} has long timeout: {timeout}min"
                            )

        except Exception as e:
            self.warnings.append(f"Failed to check timeout settings: {e}")

    def _calculate_score(self) -> float:
        """評価スコア計算"""
        base_score = 100.0

        # 問題点による減点
        base_score -= len(self.issues) * 20
        base_score -= len(self.warnings) * 5

        # 推奨事項による加点
        base_score += min(len(self.recommendations) * 2, 10)

        return max(0.0, min(100.0, base_score))


def main():
    """メイン関数"""
    project_root = Path(__file__).parent.parent

    evaluator = CICDEvaluator(project_root)
    results = evaluator.evaluate_all()

    print("\n" + "=" * 60)
    print("CI/CD Integration Validation Results")
    print("=" * 60)

    print(f"\nOverall Score: {results['score']:.1f}/100")

    if results["issues"]:
        print(f"\nIssues ({len(results['issues'])} found):")
        for issue in results["issues"]:
            print(f"  - {issue}")

    if results["warnings"]:
        print(f"\nWarnings ({len(results['warnings'])} found):")
        for warning in results["warnings"]:
            print(f"  - {warning}")

    if results["recommendations"]:
        print(f"\nRecommendations ({len(results['recommendations'])} found):")
        for rec in results["recommendations"]:
            print(f"  - {rec}")

    # 評価基準
    score = results["score"]
    if score >= 90:
        print("\nExcellent CI/CD configuration!")
        return 0
    elif score >= 70:
        print("\nGood CI/CD configuration.")
        return 0
    elif score >= 50:
        print("\nCI/CD improvements recommended.")
        return 1
    else:
        print("\nCI/CD configuration needs review.")
        return 2


if __name__ == "__main__":
    sys.exit(main())
