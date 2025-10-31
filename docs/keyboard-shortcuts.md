# Keyboard Shortcuts

Codex GUI provides global keyboard shortcuts for common operations.

## Available Shortcuts

| Action | Mac | Windows/Linux | Description |
|--------|-----|---------------|-------------|
| Run Command | ⌘+Enter | Ctrl+Enter | Execute the current command or code |
| Commit Changes | ⌘+S | Ctrl+S | Commit changes to Git |
| Push Changes | ⌘+Shift+S | Ctrl+Shift+S | Push changes to remote repository |
| Show Diff | ⌘+D | Ctrl+D | Show git diff |
| Revert Last Change | ⌘+Z | Ctrl+Z | Revert last change |
| Show Help | ? | ? | Toggle keyboard shortcuts help |

## Usage

Keyboard shortcuts are active throughout the application, except when typing in input fields. To see the full list of shortcuts at any time, press `?` to open the help modal.

## Accessibility

All shortcuts include proper ARIA attributes for screen readers:
- Buttons display shortcut hints in tooltips
- Shortcuts are announced to screen readers via `aria-keyshortcuts` attribute

## Customization

Keyboard shortcuts can be disabled by setting the environment variable:
```bash
CODEX_DISABLE_SHORTCUTS=true
```
