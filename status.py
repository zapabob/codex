import subprocess

try:
    result = subprocess.run(["git", "status", "--short"], capture_output=True, text=True, encoding='utf-8')
    print(result.stdout)
except Exception as e:
    print(f"Error: {e}")
