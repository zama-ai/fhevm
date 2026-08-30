import assert from 'node:assert/strict';
import test from 'node:test';

import {
  validateDependencyGroupPlacement,
  validateDependencyOrder,
  validateDevDependencyPlacement,
  validateForbiddenDependencies,
  validatePrivateRootPins,
  validateScriptDependencyDeclarations,
  validateSiblingRanges,
  validateWorkspaceMemberSpecs,
} from '../base/checks/dependencies.ts';
import { collectScriptPackageUses, commandInvokesBinary } from '../base/script-dependencies.ts';
import { loadedPackage, parseTestNpmManifest } from './helpers.ts';

const root = loadedPackage(
  '.',
  { kind: 'workspace-root', name: 'workspace', private: true, member: false },
  { name: 'workspace', private: true, devDependencies: { ethers: '6.17.0', hardhat: '2.28.6' } },
);

test('rule 3.3.3 rejects forbidden dependencies except on the exact package declaring an exception', () => {
  const manifest = parseTestNpmManifest({
    dependencies: { forbidden: ['solhint'] },
    packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
    packages: {
      '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      './allowed': {
        kind: 'standalone',
        name: 'allowed',
        member: false,
        dependencyExceptions: ['solhint'],
      },
      './forbidden': { kind: 'standalone', name: 'forbidden', member: false },
      './stale': {
        kind: 'standalone',
        name: 'stale',
        member: false,
        dependencyExceptions: ['solhint'],
      },
    },
  });
  const allowed = loadedPackage('./allowed', manifest.packages['./allowed']!, {
    devDependencies: { solhint: '6.2.1' },
  });
  const forbidden = loadedPackage('./forbidden', manifest.packages['./forbidden']!, {
    peerDependencies: { solhint: '^6.0.0' },
  });
  const stale = loadedPackage('./stale', manifest.packages['./stale']!, {});

  assert.deepEqual(validateForbiddenDependencies(manifest, [allowed, forbidden, stale]), [
    {
      rule: '3.3.3',
      packageKey: './forbidden',
      message: "package 'solhint' in 'peerDependencies' is forbidden by npm-manifest.json#dependencies.forbidden",
    },
    {
      rule: '3.3.3',
      packageKey: './stale',
      message: "dependency exception 'solhint' is unused; remove it from this package's manifest entry",
    },
  ]);
});

test("rule 4.2.4 requires kind 'dev' to use only devDependencies", () => {
  const valid = loadedPackage(
    './valid',
    {
      kind: 'dev',
      name: '@scope/valid-dev',
      private: true,
      member: true,
      publishedRelPath: './valid/pkg',
    },
    { devDependencies: { ethers: '6.17.0' } },
  );
  const invalid = loadedPackage(
    './invalid',
    {
      kind: 'dev',
      name: '@scope/invalid-dev',
      private: true,
      member: true,
      publishedRelPath: './invalid/pkg',
    },
    {
      dependencies: { ethers: '6.17.0' },
      peerDependencies: { hardhat: '^2.28.6' },
    },
  );

  assert.deepEqual(validateDevDependencyPlacement([valid]), []);
  assert.deepEqual(validateDevDependencyPlacement([invalid]), [
    {
      rule: '4.2.4',
      packageKey: './invalid',
      message: "kind 'dev' must declare 'ethers' in 'devDependencies', not 'dependencies'",
    },
    {
      rule: '4.2.4',
      packageKey: './invalid',
      message: "kind 'dev' must declare 'hardhat' in 'devDependencies', not 'peerDependencies'",
    },
  ]);
});

