import assert from 'node:assert/strict';
import test from 'node:test';

import { collectModuleSpecifiers, packageNameFromSpecifier } from '../base/imports.ts';

test('collects module specifiers from supported JavaScript and TypeScript forms', () => {
  const imports = collectModuleSpecifiers(`
    import type { Contract } from 'ethers';
    export { value } from '@scope/library/subpath';
    import helper = require('helper');
    const dynamic = import('dynamic-package');
    const resolved = require.resolve('resolved-package');
    type Remote = import('type-package').Remote;
  `);

  assert.deepEqual([...imports].sort(), [
    '@scope/library/subpath',
    'dynamic-package',
    'ethers',
    'helper',
    'resolved-package',
    'type-package',
  ]);
});

test('normalizes package subpaths and ignores relative and protocol imports', () => {
  assert.equal(packageNameFromSpecifier('@scope/library/subpath'), '@scope/library');
  assert.equal(packageNameFromSpecifier('library/subpath'), 'library');
  assert.equal(packageNameFromSpecifier('./local.js'), undefined);
  assert.equal(packageNameFromSpecifier('node:fs'), undefined);
});
