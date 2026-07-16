import yaml

files = [
    "C:/Users/downl/Desktop/codex-main/.github/workflows/qa-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/subagent-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/release-subagent.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/codeql.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/rust-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/ci.yml",
]
for fname in files:
    try:
        with open(fname, encoding="utf-8") as f:
            content = f.read()
        yaml.safe_load(content)
        short = fname.split("/")[-1]
        non_ascii = sum(1 for c in content if ord(c) > 127)
        print(f"OK: {short} (non-ASCII: {non_ascii})")
    except Exception as e:
        short = fname.split("/")[-1]
        print(f"ERROR {short}: {str(e)[:100]}")
