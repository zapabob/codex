import yaml

files = [
    "C:/Users/downl/Desktop/codex-main/.github/workflows/subagent-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/release-subagent.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/qa-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/codeql.yml",
]
for fname in files:
    with open(fname, encoding="utf-8") as f:
        content = f.read()
    try:
        data = yaml.safe_load(content)
        short = fname.split("/")[-1]
        issues = []
        bad_refs = [
            "@v6",
            "@v7",
            "codex-runners",
            "macos-15-xlarge",
            "windows-x64",
            "windows-arm64",
        ]
        for ref in bad_refs:
            if ref in content:
                issues.append(ref)
        print(f"OK: {short} | issues: {issues if issues else 'none'}")
    except Exception as e:
        print(f"ERROR {fname.split('/')[-1]}: {str(e)[:150]}")
