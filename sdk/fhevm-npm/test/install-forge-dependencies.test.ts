import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
  type ForgeCommandRunner,
  type ForgeDependencyProject,
  discoverForgeDependencyProjects,
  selectForgeDependencyProjects,
} from '../base/forge-dependencies.ts';
import type { NpmManifest } from '../manifest.ts';

test('discovers manifest packages with effective Forge dependencies', () => {
  const workspaceRoot = mkdtempSync(join(tmpdir(), 'fhevm-npm-forge-dependencies-'));
  try {
    createPackage(workspaceRoot, 'with-dependencies', 'with-dependencies-dev', true);
    createPackage(workspaceRoot, 'without-dependencies', 'without-dependencies-dev', true);
    createPackage(workspaceRoot, 'without-foundry', 'without-foundry-dev', false);

    const manifest = manifestFor(['with-dependencies', 'without-dependencies', 'without-foundry']);
    const runner: ForgeCommandRunner = {
      readConfig(directory) {
        return {
          dependencies: directory.endsWith('/with-dependencies') ? { 'forge-std': '1.11.0' } : {},
        };
      },
      install() {
        throw new Error('install must not run during discovery');
      },
    };

    const projects = discoverForgeDependencyProjects(workspaceRoot, manifest, runner);
    assert.deepEqual(
      projects.map(({ packageKey, dependencies }) => ({ packageKey, dependencies })),
      [{ packageKey: './with-dependencies', dependencies: { 'forge-std': '1.11.0' } }],
    );
  } finally {
    rmSync(workspaceRoot, { recursive: true, force: true });
  }
});

test('selects a Forge dependency project by manifest path or package name', () => {
  const projects: readonly ForgeDependencyProject[] = [
    {
      packageKey: './first',
      packageName: 'first-dev',
      directory: '/workspace/first',
      dependencies: { 'forge-std': '1.11.0' },
    },
    {
      packageKey: './second',
      packageName: 'second-dev',
      directory: '/workspace/second',
      dependencies: { library: '2.0.0' },
    },
  ];

  assert.equal(selectForgeDependencyProjects(projects, './first')[0]?.packageKey, './first');
  assert.equal(selectForgeDependencyProjects(projects, 'second-dev')[0]?.packageKey, './second');
  assert.deepEqual(selectForgeDependencyProjects(projects), projects);
  assert.throws(() => selectForgeDependencyProjects(projects, './missing'), /No manifest package/);
});

function createPackage(workspaceRoot: string, key: string, name: string, foundry: boolean): void {
  const directory = join(workspaceRoot, key);
  mkdirSync(directory, { recursive: true });
  writeFileSync(join(directory, 'package.json'), `${JSON.stringify({ name, private: true })}\n`);
  if (foundry) writeFileSync(join(directory, 'foundry.toml'), '[profile.default]\n');
}

function manifestFor(keys: readonly string[]): NpmManifest {
  return {
    packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
    packages: Object.fromEntries(
      keys.map((key) => [`./${key}`, { kind: 'shared-helper', name: `${key}-dev`, private: true, member: true }]),
    ),
  } as NpmManifest;
}
