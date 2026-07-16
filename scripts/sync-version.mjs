#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';

const repoRoot = process.cwd();
const isCheck = process.argv.includes('--check');
const metadataPath = path.join(repoRoot, 'version-metadata.json');
const versionPath = path.join(repoRoot, 'VERSION');

function readText(relPath) {
  return fs.readFileSync(path.join(repoRoot, relPath), 'utf8');
}

function writeText(relPath, content) {
  fs.writeFileSync(path.join(repoRoot, relPath), content);
}

function readJson(relPath) {
  return JSON.parse(readText(relPath));
}

function writeJson(relPath, value) {
  writeText(relPath, `${JSON.stringify(value, null, 2)}\n`);
}

function updateFile(relPath, nextContent) {
  const current = fs.existsSync(path.join(repoRoot, relPath)) ? readText(relPath) : null;
  if (current === nextContent) {
    return false;
  }
  if (isCheck) {
    throw new Error(`${relPath} is out of sync`);
  }
  writeText(relPath, nextContent);
  return true;
}

function replaceMatch(source, pattern, replacement, description) {
  if (!pattern.test(source)) {
    throw new Error(`Unable to update ${description}`);
  }
  pattern.lastIndex = 0;
  return source.replace(pattern, replacement);
}

const metadata = readJson('version-metadata.json');
if (metadata.canonical_source !== 'VERSION') {
  throw new Error(`Unsupported canonical source: ${metadata.canonical_source}`);
}

const versionFromFile = fs.readFileSync(versionPath, 'utf8').trim();
const canonicalVersion = versionFromFile;
const releaseDate = metadata.release_date;

if (metadata.fork_version !== canonicalVersion) {
  if (isCheck) {
    throw new Error(`version-metadata.json fork_version (${metadata.fork_version}) does not match VERSION (${canonicalVersion})`);
  }
  metadata.fork_version = canonicalVersion;
  writeJson('version-metadata.json', metadata);
}

const filesUpdated = [];

const rootPackage = readJson('package.json');
rootPackage.version = canonicalVersion;
if (!rootPackage.scripts['version:sync']) {
  rootPackage.scripts['version:sync'] = 'node scripts/sync-version.mjs';
}
if (!rootPackage.scripts['version:check']) {
  rootPackage.scripts['version:check'] = 'node scripts/sync-version.mjs --check';
}
if (JSON.stringify(rootPackage, null, 2) + '\n' !== readText('package.json')) {
  if (isCheck) throw new Error('package.json is out of sync');
  writeJson('package.json', rootPackage);
  filesUpdated.push('package.json');
}

for (const relPath of metadata.sync_targets.package_json) {
  const pkg = readJson(relPath);
  if (pkg.version !== canonicalVersion) {
    pkg.version = canonicalVersion;
    if (isCheck) throw new Error(`${relPath} is out of sync`);
    writeJson(relPath, pkg);
    filesUpdated.push(relPath);
  }
}

const cargoPath = metadata.sync_targets.cargo_workspace;
const cargoToml = readText(cargoPath);
const nextCargo = replaceMatch(
  cargoToml,
  /(\[workspace\.package\][\s\S]*?version = ")([^"]+)(")/,
  `$1${canonicalVersion}$3`,
  'workspace package version',
);
if (nextCargo !== cargoToml) {
  if (isCheck) throw new Error(`${cargoPath} is out of sync`);
  writeText(cargoPath, nextCargo);
  filesUpdated.push(cargoPath);
}

const changelogArchive = metadata.sync_targets.archives.changelog;
const releaseArchive = metadata.sync_targets.archives.release_notes;

const changelog = `# Changelog\n\nCurrent canonical version: **v${canonicalVersion}**.\nCanonical source: \`VERSION\`. Fork/upstream disambiguation lives in \`version-metadata.json\`.\n\n## Current Release — v${canonicalVersion} (${releaseDate})\n\n> This root changelog is the **current release line only**.\n> Legacy v2.x history has been moved to \`${changelogArchive}\` to make the latest release immediately obvious.\n\n### Changed\n\n- Adopted **root \`VERSION\`** as the single canonical version source for release-visible artifacts.\n- Added a machine-readable version metadata file with \`fork_version\` and \`upstream_base\` for fork/upstream conflict resolution.\n- Added generated sync automation for root/package manifests, workspace Cargo version, README display version, release notes, and changelog headers.\n\n### Docs\n\n- Split legacy **v2.x** history from the current **v3.x** release line.\n- Marked the root release notes and changelog as the current release documents.\n- Standardized the displayed release version across README badges and package metadata.\n\n## Historical Release Lines\n\n- **v2.x archive**: \`${changelogArchive}\`\n- **Legacy release notes**: \`${releaseArchive}\`\n`;
if (updateFile('CHANGELOG.md', changelog)) {
  filesUpdated.push('CHANGELOG.md');
}

