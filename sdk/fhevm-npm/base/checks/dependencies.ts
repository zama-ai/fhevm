import { resolve } from 'node:path';

import type { NpmManifest } from '../../manifest.ts';
import type { Violation } from '../diagnostics.ts';
import { collectPackageImports } from '../imports.ts';
import {
  type DependencyField,
  type LoadedPackage,
  declarationsByName,
  dependencyDeclarations,
  dependencyFields,
  isExactVersion,
} from '../npm.ts';
import { listOwnedSourceFiles } from '../paths.ts';
import { type ScriptPackageUses, collectScriptPackageUses, rootDependencyBinaryIndex } from '../script-dependencies.ts';
import { installationRootOf } from './workspaces.ts';

const PRIVATE_PACKAGE_KINDS = new Set(['dev', 'shared-helper', 'internal-consumer']);

export function validateDependencies(
  workspaceRoot: string,
  manifest: NpmManifest,
  packages: readonly LoadedPackage[],
): readonly Violation[] {
  const root = packages.find((pkg) => pkg.key === '.');
  if (root === undefined) throw new Error('The loaded inventory has no workspace root package');
  const binaryPackages = rootDependencyBinaryIndex(workspaceRoot, root);
  const usesByPackage = new Map<string, ReadonlySet<string>>();
  const scriptUsesByPackage = new Map<string, ScriptPackageUses>();

  for (const pkg of packages) {
    if (!PRIVATE_PACKAGE_KINDS.has(pkg.inventory.kind)) continue;
    const files = listOwnedSourceFiles(workspaceRoot, pkg.key, manifest);
    const sourceUses = collectPackageImports(files);
    const scriptUses = collectScriptPackageUses(pkg.packageJson.scripts, binaryPackages);
    usesByPackage.set(pkg.key, new Set([...sourceUses, ...scriptUses.keys()]));
    scriptUsesByPackage.set(pkg.key, scriptUses);
  }

  return [
    ...validateForbiddenDependencies(manifest, packages),
    ...validateDependencyOrder(packages),
    ...validateDevDependencyPlacement(packages),
    ...validateWorkspaceMemberSpecs(packages),
    ...validatePrivateRootPins(packages, usesByPackage),
    ...validatePublishedRootPinFloors(packages),
    ...validateScriptDependencyDeclarations(packages, scriptUsesByPackage),
    ...validateDependencyGroupPlacement(packages),
    ...validateSiblingRanges(packages),
  ];
}

export function validateScriptDependencyDeclarations(
  packages: readonly LoadedPackage[],
  scriptUsesByPackage: ReadonlyMap<string, ScriptPackageUses>,
): readonly Violation[] {
  const root = packages.find((pkg) => pkg.key === '.');
  if (root === undefined) throw new Error('The loaded inventory has no workspace root package');
  const rootDeclarations = declarationsByName(root.packageJson);
  const violations: Violation[] = [];

  for (const pkg of packages) {
    if (!PRIVATE_PACKAGE_KINDS.has(pkg.inventory.kind)) continue;
    const declared = declarationsByName(pkg.packageJson);
    for (const [packageName, binaryNames] of scriptUsesByPackage.get(pkg.key) ?? []) {
      const rootDeclaration = rootDeclarations.get(packageName)?.[0];
      if (rootDeclaration === undefined) continue;
      const binaries = [...binaryNames]
        .sort()
        .map((name) => `'${name}'`)
        .join(', ');
      const declarations = declared.get(packageName) ?? [];
      if (declarations.length === 0) {
        violations.push({
          rule: '3.3.1',
          packageKey: pkg.key,
          message: `npm scripts invoke ${binaries} from root dependency '${packageName}' but do not declare it; add "${packageName}": "${rootDeclaration.spec}" to 'devDependencies'`,
        });
        continue;
      }
      for (const declaration of declarations) {
        if (declaration.field === 'devDependencies') continue;
        violations.push({
          rule: '3.3.1',
          packageKey: pkg.key,
          message: `npm scripts invoke ${binaries} from package '${packageName}'; move it from '${declaration.field}' to 'devDependencies'`,
        });
      }
    }
  }

  return violations;
}

export function validateForbiddenDependencies(
  manifest: NpmManifest,
  packages: readonly LoadedPackage[],
): readonly Violation[] {
  const forbidden = new Set(manifest.dependencies?.forbidden ?? []);
  const violations: Violation[] = [];

  for (const pkg of packages) {
    const exceptions = new Set(pkg.inventory.dependencyExceptions ?? []);
    const declarations = dependencyDeclarations(pkg.packageJson);
    for (const declaration of declarations) {
      if (!forbidden.has(declaration.name) || exceptions.has(declaration.name)) continue;
      violations.push({
        rule: '3.3.3',
        packageKey: pkg.key,
        message: `package '${declaration.name}' in '${declaration.field}' is forbidden by npm-manifest.json#dependencies.forbidden`,
      });
    }
    for (const exception of exceptions) {
      if (declarations.some((declaration) => declaration.name === exception)) continue;
      violations.push({
        rule: '3.3.3',
        packageKey: pkg.key,
        message: `dependency exception '${exception}' is unused; remove it from this package's manifest entry`,
      });
    }
  }
  return violations;
}

