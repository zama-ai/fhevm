import { Interface } from 'ethers';
import assert from 'node:assert/strict';
import { cpSync, mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { describe, it } from 'node:test';

import { loadProfile } from '../../../src/config/profiles/profile.js';
import { CONTRACTS_ARTIFACTS_DIR } from '../../../src/infrastructure/paths.js';
import { MISSING_SECTION_ERROR } from '../../../src/model/execution.js';
import { Contracts } from '../../../src/runtime/modules/contracts/contracts.js';
import { MissingProfileSectionError } from '../../../src/runtime/world/errors.js';
import { COUNTER_ARTIFACT, FHE_PROFILES } from '../../helpers.js';

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

const fxFhe = () => loadProfile('fx-fhe', { profilesDir: FHE_PROFILES });

describe('Contracts (this.engine.contracts)', () => {
  it('loads a compiled artifact by contract name', () => {
    const artifact = new Contracts(artifactsDir(), 'none', undefined).artifact('Counter');
    assert.equal(artifact.contractName, 'Counter');
    assert.match(artifact.bytecode, /^0x[0-9a-f]+$/i);
  });

  it('names a missing contract, lists ambiguous ones and refuses non-identifiers', () => {
    const dir = artifactsDir();
    const contracts = new Contracts(dir, 'none', undefined);
    assert.throws(() => contracts.artifact('Missing'), /no compiled contract 'Missing' in /);
    mkdirSync(path.join(dir, 'contracts', 'other', 'Counter.sol'), { recursive: true });
    cpSync(COUNTER_ARTIFACT, path.join(dir, 'contracts', 'other', 'Counter.sol', 'Counter.json'));
    assert.throws(
      () => contracts.artifact('Counter'),
      /several compiled contracts are named 'Counter': contracts\/Counter\.sol\/Counter\.json, contracts\/other\/Counter\.sol\/Counter\.json/,
    );
    assert.throws(() => contracts.artifact('../Counter'), /'\.\.\/Counter' is not a contract name/);
    assert.throws(
      () => new Contracts(path.join(dir, 'absent'), 'none', undefined).artifact('Counter'),
      /cannot read the contracts directory .* \(run 'npm run build'\)/,
    );
  });

  it("builds the coprocessor config from the profile's acl, executor and kmsVerifier", () => {
    const profile = fxFhe();
    assert.deepEqual(new Contracts(CONTRACTS_ARTIFACTS_DIR, profile.name, profile.contracts).coprocessorConfig(), {
      ACLAddress: profile.contracts!.acl.value,
      CoprocessorAddress: profile.contracts!.executor.value,
      KMSVerifierAddress: profile.contracts!.kmsVerifier.value,
    });
  });

  it('throws a configuration error for the coprocessor config of a profile without contracts section', () => {
    assert.throws(
      () => new Contracts(CONTRACTS_ARTIFACTS_DIR, 'none', undefined).coprocessorConfig(),
      (error: unknown) =>
        error instanceof MissingProfileSectionError &&
        error.name === MISSING_SECTION_ERROR &&
        /profile 'none' has no contracts section/.test(error.message),
    );
  });

  it('ships the engine contracts, built by npm run build with the config as constructor argument', () => {
    const profile = fxFhe();
    const contracts = new Contracts(CONTRACTS_ARTIFACTS_DIR, profile.name, profile.contracts);
    const config = contracts.coprocessorConfig();
    for (const name of ['TestInput', 'TestMultiInput', 'UserDecrypt']) {
      const artifact = contracts.artifact(name);
      const abi = new Interface(artifact.abi);
      assert.equal(abi.deploy.format('minimal'), 'constructor((address,address,address))', name);
      // Encodes from the named struct, as ethers does in chain.deploy.
      assert.ok(abi.encodeDeploy([config]).length > 2, name);
      // No protocol address is compiled in: one build serves every profile.
      for (const address of Object.values(config)) {
        assert.ok(
          !artifact.bytecode.toLowerCase().includes(address.slice(2).toLowerCase()),
          `${name} embeds ${address}`,
        );
      }
    }
    assert.throws(() => contracts.artifact('EngineCoprocessorConfig'), /no compiled contract/, 'abstract: no artifact');
  });
});
