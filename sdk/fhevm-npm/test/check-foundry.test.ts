import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { inspectFoundry, parseForgeVersion } from '../base/checks/foundry.ts';
import { parseTestNpmManifest } from './helpers.ts';

test('parses the version reported by forge', () => {
  assert.equal(
    parseForgeVersion('forge Version: 1.5.1-stable\nCommit SHA: abc\nBuild Profile: maxperf\n'),
    '1.5.1-stable',
  );
});

test('accepts the centrally pinned Foundry version', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-foundry-'));
  try {
    const inspection = inspectFoundry(workspace, manifest('1.5.1-stable'), () => 'forge Version: 1.5.1-stable\n');
    assert.equal(inspection.actualVersion, '1.5.1-stable');
    assert.deepEqual(inspection.violations, []);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('reports a version mismatch and package-local pins', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-foundry-'));
  try {
    mkdirSync(join(workspace, 'member'), { recursive: true });
    writeFileSync(join(workspace, 'member', '.foundry-version'), '1.4.0-stable\n');
    const inspection = inspectFoundry(workspace, manifest('1.5.1-stable'), () => 'forge Version: 1.4.0-stable\n');
    assert.deepEqual(inspection.violations, [
      {
        rule: '4.1.2',
        packageKey: './member',
        message: "remove '.foundry-version'; the central pin is npm-manifest.json#foundry.version",
      },
      {
        rule: '4.1.2',
        packageKey: '.',
        message:
          "installed forge is '1.4.0-stable'; npm-manifest.json requires '1.5.1-stable' (run 'foundryup --install 1.5.1-stable')",
      },
    ]);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

function manifest(version: string) {
  return parseTestNpmManifest({
    foundry: { version },
    packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
    packages: {
      '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      './member': { kind: 'shared-helper', name: 'member-dev', private: true, member: true },
    },
  });
}
