import assert from 'node:assert/strict';
import test from 'node:test';

import { inspectCommitScope, pathsOfPorcelainLine } from '../base/checks/commit-scope.ts';

test('flags every pending change outside the sdk workspace, both sides of a rename included', () => {
  const porcelain = [
    ' M sdk/fhevm-npm/cli-options.ts',
    '?? sdk/new-file.ts',
    'A  sdk/host-contracts-cleartext/v12/package.json',
    ' M relayer/src/main.rs',
    'R  sdk/old.ts -> host-contracts/new.ts',
    '?? "sdk/with space.md"',
  ].join('\n');

  const inspection = inspectCommitScope(
    '/repo/sdk',
    () => porcelain,
    () => '/repo',
  );

  assert.equal(inspection.checkedFileKeys.length, 7);
  assert.deepEqual(
    inspection.violations.map((violation) => violation.packageKey),
    ['./relayer/src/main.rs', './host-contracts/new.ts'],
  );
  assert.match(inspection.violations[0]?.message ?? '', /outside the sdk workspace \('sdk\/'\)/);
});

test('passes with no pending changes, and when the workspace is the repository root', () => {
  assert.deepEqual(
    inspectCommitScope(
      '/repo/sdk',
      () => '',
      () => '/repo',
    ).violations,
    [],
  );
  // Workspace == repo root: nothing can be outside it.
  assert.deepEqual(
    inspectCommitScope(
      '/repo',
      () => ' M anything/at/all.ts\n',
      () => '/repo',
    ).violations,
    [],
  );
});

test('parses porcelain rename and quoted paths', () => {
  assert.deepEqual(pathsOfPorcelainLine('R  a/old.ts -> b/new.ts'), ['a/old.ts', 'b/new.ts']);
  assert.deepEqual(pathsOfPorcelainLine('?? "sdk/with space.md"'), ['sdk/with space.md']);
  assert.deepEqual(pathsOfPorcelainLine(' M sdk/plain.ts'), ['sdk/plain.ts']);
});
