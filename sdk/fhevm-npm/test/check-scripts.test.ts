import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
  prettierTargetsSolidity,
  validateConfigContainment,
  validateEslintConfigs,
  validatePrettierConfigs,
  validateScripts,
} from '../base/checks/scripts.ts';
import { loadedPackage } from './helpers.ts';

test('accepts conventional scripts on each resolved test owner', () => {
  const owner = devOwner({
    compile: 'tsc',
    clean: 'rm -rf ./dist *.tsbuildinfo',
    lint: 'eslint .',
    'pack:tarball': 'npm pack ./pkg',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    fmt: 'npm run prettier:write',
    'fmt:check': 'npm run prettier:check',
    'check:publint': 'publint --strict ./pkg',
    check: 'npm run check:publint && npm run check:vendored-origin',
    'test:consumer': 'node ./test-consumer.ts',
    'check:vendored-origin': 'node ./check-vendored-origin.ts',
    'check:mirror': 'node ./check-mirror.ts',
  });
  const published = publishedPackage({
    vendored: [vendoredCapability()],
    mirror: mirrorCapability(),
    consumerTests: { cjs: ['./library/test-consumer/cjs'] },
  });
  const standalone = loadedPackage(
    './standalone',
    { kind: 'standalone', name: 'consumer', member: false, mirror: mirrorCapability() },
    { name: 'consumer', scripts: { 'check:mirror': 'node ./check-mirror.ts' } },
  );

  assert.deepEqual(
    validateScripts(
      [owner, published, consumerFixture('cjs'), standalone],
      () => false,
      () => true,
    ),
    [],
  );
});

test('reports missing scripts, unresolved owners, and scripts on the wrong kind', () => {
  const owner = devOwner({});
  const published = publishedPackage(
    { vendored: [vendoredCapability()], consumerTests: { cjs: ['./library/test-consumer/cjs'] } },
    { 'test:consumer': 'wrong owner' },
  );
  const orphan = loadedPackage(
    './orphan/pkg',
    {
      kind: 'published',
      name: '@scope/orphan',
      member: false,
      mirror: mirrorCapability(),
      consumerTests: { cjs: ['./orphan/test-consumer/cjs'] },
    },
    { name: '@scope/orphan' },
  );
  const orphanConsumer = consumerFixture('cjs', './orphan', '@scope/orphan');

  const violations = validateScripts(
    [owner, published, consumerFixture('cjs'), orphan, orphanConsumer],
    () => false,
    () => true,
  );
  assert.equal(violations.filter((violation) => violation.rule === '2.1.2').length, 2);
  assert.equal(violations.filter((violation) => violation.rule === '5.1.3').length, 1);
  assert.equal(violations.filter((violation) => violation.rule === '5.1.4').length, 2);
  assert.equal(violations.filter((violation) => violation.rule === '5.2.1').length, 2);
  assert.equal(violations.filter((violation) => violation.rule === '5.3.1').length, 1);
  assert.equal(violations.filter((violation) => violation.rule === '5.3.2').length, 1);
  assert.equal(violations.filter((violation) => violation.rule === 'package-scripts').length, 5);
});

test('forbids scripts in npm-distributed published packages but preserves mirror-only consumer scripts', () => {
  const npmPackage = publishedPackage({}, { build: 'tsc' });
  const mirrorPackage = publishedPackage({ distribution: ['mirror'] }, { build: 'tsc' });

  assert.deepEqual(
    validateScripts([npmPackage, mirrorPackage]).filter((violation) => violation.rule === '2.1.2'),
    [
      {
        rule: '2.1.2',
        packageKey: './library/pkg',
        message: "npm-distributed published package must not contain 'scripts'; scripts belong on its dev owner",
      },
    ],
  );
});

test('preserves the upstream packaging, consumer, formatting and linting policy of a mirror-only payload', () => {
  const owner = devOwner({
    compile: 'tsc',
    clean: 'rm -rf ./dist *.tsbuildinfo',
    lint: 'eslint .',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    fmt: 'npm run prettier:write',
    'fmt:check': 'npm run prettier:check',
    'check:publint': 'publint --strict ./pkg',
    check: 'npm run check:publint',
  });
  const published = publishedPackage(
    { distribution: ['mirror'] },
    {
      'prettier:check': 'prettier --check "contracts/**/*.sol"',
      'prettier:write': 'prettier --write "contracts/**/*.sol"',
    },
  );

  assert.deepEqual(
    validateScripts([owner, published], (pkg) => pkg.key === published.key),
    [],
  );
});

