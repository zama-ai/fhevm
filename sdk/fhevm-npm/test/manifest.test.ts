import assert from 'node:assert/strict';
import test from 'node:test';

import { ManifestValidationError, parseNpmManifest } from '../manifest.ts';

test('parses the manifest-local package invariants', () => {
  const manifest = parseNpmManifest({
    $schema: './fhevm-npm/schemas/npm-manifest.schema.json',
    foundry: { version: '1.5.1-stable' },
    packageJson: {
      published: { required: ['name', 'version', 'description', 'license'], excluded: ['private'] },
    },
    packages: {
      '.': { kind: 'workspace-root', type: 'esm', browser: false, name: 'workspace', private: true, member: false },
      './feature': {
        kind: 'dev',
        type: 'esm',
        browser: false,
        name: '@scope/feature-dev',
        private: true,
        member: true,
        publishedRelPath: './feature/pkg',
      },
      './feature/pkg': {
        kind: 'published',
        type: 'dual',
        browser: false,
        name: '@scope/feature',
        member: true,
        consumerTests: {
          cjs: './consumer/cjs',
          esm: './consumer/esm',
        },
        mirror: { repository: 'https://github.com/example/feature' },
      },
      './consumer/cjs': {
        kind: 'standalone',
        type: 'cjs',
        browser: false,
        name: 'consumer-cjs',
        member: false,
      },
      './consumer/esm': {
        kind: 'standalone',
        type: 'esm',
        browser: false,
        name: 'consumer-esm',
        member: false,
      },
    },
  });

  assert.equal(manifest.packages['./feature/pkg']?.mirror?.repository, 'https://github.com/example/feature');
  assert.deepEqual(manifest.packages['./feature/pkg']?.consumerTests, {
    cjs: './consumer/cjs',
    esm: './consumer/esm',
  });
  assert.equal(manifest.foundry?.version, '1.5.1-stable');
  assert.deepEqual(manifest.packageJson.published, {
    required: ['name', 'version', 'description', 'license'],
    excluded: ['private'],
  });
});

test('rejects a consumerTests path that is absent or has the wrong module format', () => {
  assert.throws(
    () =>
      parseNpmManifest({
        packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
        packages: {
          '.': { kind: 'workspace-root', type: 'esm', browser: false, name: 'workspace', private: true, member: false },
          './feature/pkg': {
            kind: 'published',
            type: 'dual',
            browser: false,
            name: '@scope/feature',
            member: true,
            consumerTests: { cjs: './esm-only', esm: './missing' },
          },
          './esm-only': {
            kind: 'standalone',
            type: 'esm',
            browser: false,
            name: 'esm-only',
            member: false,
          },
        },
      }),
    ManifestValidationError,
  );
});

test("rejects a published manifest entry containing 'private', including false", () => {
  assert.throws(
    () =>
      parseNpmManifest({
        packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
        packages: {
          '.': { kind: 'workspace-root', type: 'esm', browser: false, name: 'workspace', private: true, member: false },
          './feature/pkg': {
            kind: 'published',
            type: 'esm',
            browser: false,
            name: '@scope/feature',
            private: false,
            member: true,
          },
        },
      }),
    ManifestValidationError,
  );
});

test('rejects a non-exact Foundry version', () => {
  assert.throws(
    () =>
      parseNpmManifest({
        foundry: { version: '^1.5.1' },
        packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
        packages: {
          '.': { kind: 'workspace-root', type: 'esm', browser: false, name: 'workspace', private: true, member: false },
        },
      }),
    ManifestValidationError,
  );
});

test('rejects a private package name without the -dev suffix', () => {
  assert.throws(
    () =>
      parseNpmManifest({
        packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
        packages: {
          '.': { kind: 'workspace-root', type: 'esm', browser: false, name: 'workspace', private: true, member: false },
          './feature': {
            kind: 'shared-helper',
            type: 'esm',
            browser: false,
            name: '@scope/feature',
            private: true,
            member: true,
          },
        },
      }),
    ManifestValidationError,
  );
});

test('rejects traversal in package keys', () => {
  assert.throws(
    () =>
      parseNpmManifest({
        packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
        packages: {
          '.': { kind: 'workspace-root', type: 'esm', browser: false, name: 'workspace', private: true, member: false },
          './../outside': { kind: 'standalone', type: 'esm', browser: false, name: 'outside', member: false },
        },
      }),
    ManifestValidationError,
  );
});

test('accepts a non-. workspace-root (an installation-root cluster) and its memberOf members', () => {
  const manifest = parseNpmManifest({
    packageJson: { published: { required: ['name'], excluded: ['private'] } },
    packages: {
      '.': { kind: 'workspace-root', type: 'esm', browser: false, name: 'workspace', private: true, member: false },
      './hardhat/v2': {
        kind: 'workspace-root',
        type: 'esm',
        browser: false,
        name: '@scope/hh-v2-cluster',
        private: true,
        member: false,
      },
      './hardhat/v2/plugin/pkg': {
        kind: 'published',
        type: 'esm',
        browser: false,
        name: '@scope/plugin',
        member: true,
        memberOf: './hardhat/v2',
      },
    },
  });
  assert.equal(manifest.packages['./hardhat/v2/plugin/pkg']?.memberOf, './hardhat/v2');
});

test('rejects bad memberOf declarations', () => {
  const base = {
    packageJson: { published: { required: ['name'], excluded: ['private'] } },
  };
  const root = { kind: 'workspace-root', type: 'esm', browser: false, name: 'w', private: true, member: false };
  const cases: readonly [Record<string, unknown>, RegExp][] = [
    // memberOf on a non-member
    [
      { '.': root, './x': { kind: 'non-package', type: 'esm', browser: false, member: false, memberOf: './y' } },
      /only a workspace member/,
    ],
    // explicit '.' is the default and must be omitted
    [
      {
        '.': root,
        './x/pkg': { kind: 'published', type: 'esm', browser: false, name: 'x', member: true, memberOf: '.' },
      },
      /omit memberOf/,
    ],
    // memberOf naming a non-root entry
    [
      {
        '.': root,
        './y': { kind: 'shared-helper', type: 'esm', browser: false, name: '@s/y-dev', private: true, member: true },
        './y/pkg': { kind: 'published', type: 'esm', browser: false, name: 'y', member: true, memberOf: './y' },
      },
      /not a workspace-root entry/,
    ],
    // a member outside its declared root's subtree
    [
      {
        '.': root,
        './hardhat/v2': { ...root, name: '@s/cluster' },
        './elsewhere/pkg': {
          kind: 'published',
          type: 'esm',
          browser: false,
          name: 'z',
          member: true,
          memberOf: './hardhat/v2',
        },
      },
      /must live inside its installation root/,
    ],
  ];
  for (const [packages, message] of cases) {
    assert.throws(() => parseNpmManifest({ ...base, packages }), message);
  }
});
