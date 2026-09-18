import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';

import {
  type RegistryOptions,
  checkRegistry,
  inspectPublishedTarball,
  declaredEntryPoints,
  tarballFilename,
  tarballFiles,
  tarballPackageJson,
  validateNoFileSpecs,
  validateRenderedRanges,
  validateTarballEntryPoints,
  validateTarballVersion,
} from '../base/publish-check.ts';
import { loadedPackage, parseTestNpmManifest } from './helpers.ts';

const library = loadedPackage(
  './library/pkg',
  { kind: 'published', name: '@scope/library', member: true },
  {
    name: '@scope/library',
    version: '0.13.4',
  },
);
const plugin = loadedPackage(
  './plugin/pkg',
  { kind: 'published', name: '@scope/plugin', member: true },
  {
    name: '@scope/plugin',
    version: '0.13.0',
  },
);
const shipped = (spec: string, version = '0.13.0') => ({
  name: '@scope/plugin',
  version,
  dependencies: { '@scope/library': spec, hardhat: '^3.0.0' },
});

/** A registry with the given versions of the library and of the plugin; counts the calls. */
function registry(libraryVersions: string[], pluginVersions: string[], calls: string[] = []): RegistryOptions {
  return {
    retries: 2,
    retryDelayMs: 0,
    sleep: async () => {},
    fetchRegistry: async (url) => {
      calls.push(url);
      const versions = url.endsWith('library') ? libraryVersions : pluginVersions;
      return {
        status: versions.length === 0 ? 404 : 200,
        json: async () => ({
          'dist-tags': { latest: versions.at(-1) },
          versions: Object.fromEntries(versions.map((v) => [v, {}])),
        }),
      };
    },
  };
}

test('npm names the tarball from the package name, scope marker dropped, and the version', () => {
  assert.equal(tarballFilename(plugin, '0.13.1'), 'scope-plugin-0.13.1.tgz');
  assert.equal(
    tarballFilename(
      loadedPackage('./t/pkg', { kind: 'published', name: 't', member: true }, { name: 'template', version: '1.0.0' }),
      '1.0.0',
    ),
    'template-1.0.0.tgz',
  );
});

test('statically: a surviving file: spec or a version off the central one is a 5.3.10 violation', () => {
  assert.deepEqual(validateNoFileSpecs(plugin, shipped('^0.13.0')), []);
  assert.deepEqual(
    validateNoFileSpecs(plugin, shipped('file:../../library/pkg')).map((v) => [v.rule, v.message]),
    [['5.3.10', `tarball ships '@scope/library' as "file:../../library/pkg" in dependencies; render it`]],
  );
  assert.deepEqual(validateTarballVersion(plugin, shipped('^0.13.0'), '0.13.0'), []);
  assert.match(
    validateTarballVersion(plugin, shipped('^0.13.0', '0.12.9'), '0.13.0')[0]?.message ?? '',
    /tarball version is 0.12.9; central version is 0.13.0/,
  );
});

test('a dependency on another payload must carry exactly the range its central version renders to', () => {
  const central = {
    $schema: './fhevm-npm/schemas/versions.schema.json' as const,
    schemaVersion: 1 as const,
    packages: { './library/pkg': '0.13.4', './plugin/pkg': '0.13.0' },
  };
  const shipped = (spec: string) =>
    ({ name: '@scope/plugin', dependencies: { '@scope/library': spec, hardhat: '^3.0.0' } }) as never;

  // 0.13.4 renders to ^0.13.0: the generation, any patch.
  assert.deepEqual(validateRenderedRanges(plugin, shipped('^0.13.0'), [library, plugin], central), []);
  for (const wrong of ['^0.13.4', '0.13.4', '^0.12.0', '^0.14.0', 'not-a-version']) {
    const violations = validateRenderedRanges(plugin, shipped(wrong), [library, plugin], central);
    assert.equal(violations.length, 1, wrong);
    assert.equal(violations[0]?.rule, '5.3.10');
    assert.match(violations[0]?.message ?? '', /its central version renders to "\^0\.13\.0"/);
  }
  // An external dependency is not ours to judge, and a name that is no payload is left alone.
  assert.deepEqual(validateRenderedRanges(plugin, shipped('^0.13.0'), [library, plugin], central), []);
  assert.deepEqual(validateRenderedRanges(plugin, shipped('^0.13.0'), [plugin], central), []);
});

