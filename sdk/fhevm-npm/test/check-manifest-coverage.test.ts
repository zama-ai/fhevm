import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { discoverPackageKeys, validateInventoryPaths, validateInventorySets } from '../base/checks/inventory.ts';
import { parseTestNpmManifest } from './helpers.ts';

test('manifest coverage excludes configured trees and the autonomous validator package', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-inventory-'));
  try {
    execFileSync('git', ['init', '--quiet', workspace]);
    writePackageJson(workspace);
    writePackageJson(join(workspace, 'tracked'));
    writePackageJson(join(workspace, 'deleted'));
    writePackageJson(join(workspace, 'untracked'));
    writePackageJson(join(workspace, 'ignored'));
    writePackageJson(join(workspace, 'standalone'));
    writePackageJson(join(workspace, 'validator'));
    writePackageJson(join(workspace, 'validator', 'fixture'));
    writePackageJson(join(workspace, 'excluded'));
    writePackageJson(join(workspace, 'excluded', 'nested'));
    writeFileSync(join(workspace, '.gitignore'), 'ignored/\nstandalone/\n');
    execFileSync('git', ['-C', workspace, 'add', 'tracked/package.json', 'deleted/package.json']);
    rmSync(join(workspace, 'deleted/package.json'));

    const manifest = parseTestNpmManifest({
      inventory: { exclude: ['./excluded'] },
      packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
      packages: {
        '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
        './standalone': { kind: 'standalone', name: 'consumer', member: false },
      },
    });
    assert.deepEqual(discoverPackageKeys(workspace, manifest, [join(workspace, 'validator')]), [
      '.',
      './standalone',
      './tracked',
      './untracked',
    ]);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('reports both missing and stale inventory entries', () => {
  const manifest = parseTestNpmManifest({
    packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
    packages: {
      '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      './stale': { kind: 'non-package', member: false },
    },
  });
  assert.deepEqual(validateInventorySets(manifest, ['.', './missing']), [
    {
      rule: '7.1.3',
      packageKey: './missing',
      message: 'source package.json is missing from npm-manifest.json',
    },
    {
      rule: '7.1.3',
      packageKey: './stale',
      message: 'manifest entry has no discoverable source package.json',
    },
  ]);
});

test('rejects a manifest package directory symlink escaping the workspace', () => {
  const root = mkdtempSync(join(tmpdir(), 'fhevm-npm-paths-'));
  const workspace = join(root, 'sdk');
  const outside = join(root, 'outside');
  try {
    mkdirSync(workspace);
    mkdirSync(outside);
    writePackageJson(workspace);
    writePackageJson(outside);
    symlinkSync(outside, join(workspace, 'escaped'), 'dir');
    const manifest = parseTestNpmManifest({
      packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
      packages: {
        '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
        './escaped': { kind: 'non-package', member: false },
      },
    });

    const violations = validateInventoryPaths(workspace, root, manifest);
    assert.equal(violations.length, 1);
    assert.equal(violations[0]?.rule, '7.1.4');
    assert.equal(violations[0]?.packageKey, './escaped');
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

function writePackageJson(directory: string): void {
  mkdirSync(directory, { recursive: true });
  writeFileSync(join(directory, 'package.json'), '{}\n');
}
