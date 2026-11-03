import type { PublicDecryption } from '#engine';
import { Given, Then, When, setWorldConstructor } from '@cucumber/cucumber';
import { AbiCoder, Contract } from 'ethers';
import assert from 'node:assert/strict';

import { FheFixtureWorld } from '../shared.js';

/** The call `FHE.checkSignatures` makes on the KMS verifier. */
const KMS_VERIFIER_ABI = [
  'function verifyDecryptionEIP712KMSSignatures(bytes32[] handlesList, bytes decryptedResult, bytes decryptionProof) returns (bool)',
];

class MultiInputWorld extends FheFixtureWorld {
  handleOfResult: string | undefined;
  decryption: PublicDecryption | undefined;

  /** Whether the KMS verifier accepts `cleartexts` with the decryption proof (a revert counts as no). */
  async verified(cleartexts: string): Promise<boolean> {
    // The engine gives no API for the contracts section yet; a fixture may read the resolved profile.
    const address = this.parameters.profile.contracts!.kmsVerifier.value;
    const verifier = new Contract(address, KMS_VERIFIER_ABI, this.engine.chain.provider);
    try {
      return Boolean(
        await verifier
          .getFunction('verifyDecryptionEIP712KMSSignatures')
          .staticCall([this.handleOfResult], cleartexts, this.decryption!.decryptionProof),
      );
    } catch {
      return false;
    }
  }
}
setWorldConstructor(MultiInputWorld);

Given('alice has deployed the FHEVMTestSuite4 contract', async function (this: MultiInputWorld) {
  await this.deploy('FHEVMTestSuite4', 'alice');
});

When(
  'alice encrypts the uint64 value {int} and the uint32 value {int} in one input',
  async function (this: MultiInputWorld, a: number, b: number) {
    this.input = await this.engine.fhe.encrypt({
      contract: this.deployed,
      user: 'alice',
      values: [
        { type: 'uint64', value: BigInt(a) },
        { type: 'uint32', value: BigInt(b) },
      ],
    });
  },
);

When('alice calls and_euint64_euint32 with them', async function (this: MultiInputWorld) {
  const { handles, inputProof } = this.encrypted;
  assert.equal(handles.length, 2);
  const alice = this.engine.chain.wallet('alice');
  await this.engine.chain.send(
    this.deployed.connect(alice).getFunction('and_euint64_euint32')(handles[0], handles[1], inputProof),
    { label: 'and_euint64_euint32' },
  );
});

Then('the uint64 result is publicly decryptable as {int}', async function (this: MultiInputWorld, expected: number) {
  this.handleOfResult = await this.handle('resEuint64');
  this.decryption = await this.engine.fhe.publicDecrypt([this.handleOfResult]);
  assert.deepEqual(this.decryption.values, [BigInt(expected)]);
});

Then('the KMS verifier accepts the decryption proof', async function (this: MultiInputWorld) {
  assert.equal(await this.verified(this.decryption!.abiEncodedClearValues), true);
});

Then(
  'the KMS verifier rejects the decryption proof for the value {int}',
  async function (this: MultiInputWorld, other: number) {
    const tampered = AbiCoder.defaultAbiCoder().encode(['uint256'], [BigInt(other)]);
    assert.equal(await this.verified(tampered), false);
  },
);
