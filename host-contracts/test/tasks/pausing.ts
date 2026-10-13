import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers, run } from 'hardhat';
import { ACL, ACL__factory } from '../../types';

describe('Pausing and Unpausing Tasks', function () {
  let acl: ACL;

  describe('Hardhat pausing/unpausing tasks', function () {
    before(async function () {
      const aclFactory: ACL__factory = await ethers.getContractFactory('ACL');
      const origACLAdd = dotenv.parse(fs.readFileSync('addresses/.env.host')).ACL_CONTRACT_ADDRESS;
      acl = aclFactory.attach(origACLAdd) as ACL;
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
