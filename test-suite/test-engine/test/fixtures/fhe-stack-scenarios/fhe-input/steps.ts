import { FheError } from '#engine';
import { Given, Then, When, setWorldConstructor } from '@cucumber/cucumber';
import assert from 'node:assert/strict';

import { FheFixtureWorld } from '../shared.js';

setWorldConstructor(FheFixtureWorld);

Given('alice has deployed the TestInput contract', async function (this: FheFixtureWorld) {
  await this.deploy('TestInput', 'alice');
});

When('alice encrypts the uint64 value {int} for the contract', async function (this: FheFixtureWorld, value: number) {
  this.input = await this.engine.fhe.encrypt({
    contract: this.deployed,
    user: 'alice',
    values: [{ type: 'uint64', value: BigInt(value) }],
  });
});

When('alice calls add42ToInput64 with the encrypted input', async function (this: FheFixtureWorld) {
  const { handles, inputProof } = this.encrypted;
  const alice = this.engine.chain.wallet('alice');
  await this.engine.chain.send(this.deployed.connect(alice).getFunction('add42ToInput64')(handles[0], inputProof), {
    label: 'add42ToInput64',
  });
});

Then(
  /^alice user-decrypts the result as (\d+)(?: again, with the cached permit)?$/,
  async function (this: FheFixtureWorld, expected: string) {
    const value = await this.engine.fhe.userDecrypt({
      handle: await this.handle('resUint64'),
      contract: this.deployed,
      user: 'alice',
    });
    assert.equal(value, BigInt(expected));
  },
);

Then(
  'alice user-decrypts the result as {int} with a unified permit',
  async function (this: FheFixtureWorld, expected: number) {
    const value = await this.engine.fhe.userDecrypt({
      handle: await this.handle('resUint64'),
      contract: this.deployed,
      user: 'alice',
      permit: 'unified',
    });
    assert.equal(value, BigInt(expected));
  },
);

Then('the result is publicly decryptable as {int}', async function (this: FheFixtureWorld, expected: number) {
  const { values } = await this.engine.fhe.publicDecrypt([await this.handle('resUint64')]);
  assert.deepEqual(values, [BigInt(expected)]);
});

Then('carol is not authorized to user-decrypt the result', async function (this: FheFixtureWorld) {
  const handle = await this.handle('resUint64');
  await assert.rejects(
    this.engine.fhe.userDecrypt({ handle, contract: this.deployed, user: 'carol' }),
    (error: unknown) => error instanceof FheError && /is not authorized to decrypt handle/.test(error.message),
  );
});
