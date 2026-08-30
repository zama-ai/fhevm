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
