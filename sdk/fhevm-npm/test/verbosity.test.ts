import assert from 'node:assert/strict';
import test from 'node:test';

import { hasDetailedOutput, hasProgress, npmVerbosityArguments } from '../base/verbosity.ts';

test('verbosity levels preserve the former verbose behavior at -vv', () => {
  assert.equal(hasProgress(0), false);
  assert.equal(hasProgress(1), true);
  assert.equal(hasDetailedOutput(1), false);
  assert.equal(hasDetailedOutput(2), true);
});

test('npm log levels are enabled only at -vvv and -vvvv', () => {
  assert.deepEqual(npmVerbosityArguments(0), []);
  assert.deepEqual(npmVerbosityArguments(1), []);
  assert.deepEqual(npmVerbosityArguments(2), []);
  assert.deepEqual(npmVerbosityArguments(3), ['--loglevel', 'verbose']);
  assert.deepEqual(npmVerbosityArguments(4), ['--loglevel', 'silly']);
});
