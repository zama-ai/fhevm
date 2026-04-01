import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers, run } from 'hardhat';

import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { PauserSet } from '../../types';

describe('Pauser tasks', function () {
  let pauserSet: PauserSet;

  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);

  before(async function () {
    const pauserSetFactory = await ethers.getContractFactory('PauserSet');
    const pauserSetAddress = dotenv.parse(fs.readFileSync('addresses/.env.host')).PAUSER_SET_CONTRACT_ADDRESS;
    pauserSet = (await pauserSetFactory.attach(pauserSetAddress)) as PauserSet;
  });

  it('Should add pausers through the task', async function () {
    const newPauser = '0x0000000000000000000000000000000000000011';

    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    process.env.NUM_PAUSERS = '1';
    process.env.PAUSER_ADDRESS_0 = newPauser;

    await run('task:addHostPausers', { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });

  it('Should remove a pauser through the task', async function () {
    const pauserToRemove = '0x0000000000000000000000000000000000000012';
    await pauserSet.connect(deployer).addPauser(pauserToRemove);

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(true);

    await run('task:removeHostPauser', {
      useInternalProxyAddress: true,
      pauserAddress: pauserToRemove,
    });

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(false);
  });

  it('Should swap a pauser through the task', async function () {
    const oldPauser = '0x0000000000000000000000000000000000000013';
    const newPauser = '0x0000000000000000000000000000000000000014';
    await pauserSet.connect(deployer).addPauser(oldPauser);

    expect(await pauserSet.isPauser(oldPauser)).to.eq(true);
    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    await run('task:swapHostPauser', {
      useInternalProxyAddress: true,
      oldPauserAddress: oldPauser,
      newPauserAddress: newPauser,
    });

    expect(await pauserSet.isPauser(oldPauser)).to.eq(false);
    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });
});
