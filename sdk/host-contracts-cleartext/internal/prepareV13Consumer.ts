import { execFileSync, spawnSync } from 'node:child_process';
import { mkdirSync, readdirSync, rmSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { PACKAGE_ROOT } from './createPackageTarball.ts';

// The sibling cleartext-v13 package. The e2e upgrade test deploys a fresh v13 stack via this package,
// then upgrades it to v14 via the local (v14) package's `updateV13ToV14`.
//
// The v13 package is ALSO named `@fhevm/host-contracts-cleartext` (only one version is ever published
// per release branch), so the fixture is installed under the aliased directory
// `@fhevm/host-contracts-cleartext-v13` — node resolves the package.json `exports` from the directory
// it finds the package in, regardless of its `name` field.
const V13_PACKAGE_ROOT = resolve(PACKAGE_ROOT, '..', 'host-contracts-cleartext-v13');
const V13_TARBALL_DIR = join(PACKAGE_ROOT, 'test', 'ts', '.tarballs');
const V13_CONSUMER_PACKAGE_DIR = join(
  PACKAGE_ROOT,
  'test',
  'ts',
  'node_modules',
  '@fhevm',
  'host-contracts-cleartext-v13',
);

function run(command: string, args: readonly string[], cwd: string): void {
  const result = spawnSync(command, args, { cwd, encoding: 'utf8', stdio: 'inherit' });
  if (result.status !== 0) {
    throw new Error(`\`${command} ${args.join(' ')}\` failed in ${cwd} (status ${String(result.status)})`);
  }
}

function packV13(): string {
  // Both packages share the tarball name prefix (same package name); only the version distinguishes
  // them, so match the v13 version precisely to avoid deleting this package's own tarball.
  for (const entry of readdirSync(V13_TARBALL_DIR)) {
    if (entry.startsWith('fhevm-host-contracts-cleartext-0.13') && entry.endsWith('.tgz')) {
      rmSync(join(V13_TARBALL_DIR, entry), { force: true });
    }
  }
  const result = spawnSync('npm', ['pack', '--json', '--pack-destination', V13_TARBALL_DIR], {
    cwd: V13_PACKAGE_ROOT,
    encoding: 'utf8',
    stdio: 'pipe',
  });
  if (result.status !== 0) {
    throw new Error(`npm pack (v13) failed\n${result.stdout}${result.stderr}`);
  }
  const parsed: unknown = JSON.parse(result.stdout);
  const first: unknown = Array.isArray(parsed) ? parsed[0] : undefined;
  if (typeof first !== 'object' || first === null || typeof (first as Record<string, unknown>).filename !== 'string') {
    throw new Error(`Unexpected npm pack output: ${result.stdout}`);
  }
  return join(V13_TARBALL_DIR, (first as { filename: string }).filename);
}

export function prepareV13Consumer(): void {
  // Build the v13 package (contracts → templates → TS) so its tarball ships a ready-to-import `ts/`.
  run('npm', ['run', 'build:templates'], V13_PACKAGE_ROOT);
  run('npm', ['run', 'build'], V13_PACKAGE_ROOT);

  const tarballPath = packV13();
  rmSync(V13_CONSUMER_PACKAGE_DIR, { recursive: true, force: true });
  mkdirSync(V13_CONSUMER_PACKAGE_DIR, { recursive: true });
  execFileSync('tar', ['-xzf', tarballPath, '--strip-components', '1', '-C', V13_CONSUMER_PACKAGE_DIR], {
    encoding: 'utf8',
    stdio: 'pipe',
  });
  console.log(`[v13-consumer] installed v13 fixture from ${tarballPath}`);
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  prepareV13Consumer();
}
