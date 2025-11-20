import { expect } from 'chai';
import { ethers } from 'hardhat';

import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

const ENFORCED_PAUSE_SELECTOR = '0xd93c0665';

describe('Paused host', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
  });

  // ACL.allow test.
  it('test paused host user input uint64 (non-trivial)', async function () {
    // Initialize TestInput contract.
    const testInputContractFactory = await ethers.getContractFactory('TestInput');
    const testInputContract = await testInputContractFactory.connect(this.signers.alice).deploy();
    const testInputContractAddress = await testInputContract.getAddress();
    await testInputContract.waitForDeployment();

    const inputAlice = this.instances.alice.createEncryptedInput(testInputContractAddress, this.signers.alice.address);
    inputAlice.add64(18446744073709550042n);
    const encryptedAmount = await inputAlice.encrypt();

    // The requestUint64NonTrivial call should fail because it calls to ACL.allow() which should be paused.
    await expect(
      testInputContract.requestUint64NonTrivial(encryptedAmount.handles[0], encryptedAmount.inputProof),
    ).to.be.rejectedWith(new RegExp(ENFORCED_PAUSE_SELECTOR));
  });

  // ACL.allowForDecryption test.
  it('test paused host HTTPPublicDecrypt', async function () {
    // Initialize HTTPPublicDecrypt contract.
    const httpPublicDecryptContractFactory = await ethers.getContractFactory('HTTPPublicDecrypt');

    // The HTTPPublicDecrypt contract deployment should fail because its constructor
    // makes a call to ACL.allowForDecryption() which should be paused.
    await expect(httpPublicDecryptContractFactory.connect(this.signers.alice).deploy()).to.be.rejectedWith(
      new RegExp(ENFORCED_PAUSE_SELECTOR),
    );
  });
});
