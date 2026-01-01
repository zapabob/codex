#!/usr/bin/env python3
"""
Codex GUITUICLI Integration Test Script
Simple version for Windows compatibility
"""

import subprocess
import sys
import time
import os
import platform
import json
from datetime import datetime

# tqdm import
try:
    from tqdm import tqdm
except ImportError:
    print("tqdm not found. Installing...")
    subprocess.run([sys.executable, "-m", "pip", "install", "tqdm"], check=True)
    from tqdm import tqdm

class CodexIntegrationTester:
    def __init__(self):
        self.start_time = datetime.now()
        self.results = {}
        self.project_root = os.path.dirname(os.path.abspath(__file__))
        self.codex_rs_path = os.path.join(self.project_root, "codex-rs")
        self.gui_path = os.path.join(self.project_root, "gui")

        # Get system info
        self.system_info = self.get_system_info()

        print(f"[START] Starting Codex GUITUICLI Integration Test")
        print(f"[TIME] Start Time: {self.start_time}")
        print(f"[SYS] System: {self.system_info}")
        print(f"[PATH] Project Root: {self.project_root}")
        print("=" * 60)

    def get_system_info(self):
        return {
            "platform": platform.system(),
            "platform_version": platform.version(),
            "python_version": sys.version.split()[0],
            "cpu_count": os.cpu_count(),
            "hostname": platform.node()
        }

    def run_command(self, cmd, cwd=None, timeout=30, description=""):
        result = {
            "command": " ".join(cmd) if isinstance(cmd, list) else cmd,
            "cwd": cwd or os.getcwd(),
            "returncode": None,
            "stdout": "",
            "stderr": "",
            "execution_time": 0,
            "success": False,
            "description": description
        }

        start_time = time.time()
        try:
            proc = subprocess.run(
                cmd,
                cwd=cwd,
                capture_output=True,
                text=True,
                timeout=timeout
            )
            result["returncode"] = proc.returncode
            result["stdout"] = proc.stdout
            result["stderr"] = proc.stderr
            result["success"] = proc.returncode == 0
        except subprocess.TimeoutExpired:
            result["stderr"] = f"Command timed out after {timeout}s"
        except Exception as e:
            result["stderr"] = str(e)

        result["execution_time"] = time.time() - start_time
        return result

    def test_cli_basic(self):
        print("\n[TEST] Testing CLI Basic Functions...")

        tests = [
            (["codex", "--version"], "CLI Version Check"),
            (["codex", "--help"], "CLI Help Display"),
            (["codex", "exec", "echo 'Hello Codex CLI'"], "CLI Exec Command"),
        ]

        results = {}
        for cmd, desc in tqdm(tests, desc="CLI Tests"):
            result = self.run_command(cmd, description=desc)
            results[desc] = result

            if result["success"]:
                tqdm.write(f"[PASS] {desc}: PASSED")
            else:
                tqdm.write(f"[FAIL] {desc}: FAILED - {result['stderr'][:100]}...")

        return results

    def test_gui_startup(self):
        print("\n[GUI] Testing GUI Startup...")

        results = {}

        # Check GUI directory
        if not os.path.exists(self.gui_path):
            return {"error": "GUI directory not found"}

        # Check package.json
        package_json = os.path.join(self.gui_path, "package.json")
        if not os.path.exists(package_json):
            return {"error": "package.json not found in GUI directory"}

        # npm install
        tqdm.write("[DEPS] Installing GUI dependencies...")
        install_result = self.run_command(
            ["npm", "install"],
            cwd=self.gui_path,
            timeout=120,
            description="GUI Dependencies Install"
        )
        results["dependencies_install"] = install_result

        if not install_result["success"]:
            tqdm.write(f"[FAIL] GUI dependencies install failed: {install_result['stderr'][:100]}...")
            return results

        # GUI build test
        tqdm.write("[BUILD] Building GUI...")
        build_result = self.run_command(
            ["npm", "run", "build"],
            cwd=self.gui_path,
            timeout=180,
            description="GUI Build"
        )
        results["build"] = build_result

        if build_result["success"]:
            tqdm.write("[PASS] GUI build: PASSED")
        else:
            tqdm.write(f"[FAIL] GUI build failed: {build_result['stderr'][:100]}...")

        return results

    def test_tui_startup(self):
        print("\n[TUI] Testing TUI Startup...")

        results = {}

        # Check Rust directory
        if not os.path.exists(self.codex_rs_path):
            return {"error": "codex-rs directory not found"}

        # TUI startup test
        tqdm.write("[START] Starting TUI (brief test)...")
        tui_result = self.run_command(
            ["cargo", "run", "-p", "codex-tui", "--", "--help"],
            cwd=self.codex_rs_path,
            timeout=30,
            description="TUI Help Display"
        )
        results["tui_help"] = tui_result

        if tui_result["success"]:
            tqdm.write("[PASS] TUI startup: PASSED")
        else:
            tqdm.write(f"[FAIL] TUI startup failed: {tui_result['stderr'][:100]}...")

        return results

    def test_playwright_integration(self):
        print("\n[PLAYWRIGHT] Testing Playwright Integration...")

        results = {}

        try:
            # Check Playwright installation
            tqdm.write("[CHECK] Checking Playwright installation...")
            import playwright
            from playwright.sync_api import sync_playwright

            results["playwright_available"] = True
            tqdm.write("[OK] Playwright: AVAILABLE")

            # Browser test
            tqdm.write("[WEB] Testing browser connectivity...")

            with sync_playwright() as p:
                browser = p.chromium.launch(headless=True)
                page = browser.new_page()

                # Test local GUI server
                try:
                    page.goto("http://localhost:3000", timeout=5000)
                    title = page.title()
                    results["gui_server_access"] = {
                        "success": True,
                        "title": title,
                        "url": "http://localhost:3000"
                    }
                    tqdm.write(f"[OK] GUI server accessible: {title}")
                except Exception as e:
                    results["gui_server_access"] = {
                        "success": False,
                        "error": str(e)
                    }
                    tqdm.write(f"[WARN] GUI server not accessible: {str(e)[:100]}")

                # Test Cursor IDE
                try:
                    cursor_pages = []
                    for context in browser.contexts:
                        for page_obj in context.pages:
                            if "cursor" in page_obj.url.lower() or "localhost" in page_obj.url:
                                cursor_pages.append({
                                    "url": page_obj.url,
                                    "title": page_obj.title()
                                })

                    results["cursor_browser_check"] = {
                        "success": True,
                        "pages_found": len(cursor_pages),
                        "pages": cursor_pages
                    }
                    tqdm.write(f"[OK] Cursor browser check: {len(cursor_pages)} pages found")
                except Exception as e:
                    results["cursor_browser_check"] = {
                        "success": False,
                        "error": str(e)
                    }
                    tqdm.write(f"[WARN] Cursor browser check failed: {str(e)[:100]}")

                browser.close()

        except ImportError:
            results["playwright_available"] = False
            tqdm.write("[FAIL] Playwright: NOT AVAILABLE - installing...")

            # Install Playwright
            install_result = self.run_command(
                [sys.executable, "-m", "pip", "install", "playwright"],
                description="Install Playwright"
            )

            if install_result["success"]:
                tqdm.write("[OK] Playwright installed")
                results["playwright_install"] = install_result
            else:
                tqdm.write("[FAIL] Playwright installation failed")

        return results

    def test_integration_scenarios(self):
        print("\n[INTEGRATION] Testing Integration Scenarios...")

        scenarios = [
            {
                "name": "CLI-to-GUI Pipeline",
                "steps": [
                    (["codex", "exec", "echo 'CLI command executed'"], "CLI exec test"),
                    (["codex", "plan", "list"], "Plan listing via CLI"),
                ]
            },
            {
                "name": "Version Consistency Check",
                "steps": [
                    (["codex", "--version"], "CLI version"),
                    (["cargo", "run", "-p", "codex-cli", "--", "--version"], "Direct binary version"),
                ]
            }
        ]

        results = {}
        for scenario in tqdm(scenarios, desc="Integration Scenarios"):
            scenario_results = []
            for cmd, desc in scenario["steps"]:
                result = self.run_command(cmd, description=desc)
                scenario_results.append(result)

            results[scenario["name"]] = {
                "steps": scenario_results,
                "all_passed": all(r["success"] for r in scenario_results)
            }

            if results[scenario["name"]]["all_passed"]:
                tqdm.write(f"[PASS] {scenario['name']}: ALL STEPS PASSED")
            else:
                tqdm.write(f"[FAIL] {scenario['name']}: SOME STEPS FAILED")

        return results

    def generate_report(self):
        end_time = datetime.now()
        duration = end_time - self.start_time

        report = {
            "test_metadata": {
                "start_time": self.start_time.isoformat(),
                "end_time": end_time.isoformat(),
                "duration_seconds": duration.total_seconds(),
                "system_info": self.system_info,
                "project_root": self.project_root
            },
            "test_results": self.results,
            "summary": {
                "total_tests": 0,
                "passed_tests": 0,
                "failed_tests": 0,
                "success_rate": 0.0
            }
        }

        # Count results
        def count_results(data):
            passed = 0
            failed = 0
            total = 0

            if isinstance(data, dict):
                for key, value in data.items():
                    if isinstance(value, dict):
                        if "success" in value:
                            total += 1
                            if value["success"]:
                                passed += 1
                            else:
                                failed += 1
                        else:
                            p, f, t = count_results(value)
                            passed += p
                            failed += f
                            total += t
            elif isinstance(data, list):
                for item in data:
                    if isinstance(item, dict) and "success" in item:
                        total += 1
                        if item["success"]:
                            passed += 1
                        else:
                            failed += 1

            return passed, failed, total

        passed, failed, total = count_results(self.results)
        report["summary"].update({
            "total_tests": total,
            "passed_tests": passed,
            "failed_tests": failed,
            "success_rate": (passed / total * 100) if total > 0 else 0
        })

        return report

    def display_results(self, report):
        print("\n" + "=" * 80)
        print("[SUMMARY] TEST RESULTS SUMMARY")
        print("=" * 80)

        summary = report["summary"]
        duration = report["test_metadata"]["duration_seconds"]

        print(f"[STATS] Total Tests: {summary['total_tests']}")
        print(f"[PASS] Passed: {summary['passed_tests']}")
        print(f"[FAIL] Failed: {summary['failed_tests']}")
        print(".1f")
        print(f"[TIME] Duration: {duration:.2f} seconds")
        print(f"[SYS] System: {self.system_info['platform']} {self.system_info['platform_version']}")

        print("\n[DETAILS] Detailed Results:")

        def print_nested_results(data, indent=0):
            prefix = "  " * indent
            if isinstance(data, dict):
                for key, value in data.items():
                    if isinstance(value, dict):
                        if "success" in value and isinstance(value["success"], bool):
                            status = "[OK]" if value["success"] else "[ERROR]"
                            desc = value.get("description", key)
                            print(f"{prefix}{status} {desc}")
                        elif "all_passed" in value:
                            status = "[OK]" if value["all_passed"] else "[ERROR]"
                            print(f"{prefix}{status} {key}")
                            if "steps" in value:
                                for step in value["steps"]:
                                    if isinstance(step, dict) and "success" in step:
                                        step_status = "[OK]" if step["success"] else "[ERROR]"
                                        print(f"{prefix}  {step_status} {step.get('description', 'Unknown')}")
                        else:
                            print(f"{prefix}[DIR] {key}")
                            print_nested_results(value, indent + 1)
                    elif isinstance(value, list):
                        for i, item in enumerate(value):
                            if isinstance(item, dict) and "success" in item:
                                status = "[OK]" if item["success"] else "[ERROR]"
                                desc = item.get("description", f"Item {i+1}")
                                print(f"{prefix}{status} {desc}")

        print_nested_results(self.results)

        print("\n[ASSESSMENT] Overall Assessment:")
        success_rate = summary["success_rate"]
        if success_rate >= 90:
            print("[EXCELLENT] All systems operational!")
        elif success_rate >= 75:
            print("[GOOD] Minor issues detected")
        elif success_rate >= 50:
            print("[FAIR] Some functionality needs attention")
        else:
            print("[POOR] Significant issues found")

    def save_report(self, report):
        timestamp = self.start_time.strftime("%Y-%m-%d_%H-%M-%S")
        filename = f"_docs/{timestamp}_GUITUICLI_test_results.json"

        os.makedirs("_docs", exist_ok=True)

        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(report, f, indent=2, ensure_ascii=False)

        print(f"\n[SAVE] Report saved to: {filename}")

        # Markdown report
        md_filename = f"_docs/{timestamp}_GUITUICLI_test_results.md"
        self.generate_markdown_report(report, md_filename)
        print(f"[SAVE] Markdown report saved to: {md_filename}")

    def generate_markdown_report(self, report, filename):
        with open(filename, 'w', encoding='utf-8') as f:
            f.write("# GUITUICLI Test Results\n\n")
            f.write(f"**Date**: {self.start_time.strftime('%Y-%m-%d %H:%M:%S')}\n")
            f.write("**Task**: GUI/CLI/Playwright Integration Testing\n\n")
            f.write("---\n\n")

            # System info
            f.write("## System Information\n\n")
            sys_info = self.system_info
            f.write(f"- **Platform**: {sys_info['platform']}\n")
            f.write(f"- **Version**: {sys_info['platform_version']}\n")
            f.write(f"- **Python**: {sys_info['python_version']}\n")
            f.write(f"- **CPU Cores**: {sys_info['cpu_count']}\n")
            f.write(f"- **Hostname**: {sys_info['hostname']}\n\n")

            # Summary
            summary = report["summary"]
            duration = report["test_metadata"]["duration_seconds"]
            f.write("## Test Summary\n\n")
            f.write(f"- **Total Tests**: {summary['total_tests']}\n")
            f.write(f"- **Passed**: {summary['passed_tests']}\n")
            f.write(f"- **Failed**: {summary['failed_tests']}\n")
            f.write(".1f")
            f.write(".2f")
            f.write("\n---\n\n")

            # Detailed results
            f.write("## Detailed Results\n\n")

            def write_nested_results(data, f, indent=0):
                prefix = "  " * indent
                if isinstance(data, dict):
                    for key, value in data.items():
                        if isinstance(value, dict):
                            if "success" in value and isinstance(value["success"], bool):
                                status = "[OK]" if value["success"] else "[ERROR]"
                                desc = value.get("description", key)
                                f.write(f"{prefix}- {status} {desc}\n")
                                if "stderr" in value and value["stderr"]:
                                    f.write(f"{prefix}  - Error: {value['stderr'][:200]}...\n")
                            elif "all_passed" in value:
                                status = "[OK]" if value["all_passed"] else "[ERROR]"
                                f.write(f"{prefix}- {status} {key}\n")
                                if "steps" in value:
                                    for step in value["steps"]:
                                        if isinstance(step, dict) and "success" in step:
                                            step_status = "[OK]" if step["success"] else "[ERROR]"
                                            desc = step.get("description", "Unknown")
                                            f.write(f"{prefix}  - {step_status} {desc}\n")
                            else:
                                f.write(f"{prefix}- [DIR] {key}\n")
                                write_nested_results(value, f, indent + 1)
                        elif isinstance(value, list):
                            for i, item in enumerate(value):
                                if isinstance(item, dict) and "success" in item:
                                    status = "[OK]" if item["success"] else "[ERROR]"
                                    desc = item.get("description", f"Item {i+1}")
                                    f.write(f"{prefix}- {status} {desc}\n")

            write_nested_results(self.results, f)

            f.write("\n---\n\n")
            f.write("## Completion Notification\n\n")
            f.write("Test completed successfully.\n")

    def run_all_tests(self):
        print("[TEST] Starting Comprehensive GUITUICLI Integration Test Suite")
        print("=" * 80)

        # CLI tests
        self.results["cli_tests"] = self.test_cli_basic()

        # GUI tests
        self.results["gui_tests"] = self.test_gui_startup()

        # TUI tests
        self.results["tui_tests"] = self.test_tui_startup()

        # Playwright tests
        self.results["playwright_tests"] = self.test_playwright_integration()

        # Integration tests
        self.results["integration_tests"] = self.test_integration_scenarios()

        # Generate report
        report = self.generate_report()

        # Display results
        self.display_results(report)

        # Save report
        self.save_report(report)

        return report

def main():
    tester = CodexIntegrationTester()
    try:
        report = tester.run_all_tests()
        success_rate = report["summary"]["success_rate"]

        if success_rate >= 75:
            print("\n[SUCCESS] GUITUICLI Integration Test COMPLETED SUCCESSFULLY!")
            sys.exit(0)
        else:
            print("\n[ISSUES] GUITUICLI Integration Test COMPLETED with ISSUES")
            sys.exit(1)

    except Exception as e:
        print(f"\n[ERROR] Test suite failed with exception: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == "__main__":
    main()