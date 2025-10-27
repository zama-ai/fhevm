import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers, run } from 'hardhat';

import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { ACL } from '../../types';

describe('Ownership tasks', function () {
  let acl: ACL;

  // Get the owner wallet (the deployer)
  const deployerPrivateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const deployer = new ethers.Wallet(deployerPrivateKey).connect(ethers.provider);

  // Define the private key of the new owner (Account 2)
  const newOwnerPrivateKey = '0x7ae52cf0d3011ef7fecbe22d9537aeda1a9e42a0596e8def5d49970eb59e7a40';
  const newOwner = new ethers.Wallet(newOwnerPrivateKey).connect(ethers.provider);

  before(async function () {
    let aclFactory = await ethers.getContractFactory('ACL');
    const aclAddress = dotenv.parse(fs.readFileSync('addresses/.env.host')).ACL_CONTRACT_ADDRESS;
    acl = (await aclFactory.attach(aclAddress)) as ACL;
  });

  it('Should ask transfer ownership of the ACL contract', async function () {
    expect(await acl.owner()).to.eq(deployer.address);

    await run('task:transferHostOwnership', {
      newOwnerAddress: newOwner.address,
    });

    // Check that the ownership has not been transferred as the transfer is only pending since the
    // new owner has not accepted it yet.
    expect(await acl.owner()).to.eq(deployer.address);

    // Check that the pending owner is the new owner.
    expect(await acl.pendingOwner()).to.eq(newOwner.address);
  });
});
