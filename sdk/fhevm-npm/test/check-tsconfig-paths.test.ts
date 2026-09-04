import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';

import { inspectTsconfigPaths } from '../base/checks/tsconfig-paths.ts';
import { parseTestNpmManifest } from './helpers.ts';

test('checks literal paths in root, member, and non-package tsconfigs', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-tsconfig-paths-'));
  try {
    writeJson(join(workspace, 'tsconfig.json'), {
      include: ['./existing.ts', './optional/**/*.ts'],
      exclude: ['./node_modules'],
      files: ['./missing.ts'],
    });
    writeFileSync(join(workspace, 'existing.ts'), '');
    writeJson(join(workspace, 'member', 'tsconfig.json'), { references: [{ path: '../reference' }] });
    writeJson(join(workspace, 'reference', 'tsconfig.json'), {});
    writeJson(join(workspace, 'scripts', 'tsconfig.build.json'), { extends: './missing-base.json' });
    writeJson(join(workspace, 'standalone', 'tsconfig.json'), { files: ['./missing-but-out-of-scope.ts'] });

    const manifest = parseTestNpmManifest({
      packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
      packages: {
        '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
        './member': { kind: 'shared-helper', name: 'member-dev', private: true, member: true },
        './scripts': { kind: 'non-package', member: false },
        './standalone': { kind: 'standalone', name: 'consumer', member: false },
      },
    });

    const inspection = inspectTsconfigPaths(workspace, manifest);
    assert.deepEqual(inspection.checkedConfigKeys, [
      './member/tsconfig.json',
      './scripts/tsconfig.build.json',
      './tsconfig.json',
    ]);
    assert.deepEqual(inspection.violations, [
      {
        rule: '2.1.5',
        packageKey: './scripts/tsconfig.build.json',
        message: "'extends' path './missing-base.json' does not exist",
      },
      {
        rule: '2.1.5',
        packageKey: './tsconfig.json',
        message: "'files' path './missing.ts' does not exist",
      },
    ]);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

function writeJson(file: string, value: unknown): void {
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`);
}