export function validateDevDependencyPlacement(packages: readonly LoadedPackage[]): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    if (pkg.inventory.kind !== 'dev') continue;
    for (const field of dependencyFields) {
      if (field === 'devDependencies') continue;
      for (const name of Object.keys(pkg.packageJson[field] ?? {})) {
        violations.push({
          rule: '4.2.4',
          packageKey: pkg.key,
          message: `kind 'dev' must declare '${name}' in 'devDependencies', not '${field}'`,
        });
      }
    }
  }

  return violations;
}

export function validateDependencyOrder(packages: readonly LoadedPackage[]): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    for (const field of dependencyFields) {
      const actual = Object.keys(pkg.packageJson[field] ?? {});
      const expected = [...actual].sort();
      if (actual.some((name, index) => name !== expected[index])) {
        violations.push({
          rule: 'dependencies-order',
          packageKey: pkg.key,
          message: `'${field}' entries must be alphabetically ordered`,
        });
      }
    }
  }

  return violations;
}

// Per installation root. A member referencing a member of its OWN root uses the plain exact version —
// npm resolves it by name inside that root. A member referencing ANOTHER root's member must use a
// `file:` path (name resolution cannot cross installation roots): npm installs it as a link, and the
// publish layer maps it to a registry range. An npm-distributed source may cross-root-link only to an
// npm-published target — linking a private helper would ship an unresolvable specifier.
export function validateWorkspaceMemberSpecs(packages: readonly LoadedPackage[]): readonly Violation[] {
  const violations: Violation[] = [];
  const targets = new Map<string, LoadedPackage[]>();

  for (const target of packages) {
    if (!target.inventory.member || target.packageJson.name === undefined) continue;
    const candidates = targets.get(target.packageJson.name) ?? [];
    candidates.push(target);
    targets.set(target.packageJson.name, candidates);
  }

  for (const source of packages) {
    if (!source.inventory.member) continue;
    const sourceRoot = installationRootOf(source);
    for (const declaration of dependencyDeclarations(source.packageJson)) {
      if (declaration.spec.startsWith('file:')) {
        validateFileSpec(source, sourceRoot, declaration, packages, violations);
        continue;
      }

      const allCandidates = targets.get(declaration.name) ?? [];
      const candidates = allCandidates.filter((candidate) => installationRootOf(candidate) === sourceRoot);
      if (candidates.length === 0) {
        if (allCandidates.length > 0) {
          violations.push({
            rule: '3.1.1',
            packageKey: source.key,
            message:
              `package '${declaration.name}' in '${declaration.field}' names a member of another installation ` +
              `root (${allCandidates.map((pkg) => pkg.key).join(', ')}); a cross-root reference must use a file: path`,
          });
        }
        continue;
      }
      if (candidates.length !== 1) {
        violations.push({
          rule: '3.1.1',
          packageKey: source.key,
          message: `package '${declaration.name}' in '${declaration.field}' is ambiguous across ${candidates.length} workspace members`,
        });
        continue;
      }

      const target = candidates[0];
      if (target === undefined) continue;
      const version = target.packageJson.version;
      if (version === undefined || !isExactVersion(version)) {
        violations.push({
          rule: '3.1.1',
          packageKey: source.key,
          message: `package '${declaration.name}' in '${declaration.field}' targets ${target.key}, which has no exact version`,
        });
      } else if (declaration.spec !== version || !isExactVersion(declaration.spec)) {
        violations.push({
          rule: '3.1.1',
          packageKey: source.key,
          message: `package '${declaration.name}' in '${declaration.field}' is "${declaration.spec}"; member ${target.key} requires plain version "${version}"`,
        });
      }
    }
  }

  return violations;
}

