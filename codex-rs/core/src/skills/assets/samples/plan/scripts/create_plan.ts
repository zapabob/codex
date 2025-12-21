#!/usr/bin/env node
/**
 * Create or overwrite a plan markdown file in $CODEX_HOME/plans.
 */

import * as fs from 'fs';
import * as path from 'path';
import { getPlansDir, validatePlanName } from './plan_utils.js';

const DEFAULT_TEMPLATE = `# Plan

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
`;

function readBody(args: {
  template?: boolean;
  bodyFile?: string;
}): string | null {
  if (args.template) {
    return DEFAULT_TEMPLATE;
  }
  if (args.bodyFile) {
    return fs.readFileSync(args.bodyFile, 'utf-8');
  }
  // Check if stdin is available (non-interactive)
  if (!process.stdin.isTTY) {
    return fs.readFileSync(0, 'utf-8');
  }
  return null;
}

function main(): number {
  const args: {
    name?: string;
    description?: string;
    bodyFile?: string;
    template?: boolean;
    overwrite?: boolean;
  } = {};
  
  // Simple argument parsing (can be enhanced with a proper CLI library)
  for (let i = 2; i < process.argv.length; i++) {
    const arg = process.argv[i];
    if (arg === '--name' && i + 1 < process.argv.length) {
      args.name = process.argv[++i];
    } else if (arg === '--description' && i + 1 < process.argv.length) {
      args.description = process.argv[++i];
    } else if (arg === '--body-file' && i + 1 < process.argv.length) {
      args.bodyFile = process.argv[++i];
    } else if (arg === '--template') {
      args.template = true;
    } else if (arg === '--overwrite') {
      args.overwrite = true;
    }
  }

  if (!args.name || !args.description) {
    console.error('Usage: create_plan.ts --name <name> --description <description> [--body-file <file>] [--template] [--overwrite]');
    process.exit(1);
  }

  const name = args.name.trim();
  const description = args.description.trim();
  
  try {
    validatePlanName(name);
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }

  if (!description || description.includes('\n')) {
    console.error('Description must be a single line.');
    process.exit(1);
  }

  const body = readBody(args);
  if (!body) {
    console.error('Provide --body-file, stdin, or --template to supply plan content.');
    process.exit(1);
  }

  const trimmedBody = body.trim();
  if (!trimmedBody) {
    console.error('Plan body cannot be empty.');
    process.exit(1);
  }
  if (trimmedBody.trimStart().startsWith('---')) {
    console.error('Plan body should not include frontmatter.');
    process.exit(1);
  }

  const plansDir = getPlansDir();
  fs.mkdirSync(plansDir, { recursive: true });
  const planPath = path.join(plansDir, `${name}.md`);

  if (fs.existsSync(planPath) && !args.overwrite) {
    console.error(`Plan already exists: ${planPath}. Use --overwrite to replace.`);
    process.exit(1);
  }

  const content = `---\nname: ${name}\ndescription: ${description}\n---\n\n${trimmedBody}\n`;
  fs.writeFileSync(planPath, content, 'utf-8');
  console.log(planPath);
  return 0;
}

// Run if executed directly
if (import.meta.url.endsWith(process.argv[1]) || process.argv[1]?.includes('create_plan')) {
  process.exit(main());
}

