import { expect } from 'chai';
import { ethers } from 'hardhat';

import { initDecryptionOracle } from '../asyncDecrypt';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('ACL', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    const aclFactory = await ethers.getContractFactory('ACL');
    await initDecryptionOracle();
    const acl = await aclFactory.deploy();
    await acl.waitForDeployment();
    this.acl = acl;
    this.tfheAddress = await acl.getTFHEExecutorAddress();

    const amountToDistribute = BigInt(100 * 1e24);
    await ethers.provider.send('hardhat_impersonateAccount', [this.tfheAddress]);
    await ethers.provider.send('hardhat_setBalance', [this.tfheAddress, '0x' + amountToDistribute.toString(16)]);
    this.tfheExecutor = await ethers.getSigner(this.tfheAddress);
  });

  it('allowTransient() is not persistent', async function () {
    const randomHandle = 3290232n;
    const randomAccount = this.signers.bob.address;
    await this.acl.connect(this.tfheExecutor).allowTransient(randomHandle, randomAccount);

    /// @dev The isAllowed returns false since it is transient.
    expect(await this.acl.isAllowed(randomHandle, randomAccount)).to.be.eq(false);

    /// @dev The isAllowed returns false since it is transient.
    expect(await this.acl.allowedTransient(randomHandle, randomAccount)).to.be.eq(false);
  });

  it('allowTransient() reverts if sender is not allowed', async function () {
    const randomHandle = 3290232n;
    const randomAccount = this.signers.alice.address;
    const sender = this.signers.alice;

    await expect(this.acl.connect(sender).allowTransient(randomHandle, randomAccount))
      .to.be.revertedWithCustomError(this.acl, 'SenderNotAllowed')
      .withArgs(sender);
  });

  it('allow() reverts if sender is not allowed', async function () {
    const randomHandle = 3290232n;
    const randomAccount = this.signers.alice.address;
    const sender = this.signers.alice;

    await expect(this.acl.connect(sender).allow(randomHandle, randomAccount))
      .to.be.revertedWithCustomError(this.acl, 'SenderNotAllowed')
      .withArgs(sender);
  });
});