const releaseNotes = `# Codex v${canonicalVersion} Release Notes\n\n> **Current release document** for the v${canonicalVersion} line.\n> Legacy v2.x release notes are archived at \`${releaseArchive}\`.\n\n## Canonical Versioning\n\n- **Canonical source**: root \`VERSION\`\n- **Fork version**: \`${canonicalVersion}\`\n- **Upstream base**: \`${metadata.upstream_base}\`\n- **Release date**: ${releaseDate}\n\n## What changed in v${canonicalVersion}\n\n### Version governance\n\n- Root \`VERSION\` is now the single source of truth for release-visible versioning.\n- \`version-metadata.json\` defines \`fork_version\` and \`upstream_base\` so tooling can distinguish fork releases from upstream alignment.\n- \`scripts/sync-version.mjs\` regenerates synced version displays and validates drift with \`--check\`.\n\n### Repository docs and manifests\n\n- Synced the root \`package.json\`, Rust workspace version, and \`packages/protocol-client/package.json\` to v${canonicalVersion}.\n- Rebuilt the root changelog and release notes as **current release** documents for the v3.x line.\n- Archived the older v2.x release notes so the latest release is unambiguous.\n\n## Sync procedure\n\n\`\`\`bash\n# 1) edit VERSION (and version-metadata.json upstream_base if needed)\nnode scripts/sync-version.mjs\n\n# 2) verify no drift remains\nnode scripts/sync-version.mjs --check\n\`\`\`\n`;
if (updateFile('releases/RELEASE_NOTES.md', releaseNotes)) {
  filesUpdated.push('releases/RELEASE_NOTES.md');
}

let readme = readText('README.md');
readme = replaceMatch(
  readme,
  /> Current release line: `v[^`]+` \([^)]+\)/,
  `> Current release line: \`v${canonicalVersion}\` (${releaseDate})`,
  'README current release line',
);
readme = replaceMatch(
  readme,
  /> Official base: `[^`]+` plus upstream main `[^`]+`/,
  `> Official base: \`${metadata.upstream_base}\` plus upstream main \`${metadata.upstream_main_commit}\``,
  'README official base',
);
readme = replaceMatch(
  readme,
  /- Bumps the fork semantic version to `[^`]+` because this line adds official upstream capabilities while keeping backward-compatible zapabob extension behavior\./,
  `- Bumps the fork semantic version to \`${canonicalVersion}\` because this line adds official upstream capabilities while keeping backward-compatible zapabob extension behavior.`,
  'README semantic version note',
);
readme = replaceMatch(
  readme,
  /- latest official release: `[^`]+`/,
  `- latest official release: \`${metadata.upstream_base}\``,
  'README latest official release',
);
readme = replaceMatch(
  readme,
  /- release publication time: [^\n]+/,
  `- release publication time: ${metadata.upstream_release_published_at}`,
  'README release publication time',
);
readme = replaceMatch(
  readme,
  /- latest observed upstream main commit: `[^`]+`/,
  `- latest observed upstream main commit: \`${metadata.upstream_main_commit}\``,
  'README observed upstream main commit',
);
readme = replaceMatch(
  readme,
  /- latest observed upstream main commit time: [^\n]+/,
  `- latest observed upstream main commit time: ${metadata.upstream_main_observed_at}`,
  'README observed upstream main commit time',
);
if (/- file: `releases\/codex-v[^`]+-windows-x86_64\.tar\.gz`/.test(readme)) {
  readme = replaceMatch(
    readme,
    /- file: `releases\/codex-v[^`]+-windows-x86_64\.tar\.gz`/,
    `- file: \`releases/codex-v${canonicalVersion}-windows-x86_64.tar.gz\``,
    'README current Windows asset file',
  );
}
const managedBlock = `<!-- version-sync:start -->\n> **Current release:** v${canonicalVersion} (${releaseDate}) · canonical source \`VERSION\` · fork/upstream mapping in \`version-metadata.json\`.\n> Legacy v2.x release notes are archived under \`${releaseArchive}\`.\n<!-- version-sync:end -->`;
if (/<!-- version-sync:start -->[\s\S]*?<!-- version-sync:end -->/.test(readme)) {
  readme = readme.replace(/<!-- version-sync:start -->[\s\S]*?<!-- version-sync:end -->/, managedBlock);
} else {
  readme = readme.replace('</div>\n\n---', `</div>\n\n${managedBlock}\n\n---`);
}
if (updateFile('README.md', readme)) {
  filesUpdated.push('README.md');
}

let bumpScript = readText('scripts/bump-version.ps1');
bumpScript = bumpScript.replace(
  /Write-Host "  1\. CHANGELOG\.md を更新"[\s\S]*$/,
  `Write-Host "  1. node scripts/sync-version.mjs を実行" -ForegroundColor Yellow\nWrite-Host "  2. node scripts/sync-version.mjs --check で整合性確認" -ForegroundColor Yellow\nWrite-Host "  3. git commit -m 'chore: bump version to ${'$'}NewVersion'" -ForegroundColor Yellow\n`,
);
if (updateFile('scripts/bump-version.ps1', bumpScript)) {
  filesUpdated.push('scripts/bump-version.ps1');
}

if (filesUpdated.length === 0) {
  console.log(`Version artifacts already synced at v${canonicalVersion}`);
} else {
  console.log(`Synced ${filesUpdated.length} files to v${canonicalVersion}`);
  for (const file of filesUpdated) {
    console.log(`- ${file}`);
  }
}