test('forbids lint and Prettier configs on a dev owner containing only package.json and pkg', () => {
  const owner = loadedPackage(
    './mirror',
    {
      kind: 'dev',
      name: '@scope/mirror-dev',
      private: true,
      member: true,
      publishedRelPath: './mirror/pkg',
    },
    {
      type: 'module',
      scripts: {
        clean: 'rm -rf dist *.tsbuildinfo',
        lint: 'npm run lint --prefix ./pkg',
        'prettier:check': 'npm run prettier:check --prefix ./pkg',
        'prettier:write': 'npm run prettier:write --prefix ./pkg',
      },
    },
  );
  const payload = loadedPackage(
    './mirror/pkg',
    { kind: 'published', name: 'mirror', member: true, distribution: ['mirror'] },
    { name: 'mirror', type: 'commonjs' },
  );

  const files = ['eslint.config.js', 'package.json', 'prettier.config.js'];
  const directories = ['pkg'];

  assert.deepEqual(
    validatePrettierConfigs(
      '/workspace',
      [owner, payload],
      () => undefined,
      (directory) => (directory === owner.directory ? files : []),
      (directory) => (directory === owner.directory ? directories : []),
    ),
    [
      {
        rule: '5.1.6',
        packageKey: './mirror',
        message: "source-empty dev owner must not contain Prettier configuration file 'prettier.config.js'",
      },
    ],
  );
  assert.deepEqual(
    validateEslintConfigs(
      [owner, payload],
      (directory) => (directory === owner.directory ? files : []),
      (directory) => (directory === owner.directory ? directories : []),
    ),
    [
      {
        rule: '5.1.7',
        packageKey: './mirror',
        message: "source-empty dev owner must not contain ESLint configuration file 'eslint.config.js'",
      },
    ],
  );
});