test('dependency sections must be alphabetically ordered', () => {
  const ordered = loadedPackage(
    './ordered',
    { kind: 'shared-helper', name: '@scope/ordered-dev', private: true, member: true },
    { dependencies: { '@scope/first': '1.0.0', alpha: '1.0.0', zebra: '1.0.0' } },
  );
  const unordered = loadedPackage(
    './unordered',
    { kind: 'shared-helper', name: '@scope/unordered-dev', private: true, member: true },
    {
      dependencies: { zebra: '1.0.0', alpha: '1.0.0' },
      devDependencies: { second: '1.0.0', first: '1.0.0' },
    },
  );

  assert.deepEqual(validateDependencyOrder([ordered]), []);
  assert.deepEqual(validateDependencyOrder([unordered]), [
    {
      rule: 'dependencies-order',
      packageKey: './unordered',
      message: "'dependencies' entries must be alphabetically ordered",
    },
    {
      rule: 'dependencies-order',
      packageKey: './unordered',
      message: "'devDependencies' entries must be alphabetically ordered",
    },
  ]);
});

test('rule 3.1.1 requires the exact plain workspace-member version', () => {
  const target = loadedPackage(
    './library/pkg',
    { kind: 'published', name: '@scope/library', member: true },
    { name: '@scope/library', version: '1.2.3' },
  );
  const consumer = loadedPackage(
    './consumer',
    { kind: 'internal-consumer', name: '@scope/consumer-dev', private: true, member: true },
    { name: '@scope/consumer-dev', version: '0.0.0', devDependencies: { '@scope/library': '^1.2.3' } },
  );

  const violations = validateWorkspaceMemberSpecs([root, target, consumer]);
  assert.equal(violations.length, 1);
  assert.equal(violations[0]?.rule, '3.1.1');
});

test('rule 4.2.1 requires imported root pins exactly and rejects unused declarations', () => {
  const importedWrong = loadedPackage(
    './imported',
    { kind: 'shared-helper', name: '@scope/imported-dev', private: true, member: true },
    { name: '@scope/imported-dev', private: true, devDependencies: { ethers: '^6.17.0' } },
  );
  const unused = loadedPackage(
    './unused',
    { kind: 'shared-helper', name: '@scope/unused-dev', private: true, member: true },
    { name: '@scope/unused-dev', private: true, devDependencies: { ethers: '6.17.0' } },
  );
  const missing = loadedPackage(
    './missing',
    { kind: 'shared-helper', name: '@scope/missing-dev', private: true, member: true },
    { name: '@scope/missing-dev', private: true },
  );

  const imports = new Map<string, ReadonlySet<string>>([
    ['./imported', new Set(['ethers'])],
    ['./unused', new Set()],
    ['./missing', new Set(['ethers'])],
  ]);
  const violations = validatePrivateRootPins([root, importedWrong, unused, missing], imports);
  assert.equal(violations.length, 4);
  assert.ok(violations.every((violation) => violation.rule === '4.2.1'));
  assert.equal(
    violations.find((violation) => violation.message.includes('must move'))?.message,
    "package 'ethers' must move from 'devDependencies' to 'dependencies' for kind 'shared-helper'",
  );
  assert.equal(
    violations.find((violation) => violation.packageKey === './missing')?.message,
    `imports root-pinned package 'ethers' but does not declare it; add "ethers": "6.17.0" to 'dependencies' as required for kind 'shared-helper'`,
  );
});

test('npm-script executable detection resolves commands without matching ordinary arguments', () => {
  const uses = collectScriptPackageUses(
    {
      lint: 'npm run forge:lint && eslint && tsc --noEmit',
      'prettier:check': 'prettier --check "**/*.ts"',
      report: 'echo eslint prettier typescript',
    },
    new Map([
      ['eslint', 'eslint'],
      ['prettier', 'prettier'],
      ['tsc', 'typescript'],
    ]),
  );

  assert.deepEqual([...(uses.get('eslint') ?? [])], ['eslint']);
  assert.deepEqual([...(uses.get('prettier') ?? [])], ['prettier']);
  assert.deepEqual([...(uses.get('typescript') ?? [])], ['tsc']);
  assert.equal(commandInvokesBinary('echo eslint', 'eslint'), false);
});

