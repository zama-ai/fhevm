import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import type { LoadedPackage, PackageJson } from '../base/npm.ts';
import {
  buildLinkedDependenciesWithMake,
  destinationDirectoryName,
  linkedDependencyBuildOrder,
  resolveTestConsumerTargets,
  selectTargetsToRun,
  selectTestConsumerTargets,
} from '../base/test-consumer.ts';
import type { NpmManifestEntry } from '../manifest.ts';

test('resolves a registered consumer and builds linked payload owners in dependency order', () => {
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
      { kind: 'published', name: '@scope/plugin', member: false, consumerTests: { cjs: ['./consumer'] } },
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

    const targets = resolveTestConsumerTargets(workspace, packages);
    assert.equal(targets.length, 1);
    assert.deepEqual(
      targets[0]?.registrations.map((registration) => ({
        published: registration.published.key,
        owner: registration.owner.key,
        moduleKind: registration.moduleKind,
      })),
      [{ published: './plugin/pkg', owner: './plugin', moduleKind: 'cjs' }],
    );
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
    const owner = pkg(
      workspace,
      './library',
      { kind: 'dev', name: '@scope/library-dev', private: true, member: true, publishedRelPath: './library/pkg' },
      { name: '@scope/library-dev', private: true, scripts: { compile: 'compile' } },
    );
    const payload = pkg(
      workspace,
      './library/pkg',
      { kind: 'published', name: '@scope/library', member: false, consumerTests: { cjs: ['./consumer'] } },
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
      () => resolveTestConsumerTargets(workspace, [owner, payload, consumer]),
      /declares '@scope\/wrong-name'.*resolves to package '@scope\/library'/,
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('lists only registered consumers: neither a sibling fixture directory nor a linking project registers itself', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-registry-only-'));
  try {
    const { owner, payload } = payloadWithOwner(workspace, './library', '@scope/library', {
      esm: ['./library/test-consumer/esm'],
    });
    const registered = consumer(workspace, './library/test-consumer/esm', 'module', '@scope/library', '../../pkg');
    // Conventional location, valid tests, correct link — and unregistered, so NOT a consumer test.
    const sibling = consumer(workspace, './library/test-consumer/cjs', 'commonjs', '@scope/library', '../../pkg');
    // A dev-style project with a `test` script and a `file:` dependency: what generic discovery used to invent.
    const project = consumer(workspace, './library/e2e', 'module', '@scope/library', '../pkg');
    const packages = [owner, payload, registered, sibling, project];

    const targets = resolveTestConsumerTargets(workspace, packages);
    assert.deepEqual(
      targets.map((target) => target.source.key),
      ['./library/test-consumer/esm'],
    );

    // Direct selection of either unregistered package names the registration it lacks.
    for (const key of ['./library/test-consumer/cjs', './library/e2e', 'library/e2e']) {
      assert.throws(
        () => selectTestConsumerTargets(targets, key, packages),
        /is a manifest package but no payload registers it as a consumer test; add it to the 'cjs' or 'esm' array/,
      );
    }
    assert.throws(() => selectTestConsumerTargets(targets, './nowhere', packages), /No test consumer matches/);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('owner and payload selectors run every registered suite, including two of one format, in source order', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-two-esm-'));
  try {
    const { owner, payload } = payloadWithOwner(workspace, './plugin', '@scope/plugin', {
      esm: ['./plugin/test-consumer/esm', './template/pkg'],
    });
    const fixture = consumer(workspace, './plugin/test-consumer/esm', 'module', '@scope/plugin', '../../pkg');
    const template = consumer(workspace, './template/pkg', 'module', '@scope/plugin', '../../plugin/pkg', {
      kind: 'internal-consumer',
      member: true,
    });
    const templateOwner = pkg(
      workspace,
      './template',
      { kind: 'internal-consumer', name: '@scope/template-dev', private: true, member: true },
      { name: '@scope/template-dev', private: true },
    );
    const packages = [owner, payload, fixture, template, templateOwner];
    const targets = resolveTestConsumerTargets(workspace, packages);

    const expected = ['./plugin/test-consumer/esm', './template/pkg'];
    for (const selector of ['./plugin', 'plugin/', '@scope/plugin-dev', './plugin/pkg', '@scope/plugin']) {
      assert.deepEqual(
        selectTestConsumerTargets(targets, selector, packages).map((target) => target.source.key),
        expected,
        selector,
      );
    }
    // The template's own directory owner registers nothing, so it selects nothing: owner comes from the
    // registration, not from the path.
    assert.throws(
      () => selectTestConsumerTargets(targets, './template', packages),
      /no payload registers it as a consumer test/,
    );
    assert.deepEqual(
      selectTestConsumerTargets(targets, './template/pkg', packages).map((target) => target.source.key),
      ['./template/pkg'],
    );

    // Two ESM suites must not share a destination.
    const names = targets.map((target) => destinationDirectoryName(target.source.key));
    assert.deepEqual(names, ['plugin-test-consumer-esm', 'template-pkg']);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('a consumer registered by several payloads is one target that keeps every association', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-shared-consumer-'));
  try {
    const host = payloadWithOwner(workspace, './host', '@scope/host', { cjs: ['./shared'] });
    const plugin = payloadWithOwner(workspace, './plugin', '@scope/plugin', { cjs: ['./shared'] });
    const shared = pkg(
      workspace,
      './shared',
      { kind: 'standalone', name: 'shared', private: true, member: false },
      {
        name: 'shared',
        private: true,
        type: 'commonjs',
        scripts: { test: 'node --test' },
        devDependencies: { '@scope/host': 'file:../host/pkg', '@scope/plugin': 'file:../plugin/pkg' },
      },
    );
    const packages = [host.owner, host.payload, plugin.owner, plugin.payload, shared];
    const targets = resolveTestConsumerTargets(workspace, packages);

    assert.equal(targets.length, 1);
    assert.deepEqual(
      targets[0]?.registrations.map((registration) => registration.published.key),
      ['./host/pkg', './plugin/pkg'],
    );
    for (const selector of ['./host', './plugin', './shared', 'shared']) {
      assert.equal(selectTestConsumerTargets(targets, selector, packages).length, 1, selector);
    }
    // A selector that reaches both owners at once IS ambiguous.
    const ambiguousPackages = packages.map((entry) =>
      entry.inventory.kind === 'published'
        ? { ...entry, packageJson: { ...entry.packageJson, name: '@scope/same' } }
        : entry,
    );
    // Rebuild with a shared published NAME while keeping distinct keys and links intact.
    const renamedShared = {
      ...shared,
      packageJson: {
        ...shared.packageJson,
        devDependencies: { '@scope/same': 'file:../host/pkg' },
      },
    };
    const renamedTargets = resolveTestConsumerTargets(
      workspace,
      ambiguousPackages.map((entry) => (entry.key === './shared' ? renamedShared : entry)),
    );
    assert.throws(
      () => selectTestConsumerTargets(renamedTargets, '@scope/same'),
      /ambiguous; use an owner or consumer path/,
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test("'--all' runs every registered consumer in source order and is exclusive with a selector", () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-run-all-'));
  try {
    const { owner, payload } = payloadWithOwner(workspace, './plugin', '@scope/plugin', {
      esm: ['./template/pkg', './plugin/test-consumer/esm'],
    });
    const fixture = consumer(workspace, './plugin/test-consumer/esm', 'module', '@scope/plugin', '../../pkg');
    const template = consumer(workspace, './template/pkg', 'module', '@scope/plugin', '../../plugin/pkg', {
      kind: 'internal-consumer',
      member: true,
    });
    const packages = [owner, payload, fixture, template];
    const targets = resolveTestConsumerTargets(workspace, packages);

    assert.deepEqual(
      selectTargetsToRun(targets, undefined, true, packages).map((target) => target.source.key),
      ['./plugin/test-consumer/esm', './template/pkg'],
    );
    assert.deepEqual(
      selectTargetsToRun(targets, './template/pkg', false, packages).map((target) => target.source.key),
      ['./template/pkg'],
    );
    assert.throws(() => selectTargetsToRun(targets, './plugin', true, packages), /drop the selector/);
    assert.throws(
      () => selectTargetsToRun(targets, undefined, false, packages),
      /requires a package selector or '--all'/,
    );
    assert.throws(() => selectTargetsToRun([], undefined, true, []), /No consumer tests are registered/);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('rejects a consumer registered under conflicting formats', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-conflicting-format-'));
  try {
    const host = payloadWithOwner(workspace, './host', '@scope/host', { cjs: ['./shared'] });
    const plugin = payloadWithOwner(workspace, './plugin', '@scope/plugin', { esm: ['./shared'] });
    const shared = consumer(workspace, './shared', 'commonjs', '@scope/host', '../host/pkg');
    assert.throws(
      () => resolveTestConsumerTargets(workspace, [host.owner, host.payload, plugin.owner, plugin.payload, shared]),
      /registered under conflicting formats \(cjs, esm\)/,
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

function payloadWithOwner(
  workspace: string,
  ownerKey: string,
  name: string,
  consumerTests: NonNullable<NpmManifestEntry['consumerTests']>,
) {
  const owner = pkg(
    workspace,
    ownerKey,
    { kind: 'dev', name: `${name}-dev`, private: true, member: true, publishedRelPath: `${ownerKey}/pkg` },
    { name: `${name}-dev`, private: true, scripts: { compile: 'compile' } },
  );
  const payload = pkg(
    workspace,
    `${ownerKey}/pkg`,
    { kind: 'published', name, member: false, consumerTests },
    { name, version: '1.0.0' },
  );
  return { owner, payload };
}

function consumer(
  workspace: string,
  key: string,
  type: 'module' | 'commonjs',
  payloadName: string,
  payloadRelPath: string,
  inventory: Partial<Omit<NpmManifestEntry, 'type' | 'browser'>> = {},
): LoadedPackage {
  const name = key.slice(2).replaceAll('/', '-');
  return pkg(
    workspace,
    key,
    { kind: 'standalone', name, private: true, member: false, ...inventory },
    {
      name,
      private: true,
      type,
      scripts: { test: 'node --test' },
      devDependencies: { [payloadName]: `file:${payloadRelPath}` },
    },
  );
}

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
