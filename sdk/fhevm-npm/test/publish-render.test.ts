import assert from 'node:assert/strict';
import test from 'node:test';

import {
  formatRenderedDiff,
  payloadGraph,
  renderPackageJson,
  resolvePayload,
  topologicalOrder,
} from '../base/publish-render.ts';
import type { VersionsFile } from '../base/versions.ts';
import { loadedPackage } from './helpers.ts';

const versions = (packages: Record<string, string>): VersionsFile => ({
  $schema: './fhevm-npm/schemas/versions.schema.json',
  schemaVersion: 1,
  packages,
});

// sdk root: a library (npm+mirror) and a private helper; a cluster root: a plugin linking the library
// cross-root, an owner for it, and a mirror-only template linking the plugin.
const library = loadedPackage(
  './library/pkg',
  { kind: 'published', name: '@scope/library', member: true, mirror: { repository: 'https://x/library' } },
  { name: '@scope/library', version: '0.13.4', dependencies: { zod: '^4.0.0' } },
);
const helper = loadedPackage(
  './common',
  { kind: 'shared-helper', name: '@scope/common-dev', private: true, member: true },
  { name: '@scope/common-dev', version: '0.0.0' },
);
const plugin = loadedPackage(
  './cluster/plugin/pkg',
  { kind: 'published', name: '@scope/plugin', member: true, memberOf: './cluster' },
  {
    name: '@scope/plugin',
    version: '0.13.0',
    dependencies: { '@scope/library': 'file:../../../library/pkg', hardhat: '^3.0.0' },
    peerDependencies: { '@scope/sdk': '^0.13.3' },
  },
);
const owner = loadedPackage(
  './cluster/plugin',
  {
    kind: 'dev',
    name: '@scope/plugin-dev',
    private: true,
    member: true,
    memberOf: './cluster',
    publishedRelPath: './cluster/plugin/pkg',
  },
  { name: '@scope/plugin-dev', private: true, version: '0.0.0' },
);
const template = loadedPackage(
  './cluster/template/pkg',
  {
    kind: 'published',
    name: 'template',
    member: true,
    memberOf: './cluster',
    distribution: ['mirror'],
    mirror: { repository: 'https://x/t' },
  },
  { name: 'template', version: '0.4.2', devDependencies: { '@scope/plugin': 'file:../../plugin/pkg' } },
);
const packages = [library, helper, plugin, owner, template];
const central = versions({
  './library/pkg': '0.13.4',
  './cluster/plugin/pkg': '0.13.0',
  './cluster/template/pkg': '0.4.2',
});

test('the graph has one node per npm-distributed payload and one edge per file: link between them', () => {
  const graph = payloadGraph(packages);
  assert.deepEqual(graph.nodes, ['./cluster/plugin/pkg', './library/pkg']);
  assert.deepEqual(
    [...graph.edges],
    [
      ['./library/pkg', []],
      ['./cluster/plugin/pkg', ['./library/pkg']],
    ],
  );
});

test('publish order lists dependencies first and refuses a cycle', () => {
  assert.deepEqual(topologicalOrder(payloadGraph(packages)), ['./library/pkg', './cluster/plugin/pkg']);
  const cycle = {
    nodes: ['a', 'b'],
    edges: new Map([
      ['a', ['b']],
      ['b', ['a']],
    ]),
  };
  assert.throws(() => topologicalOrder(cycle), /dependency cycle among a, b/);
});

test('rendering replaces each file: link by the generation range of its target, and nothing else', () => {
  const rendered = renderPackageJson(plugin, packages, central);
  assert.deepEqual(rendered.replacements, [
    { field: 'dependencies', name: '@scope/library', from: 'file:../../../library/pkg', to: '^0.13.0' },
  ]);
  assert.deepEqual(rendered.packageJson['dependencies'], { '@scope/library': '^0.13.0', hardhat: '^3.0.0' });
  assert.deepEqual(rendered.packageJson['peerDependencies'], { '@scope/sdk': '^0.13.3' });
  // The source object is untouched: render is pure.
  assert.equal(plugin.packageJson.dependencies?.['@scope/library'], 'file:../../../library/pkg');
  assert.equal(
    formatRenderedDiff(plugin, rendered),
    [
      '--- ./cluster/plugin/pkg/package.json',
      '+++ rendered',
      '-    "@scope/library": "file:../../../library/pkg",',
      '+    "@scope/library": "^0.13.0",',
    ].join('\n'),
  );
});

test('a payload with no file: link has nothing to render, and says so (decision 7)', () => {
  const rendered = renderPackageJson(library, packages, central);
  assert.deepEqual(rendered.replacements, []);
  assert.equal(formatRenderedDiff(library, rendered), 'nothing to render: package.json ships as-is');
});

test('a prerelease central version renders exactly; a link to a private helper or an unversioned target cannot render', () => {
  const alpha = renderPackageJson(plugin, packages, versions({ './library/pkg': '0.14.0-alpha.0' }));
  assert.equal(alpha.replacements[0]?.to, '0.14.0-alpha.0');

  const linksHelper = loadedPackage('./cluster/plugin/pkg', plugin.inventory, {
    ...plugin.packageJson,
    dependencies: { '@scope/common-dev': 'file:../../../common' },
  });
  assert.throws(() => renderPackageJson(linksHelper, packages, central), /not an npm-published payload/);
  assert.throws(() => renderPackageJson(plugin, packages, versions({})), /no canonical central version/);
  assert.throws(() => renderPackageJson(template, packages, central), /not an npm-distributed payload/);
});

test('a payload is selected by its key, its owner, or either without the leading ./', () => {
  for (const selector of ['./cluster/plugin/pkg', 'cluster/plugin/pkg', './cluster/plugin', 'cluster/plugin']) {
    assert.equal(resolvePayload(packages, selector).key, './cluster/plugin/pkg', selector);
  }
  assert.throws(() => resolvePayload(packages, './cluster/template/pkg'), /No npm-distributed payload matches/);
  assert.throws(() => resolvePayload(packages, './nowhere'), /Payloads: \.\/cluster\/plugin\/pkg, \.\/library\/pkg/);
});
