import re
import yaml

files = [
    "C:/Users/downl/Desktop/codex-main/.github/workflows/subagent-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/release-subagent.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/qa-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/codeql.yml",
]
for fname in files:
    with open(fname, encoding="utf-8", errors="replace") as f:
        content = f.read()
    # Remove control characters that are invalid in YAML 1.1
    # Keep: tab (0x09), newline (0x0A), carriage return (0x0D)
    # Remove: 0x00-0x08, 0x0B-0x0C, 0x0E-0x1F, 0x7F, 0x80-0x9F
    cleaned = re.sub(r"[\x00-\x08\x0b\x0c\x0e-\x1f\x7f\x80-\x9f]", "", content)
    # Also replace replacement character from errors="replace"
    cleaned = cleaned.replace("\ufffd", "?")
    try:
        yaml.safe_load(cleaned)
        with open(fname, "w", encoding="utf-8") as f:
            f.write(cleaned)
        short = fname.split("/")[-1]
        removed = len(content) - len(cleaned)
        print(f"OK: {short} - removed {removed} control chars")
    except yaml.YAMLError as e:
        short = fname.split("/")[-1]
        print(f"YAML ERROR after clean {short}: {str(e)[:200]}")
