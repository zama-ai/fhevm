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
  tarballFilename,
  tarballPackageJson,
  validateNoFileSpecs,
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
    const tarball = (spec: string) => {
      const staging = join(root, 'staging');
      rmSync(staging, { recursive: true, force: true });
      mkdirSync(join(staging, 'package'), { recursive: true });
      writeFileSync(join(staging, 'package', 'package.json'), JSON.stringify(shipped(spec)));
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

    rmSync(join(root, 'tarballs'), { recursive: true, force: true });
    await assert.rejects(
      inspectPublishedTarball(root, manifest, './plugin/pkg'),
      /run `publish pack \.\/plugin\/pkg` first/,
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