function validateFileSpec(
  source: LoadedPackage,
  sourceRoot: string | undefined,
  declaration: { readonly name: string; readonly field: DependencyField; readonly spec: string },
  packages: readonly LoadedPackage[],
  violations: Violation[],
): void {
  if (declaration.spec.endsWith('.tgz')) {
    violations.push({
      rule: '3.1.2',
      packageKey: source.key,
      message: `package '${declaration.name}' in '${declaration.field}' uses forbidden tarball spec "${declaration.spec}"`,
    });
    return;
  }

  const linkedTarget = packages.find(
    (candidate) => resolve(candidate.directory) === resolve(source.directory, declaration.spec.slice(5)),
  );
  if (linkedTarget === undefined || linkedTarget.packageJson.name !== declaration.name) {
    violations.push({
      rule: '3.1.1',
      packageKey: source.key,
      message: `file link '${declaration.name}' in '${declaration.field}' does not resolve to the manifest package having that name`,
    });
    return;
  }

  const targetRoot = installationRootOf(linkedTarget);
  if (targetRoot !== undefined && targetRoot === sourceRoot && !isMirrorOnly(source)) {
    violations.push({
      rule: '3.1.1',
      packageKey: source.key,
      message:
        `package '${declaration.name}' in '${declaration.field}' links a member of the SAME installation ` +
        `root (${linkedTarget.key}); use the plain exact version instead`,
    });
    return;
  }

  const targetIsPublishable = linkedTarget.inventory.kind === 'published' && isNpmDistributed(linkedTarget);
  if (source.inventory.kind === 'published' && isNpmDistributed(source) && !targetIsPublishable) {
    violations.push({
      rule: '3.1.1',
      packageKey: source.key,
      message:
        `npm-distributed package must not link private '${declaration.name}' in '${declaration.field}'; ` +
        `a published tarball cannot resolve "${declaration.spec}"`,
    });
  }
}

function isNpmDistributed(pkg: LoadedPackage): boolean {
  return (pkg.inventory.distribution ?? ['npm']).includes('npm');
}

function isMirrorOnly(source: LoadedPackage): boolean {
  const distribution = source.inventory.distribution ?? ['npm'];
  return source.inventory.kind === 'published' && distribution.length === 1 && distribution[0] === 'mirror';
}

export function validatePrivateRootPins(
  packages: readonly LoadedPackage[],
  importsByPackage: ReadonlyMap<string, ReadonlySet<string>>,
): readonly Violation[] {
  const root = packages.find((pkg) => pkg.key === '.');
  if (root === undefined) throw new Error('The loaded inventory has no workspace root package');

  const rootPins = new Map(
    dependencyDeclarations(root.packageJson)
      .filter((declaration) => isExactVersion(declaration.spec))
      .map((declaration) => [declaration.name, declaration.spec] as const),
  );
  const violations: Violation[] = [];

  for (const pkg of packages) {
    if (!PRIVATE_PACKAGE_KINDS.has(pkg.inventory.kind)) continue;
    const imported = importsByPackage.get(pkg.key) ?? new Set<string>();
    const declared = declarationsByName(pkg.packageJson);
    const requiredField = requiredPrivateDependencyField(pkg.inventory.kind);

    for (const [name, pin] of rootPins) {
      const declarations = declared.get(name) ?? [];
      if (imported.has(name) && declarations.length === 0) {
        violations.push({
          rule: '4.2.1',
          packageKey: pkg.key,
          message: `imports root-pinned package '${name}' but does not declare it; add "${name}": "${pin}" to '${requiredField}' as required for kind '${pkg.inventory.kind}'`,
        });
      }

      for (const declaration of declarations) {
        if (!imported.has(name)) {
          violations.push({
            rule: '4.2.1',
            packageKey: pkg.key,
            message: `package '${name}' in '${declaration.field}' is root-pinned but no owned source file imports it`,
          });
        } else {
          if (declaration.field !== requiredField) {
            violations.push({
              rule: '4.2.1',
              packageKey: pkg.key,
              message: `package '${name}' must move from '${declaration.field}' to '${requiredField}' for kind '${pkg.inventory.kind}'`,
            });
          }
          if (declaration.spec !== pin || !isExactVersion(declaration.spec)) {
            violations.push({
              rule: '4.2.1',
              packageKey: pkg.key,
              message: `package '${name}' in '${declaration.field}' is "${declaration.spec}"; the exact root pin is "${pin}"`,
            });
          }
        }
      }
    }
  }

  return violations;
}

export function validatePublishedRootPinFloors(packages: readonly LoadedPackage[]): readonly Violation[] {
  const root = packages.find((pkg) => pkg.key === '.');
  if (root === undefined) throw new Error('The loaded inventory has no workspace root package');

  const rootPins = new Map(
    dependencyDeclarations(root.packageJson)
      .filter((declaration) => isExactVersion(declaration.spec))
      .map((declaration) => [declaration.name, declaration.spec] as const),
  );
  const violations: Violation[] = [];

  for (const pkg of packages) {
    if (pkg.inventory.kind !== 'published') continue;
    for (const field of ['dependencies', 'peerDependencies'] as const) {
      for (const [name, spec] of Object.entries(pkg.packageJson[field] ?? {})) {
        const rootPin = rootPins.get(name);
        if (rootPin === undefined) continue;
        const floor = dependencyRangeFloor(spec);
        if (floor === rootPin) continue;

        violations.push({
          rule: '4.3.1',
          packageKey: pkg.key,
          message:
            floor === undefined
              ? `'${name}' in '${field}' has unsupported range "${spec}"; use an exact, caret, or tilde range whose floor equals root pin "${rootPin}"`
              : `'${name}' in '${field}' has range "${spec}" with floor "${floor}"; its floor must equal root pin "${rootPin}"`,
        });
      }
    }
  }

  return violations;
}

