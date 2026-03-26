import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers, run } from 'hardhat';

describe('Pausing and Unpausing Tasks', function () {
  let pauserSet;
  let acl;

  describe('Hardhat pausing/unpausing tasks', function () {
    before(async function () {
      let pauserSetFactory = await ethers.getContractFactory('PauserSet');
      let aclFactory = await ethers.getContractFactory('ACL');
      const origPauserSetAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).PAUSER_SET_CONTRACT_ADDRESS;
      const origACLAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).ACL_CONTRACT_ADDRESS;
      pauserSet = await pauserSetFactory.attach(origPauserSetAdd);
      acl = await aclFactory.attach(origACLAdd);
    });

    it('Should pause acl', async function () {
      expect(await acl.paused()).to.eq(false);
      await run('task:pauseACL', { useInternalProxyAddress: true });
      expect(await acl.paused()).to.eq(true);
    });

    it('Should unpause acl', async function () {
      expect(await acl.paused()).to.eq(true);
      await run('task:unpauseACL', { useInternalProxyAddress: true });
      expect(await acl.paused()).to.eq(false);
    });
  });
});
