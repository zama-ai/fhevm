import { execFileSync, spawnSync } from 'node:child_process';
import { mkdirSync, readdirSync, rmSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { PACKAGE_ROOT } from './createPackageTarball.ts';

// The sibling cleartext-v12 package. The e2e upgrade test deploys a fresh v12 stack via this package,
// then upgrades it to v13 via the local (v13) package's `updateV12ToV13`.
const V12_PACKAGE_ROOT = resolve(PACKAGE_ROOT, '..', 'host-contracts-cleartext-v12');
const V12_TARBALL_DIR = join(PACKAGE_ROOT, 'test', 'ts', '.tarballs');
const V12_CONSUMER_PACKAGE_DIR = join(
  PACKAGE_ROOT,
  'test',
  'ts',
  'node_modules',
  '@fhevm',
  'host-contracts-cleartext-v12',
);

function run(command: string, args: readonly string[], cwd: string): void {
  const result = spawnSync(command, args, { cwd, encoding: 'utf8', stdio: 'inherit' });
  if (result.status !== 0) {
    throw new Error(`\`${command} ${args.join(' ')}\` failed in ${cwd} (status ${String(result.status)})`);
  }
}

function packV12(): string {
  for (const entry of readdirSync(V12_TARBALL_DIR)) {
    if (entry.startsWith('fhevm-host-contracts-cleartext-v12') && entry.endsWith('.tgz')) {
      rmSync(join(V12_TARBALL_DIR, entry), { force: true });
    }
  }
  const result = spawnSync('npm', ['pack', '--json', '--pack-destination', V12_TARBALL_DIR], {
    cwd: V12_PACKAGE_ROOT,
    encoding: 'utf8',
    stdio: 'pipe',
  });
  if (result.status !== 0) {
    throw new Error(`npm pack (v12) failed\n${result.stdout}${result.stderr}`);
  }
  const parsed: unknown = JSON.parse(result.stdout);
  const first: unknown = Array.isArray(parsed) ? parsed[0] : undefined;
  if (typeof first !== 'object' || first === null || typeof (first as Record<string, unknown>).filename !== 'string') {
    throw new Error(`Unexpected npm pack output: ${result.stdout}`);
  }
  return join(V12_TARBALL_DIR, (first as { filename: string }).filename);
}

export function prepareV12Consumer(): void {
  // Build the v12 package (contracts → templates → TS) so its tarball ships a ready-to-import `ts/`.
  run('npm', ['run', 'build:templates'], V12_PACKAGE_ROOT);
  run('npm', ['run', 'build'], V12_PACKAGE_ROOT);

  const tarballPath = packV12();
  rmSync(V12_CONSUMER_PACKAGE_DIR, { recursive: true, force: true });
  mkdirSync(V12_CONSUMER_PACKAGE_DIR, { recursive: true });
  execFileSync('tar', ['-xzf', tarballPath, '--strip-components', '1', '-C', V12_CONSUMER_PACKAGE_DIR], {
    encoding: 'utf8',
    stdio: 'pipe',
  });
  console.log(`[v12-consumer] installed v12 fixture from ${tarballPath}`);
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  prepareV12Consumer();
}
