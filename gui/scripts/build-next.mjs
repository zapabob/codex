import { spawn } from 'node:child_process'
import { mkdir, writeFile } from 'node:fs/promises'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const projectRoot = path.resolve(scriptDir, '..')
const nextDir = path.join(projectRoot, '.next')
const serverDir = path.join(nextDir, 'server')
const pagesManifestPath = path.join(serverDir, 'pages-manifest.json')
const nextBin = path.join(projectRoot, 'node_modules', 'next', 'dist', 'bin', 'next')

const seedPagesManifest = {
  '/_app': 'pages/_app.js',
  '/_document': 'pages/_document.js',
  '/_error': 'pages/_error.js',
}

await mkdir(serverDir, { recursive: true })
await writeFile(
  pagesManifestPath,
  `${JSON.stringify(seedPagesManifest, null, 2)}\n`,
  'utf8'
)

const child = spawn(process.execPath, [nextBin, 'build'], {
  cwd: projectRoot,
  env: process.env,
  stdio: 'inherit',
})

child.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal)
    return
  }

  process.exit(code ?? 1)
})
