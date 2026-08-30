import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import test from 'node:test';

import {
  expectedVendoredContent,
  validateVendoredMetadata,
  validateVendoredPackage,
  vendoredPackageKeys,
} from '../base/checks/vendored.ts';
import type { NpmManifest } from '../manifest.ts';

test('checks a manifest-selected local vendored copy with its declared rewrite', () => {
  const repositoryRoot = execFileSync('git', ['rev-parse', '--show-toplevel'], { encoding: 'utf8' }).trim();
  const workspaceRoot = mkdtempSync(join(process.cwd(), '.tmp-check-vendored-'));
  try {
    const sourceDirectory = join(workspaceRoot, 'common-vendored', 'src');
    const destinationDirectory = join(workspaceRoot, 'library', 'pkg', 'vendored');
    mkdirSync(sourceDirectory, { recursive: true });
    mkdirSync(destinationDirectory, { recursive: true });
    mkdirSync(join(workspaceRoot, 'library'), { recursive: true });
    writeFileSync(join(workspaceRoot, 'library', 'package.json'), '{"name":"library-dev","private":true}\n');
    writeFileSync(join(workspaceRoot, 'library', 'pkg', 'package.json'), '{"name":"library"}\n');
    writeFileSync(join(sourceDirectory, 'adapter.ts'), "export { value } from './types.ts';\n");
    writeFileSync(join(destinationDirectory, 'adapter.ts'), "export { value } from 'types-package';\n");
    writeFileSync(
      join(workspaceRoot, 'common-vendored', 'manifest.json'),
      `${JSON.stringify(
        {
          source: 'common-vendored/src',
          destinations: [
            {
              to: 'library/pkg/vendored',
              files: ['adapter.ts'],
              rewrites: [{ file: 'adapter.ts', from: "'./types.ts'", to: "'types-package'" }],
            },
          ],
        },
        null,
        2,
      )}\n`,
    );

    const repositoryRelativeSource = `./${relative(repositoryRoot, sourceDirectory).replaceAll('\\', '/')}`;
    const manifest = {
      packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
      packages: {
        './library': {
          kind: 'dev',
          type: 'esm',
          browser: false,
          name: 'library-dev',
          private: true,
          member: true,
          publishedRelPath: './library/pkg',
        },
        './library/pkg': {
          kind: 'published',
          type: 'esm',
          browser: false,
          name: 'library',
          member: false,
          vendored: [
            {
              relPath: './vendored',
              files: ['adapter.ts'],
              source: repositoryRelativeSource,
              reason: 'The published package cannot import the private source package.',
            },
          ],
        },
      },
    } satisfies NpmManifest;

    const valid = validateVendoredPackage(workspaceRoot, manifest, './library');
    assert.deepEqual(valid.violations, []);
    assert.equal(valid.successes.length, 1);

    writeFileSync(join(destinationDirectory, 'adapter.ts'), 'drift\n');
    const drifted = validateVendoredPackage(workspaceRoot, manifest, './library/pkg');
    assert.equal(drifted.violations.length, 1);
    assert.match(drifted.violations[0]!.message, /differs from/);
  } finally {
    rmSync(workspaceRoot, { recursive: true, force: true });
  }
});

test('requires package.json fhevm.vendoredFrom to match the pinned manifest source', () => {
  const entry = {
    relPath: './src/contracts',
    source: {
      repository: 'https://github.com/example/repository',
      tag: 'v1.2.3',
      commit: '0123456789abcdef0123456789abcdef01234567',
      from: 'contracts',
    },
    reason: 'The upstream source is not available as an npm dependency.',
  } as const;
  const valid = {
    repository: entry.source.repository,
    tag: entry.source.tag,
    commit: entry.source.commit,
    from: entry.source.from,
    to: 'src/contracts',
  };

  assert.deepEqual(validateVendoredMetadata({ fhevm: { vendoredFrom: valid } }, [entry]), []);
  assert.deepEqual(validateVendoredMetadata({}, [entry]), [
    'package.json must define fhevm.vendoredFrom for its pinned vendored source',
  ]);
  assert.deepEqual(validateVendoredMetadata({ fhevm: { vendoredFrom: { ...valid, tag: 'v9.9.9' } } }, [entry]), [
    'package.json#fhevm.vendoredFrom differs from npm-manifest.json: tag="v9.9.9" (expected "v1.2.3")',
  ]);
});

test('rejects stale fhevm.vendoredFrom metadata for a package with only local vendored sources', () => {
  const localEntry = {
    relPath: './src/vendored',
    source: './sdk/common-vendored/src',
    reason: 'The published package cannot depend on a private package.',
  } as const;

  assert.deepEqual(validateVendoredMetadata({}, [localEntry]), []);
  assert.deepEqual(validateVendoredMetadata({ fhevm: { vendoredFrom: {} } }, [localEntry]), [
    'package.json#fhevm.vendoredFrom is declared, but npm-manifest.json has no pinned vendored source',
  ]);
});

test('with no selector, enumerates every package that declares vendored content', () => {
  const manifest = {
    packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
    packages: {
      './library': { kind: 'dev', type: 'esm', browser: false, name: 'library-dev', private: true, member: true },
      './library/pkg': {
        kind: 'published',
        type: 'esm',
        browser: false,
        name: 'library',
        member: false,
        vendored: [{ relPath: './vendored', source: './sdk/common-vendored/src', reason: 'reason' }],
      },
      './plain': { kind: 'published', type: 'esm', browser: false, name: 'plain', member: false },
    },
  } satisfies NpmManifest;

  const workspaceRoot = mkdtempSync(join(process.cwd(), '.tmp-vendored-keys-'));
  try {
    mkdirSync(join(workspaceRoot, 'library', 'pkg'), { recursive: true });
    mkdirSync(join(workspaceRoot, 'plain'), { recursive: true });
    writeFileSync(join(workspaceRoot, 'library', 'package.json'), '{"name":"library-dev","private":true}\n');
    writeFileSync(join(workspaceRoot, 'library', 'pkg', 'package.json'), '{"name":"library"}\n');
    writeFileSync(join(workspaceRoot, 'plain', 'package.json'), '{"name":"plain"}\n');

    // Only the package that declares `vendored` — a package without it is not a silent pass, it is
    // simply not a subject of this check.
    assert.deepEqual(vendoredPackageKeys(workspaceRoot, manifest), ['./library/pkg']);
  } finally {
    rmSync(workspaceRoot, { recursive: true, force: true });
  }
});

test('expectedVendoredContent applies declared rewrites and fails loudly when one does not', () => {
  const mapping = {
    to: 'destination',
    files: ['adapter.ts'],
    rewrites: [{ file: 'adapter.ts', from: "'./types.ts'", to: "'types-package'" }],
  };

  const rewritten = expectedVendoredContent("export { value } from './types.ts';\n", mapping, 'adapter.ts');
  assert.equal(rewritten.error, undefined);
  assert.equal(rewritten.error === undefined ? rewritten.content : '', "export { value } from 'types-package';\n");

  // A rewrite that matches nothing is a hard failure: a renamed import would otherwise leave the
  // destination unrewritten but still compiling.
  const missing = expectedVendoredContent("export { value } from './other.ts';\n", mapping, 'adapter.ts');
  assert.match(missing.error ?? '', /was not found/);

  // A rewrite is scoped to its own file, so another file in the same destination is untouched.
  const other = expectedVendoredContent("export { value } from './types.ts';\n", mapping, 'elsewhere.ts');
  assert.equal(other.error === undefined ? other.content : '', "export { value } from './types.ts';\n");
});
