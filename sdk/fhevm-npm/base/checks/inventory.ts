import { existsSync, realpathSync } from 'node:fs';
import { isAbsolute, join, posix, relative, resolve, sep } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { packageDirectory, toolRoot } from '../paths.ts';
import { gitRepositoryRoot, gitVisibleFiles } from '../repository.ts';

export type InventoryInspection = {
  readonly checkedPackageKeys: readonly string[];
  readonly violations: readonly Violation[];
};

export function inspectInventory(workspaceRoot: string, manifest: NpmManifest): InventoryInspection {
  const discoveredPackageKeys = discoverPackageKeys(workspaceRoot, manifest);
  const repositoryRoot = gitRepositoryRoot(workspaceRoot);
  const violations = [
    ...validateInventorySets(manifest, discoveredPackageKeys),
    ...validateInventoryPaths(workspaceRoot, repositoryRoot, manifest),
  ];
  return {
    checkedPackageKeys: [...new Set([...Object.keys(manifest.packages), ...discoveredPackageKeys])].sort(),
    violations,
  };
}

export function discoverPackageKeys(
  workspaceRoot: string,
  manifest: NpmManifest,
  excludedToolRoots: readonly string[] = [toolRoot],
): readonly string[] {
  const excludedRoots = [
    ...excludedToolRoots,
    ...(manifest.inventory?.exclude ?? []).map((key) => packageDirectory(workspaceRoot, key)),
  ];
  const keys = new Set(
    gitVisibleFiles(workspaceRoot)
      .filter((file) => posix.basename(file) === 'package.json')
      .filter((file) => existsSync(resolve(workspaceRoot, file)))
      .filter((file) => !excludedRoots.some((root) => isInside(root, resolve(workspaceRoot, file))))
      .map(packageKeyFromFile),
  );

  for (const [key, entry] of Object.entries(manifest.packages)) {
    if (entry.kind !== 'standalone') continue;
    const packageJson = join(packageDirectory(workspaceRoot, key), 'package.json');
    if (excludedRoots.some((root) => isInside(root, packageJson))) continue;
    if (existsSync(packageJson)) keys.add(key);
  }

  return [...keys].sort();
}

export function validateInventorySets(
  manifest: NpmManifest,
  discoveredPackageKeys: readonly string[],
): readonly Violation[] {
  const declared = new Set(Object.keys(manifest.packages));
  const discovered = new Set(discoveredPackageKeys);
  const violations: Violation[] = [];

  for (const key of [...discovered].sort()) {
    if (!declared.has(key)) {
      violations.push({
        rule: '7.1.3',
        packageKey: key,
        message: `source package.json is missing from npm-manifest.json`,
      });
    }
  }
  for (const key of [...declared].sort()) {
    if (!discovered.has(key)) {
      violations.push({
        rule: '7.1.3',
        packageKey: key,
        message: `manifest entry has no discoverable source package.json`,
      });
    }
  }

  return violations;
}

export function validateInventoryPaths(
  workspaceRoot: string,
  repositoryRoot: string,
  manifest: NpmManifest,
): readonly Violation[] {
  const violations: Violation[] = [];

  for (const excluded of manifest.inventory?.exclude ?? []) {
    checkExistingPath(
      violations,
      '.',
      `'inventory.exclude' path '${excluded}'`,
      packageDirectory(workspaceRoot, excluded),
      workspaceRoot,
    );
  }

  for (const [key, entry] of Object.entries(manifest.packages)) {
    const directory = packageDirectory(workspaceRoot, key);
    checkExistingPath(violations, key, 'package.json', join(directory, 'package.json'), workspaceRoot);

    if (entry.publishedRelPath !== undefined) {
      checkExistingPath(
        violations,
        key,
        `'publishedRelPath'`,
        packageDirectory(workspaceRoot, entry.publishedRelPath),
        workspaceRoot,
      );
    }

    for (const [index, vendored] of (entry.vendored ?? []).entries()) {
      checkExistingPath(
        violations,
        key,
        `'vendored[${index}].relPath'`,
        resolve(directory, vendored.relPath.slice(2)),
        directory,
      );
      if (typeof vendored.source === 'string') {
        checkExistingPath(
          violations,
          key,
          `'vendored[${index}].source'`,
          resolve(repositoryRoot, vendored.source.slice(2)),
          repositoryRoot,
        );
      }
    }
  }

  return violations;
}

function packageKeyFromFile(file: string): string {
  const directory = posix.dirname(file);
  return directory === '.' ? '.' : `./${directory}`;
}

function isInside(root: string, candidate: string): boolean {
  const rel = relative(resolve(root), resolve(candidate));
  return rel === '' || (!rel.startsWith(`..${sep}`) && rel !== '..' && !isAbsolute(rel));
}

function checkExistingPath(
  violations: Violation[],
  packageKey: string,
  field: string,
  candidate: string,
  root: string,
): void {
  if (!existsSync(candidate)) return;
  const realRoot = realpathSync(root);
  const realCandidate = realpathSync(candidate);
  const rel = relative(realRoot, realCandidate);
  if (rel !== '..' && !rel.startsWith(`..${sep}`) && !isAbsolute(rel)) return;
  violations.push({
    rule: '7.1.4',
    packageKey,
    message: `${field} resolves outside '${realRoot}' to '${realCandidate}'`,
  });
}
