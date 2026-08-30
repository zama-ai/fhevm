import assert from 'node:assert/strict';
import test from 'node:test';

import {
  prettierTargetsSolidity,
  validateEslintConfigs,
  validatePrettierConfigs,
  validateScripts,
} from '../base/checks/scripts.ts';
import { loadedPackage } from './helpers.ts';

test('accepts conventional scripts on each resolved test owner', () => {
  const owner = devOwner({
    build: 'tsc',
    clean: 'rm -rf ./dist',
    lint: 'eslint .',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    'test:publint': 'publint --strict ./pkg',
    'test:consumer': 'node ./test-consumer.ts',
    'test:vendored': 'node ./check-vendored.ts',
    'test:mirror': 'node ./check-mirror.ts',
  });
  const published = publishedPackage({ vendored: [vendoredCapability()], mirror: mirrorCapability() });
  const standalone = loadedPackage(
    './standalone',
    { kind: 'standalone', name: 'consumer', member: false, mirror: mirrorCapability() },
    { name: 'consumer', scripts: { 'test:mirror': 'node ./check-mirror.ts' } },
  );

  assert.deepEqual(
    validateScripts([owner, published, consumerFixture('cjs'), standalone], () => true),
    [],
  );
});

test('reports missing scripts, unresolved owners, and scripts on the wrong kind', () => {
  const owner = devOwner({});
  const published = publishedPackage({ vendored: [vendoredCapability()] }, { 'test:consumer': 'wrong owner' });
  const orphan = loadedPackage(
    './orphan/pkg',
    { kind: 'published', name: '@scope/orphan', member: false, mirror: mirrorCapability() },
    { name: '@scope/orphan' },
  );
  const orphanConsumer = consumerFixture('cjs', './orphan');

  const violations = validateScripts([owner, published, consumerFixture('cjs'), orphan, orphanConsumer], () => true);
  assert.equal(violations.filter((violation) => violation.rule === '5.1.3').length, 2);
  assert.equal(violations.filter((violation) => violation.rule === '5.2.1').length, 1);
  assert.equal(violations.filter((violation) => violation.rule === '5.3.1').length, 1);
  assert.equal(violations.filter((violation) => violation.rule === '5.3.2').length, 1);
  assert.equal(violations.filter((violation) => violation.rule === 'package-scripts').length, 5);
});

