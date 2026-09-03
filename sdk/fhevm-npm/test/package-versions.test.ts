import assert from 'node:assert/strict';
import test from 'node:test';

import { formatPackageVersions, packageVersionEntries } from '../base/package-versions.ts';
import { loadedPackage } from './helpers.ts';

const packages = [
  loadedPackage(
    './b/pkg',
    { kind: 'published', name: '@fhevm/b', member: true },
    {
      name: '@fhevm/b',
      version: '1.2.3',
      repository: { type: 'git', url: 'git+https://github.com/zama-ai/b-local.git' },
    },
  ),
  loadedPackage(
    './a/pkg',
    {
      kind: 'published',
      name: 'template',
      member: true,
      distribution: ['mirror'],
      mirror: { repository: 'https://github.com/zama-ai/template' },
    },
    { name: 'template', version: '0.0.0' },
  ),
  loadedPackage(
    './c/pkg',
    { kind: 'published', name: '@fhevm/c', member: false, mirror: { repository: 'https://github.com/zama-ai/c' } },
    { name: '@fhevm/c', version: '0.13.0' },
  ),
  loadedPackage('./a', { kind: 'dev', name: '@fhevm/a-dev', private: true, member: true }, { name: '@fhevm/a-dev' }),
];

test('only published payloads are listed, sorted by key, with their channels', () => {
  const entries = packageVersionEntries(packages);
  assert.deepEqual(
    entries.map((e) => [e.key, e.name, e.version, e.channels.join('+'), e.mirrorRepository]),
    [
      ['./a/pkg', 'template', '0.0.0', 'mirror', 'https://github.com/zama-ai/template'],
      ['./b/pkg', '@fhevm/b', '1.2.3', 'npm', undefined],
      // No `distribution` field but a mirror block: npm-published AND mirrored.
      ['./c/pkg', '@fhevm/c', '0.13.0', 'npm+mirror', 'https://github.com/zama-ai/c'],
    ],
  );
});

test('the table is aligned and carries a header', () => {
  const lines = formatPackageVersions(packageVersionEntries(packages)).split('\n');
  assert.equal(lines.length, 4);
  assert.match(lines[0] ?? '', /^package\s+name\s+version\s+distribution\s+mirror$/);
  assert.match(lines[2] ?? '', /^\.\/b\/pkg\s+@fhevm\/b\s+1\.2\.3\s+npm$/);
});

test('--check-npmjs classifies each npm-distributed entry from the registry answer', async () => {
  const { checkNpmjs, formatCheckedPackageVersions, npmjsPackageUrl } = await import('../base/package-versions.ts');
  const registry: Record<string, { status: number; body: unknown }> = {
    [npmjsPackageUrl('@fhevm/b')]: {
      status: 200,
      body: {
        'dist-tags': { latest: '1.2.3' },
        time: { '1.2.3': '2026-01-31T13:15:05.139Z' },
        versions: {
          '1.2.3': { gitHead: 'abc123', repository: { type: 'git', url: 'git+https://github.com/zama-ai/b.git' } },
        },
      },
    },
    [npmjsPackageUrl('@fhevm/c')]: {
      status: 200,
      body: { 'dist-tags': { latest: '0.12.0' }, versions: { '0.12.0': {} } },
    },
  };
  const seen: string[] = [];
  const fetchRegistry = (url: string): Promise<{ status: number; json: () => Promise<unknown> }> => {
    seen.push(url);
    const answer = registry[url];
    return Promise.resolve(
      answer === undefined
        ? { status: 404, json: () => Promise.resolve({}) }
        : { status: answer.status, json: () => Promise.resolve(answer.body) },
    );
  };

  const checked = await checkNpmjs(packageVersionEntries(packages), fetchRegistry);
  assert.deepEqual(
    checked.map((e) => [e.key, e.npmjs]),
    [
      ['./a/pkg', undefined], // mirror-only: not asked
      [
        './b/pkg',
        {
          kind: 'published',
          latest: '1.2.3',
          gitHead: 'abc123',
          repository: 'https://github.com/zama-ai/b',
          publishedAt: '2026-01-31T13:15:05.139Z',
        },
      ],
      ['./c/pkg', { kind: 'unpublished', latest: '0.12.0' }],
    ],
  );
  // Scoped names are encoded for the registry path; mirror-only payloads never reach it.
  assert.deepEqual(seen, ['https://registry.npmjs.org/%40fhevm%2Fb', 'https://registry.npmjs.org/%40fhevm%2Fc']);

  const unknown = await checkNpmjs([{ key: './x', name: 'nope', version: '1.0.0', channels: ['npm'] }], () =>
    Promise.resolve({ status: 404, json: () => Promise.resolve({}) }),
  );
  assert.deepEqual(unknown[0]?.npmjs, { kind: 'unknown-package' });
  const failing = await checkNpmjs([{ key: './x', name: 'nope', version: '1.0.0', channels: ['npm'] }], () =>
    Promise.reject(new Error('offline')),
  );
  assert.deepEqual(failing[0]?.npmjs, { kind: 'error', detail: 'offline' });

  const lines = formatCheckedPackageVersions(checked).split('\n');
  assert.match(
    lines[0] ?? '',
    /^package\s+name\s+version\s+distribution\s+npmjs\s+published\s+npmjs repository\s+gitHead\s+mirror$/,
  );
  // Registry says zama-ai/b, the local payload says zama-ai/b-local: flagged.
  assert.match(lines[2] ?? '', /2026-01-31\s+https:\/\/github\.com\/zama-ai\/b \(!= local\)\s+abc123$/);
  assert.equal(checked[1]?.repositoryMismatch, true);
  assert.match(lines[3] ?? '', /NOT published \(latest 0\.12\.0\)/);
});
