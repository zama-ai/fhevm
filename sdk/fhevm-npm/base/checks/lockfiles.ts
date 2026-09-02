import { existsSync } from 'node:fs';
import { join } from 'node:path';

import type { Violation } from '../diagnostics.ts';
import type { LoadedPackage } from '../npm.ts';

export function validateLockfiles(
  packages: readonly LoadedPackage[],
  fileExists: (file: string) => boolean = existsSync,
): readonly Violation[] {
  const violations: Violation[] = [];
  const configuredConsumerKeys = new Set(packages.flatMap((pkg) => Object.values(pkg.inventory.consumerTests ?? {})));

  for (const pkg of packages) {
    const lockfile = join(pkg.directory, 'package-lock.json');
    const exists = fileExists(lockfile);
    // A workspace-member consumer is covered by its installation root's lock; only an ISOLATED
    // consumer fixture carries its own for `test-consumer --ci`.
    const configuredConsumer = configuredConsumerKeys.has(pkg.key) && !pkg.inventory.member;
    const required =
      pkg.inventory.kind === 'workspace-root' || pkg.inventory.kind === 'standalone' || configuredConsumer;

    if (required && !exists) {
      violations.push({
        rule: '6.1.1',
        packageKey: pkg.key,
        message: configuredConsumer
          ? 'manifest-selected consumer must have its own package-lock.json for isolated npm ci'
          : `kind '${pkg.inventory.kind}' must have its own package-lock.json`,
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
