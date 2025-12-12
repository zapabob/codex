#!/usr/bin/env python3
"""
Improved Codex Integration Test for Continuous Testing
"""

import subprocess
import sys
import time
import os
import json
from datetime import datetime

try:
    from tqdm import tqdm
except ImportError:
    subprocess.run([sys.executable, "-m", "pip", "install", "tqdm"], check=True)
    from tqdm import tqdm

class ContinuousIntegrationTester:
    def __init__(self):
        self.start_time = datetime.now()
        self.results = {}
        self.project_root = os.path.dirname(os.path.abspath(__file__))
        
    def run_command(self, cmd, cwd=None, timeout=30):
        result = {
            "command": " ".join(cmd) if isinstance(cmd, list) else cmd,
            "success": False,
            "stdout": "",
            "stderr": "",
            "execution_time": 0
        }
        
        start_time = time.time()
        try:
            proc = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, timeout=timeout)
            result["success"] = proc.returncode == 0
            result["stdout"] = proc.stdout
            result["stderr"] = proc.stderr
        except subprocess.TimeoutExpired:
            result["stderr"] = f"Timeout after {timeout}s"
        except Exception as e:
            result["stderr"] = str(e)
            
        result["execution_time"] = time.time() - start_time
        return result
    
    def test_core_systems(self):
        """Test core system components"""
        print("[CORE] Testing core system components...")
        
        tests = [
            (["codex", "--version"], "CLI Version"),
            (["node", "--version"], "Node.js Version"),
            (["npm", "--version"], "NPM Version"),
        ]
        
        results = {}
        for cmd, desc in tqdm(tests, desc="Core Tests"):
            result = self.run_command(cmd)
            results[desc] = result
            status = "[OK]" if result["success"] else "[FAIL]"
            tqdm.write(f"{status} {desc}")
            
        return results
    
    def test_build_systems(self):
        """Test build systems"""
        print("[BUILD] Testing build systems...")
        
        results = {}
        
        # Rust check
        rust_result = self.run_command(["cargo", "check"], cwd=os.path.join(self.project_root, "codex-rs"))
        results["Rust Check"] = rust_result
        tqdm.write(f"{'[OK]' if rust_result['success'] else '[FAIL]'} Rust Check")
        
        # GUI build check
        gui_path = os.path.join(self.project_root, "gui")
        if os.path.exists(gui_path):
            gui_result = self.run_command(["npm", "run", "build"], cwd=gui_path, timeout=60)
            results["GUI Build"] = gui_result
            tqdm.write(f"{'[OK]' if gui_result['success'] else '[FAIL]'} GUI Build")
        
        return results
    
    def test_integration_scenarios(self):
        """Test integration scenarios"""
        print("[INTEGRATION] Testing integration scenarios...")
        
        results = {}
        
        # CLI exec test
        exec_result = self.run_command(["codex", "exec", "echo 'integration test'"], timeout=60)
        results["CLI Exec Integration"] = exec_result
        tqdm.write(f"{'[OK]' if exec_result['success'] else '[WARN]'} CLI Exec Integration")
        
        return results
    
    def generate_report(self):
        """Generate test report"""
        end_time = datetime.now()
        duration = end_time - self.start_time
        
        total_tests = sum(len(v) if isinstance(v, dict) else 1 for v in self.results.values())
        passed_tests = sum(1 for v in self.results.values() 
                          if isinstance(v, dict) and all(r.get("success", False) for r in v.values())
                          else v.get("success", False) if isinstance(v, dict) else False)
        
        report = {
            "timestamp": self.start_time.isoformat(),
            "duration_seconds": duration.total_seconds(),
            "total_tests": total_tests,
            "passed_tests": passed_tests,
            "success_rate": (passed_tests / total_tests * 100) if total_tests > 0 else 0,
            "results": self.results
        }
        
        return report
    
    def run_continuous_tests(self):
        """Run continuous integration tests"""
        print(f"[START] Continuous Integration Test Suite - {self.start_time}")
        print("=" * 80)
        
        self.results["core_systems"] = self.test_core_systems()
        self.results["build_systems"] = self.test_build_systems()
        self.results["integration"] = self.test_integration_scenarios()
        
        report = self.generate_report()
        
        # Display results
        print("\n" + "=" * 80)
        print("[RESULTS] CONTINUOUS INTEGRATION TEST SUMMARY")
        print("=" * 80)
        
        print(f"Tests Run: {report['total_tests']}")
        print(f"Tests Passed: {report['passed_tests']}")
        print(".1f")
        print(f"Duration: {report['duration_seconds']:.2f} seconds")
        
        success_rate = report['success_rate']
        if success_rate >= 90:
            print("[STATUS] EXCELLENT - All systems operational")
        elif success_rate >= 75:
            print("[STATUS] GOOD - Minor issues detected")
        elif success_rate >= 50:
            print("[STATUS] FAIR - Some attention needed")
        else:
            print("[STATUS] POOR - Significant issues found")
        
        return report

def main():
    tester = ContinuousIntegrationTester()
    try:
        report = tester.run_continuous_tests()
        success_rate = report["success_rate"]
        
        if success_rate >= 75:
            print("\n[SUCCESS] Continuous integration tests PASSED")
            return 0
        else:
            print("\n[ISSUES] Continuous integration tests have issues")
            return 1
            
    except Exception as e:
        print(f"\n[ERROR] CI test suite failed: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(main())
