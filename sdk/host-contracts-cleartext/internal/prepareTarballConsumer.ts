import { execFileSync } from 'node:child_process';
import { mkdirSync, rmSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';
import { createPackageTarball, PACKAGE_ROOT } from './createPackageTarball.ts';

export const TARBALL_CONSUMER_DIR = join(PACKAGE_ROOT, 'test', 'ts');
export const TARBALL_CONSUMER_PACKAGE_DIR = join(
  TARBALL_CONSUMER_DIR,
  'node_modules',
  '@fhevm',
  'host-contracts-cleartext',
);

function extractPackageTarball(tarballPath: string): void {
  rmSync(TARBALL_CONSUMER_PACKAGE_DIR, { recursive: true, force: true });
  mkdirSync(TARBALL_CONSUMER_PACKAGE_DIR, { recursive: true });
  execFileSync('tar', ['-xzf', tarballPath, '--strip-components', '1', '-C', TARBALL_CONSUMER_PACKAGE_DIR], {
    encoding: 'utf8',
    stdio: 'pipe',
  });
}

export function prepareTarballConsumer(): string {
  const tarballPath = createPackageTarball();
  extractPackageTarball(tarballPath);
  console.log(`[tarball-consumer] prepared fixture from ${tarballPath}`);
  return tarballPath;
}

if (process.argv[1] !== undefined && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  prepareTarballConsumer();
}
