import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { inspectLintPolicy } from '../base/checks/lint-policy.ts';
import { parseTestNpmManifest } from './helpers.ts';

test('checks every banned lint surface while excluding mirror-only and vendored sources', () => {
  const workspaceRoot = mkdtempSync(join(tmpdir(), 'fhevm-npm-lint-policy-'));
  try {
    write(workspaceRoot, 'package.json', '{ "note": "solhint is forbidden" }\n');
    write(workspaceRoot, '.solhintrc.json', '{}\n');
    write(workspaceRoot, 'node_modules/.bin/solhint', '');

    write(workspaceRoot, 'project/foundry.toml', '[profile.default]\n');
    write(workspaceRoot, 'project/package.json', '{}\n');
    write(workspaceRoot, 'project/src/Owned.sol', '// solhint-disable-next-line\n');
    write(workspaceRoot, 'project/pkg/package.json', '{}\n');
    write(workspaceRoot, 'project/pkg/src/contracts/Vendored.sol', '// solhint-disable-next-line\n');

    write(workspaceRoot, 'mirror/package.json', '{ "devDependency": "solhint" }\n');
    write(workspaceRoot, 'mirror/.solhintignore', 'contracts/**\n');
    write(workspaceRoot, 'mirror/pkg/package.json', '{ "devDependency": "solhint" }\n');
    write(workspaceRoot, 'mirror/pkg/contracts/Mirrored.sol', '// solhint-disable-next-line\n');
    write(workspaceRoot, 'mirror/pkg/node_modules/.bin/solhint', '');

    const manifest = parseTestNpmManifest({
      packageJson: { published: { required: [], excluded: [] } },
      packages: {
        '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
        './project': {
          kind: 'dev',
          name: '@scope/project-dev',
          private: true,
          member: true,
          publishedRelPath: './project/pkg',
        },
        './project/pkg': {
          kind: 'published',
          name: '@scope/project',
          member: true,
          vendored: [
            {
              relPath: './src/contracts',
              source: './source',
              reason: 'Upstream sources must remain byte-identical.',
            },
          ],
        },
        './mirror': {
          kind: 'dev',
          name: '@scope/mirror-dev',
          private: true,
          member: true,
          publishedRelPath: './mirror/pkg',
        },
        './mirror/pkg': {
          kind: 'published',
          name: 'mirror-project',
          member: true,
          distribution: ['mirror'],
          mirror: { repository: 'https://github.com/example/mirror' },
        },
      },
    });

    const inspection = inspectLintPolicy(workspaceRoot, manifest);
    assert.deepEqual(
      inspection.violations.map(({ packageKey, message }) => ({ packageKey, message })),
      [
        {
          packageKey: './.solhintrc.json',
          message: 'forbidden Solidity-linter configuration file exists',
        },
        {
          packageKey: './package.json',
          message: "package.json contains the banned Solidity linter 'solhint' at line 1",
        },
        {
          packageKey: './node_modules/.bin/solhint',
          message: 'installed banned Solidity-linter binary is runnable',
        },
        {
          packageKey: './project/src/Owned.sol',
          message: "owned Solidity contains a banned linter directive 'solhint' at line 1",
        },
      ],
    );
  } finally {
    rmSync(workspaceRoot, { recursive: true, force: true });
  }
});

test('tolerates the hoisted banned binary iff every declarer of the tool is mirror-only', () => {
  const workspaceRoot = mkdtempSync(join(tmpdir(), 'fhevm-npm-lint-policy-'));
  try {
    // npm hoists the mirror-only member's devDependency to the ROOT tree: sanctioned.
    write(workspaceRoot, 'node_modules/.bin/solhint', '');
    write(workspaceRoot, 'package.json', '{}\n');
    write(workspaceRoot, 'mirror/package.json', '{}\n');
    write(workspaceRoot, 'mirror/pkg/package.json', '{ "devDependencies": { "solhint": "^6.2.1" } }\n');
    write(workspaceRoot, 'project/package.json', '{}\n');

    const packages = {
      '.': { kind: 'workspace-root', name: 'workspace', private: true, member: false },
      './project': { kind: 'shared-helper', name: '@scope/project-dev', private: true, member: true },
      './mirror': {
        kind: 'dev',
        name: '@scope/mirror-dev',
        private: true,
        member: true,
        publishedRelPath: './mirror/pkg',
      },
      './mirror/pkg': {
        kind: 'published',
        name: 'mirror-project',
        member: true,
        distribution: ['mirror'],
        mirror: { repository: 'https://github.com/example/mirror' },
      },
    } as const;
    const manifest = parseTestNpmManifest({
      packageJson: { published: { required: [], excluded: [] } },
      packages,
    });
    assert.deepEqual(inspectLintPolicy(workspaceRoot, manifest).violations, []);

    // A NON-mirror package also declares it: the same binary goes back to being a violation.
    write(workspaceRoot, 'project/package.json', '{ "devDependencies": { "solhint": "^6.2.1" } }\n');
    const withNonMirrorDeclarer = inspectLintPolicy(workspaceRoot, manifest).violations;
    assert.ok(
      withNonMirrorDeclarer.some(
        ({ packageKey, message }) =>
          packageKey === './node_modules/.bin/solhint' && message.includes('binary is runnable'),
      ),
      'a non-mirror declarer must keep the installed binary a violation',
    );
  } finally {
    rmSync(workspaceRoot, { recursive: true, force: true });
  }
});

function write(root: string, relativePath: string, contents: string): void {
  const file = join(root, relativePath);
  mkdirSync(join(file, '..'), { recursive: true });
  writeFileSync(file, contents);
}
