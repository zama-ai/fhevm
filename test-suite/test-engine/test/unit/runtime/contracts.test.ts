import assert from 'node:assert/strict';
import { cpSync, mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { describe, it } from 'node:test';

import { MISSING_CONTRACTS_DIR_ERROR } from '../../../src/model/execution.js';
import { Contracts, MissingContractsDirError } from '../../../src/runtime/modules/contracts/contracts.js';
import { COUNTER_ARTIFACT } from '../../helpers.js';

/** A Hardhat-like artifacts directory holding Counter (plus its debug file and a build-info entry). */
function artifactsDir(): string {
  const dir = mkdtempSync(path.join(tmpdir(), 'te-artifacts-'));
  mkdirSync(path.join(dir, 'contracts', 'Counter.sol'), { recursive: true });
  cpSync(COUNTER_ARTIFACT, path.join(dir, 'contracts', 'Counter.sol', 'Counter.json'));
  writeFileSync(path.join(dir, 'contracts', 'Counter.sol', 'Counter.dbg.json'), '{}');
  mkdirSync(path.join(dir, 'build-info', 'Counter.sol'), { recursive: true });
  writeFileSync(path.join(dir, 'build-info', 'Counter.sol', 'Counter.json'), '{}');
  return dir;
}

describe('Contracts (this.engine.contracts)', () => {
  it('loads a compiled artifact by contract name', () => {
    const dir = artifactsDir();
    const artifact = new Contracts(dir).artifact('Counter');
    assert.equal(artifact.contractName, 'Counter');
    assert.match(artifact.bytecode, /^0x[0-9a-f]+$/i);
  });

  it('throws a configuration error when the run has no --contracts-dir', () => {
    assert.throws(
      () => new Contracts(undefined).artifact('Counter'),
      (error: unknown) =>
        error instanceof MissingContractsDirError &&
        error.name === MISSING_CONTRACTS_DIR_ERROR &&
        /contract 'Counter' needs compiled contracts, but the run has no --contracts-dir/.test(error.message),
    );
  });

  it('names a missing contract, lists ambiguous ones and refuses non-identifiers', () => {
    const dir = artifactsDir();
    assert.throws(() => new Contracts(dir).artifact('Missing'), /no compiled contract 'Missing' in /);
    mkdirSync(path.join(dir, 'contracts', 'other', 'Counter.sol'), { recursive: true });
    cpSync(COUNTER_ARTIFACT, path.join(dir, 'contracts', 'other', 'Counter.sol', 'Counter.json'));
    assert.throws(
      () => new Contracts(dir).artifact('Counter'),
      /several compiled contracts are named 'Counter': contracts\/Counter\.sol\/Counter\.json, contracts\/other\/Counter\.sol\/Counter\.json/,
    );
    assert.throws(() => new Contracts(dir).artifact('../Counter'), /'\.\.\/Counter' is not a contract name/);
    assert.throws(
      () => new Contracts(path.join(dir, 'absent')).artifact('Counter'),
      /cannot read the contracts directory/,
    );
  });
});
