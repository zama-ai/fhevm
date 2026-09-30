import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
  inspectPublishedFiles,
  validatePublishedFiles,
  validateRequiredPayloadFiles,
} from '../base/checks/published-files.ts';
import { loadedPackage, parseTestNpmManifest } from './helpers.ts';

const published = (files: unknown) =>
  loadedPackage('./project/pkg', { kind: 'published', name: '@scope/project', member: true }, {
    name: '@scope/project',
    ...(files === undefined ? {} : { files }),
  } as never);

const probes = (visible: readonly string[], packed: readonly string[]) => ({
  visibleFiles: () => visible,
  packedFiles: () => packed,
});

test('a file that npm would not pack and no "!" pattern names is a stray', () => {
  const pkg = published(['src', 'LICENSE', '!**/tsconfig*.json']);
  assert.deepEqual(
    validatePublishedFiles(
      pkg,
      probes(['package.json', 'LICENSE', 'src/index.js', 'NOTES.md'], ['package.json', 'LICENSE', 'src/index.js']),
    ),
    [
      {
        rule: '5.2.6',
        packageKey: './project/pkg',
        message: `'NOTES.md' is neither published (not selected by "files") nor excluded by a "!" pattern in "files"; delete it or declare it`,
      },
    ],
  );
});

test('a "!" pattern declares a deliberately unpublished file, at any depth like npm reads it', () => {
  const pkg = published(['src', '!**/tsconfig*.json', '!*.tsbuildinfo']);
  const visible = [
    'package.json',
    'src/index.js',
    'tsconfig.json',
    'src/tsconfig.build.json',
    'src/deep/x.tsbuildinfo',
  ];
  assert.deepEqual(validatePublishedFiles(pkg, probes(visible, ['package.json', 'src/index.js'])), []);
});

test('npm is the judge of what ships: a packed file needs no declaration', () => {
  const pkg = published(['dist']);
  // README and package.json are always packed by npm, whatever "files" says.
  assert.deepEqual(
    validatePublishedFiles(
      pkg,
      probes(['package.json', 'README.md', 'dist/a.js'], ['package.json', 'README.md', 'dist/a.js']),
    ),
    [],
  );
});

test('a published package without "files" ships every stray file, so the whitelist itself is required', () => {
  const pkg = published(undefined);
  assert.deepEqual(
    validatePublishedFiles(pkg, probes(['package.json', 'anything.md'], ['package.json', 'anything.md'])),
    [
      {
        rule: '5.2.6',
        packageKey: './project/pkg',
        message: 'published package.json must declare "files": without the whitelist, every stray file ships',
      },
    ],
  );
});

test('several strays are reported one per file, sorted', () => {
  const pkg = published(['src']);
  const messages = validatePublishedFiles(
    pkg,
    probes(['package.json', 'b.md', 'a.md', 'src/x.js'], ['package.json', 'src/x.js']),
  ).map((violation) => violation.message.split("'")[1]);
  assert.deepEqual(messages, ['a.md', 'b.md']);
});

test('rule 5.2.7: LICENSE and README.md must exist and be listed in "files"', () => {
  const complete = published(['src', 'LICENSE', 'README.md']);
  const onDisk = ['package.json', 'LICENSE', 'README.md', 'src/index.js'];
  assert.deepEqual(validateRequiredPayloadFiles(complete, probes(onDisk, onDisk)), []);

  // Present on disk (npm would pack them anyway) but absent from the whitelist: one finding each.
  const unlisted = published(['src']);
  assert.deepEqual(
    validateRequiredPayloadFiles(unlisted, probes(onDisk, onDisk)).map((violation) => violation.message),
    [
      `'LICENSE' is not listed in "files"; the whitelist must name what ships`,
      `'README.md' is not listed in "files"; the whitelist must name what ships`,
    ],
  );

  // Listed but not on disk: the listing is a promise the directory does not keep.
  const missing = validateRequiredPayloadFiles(complete, probes(['package.json', 'LICENSE', 'src/index.js'], []));
  assert.deepEqual(missing, [
    { rule: '5.2.7', packageKey: './project/pkg', message: `'README.md' is missing from the payload directory` },
  ]);

  // Case matters: npmjs.com shows the file, the rule names it exactly.
  const lowercase = published(['src', 'license', 'readme.md']);
  assert.equal(validateRequiredPayloadFiles(lowercase, probes(['package.json', 'license', 'readme.md'], [])).length, 4);
});

test('rule 5.2.7 does not repeat 5.2.6 when "files" is absent altogether', () => {
  const pkg = published(undefined);
  assert.deepEqual(validateRequiredPayloadFiles(pkg, probes(['package.json', 'LICENSE', 'README.md'], [])), []);
});

test('only npm-distributed payloads are inspected: a mirror-only payload is its own mirror', () => {
  const workspaceRoot = mkdtempSync(join(tmpdir(), 'fhevm-npm-published-files-'));
  try {
    write(workspaceRoot, 'package.json', '{ "name": "workspace", "private": true }\n');
    write(workspaceRoot, 'plugin/pkg/package.json', '{ "name": "@scope/plugin", "files": ["src"] }\n');
    write(workspaceRoot, 'template/pkg/package.json', '{ "name": "template", "files": ["contracts"] }\n');
    const manifest = parseTestNpmManifest({
      packageJson: { published: { required: [], excluded: [] } },
      packages: {
        '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
        './plugin/pkg': { kind: 'published', name: '@scope/plugin', member: true },
        './template/pkg': {
          kind: 'published',
          name: 'template',
          member: true,
          distribution: ['mirror'],
          mirror: { repository: 'https://github.com/scope/template' },
        },
      },
    });
    // Both directories hold a stray; only the npm-distributed one is a finding.
    const inspection = inspectPublishedFiles(workspaceRoot, manifest, {
      visibleFiles: () => ['package.json', 'hardhat.config.ts'],
      packedFiles: () => ['package.json'],
    });
    assert.deepEqual(inspection.checkedPackageKeys, ['./plugin/pkg']);
    assert.deepEqual([...new Set(inspection.violations.map((violation) => violation.packageKey))], ['./plugin/pkg']);
    // Both rules run on the same payload: the stray (5.2.6) and the two required files it lacks (5.2.7).
    assert.deepEqual(
      inspection.violations.map((violation) => violation.rule),
      ['5.2.6', '5.2.7', '5.2.7', '5.2.7', '5.2.7'],
    );
  } finally {
    rmSync(workspaceRoot, { recursive: true, force: true });
  }
});

function write(root: string, relativePath: string, contents: string): void {
  const file = join(root, relativePath);
  mkdirSync(join(file, '..'), { recursive: true });
  writeFileSync(file, contents);
}
