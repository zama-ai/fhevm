import { resolve } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import {
  type DependencyDeclaration,
  type DependencyMap,
  type LoadedPackage,
  dependencyDeclarations,
  isExactVersion,
} from '../npm.ts';

/**
 * Which generation of a family an inventory key belongs to, if any.
 *
 *   current   the V(N) dev package or anything below it (its payload, its consumers)
 *   previous  the V(N-1) dev package or anything below it
 *   other     under the family directory, but under neither live generation — a retired or unlisted one
 */
export type Generation = 'current' | 'previous' | 'other';

export type GenerationFamily = {
  readonly family: string;
  readonly current: string;
  readonly previous: string | undefined;
};

export function generationFamilies(manifest: NpmManifest): readonly GenerationFamily[] {
  return Object.entries(manifest.generations ?? {}).map(([family, pair]) => ({
    family,
    current: pair.current,
    previous: pair.previous,
  }));
}

/** The generation `key` belongs to within `family`, or undefined when it is not a family package at all. */
export function generationOf(family: GenerationFamily, key: string): Generation | undefined {
  if (isAtOrBelow(key, family.current)) return 'current';
  if (family.previous !== undefined && isAtOrBelow(key, family.previous)) return 'previous';
  if (key.startsWith(`./${family.family}/`)) return 'other';
  return undefined;
}

/** The directory basenames of a family's live generations: `v13`, and `v12` when there is a V(N-1). */
export function liveGenerationNames(family: GenerationFamily): readonly string[] {
  const basename = (key: string): string => key.slice(key.lastIndexOf('/') + 1);
  return family.previous === undefined
    ? [basename(family.current)]
    : [basename(family.current), basename(family.previous)];
}

/**
 * Rule 3.4.1: a dependency on a package of a generation family targets V(N), with one exception — a
 * package of V(N) may depend on V(N-1), because V(N-1) exists only so V(N)'s upgrade path from it can be
 * built and tested. Every other edge into the family (from a consumer to V(N-1), from anywhere to a
 * retired generation) is a violation. A generation may always depend on itself.
 *
 * A `file:` spec is resolved by directory. A version spec is resolved by name; when the name is shared
 * across generations (the published payload has one name in every generation) the spec must be the exact
 * version of one of them, or the check cannot tell which generation is meant and says so.
 */
export function validateGenerationDependencies(
  manifest: NpmManifest,
  packages: readonly LoadedPackage[],
): readonly Violation[] {
  const violations: Violation[] = [];
  for (const family of generationFamilies(manifest)) {
    const familyPackages = packages.filter((pkg) => generationOf(family, pkg.key) !== undefined);
    if (familyPackages.length === 0) continue;
    for (const source of packages) {
      validateSourceEdges(source, dependencyDeclarations(source.packageJson), familyPackages, family, '', violations);
    }
  }
  return violations;
}

/**
 * Rule 3.4.1, for a dependency that is not in any committed package.json: the spec a mirror patch
 * INJECTS into a rendered manifest. The patch's output is resolved as if it were declared by the
 * template package it targets, so a pin left on V(N-1) fails here rather than in the published mirror.
 */
export function validateGenerationMirrorPatch(
  manifest: NpmManifest,
  packages: readonly LoadedPackage[],
  templateKey: string,
  injected: { readonly dependencies?: DependencyMap; readonly devDependencies?: DependencyMap },
  patchLabel: string,
): readonly Violation[] {
  const template = packages.find((pkg) => pkg.key === templateKey);
  if (template === undefined) {
    return [
      {
        rule: '3.4.1',
        packageKey: templateKey,
        message: `${patchLabel} targets '${templateKey}', which is not a manifest package`,
      },
    ];
  }
  const declarations: DependencyDeclaration[] = [
    ...Object.entries(injected.dependencies ?? {}).map(([name, spec]) => ({
      field: 'dependencies' as const,
      name,
      spec,
    })),
    ...Object.entries(injected.devDependencies ?? {}).map(([name, spec]) => ({
      field: 'devDependencies' as const,
      name,
      spec,
    })),
  ];
  const violations: Violation[] = [];
  for (const family of generationFamilies(manifest)) {
    const familyPackages = packages.filter((pkg) => generationOf(family, pkg.key) !== undefined);
    if (familyPackages.length === 0) continue;
    validateSourceEdges(template, declarations, familyPackages, family, `${patchLabel}: `, violations);
  }
  return violations;
}

/**
 * Rule 3.4.2: a `sync-vendored` destination under a generation family's directory belongs to V(N) or
 * V(N-1). A destination under a retired generation is the entry a rotation forgets; the check names it.
 * The vendored manifest keeps plain paths — this only reads them.
 */
export function validateGenerationVendoredDestinations(
  manifest: NpmManifest,
  destinations: readonly { readonly to: string }[],
): readonly Violation[] {
  const violations: Violation[] = [];
  for (const family of generationFamilies(manifest)) {
    for (const destination of destinations) {
      if (generationOf(family, `./${destination.to}`) !== 'other') continue;
      violations.push({
        rule: '3.4.2',
        packageKey: `./${destination.to}`,
        message:
          `common-vendored/manifest.json destination '${destination.to}' is under ${family.family} ` +
          `but not under ${liveDescription(family)}; retarget it to a live generation`,
      });
    }
  }
  return violations;
}

