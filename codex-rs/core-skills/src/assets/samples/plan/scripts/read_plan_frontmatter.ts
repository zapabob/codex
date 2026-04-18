#!/usr/bin/env node
/**
 * Read plan frontmatter without loading the full markdown body.
 */

import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';
import { parseFrontmatter } from './plan_utils.js';

function main(): number {
  const args: {
    planPath?: string;
    json?: boolean;
  } = {};

  for (let i = 2; i < process.argv.length; i++) {
    const arg = process.argv[i];
    if (i === 2 && !arg.startsWith('--')) {
      args.planPath = arg;
    } else if (arg === '--json') {
      args.json = true;
    }
  }

  if (!args.planPath) {
    console.error('Usage: read_plan_frontmatter.ts <plan_path> [--json]');
    process.exit(1);
  }

  const filePath = args.planPath.replace(/^~/, os.homedir());
  if (!fs.existsSync(filePath)) {
    console.error(`Plan not found: ${filePath}`);
    process.exit(1);
  }

  try {
    const data = parseFrontmatter(filePath);
    const name = data.name;
    const description = data.description;

    if (!name || !description) {
      console.error('Frontmatter must include name and description.');
      process.exit(1);
    }

    const payload = { name, description, path: filePath };
    if (args.json) {
      console.log(JSON.stringify(payload));
    } else {
      console.log(`name: ${name}`);
      console.log(`description: ${description}`);
      console.log(`path: ${filePath}`);
    }
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }

  return 0;
}

// Run if executed directly
if (import.meta.url.endsWith(process.argv[1]) || process.argv[1]?.includes('read_plan_frontmatter')) {
  process.exit(main());
}

