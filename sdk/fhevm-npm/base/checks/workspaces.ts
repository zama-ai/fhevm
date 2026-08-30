import type { Violation } from '../diagnostics.ts';
import type { LoadedPackage } from '../npm.ts';

const GLOB_CHARACTER = /[*?[\]{}]/;

export function validateWorkspaces(packages: readonly LoadedPackage[]): readonly Violation[] {
  const root = packages.find((pkg) => pkg.key === '.');
  if (root === undefined) throw new Error('The loaded inventory has no workspace root package');

  const actual = root.packageJson.workspaces ?? [];
  const expected = new Map(
    packages.filter((pkg) => pkg.inventory.member).map((pkg) => [withoutLeadingDotSlash(pkg.key), pkg] as const),
  );
  const actualSet = new Set(actual);
  const violations: Violation[] = [];

  for (const entry of actual) {
    if (GLOB_CHARACTER.test(entry)) {
      violations.push({
        rule: '2.1.1',
        packageKey: '.',
        message: `workspace entry '${entry}' must be an explicit relative path, not a glob`,
      });
      continue;
    }
    if (expected.has(entry)) continue;
    const inventoried = packages.find((pkg) => withoutLeadingDotSlash(pkg.key) === entry);
    violations.push({
      rule: inventoried?.inventory.kind === 'standalone' ? '2.1.4' : '2.1.1',
      packageKey: inventoried?.key ?? '.',
      message:
        inventoried?.inventory.kind === 'standalone'
          ? `standalone package '${inventoried.key}' must not appear in sdk/package.json#workspaces`
          : `workspace entry '${entry}' has no matching manifest entry with member: true`,
    });
  }

  for (const [entry, pkg] of expected) {
    if (!actualSet.has(entry)) {
      violations.push({
        rule: '2.1.1',
        packageKey: pkg.key,
        message: `manifest member is missing from sdk/package.json#workspaces as '${entry}'`,
      });
    }
  }

  const publishedMembersByName = new Map<string, LoadedPackage[]>();
  for (const pkg of packages) {
    if (pkg.inventory.kind !== 'published' || !pkg.inventory.member || pkg.packageJson.name === undefined) continue;
    const sameName = publishedMembersByName.get(pkg.packageJson.name) ?? [];
    sameName.push(pkg);
    publishedMembersByName.set(pkg.packageJson.name, sameName);
  }
  for (const [name, sameName] of publishedMembersByName) {
    if (sameName.length < 2) continue;
    const packageKeys = sameName
      .map((pkg) => pkg.key)
      .sort()
      .join(', ');
    for (const pkg of sameName) {
      violations.push({
        rule: '2.1.3',
        packageKey: pkg.key,
        message: `published name '${name}' is assigned to multiple workspace members: ${packageKeys}`,
      });
    }
  }

  return violations;
}

function withoutLeadingDotSlash(key: string): string {
  return key.startsWith('./') ? key.slice(2) : key;
}
