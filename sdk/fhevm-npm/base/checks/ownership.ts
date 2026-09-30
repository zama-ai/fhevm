import type { Violation } from '../diagnostics.ts';
import type { LoadedPackage } from '../npm.ts';

export function validateOwnership(packages: readonly LoadedPackage[]): readonly Violation[] {
  const byKey = new Map(packages.map((pkg) => [pkg.key, pkg] as const));
  const ownersByPublishedKey = new Map<string, LoadedPackage[]>();
  const violations: Violation[] = [];

  for (const owner of packages) {
    if (owner.inventory.kind !== 'dev') continue;
    const targetKey = owner.inventory.publishedRelPath;
    if (targetKey === undefined) {
      violations.push({
        rule: '5.3.2',
        packageKey: owner.key,
        message: `dev owner has no 'publishedRelPath'`,
      });
      continue;
    }

    const expectedTargetKey = `${owner.key}/pkg`;
    if (targetKey !== expectedTargetKey) {
      violations.push({
        rule: '2.1.2',
        packageKey: owner.key,
        message: `'publishedRelPath' is '${targetKey}'; a dev owner's payload must be '${expectedTargetKey}'`,
      });
    }

    const target = byKey.get(targetKey);
    if (target === undefined) {
      violations.push({
        rule: '5.3.2',
        packageKey: owner.key,
        message: `'publishedRelPath' target '${targetKey}' is absent from the manifest`,
      });
      continue;
    }
    if (target.inventory.kind !== 'published') {
      violations.push({
        rule: '5.3.2',
        packageKey: owner.key,
        message: `'publishedRelPath' target '${targetKey}' has kind '${target.inventory.kind}', not 'published'`,
      });
      continue;
    }

    const owners = ownersByPublishedKey.get(targetKey) ?? [];
    owners.push(owner);
    ownersByPublishedKey.set(targetKey, owners);
  }

  for (const published of packages.filter((pkg) => pkg.inventory.kind === 'published')) {
    const owners = ownersByPublishedKey.get(published.key) ?? [];
    if (owners.length === 1) continue;
    violations.push({
      rule: '5.3.2',
      packageKey: published.key,
      message:
        owners.length === 0
          ? `published package has no dev owner`
          : `published package has ${owners.length} dev owners: ${owners
              .map((owner) => owner.key)
              .sort()
              .join(', ')}`,
    });
  }

  return violations;
}
