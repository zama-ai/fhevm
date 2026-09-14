import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
  type InstallationLock,
  readInstallationLocks,
  validateLockfileMemberVersions,
  validatePackageVersions,
} from '../base/version-check.ts';
import type { VersionsFile } from '../base/versions.ts';
import { loadedPackage } from './helpers.ts';

const versions = (packages: Record<string, string>): VersionsFile => ({
  $schema: './fhevm-npm/schemas/versions.schema.json',
  schemaVersion: 1,
  packages,
});

const library = (version: string) =>
  loadedPackage(
    './library/pkg',
    { kind: 'published', name: '@scope/library', member: true },
    {
      name: '@scope/library',
      version,
    },
  );
const plugin = loadedPackage(
  './cluster/plugin/pkg',
  { kind: 'published', name: '@scope/plugin', member: true, memberOf: './cluster' },
  { name: '@scope/plugin', version: '0.4.2' },
);

test('a package.json version must equal the central one; payloads the file lacks are not compared here', () => {
  const central = versions({ './library/pkg': '1.2.3' });
  assert.deepEqual(validatePackageVersions([library('1.2.3'), plugin], central), []);
  assert.deepEqual(validatePackageVersions([library('1.2.4'), plugin], central), [
    {
      rule: 'version-package',
      packageKey: './library/pkg',
      message: 'package.json has 1.2.4; central version is 1.2.3 — run `version apply`',
    },
  ]);
});

test('every lockfile entry resolving to a payload directory must record the central version', () => {
  // The sdk root lists the library as a member; the cluster root links it cross-root by relative path.
  const locks: InstallationLock[] = [
    { rootKey: '.', rootDirectory: '/workspace', entries: { '': {}, 'library/pkg': { version: '1.2.3' } } },
    {
      rootKey: './cluster',
      rootDirectory: '/workspace/cluster',
      entries: {
        'plugin/pkg': { version: '0.4.2' },
        '../library/pkg': { version: '1.2.2' },
        'node_modules/@scope/library': {}, // a link entry: no version, never compared
      },
    },
  ];
  const central = versions({ './library/pkg': '1.2.3', './cluster/plugin/pkg': '0.4.2' });
  assert.deepEqual(validateLockfileMemberVersions([library('1.2.3'), plugin], central, locks), [
    {
      rule: 'version-lockfile',
      packageKey: './library/pkg',
      message:
        "./cluster/package-lock.json records 1.2.2 for '../library/pkg'; central version is 1.2.3 — run `version apply`",
    },
  ]);
});

test('readInstallationLocks reads one lockfile per installation root and skips roots without one', () => {
  const root = mkdtempSync(join(tmpdir(), 'fhevm-npm-version-check-'));
  try {
    mkdirSync(join(root, 'cluster'), { recursive: true });
    writeFileSync(
      join(root, 'package-lock.json'),
      JSON.stringify({ packages: { '': {}, 'library/pkg': { version: '1.2.3' } } }),
    );
    const locks = readInstallationLocks(root, [library('1.2.3'), plugin]);
    assert.deepEqual(
      locks.map((lock) => [lock.rootKey, Object.keys(lock.entries)]),
      [['.', ['', 'library/pkg']]],
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
