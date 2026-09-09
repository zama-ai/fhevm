import assert from 'node:assert/strict';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';

import { type Packer, publishPack, stagePayload } from '../base/publish-pack.ts';
import { parseTestNpmManifest } from './helpers.ts';

const manifest = parseTestNpmManifest({
  tarballs: { relPath: './tarballs' },
  packageJson: { published: { required: [], excluded: [] } },
  packages: {
    '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
    './library/pkg': { kind: 'published', name: '@scope/library', member: true },
    './plugin/pkg': { kind: 'published', name: '@scope/plugin', member: true },
  },
});

/** A workspace whose plugin links the library and carries a node_modules that must not ship. */
function workspace(): string {
  const root = mkdtempSync(join(tmpdir(), 'fhevm-npm-pack-test-'));
  const write = (rel: string, text: string) => {
    mkdirSync(dirname(join(root, rel)), { recursive: true });
    writeFileSync(join(root, rel), text);
  };
  write('package.json', '{ "name": "workspace", "private": true }\n');
  write('library/pkg/package.json', '{ "name": "@scope/library", "version": "0.13.4", "files": ["src"] }\n');
  write(
    'plugin/pkg/package.json',
    JSON.stringify(
      {
        name: '@scope/plugin',
        version: '0.13.0',
        files: ['src'],
        dependencies: { '@scope/library': 'file:../../library/pkg' },
      },
      null,
      2,
    ),
  );
  write('plugin/pkg/src/index.js', 'export {};\n');
  write('plugin/pkg/node_modules/left-pad/index.js', 'module.exports = 1;\n');
  write('plugin/pkg/src/node_modules/nested/index.js', 'module.exports = 2;\n');
  write(
    'versions.json',
    JSON.stringify({
      $schema: './fhevm-npm/schemas/versions.schema.json',
      schemaVersion: 1,
      packages: { './library/pkg': '0.13.4', './plugin/pkg': '0.13.0' },
    }),
  );
  return root;
}

/** Stands in for `npm pack`: records what the staged directory held and writes a marker "tarball". */
function recordingPacker(seen: { staged?: { packageJson: string; entries: string[] } }): Packer {
  return (staged, outDir) => {
    seen.staged = {
      packageJson: readFileSync(join(staged, 'package.json'), 'utf8'),
      entries: readdirSync(staged, { recursive: true }).map(String).sort(),
    };
    const tarball = join(outDir, 'scope-plugin-0.13.0.tgz');
    writeFileSync(tarball, 'tgz');
    return tarball;
  };
}

test('publish pack stages a node_modules-free copy with the rendered package.json, packs it, and cleans up', () => {
  const root = workspace();
  try {
    const seen: { staged?: { packageJson: string; entries: string[] } } = {};
    const tarball = publishPack(root, manifest, './plugin/pkg', { pack: recordingPacker(seen) });
    assert.equal(tarball, join(root, 'tarballs', 'scope-plugin-0.13.0.tgz'));
    assert.ok(existsSync(tarball));
    // The staged manifest is the rendered one: the link became the library's generation range.
    const staged = JSON.parse(seen.staged?.packageJson ?? '{}') as { dependencies: Record<string, string> };
    assert.deepEqual(staged.dependencies, { '@scope/library': '^0.13.0' });
    // node_modules is excluded at every depth; sources travel.
    assert.deepEqual(seen.staged?.entries, ['package.json', 'src', 'src/index.js']);
    // The tree is untouched and the scratch copy is gone.
    assert.match(readFileSync(join(root, 'plugin', 'pkg', 'package.json'), 'utf8'), /file:\.\.\/\.\.\/library\/pkg/);
    assert.equal(
      readdirSync(tmpdir()).some((entry) => entry.startsWith('fhevm-npm-publish-pack-')),
      false,
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('--out-dir overrides the manifest tarballs directory; a mirror-only or unknown payload is refused before staging', () => {
  const root = workspace();
  try {
    const elsewhere = join(root, 'elsewhere');
    const tarball = publishPack(root, manifest, 'plugin/pkg', { outDir: elsewhere, pack: recordingPacker({}) });
    assert.equal(dirname(tarball), elsewhere);
    assert.throws(
      () => publishPack(root, manifest, './nowhere', { pack: recordingPacker({}) }),
      /No npm-distributed payload/,
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('stagePayload copies into a fresh temp directory the caller owns', () => {
  const root = workspace();
  try {
    const staged = stagePayload(join(root, 'plugin', 'pkg'));
    try {
      assert.ok(staged.startsWith(tmpdir()));
      assert.equal(existsSync(join(staged, 'node_modules')), false);
      assert.ok(existsSync(join(staged, 'src', 'index.js')));
    } finally {
      rmSync(staged, { recursive: true, force: true });
    }
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