function dependencyRangeFloor(spec: string): string | undefined {
  const match = /^[~^]?(\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?)$/.exec(spec);
  return match?.[1];
}

function requiredPrivateDependencyField(kind: string): DependencyField {
  return kind === 'shared-helper' ? 'dependencies' : 'devDependencies';
}

export function validateDependencyGroupPlacement(packages: readonly LoadedPackage[]): readonly Violation[] {
  const root = packages.find((pkg) => pkg.key === '.');
  if (root === undefined) throw new Error('The loaded inventory has no workspace root package');

  const rootDeclarations = declarationsByName(root.packageJson);
  const specsByDependency = new Map<string, Map<string, Map<string, Set<string>>>>();

  for (const pkg of packages) {
    const dependencyGroup = pkg.inventory.dependencyGroup;
    if (!PRIVATE_PACKAGE_KINDS.has(pkg.inventory.kind) || dependencyGroup === undefined) continue;
    for (const declaration of dependencyDeclarations(pkg.packageJson)) {
      const byDependencyGroup = specsByDependency.get(declaration.name) ?? new Map();
      const byPackage = byDependencyGroup.get(dependencyGroup) ?? new Map<string, Set<string>>();
      const specs = byPackage.get(pkg.key) ?? new Set<string>();
      specs.add(declaration.spec);
      byPackage.set(pkg.key, specs);
      byDependencyGroup.set(dependencyGroup, byPackage);
      specsByDependency.set(declaration.name, byDependencyGroup);
    }
  }

  const violations: Violation[] = [];
  for (const [name, byDependencyGroup] of specsByDependency) {
    if (byDependencyGroup.size < 2) continue;
    const specs = new Set(
      [...byDependencyGroup.values()].flatMap((byPackage) => [...byPackage.values()].flatMap((values) => [...values])),
    );
    const rootDependencyDeclarations = rootDeclarations.get(name);
    if (specs.size > 1 && rootDependencyDeclarations !== undefined) {
      const rootSummary = rootDependencyDeclarations
        .map((declaration) => `"${declaration.spec}" in sdk/package.json field '${declaration.field}'`)
        .sort()
        .join(', ');
      const packageSummary = [...byDependencyGroup.values()]
        .flatMap((byPackage) => [...byPackage])
        .map(([packageKey, values]) => `${packageKey}=${[...values].sort().join('|')}`)
        .sort()
        .join(', ');
      violations.push({
        rule: '4.2.2',
        packageKey: '.',
        message: `'${name}' has master declaration ${rootSummary}, but member packages across dependency groups use different ranges (${packageSummary}); remove it from sdk/package.json, or align the member ranges if the difference is unintended`,
      });
    }
  }
  return violations;
}

export function validateSiblingRanges(packages: readonly LoadedPackage[]): readonly Violation[] {
  const byDependencyGroup = new Map<string, LoadedPackage[]>();
  for (const pkg of packages) {
    const dependencyGroup = pkg.inventory.dependencyGroup;
    if (!PRIVATE_PACKAGE_KINDS.has(pkg.inventory.kind) || dependencyGroup === undefined) continue;
    const siblings = byDependencyGroup.get(dependencyGroup) ?? [];
    siblings.push(pkg);
    byDependencyGroup.set(dependencyGroup, siblings);
  }

  const violations: Violation[] = [];
  for (const [dependencyGroup, siblings] of byDependencyGroup) {
    const byDependency = new Map<string, Map<string, Set<string>>>();
    for (const sibling of siblings) {
      for (const declaration of dependencyDeclarations(sibling.packageJson)) {
        const byPackage = byDependency.get(declaration.name) ?? new Map<string, Set<string>>();
        const specs = byPackage.get(sibling.key) ?? new Set<string>();
        specs.add(declaration.spec);
        byPackage.set(sibling.key, specs);
        byDependency.set(declaration.name, byPackage);
      }
    }

    for (const [name, byPackage] of byDependency) {
      if (byPackage.size < 2) continue;
      const allSpecs = new Set([...byPackage.values()].flatMap((specs) => [...specs]));
      if (allSpecs.size < 2) continue;
      const summary = [...byPackage]
        .map(([key, specs]) => `${key}=${[...specs].sort().join('|')}`)
        .sort()
        .join(', ');
      for (const packageKey of byPackage.keys()) {
        violations.push({
          rule: '4.2.3',
          packageKey,
          message: `'${name}' differs between siblings in dependency group ${dependencyGroup}: ${summary}`,
        });
      }
    }
  }
  return violations;
}
