import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import type { LoadedPackage, PackageJson } from '../base/npm.ts';
import {
  buildLinkedDependenciesWithMake,
  findTestConsumerTargets,
  linkedDependencyBuildOrder,
  selectTestConsumerTargets,
} from '../base/test-consumer.ts';
import type { NpmManifestEntry } from '../manifest.ts';

test('discovers an explicit manifest consumer and builds linked payload owners in dependency order', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-explicit-consumer-'));
  try {
    const hostOwner = pkg(
      workspace,
      './host',
      { kind: 'dev', name: '@scope/host-dev', private: true, member: true, publishedRelPath: './host/pkg' },
      { name: '@scope/host-dev', private: true, scripts: { compile: 'compile-host' } },
    );
    const hostPayload = pkg(
      workspace,
      './host/pkg',
      { kind: 'published', name: '@scope/host', member: false },
      { name: '@scope/host', version: '1.0.0' },
    );
    const pluginOwner = pkg(
      workspace,
      './plugin',
      {
        kind: 'dev',
        name: '@scope/plugin-dev',
        private: true,
        member: true,
        publishedRelPath: './plugin/pkg',
      },
      { name: '@scope/plugin-dev', private: true, scripts: { compile: 'compile-plugin' } },
    );
    const pluginPayload = pkg(
      workspace,
      './plugin/pkg',
      { kind: 'published', name: '@scope/plugin', member: false },
      { name: '@scope/plugin', version: '1.0.0', dependencies: { '@scope/host': '1.0.0' } },
    );
    const consumer = pkg(
      workspace,
      './consumer',
      { kind: 'standalone', name: 'consumer', private: true, member: false },
      {
        name: 'consumer',
        private: true,
        type: 'commonjs',
        scripts: { test: 'node --test' },
        devDependencies: {
          '@scope/plugin': 'file:../plugin/pkg',
        },
      },
    );
    const packages = [hostOwner, hostPayload, pluginOwner, pluginPayload, consumer];

    const targets = findTestConsumerTargets(workspace, packages);
    assert.equal(targets.length, 1);
    const selected = selectTestConsumerTargets(targets, 'consumer');
    assert.deepEqual(
      selected.map((target) => target.source.key),
      ['./consumer'],
    );
    assert.deepEqual(
      selected[0]?.linkedDependencies.map((dependency) => dependency.package.key),
      ['./host/pkg', './plugin/pkg'],
    );
    assert.deepEqual(
      selected[0]?.linkedDependencies.map((dependency) => ({
        package: dependency.package.key,
        direct: dependency.direct,
        declaredBy: dependency.declaredBy.key,
      })),
      [
        { package: './host/pkg', direct: false, declaredBy: './plugin/pkg' },
        { package: './plugin/pkg', direct: true, declaredBy: './consumer' },
      ],
    );
    assert.deepEqual(
      linkedDependencyBuildOrder(
        packages,
        selected.flatMap((target) => target.linkedDependencies),
      ).map((owner) => owner.key),
      ['./host', './plugin'],
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('delegates linked dependency builds to Make serially with numbered progress', () => {
  const host = {
    key: './host',
    packageJson: { name: '@scope/host-dev' },
  } as LoadedPackage;
  const plugin = {
    key: './plugin',
    packageJson: { name: '@scope/plugin-dev' },
  } as LoadedPackage;
  const calls: Array<{ workspaceRoot: string; packageKey: string; verbosity: number }> = [];
  const messages: string[] = [];

  buildLinkedDependenciesWithMake(
    '/workspace',
    [host, plugin],
    2,
    (workspaceRoot, packageKey, verbosity = 0) => calls.push({ workspaceRoot, packageKey, verbosity }),
    (message) => messages.push(message),
  );

  assert.deepEqual(calls, [
    { workspaceRoot: '/workspace', packageKey: './host', verbosity: 2 },
    { workspaceRoot: '/workspace', packageKey: './plugin', verbosity: 2 },
  ]);
  assert.deepEqual(messages, [
    '\n🎃 Building linked dependencies once before the serial consumer runs.\n',
    '  - ./host (@scope/host-dev)',
    '  - ./plugin (@scope/plugin-dev)',
    '',
    '🚀 Building linked dependency 1/2: ./host (@scope/host-dev)',
    '🚀 Building linked dependency 2/2: ./plugin (@scope/plugin-dev)',
  ]);
});

test('rejects a linked dependency whose declared name differs from the manifest package', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-explicit-consumer-name-'));
  try {
    const payload = pkg(
      workspace,
      './library/pkg',
      { kind: 'published', name: '@scope/library', member: false },
      { name: '@scope/library', version: '1.0.0' },
    );
    const consumer = pkg(
      workspace,
      './consumer',
      { kind: 'standalone', name: 'consumer', private: true, member: false },
      {
        name: 'consumer',
        private: true,
        scripts: { test: 'node --test' },
        dependencies: { '@scope/wrong-name': 'file:../library/pkg' },
      },
    );
    assert.throws(
      () => findTestConsumerTargets(workspace, [payload, consumer]),
      /declares '@scope\/wrong-name'.*resolves to package '@scope\/library'/,
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

function pkg(
  workspace: string,
  key: string,
  inventory: Omit<NpmManifestEntry, 'type' | 'browser'>,
  packageJson: PackageJson,
): LoadedPackage {
  const directory = join(workspace, key.slice(2));
  mkdirSync(directory, { recursive: true });
  writeFileSync(join(directory, 'package.json'), `${JSON.stringify(packageJson, null, 2)}\n`);
  return {
    key,
    directory,
    inventory: { type: packageJson.type === 'module' ? 'esm' : 'cjs', browser: false, ...inventory },
    packageJson,
  };
}
