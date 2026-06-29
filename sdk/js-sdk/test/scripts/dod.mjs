import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { resolve, dirname } from 'node:path';

const sdkRoot = resolve(dirname(fileURLToPath(import.meta.url)), '../..');

function formatElapsed(ms) {
  const s = Math.floor(ms / 1000);
  const m = Math.floor(s / 60);
  return m > 0 ? `${m}m${String(s % 60).padStart(2, '0')}s` : `${s}s`;
}

function banner(cmd, index, total, startTime) {
  const counter = `${index + 1}/${total}`;
  const elapsed = formatElapsed(Date.now() - startTime);
  const pad = 2;
  const minGap = 2;
  const innerWidth = Math.max(80 - pad * 2, cmd.length + counter.length + minGap, elapsed.length);
  const width = innerWidth + pad * 2;
  const line = '═'.repeat(width);
  const inner = cmd.padEnd(innerWidth);
  const cmdLine = cmd + counter.padStart(innerWidth - cmd.length);
  const elapsedLine = elapsed.padStart(innerWidth);
  console.log(`╔${line}╗`);
  console.log(`║${''.padEnd(pad)}${cmdLine}${''.padEnd(pad)}║`);
  console.log(`║${''.padEnd(pad)}${elapsedLine}${''.padEnd(pad)}║`);
  console.log(`╚${line}╝`);
}

const commands = [
  'npm run clean',
  'export BUILD_PROFILE=prod ; npm run codegen:loaders',
  'npm run prettier:check',
  'npm run prettier:ext',
  'npm run lint',
  'npm run test:unit',
  'export BUILD_PROFILE=dev  ; npm run codegen:loaders && npm run build:cjs && npm run build:esm && npm run build:types && npm run build:tests',
  'npm run clean',
  'export BUILD_PROFILE=prod ; npm run codegen:loaders && npm run build:cjs && npm run build:esm && npm run build:types && npm run build:tests',
  'npm run test:browser',
  './test/scripts/rebuild_sdk_and_pack.sh --build-profile=skip',
  './test/scripts/localcleartext-run-tests.sh --use-pack --foundry-profile=v12',
  './test/scripts/localcleartext-run-tests.sh --use-pack --foundry-profile=v13',
  //'node test/multi-wasm/run.mjs',
];

const longCommands = [
  'npm run test:full:testnet',
  'npm run test:full:devnet',
  'npm run test:localstack:v11',
  'npm run test:localstack:v12',
  'npm run test:localstack:v13',
  'npm run test:localstack',
];

const full = process.argv.includes('--full');

if (process.argv.includes('--help')) {
  console.log(`Usage: node test/scripts/dod.mjs [options]

Options:
  --full   Also run long-running localstack tests after the standard commands
  --help   Show this help message

Standard commands (${commands.length}):
${commands.map((c) => `  ${c}`).join('\n')}

Long commands (${longCommands.length}, requires --full):
${longCommands.map((c) => `  ${c}`).join('\n')}
`);
  process.exit(0);
}

const queue = [...commands, ...(full ? longCommands : [])];
const startTime = Date.now();

for (const [i, cmd] of queue.entries()) {
  banner(cmd, i, queue.length, startTime);
  const result = spawnSync(cmd, { cwd: sdkRoot, stdio: 'inherit', shell: true });
  if (result.status !== 0) {
    console.error(`\nFailed command: \x1b[31m${cmd}\x1b[0m`);
    process.exit(result.status ?? 1);
  }
}

banner('Success!', queue.length - 1, queue.length, startTime);
