import { spawnSync } from 'node:child_process';
import { mkdirSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

export const PACKAGE_ROOT = join(dirname(fileURLToPath(import.meta.url)), '..');
export const TARBALL_DIR = join(PACKAGE_ROOT, 'test', 'ts', '.tarballs');

type NpmPackEntry = {
  filename: string;
};

function removeExistingTarballs(): void {
  mkdirSync(TARBALL_DIR, { recursive: true });

  for (const entry of readdirSync(TARBALL_DIR)) {
    if (entry.endsWith('.tgz')) {
      rmSync(join(TARBALL_DIR, entry), { force: true });
    }
  }
}

function parseNpmPackOutput(stdout: string): NpmPackEntry {
  const parsed: unknown = JSON.parse(stdout);
  if (!Array.isArray(parsed)) {
    throw new Error(`Unexpected npm pack output: ${stdout}`);
  }

  const firstEntry: unknown = parsed[0];
  if (typeof firstEntry !== 'object' || firstEntry === null) {
    throw new Error(`Unexpected npm pack output: ${stdout}`);
  }

  const filename = (firstEntry as Record<string, unknown>).filename;
  if (typeof filename !== 'string') {
    throw new Error(`npm pack output does not contain a filename: ${stdout}`);
  }

  return { filename };
}

export function createPackageTarball(): string {
  removeExistingTarballs();

  const npmCache = join(tmpdir(), 'fhevm-host-contracts-cleartext-npm-cache');
  mkdirSync(npmCache, { recursive: true });

  const result = spawnSync('npm', ['pack', '--json', '--pack-destination', TARBALL_DIR], {
    cwd: PACKAGE_ROOT,
    encoding: 'utf8',
    env: {
      ...process.env,
      npm_config_cache: npmCache,
    },
    stdio: 'pipe',
  });

  if (result.error !== undefined) {
    throw result.error;
  }

  if (result.status !== 0) {
    throw new Error(`npm pack failed\n${result.stdout}${result.stderr}`);
  }

  const entry = parseNpmPackOutput(result.stdout);
  return join(TARBALL_DIR, entry.filename);
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  console.log(createPackageTarball());
}
