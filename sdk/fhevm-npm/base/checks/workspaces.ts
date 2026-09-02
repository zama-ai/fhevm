import type { Violation } from '../diagnostics.ts';
import type { LoadedPackage } from '../npm.ts';

const GLOB_CHARACTER = /[*?[\]{}]/;

// N installation roots: '.' plus every non-'.' workspace-root entry (a CLUSTER — its own npm workspace).
// Each root's package.json#workspaces is compared against the manifest members that declare it via
// memberOf ('.' being the default). Published-name uniqueness is scoped PER ROOT: two generations may
// publish the same npm name as long as they never share an installation root.
export function validateWorkspaces(packages: readonly LoadedPackage[]): readonly Violation[] {
  const roots = packages.filter((pkg) => pkg.inventory.kind === 'workspace-root');
  if (!roots.some((root) => root.key === '.')) throw new Error('The loaded inventory has no workspace root package');

  const violations: Violation[] = [];
  for (const root of roots) validateOneRoot(root, packages, roots, violations);
  validatePublishedNamesPerRoot(packages, violations);
  return violations;
}

function validateOneRoot(
  root: LoadedPackage,
  packages: readonly LoadedPackage[],
  roots: readonly LoadedPackage[],
  violations: Violation[],
): void {
  const workspacesFile = `${root.key === '.' ? 'sdk' : root.key}/package.json#workspaces`;
  const actual = root.packageJson.workspaces ?? [];
  const expected = new Map(
    packages
      .filter((pkg) => installationRootOf(pkg) === root.key)
      .map((pkg) => [relativeToRoot(pkg.key, root.key), pkg] as const),
  );
  const actualSet = new Set(actual);

  for (const entry of actual) {
    if (GLOB_CHARACTER.test(entry)) {
      violations.push({
        rule: '2.1.1',
        packageKey: root.key,
        message: `workspace entry '${entry}' must be an explicit relative path, not a glob`,
      });
      continue;
    }
    if (expected.has(entry)) continue;
    const key = joinRoot(root.key, entry);
    const inventoried = packages.find((pkg) => pkg.key === key);
    violations.push({
      rule: inventoried?.inventory.kind === 'standalone' ? '2.1.4' : '2.1.1',
      packageKey: inventoried?.key ?? root.key,
      message:
        inventoried?.inventory.kind === 'standalone'
          ? `standalone package '${inventoried.key}' must not appear in ${workspacesFile}`
          : `workspace entry '${entry}' has no matching manifest member of '${root.key}'`,
    });
  }

  for (const [entry, pkg] of expected) {
    if (!actualSet.has(entry)) {
      violations.push({
        rule: '2.1.1',
        packageKey: pkg.key,
        message: `manifest member is missing from ${workspacesFile} as '${entry}'`,
      });
    }
  }

  // A '.'-rooted member physically inside a cluster would be installed by TWO roots at once.
  if (root.key !== '.') {
    for (const pkg of packages) {
      if (installationRootOf(pkg) !== '.' || !pkg.key.startsWith(`${root.key}/`)) continue;
      violations.push({
        rule: '2.1.6',
        packageKey: pkg.key,
        message: `a member inside installation root '${root.key}' must declare memberOf: "${root.key}"`,
      });
    }
  }
}

function validatePublishedNamesPerRoot(packages: readonly LoadedPackage[], violations: Violation[]): void {
  const publishedMembersByRootAndName = new Map<string, LoadedPackage[]>();
  for (const pkg of packages) {
    const rootKey = installationRootOf(pkg);
    if (pkg.inventory.kind !== 'published' || rootKey === undefined || pkg.packageJson.name === undefined) continue;
    const groupKey = `${rootKey} ${pkg.packageJson.name}`;
    const sameName = publishedMembersByRootAndName.get(groupKey) ?? [];
    sameName.push(pkg);
    publishedMembersByRootAndName.set(groupKey, sameName);
  }
  for (const [groupKey, sameName] of publishedMembersByRootAndName) {
    if (sameName.length < 2) continue;
    const name = groupKey.split(' ')[1] ?? '';
    const packageKeys = sameName
      .map((pkg) => pkg.key)
      .sort()
      .join(', ');
    for (const pkg of sameName) {
      violations.push({
        rule: '2.1.3',
        packageKey: pkg.key,
        message: `published name '${name}' is assigned to multiple members of one installation root: ${packageKeys}`,
      });
    }
  }
}

/** The installation root a member belongs to, or undefined for non-members. */
export function installationRootOf(pkg: LoadedPackage): string | undefined {
  if (!pkg.inventory.member) return undefined;
  return pkg.inventory.memberOf ?? '.';
}

function relativeToRoot(key: string, rootKey: string): string {
  if (rootKey === '.') return key.startsWith('./') ? key.slice(2) : key;
  return key.slice(rootKey.length + 1);
}

function joinRoot(rootKey: string, entry: string): string {
  return rootKey === '.' ? `./${entry}` : `${rootKey}/${entry}`;
}
