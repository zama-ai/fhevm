import type { Violation } from '../diagnostics.ts';
import type { LoadedPackage } from '../npm.ts';

const PRIVATE_PACKAGE_KINDS = new Set(['dev', 'shared-helper', 'internal-consumer']);

export function validatePackageNames(packages: readonly LoadedPackage[]): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    const expectedName = pkg.inventory.name;
    const actualName = pkg.packageJson.name;

    if (expectedName === undefined && actualName !== undefined) {
      violations.push(failure(pkg, `package.json name "${actualName}" is forbidden for kind ${pkg.inventory.kind}`));
    } else if (expectedName !== undefined && actualName !== expectedName) {
      violations.push(
        failure(pkg, `package.json name ${quote(actualName)} does not match manifest name "${expectedName}"`),
      );
    }

    if (PRIVATE_PACKAGE_KINDS.has(pkg.inventory.kind)) {
      if (pkg.packageJson.private !== true) {
        violations.push(failure(pkg, 'private development packages must set private: true'));
      }
      if (actualName === undefined || !actualName.endsWith('-dev')) {
        violations.push(failure(pkg, `private development package name ${quote(actualName)} must end in -dev`));
      }
    }

    if (pkg.inventory.kind === 'published') {
      if (pkg.packageJson.private === true) {
        violations.push(failure(pkg, 'a published package cannot set private: true'));
      }
      if (actualName?.endsWith('-dev')) {
        violations.push(failure(pkg, `published package name "${actualName}" cannot end in -dev`));
      }
    }
  }

  return violations;
}

function failure(pkg: LoadedPackage, message: string): Violation {
  return { rule: '5.1.1', packageKey: pkg.key, message };
}

function quote(value: string | undefined): string {
  return value === undefined ? '<missing>' : `"${value}"`;
}
