import assert from 'node:assert/strict';
import { resolve } from 'node:path';
import test from 'node:test';

import { tarballsOutDir } from '../base/pack-tarball.ts';
import { parseTestNpmManifest } from './helpers.ts';

test('the tarballs directory comes from the manifest, not a guess; --out-dir overrides it', () => {
  const manifest = parseTestNpmManifest({
    packageJson: { published: { required: [], excluded: [] } },
    tarballs: { relPath: './tarballs' },
    packages: { '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false } },
  });
  assert.equal(tarballsOutDir('/workspace', manifest), resolve('/workspace', 'tarballs'));
  assert.equal(tarballsOutDir('/workspace', manifest, '/tmp/elsewhere'), resolve('/tmp/elsewhere'));
  assert.throws(
    () => tarballsOutDir('/workspace', { ...manifest, tarballs: undefined }),
    /tarballs\.relPath is required/,
  );
});
