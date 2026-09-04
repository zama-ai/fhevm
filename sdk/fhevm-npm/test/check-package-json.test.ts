import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { sortPackageJson, validatePackageJson } from '../base/checks/package-json.ts';
import { loadedPackage } from './helpers.ts';

const inventory = { kind: 'shared-helper', name: '@scope/feature-dev', private: true, member: true } as const;

test('accepts a package.json without a comment entry', () => {
  const violations = validatePackageJson([
    loadedPackage('./feature', inventory, { name: '@scope/feature-dev', private: true, type: 'module' }),
  ]);
  assert.deepEqual(violations, []);
});

test('rejects a top-level package.json comment entry', () => {
  const violations = validatePackageJson([
    loadedPackage('./feature', inventory, {
      name: '@scope/feature-dev',
      private: true,
      type: 'module',
      '//': 'This is not allowed.',
    }),
  ]);
  assert.deepEqual(violations, [
    {
      rule: 'package-json',
      packageKey: './feature/package.json',
      message: 'package.json must not contain a top-level "//" entry',
    },
  ]);
});

test('requires top-level package.json entries to follow the standalone canonical order', () => {
  const ordered = loadedPackage('./ordered', inventory, {
    name: '@scope/ordered-dev',
    version: '0.0.0',
    description: 'Ordered package',
    private: true,
    type: 'module',
    scripts: {},
    dependencies: {},
  });
  const unordered = loadedPackage('./unordered', inventory, {
    dependencies: {},
    name: '@scope/unordered-dev',
    type: 'module',
    private: true,
  });

  assert.deepEqual(validatePackageJson([ordered]), []);
  assert.deepEqual(validatePackageJson([unordered]), [
    {
      rule: 'package-json-order',
      packageKey: './unordered/package.json',
      message: 'top-level entries must follow package-json-order.ts; expected: name, private, type, dependencies',
    },
  ]);
});

test('requires scripts to be alphabetically ordered', () => {
  const ordered = loadedPackage('./ordered', inventory, {
    type: 'module',
    scripts: {
      build: 'tsc',
      lint: 'eslint .',
      test: 'node --test',
    },
  });
  const unordered = loadedPackage('./unordered', inventory, {
    type: 'module',
    scripts: {
      test: 'node --test',
      build: 'tsc',
      lint: 'eslint .',
    },
  });

  assert.deepEqual(validatePackageJson([ordered]), []);
  assert.deepEqual(validatePackageJson([unordered]), [
    {
      rule: 'scripts-order',
      packageKey: './unordered/package.json',
      message: "'scripts' entries must be alphabetically ordered",
    },
  ]);
});

test('leaves workspaces order alone — it encodes build order, not a sort', () => {
  // npm runs `--workspaces` scripts in this order, so a member consuming a sibling's build output has
  // to come after it. Alphabetical order cannot express that.
  const buildOrder = loadedPackage('.', inventory, {
    type: 'module',
    workspaces: ['hardhat/plugin', 'hardhat/e2e', 'common'],
  });

  assert.deepEqual(validatePackageJson([buildOrder]), []);
});

