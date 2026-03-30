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

  const managedEnvVars = [
    'NUM_PAUSERS',
    'PAUSER_ADDRESS_0',
    'OLD_PAUSER_ADDRESS_0',
    'NEW_PAUSER_ADDRESS_0',
  ] as const;

  const originalEnv = new Map<string, string | undefined>();

  before(async function () {
    const pauserSetFactory = await ethers.getContractFactory('PauserSet');
    const pauserSetAddress = dotenv.parse(fs.readFileSync('addresses/.env.host')).PAUSER_SET_CONTRACT_ADDRESS;
    pauserSet = (await pauserSetFactory.attach(pauserSetAddress)) as PauserSet;
  });

  // These tasks read pauser inputs from process.env at runtime, so restore overrides per test.
  beforeEach(function () {
    for (const envVar of managedEnvVars) {
      originalEnv.set(envVar, process.env[envVar]);
    }
  });

  afterEach(function () {
    for (const envVar of managedEnvVars) {
      const originalValue = originalEnv.get(envVar);
      if (originalValue === undefined) {
        delete process.env[envVar];
      } else {
        process.env[envVar] = originalValue;
      }
    }
    originalEnv.clear();
  });

  it('Should add pausers through the task', async function () {
    const newPauser = '0x0000000000000000000000000000000000000011';

    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    process.env.NUM_PAUSERS = '1';
    process.env.PAUSER_ADDRESS_0 = newPauser;

    await run('task:addHostPausers', { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });

  it('Should remove pausers through the task', async function () {
    const pauserToRemove = '0x0000000000000000000000000000000000000012';
    await pauserSet.connect(deployer).addPauser(pauserToRemove);

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(true);

    process.env.NUM_PAUSERS = '1';
    process.env.PAUSER_ADDRESS_0 = pauserToRemove;

    await run('task:removeHostPausers', { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(pauserToRemove)).to.eq(false);
  });

  it('Should swap pausers through the task', async function () {
    const oldPauser = '0x0000000000000000000000000000000000000013';
    const newPauser = '0x0000000000000000000000000000000000000014';
    await pauserSet.connect(deployer).addPauser(oldPauser);

    expect(await pauserSet.isPauser(oldPauser)).to.eq(true);
    expect(await pauserSet.isPauser(newPauser)).to.eq(false);

    process.env.NUM_PAUSERS = '1';
    process.env.OLD_PAUSER_ADDRESS_0 = oldPauser;
    process.env.NEW_PAUSER_ADDRESS_0 = newPauser;

    await run('task:swapHostPausers', { useInternalProxyAddress: true });

    expect(await pauserSet.isPauser(oldPauser)).to.eq(false);
    expect(await pauserSet.isPauser(newPauser)).to.eq(true);
  });
});
