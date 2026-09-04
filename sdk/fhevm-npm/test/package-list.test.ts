import assert from 'node:assert/strict';
import test from 'node:test';

import { packageListEntries } from '../base/package-list.ts';
import { parseTestNpmManifest } from './helpers.ts';

test('lists manifest package paths and kinds in deterministic path order', () => {
  const manifest = parseTestNpmManifest({
    packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
    packages: {
      './zebra': { kind: 'standalone', name: 'zebra', member: false },
      '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      './alpha': { kind: 'shared-helper', name: '@scope/alpha-dev', private: true, member: true },
    },
  });

  assert.deepEqual(packageListEntries(manifest), [
    { path: '.', kind: 'workspace-root' },
    { path: './alpha', kind: 'shared-helper' },
    { path: './zebra', kind: 'standalone' },
  ]);
});
