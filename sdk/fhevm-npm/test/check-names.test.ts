import assert from 'node:assert/strict';
import test from 'node:test';

import { validatePackageNames } from '../base/checks/package-names.ts';
import { loadedPackage } from './helpers.ts';

test('accepts the -dev convention and matching manifest names', () => {
  const violations = validatePackageNames([
    loadedPackage(
      './feature',
      { kind: 'shared-helper', name: '@scope/feature-dev', private: true, member: true },
      { name: '@scope/feature-dev', private: true, version: '0.0.0' },
    ),
  ]);
  assert.deepEqual(violations, []);
});

test('reports suffix, privacy, and inventory mismatches', () => {
  const violations = validatePackageNames([
    loadedPackage(
      './feature',
      { kind: 'shared-helper', name: '@scope/feature-dev', private: true, member: true },
      { name: '@scope/feature' },
    ),
  ]);
  assert.equal(violations.length, 3);
  assert.ok(violations.every((violation) => violation.rule === '5.1.1'));
});
