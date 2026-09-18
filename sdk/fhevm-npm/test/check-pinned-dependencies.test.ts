import assert from 'node:assert/strict';
import test from 'node:test';

import { validatePinnedDependencies } from '../base/checks/pinned-dependencies.ts';
import { loadedPackage, parseTestNpmManifest } from './helpers.ts';

function manifestWithPin(pinned?: Record<string, string>) {
  return parseTestNpmManifest({
    dependencies: { forbidden: ['solhint'], ...(pinned === undefined ? {} : { pinned }) },
    packageJson: { published: { required: ['name', 'version'], excluded: ['private'] } },
    packages: {
      '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      './plugin': { kind: 'standalone', name: 'plugin-dev', private: true, member: false },
      './plugin/pkg': { kind: 'standalone', name: 'plugin', private: true, member: false },
    },
  });
}

const root = (manifest: ReturnType<typeof manifestWithPin>) =>
  loadedPackage('.', manifest.packages['.']!, { name: 'workspace', private: true });

test('rule 3.3.4 accepts a pinned dependency repeated verbatim in every field that declares it', () => {
  const manifest = manifestWithPin({ '@fhevm/sdk': '^0.13.3' });
  const packages = [
    root(manifest),
    loadedPackage('./plugin', manifest.packages['./plugin']!, {
      name: 'plugin-dev',
      private: true,
      devDependencies: { '@fhevm/sdk': '^0.13.3' },
    }),
    loadedPackage('./plugin/pkg', manifest.packages['./plugin/pkg']!, {
      name: 'plugin',
      private: true,
      peerDependencies: { '@fhevm/sdk': '^0.13.3' },
    }),
  ];

  assert.deepEqual(
    validatePinnedDependencies(manifest, packages, () => undefined),
    [],
  );
});

test('rule 3.3.4 rejects a spec that differs from the pin, including one that only differs by range operator', () => {
  const manifest = manifestWithPin({ '@fhevm/sdk': '^0.13.3' });
  const packages = [
    root(manifest),
    loadedPackage('./plugin', manifest.packages['./plugin']!, {
      name: 'plugin-dev',
      private: true,
      devDependencies: { '@fhevm/sdk': '0.13.3' },
    }),
    loadedPackage('./plugin/pkg', manifest.packages['./plugin/pkg']!, {
      name: 'plugin',
      private: true,
      peerDependencies: { '@fhevm/sdk': '^0.13.2' },
    }),
  ];

  const violations = validatePinnedDependencies(manifest, packages, () => undefined);
  assert.equal(violations.length, 2);
  assert.deepEqual(
    violations.map((violation) => violation.packageKey),
    ['./plugin/package.json', './plugin/pkg/package.json'],
  );
  assert.equal(violations[0]?.rule, '3.3.4');
  assert.match(violations[0]?.message ?? '', /'devDependencies' is "0\.13\.3"; .*requires "\^0\.13\.3"/);
  assert.match(violations[1]?.message ?? '', /'peerDependencies' is "\^0\.13\.2"; .*requires "\^0\.13\.3"/);
});

test('rule 3.3.4 reports every offending field of one package separately', () => {
  const manifest = manifestWithPin({ '@fhevm/sdk': '^0.13.3' });
  const packages = [
    root(manifest),
    loadedPackage('./plugin', manifest.packages['./plugin']!, {
      name: 'plugin-dev',
      private: true,
      dependencies: { '@fhevm/sdk': '0.13.1' },
      devDependencies: { '@fhevm/sdk': '0.13.2' },
    }),
  ];

  const violations = validatePinnedDependencies(manifest, packages, () => undefined);
  assert.equal(violations.length, 2);
  assert.deepEqual(
    violations.map((violation) => violation.rule),
    ['3.3.4', '3.3.4'],
  );
});

