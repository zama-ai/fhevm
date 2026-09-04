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
      { name: '@scope/feature', version: '0.0.0' },
    ),
  ]);
  assert.equal(violations.length, 3);
  assert.ok(violations.every((violation) => violation.rule === '5.1.1'));
});

test('reports missing and nonzero private development package versions', () => {
  const violations = validatePackageNames([
    loadedPackage(
      './missing-version',
      { kind: 'dev', name: '@scope/missing-version-dev', private: true, member: true },
      { name: '@scope/missing-version-dev', private: true },
    ),
    loadedPackage(
      './released-version',
      { kind: 'internal-consumer', name: '@scope/released-version-dev', private: true, member: true },
      { name: '@scope/released-version-dev', private: true, version: '1.2.3' },
    ),
  ]);

  assert.deepEqual(violations, [
    {
      rule: '1.2.1',
      packageKey: './missing-version/package.json',
      message: 'private development package version <missing> must be "0.0.0"',
    },
    {
      rule: '1.2.1',
      packageKey: './released-version/package.json',
      message: 'private development package version "1.2.3" must be "0.0.0"',
    },
  ]);
});
