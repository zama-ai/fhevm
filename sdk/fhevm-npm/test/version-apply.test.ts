import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import {
  type GitRunner,
  centralDiffAgainstHead,
  classifyWorktree,
  formatPlan,
  planDerivedWrites,
} from '../base/version-apply.ts';
import type { InstallationLock } from '../base/version-check.ts';
import type { VersionsFile } from '../base/versions.ts';
import { loadedPackage } from './helpers.ts';

const versions = (packages: Record<string, string>): VersionsFile => ({
  $schema: './fhevm-npm/schemas/versions.schema.json',
  schemaVersion: 1,
  packages,
});

/** A throwaway repository whose `sdk/` holds the central file, committed at the given versions. */
function repoWithCentral(committed: Record<string, string>): { readonly root: string; readonly workspace: string } {
  const root = mkdtempSync(join(tmpdir(), 'fhevm-npm-version-apply-'));
  const workspace = join(root, 'sdk');
  mkdirSync(workspace);
  const git = (...args: string[]) => execFileSync('git', args, { cwd: root, encoding: 'utf8', stdio: 'pipe' });
  git('init', '-q');
  writeFileSync(join(workspace, 'versions.json'), JSON.stringify(versions(committed), null, 2));
  git('add', '.');
  git('-c', 'user.name=t', '-c', 'user.email=t@t', 'commit', '-q', '-m', 'central');
  return { root, workspace };
}

test('the worktree is clean, carries only the central edit, or is dirty with the offending paths', () => {
  const { root, workspace } = repoWithCentral({ './plugin/pkg': '0.13.0' });
  try {
    assert.deepEqual(classifyWorktree(workspace), { kind: 'clean' });
    writeFileSync(join(workspace, 'versions.json'), JSON.stringify(versions({ './plugin/pkg': '0.13.1' }), null, 2));
    assert.deepEqual(classifyWorktree(workspace), { kind: 'central-edit' });
    writeFileSync(join(workspace, 'stray.txt'), 'x');
    assert.deepEqual(classifyWorktree(workspace), { kind: 'dirty', paths: ['sdk/versions.json', 'sdk/stray.txt'] });
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('a staged central edit is dirty too: the operation wants the unstaged file it can diff against HEAD', () => {
  const { root, workspace } = repoWithCentral({ './plugin/pkg': '0.13.0' });
  try {
    writeFileSync(join(workspace, 'versions.json'), JSON.stringify(versions({ './plugin/pkg': '0.13.1' }), null, 2));
    execFileSync('git', ['add', '.'], { cwd: root });
    assert.equal(classifyWorktree(workspace).kind, 'dirty');
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('the central diff lists forward moves and new entries, and refuses a version that does not increase', () => {
  const { root, workspace } = repoWithCentral({ './plugin/pkg': '0.13.0', './library/pkg': '1.0.0' });
  try {
    const edited = versions({ './plugin/pkg': '0.13.1', './library/pkg': '1.0.0', './new/pkg': '0.1.0' });
    assert.deepEqual(centralDiffAgainstHead(workspace, edited), [
      { key: './plugin/pkg', from: '0.13.0', to: '0.13.1' },
      { key: './new/pkg', to: '0.1.0' },
    ]);
    const prerelease = versions({ './plugin/pkg': '0.14.0-alpha.0', './library/pkg': '1.0.0' });
    assert.equal(centralDiffAgainstHead(workspace, prerelease)[0]?.to, '0.14.0-alpha.0');
    const backwards = versions({ './plugin/pkg': '0.12.9', './library/pkg': '1.0.0' });
    assert.throws(() => centralDiffAgainstHead(workspace, backwards), /only moves forward/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('the plan names every derived field that disagrees: package.json versions and lockfile member lines', () => {
  const plugin = loadedPackage(
    './cluster/plugin/pkg',
    { kind: 'published', name: '@scope/plugin', member: true, memberOf: './cluster' },
    { name: '@scope/plugin', version: '0.13.0' },
  );
  const library = loadedPackage(
    './library/pkg',
    { kind: 'published', name: '@scope/library', member: true },
    {
      name: '@scope/library',
      version: '1.0.0',
    },
  );
  const locks: InstallationLock[] = [
    { rootKey: '.', rootDirectory: '/workspace', entries: { 'library/pkg': { version: '1.0.0' } } },
    {
      rootKey: './cluster',
      rootDirectory: '/workspace/cluster',
      entries: { 'plugin/pkg': { version: '0.13.0' }, '../library/pkg': { version: '1.0.0' } },
    },
  ];
  const central = versions({ './library/pkg': '1.0.0', './cluster/plugin/pkg': '0.13.1' });
  const writes = planDerivedWrites('/workspace', [plugin, library], central, locks);
  assert.deepEqual(writes, [
    { path: 'cluster/plugin/pkg/package.json', description: 'version', from: '0.13.0', to: '0.13.1' },
    { path: 'cluster/package-lock.json', description: 'plugin/pkg version', from: '0.13.0', to: '0.13.1' },
  ]);
  // The library agrees everywhere, so a plan for it alone is empty and says so.
  assert.deepEqual(planDerivedWrites('/workspace', [library], central, locks), []);

  const text = formatPlan({ transitions: [{ key: './cluster/plugin/pkg', from: '0.13.0', to: '0.13.1' }], writes }, [
    plugin,
  ]);
  assert.equal(
    text,
    [
      'central edit',
      '  @scope/plugin  0.13.0 → 0.13.1',
      'would reconcile',
      '  cluster/plugin/pkg/package.json  version 0.13.0 → 0.13.1',
      '  cluster/package-lock.json  plugin/pkg version 0.13.0 → 0.13.1',
    ].join('\n'),
  );
  assert.equal(
    formatPlan({ transitions: [], writes: [] }, []),
    'no central edit\nnothing to reconcile: derived files already agree',
  );
});

test('an injected git runner is honored, so the classifier is testable without a repository', () => {
  const git: GitRunner = (args) => (args[0] === 'rev-parse' ? 'sdk/\n' : ' M sdk/versions.json\n');
  assert.deepEqual(classifyWorktree('/anywhere', git), { kind: 'central-edit' });
});
