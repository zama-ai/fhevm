import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { type LoadedPackage, dependencyFields, declarationsByName } from '../npm.ts';
import { packageJsonKey } from './package-names.ts';

const MANIFEST_KEY = './npm-manifest.json';

/** The manifests a lockfile mirrors from the tree. Everything under node_modules is resolved output. */
type LockfileEntries = Readonly<Record<string, Readonly<Record<string, unknown>>>>;

export type LockfileReader = (file: string) => LockfileEntries | undefined;

export function validatePinnedDependencies(
  manifest: NpmManifest,
  packages: readonly LoadedPackage[],
  readLockfile: LockfileReader = defaultLockfileReader,
): readonly Violation[] {
  const pinned = Object.entries(manifest.dependencies?.pinned ?? {});
  if (pinned.length === 0) return [];

  const violations: Violation[] = [];
  const declaredSomewhere = new Set<string>();

  for (const pkg of packages) {
    const declared = declarationsByName(pkg.packageJson);
    for (const [name, pin] of pinned) {
      for (const declaration of declared.get(name) ?? []) {
        declaredSomewhere.add(name);
        if (declaration.spec === pin) continue;
        violations.push({
          rule: '3.3.4',
          packageKey: packageJsonKey(pkg),
          message: `'${name}' in '${declaration.field}' is "${declaration.spec}"; npm-manifest.json#dependencies.pinned requires "${pin}"`,
        });
      }
    }
    violations.push(...validateLockfile(pkg, pinned, readLockfile));
  }

  for (const [name, pin] of pinned) {
    if (declaredSomewhere.has(name)) continue;
    violations.push({
      rule: '3.3.4',
      packageKey: MANIFEST_KEY,
      message: `pin '${name}': "${pin}" is unused; no inventoried package declares it, so remove it from npm-manifest.json#dependencies.pinned`,
    });
  }

  return violations;
}

// A lockfile copies each workspace manifest's dependency specs into its own `packages` map. Aligning a
// package.json without regenerating the lock leaves that copy behind, and `npm ci` refuses the mismatch —
// so an unregenerated lock is a violation of the pin, not a separate bookkeeping problem.
function validateLockfile(
  pkg: LoadedPackage,
  pinned: readonly (readonly [string, string])[],
  readLockfile: LockfileReader,
): readonly Violation[] {
  const entries = readLockfile(join(pkg.directory, 'package-lock.json'));
  if (entries === undefined) return [];

  const lockfileKey = pkg.key === '.' ? './package-lock.json' : `${pkg.key}/package-lock.json`;
  const violations: Violation[] = [];

  for (const [entryPath, entry] of Object.entries(entries)) {
    if (entryPath.split('/').includes('node_modules')) continue;
    for (const field of dependencyFields) {
      const specs = entry[field];
      if (typeof specs !== 'object' || specs === null) continue;
      for (const [name, pin] of pinned) {
        const spec = (specs as Record<string, unknown>)[name];
        if (typeof spec !== 'string' || spec === pin) continue;
        violations.push({
          rule: '3.3.4',
          packageKey: lockfileKey,
          message: `stale lockfile: '${name}' in '${field}' of '${entryPath === '' ? '.' : entryPath}' is "${spec}"; npm-manifest.json#dependencies.pinned requires "${pin}". Regenerate with 'npm install --package-lock-only'`,
        });
      }
    }
  }

  return violations;
}

function defaultLockfileReader(file: string): LockfileEntries | undefined {
  if (!existsSync(file)) return undefined;
  const packages: unknown = JSON.parse(readFileSync(file, 'utf8')).packages;
  return typeof packages === 'object' && packages !== null ? (packages as LockfileEntries) : undefined;
}
