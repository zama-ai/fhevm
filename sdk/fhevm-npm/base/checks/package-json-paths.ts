import { existsSync } from 'node:fs';
import { dirname, resolve } from 'node:path';

import type { Violation } from '../diagnostics.ts';
import type { LoadedPackage, PackageJson } from '../npm.ts';

const DIRECT_PATH_FIELDS = ['main', 'module', 'types', 'typings'] as const;

type PathClaim = {
  readonly field: string;
  readonly target: string;
};

export type PackageJsonPathInspection = {
  readonly checkedPackageKeys: readonly string[];
  readonly successfulClaims: readonly string[];
  readonly violations: readonly Violation[];
};

export function inspectPackageJsonPaths(packages: readonly LoadedPackage[]): PackageJsonPathInspection {
  const successfulClaims: string[] = [];
  const violations: Violation[] = [];

  for (const pkg of packages) {
    for (const claim of collectPackageJsonPathClaims(pkg.packageJson)) {
      const missing = missingTarget(pkg.directory, claim.target);
      if (missing === undefined) {
        successfulClaims.push(`${pkg.key} [${claim.field}] ${claim.target}`);
      } else {
        violations.push({
          rule: '2.1.6',
          packageKey: pkg.key,
          message: `'${claim.field}' target '${claim.target}' ${missing}`,
        });
      }
    }
  }

  return {
    checkedPackageKeys: packages.map((pkg) => pkg.key),
    successfulClaims,
    violations,
  };
}

export function collectPackageJsonPathClaims(packageJson: PackageJson): readonly PathClaim[] {
  const claims: PathClaim[] = [];
  for (const field of DIRECT_PATH_FIELDS) {
    const target = packageJson[field];
    if (target !== undefined) claims.push({ field, target });
  }
  collectTargetClaims(packageJson.exports, 'exports', false, claims);
  collectTargetClaims(packageJson.imports, 'imports', true, claims);
  return claims;
}

function collectTargetClaims(value: unknown, field: string, allowExternal: boolean, claims: PathClaim[]): void {
  if (typeof value === 'string') {
    if (!allowExternal || value.startsWith('.')) claims.push({ field, target: value });
    return;
  }
  if (Array.isArray(value)) {
    value.forEach((entry, index) => collectTargetClaims(entry, `${field}[${index}]`, allowExternal, claims));
    return;
  }
  if (!isRecord(value)) return;
  for (const [key, target] of Object.entries(value)) {
    collectTargetClaims(target, `${field}[${JSON.stringify(key)}]`, allowExternal, claims);
  }
}

function missingTarget(packageDirectory: string, target: string): string | undefined {
  const wildcard = target.search(/[?*]/);
  if (wildcard === -1) {
    return existsSync(resolve(packageDirectory, target)) ? undefined : 'does not exist';
  }

  const prefix = target.slice(0, wildcard);
  const directory = prefix.endsWith('/') ? prefix : dirname(prefix);
  return existsSync(resolve(packageDirectory, directory))
    ? undefined
    : `has a missing wildcard directory '${directory}'`;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
