# GUITUICLI Test Results

**Date**: 2025-12-12 04:34:51
**Task**: GUI/CLI/Playwright Integration Testing

---

## System Information

- **Platform**: Windows
- **Version**: 10.0.26200
- **Python**: 3.12.9
- **CPU Cores**: 12
- **Hostname**: downl

## Test Summary

- **Total Tests**: 7
- **Passed**: 3
- **Failed**: 4
.1f.2f
---

## Detailed Results

- [DIR] cli_tests
  - [OK] CLI Version Check
  - [OK] CLI Help Display
  - [ERROR] CLI Exec Command
    - Error: OpenAI Codex v2.3.2 (research preview)
--------
workdir: C:\Users\downl\Desktop\codex-main
model: gpt-5-codex
provider: openai
approval: never
sandbox: danger-full-access
reasoning effort: high
reason...
- [DIR] gui_tests
  - [ERROR] GUI Dependencies Install
    - Error: [WinError 2] 指定されたファイルが見つかりません。...
- [DIR] tui_tests
  - [ERROR] TUI Help Display
    - Error: Command timed out after 30s...
- [DIR] playwright_tests
  - [ERROR] gui_server_access
  - [OK] cursor_browser_check
- [DIR] integration_tests
  - [ERROR] CLI-to-GUI Pipeline
    - [ERROR] CLI exec test
    - [OK] Plan listing via CLI
  - [ERROR] Version Consistency Check
    - [OK] CLI version
    - [ERROR] Direct binary version

---

## Completion Notification

Test completed successfully.
