import { Wallet } from 'ethers';
import { task, types } from 'hardhat/config';

import { getRequiredEnvVar } from './utils/loadVariables';

task('task:transferHostOwnership', "Transfers the deployer's ownership of the host contracts to the provided address.")
  .addParam('newOwnerAddress', 'Address of the new owner of the host contracts.', undefined, types.string)
  .setAction(async function ({ newOwnerAddress }, { ethers }) {
    // Get the deployer wallet.
    const deployer = new Wallet(getRequiredEnvVar('DEPLOYER_PRIVATE_KEY')).connect(ethers.provider);

    // Get the ACL contract: its owner is the owner of all host contracts
    const contractName = 'ACL';
    const aclContractAddress = getRequiredEnvVar(`${contractName}_CONTRACT_ADDRESS`);
    const aclContract = await ethers.getContractAt(contractName, aclContractAddress);

    if ((await aclContract.owner()) !== deployer.address) {
      throw new Error(
        `The deployer account ${deployer.address} is not the owner of the ACL contract ${aclContractAddress}`,
      );
    }

    // Transfer ownership of the ACL contract to the destination address.
    const tx = await aclContract.connect(deployer).transferOwnership(newOwnerAddress);

    await tx.wait();

    console.log(
      `Ownership of ACL contract ${aclContractAddress} is now successfully pending for account ${newOwnerAddress}.
       The new owner needs to send an acceptOwnership transaction to validate the transfer`,
    );
  });

task(
  'task:acceptHostOwnership',
  'Accepts ownership of the host contracts. This can only be used if the account is an EOA.',
).setAction(async function ({}, { ethers }) {
  // Get the new owner wallet.
  const newOwner = new Wallet(getRequiredEnvVar('NEW_OWNER_PRIVATE_KEY')).connect(ethers.provider);

  // Get the ACL contract: its owner is the owner of all host contracts
  const contractName = 'ACL';
  const aclContractAddress = getRequiredEnvVar(`${contractName}_CONTRACT_ADDRESS`);
  const aclContract = await ethers.getContractAt(contractName, aclContractAddress);

  if ((await aclContract.pendingOwner()) !== newOwner.address) {
    throw new Error(
      `The new owner account ${newOwner.address} is not the pending owner of the ACL contract ${aclContractAddress}`,
    );
  }

  // Accept ownership of the ACL contract.
  const tx = await aclContract.connect(newOwner).acceptOwnership();

  await tx.wait();

  console.log(`Ownership of ACL contract ${aclContractAddress} has been accepted by account ${newOwner.address}`);
});
