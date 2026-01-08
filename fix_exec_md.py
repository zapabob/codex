#!/usr/bin/env python3
"""
docs/exec.mdの競合を修正
"""

# Read the file
with open('docs/exec.md', 'r', encoding='utf-8') as f:
    content = f.read()

# Remove conflict markers and keep HEAD version + add upstream link
content = content.replace('''<<<<<<< HEAD
Use Codex in non-interactive mode to automate common workflows.

```shell
codex exec "count the total number of lines of code in this project"
```

> [!NOTE]
> When launching Codex through package runners such as `npx`/`npm exec` or `pnpm exec`, include `--` before the `exec` subcommand
> to force non-interactive mode (for example, `npx @openai/codex -- exec "list stale branches"`). Without the separator, the
> package runner may forward the word `exec` as part of your prompt and Codex will stay in the interactive TUI instead of running
> the automation flow.

In non-interactive mode, Codex does not ask for command or edit approvals. By default it runs in `read-only` mode, so it cannot edit files or run commands that require network access.

Use `codex exec --full-auto` to allow file edits. Use `codex exec --sandbox danger-full-access` to allow edits and networked commands.

### Default output mode

By default, Codex streams its activity to stderr and only writes the final message from the agent to stdout. This makes it easier to pipe `codex exec` into another tool without extra filtering.

To write the output of `codex exec` to a file, in addition to using a shell redirect like `>`, there is also a dedicated flag to specify an output file: `-o`/`--output-last-message`.

### JSON output mode

`codex exec` supports a `--json` mode that streams events to stdout as JSON Lines (JSONL) while the agent runs.

Supported event types:

- `thread.started` - when a thread is started or resumed.
- `turn.started` - when a turn starts. A turn encompasses all events between the user message and the assistant response.
- `turn.completed` - when a turn completes; includes token usage.
- `turn.failed` - when a turn fails; includes error details.
- `item.started`/`item.updated`/`item.completed` - when a thread item is added/updated/completed.
- `error` - when the stream reports an unrecoverable error; includes the error message.

Supported item types:

- `agent_message` - assistant message.
- `reasoning` - a summary of the assistant's thinking.
- `command_execution` - assistant executing a command.
- `file_change` - assistant making file changes.
- `mcp_tool_call` - assistant calling an MCP tool.
- `web_search` - assistant performing a web search.
- `todo_list` - the agent's running plan when the plan tool is active, updating as steps change.

Typically, an `agent_message` is added at the end of the turn.

Sample output:

```jsonl
{"type":"thread.started","thread_id":"0199a213-81c0-7800-8aa1-bbab2a035a53"}''', '''Use Codex in non-interactive mode to automate common workflows.

```shell
codex exec "count the total number of lines of code in this project"
```

> [!NOTE]
> When launching Codex through package runners such as `npx`/`npm exec` or `pnpm exec`, include `--` before the `exec` subcommand
> to force non-interactive mode (for example, `npx @openai/codex -- exec "list stale branches"`). Without the separator, the
> package runner may forward the word `exec` as part of your prompt and Codex will stay in the interactive TUI instead of running
> the automation flow.

In non-interactive mode, Codex does not ask for command or edit approvals. By default it runs in `read-only` mode, so it cannot edit files or run commands that require network access.

Use `codex exec --full-auto` to allow file edits. Use `codex exec --sandbox danger-full-access` to allow edits and networked commands.

### Default output mode

By default, Codex streams its activity to stderr and only writes the final message from the agent to stdout. This makes it easier to pipe `codex exec` into another tool without extra filtering.

To write the output of `codex exec` to a file, in addition to using a shell redirect like `>`, there is also a dedicated flag to specify an output file: `-o`/`--output-last-message`.

### JSON output mode

`codex exec` supports a `--json` mode that streams events to stdout as JSON Lines (JSONL) while the agent runs.

Supported event types:

- `thread.started` - when a thread is started or resumed.
- `turn.started` - when a turn starts. A turn encompasses all events between the user message and the assistant response.
- `turn.completed` - when a turn completes; includes token usage.
- `turn.failed` - when a turn fails; includes error details.
- `item.started`/`item.updated`/`item.completed` - when a thread item is added/updated/completed.
- `error` - when the stream reports an unrecoverable error; includes the error message.

Supported item types:

- `agent_message` - assistant message.
- `reasoning` - a summary of the assistant's thinking.
- `command_execution` - assistant executing a command.
- `file_change` - assistant making file changes.
- `mcp_tool_call` - assistant calling an MCP tool.
- `web_search` - assistant performing a web search.
- `todo_list` - the agent's running plan when the plan tool is active, updating as steps change.

Typically, an `agent_message` is added at the end of the turn.

Sample output:

```jsonl
{"type":"thread.started","thread_id":"0199a213-81c0-7800-8aa1-bbab2a035a53"}''')

# Add upstream documentation link at the end
content = content.replace('''=======
For information about non-interactive mode, see [this documentation](https://developers.openai.com/codex/noninteractive).
>>>>>>> upstream/main''', '''

For more information about non-interactive mode, see [the official documentation](https://developers.openai.com/codex/noninteractive).''')

# Write back
with open('docs/exec.md', 'w', encoding='utf-8') as f:
    f.write(content)

print("Fixed conflicts in docs/exec.md")