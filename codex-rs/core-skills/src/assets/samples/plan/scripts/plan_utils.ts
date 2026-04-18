#!/usr/bin/env node
/**
 * Shared helpers for plan scripts.
 */

import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';

const NAME_RE = /^[a-z0-9]+(-[a-z0-9]+)*$/;

export function getCodexHome(): string {
  const codexHome = process.env.CODEX_HOME;
  if (codexHome) {
    return codexHome.replace(/^~/, os.homedir());
  }
  return path.join(os.homedir(), '.codex');
}

export function getPlansDir(): string {
  // Use lowercase 'plans' for simple markdown files (plan skill)
  // Official Plan Mode uses 'Plans' (capital P) - see codex-rs/cli/src/plan_commands.rs
  return path.join(getCodexHome(), 'plans');
}

export function validatePlanName(name: string): void {
  if (!name || !NAME_RE.test(name)) {
    throw new Error(
      'Invalid plan name. Use short, lower-case, hyphen-delimited names ' +
      '(e.g., codex-rate-limit-overview).'
    );
  }
}

export interface Frontmatter {
  [key: string]: string;
}

export function parseFrontmatter(filePath: string): Frontmatter {
  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split('\n');
  
  if (lines[0]?.trim() !== '---') {
    throw new Error("Frontmatter must start with '---'.");
  }

  const data: Frontmatter = {};
  let foundClosing = false;

  for (let i = 1; i < lines.length; i++) {
    const line = lines[i];
    const stripped = line.trim();
    
    if (stripped === '---') {
      foundClosing = true;
      break;
    }
    
    if (!stripped || stripped.startsWith('#')) {
      continue;
    }
    
    if (!line.includes(':')) {
      throw new Error(`Invalid frontmatter line: ${line}`);
    }
    
    const colonIndex = line.indexOf(':');
    const key = line.substring(0, colonIndex).trim();
    let value = line.substring(colonIndex + 1).trim();
    
    // Remove quotes if present
    if (value.length >= 2 && 
        ((value[0] === '"' && value[value.length - 1] === '"') ||
         (value[0] === "'" && value[value.length - 1] === "'"))) {
      value = value.slice(1, -1);
    }
    
    data[key] = value;
  }

  if (!foundClosing) {
    throw new Error("Frontmatter must end with '---'.");
  }

  return data;
}

