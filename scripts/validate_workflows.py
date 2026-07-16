import yaml
import sys

files = [
    "C:/Users/downl/Desktop/codex-main/.github/workflows/subagent-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/release-subagent.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/qa-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/codeql.yml",
]
for fname in files:
    try:
        with open(fname, encoding="utf-8") as f:
            content = f.read()
        yaml.safe_load(content)
        short = fname.split("/")[-1]
        issues = []
        if "@v6" in content: issues.append("has @v6 refs: " + str(content.count("@v6")))
        if "codex-runners" in content: issues.append("has codex-runners")
        if "macos-15-xlarge" in content: issues.append("has macos-15-xlarge")
        print(f"OK: {short}", "| ISSUES:", ", ".join(issues) if issues else "none")
    except Exception as e:
        short = fname.split("/")[-1]
        print(f"ERROR {short}:", str(e)[:200])
