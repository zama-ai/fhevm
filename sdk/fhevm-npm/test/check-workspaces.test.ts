import assert from 'node:assert/strict';
import test from 'node:test';

import { validateWorkspaces } from '../base/checks/workspaces.ts';
import { loadedPackage } from './helpers.ts';

test('accepts the exact explicit workspace member set', () => {
  const root = workspaceRoot(['dev', 'published']);
  const packages = [root, devPackage(), publishedPackage('./published', '@scope/library', true)];
  assert.deepEqual(validateWorkspaces(packages), []);
});

test('reports globs, missing members, standalone members, and duplicate published names', () => {
  const root = workspaceRoot(['packages/*', 'standalone', 'published-a', 'published-b']);
  const packages = [
    root,
    devPackage(),
    loadedPackage('./standalone', { kind: 'standalone', name: 'consumer', member: false }, { name: 'consumer' }),
    publishedPackage('./published-a', '@scope/library', true),
    publishedPackage('./published-b', '@scope/library', true),
  ];

  const violations = validateWorkspaces(packages);
  assert.equal(violations.filter((violation) => violation.rule === '2.1.1').length, 2);
  assert.equal(violations.filter((violation) => violation.rule === '2.1.3').length, 2);
  assert.equal(violations.filter((violation) => violation.rule === '2.1.4').length, 1);
});

function workspaceRoot(workspaces: readonly string[]) {
  return loadedPackage(
    '.',
    { kind: 'workspace-root', name: 'workspace', private: true, member: false },
    { name: 'workspace', private: true, workspaces: [...workspaces] },
  );
}

function devPackage() {
  return loadedPackage(
    './dev',
    {
      kind: 'dev',
      name: '@scope/dev-owner-dev',
      private: true,
      member: true,
      publishedRelPath: './published',
    },
    { name: '@scope/dev-owner-dev', private: true },
  );
}

function publishedPackage(key: string, name: string, member: boolean) {
  return loadedPackage(key, { kind: 'published', name, member }, { name, version: '1.0.0' });
}

test('validates each installation root independently, with per-root published-name uniqueness', () => {
  const clusterRoot = loadedPackage(
    './hardhat/v2',
    { kind: 'workspace-root', name: '@scope/hh-v2-cluster', private: true, member: false },
    { name: '@scope/hh-v2-cluster', private: true, workspaces: ['plugin/pkg'] },
  );
  const packages = [
    workspaceRoot(['dev', 'published']),
    devPackage(),
    publishedPackage('./published', '@scope/library', true),
    clusterRoot,
    loadedPackage(
      './hardhat/v2/plugin/pkg',
      { kind: 'published', name: '@scope/library', member: true, memberOf: './hardhat/v2' },
      { name: '@scope/library', version: '2.0.0' },
    ),
  ];
  // Same published name in TWO roots is legal; each root's workspaces array is complete.
  assert.deepEqual(validateWorkspaces(packages), []);
});

test("flags a cluster root's missing member, and a '.'-rooted member living inside a cluster", () => {
  const clusterRoot = loadedPackage(
    './hardhat/v2',
    { kind: 'workspace-root', name: '@scope/hh-v2-cluster', private: true, member: false },
    { name: '@scope/hh-v2-cluster', private: true, workspaces: [] },
  );
  const strayMember = loadedPackage(
    './hardhat/v2/e2e',
    { kind: 'internal-consumer', name: '@scope/e2e-dev', private: true, member: true },
    { name: '@scope/e2e-dev', private: true },
  );
  const clusterMember = loadedPackage(
    './hardhat/v2/plugin/pkg',
    { kind: 'published', name: '@scope/plugin', member: true, memberOf: './hardhat/v2' },
    { name: '@scope/plugin', version: '2.0.0' },
  );

  const violations = validateWorkspaces([
    workspaceRoot(['dev', 'published', 'hardhat/v2/e2e']),
    devPackage(),
    publishedPackage('./published', '@scope/library', true),
    clusterRoot,
    strayMember,
    clusterMember,
  ]);
  assert.ok(
    violations.some((v) => v.rule === '2.1.1' && v.packageKey === './hardhat/v2/plugin/pkg'),
    'the cluster member missing from the cluster workspaces array must be flagged',
  );
  assert.ok(
    violations.some((v) => v.rule === '2.1.6' && v.packageKey === './hardhat/v2/e2e'),
    "a '.'-rooted member inside the cluster must be told to declare memberOf",
  );
});
