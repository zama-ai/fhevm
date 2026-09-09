import assert from 'node:assert/strict';
import test from 'node:test';

import {
  generationOf,
  liveGenerationNames,
  validateGenerationCleartextConfig,
  validateGenerationDependencies,
  validateGenerationMirrorPatch,
  validateGenerationVendoredDestinations,
} from '../base/checks/generations.ts';
import type { LoadedPackage } from '../base/npm.ts';
import { loadedPackage, parseTestNpmManifest } from './helpers.ts';

const FAMILY = 'host-contracts-cleartext';
const CURRENT = `./${FAMILY}/v13`;
const PREVIOUS = `./${FAMILY}/v12`;

// `null` means "no generations block at all"; `undefined` would select the default parameter instead.
function manifest(
  generations: Record<string, unknown> | null = { [FAMILY]: { current: CURRENT, previous: PREVIOUS } },
) {
  return parseTestNpmManifest({
    ...(generations === null ? {} : { generations }),
    packageJson: { published: { required: ['name'], excluded: ['private'] } },
    packages: {
      '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      [PREVIOUS]: dev('v12'),
      [`${PREVIOUS}/pkg`]: { kind: 'published', name: '@fhevm/host-contracts-cleartext', member: false },
      [CURRENT]: dev('v13'),
      [`${CURRENT}/pkg`]: { kind: 'published', name: '@fhevm/host-contracts-cleartext', member: true },
      [`./${FAMILY}/v11`]: dev('v11'),
      [`./${FAMILY}/v11/pkg`]: { kind: 'published', name: '@fhevm/host-contracts-cleartext', member: false },
      './hardhat/v3': { kind: 'workspace-root', name: '@fhevm/hh-v3-cluster', private: true, member: false },
      './hardhat/v3/plugin/pkg': {
        kind: 'published',
        name: '@fhevm/hardhat-plugin',
        member: true,
        memberOf: './hardhat/v3',
      },
    },
  });
}

function dev(gen: string) {
  return {
    kind: 'dev',
    name: `@fhevm/host-contracts-cleartext-${gen}-dev`,
    private: true,
    member: true,
    publishedRelPath: `./${FAMILY}/${gen}/pkg`,
  } as const;
}

function devPackage(gen: string, devDependencies: Record<string, string> = {}): LoadedPackage {
  return loadedPackage(`./${FAMILY}/${gen}`, dev(gen), {
    name: `@fhevm/host-contracts-cleartext-${gen}-dev`,
    private: true,
    version: '0.0.0',
    devDependencies,
  });
}

function payload(gen: string, version: string, member: boolean): LoadedPackage {
  return loadedPackage(
    `./${FAMILY}/${gen}/pkg`,
    { kind: 'published', name: '@fhevm/host-contracts-cleartext', member },
    { name: '@fhevm/host-contracts-cleartext', version },
  );
}

function plugin(dependencies: Record<string, string>): LoadedPackage {
  return loadedPackage(
    './hardhat/v3/plugin/pkg',
    { kind: 'published', name: '@fhevm/hardhat-plugin', member: true, memberOf: './hardhat/v3' },
    { name: '@fhevm/hardhat-plugin', version: '0.13.0', dependencies },
  );
}

const root = loadedPackage(
  '.',
  { kind: 'workspace-root', name: 'workspace', private: true, member: false },
  { name: 'workspace', private: true },
);

test('classifies inventory keys by generation', () => {
  const family = { family: FAMILY, current: CURRENT, previous: PREVIOUS };
  assert.equal(generationOf(family, CURRENT), 'current');
  assert.equal(generationOf(family, `${CURRENT}/pkg`), 'current');
  assert.equal(generationOf(family, `${CURRENT}/test-consumer/esm`), 'current');
  assert.equal(generationOf(family, PREVIOUS), 'previous');
  assert.equal(generationOf(family, `./${FAMILY}/v11/pkg`), 'other');
  assert.equal(generationOf(family, `./${FAMILY}/v130`), 'other');
  assert.equal(generationOf(family, './hardhat/v3/plugin/pkg'), undefined);
  assert.equal(generationOf({ ...family, previous: undefined }, PREVIOUS), 'other');
});

