import assert from 'node:assert/strict';
import { join } from 'node:path';
import test from 'node:test';

import { validateLockfiles } from '../base/checks/lockfiles.ts';
import { loadedPackage } from './helpers.ts';

test('accepts root and standalone lockfiles with no member lockfile', () => {
  const packages = fixtures();
  const existing = new Set([
    join(packages[0]!.directory, 'package-lock.json'),
    join(packages[2]!.directory, 'package-lock.json'),
  ]);
  assert.deepEqual(
    validateLockfiles(packages, (file) => existing.has(file)),
    [],
  );
});

test('reports missing root and standalone lockfiles and a member lockfile', () => {
  const packages = fixtures();
  const existing = new Set([join(packages[1]!.directory, 'package-lock.json')]);
  const violations = validateLockfiles(packages, (file) => existing.has(file));
  assert.equal(violations.length, 3);
  assert.ok(violations.every((violation) => violation.rule === '6.1.1'));
});

function fixtures() {
  return [
    loadedPackage(
      '.',
      { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      { name: 'workspace', private: true },
    ),
    loadedPackage(
      './member',
      { kind: 'shared-helper', name: '@scope/member-dev', private: true, member: true },
      { name: '@scope/member-dev', private: true },
    ),
    loadedPackage('./standalone', { kind: 'standalone', name: 'consumer', member: false }, { name: 'consumer' }),
  ] as const;
}
