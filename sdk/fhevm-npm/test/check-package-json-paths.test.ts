import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { inspectPackageJsonPaths } from '../base/checks/package-json-paths.ts';
import type { LoadedPackage } from '../base/npm.ts';

test('checks exact and wildcard package.json entrypoint paths recursively', () => {
  const directory = mkdtempSync(join(tmpdir(), 'fhevm-npm-package-json-paths-'));
  try {
    mkdirSync(join(directory, 'src'));
    writeFileSync(join(directory, 'index.js'), '');
    const pkg: LoadedPackage = {
      key: './library',
      directory,
      inventory: { kind: 'published', type: 'dual', browser: false, name: '@scope/library', member: true },
      packageJson: {
        main: './index.js',
        module: './missing.mjs',
        exports: {
          '.': { import: './index.js', types: './missing.d.ts' },
          './feature/*': './src/*',
        },
        imports: {
          '#external': 'some-package',
          '#local': ['./missing-local.js', null],
        },
      },
    };

    const inspection = inspectPackageJsonPaths([pkg]);
    assert.deepEqual(inspection.violations, [
      {
        rule: '2.1.6',
        packageKey: './library',
        message: "'module' target './missing.mjs' does not exist",
      },
      {
        rule: '2.1.6',
        packageKey: './library',
        message: `'exports["."]["types"]' target './missing.d.ts' does not exist`,
      },
      {
        rule: '2.1.6',
        packageKey: './library',
        message: `'imports["#local"][0]' target './missing-local.js' does not exist`,
      },
    ]);
    assert.deepEqual(inspection.successfulClaims, [
      './library [main] ./index.js',
      './library [exports["."]["import"]] ./index.js',
      './library [exports["./feature/*"]] ./src/*',
    ]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
