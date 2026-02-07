import subprocess

try:
    with open("status.txt", "w", encoding="utf-8") as f:
        result = subprocess.run(["git", "status", "--short"], capture_output=True, text=True, encoding='utf-8')
        f.write(result.stdout)
except Exception as e:
    print(f"Error: {e}")
