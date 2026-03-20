import { spawnSync } from 'node:child_process';

const tsPackages = [
  'gui',
  'sdk',
  'sdk/typescript',
  'packages/protocol-client',
  'shell-tool-mcp',
  'extensions',
  'extensions/vscode-codex',
  'extensions/windsurf-extension',
  'extensions/codex-viz-web/frontend',
  'extensions/codex-viz-web/desktop',
];

function run(command, args) {
  const result = spawnSync(command, args, {
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });

  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

for (const pkg of tsPackages) {
  console.log(`\n==> type-check: ${pkg}`);
  run('npm', ['--prefix', pkg, 'run', 'type-check']);
}

console.log('\n==> type-check: python');
run('pyright', ['--project', 'pyrightconfig.json']);
