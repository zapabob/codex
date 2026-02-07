import sys
import re
import os

def resolve_local(file_path):
    print(f"Resolving (Force Local) in: {file_path}")
    try:
        with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()
    except FileNotFoundError:
        print(f"File not found: {file_path}")
        return

    # Pattern to match git conflict markers
    # Handles:
    # <<<<<<< HEAD ...
    # content
    # =======
    # content
    # >>>>>>> ...
    # And markers potentially having text on the same line.
    
    # We want to keep GROUP 1 (Local).
    
    pattern = re.compile(
        r'<<<<<<< HEAD.*?\n(.*?)\n=======\n(?:.*?)\n>>>>>>> .*?\n',
        re.DOTALL
    )
    
    # Also handle case where marker doesn't have newline immediately if start of file? 
    # Or if text is on same line as marker.
    # Logic: 
    # 1. Find <<<<<<< HEAD
    # 2. Keep content until =======
    # 3. Discard content until >>>>>>>
    
    # The regex above expects newline after HEAD and ======= and >>>>>>>. 
    # If markers are mixed with code on same line, it's safer to process line by line.

    lines = content.splitlines(keepends=True)
    new_lines = []
    in_conflict = False
    in_upstream = False
    
    for line in lines:
        if line.startswith("<<<<<<< HEAD"):
            in_conflict = True
            in_upstream = False
            # If there is content on the same line after <<<<<<< HEAD, we should keep it?
            # Usually git puts marker on its own line unless manual edit messed it up.
            # But earlier we saw `<<<<<<< HEAD    fn ...`
            # So we should strip the marker and keep the rest of the line.
            clean_line = line.replace("<<<<<<< HEAD", "", 1)
            if clean_line.strip():
                new_lines.append(clean_line)
            continue
            
        if line.startswith("======="):
            if in_conflict:
                in_upstream = True
                continue
        
        if line.startswith(">>>>>>>"):
            if in_conflict:
                in_conflict = False
                in_upstream = False
                continue

        if in_conflict:
            if not in_upstream:
                new_lines.append(line)
        else:
            new_lines.append(line)

    new_content = "".join(new_lines)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(new_content)

if __name__ == "__main__":
    files = [
        "codex-rs/app-server/src/bespoke_event_handling.rs",
        "codex-rs/app-server/src/lib.rs",
        "codex-rs/tui/src/chatwidget.rs",
        "codex-rs/tui/src/history_cell.rs",
        "codex-rs/tui/src/updates.rs"
    ]
    for f in files:
        if os.path.exists(f):
            resolve_local(f)
        else:
            print(f"Skipping missing file: {f}")
