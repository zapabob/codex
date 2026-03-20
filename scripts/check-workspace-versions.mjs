import { readFileSync } from 'node:fs';

const rootVersion = JSON.parse(readFileSync('package.json', 'utf8')).version;
const packageFiles = [
  'gui/package.json',
  'sdk/package.json',
  'sdk/typescript/package.json',
  'packages/protocol-client/package.json',
  'shell-tool-mcp/package.json',
  'extensions/package.json',
  'extensions/vscode-codex/package.json',
  'extensions/windsurf-extension/package.json',
  'extensions/codex-viz-web/frontend/package.json',
  'extensions/codex-viz-web/desktop/package.json',
];

let hasError = false;
for (const file of packageFiles) {
  const pkg = JSON.parse(readFileSync(file, 'utf8'));
  if (pkg.version !== rootVersion) {
    console.error(`${file}: expected version ${rootVersion}, found ${pkg.version}`);
    hasError = true;
  }
}

const vscodeCodex = JSON.parse(readFileSync('extensions/vscode-codex/package.json', 'utf8'));
if (vscodeCodex.dependencies?.['@zapabob/codex-protocol-client'] !== 'file:../../packages/protocol-client') {
  console.error(
    'extensions/vscode-codex/package.json: expected @zapabob/codex-protocol-client to use file:../../packages/protocol-client',
  );
  hasError = true;
}

if (hasError) {
  process.exit(1);
}

console.log(`Workspace package versions are aligned at ${rootVersion}.`);
