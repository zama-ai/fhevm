/**
 * Reference scenario for scenario authors: it uses only the engine API (`#engine`). The contract
 * comes from `this.engine.contracts` (compiled for the profile by `test-engine compile`), every
 * transaction and FHE operation is recorded as evidence by the engine, and the relayer job IDs
 * end up in the report.
 */
import { type EncryptedInput, EngineWorld } from '#engine';
import { Given, Then, When, setWorldConstructor } from '@cucumber/cucumber';
import type { Contract } from 'ethers';
import assert from 'node:assert/strict';

class InputDecryptWorld extends EngineWorld {
  contract: Contract | undefined;
  input: EncryptedInput | undefined;

  get testInput(): Contract {
    assert.ok(this.contract, 'the TestInput contract is not deployed yet');
    return this.contract;
  }

  /** The handle of the result stored by `add42ToInput64`. */
  async result(): Promise<string> {
    return String(await this.testInput.getFunction('resUint64')());
  }
}

setWorldConstructor(InputDecryptWorld);

Given('alice has deployed the TestInput contract', async function (this: InputDecryptWorld) {
  this.contract = await this.engine.chain.deploy(this.engine.contracts.artifact('TestInput'), [], {
    from: 'alice',
    label: 'Deploy TestInput',
  });
});

When('alice encrypts the uint64 value {int} for the contract', async function (this: InputDecryptWorld, value: number) {
  this.input = await this.engine.fhe.encrypt({
    contract: this.testInput,
    user: 'alice',
    values: [{ type: 'uint64', value: BigInt(value) }],
  });
});

When('alice submits the encrypted value to add42ToInput64', async function (this: InputDecryptWorld) {
  assert.ok(this.input, 'no value encrypted yet');
  const { handles, inputProof } = this.input;
  const alice = this.engine.chain.wallet('alice');
  await this.engine.chain.send(this.testInput.connect(alice).getFunction('add42ToInput64')(handles[0], inputProof), {
    label: 'add42ToInput64',
  });
});

Then('alice decrypts the result as {int}', async function (this: InputDecryptWorld, expected: number) {
  const value = await this.engine.fhe.userDecrypt({
    handle: await this.result(),
    contract: this.testInput,
    user: 'alice',
  });
  assert.equal(value, BigInt(expected));
});

Then('the result is publicly decryptable as {int}', async function (this: InputDecryptWorld, expected: number) {
  const { values } = await this.engine.fhe.publicDecrypt([await this.result()]);
  assert.deepEqual(values, [BigInt(expected)]);
});
