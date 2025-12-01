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

  // The following test case should cover the ACL.allow method call.
  it('test paused host user input (allow)', async function () {
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

  // The following test case should cover the ACL.allowForDecryption method call.
  it('test paused host HTTP public decrypt (allow for decryption)', async function () {
    // Initialize HTTPPublicDecrypt contract.
    const httpPublicDecryptContractFactory = await ethers.getContractFactory('HTTPPublicDecrypt');

    // The HTTPPublicDecrypt contract deployment should fail because its constructor
    // makes a call to ACL.allowForDecryption() which should be paused.
    await expect(httpPublicDecryptContractFactory.connect(this.signers.alice).deploy()).to.be.rejectedWith(
      new RegExp(ENFORCED_PAUSE_SELECTOR),
    );
  });

  // The following test case should cover the ACL.allowTransient method call.
  it('test paused host operators (allow transient)', async function () {
    const fhevmTestSuite1ContractFactory = await ethers.getContractFactory('FHEVMTestSuite1');
    const fhevmTestSuite1Contract = await fhevmTestSuite1ContractFactory.connect(this.signers.alice).deploy();
    const fhevmTestSuite1ContractAddress = await fhevmTestSuite1Contract.getAddress();
    await fhevmTestSuite1Contract.waitForDeployment();

    const input = this.instances.alice.createEncryptedInput(fhevmTestSuite1ContractAddress, this.signers.alice.address);
    input.add32(1488611147n);
    input.add64(1488611147n);
    const encryptedAmount = await input.encrypt();

    // The sub_euint32_euint64 call should fail because it calls to ACL.allowTransient() which should be paused.
    await expect(
      fhevmTestSuite1Contract.sub_euint32_euint64(
        encryptedAmount.handles[0],
        encryptedAmount.handles[1],
        encryptedAmount.inputProof,
      ),
    ).to.be.rejectedWith(new RegExp(ENFORCED_PAUSE_SELECTOR));
  });
});
