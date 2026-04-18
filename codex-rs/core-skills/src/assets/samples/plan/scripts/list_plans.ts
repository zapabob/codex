#!/usr/bin/env node
/**
 * List plan summaries by reading frontmatter only.
 */

import * as fs from 'fs';
import * as path from 'path';
import { getPlansDir, parseFrontmatter } from './plan_utils.js';

interface PlanItem {
  name: string;
  description: string;
  path: string;
}

function main(): number {
  const args: {
    query?: string;
    json?: boolean;
  } = {};

  for (let i = 2; i < process.argv.length; i++) {
    const arg = process.argv[i];
    if (arg === '--query' && i + 1 < process.argv.length) {
      args.query = process.argv[++i];
    } else if (arg === '--json') {
      args.json = true;
    }
  }

  const plansDir = getPlansDir();
  if (!fs.existsSync(plansDir)) {
    console.error(`Plans directory not found: ${plansDir}`);
    process.exit(1);
  }

  const query = args.query?.toLowerCase();
  const items: PlanItem[] = [];

  const files = fs.readdirSync(plansDir)
    .filter(f => f.endsWith('.md'))
    .sort();

  for (const file of files) {
    const filePath = path.join(plansDir, file);
    try {
      const data = parseFrontmatter(filePath);
      const name = data.name;
      const description = data.description;
      
      if (!name || !description) {
        continue;
      }

      if (query) {
        const haystack = `${name} ${description}`.toLowerCase();
        if (!haystack.includes(query)) {
          continue;
        }
      }

      items.push({ name, description, path: filePath });
    } catch (error) {
      // Skip invalid files
      continue;
    }
  }

  if (args.json) {
    console.log(JSON.stringify(items));
  } else {
    for (const item of items) {
      console.log(`${item.name}\t${item.description}\t${item.path}`);
    }
  }

  return 0;
}

// Run if executed directly
if (import.meta.url.endsWith(process.argv[1]) || process.argv[1]?.includes('list_plans')) {
  process.exit(main());
}

