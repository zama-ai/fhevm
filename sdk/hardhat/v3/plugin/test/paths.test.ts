// Sibling-module resolution against consumer trees built in a temp directory: an npm layout (flat
// node_modules), a pnpm layout (symlink into the nested store) and a tree missing a sibling. The
// temp root has no node_modules above it, so the plugin's own dependencies cannot leak in.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, realpathSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';

import { HardhatPluginError } from 'hardhat/plugins';

import { FhevmPaths, resolveFromConsumer } from '../pkg/_esm/internal/paths.js';

function writePackage(dir: string, name: string): void {
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, 'package.json'), JSON.stringify({ name, version: '0.0.0' }));
}

function makeRoot(): string {
  return realpathSync(mkdtempSync(join(tmpdir(), 'fhevm-paths-')));
}

void test('npm layout: siblings resolve to the flat node_modules', (t) => {
  const root = makeRoot();
  t.after(() => {
    rmSync(root, { recursive: true, force: true });
  });
  writePackage(join(root, 'node_modules/@fhevm/solidity'), '@fhevm/solidity');
  mkdirSync(join(root, 'node_modules/@fhevm/solidity/config'));
  writeFileSync(join(root, 'node_modules/@fhevm/solidity/config/ZamaConfig.sol'), '');
  writePackage(join(root, 'node_modules/@fhevm/sdk'), '@fhevm/sdk');

  const paths = new FhevmPaths(root);
  assert.equal(paths.root, root);
  assert.equal(paths.nodeModulesDir, join(root, 'node_modules'));
  assert.equal(paths.fhevmSolidityDir, join(root, 'node_modules/@fhevm/solidity'));
  assert.equal(paths.fhevmSolidityConfigFile, join(root, 'node_modules/@fhevm/solidity/config/ZamaConfig.sol'));
  assert.equal(paths.fhevmSdkDir, join(root, 'node_modules/@fhevm/sdk'));
});

void test('pnpm layout: a symlinked sibling resolves to its real nested-store path', (t) => {
  const root = makeRoot();
  t.after(() => {
    rmSync(root, { recursive: true, force: true });
  });
  const store = join(root, 'node_modules/.pnpm/@fhevm+sdk@0.13.3/node_modules/@fhevm/sdk');
  writePackage(store, '@fhevm/sdk');
  mkdirSync(join(root, 'node_modules/@fhevm'), { recursive: true });
  symlinkSync(store, join(root, 'node_modules/@fhevm/sdk'), 'dir');

  assert.equal(new FhevmPaths(root).fhevmSdkDir, store);
});

void test('a missing sibling is a named plugin error, and the plugin tree does not leak in', (t) => {
  const root = makeRoot();
  t.after(() => {
    rmSync(root, { recursive: true, force: true });
  });
  // The test process itself resolves @fhevm/sdk; the consumer root must not.
  assert.equal(dirname(resolveFromConsumer('@fhevm/sdk/package.json', process.cwd())).includes('node_modules'), true);

  assert.throws(
    () => new FhevmPaths(root).fhevmSdkDir,
    (error: unknown) =>
      HardhatPluginError.isHardhatPluginError(error) &&
      error.pluginId === 'fhevm' &&
      error.message.includes(`Unable to resolve '@fhevm/sdk/package.json' from the project at ${root}`),
  );
  assert.equal(new FhevmPaths(root).nodeModulesDir, join(root, 'node_modules'), 'pure paths never resolve');
});