test('rule 3.3.4 ignores a package that does not declare the pinned dependency at all', () => {
  const manifest = manifestWithPin({ '@fhevm/sdk': '^0.13.3' });
  const packages = [
    root(manifest),
    loadedPackage('./plugin', manifest.packages['./plugin']!, {
      name: 'plugin-dev',
      private: true,
      devDependencies: { '@fhevm/sdk': '^0.13.3' },
    }),
    loadedPackage('./plugin/pkg', manifest.packages['./plugin/pkg']!, { name: 'plugin', private: true }),
  ];

  assert.deepEqual(
    validatePinnedDependencies(manifest, packages, () => undefined),
    [],
  );
});

test('rule 3.3.4 rejects a pin that no inventoried package declares', () => {
  const manifest = manifestWithPin({ '@fhevm/sdk': '^0.13.3' });
  const packages = [
    root(manifest),
    loadedPackage('./plugin', manifest.packages['./plugin']!, { name: 'plugin-dev', private: true }),
  ];

  const violations = validatePinnedDependencies(manifest, packages, () => undefined);
  assert.equal(violations.length, 1);
  assert.equal(violations[0]?.packageKey, './npm-manifest.json');
  assert.match(violations[0]?.message ?? '', /is unused/);
});

test('rule 3.3.4 rejects a lockfile still mirroring a pre-alignment spec', () => {
  const manifest = manifestWithPin({ '@fhevm/sdk': '^0.13.4' });
  const packages = [
    root(manifest),
    loadedPackage('./plugin', manifest.packages['./plugin']!, {
      name: 'plugin-dev',
      private: true,
      devDependencies: { '@fhevm/sdk': '^0.13.4' },
    }),
  ];
  const readLockfile = (file: string) =>
    file === '/workspace/plugin/package-lock.json'
      ? { '': { devDependencies: { '@fhevm/sdk': '^0.13.3' } } }
      : undefined;

  const violations = validatePinnedDependencies(manifest, packages, readLockfile);
  assert.equal(violations.length, 1);
  assert.equal(violations[0]?.packageKey, './plugin/package-lock.json');
  assert.match(violations[0]?.message ?? '', /stale lockfile: '@fhevm\/sdk' in 'devDependencies' of '\.'/);
});

test('rule 3.3.4 ignores resolved node_modules entries inside a lockfile', () => {
  const manifest = manifestWithPin({ '@fhevm/sdk': '^0.13.4' });
  const packages = [
    root(manifest),
    loadedPackage('./plugin', manifest.packages['./plugin']!, {
      name: 'plugin-dev',
      private: true,
      devDependencies: { '@fhevm/sdk': '^0.13.4' },
    }),
  ];
  const readLockfile = () => ({
    '': { devDependencies: { '@fhevm/sdk': '^0.13.4' } },
    'node_modules/some-dep': { dependencies: { '@fhevm/sdk': '^0.13.1' } },
  });

  assert.deepEqual(validatePinnedDependencies(manifest, packages, readLockfile), []);
});

test('rule 3.3.4 is inert when the manifest pins nothing', () => {
  const manifest = manifestWithPin();
  const packages = [
    root(manifest),
    loadedPackage('./plugin', manifest.packages['./plugin']!, {
      name: 'plugin-dev',
      private: true,
      devDependencies: { '@fhevm/sdk': '0.1.0' },
    }),
  ];

  assert.deepEqual(
    validatePinnedDependencies(manifest, packages, () => undefined),
    [],
  );
});

test('the manifest rejects a pinned spec that is not an exact, caret or tilde version', () => {
  assert.throws(() => manifestWithPin({ '@fhevm/sdk': '>=0.13.3 <0.14.0' }), /exact, caret, or tilde/);
  assert.throws(() => manifestWithPin({ '@fhevm/sdk': 'latest' }), /exact, caret, or tilde/);
});

test('the manifest rejects an empty pinned map', () => {
  assert.throws(() => manifestWithPin({}), /must pin at least one package/);
});
