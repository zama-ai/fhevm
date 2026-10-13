import { assert, expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

// Sustained-load SDK check: whenever the coprocessor set or the KMS context
// changes (new signers, new threshold, priority toggled on/off), the public
// SDK must keep servicing input, user-decrypt and public-decrypt round-trips.
// The loop exercises the three paths back-to-back for `SDK_LOAD_ITERATIONS`
// cycles so a single stalled signer or a flaky quorum shows up as a failed
// cycle instead of a lucky single-shot pass.
const ITERATIONS = Number(process.env.SDK_LOAD_ITERATIONS ?? 5);

describe('sdk sustained load', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const contractFactory = await ethers.getContractFactory('TestInput');
    this.contract = await contractFactory.connect(this.signers.alice).deploy();
    await this.contract.waitForDeployment();
    this.contractAddress = await this.contract.getAddress();
  });

  for (let i = 0; i < ITERATIONS; i++) {
    const iteration = i + 1;
    const inputValue = BigInt(1_000_000 + i);
    const expectedResult = inputValue + 42n;

    it(`iteration ${iteration}/${ITERATIONS}: encrypt input, user decrypt, public decrypt`, async function () {
      const started = Date.now();
      console.log(`[sdk-load] iteration ${iteration}: input=${inputValue}`);

      const encryptedInput = await this.instances.alice.encryptUint64({
        value: inputValue,
        contractAddress: this.contractAddress,
        userAddress: this.signers.alice.address,
      });

      const tx = await this.contract.add42ToInput64(encryptedInput.handles[0], encryptedInput.inputProof);
      const receipt = await tx.wait();
      expect(receipt.status).to.equal(1);

      const handle = await this.contract.resUint64();

      const userDecrypted = await this.instances.alice.userDecryptSingleHandle({
        handle,
        contractAddress: this.contractAddress,
        signer: this.signers.alice,
      });
      expect(userDecrypted).to.equal(expectedResult);

      const publicDecrypted = await this.instances.alice.publicDecrypt([handle]);
      assert.deepEqual(publicDecrypted.clearValues, { [handle]: expectedResult });

      console.log(`[sdk-load] iteration ${iteration}: completed in ${Date.now() - started}ms`);
    });
  }
});
