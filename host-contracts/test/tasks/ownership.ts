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

  // Get the private key of the new owner
  const newOwnerPrivateKey = getRequiredEnvVar('NEW_OWNER_PRIVATE_KEY');
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

  it('Should accept ownership of the ACL contract', async function () {
    await run('task:acceptHostOwnership');

    // Check that the ownership has been transferred to the new owner.
    expect(await acl.owner()).to.eq(newOwner.address);
  });

  // This is to avoid to break other tests
  it('Should put back ownership of the ACL contract to the deployer', async function () {
    // Temporarily swap the deployer and the new owner private keys
    process.env.DEPLOYER_PRIVATE_KEY = newOwnerPrivateKey;
    process.env.NEW_OWNER_PRIVATE_KEY = deployerPrivateKey;

    // Transfer ownership to the deployer and accept it
    await run('task:transferHostOwnership', {
      newOwnerAddress: deployer.address,
    });
    await run('task:acceptHostOwnership');

    // Check that the ownership has been transferred back to the deployer.
    expect(await acl.owner()).to.eq(deployer.address);

    // Restore the original private keys
    process.env.DEPLOYER_PRIVATE_KEY = deployerPrivateKey;
    process.env.NEW_OWNER_PRIVATE_KEY = newOwnerPrivateKey;
  });
});
