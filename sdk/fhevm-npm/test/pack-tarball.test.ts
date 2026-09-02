import assert from 'node:assert/strict';
import test from 'node:test';

import { selectPackTargets, tarballsOutDir } from '../base/pack-tarball.ts';
import { parseTestNpmManifest } from './helpers.ts';

import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

function writeJson(file: string, value: unknown): void {
  mkdirSync(join(file, '..'), { recursive: true });
  writeFileSync(file, `${JSON.stringify(value)}\n`);
}

test('selects npm-distributed payloads by dev owner, refusing mirror-only ones', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-pack-'));
  try {
    writeJson(join(workspace, 'package.json'), { name: 'workspace', private: true });
    writeJson(join(workspace, 'lib', 'package.json'), { name: '@scope/lib-dev', private: true });
    writeJson(join(workspace, 'lib', 'pkg', 'package.json'), { name: '@scope/lib', version: '1.0.0' });
    writeJson(join(workspace, 'mirror', 'package.json'), { name: '@scope/mirror-dev', private: true });
    writeJson(join(workspace, 'mirror', 'pkg', 'package.json'), { name: 'mirror', version: '1.0.0' });

    const manifest = parseTestNpmManifest({
      packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
      tarballs: { relPath: './tarballs' },
      packages: {
        '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
        './lib': { kind: 'dev', name: '@scope/lib-dev', private: true, member: true, publishedRelPath: './lib/pkg' },
        './lib/pkg': { kind: 'published', name: '@scope/lib', member: true },
        './mirror': {
          kind: 'dev',
          name: '@scope/mirror-dev',
          private: true,
          member: true,
          publishedRelPath: './mirror/pkg',
        },
        './mirror/pkg': {
          kind: 'published',
          name: 'mirror',
          member: true,
          distribution: ['mirror'],
          mirror: { repository: 'https://github.com/example/mirror' },
        },
      },
    });

    // The mirror-only payload is not packable: no selector reaches it, and the universe omits it.
    assert.deepEqual(
      selectPackTargets(workspace, manifest).map((target) => target.payloadKey),
      ['./lib/pkg'],
    );
    assert.equal(selectPackTargets(workspace, manifest, './lib')[0]?.payloadKey, './lib/pkg');
    assert.equal(selectPackTargets(workspace, manifest, './lib/pkg')[0]?.ownerKey, './lib');
    assert.throws(() => selectPackTargets(workspace, manifest, './mirror'), /No npm-distributed payload/);

    // The out dir comes from the manifest, not a guess; --out-dir overrides it.
    assert.equal(tarballsOutDir(workspace, manifest), join(workspace, 'tarballs'));
    assert.equal(tarballsOutDir(workspace, manifest, '/tmp/elsewhere'), resolve('/tmp/elsewhere'));

    const withoutRelPath = { ...manifest, tarballs: undefined };
    assert.throws(() => tarballsOutDir(workspace, withoutRelPath), /tarballs\.relPath is required/);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});