/**
 * Rule 3.4.3: `cleartext-config.json#appliesTo.generations` names exactly the live generations of the
 * family its faces are written into — V(N) and V(N-1), by directory basename. A retired generation left
 * in the list makes the generator write into a directory that no longer exists; a live one missing from
 * it gets no face at all, and `check-cleartext-config` cannot notice a face it was never told to expect.
 */
export function validateGenerationCleartextConfig(
  manifest: NpmManifest,
  family: string,
  generations: readonly string[],
): readonly Violation[] {
  const target = generationFamilies(manifest).find((candidate) => candidate.family === family);
  if (target === undefined) return [];
  const live = liveGenerationNames(target);
  const violations: Violation[] = [];
  for (const gen of generations) {
    if (live.includes(gen)) continue;
    violations.push({
      rule: '3.4.3',
      packageKey: `./${family}/${gen}`,
      message: `cleartext-config.json#appliesTo.generations lists '${gen}', which is not a live generation of ${family} (${live.join(', ')})`,
    });
  }
  for (const gen of live) {
    if (generations.includes(gen)) continue;
    violations.push({
      rule: '3.4.3',
      packageKey: `./${family}/${gen}`,
      message: `cleartext-config.json#appliesTo.generations omits live generation '${gen}' of ${family}; it would receive no generated face`,
    });
  }
  return violations;
}

function validateSourceEdges(
  source: LoadedPackage,
  declarations: readonly DependencyDeclaration[],
  familyPackages: readonly LoadedPackage[],
  family: GenerationFamily,
  prefix: string,
  violations: Violation[],
): void {
  for (const declaration of declarations) {
    const target = resolveTarget(source, declaration, familyPackages, family, prefix, violations);
    if (target === undefined) continue;
    checkEdge(source, declaration, target, family, prefix, violations);
  }
}

function checkEdge(
  source: LoadedPackage,
  declaration: DependencyDeclaration,
  target: LoadedPackage,
  family: GenerationFamily,
  prefix: string,
  violations: Violation[],
): void {
  const sourceGeneration = generationOf(family, source.key);
  const targetGeneration = generationOf(family, target.key);
  if (targetGeneration === undefined || targetGeneration === 'current') return;
  if (targetGeneration === 'previous' && (sourceGeneration === 'current' || sourceGeneration === 'previous')) return;
  if (targetGeneration === 'other' && sourceGeneration === 'other' && sameGenerationDirectory(family, source, target)) {
    return;
  }

  const role =
    targetGeneration === 'previous'
      ? `V(N-1) '${family.previous ?? ''}'`
      : 'a generation that is neither V(N) nor V(N-1)';
  violations.push({
    rule: '3.4.1',
    packageKey: source.key,
    message:
      `${prefix}package '${declaration.name}' in '${declaration.field}' targets ${target.key}, ${role} of ${family.family}; ` +
      `only V(N) '${family.current}' may be depended on from outside V(N)`,
  });
}

function resolveTarget(
  source: LoadedPackage,
  declaration: DependencyDeclaration,
  familyPackages: readonly LoadedPackage[],
  family: GenerationFamily,
  prefix: string,
  violations: Violation[],
): LoadedPackage | undefined {
  if (declaration.spec.startsWith('file:')) {
    // Unresolvable links and tarballs are rule 3.1's business; here only a link INTO the family matters.
    if (declaration.spec.endsWith('.tgz')) return undefined;
    const linked = resolve(source.directory, declaration.spec.slice('file:'.length));
    return familyPackages.find((candidate) => resolve(candidate.directory) === linked);
  }

  const candidates = familyPackages.filter((candidate) => candidate.packageJson.name === declaration.name);
  if (candidates.length === 0) return undefined;
  if (candidates.length === 1) return candidates[0];

  // One published name, several generations: only an exact version singles one out.
  const exact = isExactVersion(declaration.spec)
    ? candidates.filter((candidate) => candidate.packageJson.version === declaration.spec)
    : [];
  if (exact.length === 1) return exact[0];

  // A range over a shared name cannot say which generation is meant; report rather than guess.
  violations.push({
    rule: '3.4.1',
    packageKey: source.key,
    message:
      `${prefix}package '${declaration.name}' in '${declaration.field}' is "${declaration.spec}", a name shared by ` +
      `${candidates.length} generations of ${family.family} (${candidates.map((pkg) => pkg.key).join(', ')}); ` +
      `use a file: path to V(N) '${family.current}'`,
  });
  return undefined;
}

function liveDescription(family: GenerationFamily): string {
  return family.previous === undefined
    ? `V(N) '${family.current}'`
    : `V(N) '${family.current}' or V(N-1) '${family.previous}'`;
}

function isAtOrBelow(key: string, generationKey: string): boolean {
  return key === generationKey || key.startsWith(`${generationKey}/`);
}

/** Two 'other' packages share a generation when their first path segment below the family directory agrees. */
function sameGenerationDirectory(family: GenerationFamily, left: LoadedPackage, right: LoadedPackage): boolean {
  const prefix = `./${family.family}/`;
  const segment = (key: string): string => key.slice(prefix.length).split('/')[0] ?? '';
  return segment(left.key) === segment(right.key);
}