test('requires a consumerTests registration for every module format a payload exposes', () => {
  const owner = devOwner({
    compile: 'tsc',
    clean: 'rm -rf ./dist *.tsbuildinfo',
    lint: 'eslint .',
    'pack:tarball': 'npm pack ./pkg',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    fmt: 'npm run prettier:write',
    'fmt:check': 'npm run prettier:check',
    'check:publint': 'publint --strict ./pkg',
    check: 'npm run check:publint',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage({}, undefined, {
    type: 'module',
    main: './dist/index.cjs',
    module: './dist/index.js',
    exports: { '.': { import: './dist/index.js', require: './dist/index.cjs' } },
  });

  // A conventional sibling fixture registers NOTHING by existing: it is present here and still unused.
  const violations = validateScripts([owner, published, consumerFixture('cjs'), consumerFixture('esm')]);

  assert.deepEqual(violations, [
    {
      rule: '5.3.1',
      packageKey: './library/pkg',
      message: 'published package exposes CJS but registers no CJS consumer in npm-manifest.json#consumerTests',
    },
    {
      rule: '5.3.1',
      packageKey: './library/pkg',
      message: 'published package exposes ESM but registers no ESM consumer in npm-manifest.json#consumerTests',
    },
  ]);
});

test('accepts a manifest-selected consumer instead of a sibling fixture', () => {
  const owner = devOwner({
    compile: 'tsc',
    clean: 'rm -rf ./dist *.tsbuildinfo',
    lint: 'eslint .',
    'pack:tarball': 'npm pack ./pkg',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    fmt: 'npm run prettier:write',
    'fmt:check': 'npm run prettier:check',
    'test:consumer': 'node ./consumer.js',
    'check:publint': 'publint --strict ./pkg',
    check: 'npm run check:publint',
  });
  const published = publishedPackage({ consumerTests: { cjs: ['./template/pkg'] } });
  const template = loadedPackage(
    './template/pkg',
    {
      kind: 'published',
      name: 'template',
      member: true,
      distribution: ['mirror'],
    },
    {
      name: 'template',
      type: 'commonjs',
      scripts: { test: 'node ./test.js' },
      devDependencies: { '@scope/library': 'file:../../library/pkg' },
    },
  );

  assert.deepEqual(
    validateScripts(
      [owner, published, template],
      () => false,
      () => true,
    ),
    [],
  );
});

test('validates every consumer of a registration array, holding a registered consumer to the fixture rules', () => {
  const owner = devOwner({
    build: 'tsc --build',
    clean: 'rm -rf ./dist *.tsbuildinfo',
    lint: 'eslint .',
    'pack:tarball': 'npm pack ./pkg',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    fmt: 'npm run prettier:write',
    'fmt:check': 'npm run prettier:check',
    'test:consumer': 'node ./consumer.js',
    'check:publint': 'publint --strict ./pkg',
    check: 'npm run check:publint',
  });
  const published = publishedPackage(
    { consumerTests: { esm: ['./library/test-consumer/esm', './template/pkg', './missing'] } },
    undefined,
    { type: 'module', exports: { '.': { import: './dist/index.js' } } },
  );
  // Registered, member and internal-consumer: a valid shape, but it forgot `private`.
  const template = loadedPackage(
    './template/pkg',
    { kind: 'internal-consumer', name: 'template', private: true, member: true },
    {
      name: 'template',
      type: 'module',
      scripts: { test: 'node ./test.js' },
      devDependencies: { '@scope/library': 'file:../../library/pkg' },
    },
  );

  const violations = validateScripts(
    [owner, published, consumerFixture('esm'), template],
    () => false,
    () => true,
  );

  assert.deepEqual(
    violations.filter((violation) => violation.rule === '5.3.1'),
    [
      {
        rule: '5.3.1',
        packageKey: './template/pkg',
        message: 'consumer fixture must set private=true',
      },
      {
        rule: '5.3.1',
        packageKey: './library/pkg',
        message: "registered ESM consumer './missing' is not a package in npm-manifest.json",
      },
    ],
  );
});

test('validates each format-specific consumer fixture', () => {
  const owner = devOwner({
    compile: 'tsc',
    clean: 'rm -rf ./dist *.tsbuildinfo',
    lint: 'eslint .',
    'pack:tarball': 'npm pack ./pkg',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    fmt: 'npm run prettier:write',
    'fmt:check': 'npm run prettier:check',
    'check:publint': 'publint --strict ./pkg',
    check: 'npm run check:publint',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage(
    { consumerTests: { cjs: ['./library/test-consumer/cjs'], esm: ['./library/test-consumer/esm'] } },
    undefined,
    {
      type: 'module',
      main: './dist/index.cjs',
      module: './dist/index.js',
      exports: { '.': { import: './dist/index.js', require: './dist/index.cjs' } },
    },
  );
  const invalidCjs = loadedPackage(
    './library/test-consumer/cjs',
    { kind: 'standalone', name: 'library-consumer-cjs', private: true, member: true },
    { name: 'library-consumer-cjs', private: false, type: 'module', scripts: {} },
  );

  const violations = validateScripts(
    [owner, published, invalidCjs, consumerFixture('esm')],
    () => false,
    () => true,
  );

  // The conventional sibling and a registered consumer go through the SAME validator, so the sibling is
  // also held to the format and the direct payload link that only registered consumers used to prove.
  assert.deepEqual(
    violations.filter((violation) => violation.packageKey === './library/test-consumer/cjs'),
    [
      {
        rule: '5.3.1',
        packageKey: './library/test-consumer/cjs',
        message:
          "consumer must be a non-member 'standalone' fixture, or a member 'internal-consumer' or mirror-only 'published' template; found kind 'standalone' with member=true",
      },
      {
        rule: '5.3.1',
        packageKey: './library/test-consumer/cjs',
        message: 'consumer fixture must set private=true',
      },
      {
        rule: '5.3.1',
        packageKey: './library/test-consumer/cjs',
        message: "consumer of './library/pkg' does not execute as CJS",
      },
      {
        rule: '5.3.1',
        packageKey: './library/test-consumer/cjs',
        message: "consumer fixture must define a non-empty 'test' script",
      },
      {
        rule: '5.3.1',
        packageKey: './library/test-consumer/cjs',
        message: "consumer must directly link '@scope/library' to './library/pkg' with a directory 'file:' dependency",
      },
    ],
  );
});

test("requires serial execution when a consumer fixture uses 'node --test'", () => {
  const owner = devOwner({
    compile: 'tsc',
    clean: 'rm -rf ./dist *.tsbuildinfo',
    lint: 'eslint .',
    'pack:tarball': 'npm pack ./pkg',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    fmt: 'npm run prettier:write',
    'fmt:check': 'npm run prettier:check',
    'check:publint': 'publint --strict ./pkg',
    check: 'npm run check:publint',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage(
    { consumerTests: { cjs: ['./library/test-consumer/cjs'], esm: ['./library/test-consumer/esm'] } },
    undefined,
    {
      type: 'module',
      main: './dist/index.cjs',
      module: './dist/index.js',
      exports: { '.': { import: './dist/index.js', require: './dist/index.cjs' } },
    },
  );
  const cjs = consumerFixture('cjs');
  const esmFixture = consumerFixture('esm');
  const esm = {
    ...esmFixture,
    packageJson: {
      ...esmFixture.packageJson,
      scripts: { test: 'node --import tsx --test test/*.test.ts' },
    },
  };

  assert.deepEqual(
    validateScripts(
      [owner, published, cjs, esm],
      () => false,
      () => true,
    ),
    [
      {
        rule: '5.3.9',
        packageKey: './library/test-consumer/esm',
        message: "test-consumer parallelism is forbidden; 'node --test' must set '--test-concurrency=1'",
      },
    ],
  );
});

test('requires Forge scripts on the owner of a published payload containing Solidity', () => {
  const owner = devOwner({
    compile: 'tsc',
    clean: 'rm -rf ./dist *.tsbuildinfo',
    lint: 'eslint .',
    'pack:tarball': 'npm pack ./pkg',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    fmt: 'npm run prettier:write',
    'fmt:check': 'npm run prettier:check',
    'check:publint': 'publint --strict ./pkg',
    check: 'npm run check:publint',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage({});
  const fixture = consumerFixture('cjs');

  const violations = validateScripts([owner, published, fixture], (pkg) => pkg.key === published.key);

  assert.deepEqual(
    violations.filter((violation) => violation.rule === 'package-scripts'),
    [
      {
        rule: 'package-scripts',
        packageKey: './library',
        message: "package must define a non-empty 'forge:fmt' script for package './library/pkg' containing Solidity",
      },
      {
        rule: 'package-scripts',
        packageKey: './library',
        message:
          "package must define a non-empty 'forge:fmt:check' script for package './library/pkg' containing Solidity",
      },
      {
        rule: 'package-scripts',
        packageKey: './library',
        message: "package must define a non-empty 'forge:lint' script for package './library/pkg' containing Solidity",
      },
    ],
  );
});

test('requires Forge scripts directly on a non-published package containing Solidity', () => {
  const internalConsumer = loadedPackage(
    './e2e',
    { kind: 'internal-consumer', name: '@scope/e2e-dev', private: true, member: true },
    { name: '@scope/e2e-dev', private: true, scripts: { clean: 'rm -rf dist' } },
  );

  assert.deepEqual(
    validateScripts([internalConsumer], () => true),
    [
      {
        rule: '5.1.4',
        packageKey: './e2e',
        message: "package must define a non-empty 'fmt' script for private workspace hygiene",
      },
      {
        rule: '5.1.4',
        packageKey: './e2e',
        message: "package must define a non-empty 'fmt:check' script for private workspace hygiene",
      },
      {
        rule: '5.1.4',
        packageKey: './e2e',
        message: "package must define a non-empty 'lint' script for private workspace hygiene",
      },
      {
        rule: '5.1.4',
        packageKey: './e2e',
        message: "package must define a non-empty 'prettier:check' script for private workspace hygiene",
      },
      {
        rule: '5.1.4',
        packageKey: './e2e',
        message: "package must define a non-empty 'prettier:write' script for private workspace hygiene",
      },
      {
        rule: 'package-scripts',
        packageKey: './e2e',
        message: "package must define a non-empty 'forge:fmt' script for package './e2e' containing Solidity",
      },
      {
        rule: 'package-scripts',
        packageKey: './e2e',
        message: "package must define a non-empty 'forge:fmt:check' script for package './e2e' containing Solidity",
      },
      {
        rule: 'package-scripts',
        packageKey: './e2e',
        message: "package must define a non-empty 'forge:lint' script for package './e2e' containing Solidity",
      },
    ],
  );
});

test('requires Forge scripts on the adjacent -dev owner of a workspace-native pkg template', () => {
  const owner = loadedPackage(
    './template',
    { kind: 'internal-consumer', name: '@scope/template-dev', private: true, member: true },
    {
      name: '@scope/template-dev',
      private: true,
      scripts: {
        clean: 'npm run clean --prefix ./pkg',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
        lint: 'npm run lint --prefix ./pkg',
        'prettier:check': 'npm run prettier:check --prefix ./pkg',
        'prettier:write': 'npm run prettier:write --prefix ./pkg',
      },
    },
  );
  const template = loadedPackage(
    './template/pkg',
    { kind: 'internal-consumer', name: 'template-dev', private: true, member: true },
    {
      name: 'template-dev',
      private: true,
      scripts: {
        clean: 'rm -rf dist',
        lint: 'eslint .',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
      },
    },
  );

  const violations = validateScripts([owner, template], (pkg) => pkg.key === template.key);

  assert.deepEqual(
    violations.filter((violation) => violation.rule === '5.1.4'),
    [],
  );
  assert.deepEqual(
    violations.filter((violation) => violation.rule === 'package-scripts'),
    [
      {
        rule: 'package-scripts',
        packageKey: './template',
        message: "package must define a non-empty 'forge:fmt' script for package './template/pkg' containing Solidity",
      },
      {
        rule: 'package-scripts',
        packageKey: './template',
        message:
          "package must define a non-empty 'forge:fmt:check' script for package './template/pkg' containing Solidity",
      },
      {
        rule: 'package-scripts',
        packageKey: './template',
        message: "package must define a non-empty 'forge:lint' script for package './template/pkg' containing Solidity",
      },
    ],
  );
});

test('does not require package-level Forge scripts on the Make-orchestrated workspace root', () => {
  const root = loadedPackage(
    '.',
    { kind: 'workspace-root', name: 'workspace', private: true, member: false },
    { name: 'workspace', private: true, type: 'module', scripts: { build: 'make build', 'test:ci': 'make ci' } },
  );

  assert.deepEqual(
    validateScripts([root], () => true),
    [],
  );
});

test('requires hygiene scripts on shared helpers', () => {
  const helper = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    { name: '@scope/common-dev', private: true, scripts: { clean: 'rm -rf dist', lint: 'eslint .' } },
  );

  assert.deepEqual(
    validateScripts([helper], () => false),
    [
      {
        rule: '5.1.4',
        packageKey: './common',
        message: "package must define a non-empty 'fmt' script for private workspace hygiene",
      },
      {
        rule: '5.1.4',
        packageKey: './common',
        message: "package must define a non-empty 'fmt:check' script for private workspace hygiene",
      },
      {
        rule: '5.1.4',
        packageKey: './common',
        message: "package must define a non-empty 'prettier:check' script for private workspace hygiene",
      },
      {
        rule: '5.1.4',
        packageKey: './common',
        message: "package must define a non-empty 'prettier:write' script for private workspace hygiene",
      },
    ],
  );
});

test("requires 'generate' and 'clean:generated' on any package that defines a 'generate:*' script", () => {
  const generator = loadedPackage(
    './generator',
    { kind: 'shared-helper', name: '@scope/generator-dev', private: true, member: true },
    {
      name: '@scope/generator-dev',
      private: true,
      scripts: {
        clean: 'rm -rf dist *.tsbuildinfo',
        lint: 'eslint .',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
        'generate:exports': 'node ./generate.ts',
      },
    },
  );

  assert.deepEqual(
    validateScripts([generator], () => false),
    [
      {
        rule: 'package-scripts',
        packageKey: './generator',
        message: "package must define a non-empty 'generate' script for the 'generate:*' scripts it defines",
      },
      {
        rule: 'package-scripts',
        packageKey: './generator',
        message: "package must define a non-empty 'clean:generated' script for the 'generate:*' scripts it defines",
      },
    ],
  );
});

test("rejects 'generate' or 'clean:generated' on a package with no 'generate:*' leaf", () => {
  const deadWiring = loadedPackage(
    './hollow',
    { kind: 'shared-helper', name: '@scope/hollow-dev', private: true, member: true },
    {
      name: '@scope/hollow-dev',
      private: true,
      scripts: {
        clean: 'rm -rf dist',
        'clean:generated': 'rm -rf generated',
        generate: 'echo nothing',
        lint: 'eslint .',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
      },
    },
  );

  const violations = validateScripts([deadWiring], () => false);
  assert.deepEqual(
    violations.map((violation) => violation.message),
    [
      "'generate' exists but the package defines no 'generate:*' script; an aggregate over nothing is dead wiring",
      "'clean:generated' exists but the package defines no 'generate:*' script; an aggregate over nothing is dead wiring",
    ],
  );
});

test('requires every generator and deliverable check to be reachable from its verb', () => {
  const generator = loadedPackage(
    './generator',
    { kind: 'shared-helper', name: '@scope/generator-dev', private: true, member: true },
    {
      name: '@scope/generator-dev',
      private: true,
      scripts: {
        check: 'npm run check:sizes',
        'check:exports': 'node ./generate.ts --check',
        'check:mirror': 'node ./check-mirror.ts',
        'check:sizes': 'node ./check-sizes.ts',
        'check:stray': 'node ./check-stray.ts',
        clean: 'rm -rf dist *.tsbuildinfo',
        'clean:generated': 'rm -rf generated',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
        generate: 'npm run generate:exports',
        'generate:exports': 'node ./generate.ts',
        'generate:genesis': 'node ./genesis.ts',
        'generate:stray': 'node ./stray.ts',
        lint: 'eslint .',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
      },
    },
  );

  assert.deepEqual(
    validateScripts([generator], () => false),
    [
      {
        rule: '5.1.4b',
        packageKey: './generator',
        message: "'generate:stray' is not reachable from 'generate'; the regeneration gate would never run it",
      },
      {
        rule: '5.2.1',
        packageKey: './generator',
        message: "'check:stray' is not reachable from 'check'",
      },
    ],
  );
});

test("requires 'clean:generated' to delete every export-manifest output", () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-scripts-manifest-'));
  try {
    writeFileSync(
      join(workspace, 'export.manifest.json'),
      JSON.stringify({
        outputs: {
          exports: './pkg/ts/index.ts',
          testConsumers: { esm: './test-consumer/esm/src/export.ts', cjs: './test-consumer/cjs/src/export.ts' },
        },
      }),
    );
    const generator = {
      ...loadedPackage(
        './generator',
        { kind: 'shared-helper', name: '@scope/generator-dev', private: true, member: true },
        {
          name: '@scope/generator-dev',
          private: true,
          scripts: {
            clean: 'rm -rf dist *.tsbuildinfo',
            'clean:generated': 'npm run clean:generate:exports',
            'clean:generate:exports': 'rm -rf pkg/ts/index.ts test-consumer/esm/src/export.ts',
            fmt: 'npm run prettier:write',
            'fmt:check': 'npm run prettier:check',
            generate: 'npm run generate:exports',
            'generate:exports': 'node ./generate.ts',
            lint: 'eslint .',
            'prettier:check': 'prettier --check .',
            'prettier:write': 'prettier --write .',
          },
        },
      ),
      directory: workspace,
    };

    assert.deepEqual(
      validateScripts([generator], () => false),
      [
        {
          rule: '5.1.4b',
          packageKey: './generator',
          message: "'clean:generated' does not delete export-manifest output './test-consumer/cjs/src/export.ts'",
        },
      ],
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test("requires 'clean' to delete '*.tsbuildinfo' when the package runs tsc", () => {
  const helper = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    {
      name: '@scope/common-dev',
      private: true,
      scripts: {
        clean: 'rm -rf dist',
        lint: 'eslint && tsc -p ./tsconfig.json --noEmit',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
      },
    },
  );

  assert.deepEqual(
    validateScripts([helper], () => false),
    [
      {
        rule: 'package-scripts',
        packageKey: './common',
        message:
          "'clean' must delete '*.tsbuildinfo'; the package runs tsc, and a surviving build-info file " +
          'lets the next typecheck resume from stale state',
      },
    ],
  );
});

test("follows 'npm run' references when checking what 'clean' deletes", () => {
  const helper = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    {
      name: '@scope/common-dev',
      private: true,
      scripts: {
        clean: 'npm run clean:ts && npm run clean:self',
        'clean:self': 'npm run clean', // A cycle must not recurse forever.
        'clean:ts': 'rm -rf dist *.tsbuildinfo',
        lint: 'eslint && tsc -p ./tsconfig.json --noEmit',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
      },
    },
  );

  assert.deepEqual(
    validateScripts([helper], () => false),
    [],
  );
});

test("skips flags after 'npm run' when resolving clean references", () => {
  const helper = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    {
      name: '@scope/common-dev',
      private: true,
      scripts: {
        clean: 'npm run --silent clean:ts',
        'clean:ts': 'rm -rf dist *.tsbuildinfo',
        lint: 'eslint && tsc -p ./tsconfig.json --noEmit',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
      },
    },
  );

  assert.deepEqual(
    validateScripts([helper], () => false),
    [],
  );
});

test("does not resolve cross-package 'npm run' clauses in the caller's namespace", () => {
  const helper = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    {
      name: '@scope/common-dev',
      private: true,
      scripts: {
        // `wipe` exists locally but the clause is routed elsewhere; it must not count as coverage.
        clean: 'npm run wipe --prefix ../elsewhere',
        wipe: 'rm -rf dist *.tsbuildinfo',
        lint: 'eslint && tsc -p ./tsconfig.json --noEmit',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
      },
    },
  );

  const violations = validateScripts([helper], () => false);
  assert.equal(violations.length, 1);
  assert.match(violations[0]?.message ?? '', /'clean' must delete '\*\.tsbuildinfo'/);
});

test("requires a 'clean' script on a private source-owning package that has none", () => {
  const helper = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    {
      name: '@scope/common-dev',
      private: true,
      // No `clean` on purpose — that is what this test asserts.
      scripts: {
        lint: 'eslint .',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
      },
    },
  );

  assert.deepEqual(
    validateScripts([helper], () => false),
    [
      {
        rule: 'package-scripts',
        packageKey: './common',
        message: "package must define a non-empty 'clean' script for removing its own build output",
      },
    ],
  );
});

test("does not require 'clean:generated' on a package with no generators", () => {
  const plain = loadedPackage(
    './plain',
    { kind: 'shared-helper', name: '@scope/plain-dev', private: true, member: true },
    {
      name: '@scope/plain-dev',
      private: true,
      scripts: {
        clean: 'rm -rf dist',
        lint: 'eslint .',
        'prettier:check': 'prettier --check .',
        'prettier:write': 'prettier --write .',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
      },
    },
  );

  assert.deepEqual(
    validateScripts([plain], () => false),
    [],
  );
});

test('rejects Solidity targets in Prettier scripts', () => {
  const internalConsumer = loadedPackage(
    './e2e',
    { kind: 'internal-consumer', name: '@scope/e2e-dev', private: true, member: true },
    {
      name: '@scope/e2e-dev',
      private: true,
      scripts: {
        clean: 'rm -rf dist *.tsbuildinfo',
        lint: 'eslint .',
        'prettier:check': 'prettier --check "contracts/**/*.sol"',
        'prettier:write': 'prettier --write "**/*.{js,json,md,sol,ts,yml}"',
        fmt: 'npm run prettier:write',
        'fmt:check': 'npm run prettier:check',
      },
    },
  );

  assert.deepEqual(
    validateScripts([internalConsumer], () => false),
    [
      {
        rule: '5.1.5',
        packageKey: './e2e',
        message: "'prettier:check' must not target Solidity; use 'forge:fmt:check'",
      },
      {
        rule: '5.1.5',
        packageKey: './e2e',
        message: "'prettier:write' must not target Solidity; use 'forge:fmt'",
      },
    ],
  );
});

test('recognizes Solidity extensions without matching unrelated text', () => {
  assert.equal(prettierTargetsSolidity('prettier --check "**/*.{js,sol,ts}"'), true);
  assert.equal(prettierTargetsSolidity('prettier --check contracts/FHE.sol'), true);
  assert.equal(prettierTargetsSolidity('prettier --check "**/*.{js,json,md,ts,yml}"'), false);
  assert.equal(prettierTargetsSolidity('prettier --check console.ts'), false);
});

test("requires the exact 'prettier.config.js' filename and a reference to the workspace config", () => {
  const validCommonjs = loadedPackage(
    './hardhat/v2/e2e',
    { kind: 'internal-consumer', name: '@scope/e2e-dev', private: true, member: true },
    { type: 'commonjs', scripts: { 'prettier:check': 'prettier --check .' } },
  );
  const validJs = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    { type: 'module', scripts: { 'prettier:write': 'prettier --write .' } },
  );
  const missing = loadedPackage(
    './missing',
    { kind: 'shared-helper', name: '@scope/missing-dev', private: true, member: true },
    { type: 'module', scripts: { 'prettier:write': 'prettier --write .' } },
  );
  const incorrect = loadedPackage(
    './library',
    {
      kind: 'dev',
      name: '@scope/library-dev',
      private: true,
      member: true,
      publishedRelPath: './library/pkg',
    },
    { type: 'module', scripts: { 'prettier:check': 'prettier --check .' } },
  );
  const published = publishedPackage({}, { 'prettier:check': 'prettier --check .' });
  const contentsByFile = new Map([
    [
      '/workspace/hardhat/v2/e2e/prettier.config.js',
      "module.exports = import('../../../prettier.base.mjs').then((module) => module.default);\n",
    ],
    ['/workspace/common/prettier.config.js', "export { default } from '../prettier.base.mjs';\n"],
    ['/workspace/library/prettier.config.js', "export { default } from './local-prettier.mjs';\n"],
  ]);
  const filesByDirectory = new Map<string, readonly string[]>([
    ['/workspace/hardhat/v2/e2e', ['prettier.config.js']],
    ['/workspace/common', ['prettier.config.js']],
    ['/workspace/missing', []],
    ['/workspace/library', ['.prettierrc.mjs', 'prettier.config.js']],
    ['/workspace/library/pkg', []],
  ]);

  assert.deepEqual(
    validatePrettierConfigs(
      '/workspace',
      [validCommonjs, validJs, missing, incorrect, published],
      (file) => contentsByFile.get(file),
      (directory) => filesByDirectory.get(directory) ?? [],
    ),
    [
      {
        rule: '5.1.6',
        packageKey: './missing',
        message: "package with Prettier scripts must contain 'prettier.config.js' referencing '../prettier.base.mjs'",
      },
      {
        rule: '5.1.6',
        packageKey: './library',
        message: "Prettier configuration file '.prettierrc.mjs' is forbidden; use only 'prettier.config.js'",
      },
      {
        rule: '5.1.6',
        packageKey: './library',
        message: "'prettier.config.js' must contain: export { default } from '../prettier.base.mjs';",
      },
    ],
  );
});

test('forbids portable package configs from importing workspace files', () => {
  const owner = loadedPackage(
    './template',
    { kind: 'internal-consumer', name: '@scope/template-dev', private: true, member: true },
    { name: '@scope/template-dev', private: true },
  );
  const template = loadedPackage(
    './template/pkg',
    { kind: 'internal-consumer', name: 'template-dev', private: true, member: true },
    {
      name: 'template-dev',
      private: true,
      type: 'module',
      scripts: { 'prettier:check': 'prettier --check .' },
    },
  );
  const published = publishedPackage({ distribution: ['mirror'] });
  const listFiles = (directory: string): readonly string[] => {
    if (directory === '/workspace/template/pkg') return ['prettier.config.js'];
    if (directory === '/workspace/library/pkg') return ['eslint.config.js'];
    return [];
  };
  const escapingContents = new Map([
    ['/workspace/template/pkg/prettier.config.js', "export { default } from '../../../prettier.base.mjs';\n"],
    ['/workspace/library/pkg/eslint.config.js', "export { default } from '../../eslint.base.mjs';\n"],
  ]);

  assert.deepEqual(
    validateConfigContainment([owner, template, published], (file) => escapingContents.get(file), listFiles),
    [
      {
        rule: 'package-config-containment',
        packageKey: './template/pkg',
        message:
          "'prettier.config.js' imports '../../../prettier.base.mjs', which resolves outside the portable package",
      },
      {
        rule: 'package-config-containment',
        packageKey: './library/pkg',
        message: "'eslint.config.js' imports '../../eslint.base.mjs', which resolves outside the portable package",
      },
    ],
  );

  assert.deepEqual(
    validateConfigContainment(
      [owner, template, published],
      () => "import eslint from '@eslint/js'; export default eslint.configs.recommended;\n",
      listFiles,
    ),
    [],
  );
});

test("reserves 'prettier.base.mjs' for the workspace root, beside its own 'prettier.config.js'", () => {
  const root = loadedPackage(
    '.',
    { kind: 'workspace-root', name: 'workspace', private: true, member: false },
    { name: 'workspace', private: true },
  );
  const rootConfig = "export { default } from './prettier.base.mjs';\n";

  // The root is the sole package carrying two configs: the shared base, and the re-export that makes
  // Prettier discover a config for the root's own files. Any third filename is still forbidden.
  assert.deepEqual(
    validatePrettierConfigs(
      '/workspace',
      [root],
      () => rootConfig,
      () => ['.prettierrc.yml', 'prettier.base.mjs', 'prettier.config.js'],
    ),
    [
      {
        rule: '5.1.6',
        packageKey: '.',
        message:
          "Prettier configuration file '.prettierrc.yml' is forbidden; use only 'prettier.base.mjs' or 'prettier.config.js'",
      },
    ],
  );

  // Both are required.
  assert.deepEqual(
    validatePrettierConfigs(
      '/workspace',
      [root],
      () => undefined,
      () => [],
    ),
    [
      {
        rule: '5.1.6',
        packageKey: '.',
        message: "workspace root must contain 'prettier.base.mjs'",
      },
      {
        rule: '5.1.6',
        packageKey: '.',
        message: "workspace root must contain 'prettier.config.js' referencing './prettier.base.mjs'",
      },
    ],
  );

  // Present and correct, with a comment above the statement: prose is not a second source of truth.
  assert.deepEqual(
    validatePrettierConfigs(
      '/workspace',
      [root],
      () => `// Mandatory at the root: Prettier does not discover 'prettier.base.mjs'.\n${rootConfig}`,
      () => ['prettier.base.mjs', 'prettier.config.js'],
    ),
    [],
  );

  // A root config that restates options instead of re-exporting the base is a fork of it.
  assert.deepEqual(
    validatePrettierConfigs(
      '/workspace',
      [root],
      () => 'export default { singleQuote: true };\n',
      () => ['prettier.base.mjs', 'prettier.config.js'],
    ),
    [
      {
        rule: '5.1.6',
        packageKey: '.',
        message: "'prettier.config.js' must contain: export { default } from './prettier.base.mjs';",
      },
    ],
  );
});

test("requires root 'eslint.base.mjs' and package-level 'eslint.config.js'", () => {
  const root = loadedPackage(
    '.',
    { kind: 'workspace-root', name: 'workspace', private: true, member: false },
    { name: 'workspace', private: true },
  );
  const valid = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    { scripts: { lint: 'eslint .' } },
  );
  const missing = loadedPackage(
    './e2e',
    { kind: 'internal-consumer', name: '@scope/e2e-dev', private: true, member: true },
    { scripts: { lint: 'eslint .' } },
  );
  const alternate = loadedPackage(
    './library',
    {
      kind: 'dev',
      name: '@scope/library-dev',
      private: true,
      member: true,
      publishedRelPath: './library/pkg',
    },
    { scripts: { lint: 'eslint .' } },
  );
  const commonjs = loadedPackage(
    './cjs-consumer',
    { kind: 'internal-consumer', name: '@scope/cjs-consumer-dev', private: true, member: true },
    { type: 'commonjs', scripts: { lint: 'eslint .' } },
  );
  const published = publishedPackage({}, { lint: 'eslint .' });
  const files = new Map<string, readonly string[]>([
    ['/workspace/', ['eslint.base.mjs', 'eslint.config.js']],
    ['/workspace/common', ['eslint.config.js']],
    ['/workspace/e2e', []],
    ['/workspace/library', ['eslint.config.js', 'eslint.config.mjs', '.eslintrc.json']],
    ['/workspace/library/pkg', ['eslint.config.mjs']],
    ['/workspace/cjs-consumer', ['eslint.config.js']],
  ]);

  assert.deepEqual(
    validateEslintConfigs(
      [root, valid, missing, alternate, commonjs, published],
      (directory) => files.get(directory) ?? [],
    ),
    [
      {
        rule: '5.1.7',
        packageKey: '.',
        message: "ESLint configuration file 'eslint.config.js' is forbidden; use only 'eslint.base.mjs'",
      },
      {
        rule: '5.1.7',
        packageKey: './e2e',
        message: "package with a 'lint' script must contain the exact file 'eslint.config.js'",
      },
      {
        rule: '5.1.7',
        packageKey: './library',
        message: "ESLint configuration file '.eslintrc.json' is forbidden; use only 'eslint.config.js'",
      },
      {
        rule: '5.1.7',
        packageKey: './library',
        message: "ESLint configuration file 'eslint.config.mjs' is forbidden; use only 'eslint.config.js'",
      },
      {
        rule: '5.1.7',
        packageKey: './library/pkg',
        message: "ESLint configuration file 'eslint.config.mjs' is forbidden; use only 'eslint.config.js'",
      },
    ],
  );

  assert.deepEqual(
    validateEslintConfigs([root], () => []),
    [
      {
        rule: '5.1.7',
        packageKey: '.',
        message: "workspace root must contain 'eslint.base.mjs'",
      },
    ],
  );
});

function devOwner(scripts: Readonly<Record<string, string>>) {
  return loadedPackage(
    './library',
    {
      kind: 'dev',
      name: '@scope/library-dev',
      private: true,
      member: true,
      publishedRelPath: './library/pkg',
    },
    { name: '@scope/library-dev', private: true, scripts },
  );
}

function publishedPackage(
  capabilities: {
    readonly vendored?: ReturnType<typeof vendoredCapability>[];
    readonly mirror?: ReturnType<typeof mirrorCapability>;
    readonly distribution?: ('npm' | 'mirror')[];
    readonly consumerTests?: { readonly cjs?: string[]; readonly esm?: string[] };
  },
  scripts?: Readonly<Record<string, string>>,
  packageJson?: Parameters<typeof loadedPackage>[2],
) {
  return loadedPackage(
    './library/pkg',
    { kind: 'published', name: '@scope/library', member: true, ...capabilities },
    { name: '@scope/library', version: '1.0.0', scripts, ...packageJson },
  );
}

function consumerFixture(moduleKind: 'cjs' | 'esm', ownerKey = './library', payloadName = '@scope/library') {
  const fixtureName = `${ownerKey.slice(2).replaceAll('/', '-')}-consumer-${moduleKind}`;
  return loadedPackage(
    `${ownerKey}/test-consumer/${moduleKind}`,
    {
      kind: 'standalone',
      name: fixtureName,
      private: true,
      member: false,
    },
    {
      name: fixtureName,
      private: true,
      type: moduleKind === 'esm' ? 'module' : 'commonjs',
      scripts: { test: 'node ./test.js' },
      devDependencies: { [payloadName]: 'file:../../pkg' },
    },
  );
}

function mirrorCapability() {
  return { repository: 'https://github.com/example/project' } as const;
}

function vendoredCapability() {
  return { relPath: './src/vendored', source: './sdk/source', reason: 'Private source cannot be imported.' } as const;
}
