import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';

import { inspectTscMode } from '../base/checks/tsc-mode.ts';
import { parseTestNpmManifest } from './helpers.ts';

const SOLUTION = { files: [], references: [{ path: './src' }] };
const LEAF = { include: ['./**/*.ts'] };

test('flags project-mode and bare tsc invocations that target a solution-style tsconfig', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-tsc-mode-'));
  try {
    writeJson(join(workspace, 'package.json'), { name: 'workspace', private: true });
    writeJson(join(workspace, 'app', 'package.json'), {
      name: 'app-dev',
      private: true,
      scripts: {
        lint: 'eslint && tsc -p ./tsconfig.json --noEmit',
        'lint:bare': 'tsc --noEmit',
        'lint:build': 'tsc -b ./tsconfig.json --noEmit',
        'lint:leaf': 'tsc --project ./scripts/tsconfig.json --noEmit',
      },
    });
    writeJson(join(workspace, 'app', 'tsconfig.json'), SOLUTION);
    writeJson(join(workspace, 'app', 'scripts', 'tsconfig.json'), LEAF);

    const inspection = inspectTscMode(workspace, testManifest());
    assert.deepEqual(inspection.violations, [
      {
        rule: '2.1.13',
        packageKey: './app',
        message:
          "'lint' runs 'tsc -p ./tsconfig.json --noEmit', which is in project mode; that tsconfig is " +
          "solution-style (empty 'files' plus 'references'), so project mode checks zero files and " +
          "exits 0 — use 'tsc -b'",
      },
      {
        rule: '2.1.13',
        packageKey: './app',
        message:
          "'lint:bare' runs 'tsc --noEmit', which resolves to './tsconfig.json'; that tsconfig is " +
          "solution-style (empty 'files' plus 'references'), so project mode checks zero files and " +
          "exits 0 — use 'tsc -b'",
      },
    ]);
    assert.equal(inspection.checkedInvocationKeys.length, 4);
    assert.deepEqual(inspection.successfulInvocations, [
      './app [lint:build] tsc -b ./tsconfig.json --noEmit (build mode)',
      './app [lint:leaf] tsc --project ./scripts/tsconfig.json --noEmit (project mode)',
    ]);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('skips invocations whose tsconfig cannot silently pass', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-tsc-mode-'));
  try {
    writeJson(join(workspace, 'package.json'), { name: 'workspace', private: true });
    writeJson(join(workspace, 'app', 'package.json'), {
      name: 'app-dev',
      private: true,
      scripts: {
        build: 'tsc --project ./tsconfig.missing.json --outDir ./dist',
        'build:files': 'tsc ./main.ts --noEmit',
        'lint:directory': 'tsc -p ./project --noEmit',
      },
    });
    writeJson(join(workspace, 'app', 'project', 'tsconfig.json'), SOLUTION);

    const inspection = inspectTscMode(workspace, testManifest());
    assert.deepEqual(inspection.violations, [
      {
        rule: '2.1.13',
        packageKey: './app',
        message:
          "'lint:directory' runs 'tsc -p ./project --noEmit', which is in project mode; that tsconfig " +
          "is solution-style (empty 'files' plus 'references'), so project mode checks zero files and " +
          "exits 0 — use 'tsc -b'",
      },
    ]);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('refuses to pass vacuously when no tsc invocation exists', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-tsc-mode-'));
  try {
    writeJson(join(workspace, 'package.json'), { name: 'workspace', private: true });
    writeJson(join(workspace, 'app', 'package.json'), { name: 'app-dev', private: true, scripts: { lint: 'eslint' } });

    const inspection = inspectTscMode(workspace, testManifest());
    assert.deepEqual(inspection.violations, [
      {
        rule: '2.1.13',
        packageKey: '.',
        message: 'found no tsc invocation to inspect; refusing to pass vacuously',
      },
    ]);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

function testManifest() {
  return parseTestNpmManifest({
    packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
    packages: {
      '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      './app': { kind: 'shared-helper', name: 'app-dev', private: true, member: true },
    },
  });
}

function writeJson(file: string, value: unknown): void {
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`);
}
