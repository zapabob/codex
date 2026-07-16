#!/usr/bin/env python3
"""CI failure analyzer - reads gh run JSON and reports failures."""
import json
import subprocess
import sys


def get_run_jobs(run_id):
    result = subprocess.run(
        ["gh", "run", "view", str(run_id), "--json", "jobs"],
        capture_output=True, text=True, encoding="utf-8", errors="replace",
        cwd=r"C:\Users\downl\Desktop\codex-main"
    )
    if result.returncode != 0:
        return None
    return json.loads(result.stdout)


def get_runs():
    result = subprocess.run(
        ["gh", "run", "list", "--limit", "10", "--json",
         "databaseId,name,conclusion,workflowName"],
        capture_output=True, text=True, encoding="utf-8", errors="replace",
        cwd=r"C:\Users\downl\Desktop\codex-main"
    )
    return json.loads(result.stdout)


def get_run_log(run_id):
    result = subprocess.run(
        ["gh", "run", "view", str(run_id), "--log-failed"],
        capture_output=True, text=True, encoding="utf-8", errors="replace",
        cwd=r"C:\Users\downl\Desktop\codex-main"
    )
    return result.stdout + result.stderr


def main():
    print("=== CI Failure Analysis ===\n")
    runs = get_runs()

    failed_runs = [r for r in runs if r.get("conclusion") == "failure"]
    print(f"Failed runs: {len(failed_runs)}/{len(runs)}\n")

    for run in failed_runs[:5]:
        wf = run.get("workflowName", "")[:40]
        rid = run.get("databaseId")
        print(f"[FAIL] {wf} (id:{rid})")

        jobs_data = get_run_jobs(rid)
        if jobs_data and "jobs" in jobs_data:
            for job in jobs_data["jobs"]:
                if job.get("conclusion") == "failure":
                    print(f"  Job: {job['name']}")
                    for step in job.get("steps", []):
                        if step.get("conclusion") == "failure":
                            print(f"    Step FAILED: {step['name']}")

        # Get failure log keywords
        log = get_run_log(rid)
        keywords = []
        for line in log.splitlines():
            if any(k in line for k in ["error[", "Error:", "FAIL", "misspell", "advisory", "banned", "denied", "not found"]):
                if "checkout" not in line.lower() and "git config" not in line.lower():
                    keywords.append(line.strip()[:120])

        if keywords:
            print("  Key errors:")
            for kw in keywords[:8]:
                print(f"    {kw}")
        print()


if __name__ == "__main__":
    main()
