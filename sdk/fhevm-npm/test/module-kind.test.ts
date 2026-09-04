import assert from 'node:assert/strict';
import test from 'node:test';

import { consumerModuleKinds } from '../base/module-kind.ts';

test('detects a dual package with import and require conditions', () => {
  assert.deepEqual(
    consumerModuleKinds({
      type: 'module',
      exports: { '.': { import: './index.js', require: './index.cjs' } },
    }),
    ['cjs', 'esm'],
  );
});

test('detects the main/module dual-package convention', () => {
  assert.deepEqual(
    consumerModuleKinds({
      type: 'module',
      main: './_cjs/index.js',
      module: './_esm/index.js',
      exports: { '.': { import: './_esm/index.js', default: './_cjs/index.js' } },
    }),
    ['cjs', 'esm'],
  );
});

test('distinguishes ESM-only and CJS-only packages', () => {
  assert.deepEqual(consumerModuleKinds({ type: 'module', exports: './index.js' }), ['esm']);
  assert.deepEqual(consumerModuleKinds({ type: 'commonjs', main: './index.js' }), ['cjs']);
});
