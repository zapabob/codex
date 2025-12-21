---
name: plan
description: Generate a plan for how an agent should accomplish a complex coding task. Use when a user asks for a plan, and optionally when they want to save, find, read, update, or delete plan files. For full Plan Mode features (approval, execution, budgets), use official commands (`/Plan`, `/approve`, `/Plan export`). For simple markdown files, use $CODEX_HOME/plans (default ~/.codex/plans).
metadata:
  short-description: Generate a plan for a complex task
---

# Plan

## Overview

Draft structured plans that clarify intent, scope, requirements, action items, testing/validation, and risks.

Optionally, save plans to disk as markdown files with YAML frontmatter and free-form content. When drafting in chat, output only the plan body without frontmatter; add frontmatter only when saving to disk. Only write to the plans folder; do not modify the repository codebase.

This skill can also be used to draft codebase or system overviews.

## Core rules

- Resolve the plans directory as `$CODEX_HOME/Plans` or `~/.codex/Plans` when `CODEX_HOME` is not set (matches official Plan Mode CLI).
- Create the plans directory if it does not exist.
- Never write to the repo; only read files to understand context.
- Require frontmatter with **only** `name` and `description` (single-line values) for on-disk plans.
- When presenting a draft plan in chat, omit frontmatter and start at `# Plan`.
- Enforce naming rules: short, lower-case, hyphen-delimited; filename must equal `<name>.md`.
- If a plan is not found, state it clearly and offer to create one.
- Allow overview-style plans that document flows, architecture, or context without a work checklist.

## Decide the task

1. **Find/list**: discover plans by frontmatter summary; confirm if multiple matches exist.
2. **Read/use**: validate frontmatter; present summary and full contents.
3. **Create**: inspect repo read-only; choose plan style (implementation vs overview); draft plan; write to plans directory only.
4. **Update**: load plan; revise content and/or description; preserve frontmatter keys; overwrite the plan file.
5. **Delete**: confirm intent, then remove the plan file if asked.

## Plan discovery

- Use official Plan Mode commands: `codex /Plan export <plan-id>` to view plans
- For listing plans, use `codex /Plan list` (CLI) or browse in the TUI/GUI
- Optional: Use `scripts/list_plans.ts` for quick frontmatter summaries (TypeScript helper)
- Optional: Use `scripts/read_plan_frontmatter.ts` to validate a specific plan (TypeScript helper)
- If name mismatches filename or frontmatter is missing fields, call it out and ask whether to fix.

## Plan creation workflow

1. Scan context quickly: read README.md and obvious docs (docs/, CONTRIBUTING.md, ARCHITECTURE.md); skim likely touched files; identify constraints (language, frameworks, CI/test commands, deployment).
2. Ask follow-ups only if blocked: at most 1-2 questions, prefer multiple-choice. If unsure but not blocked, state assumptions and proceed.
3. Identify scope, constraints, and data model/API implications (or capture existing behavior for an overview).
4. Draft either an ordered implementation plan or a structured overview plan with diagrams/notes as needed.
5. Immediately output the plan body only (no frontmatter), then ask the user if they want to 1. Make changes, 2. Implement it, 3. Save it.
6. If the user wants to save it:
   - **Preferred**: Use official Plan Mode: `codex /Plan "<title>"` to create a plan (saves to `~/.codex/Plans` or configured directory)
   - **Alternative**: Use TypeScript helper `scripts/create_plan.ts` to save to `$CODEX_HOME/plans` (for simple markdown files with frontmatter)


## Plan update workflow

- Re-read the plan and related code/docs before updating.
- Keep the plan name stable unless the user explicitly wants a rename.
- If renaming, update both frontmatter `name` and filename together.

## Official Plan Mode Commands (Preferred)

Use the official Plan Mode commands for creating, managing, and exporting plans:

```bash
# Create a plan
codex /Plan "Add request logging" --mode=orchestrated

# List plans
codex /Plan list

# Export a plan
codex /Plan export <plan-id> --format=md

# Approve a plan
codex /approve <plan-id>

# Reject a plan
codex /reject <plan-id> --reason="..."
```

See `docs/plan/README.md` and `docs/plan/slash-commands.md` for full documentation.

## Helper Scripts (TypeScript, Optional)

For simple markdown plan files with frontmatter (saved to `$CODEX_HOME/plans`), TypeScript helpers are available:

Create a plan file (body only; frontmatter is written for you). Run from the plan skill directory:

```bash
node ./scripts/create_plan.ts \
  --name codex-rate-limit-overview \
  --description "Scope and update plan for Codex rate limiting" \
  --body-file /tmp/plan-body.md
```

Read frontmatter summary for a plan (run from the plan skill directory):

```bash
node ./scripts/read_plan_frontmatter.ts ~/.codex/plans/codex-rate-limit-overview.md
```

List plan summaries (optional filter; run from the plan skill directory):

```bash
node ./scripts/list_plans.ts --query "rate limit"
```

**Note**: These TypeScript helpers work with simple markdown files in `$CODEX_HOME/plans`. For full Plan Mode features (approval, execution, budgets), use the official commands above.

## Plan file format

Use one of the structures below for the plan body. When drafting, output only the body (no frontmatter). When saving, prepend this frontmatter:

```markdown
---
name: <plan-name>
description: <1-line summary>
---
```

### Implementation plan body template

```markdown
# Plan

<1-3 sentences: intent, scope, and approach.>

## Requirements
- <Requirement 1>
- <Requirement 2>

## Scope
- In:
- Out:

## Files and entry points
- <File/module/entry point 1>
- <File/module/entry point 2>

## Data model / API changes
- <If applicable, describe schema or contract changes>

## Action items
[ ] <Step 1>
[ ] <Step 2>
[ ] <Step 3>
[ ] <Step 4>
[ ] <Step 5>
[ ] <Step 6>

## Testing and validation
- <Tests, commands, or validation steps>

## Risks and edge cases
- <Risk 1>
- <Risk 2>

## Open questions
- <Question 1>
- <Question 2>
```

### Overview plan body template

```markdown
# Plan

<1-3 sentences: intent and scope of the overview.>

## Overview
<Describe the system, flow, or architecture at a high level.>

## Diagrams
<Include text or Mermaid diagrams if helpful.>

## Key file references
- <File/module/entry point 1>
- <File/module/entry point 2>

## Auth / routing / behavior notes
- <Capture relevant differences (e.g., auth modes, routing paths).>

## Current status
- <What is live today vs pending work, if known.>

## Action items
- None (overview only).

## Testing and validation
- None (overview only).

## Risks and edge cases
- None (overview only).

## Open questions
- None.
```

## Writing guidance

- Start with 1 short paragraph describing intent and approach.
- Keep action items ordered and atomic (discovery -> changes -> tests -> rollout); use verb-first phrasing.
- Scale action item count to complexity (simple: 1-2; complex: up to about 10).
- Include file/entry-point hints and concrete validation steps where useful.
- Always include testing/validation and risks/edge cases in implementation plans; include safe rollout/rollback when relevant.
- Use open questions only when necessary (max 3).
- Avoid vague steps, micro-steps, and code snippets; keep the plan implementation-agnostic.
- For overview plans, keep action items minimal and set non-applicable sections to "None."
