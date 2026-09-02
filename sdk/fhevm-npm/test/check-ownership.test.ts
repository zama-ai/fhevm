import assert from 'node:assert/strict';
import test from 'node:test';

import { validateOwnership } from '../base/checks/ownership.ts';
import { loadedPackage } from './helpers.ts';

test('accepts one dev owner with its published payload in pkg', () => {
  const owner = devOwner('./library', './library/pkg');
  const published = publishedPackage('./library/pkg');
  assert.deepEqual(validateOwnership([owner, published]), []);
});

test('reports invalid targets, missing owners, and multiple owners', () => {
  const misplaced = devOwner('./misplaced', './somewhere/pkg');
  const first = devOwner('./first', './shared/pkg');
  const second = devOwner('./second', './shared/pkg');
  const shared = publishedPackage('./shared/pkg');
  const orphan = publishedPackage('./orphan/pkg');

  const violations = validateOwnership([misplaced, first, second, shared, orphan]);
  assert.equal(violations.filter((violation) => violation.rule === '2.1.2').length, 3);
  assert.equal(violations.filter((violation) => violation.rule === '5.3.2').length, 3);
  assert.ok(violations.some((violation) => violation.message.includes('2 dev owners')));
  assert.ok(violations.some((violation) => violation.packageKey === './orphan/pkg'));
});

function devOwner(key: string, publishedRelPath: string) {
  return loadedPackage(
    key,
    {
      kind: 'dev',
      name: `@scope/${key.slice(2)}-dev`,
      private: true,
      member: true,
      publishedRelPath,
    },
    { name: `@scope/${key.slice(2)}-dev`, private: true },
  );
}

function publishedPackage(key: string) {
  return loadedPackage(
    key,
    { kind: 'published', name: `@scope/${key.slice(2)}`, member: true },
    { name: `@scope/${key.slice(2)}`, version: '1.0.0' },
  );
}