test('sorts top-level entries and scripts, but never the workspaces order', () => {
  const directory = mkdtempSync(join(tmpdir(), 'fhevm-npm-package-json-'));
  try {
    const packageJsonFile = join(directory, 'package.json');
    const pkg = {
      ...loadedPackage('./feature', inventory, {
        name: '@scope/feature-dev',
        private: true,
        type: 'module',
        scripts: { test: 'node --test', build: 'tsc', lint: 'eslint .' },
        workspaces: ['zebra', 'alpha'],
      }),
      directory,
    };
    writeFileSync(packageJsonFile, `${JSON.stringify(pkg.packageJson, null, 2)}\n`);

    assert.deepEqual(sortPackageJson([pkg]), ['./feature/package.json']);
    const sorted = JSON.parse(readFileSync(packageJsonFile, 'utf8')) as {
      scripts: Record<string, string>;
      workspaces: string[];
    };
    assert.deepEqual(Object.keys(sorted), ['name', 'private', 'type', 'workspaces', 'scripts']);
    assert.deepEqual(Object.keys(sorted.scripts), ['build', 'lint', 'test']);
    // Written order preserved: --sort moves the FIELD, never the entries inside it.
    assert.deepEqual(sorted.workspaces, ['zebra', 'alpha']);
    assert.deepEqual(sortPackageJson([{ ...pkg, packageJson: sorted }]), []);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test('sorts a package whose nested fields are already ordered', () => {
  const directory = mkdtempSync(join(tmpdir(), 'fhevm-npm-package-json-fields-'));
  try {
    const packageJsonFile = join(directory, 'package.json');
    const pkg = {
      ...loadedPackage('./feature', inventory, {
        dependencies: { alpha: '1.0.0' },
        type: 'module',
        name: '@scope/feature-dev',
      }),
      directory,
    };
    writeFileSync(packageJsonFile, `${JSON.stringify(pkg.packageJson, null, 2)}\n`);

    assert.deepEqual(sortPackageJson([pkg]), ['./feature/package.json']);
    const sorted = JSON.parse(readFileSync(packageJsonFile, 'utf8')) as Record<string, unknown>;
    assert.deepEqual(Object.keys(sorted), ['name', 'type', 'dependencies']);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("requires an explicit package type and enforces each test-consumer directory's module kind", () => {
  const missing = loadedPackage('./missing', inventory, {});
  const validCjs = loadedPackage(
    './library/test-consumer/cjs',
    { kind: 'standalone', name: 'library-consumer-cjs', private: true, member: false },
    { type: 'commonjs' },
  );
  const invalidCjs = loadedPackage(
    './invalid/test-consumer/cjs',
    { kind: 'standalone', name: 'invalid-consumer-cjs', private: true, member: false },
    { type: 'module' },
  );
  const validEsm = loadedPackage(
    './library/test-consumer/esm',
    { kind: 'standalone', name: 'library-consumer-esm', private: true, member: false },
    { type: 'module' },
  );
  const missingEsmType = loadedPackage(
    './missing/test-consumer/esm',
    { kind: 'standalone', name: 'missing-consumer-esm', private: true, member: false },
    {},
  );

  assert.deepEqual(validatePackageJson([missing, validCjs, invalidCjs, validEsm, missingEsmType]), [
    {
      rule: 'package-json-type',
      packageKey: './missing/package.json',
      message: "package.json must define 'type' as 'module' or 'commonjs'",
    },
    {
      rule: 'package-json-type',
      packageKey: './invalid/test-consumer/cjs/package.json',
      message: 'CJS consumer package must define "type": "commonjs"',
    },
    {
      rule: 'package-json-type',
      packageKey: './missing/test-consumer/esm/package.json',
      message: 'ESM consumer package must define "type": "module"',
    },
  ]);
});

test('requires configured metadata and forbids private on published packages', () => {
  const published = loadedPackage(
    './library/pkg',
    { kind: 'published', name: '@scope/library', member: true },
    {
      name: '@scope/library',
      version: '1.2.3',
      private: false,
      license: 'BSD-3-Clause-Clear',
      type: 'module',
    },
  );

  assert.deepEqual(
    validatePackageJson(
      [published],
      {
        published: { required: ['name', 'version', 'description', 'license'], excluded: ['private'] },
      },
      () => true,
    ),
    [
      {
        rule: 'package-json-policy',
        packageKey: './library/pkg/package.json',
        message:
          "kind 'published' must define 'description' as required by npm-manifest.json#packageJson.published.required",
      },
      {
        rule: 'package-json-policy',
        packageKey: './library/pkg/package.json',
        message:
          "kind 'published' must not contain 'private' as excluded by npm-manifest.json#packageJson.published.excluded",
      },
    ],
  );
});

test("requires the published license value and an exact root 'LICENSE' file", () => {
  const published = loadedPackage(
    './library/pkg',
    { kind: 'published', name: '@scope/library', member: true },
    {
      name: '@scope/library',
      version: '1.2.3',
      license: 'MIT',
      type: 'module',
    },
  );

  assert.deepEqual(
    validatePackageJson([published], {}, () => false),
    [
      {
        rule: 'published-license',
        packageKey: './library/pkg/package.json',
        message: 'published package must set "license": "BSD-3-Clause-Clear"; found "MIT"',
      },
      {
        rule: 'published-license',
        packageKey: './library/pkg/package.json',
        message: "published package must contain a regular 'LICENSE' file next to package.json",
      },
    ],
  );
});

test('forbids devDependencies in npm-distributed published packages but permits them in mirror-only projects', () => {
  const npmPackage = loadedPackage(
    './library/pkg',
    { kind: 'published', name: '@scope/library', member: false },
    {
      name: '@scope/library',
      version: '1.2.3',
      license: 'BSD-3-Clause-Clear',
      type: 'module',
      devDependencies: { typescript: '^5.9.3' },
    },
  );
  const mirrorPackage = loadedPackage(
    './mirror/pkg',
    { kind: 'published', name: 'mirror', member: false, distribution: ['mirror'] },
    {
      name: 'mirror',
      version: '1.2.3',
      license: 'BSD-3-Clause-Clear',
      type: 'module',
      devDependencies: { typescript: '^5.9.3' },
    },
  );

  assert.deepEqual(
    validatePackageJson([npmPackage, mirrorPackage], {}, () => true).filter((violation) => violation.rule === '2.1.2'),
    [
      {
        rule: '2.1.2',
        packageKey: './library/pkg/package.json',
        message:
          "npm-distributed published package must not contain 'devDependencies'; development dependencies belong on its dev owner",
      },
    ],
  );
});

test("requires the '@types/node' major to match the minimum 'engines.node' major", () => {
  const matchingRange = loadedPackage('./matching-range', inventory, {
    type: 'module',
    engines: { node: '>=22.6' },
    devDependencies: { '@types/node': '^22.0.0' },
  });
  const matchingExact = loadedPackage('./matching-exact', inventory, {
    type: 'module',
    engines: { node: '>=22' },
    devDependencies: { '@types/node': '22.20.1' },
  });
  const mismatched = loadedPackage('./mismatched', inventory, {
    type: 'module',
    engines: { node: '>=22' },
    devDependencies: { '@types/node': '^20.19.30' },
  });
  const missingEngine = loadedPackage('./missing-engine', inventory, {
    type: 'module',
    devDependencies: { '@types/node': '^22.0.0' },
  });

  assert.deepEqual(validatePackageJson([matchingRange, matchingExact]), []);
  assert.deepEqual(validatePackageJson([mismatched, missingEngine]), [
    {
      rule: 'node-types-version',
      packageKey: './mismatched/package.json',
      message: "'@types/node' in 'devDependencies' has major 20, but 'engines.node' \">=22\" requires major 22",
    },
    {
      rule: 'node-types-version',
      packageKey: './missing-engine/package.json',
      message: `declares '@types/node' but must also define 'engines.node', for example "node": ">=22"`,
    },
  ]);
});

test("requires the manifest's cjs, esm or dual type to match package.json entry points", () => {
  const mismatched = loadedPackage('./library', { ...inventory, type: 'dual' }, { type: 'module' });

  assert.deepEqual(validatePackageJson([mismatched]), [
    {
      rule: 'package-json-type',
      packageKey: './library/package.json',
      message: "manifest type 'dual' does not match package.json entry points 'esm'",
    },
  ]);
});
