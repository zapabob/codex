import sys

try:
    with open("core_error.log", "r", encoding="utf-8", errors="replace") as f:
        lines = f.readlines()
        
    found_error = False
    for i, line in enumerate(lines):
        if "error[" in line or "error:" in line:
            found_error = True
            print(f"--- Error at line {i+1} ---")
            start = max(0, i - 5)
            end = min(len(lines), i + 20)
            for j in range(start, end):
                print(lines[j].rstrip())
            print("-------------------------")
            
    if not found_error:
        print("No errors found in log (maybe warnings only?)")
        # Print last 20 lines just in case
        print("--- Last 20 lines ---")
        for line in lines[-20:]:
            print(line.rstrip())

except FileNotFoundError:
    print("core_error.log not found.")
except Exception as e:
    print(f"Error: {e}")
