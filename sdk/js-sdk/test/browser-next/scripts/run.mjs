import { existsSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const __dirname = dirname(fileURLToPath(import.meta.url));
const browserNextDir = resolve(__dirname, '..');
const sdkRoot = resolve(browserNextDir, '../..');
const manualPackDir = resolve(browserNextDir, '../manual-pack');
const tarballPath = resolve(manualPackDir, 'fhevm-sdk-1.1.0-alpha.5.tgz');
const rebuildScript = resolve(browserNextDir, '../scripts/rebuild_sdk_and_pack.sh');
const playwrightBin = resolve(browserNextDir, '../../node_modules/.bin/playwright');

const args = process.argv.slice(2);
let rebuild = false;
let buildProfile = 'dev';

for (const arg of args) {
  if (arg === '--rebuild') {
    rebuild = true;
  } else if (arg.startsWith('--build-profile=')) {
    buildProfile = arg.slice('--build-profile='.length);
  } else {
    throw new Error(`Unknown argument: ${arg}`);
  }
}

if (!['dev', 'prod', 'skip'].includes(buildProfile)) {
  throw new Error(`Invalid --build-profile value: ${buildProfile}`);
}

if (rebuild) {
  run('bash', [rebuildScript, `--build-profile=${buildProfile}`], sdkRoot);
}

if (!existsSync(tarballPath)) {
  throw new Error(`Missing packed SDK tarball: ${tarballPath}. Run npm run test -- --rebuild.`);
}

if (rebuild || shouldInstall()) {
  run('npm', ['install'], browserNextDir);
}

run(playwrightBin, ['test', '--config', 'playwright.config.ts'], browserNextDir);

function shouldInstall() {
  return (
    !existsSync(resolve(browserNextDir, 'node_modules/@fhevm/sdk/package.json')) ||
    !existsSync(resolve(browserNextDir, 'node_modules/ethers/package.json')) ||
    !existsSync(resolve(browserNextDir, 'node_modules/next/package.json')) ||
    !existsSync(resolve(browserNextDir, 'node_modules/react/package.json')) ||
    !existsSync(resolve(browserNextDir, 'node_modules/react-dom/package.json'))
  );
}

function run(command, commandArgs, cwd) {
  const result = spawnSync(command, commandArgs, {
    cwd,
    stdio: 'inherit',
  });

  if (result.error !== undefined) {
    throw result.error;
  }
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}
