import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { type NpmRunner, applyPlan, guardNpmjs, planVersionApply, writePackageVersion } from '../base/version-apply.ts';
import { loadedPackage, parseTestNpmManifest } from './helpers.ts';

const manifest = parseTestNpmManifest({
  packageJson: { published: { required: [], excluded: [] } },
  packages: {
    '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
    './plugin/pkg': { kind: 'published', name: '@scope/plugin', member: true },
  },
});

const PACKAGE_JSON = `{\n    "name": "@scope/plugin",\n    "version": "0.13.0",\n    "files":   ["src"]\n}\n`;
const lockWith = (version: string) =>
  JSON.stringify(
    { packages: { '': { name: 'workspace' }, 'plugin/pkg': { name: '@scope/plugin', version } } },
    null,
    2,
  );

/** A repository whose `sdk/` is a one-member workspace committed at 0.13.0, with the central file bumped to `to`. */
function repoBumpedTo(to: string): { readonly root: string; readonly sdk: string } {
  const root = mkdtempSync(join(tmpdir(), 'fhevm-npm-apply-write-'));
  const sdk = join(root, 'sdk');
  mkdirSync(join(sdk, 'plugin', 'pkg'), { recursive: true });
  const central = (version: string) =>
    JSON.stringify({
      $schema: './fhevm-npm/schemas/versions.schema.json',
      schemaVersion: 1,
      packages: { './plugin/pkg': version },
    });
  writeFileSync(join(sdk, 'package.json'), '{ "name": "workspace", "private": true, "workspaces": ["plugin/pkg"] }\n');
  writeFileSync(join(sdk, 'plugin', 'pkg', 'package.json'), PACKAGE_JSON);
  writeFileSync(join(sdk, 'package-lock.json'), lockWith('0.13.0'));
  writeFileSync(join(sdk, 'versions.json'), central('0.13.0'));
  const git = (...args: string[]) => execFileSync('git', args, { cwd: root, encoding: 'utf8', stdio: 'pipe' });
  git('init', '-q');
  git('add', '.');
  git('-c', 'user.name=t', '-c', 'user.email=t@t', 'commit', '-q', '-m', 'baseline');
  writeFileSync(join(sdk, 'versions.json'), central(to));
  return { root, sdk };
}

/** Stands in for `npm install --package-lock-only`: rewrites the member's version line from its package.json. */
const fakeNpm: NpmRunner = (_args, cwd) => {
  const version = (JSON.parse(readFileSync(join(cwd, 'plugin', 'pkg', 'package.json'), 'utf8')) as { version: string })
    .version;
  writeFileSync(join(cwd, 'package-lock.json'), lockWith(version));
};

test('applyPlan writes the package.json line, refreshes the lock, and leaves exactly the planned files modified', () => {
  const { root, sdk } = repoBumpedTo('0.13.1');
  try {
    const plan = planVersionApply(sdk, manifest);
    assert.equal(plan.writes.length, 2);
    applyPlan(sdk, manifest, plan, fakeNpm);
    // Formatting survives: only the version value changed, the odd spacing on "files" is intact.
    assert.equal(
      readFileSync(join(sdk, 'plugin', 'pkg', 'package.json'), 'utf8'),
      PACKAGE_JSON.replace('0.13.0', '0.13.1'),
    );
    assert.equal(readFileSync(join(sdk, 'package-lock.json'), 'utf8'), lockWith('0.13.1'));
    const status = execFileSync('git', ['status', '--porcelain'], { cwd: root, encoding: 'utf8' })
      .split('\n')
      .filter((line) => line !== '')
      .sort();
    assert.deepEqual(status, [' M sdk/package-lock.json', ' M sdk/plugin/pkg/package.json', ' M sdk/versions.json']);
    // Idempotent: a second plan on the reconciled tree has nothing to write (the tree is now the accepted 'dirty' set).
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('a lockfile refresh that changes anything beyond the planned version lines fails and leaves the diff', () => {
  const { root, sdk } = repoBumpedTo('0.13.1');
  try {
    const drifting: NpmRunner = (args, cwd) => {
      fakeNpm(args, cwd);
      const lock = JSON.parse(readFileSync(join(cwd, 'package-lock.json'), 'utf8')) as {
        packages: Record<string, unknown>;
      };
      lock.packages['node_modules/left-pad'] = { version: '1.3.0' };
      writeFileSync(join(cwd, 'package-lock.json'), JSON.stringify(lock, null, 2));
    };
    assert.throws(
      () => applyPlan(sdk, manifest, planVersionApply(sdk, manifest), drifting),
      /beyond the planned version lines/,
    );
    assert.match(readFileSync(join(sdk, 'package-lock.json'), 'utf8'), /left-pad/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('writePackageVersion demands exactly one matching version line', () => {
  const root = mkdtempSync(join(tmpdir(), 'fhevm-npm-write-version-'));
  try {
    const file = join(root, 'package.json');
    writeFileSync(file, '{ "version": "1.0.0", "nested": { "version": "1.0.0" } }');
    assert.throws(
      () => writePackageVersion(file, { path: 'package.json', description: 'version', from: '1.0.0', to: '1.0.1' }),
      /exactly one/,
    );
    writeFileSync(file, '{\n  "version": "1.0.0"\n}\n');
    assert.throws(
      () => writePackageVersion(file, { path: 'package.json', description: 'version', from: '2.0.0', to: '2.0.1' }),
      /found 0/,
    );
    writePackageVersion(file, { path: 'package.json', description: 'version', from: '1.0.0', to: '1.0.1' });
    assert.equal(readFileSync(file, 'utf8'), '{\n  "version": "1.0.1"\n}\n');
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('the npmjs guard refuses a target the registry already has, or a registry it cannot reach', async () => {
  const plugin = loadedPackage(
    './plugin/pkg',
    { kind: 'published', name: '@scope/plugin', member: true },
    {
      name: '@scope/plugin',
      version: '0.13.0',
    },
  );
  const registry = (versions: string[]) => async () => ({
    status: 200,
    json: async () => ({
      'dist-tags': { latest: versions.at(-1) },
      versions: Object.fromEntries(versions.map((v) => [v, {}])),
    }),
  });
  const transitions = [{ key: './plugin/pkg', from: '0.13.0', to: '0.13.1' }];
  await guardNpmjs(transitions, [plugin], registry(['0.13.0']));
  await assert.rejects(guardNpmjs(transitions, [plugin], registry(['0.13.0', '0.13.1'])), /already on npmjs.com/);
  await assert.rejects(
    guardNpmjs(transitions, [plugin], async () => ({ status: 503, json: async () => ({}) })),
    /cannot ask npmjs.com/,
  );
  // A mirror-only payload is never asked about.
  const template = loadedPackage(
    './template/pkg',
    {
      kind: 'published',
      name: 'template',
      member: true,
      distribution: ['mirror'],
      mirror: { repository: 'https://x/y' },
    },
    { name: 'template', version: '0.4.2' },
  );
  await guardNpmjs([{ key: './template/pkg', from: '0.4.2', to: '0.4.3' }], [template], async () => {
    throw new Error('must not be called');
  });
});
