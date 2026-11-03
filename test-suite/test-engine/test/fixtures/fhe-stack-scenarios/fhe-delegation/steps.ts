import { Given, Then, When, setWorldConstructor } from '@cucumber/cucumber';
import { Contract } from 'ethers';
import assert from 'node:assert/strict';

import { FheFixtureWorld } from '../shared.js';

const ACL_ABI = [
  'function delegateForUserDecryption(address delegate, address contractAddress, uint64 expirationDate)',
];
/** Host-chain blocks the coprocessor needs to absorb an ACL change (the e2e suite's value). */
const PROPAGATION_BLOCKS = 15;
const ONE_DAY_SECONDS = 24 * 3600;

setWorldConstructor(FheFixtureWorld);

Given('alice has deployed the TestInput contract', async function (this: FheFixtureWorld) {
  await this.deploy('TestInput', 'alice');
});

Given(
  'alice has stored the encrypted uint64 value {int} plus 42',
  async function (this: FheFixtureWorld, value: number) {
    const { handles, inputProof } = await this.engine.fhe.encrypt({
      contract: this.deployed,
      user: 'alice',
      values: [{ type: 'uint64', value: BigInt(value) }],
    });
    const alice = this.engine.chain.wallet('alice');
    await this.engine.chain.send(this.deployed.connect(alice).getFunction('add42ToInput64')(handles[0], inputProof), {
      label: 'add42ToInput64',
    });
  },
);

When('alice delegates user decryption on the contract to bob', async function (this: FheFixtureWorld) {
  const { chain } = this.engine;
  // The engine gives no API for the contracts section yet; a fixture may read the resolved profile.
  const acl = new Contract(this.parameters.profile.contracts!.acl.value, ACL_ABI, chain.wallet('alice'));
  const expiration = Math.floor(Date.now() / 1000) + ONE_DAY_SECONDS;
  const receipt = await chain.send(
    acl.getFunction('delegateForUserDecryption')(
      chain.wallet('bob').address,
      await this.deployed.getAddress(),
      expiration,
    ),
    { label: 'alice delegates bob' },
  );
  await chain.waitForBlock(receipt.blockNumber + PROPAGATION_BLOCKS);
});

Then("bob decrypts alice's result as {int} as her delegate", async function (this: FheFixtureWorld, expected: number) {
  const value = await this.engine.fhe.delegatedUserDecrypt({
    handle: await this.handle('resUint64'),
    contract: this.deployed,
    delegate: 'bob',
    delegator: 'alice',
  });
  assert.equal(value, BigInt(expected));
});
