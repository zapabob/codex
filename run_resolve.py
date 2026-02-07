import subprocess
import sys
import os

# Add current directory to path so we can import resolve_conflicts
sys.path.append(os.getcwd())

try:
    import resolve_conflicts
except ImportError:
    print("Could not import resolve_conflicts.py. Make sure it is in the current directory.")
    sys.exit(1)

def main():
    try:
        # Get list of conflicted files using git
        # diff-filter=U selects files with Unmerged status
        result = subprocess.run(
            ["git", "diff", "--name-only", "--diff-filter=U"],
            capture_output=True,
            text=True,
            check=True,
            encoding='utf-8' 
        )
        files = result.stdout.strip().split('\n')
        
        files = [f.strip() for f in files if f.strip()]
        
        if not files:
            print("No conflicted files found.")
            return

        print(f"Found {len(files)} conflicted files:")
        for f in files:
            print(f" - {f}")

        print("\nStarting resolution...")
        for f in files:
            resolve_conflicts.resolve_conflict(f)
            
        print("\nResolution complete. Please check git status.")

    except subprocess.CalledProcessError as e:
        print(f"Error running git command: {e}")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")

if __name__ == "__main__":
    main()
