import assert from 'node:assert/strict';
import test from 'node:test';

import { parseCliOptions } from '../cli-options.ts';

test('global verbosity counts -v through -vvvv and caps additional flags', () => {
  assert.equal(parseCliOptions(['check-names']).verbosity, 0);
  assert.equal(parseCliOptions(['check-names', '-v']).verbosity, 1);
  assert.equal(parseCliOptions(['-vv', 'check-names']).verbosity, 2);
  assert.equal(parseCliOptions(['check-names', '-vvv']).verbosity, 3);
  assert.equal(parseCliOptions(['-vvvv', 'check-names']).verbosity, 4);
  assert.equal(parseCliOptions(['-vvvvv', 'check-names']).verbosity, 4);
  assert.equal(parseCliOptions(['check-names', '--verbose', '--verbose']).verbosity, 2);
});

test('consumer lock regeneration defaults to every fixture', () => {
  const options = parseCliOptions(['test-consumer-regenerate-package-lock']);
  assert.equal(options.command, 'test-consumer-regenerate-package-lock');
  if (options.command !== 'test-consumer-regenerate-package-lock') throw new Error('unreachable');
  assert.equal(options.packageSelector, undefined);
});

test('consumer lock regeneration accepts one owner or fixture selector', () => {
  const options = parseCliOptions([
    'test-consumer-regenerate-package-lock',
    './host-contracts-cleartext/v13/test-consumer/esm',
  ]);
  assert.equal(options.command, 'test-consumer-regenerate-package-lock');
  if (options.command !== 'test-consumer-regenerate-package-lock') throw new Error('unreachable');
  assert.equal(options.packageSelector, './host-contracts-cleartext/v13/test-consumer/esm');
});

test("test-consumer uses fresh installation by default and accepts '--ci'", () => {
  const fresh = parseCliOptions(['test-consumer', './host-contracts-cleartext/v13']);
  assert.equal(fresh.command, 'test-consumer');
  if (fresh.command !== 'test-consumer') throw new Error('unreachable');
  assert.equal(fresh.ci, false);

  const ci = parseCliOptions(['test-consumer', './host-contracts-cleartext/v13', '--ci']);
  assert.equal(ci.command, 'test-consumer');
  if (ci.command !== 'test-consumer') throw new Error('unreachable');
  assert.equal(ci.ci, true);
});

test("test-consumer accepts '--build-linked-dependencies'", () => {
  const options = parseCliOptions([
    'test-consumer',
    './host-contracts-cleartext/v13/test-consumer/esm',
    '--build-linked-dependencies',
  ]);
  assert.equal(options.command, 'test-consumer');
  if (options.command !== 'test-consumer') throw new Error('unreachable');
  assert.equal(options.buildLinkedDependencies, true);
});

test("test-consumer accepts '--test-file' without implicitly enabling '--run'", () => {
  const options = parseCliOptions([
    'test-consumer',
    './host-contracts-cleartext/v13/test-consumer/esm',
    '--test-file',
    'test/fhe-rand.test.ts',
  ]);
  assert.equal(options.command, 'test-consumer');
  if (options.command !== 'test-consumer') throw new Error('unreachable');
  assert.equal(options.testFile, 'test/fhe-rand.test.ts');
  assert.equal(options.run, false);
});
