import { expect } from 'chai';
import dotenv from 'dotenv';
import fs from 'fs';
import { ethers, run } from 'hardhat';

import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { ACL } from '../../types';

describe('Ownership tasks', function () {
  let acl: ACL;

  // Get the owner wallet (the deployer)
  const privateKey = getRequiredEnvVar('DEPLOYER_PRIVATE_KEY');
  const initialOwner = new ethers.Wallet(privateKey).connect(ethers.provider);

  // Define the private key of the new owner (Account 2)
  const newOwnerPrivateKey = '0x7ae52cf0d3011ef7fecbe22d9537aeda1a9e42a0596e8def5d49970eb59e7a40';
  const newOwner = new ethers.Wallet(newOwnerPrivateKey).connect(ethers.provider);

  before(async function () {
    let aclFactory = await ethers.getContractFactory('ACL');
    const aclAddress = dotenv.parse(fs.readFileSync('addresses/.env.host')).ACL_CONTRACT_ADDRESS;
    acl = (await aclFactory.attach(aclAddress)) as ACL;
  });

  it('Should ask transfer ownership of the ACL contract', async function () {
    expect(await acl.owner()).to.eq(initialOwner.address);

    await run('task:transferHostOwnership', {
      currentOwnerPrivateKey: initialOwner.privateKey,
      newOwnerAddress: newOwner.address,
    });

    // Check that the ownership has not been transferred as the new owner has not accepted it yet
    expect(await acl.owner()).to.eq(initialOwner.address);
  });

  it('Should accept ownership of the ACL contract', async function () {
    await run('task:acceptHostOwnership', { newOwnerPrivateKey: newOwner.privateKey });
    expect(await acl.owner()).to.eq(newOwner.address);
  });

  it('Should give ownership of the ACL contract back to the original owner', async function () {
    await run('task:transferHostOwnership', {
      currentOwnerPrivateKey: newOwner.privateKey,
      newOwnerAddress: initialOwner.address,
    });

    await run('task:acceptHostOwnership', { newOwnerPrivateKey: initialOwner.privateKey });
    expect(await acl.owner()).to.eq(initialOwner.address);
  });
});
