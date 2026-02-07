import sys
import os

log_file = sys.argv[1] if len(sys.argv) > 1 else "build_fix.log"

content = ""
try:
    with open(log_file, "r", encoding="utf-16le", errors="replace") as f:
        content = f.read()
        # if looks like garbage, maybe it was utf-8
        if "\0" in content and len(content) > 100: # heuristic
             pass 
except Exception:
     pass

if not content:
    try:
        with open(log_file, "r", encoding="utf-8", errors="replace") as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {log_file}: {e}")
        sys.exit(1)

lines = content.splitlines()
        
found_error = False
for i, line in enumerate(lines):
    if "error[" in line or "error:" in line:
        found_error = True
        print(f"--- Error at line {i+1} ---")
        start = max(0, i - 5)
        end = min(len(lines), i + 20)
        for j in range(start, end):
            print(lines[j])
        print("-------------------------")
        
if not found_error:
    print(f"No errors found in {log_file} (maybe warnings only?)")
    # Print last 20 lines just in case
    print("--- Last 20 lines ---")
    for line in lines[-20:]:
        print(line)
