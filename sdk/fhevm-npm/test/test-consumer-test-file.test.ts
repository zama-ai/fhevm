import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';

import { resolveConsumerTestFile } from '../base/test-consumer.ts';

test('resolves a consumer test file from absolute, workspace-relative, and fixture-relative paths', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-test-file-'));
  try {
    const fixture = join(workspace, 'library', 'test-consumer', 'esm');
    const testFile = join(fixture, 'test', 'focused.test.ts');
    mkdirSync(join(fixture, 'test'), { recursive: true });
    writeFileSync(testFile, 'export {};\n');

    assert.equal(resolveConsumerTestFile(testFile, workspace, fixture), 'test/focused.test.ts');
    assert.equal(
      resolveConsumerTestFile('library/test-consumer/esm/test/focused.test.ts', workspace, fixture),
      'test/focused.test.ts',
    );
    assert.equal(resolveConsumerTestFile('test/focused.test.ts', workspace, fixture), 'test/focused.test.ts');
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test('rejects a selected test file outside the consumer fixture', () => {
  const workspace = mkdtempSync(join(tmpdir(), 'fhevm-npm-test-file-'));
  try {
    const fixture = join(workspace, 'library', 'test-consumer', 'esm');
    const outside = join(workspace, 'outside.test.ts');
    mkdirSync(fixture, { recursive: true });
    writeFileSync(outside, 'export {};\n');

    assert.throws(
      () => resolveConsumerTestFile(outside, workspace, fixture),
      /Consumer test file must be inside/,
    );
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});