test('every entry point the shipped package.json declares is collected, from every field and nested condition', () => {
  assert.deepEqual(
    declaredEntryPoints({
      name: '@scope/plugin',
      main: './_cjs/index.js',
      types: './_types/index.d.ts',
      bin: { plugin: './bin/cli.js' },
      exports: {
        '.': { import: { types: './_types/index.d.ts', default: './_esm/index.js' }, require: './_cjs/index.js' },
        './abi/*.json': './abi/*.json',
        './package.json': './package.json',
      },
      dependencies: { hardhat: '^3.0.0' },
    } as never),
    ['_cjs/index.js', '_esm/index.js', '_types/index.d.ts', 'abi/*.json', 'bin/cli.js', 'package.json'],
  );
});

test('an entry point the tarball does not contain is a 5.3.10 violation; a subpath pattern needs one match', () => {
  const shipped = {
    name: '@scope/plugin',
    version: '0.13.0',
    types: './_types/index.d.ts',
    exports: { '.': { import: './_esm/index.js' }, './abi/*.json': './abi/*.json' },
  } as never;
  const complete = ['package.json', '_types/index.d.ts', '_esm/index.js', 'abi/ACL.json'];
  assert.deepEqual(validateTarballEntryPoints(plugin, shipped, complete), []);

  // Built on disk but left out of "files": the published package resolves to nothing.
  assert.deepEqual(
    validateTarballEntryPoints(plugin, shipped, ['package.json', '_esm/index.js', 'abi/ACL.json']).map((v) => [
      v.rule,
      v.message,
    ]),
    [['5.3.10', `package.json points at '_types/index.d.ts', which the tarball does not contain`]],
  );
  // The pattern matches nothing: the whole abi/ tree was excluded.
  assert.deepEqual(
    validateTarballEntryPoints(plugin, shipped, ['package.json', '_types/index.d.ts', '_esm/index.js']).map(
      (v) => v.message,
    ),
    [`package.json points at 'abi/*.json', which the tarball does not contain`],
  );
});

test("in `exports` a `*` spans separators, unlike a filesystem glob: './src/*' covers a nested file", () => {
  const shipped = {
    name: '@scope/library',
    exports: { './src/*': './src/*', './flat/*.json': './flat/*.json' },
  } as never;
  assert.deepEqual(validateTarballEntryPoints(library, shipped, ['src/deep/nested/A.sol', 'flat/a.json']), []);
  // A dot in the pattern stays literal: 'flat/a.txt' does not satisfy './flat/*.json'.
  assert.deepEqual(
    validateTarballEntryPoints(library, shipped, ['src/A.sol', 'flat/a.txt']).map((v) => v.message),
    [`package.json points at 'flat/*.json', which the tarball does not contain`],
  );
});

test('registry: a rendered range needs a satisfying published version, retried; the own version must be absent, not retried', async () => {
  const ok = await checkRegistry(
    plugin,
    shipped('^0.13.0'),
    [library, plugin],
    registry(['0.13.0', '0.13.2'], ['0.12.0']),
  );
  assert.deepEqual(ok, []);

  // The dependency is missing: probed 1 + retries times, then reported; the own lookup happens once.
  const calls: string[] = [];
  const missing = await checkRegistry(plugin, shipped('^0.13.0'), [library, plugin], registry([], ['0.12.0'], calls));
  assert.deepEqual(
    missing.map((v) => v.message),
    ['npmjs.com has no version of @scope/library satisfying ^0.13.0'],
  );
  assert.deepEqual(calls.filter((url) => url.endsWith('library')).length, 3);
  assert.deepEqual(calls.filter((url) => url.endsWith('plugin')).length, 1);

  // A dependency that only has the next generation does not satisfy this one.
  const wrongGeneration = await checkRegistry(plugin, shipped('^0.13.0'), [library, plugin], registry(['0.14.0'], []));
  assert.equal(wrongGeneration.length, 1);

  // The payload itself already on the registry at this version: refused.
  const already = await checkRegistry(plugin, shipped('^0.13.0'), [library, plugin], registry(['0.13.0'], ['0.13.0']));
  assert.deepEqual(
    already.map((v) => v.message),
    ['@scope/plugin@0.13.0 is already on npmjs.com'],
  );

  // External dependencies (hardhat) are never asked about.
  const asked: string[] = [];
  await checkRegistry(plugin, shipped('^0.13.0'), [library, plugin], registry(['0.13.0'], [], asked));
  assert.equal(
    asked.some((url) => url.includes('hardhat')),
    false,
  );
});