test('accepts the intended edges: consumers on V(N), V(N) on V(N-1), a generation on itself', () => {
  const packages = [
    root,
    devPackage('v12'),
    payload('v12', '0.12.0', false),
    devPackage('v13', { '@fhevm/host-contracts-cleartext-v12-dev': '0.0.0' }),
    payload('v13', '0.13.0', true),
    loadedPackage(
      `${CURRENT}/test-consumer/esm`,
      { kind: 'standalone', name: 'consumer-esm', member: false },
      { name: 'consumer-esm', dependencies: { '@fhevm/host-contracts-cleartext': 'file:../../pkg' } },
    ),
    loadedPackage(
      `${PREVIOUS}/test-consumer/esm`,
      { kind: 'standalone', name: 'consumer-esm-v12', member: false },
      { name: 'consumer-esm-v12', dependencies: { '@fhevm/host-contracts-cleartext': 'file:../../pkg' } },
    ),
    plugin({ '@fhevm/host-contracts-cleartext': `file:../../../../${FAMILY}/v13/pkg` }),
  ];
  assert.deepEqual(validateGenerationDependencies(manifest(), packages), []);
});

test('rejects a consumer pinned to V(N-1) by file path', () => {
  const packages = [
    root,
    devPackage('v12'),
    payload('v12', '0.12.0', false),
    devPackage('v13'),
    payload('v13', '0.13.0', true),
    plugin({ '@fhevm/host-contracts-cleartext': `file:../../../../${FAMILY}/v12/pkg` }),
  ];
  const violations = validateGenerationDependencies(manifest(), packages);
  assert.equal(violations.length, 1);
  assert.equal(violations[0]?.rule, '3.4.1');
  assert.equal(violations[0]?.packageKey, './hardhat/v3/plugin/pkg');
  assert.match(violations[0]?.message ?? '', /targets \.\/host-contracts-cleartext\/v12\/pkg, V\(N-1\)/);
  assert.match(violations[0]?.message ?? '', /only V\(N\) '\.\/host-contracts-cleartext\/v13'/);
});

test('rejects a consumer depending on the V(N-1) dev package by name, and V(N-1) is not exempt toward a retired one', () => {
  const packages = [
    root,
    devPackage('v11'),
    devPackage('v12', { '@fhevm/host-contracts-cleartext-v11-dev': '0.0.0' }),
    devPackage('v13'),
    payload('v13', '0.13.0', true),
    loadedPackage(
      './common',
      { kind: 'shared-helper', name: '@fhevm/sdk-common-dev', private: true, member: true },
      {
        name: '@fhevm/sdk-common-dev',
        private: true,
        devDependencies: { '@fhevm/host-contracts-cleartext-v12-dev': '0.0.0' },
      },
    ),
  ];
  const violations = validateGenerationDependencies(manifest(), packages);
  assert.deepEqual(
    violations.map((violation) => [violation.packageKey, violation.rule]),
    [
      [PREVIOUS, '3.4.1'],
      ['./common', '3.4.1'],
    ],
  );
  assert.match(violations[0]?.message ?? '', /neither V\(N\) nor V\(N-1\)/);
});

test('resolves a shared published name by exact version, and reports a range it cannot place', () => {
  const packages = [
    root,
    devPackage('v12'),
    payload('v12', '0.12.0', false),
    devPackage('v13'),
    payload('v13', '0.13.0', true),
    plugin({ '@fhevm/host-contracts-cleartext': '0.13.0' }),
  ];
  assert.deepEqual(validateGenerationDependencies(manifest(), packages), []);

  const onPrevious = [...packages.slice(0, -1), plugin({ '@fhevm/host-contracts-cleartext': '0.12.0' })];
  assert.equal(validateGenerationDependencies(manifest(), onPrevious).length, 1);

  const ranged = [...packages.slice(0, -1), plugin({ '@fhevm/host-contracts-cleartext': '^0.13.0' })];
  const violations = validateGenerationDependencies(manifest(), ranged);
  assert.equal(violations.length, 1);
  assert.match(violations[0]?.message ?? '', /shared by 2 generations/);
  assert.match(violations[0]?.message ?? '', /use a file: path to V\(N\)/);
});

test('is a no-op without a generations block', () => {
  const packages = [
    root,
    devPackage('v13'),
    plugin({ '@fhevm/host-contracts-cleartext': `file:../../../../${FAMILY}/v12/pkg` }),
  ];
  assert.deepEqual(validateGenerationDependencies(manifest(null), packages), []);
});

