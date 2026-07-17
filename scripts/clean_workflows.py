import re

files = [
    "C:/Users/downl/Desktop/codex-main/.github/workflows/qa-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/subagent-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/release-subagent.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/codeql.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/bazel.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/rust-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/kernel-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/security-scan.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/integration-tests.yml",
]
for fname in files:
    try:
        with open(fname, encoding="utf-8", errors="replace") as f:
            content = f.read()
        cleaned = re.sub(r"[^\x00-\x7F]", "", content)
        removed = len(content) - len(cleaned)
        if removed > 0:
            with open(fname, "w", encoding="utf-8") as f:
                f.write(cleaned)
            short = fname.split("/")[-1]
            print(f"Cleaned {short}: removed {removed} non-ASCII chars")
        else:
            short = fname.split("/")[-1]
            print(f"OK {short}: already ASCII-clean")
    except Exception as e:
        print(f"ERROR: {e}")