test("requires format-specific directories below 'test-consumer'", () => {
  const owner = devOwner({
    build: 'tsc',
    clean: 'rm -rf ./dist',
    lint: 'eslint .',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    'test:publint': 'publint --strict ./pkg',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage({}, undefined, {
    type: 'module',
    main: './dist/index.cjs',
    module: './dist/index.js',
    exports: { '.': { import: './dist/index.js', require: './dist/index.cjs' } },
  });

  const checkedDirectories: string[] = [];
  const violations = validateScripts([owner, published], (directory) => {
    checkedDirectories.push(directory);
    return false;
  });

  assert.deepEqual(checkedDirectories, [
    '/workspace/library/test-consumer/cjs',
    '/workspace/library/test-consumer/esm',
  ]);
  assert.deepEqual(violations, [
    {
      rule: '5.3.1',
      packageKey: './library/pkg',
      message: "published package exposes CJS but has no sibling './library/test-consumer/cjs' directory",
    },
    {
      rule: '5.3.1',
      packageKey: './library/pkg',
      message: "published package exposes ESM but has no sibling './library/test-consumer/esm' directory",
    },
  ]);
});

test('validates each format-specific consumer fixture', () => {
  const owner = devOwner({
    build: 'tsc',
    clean: 'rm -rf ./dist',
    lint: 'eslint .',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    'test:publint': 'publint --strict ./pkg',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage({}, undefined, {
    type: 'module',
    main: './dist/index.cjs',
    module: './dist/index.js',
    exports: { '.': { import: './dist/index.js', require: './dist/index.cjs' } },
  });
  const invalidCjs = loadedPackage(
    './library/test-consumer/cjs',
    { kind: 'standalone', name: 'library-consumer-cjs', private: true, member: true },
    { name: 'library-consumer-cjs', private: false, type: 'module', scripts: {} },
  );

  const violations = validateScripts([owner, published, invalidCjs, consumerFixture('esm')], () => true);

  assert.deepEqual(
    violations.filter((violation) => violation.packageKey === './library/test-consumer/cjs'),
    [
      {
        rule: '5.3.1',
        packageKey: './library/test-consumer/cjs',
        message: "consumer fixture must be kind 'standalone' with member=false",
      },
      {
        rule: '5.3.1',
        packageKey: './library/test-consumer/cjs',
        message: 'consumer fixture must set private=true',
      },
      {
        rule: '5.3.1',
        packageKey: './library/test-consumer/cjs',
        message: "consumer fixture must define a non-empty 'test' script",
      },
    ],
  );
});

test('requires an existing consumer directory to be registered in the manifest', () => {
  const owner = devOwner({
    build: 'tsc',
    clean: 'rm -rf ./dist',
    lint: 'eslint .',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    'test:publint': 'publint --strict ./pkg',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage({});

  assert.deepEqual(
    validateScripts([owner, published], () => true),
    [
      {
        rule: '5.3.1',
        packageKey: './library/pkg',
        message: "consumer fixture './library/test-consumer/cjs' exists but is not registered in npm-manifest.json",
      },
    ],
  );
});

test("requires serial execution when a consumer fixture uses 'node --test'", () => {
  const owner = devOwner({
    build: 'tsc',
    clean: 'rm -rf ./dist',
    lint: 'eslint .',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    'test:publint': 'publint --strict ./pkg',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage({}, undefined, {
    type: 'module',
    main: './dist/index.cjs',
    module: './dist/index.js',
    exports: { '.': { import: './dist/index.js', require: './dist/index.cjs' } },
  });
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
    validateScripts([owner, published, cjs, esm], () => true),
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
    build: 'tsc',
    clean: 'rm -rf ./dist',
    lint: 'eslint .',
    'prettier:check': 'prettier --check .',
    'prettier:write': 'prettier --write .',
    'test:publint': 'publint --strict ./pkg',
    'test:consumer': 'node ./test-consumer.ts',
  });
  const published = publishedPackage({});
  const fixture = consumerFixture('cjs');

  const violations = validateScripts(
    [owner, published, fixture],
    () => true,
    (pkg) => pkg.key === published.key,
  );

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
    { name: '@scope/e2e-dev', private: true },
  );

  assert.deepEqual(
    validateScripts(
      [internalConsumer],
      () => true,
      () => true,
    ),
    [
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

test('requires hygiene scripts on shared helpers', () => {
  const helper = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    { name: '@scope/common-dev', private: true, scripts: { lint: 'eslint .' } },
  );

  assert.deepEqual(
    validateScripts(
      [helper],
      () => true,
      () => false,
    ),
    [
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

test('rejects Solidity targets in Prettier scripts', () => {
  const internalConsumer = loadedPackage(
    './e2e',
    { kind: 'internal-consumer', name: '@scope/e2e-dev', private: true, member: true },
    {
      name: '@scope/e2e-dev',
      private: true,
      scripts: {
        lint: 'eslint .',
        'prettier:check': 'prettier --check "contracts/**/*.sol"',
        'prettier:write': 'prettier --write "**/*.{js,json,md,sol,ts,yml}"',
      },
    },
  );

  assert.deepEqual(
    validateScripts(
      [internalConsumer],
      () => true,
      () => false,
    ),
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

test('requires non-published packages with Prettier scripts to re-export the workspace config', () => {
  const valid = loadedPackage(
    './hardhat/v2/e2e',
    { kind: 'internal-consumer', name: '@scope/e2e-dev', private: true, member: true },
    { scripts: { 'prettier:check': 'prettier --check .' } },
  );
  const missing = loadedPackage(
    './common',
    { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
    { scripts: { 'prettier:write': 'prettier --write .' } },
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
    { scripts: { 'prettier:check': 'prettier --check .' } },
  );
  const published = publishedPackage({}, { 'prettier:check': 'prettier --check .' });
  const files = new Map([
    ['/workspace/hardhat/v2/e2e/.prettierrc.mjs', "export { default } from '../../../prettier.base.mjs';\n"],
    ['/workspace/library/.prettierrc.mjs', "export { default } from './local-prettier.mjs';\n"],
  ]);

  assert.deepEqual(
    validatePrettierConfigs('/workspace', [valid, missing, incorrect, published], (file) => files.get(file)),
    [
      {
        rule: '5.1.6',
        packageKey: './common',
        message: "package with Prettier scripts must contain '.prettierrc.mjs' re-exporting '../prettier.base.mjs'",
      },
      {
        rule: '5.1.6',
        packageKey: './library',
        message: "'.prettierrc.mjs' must contain: export { default } from '../prettier.base.mjs';",
      },
    ],
  );
});

test("requires one ESLint config, named for the package's module type", () => {
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
  // A CommonJS package cannot `import` the shared .mjs base from a `.js` config, so `.mjs` is the
  // required name there — and `.js` becomes the forbidden alternate.
  const commonjs = loadedPackage(
    './cjs-consumer',
    { kind: 'internal-consumer', name: '@scope/cjs-consumer-dev', private: true, member: true },
    { type: 'commonjs', scripts: { lint: 'eslint .' } },
  );
  const commonjsWrong = loadedPackage(
    './cjs-wrong',
    { kind: 'internal-consumer', name: '@scope/cjs-wrong-dev', private: true, member: true },
    { type: 'commonjs', scripts: { lint: 'eslint .' } },
  );
  const published = publishedPackage({}, { lint: 'eslint .' });
  const files = new Map<string, readonly string[]>([
    ['/workspace/common', ['eslint.config.js']],
    ['/workspace/e2e', []],
    ['/workspace/library', ['eslint.config.js', 'eslint.config.mjs', '.eslintrc.json']],
    ['/workspace/library/pkg', ['eslint.config.mjs']],
    ['/workspace/cjs-consumer', ['eslint.config.mjs']],
    ['/workspace/cjs-wrong', ['eslint.config.js']],
  ]);

  assert.deepEqual(
    validateEslintConfigs(
      [valid, missing, alternate, commonjs, commonjsWrong, published],
      (directory) => files.get(directory) ?? [],
    ),
    [
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
        packageKey: './cjs-wrong',
        message: "package with a 'lint' script must contain the exact file 'eslint.config.mjs'",
      },
      {
        rule: '5.1.7',
        packageKey: './cjs-wrong',
        message: "ESLint configuration file 'eslint.config.js' is forbidden; use only 'eslint.config.mjs'",
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

function consumerFixture(moduleKind: 'cjs' | 'esm', ownerKey = './library') {
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
    },
  );
}

function mirrorCapability() {
  return { repository: 'https://github.com/example/project' } as const;
}

function vendoredCapability() {
  return { relPath: './src/vendored', source: './sdk/source', reason: 'Private source cannot be imported.' } as const;
}
