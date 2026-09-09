import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { type LoadedPackage, declarationsByName } from '../npm.ts';
import { packageJsonKey } from './package-names.ts';

const MANIFEST_KEY = './npm-manifest.json';

export function validatePinnedDependencies(
  manifest: NpmManifest,
  packages: readonly LoadedPackage[],
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
