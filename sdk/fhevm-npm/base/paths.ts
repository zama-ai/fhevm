import { execFileSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { dirname, isAbsolute, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

import type { NpmManifest } from '../manifest.ts';

const SOURCE_EXTENSION = /\.(?:c|m)?(?:j|t)sx?$/;

export const toolRoot = dirname(dirname(fileURLToPath(import.meta.url)));
export const defaultWorkspaceRoot = dirname(toolRoot);

export function resolveFrom(baseDir: string, value: string): string {
  return isAbsolute(value) ? resolve(value) : resolve(baseDir, value);
}

export function packageDirectory(workspaceRoot: string, packageKey: string): string {
  const directory = packageKey === '.' ? workspaceRoot : resolve(workspaceRoot, packageKey.slice(2));
  assertInside(workspaceRoot, directory, `manifest package key ${packageKey}`);
  return directory;
}

export function workspaceRelativePath(workspaceRoot: string, absolutePath: string): string {
  return relative(workspaceRoot, absolutePath).split(sep).join('/');
}

export function listOwnedSourceFiles(
  workspaceRoot: string,
  ownerKey: string,
  manifest: NpmManifest,
): readonly string[] {
  const ownerDirectory = packageDirectory(workspaceRoot, ownerKey);
  const ownerRelative = workspaceRelativePath(workspaceRoot, ownerDirectory) || '.';
  const output = execFileSync(
    'git',
    ['-C', workspaceRoot, 'ls-files', '--cached', '--others', '--exclude-standard', '--', ownerRelative],
    { encoding: 'utf8' },
  );

  const excludedRoots = Object.entries(manifest.packages)
    .filter(
      ([candidateKey, entry]) =>
        candidateKey !== ownerKey && entry.kind !== 'non-package' && isDescendantKey(ownerKey, candidateKey),
    )
    .map(([candidateKey]) => (candidateKey === '.' ? '' : candidateKey.slice(2)));

  return output
    .split('\n')
    .filter(Boolean)
    .filter((file) => SOURCE_EXTENSION.test(file))
    .filter((file) => !excludedRoots.some((root) => file === root || file.startsWith(`${root}/`)))
    .map((file) => resolve(workspaceRoot, file))
    .filter(existsSync)
    .sort();
}

function isDescendantKey(parentKey: string, candidateKey: string): boolean {
  if (parentKey === '.') return candidateKey !== '.';
  return candidateKey.startsWith(`${parentKey}/`);
}

function assertInside(root: string, candidate: string, label: string): void {
  const rel = relative(resolve(root), resolve(candidate));
  if (rel === '..' || rel.startsWith(`..${sep}`) || isAbsolute(rel)) {
    throw new Error(`${label} resolves outside the workspace root`);
  }
}