test('rule 3.3.1 requires npm-script tools in devDependencies', () => {
  const toolRoot = loadedPackage(
    '.',
    { kind: 'workspace-root', name: 'workspace', private: true, member: false },
    {
      name: 'workspace',
      private: true,
      devDependencies: { eslint: '^10.0.2', prettier: '^3.8.3', typescript: '^6.0.2' },
    },
  );
  const consumer = loadedPackage(
    './e2e',
    { kind: 'internal-consumer', name: '@scope/e2e-dev', private: true, member: true },
    {
      name: '@scope/e2e-dev',
      private: true,
      dependencies: { prettier: '^3.8.3' },
      devDependencies: { eslint: '^10.0.2' },
    },
  );
  const scriptUses = new Map([
    [
      './e2e',
      new Map([
        ['eslint', new Set(['eslint'])],
        ['prettier', new Set(['prettier'])],
        ['typescript', new Set(['tsc'])],
      ]),
    ],
  ]);

  assert.deepEqual(validateScriptDependencyDeclarations([toolRoot, consumer], scriptUses), [
    {
      rule: '3.3.1',
      packageKey: './e2e',
      message:
        "npm scripts invoke 'prettier' from package 'prettier'; move it from 'dependencies' to 'devDependencies'",
    },
    {
      rule: '3.3.1',
      packageKey: './e2e',
      message:
        "npm scripts invoke 'tsc' from root dependency 'typescript' but do not declare it; add \"typescript\": \"^6.0.2\" to 'devDependencies'",
    },
  ]);
});

test('rule 4.2.2 keeps a dependency that differs by dependency group out of the root', () => {
  const first = loadedPackage(
    './family/v1',
    {
      kind: 'dev',
      name: '@scope/v1-dev',
      private: true,
      member: true,
      dependencyGroup: 'family/v1',
      publishedRelPath: './family/v1/pkg',
    },
    { name: '@scope/v1-dev', private: true, devDependencies: { hardhat: '^2.0.0' } },
  );
  const second = loadedPackage(
    './family/v2',
    {
      kind: 'dev',
      name: '@scope/v2-dev',
      private: true,
      member: true,
      dependencyGroup: 'family/v2',
      publishedRelPath: './family/v2/pkg',
    },
    { name: '@scope/v2-dev', private: true, devDependencies: { hardhat: '^3.0.0' } },
  );

  const violations = validateDependencyGroupPlacement([root, first, second]);
  assert.deepEqual(violations, [
    {
      rule: '4.2.2',
      packageKey: '.',
      message:
        "'hardhat' has master declaration \"2.28.6\" in sdk/package.json field 'devDependencies', but member packages across dependency groups use different ranges (./family/v1=^2.0.0, ./family/v2=^3.0.0); remove it from sdk/package.json, or align the member ranges if the difference is unintended",
    },
  ]);
});

test('rule 4.2.3 requires sibling ranges to agree', () => {
  const first = loadedPackage(
    './family/plugin',
    {
      kind: 'dev',
      name: '@scope/plugin-dev',
      private: true,
      member: true,
      dependencyGroup: 'family/v1',
      publishedRelPath: './family/plugin/pkg',
    },
    { name: '@scope/plugin-dev', private: true, devDependencies: { hardhat: '^2.1.0' } },
  );
  const second = loadedPackage(
    './family/e2e',
    {
      kind: 'internal-consumer',
      name: '@scope/e2e-dev',
      private: true,
      member: true,
      dependencyGroup: 'family/v1',
    },
    { name: '@scope/e2e-dev', private: true, devDependencies: { hardhat: '^2.2.0' } },
  );

  const violations = validateSiblingRanges([root, first, second]);
  assert.equal(violations.length, 2);
  assert.ok(violations.every((violation) => violation.rule === '4.2.3'));
});
