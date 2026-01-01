#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const repoRoot = path.resolve(__dirname, '..');

function parseArgs(argv) {
  const args = {};
  for (let i = 2; i < argv.length; i += 1) {
    const arg = argv[i];
    if (!arg.startsWith('--')) {
      continue;
    }
    const key = arg.slice(2);
    const next = argv[i + 1];
    if (next && !next.startsWith('--')) {
      args[key] = next;
      i += 1;
    } else {
      args[key] = true;
    }
  }
  return args;
}

function run(command) {
  try {
    return execSync(command, { cwd: repoRoot, encoding: 'utf8' }).trim();
  } catch (error) {
    return `failed to run "${command}": ${error.message}`;
  }
}

function sanitize(value, fallback) {
  const base = (value || fallback || '').trim();
  if (!base) {
    return 'unknown';
  }
  return base.replace(/[^a-zA-Z0-9_-]/g, '-');
}

function usage() {
  return `Usage: node scripts/create-implementation-log.js \\\n+  --worktree <name> \\\n+  --functionality <summary> \\\n+  --verification <tests> \\\n+  --agent <agent-name> \\\n+  --ai <ai-model> \\\n+  [--prompt <prompt-or-ticket>] \\\n+  [--qc <qc-summary>] \\\n+  [--stats <statistical-result>] \\\n+  [--optimization <quantum-or-math-notes>] \\\n+  [--decision <final-decision>] \\\n+  [--tests <test-output>] \\\n+  [--notes <extra>]`;
}

const args = parseArgs(process.argv);

if (args.help || args.h) {
  console.log(usage());
  process.exit(0);
}

const required = ['worktree', 'functionality', 'verification', 'agent', 'ai'];
const missing = required.filter((key) => !args[key]);
if (missing.length > 0) {
  console.error(`Missing required arguments: ${missing.join(', ')}`);
  console.log(usage());
  process.exit(1);
}

const now = new Date();
const isoTimestamp = now.toISOString();
const datePart = isoTimestamp.slice(0, 10);
const timePart = isoTimestamp.slice(11, 19);
const timezone = process.env.TZ || 'UTC';
const localeTimestamp = now.toLocaleString('ja-JP', { timeZone: timezone });

const worktreeSlug = sanitize(args.worktree, path.basename(repoRoot));
let fileName = `${datePart}-${worktreeSlug}実装ログ.md`;
const logDir = path.join(repoRoot, '_doc');
if (!fs.existsSync(logDir)) {
  fs.mkdirSync(logDir, { recursive: true });
}
let filePath = path.join(logDir, fileName);
if (fs.existsSync(filePath)) {
  const suffix = timePart.replace(/:/g, '');
  fileName = `${datePart}-${worktreeSlug}-${suffix}実装ログ.md`;
  filePath = path.join(logDir, fileName);
}

const branch = run('git rev-parse --abbrev-ref HEAD');
const gitStatus = run('git status --short');
const prompt = args.prompt || 'N/A';
const tests = args.tests || '未実行 / 別所に記載';
const qcSummary = args.qc || '未記入';
const statsSummary = args.stats || '未記入';
const optimizationNotes = args.optimization || '未記入';
const decision = args.decision || '未記入';
const notes = args.notes || 'N/A';

const content = `# 実装ログ (${worktreeSlug})

- **作成日時 (UTC)**: ${isoTimestamp}
- **ローカルタイム (${timezone})**: ${localeTimestamp}
- **ワークツリー**: ${args.worktree}
- **ブランチ**: ${branch}
- **担当エージェント**: ${args.agent}
- **AIモデル**: ${args.ai}

## 1. 受け付けたプロンプト / 要件
${prompt}

## 2. 実装概要
- **機能**: ${args.functionality}
- **動作確認**: ${args.verification}

## 3. Git変更サマリ

\
\`\`\`
${gitStatus || '変更なし'}
\`\`\`

## 4. テスト結果
${tests}

## 5. QC・統計・最適化判断
- **QC結論**: ${qcSummary}
- **統計学的有意差**: ${statsSummary}
- **量子/数理最適化ノート**: ${optimizationNotes}
- **最終判断理由**: ${decision}

## 6. 追加メモ
${notes}
`;

fs.writeFileSync(filePath, content, 'utf8');
console.log(`Implementation log saved to ${path.relative(repoRoot, filePath)}`);
