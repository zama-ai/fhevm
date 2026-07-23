import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readdirSync, rmSync } from 'node:fs';
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
  // The upgrade-e2e test is optional: it needs the sibling cleartext-v12 package to deploy the
  // "before" stack. When that package isn't checked out, skip fixture prep — the library and every
  // other test build/run without it, and the upgrade e2e self-skips (see internal/runUpgradeE2e.ts).
  if (!existsSync(V12_PACKAGE_ROOT)) {
    console.log(
      `[v12-consumer] sibling host-contracts-cleartext-v12 not found at ${V12_PACKAGE_ROOT} — skipping upgrade-e2e fixture.`,
    );
    return;
  }

  // Preparing the fixture requires the sibling package to be fully set up (its deps installed so
  // forge + tsc can build it). If anything fails — deps not installed, forge/tsc error — treat the
  // fixture as unavailable and skip: the upgrade e2e is optional and self-skips (runUpgradeE2e.ts).
  try {
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
  } catch (error) {
    console.warn(
      `[v12-consumer] could not prepare the host-contracts-cleartext-v12 fixture — skipping upgrade-e2e. ` +
        `If you need the v12→v13 upgrade test, install + build the sibling package first ` +
        `(cd ../host-contracts-cleartext-v12 && npm ci). Reason: ${error instanceof Error ? error.message : String(error)}`,
    );
  }
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  prepareV12Consumer();
}
