import { expect } from 'chai';
import { ethers } from 'hardhat';

import { initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('InputVerifier.coprocessor', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const inputVerifierFactory = await ethers.getContractFactory('contracts/InputVerifier.native.sol:InputVerifier');
    await initDecryptionOracle();
    const inputVerifier = await inputVerifierFactory.deploy();
    await inputVerifier.waitForDeployment();
    this.inputVerifier = inputVerifier;
  });

  it('cannot initialize if not initializer', async function () {
    const randomAccount = this.signers.carol;

    await expect(this.inputVerifier.connect(randomAccount).initialize(randomAccount)).to.be.revertedWithCustomError(
      this.inputVerifier,
      'InvalidInitialization',
    );
  });
});