test('end to end on a real tarball: the packed manifest is read with tar and judged against the central file', async () => {
  const root = mkdtempSync(join(tmpdir(), 'fhevm-npm-publish-check-'));
  try {
    const write = (rel: string, text: string) => {
      mkdirSync(dirname(join(root, rel)), { recursive: true });
      writeFileSync(join(root, rel), text);
    };
    write('package.json', '{ "name": "workspace", "private": true }\n');
    write('library/pkg/package.json', JSON.stringify({ name: '@scope/library', version: '0.13.4' }));
    write(
      'plugin/pkg/package.json',
      JSON.stringify({
        name: '@scope/plugin',
        version: '0.13.0',
        dependencies: { '@scope/library': 'file:../../library/pkg' },
      }),
    );
    write(
      'versions.json',
      JSON.stringify({
        $schema: './fhevm-npm/schemas/versions.schema.json',
        schemaVersion: 1,
        packages: { './library/pkg': '0.13.4', './plugin/pkg': '0.13.0' },
      }),
    );
    const manifest = parseTestNpmManifest({
      tarballs: { relPath: './tarballs' },
      packageJson: { published: { required: [], excluded: [] } },
      packages: {
        '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
        './library/pkg': { kind: 'published', name: '@scope/library', member: true },
        './plugin/pkg': { kind: 'published', name: '@scope/plugin', member: true },
      },
    });
    // Build the tarball the way npm lays it out: a top-level `package/` directory.
    const tarball = (spec: string, withEntryPoint = true) => {
      const staging = join(root, 'staging');
      rmSync(staging, { recursive: true, force: true });
      mkdirSync(join(staging, 'package', '_esm'), { recursive: true });
      writeFileSync(
        join(staging, 'package', 'package.json'),
        JSON.stringify({ ...shipped(spec), exports: { '.': './_esm/index.js' } }),
      );
      if (withEntryPoint) writeFileSync(join(staging, 'package', '_esm', 'index.js'), 'export {};\n');
      mkdirSync(join(root, 'tarballs'), { recursive: true });
      execFileSync('tar', ['-czf', join(root, 'tarballs', 'scope-plugin-0.13.0.tgz'), '-C', staging, 'package']);
    };
    tarball('^0.13.0');
    assert.deepEqual(
      tarballPackageJson(join(root, 'tarballs', 'scope-plugin-0.13.0.tgz')).dependencies,
      shipped('^0.13.0').dependencies,
    );
    assert.deepEqual((await inspectPublishedTarball(root, manifest, './plugin/pkg')).violations, []);
    const withRegistry = await inspectPublishedTarball(root, manifest, 'plugin/pkg', {
      registry: registry(['0.13.0'], []),
    });
    assert.deepEqual(withRegistry.violations, []);

    tarball('file:../../library/pkg');
    const unrendered = await inspectPublishedTarball(root, manifest, './plugin/pkg');
    assert.equal(unrendered.violations.length, 1);
    assert.match(unrendered.violations[0]?.message ?? '', /render it/);

    // The entry point is declared but was not packed: caught on the artifact, where publint never looks.
    tarball('^0.13.0', false);
    const hollow = await inspectPublishedTarball(root, manifest, './plugin/pkg');
    assert.deepEqual(
      hollow.violations.map((v) => v.message),
      [`package.json points at '_esm/index.js', which the tarball does not contain`],
    );
    assert.deepEqual(tarballFiles(hollow.tarball), ['package.json']);

    rmSync(join(root, 'tarballs'), { recursive: true, force: true });
    await assert.rejects(
      inspectPublishedTarball(root, manifest, './plugin/pkg'),
      /run `publish pack \.\/plugin\/pkg` first/,
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
