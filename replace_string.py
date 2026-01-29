import sys
import os

def replace_in_file(path, old, new):
    if not os.path.exists(path):
        print(f"File not found: {path}")
        return
    try:
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        if old in content:
            new_content = content.replace(old, new)
            with open(path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            print(f"Updated {path}")
        else:
            print(f"String '{old}' not found in {path}")
    except Exception as e:
        print(f"Error processing {path}: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage: python replace.py <file> <old> <new>")
        sys.exit(1)
    
    path = sys.argv[1]
    old = sys.argv[2]
    new = sys.argv[3]
    replace_in_file(path, old, new)
