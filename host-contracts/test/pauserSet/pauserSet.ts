import { expect } from 'chai';
import dotenv from 'dotenv';
import { Wallet } from 'ethers';
import fs from 'fs';
import { ethers } from 'hardhat';

import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { createInstances } from '../instance';
import { getSigners, initSigners } from '../signers';

describe('PauserSet', function () {
  before(async function () {
    await initSigners(2);
    this.signers = await getSigners();
    this.instances = await createInstances(this.signers);
    this.pauserSetFactory = await ethers.getContractFactory('PauserSet');
    const origPauserSetAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).PAUSER_SET_CONTRACT_ADDRESS;
    this.pauserSet = await this.pauserSetFactory.attach(origPauserSetAdd);
    this.aclFactory = await ethers.getContractFactory('ACL');
    const origACLAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).ACL_CONTRACT_ADDRESS;
    this.acl = await this.aclFactory.attach(origACLAdd);
    this.pauser = new Wallet(getRequiredEnvVar('PAUSER_PRIVATE_KEY'), ethers.provider);
    this.deployer = new Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY'), ethers.provider);
  });

  it('Should return true for the initial pauser address', async function () {
    expect(await this.acl.isPauser(this.pauser.address)).to.equal(true);
  });

  it('Should revert because the sender is not the owner', async function () {
    const fakeAccount = '0x0000000000000000000000000000000000000001';
    await expect(this.pauserSet.connect(this.pauser).addPauser(fakeAccount))
      .to.be.revertedWithCustomError(this.pauserSet, 'NotHostOwner')
      .withArgs(this.pauser.address);
    await expect(this.pauserSet.connect(this.pauser).removePauser(this.pauser.address))
      .to.be.revertedWithCustomError(this.pauserSet, 'NotHostOwner')
      .withArgs(this.pauser.address);
    await expect(this.pauserSet.connect(this.pauser).swapPauser(this.pauser.address, fakeAccount))
      .to.be.revertedWithCustomError(this.pauserSet, 'NotHostOwner')
      .withArgs(this.pauser.address);
  });

  it('Should add the pauser', async function () {
    const newPauser = '0x0000000000000000000000000000000000000001';

    const tx = await this.pauserSet.connect(this.deployer).addPauser(newPauser);

    await expect(tx).to.emit(this.pauserSet, 'AddPauser').withArgs(newPauser);
  });

  it('Should revert when adding an already added pauser', async function () {
    await expect(this.pauserSet.connect(this.deployer).addPauser(this.pauser.address))
      .to.be.revertedWithCustomError(this.pauserSet, 'AccountAlreadyPauser')
      .withArgs(this.pauser.address);
  });

  it('Should revert when removing a non-pauser', async function () {
    const newNotPauser = '0x0000000000000000000000000000000000000002';
    await expect(this.pauserSet.connect(this.deployer).removePauser(newNotPauser))
      .to.be.revertedWithCustomError(this.pauserSet, 'AccountNotPauser')
      .withArgs(newNotPauser);
  });

  it('Should remove when removing a pauser', async function () {
    const newPauser = '0x0000000000000000000000000000000000000003';
    await this.pauserSet.connect(this.deployer).addPauser(newPauser);
    const tx = await this.pauserSet.connect(this.deployer).removePauser(newPauser);

    await expect(tx).to.emit(this.pauserSet, 'RemovePauser').withArgs(newPauser);
  });

  it('Should revert because the pauser is the null address', async function () {
    const nullPauser = ethers.ZeroAddress;

    await expect(this.pauserSet.connect(this.deployer).addPauser(nullPauser)).to.be.revertedWithCustomError(
      this.pauserSet,
      'InvalidNullPauser',
    );
  });

  it('Should swap the pauser', async function () {
    const oldPauser = '0x0000000000000000000000000000000000000003';
    await this.pauserSet.connect(this.deployer).addPauser(oldPauser);
    const newPauser = '0x0000000000000000000000000000000000000004';
    const tx = await this.pauserSet.connect(this.deployer).swapPauser(oldPauser, newPauser);
    await expect(tx).to.emit(this.pauserSet, 'SwapPauser').withArgs(oldPauser, newPauser);
    expect(await this.pauserSet.isPauser(oldPauser)).to.be.false;
    expect(await this.pauserSet.isPauser(newPauser)).to.be.true;
  });

  it('Should revert swappig the pauser', async function () {
    const newPauser = '0x0000000000000000000000000000000000000005';
    await expect(this.pauserSet.connect(this.deployer).swapPauser(newPauser, newPauser))
      .to.be.revertedWithCustomError(this.pauserSet, 'AccountNotPauser')
      .withArgs(newPauser);
    await expect(this.pauserSet.connect(this.deployer).swapPauser(this.pauser.address, this.pauser.address))
      .to.be.revertedWithCustomError(this.pauserSet, 'AccountAlreadyPauser')
      .withArgs(this.pauser.address);
    const nullPauser = ethers.ZeroAddress;
    await expect(this.pauserSet.connect(this.deployer).swapPauser(nullPauser, newPauser)).to.be.revertedWithCustomError(
      this.pauserSet,
      'InvalidNullPauser',
    );
    await expect(
      this.pauserSet.connect(this.deployer).swapPauser(this.pauser.address, nullPauser),
    ).to.be.revertedWithCustomError(this.pauserSet, 'InvalidNullPauser');
  });
});
