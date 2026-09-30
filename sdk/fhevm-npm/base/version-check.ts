// `version check`: is every derived version equal to the central one? The central file is validated first
// (versions.ts); only a valid graph is compared against the tree. Derived state is the payload's own
// package.json version and, in every installation root whose lockfile records the member, the `version`
// npm wrote for it. Nothing here reads a version back into the authority.

import { existsSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

import type { NpmManifest } from '../manifest.ts';
import type { Violation } from './diagnostics.ts';
import { type LoadedPackage, loadPackages } from './npm.ts';
import { installationRootOf } from './checks/workspaces.ts';
import { type VersionsFile, loadVersions, validateVersionGraph } from './versions.ts';

export type VersionInspection = {
  readonly checkedPackageKeys: readonly string[];
  readonly violations: readonly Violation[];
};

/** One installation root's lockfile, reduced to the entries that carry a version. */
export type InstallationLock = {
  readonly rootKey: string;
  readonly rootDirectory: string;
  readonly entries: Readonly<Record<string, { readonly version?: string }>>;
};

export function inspectVersions(workspaceRoot: string, manifest: NpmManifest): VersionInspection {
  const versions = loadVersions(workspaceRoot);
  const graph = validateVersionGraph(manifest, versions);
  const checkedPackageKeys = Object.keys(versions.packages);
  // An invalid graph makes every derived comparison meaningless; report it alone.
  if (graph.length > 0) return { checkedPackageKeys, violations: graph };
  const packages = loadPackages(workspaceRoot, manifest);
  return {
    checkedPackageKeys,
    violations: [
      ...validatePackageVersions(packages, versions),
      ...validateLockfileMemberVersions(packages, versions, readInstallationLocks(workspaceRoot, packages)),
    ],
  };
}

/** Each payload's package.json carries exactly its central version. */
export function validatePackageVersions(
  packages: readonly LoadedPackage[],
  versions: VersionsFile,
): readonly Violation[] {
  return centralPayloads(packages, versions).flatMap(({ pkg, central }) => {
    const derived = pkg.packageJson.version;
    if (derived === central) return [];
    return [
      {
        rule: 'version-package',
        packageKey: pkg.key,
        message: `package.json has ${derived ?? 'no version'}; central version is ${central} — run \`version apply\``,
      },
    ];
  });
}

/** Every lockfile entry that resolves to a payload directory records that payload's central version. */
export function validateLockfileMemberVersions(
  packages: readonly LoadedPackage[],
  versions: VersionsFile,
  locks: readonly InstallationLock[],
): readonly Violation[] {
  return centralPayloads(packages, versions).flatMap(({ pkg, central }) =>
    locks.flatMap((lock) =>
      memberEntries(lock, pkg.directory)
        .filter(([, entry]) => entry.version !== central)
        .map(([path, entry]) => ({
          rule: 'version-lockfile',
          packageKey: pkg.key,
          message:
            `${lock.rootKey}/package-lock.json records ${entry.version ?? 'no version'} for '${path}'; ` +
            `central version is ${central} — run \`version apply\``,
        })),
    ),
  );
}

/** The lockfile of every installation root the members belong to, missing files skipped. */
export function readInstallationLocks(workspaceRoot: string, packages: readonly LoadedPackage[]): InstallationLock[] {
  const rootKeys = [...new Set(packages.map(installationRootOf))].filter((key): key is string => key !== undefined);
  return rootKeys.flatMap((rootKey) => {
    const rootDirectory = resolve(workspaceRoot, rootKey);
    const file = join(rootDirectory, 'package-lock.json');
    if (!existsSync(file)) return [];
    const parsed = JSON.parse(readFileSync(file, 'utf8')) as { packages?: InstallationLock['entries'] };
    return [{ rootKey, rootDirectory, entries: parsed.packages ?? {} }];
  });
}

/** The payloads the central file knows, paired with their version; the graph check already reported the rest. */
export function centralPayloads(
  packages: readonly LoadedPackage[],
  versions: VersionsFile,
): readonly { readonly pkg: LoadedPackage; readonly central: string }[] {
  return packages.flatMap((pkg) => {
    const central = versions.packages[pkg.key];
    return central === undefined ? [] : [{ pkg, central }];
  });
}

/**
 * Lock entries are keyed by path relative to the root ('plugin/pkg', '../../host-contracts-cleartext/v13/pkg');
 * link entries under node_modules carry no version and are skipped by the version filter.
 */
export function memberEntries(
  lock: InstallationLock,
  payloadDirectory: string,
): [string, { readonly version?: string }][] {
  return Object.entries(lock.entries).filter(
    ([path, entry]) =>
      path !== '' && entry.version !== undefined && resolve(lock.rootDirectory, path) === resolve(payloadDirectory),
  );
}
