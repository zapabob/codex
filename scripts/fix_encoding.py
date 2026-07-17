files = [
    "C:/Users/downl/Desktop/codex-main/.github/workflows/subagent-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/release-subagent.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/qa-ci.yml",
    "C:/Users/downl/Desktop/codex-main/.github/workflows/codeql.yml",
]
for fname in files:
    with open(fname, "rb") as f:
        raw = f.read()
    for enc in ["utf-8", "utf-8-sig", "cp932", "shift_jis", "latin-1"]:
        try:
            content = raw.decode(enc)
            short = fname.split("/")[-1]
            with open(fname, "w", encoding="utf-8") as f:
                f.write(content)
            print(f"Converted {short}: {enc} -> utf-8")
            break
        except Exception as e:
            pass
