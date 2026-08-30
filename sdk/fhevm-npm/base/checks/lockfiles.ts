import { existsSync } from 'node:fs';
import { join } from 'node:path';

import type { Violation } from '../diagnostics.ts';
import type { LoadedPackage } from '../npm.ts';

export function validateLockfiles(
  packages: readonly LoadedPackage[],
  fileExists: (file: string) => boolean = existsSync,
): readonly Violation[] {
  const violations: Violation[] = [];

  for (const pkg of packages) {
    const lockfile = join(pkg.directory, 'package-lock.json');
    const exists = fileExists(lockfile);
    const required = pkg.inventory.kind === 'workspace-root' || pkg.inventory.kind === 'standalone';

    if (required && !exists) {
      violations.push({
        rule: '6.1.1',
        packageKey: pkg.key,
        message: `kind '${pkg.inventory.kind}' must have its own package-lock.json`,
      });
    } else if (!required && exists) {
      violations.push({
        rule: '6.1.1',
        packageKey: pkg.key,
        message: `kind '${pkg.inventory.kind}' must use the workspace root lockfile, not '${lockfile}'`,
      });
    }
  }

  return violations;
}