test('accepts vendored destinations under V(N) and V(N-1), rejects one under a retired generation', () => {
  const live = [
    { to: `${FAMILY}/v13/pkg/ts/types` },
    { to: `${FAMILY}/v12/pkg/ts` },
    { to: 'hardhat/v3/plugin/pkg/src/internal/vendored' },
  ];
  assert.deepEqual(validateGenerationVendoredDestinations(manifest(), live), []);

  const stale = [...live, { to: `${FAMILY}/v11/pkg/ts` }];
  const violations = validateGenerationVendoredDestinations(manifest(), stale);
  assert.equal(violations.length, 1);
  assert.equal(violations[0]?.rule, '3.4.2');
  assert.equal(violations[0]?.packageKey, `./${FAMILY}/v11/pkg/ts`);
  assert.match(violations[0]?.message ?? '', /not under V\(N\) '\.\/host-contracts-cleartext\/v13' or V\(N-1\)/);

  assert.deepEqual(validateGenerationVendoredDestinations(manifest(null), stale), []);
});

test('names the live generations by directory basename', () => {
  assert.deepEqual(liveGenerationNames({ family: FAMILY, current: CURRENT, previous: PREVIOUS }), ['v13', 'v12']);
  assert.deepEqual(liveGenerationNames({ family: FAMILY, current: CURRENT, previous: undefined }), ['v13']);
});

test('resolves what the mirror patch injects as an edge from the template package', () => {
  const template = loadedPackage(
    './hardhat/v2/fhevm-hardhat-template/pkg',
    {
      kind: 'published',
      name: 'fhevm-hardhat-template-v2',
      member: false,
      distribution: ['mirror'],
      mirror: { repository: 'https://github.com/example/template' },
    },
    { name: 'fhevm-hardhat-template-v2', version: '0.4.2' },
  );
  const packages = [
    root,
    devPackage('v12'),
    payload('v12', '0.12.0', false),
    devPackage('v13'),
    payload('v13', '0.13.0', true),
    template,
  ];
  const label = 'mirror patch';

  const onCurrent = { devDependencies: { '@fhevm/host-contracts-cleartext': `file:../../../../${FAMILY}/v13/pkg` } };
  assert.deepEqual(validateGenerationMirrorPatch(manifest(), packages, template.key, onCurrent, label), []);

  const onPrevious = { devDependencies: { '@fhevm/host-contracts-cleartext': `file:../../../../${FAMILY}/v12/pkg` } };
  const violations = validateGenerationMirrorPatch(manifest(), packages, template.key, onPrevious, label);
  assert.equal(violations.length, 1);
  assert.equal(violations[0]?.rule, '3.4.1');
  assert.equal(violations[0]?.packageKey, template.key);
  assert.match(violations[0]?.message ?? '', /^mirror patch: package '@fhevm\/host-contracts-cleartext'/);
  assert.match(violations[0]?.message ?? '', /V\(N-1\)/);

  const missing = validateGenerationMirrorPatch(manifest(), packages, './hardhat/v2/nowhere/pkg', onCurrent, label);
  assert.equal(missing.length, 1);
  assert.match(missing[0]?.message ?? '', /not a manifest package/);
});

test('requires cleartext-config.json#appliesTo.generations to be exactly the live generations', () => {
  assert.deepEqual(validateGenerationCleartextConfig(manifest(), FAMILY, ['v12', 'v13']), []);
  assert.deepEqual(validateGenerationCleartextConfig(manifest(), FAMILY, ['v13', 'v12']), []);

  const stale = validateGenerationCleartextConfig(manifest(), FAMILY, ['v11', 'v12', 'v13']);
  assert.deepEqual(
    stale.map((violation) => [violation.rule, violation.packageKey]),
    [['3.4.3', `./${FAMILY}/v11`]],
  );
  assert.match(stale[0]?.message ?? '', /lists 'v11', which is not a live generation/);

  const omitted = validateGenerationCleartextConfig(manifest(), FAMILY, ['v13']);
  assert.deepEqual(
    omitted.map((violation) => [violation.rule, violation.packageKey]),
    [['3.4.3', `./${FAMILY}/v12`]],
  );
  assert.match(omitted[0]?.message ?? '', /omits live generation 'v12'/);

  assert.deepEqual(validateGenerationCleartextConfig(manifest(), 'other-family', ['v1']), []);
  assert.deepEqual(validateGenerationCleartextConfig(manifest(null), FAMILY, ['v11']), []);
});
