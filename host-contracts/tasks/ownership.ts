import { Wallet } from 'ethers';
import { task, types } from 'hardhat/config';

import { getRequiredEnvVar } from './utils/loadVariables';

task(
  'task:transferHostOwnership',
  'Transfers ownership of the host contracts to the provided address. This can only be used if the current owner is an EOA.',
)
  .addParam(
    'currentOwnerPrivateKey',
    'Private key of the current owner of the host contracts.',
    undefined,
    types.string,
  )
  .addParam('newOwnerAddress', 'Address of the new owner of the host contracts.', undefined, types.string)
  .setAction(async function ({ currentOwnerPrivateKey, newOwnerAddress }, { ethers }) {
    // Get the current owner wallet.
    const currentOwner = new Wallet(currentOwnerPrivateKey).connect(ethers.provider);

    // Get the ACL contract: its owner is the owner of all host contracts
    const contractName = 'ACL';
    const aclContractAddress = getRequiredEnvVar(`${contractName}_CONTRACT_ADDRESS`);
    const aclContract = await ethers.getContractAt(contractName, aclContractAddress);

    // Transfer ownership of the ACL contract to the destination address.
    const tx = await aclContract.connect(currentOwner).transferOwnership(newOwnerAddress);

    await tx.wait();

    console.log(`Ownership of ACL contract ${aclContractAddress} successfully transferred to EOA ${newOwnerAddress}`);
  });

task(
  'task:acceptHostOwnership',
  `Accepts ownership of the host contracts. This can only be used if the new owner is an EOA.`,
)
  .addParam(
    'newOwnerPrivateKey',
    'Private key of the new owner that will accept the ownership of the host contracts.',
    undefined,
    types.string,
  )
  .setAction(async function ({ newOwnerPrivateKey }, { ethers }) {
    // Get the new owner wallet.
    const newOwner = new Wallet(newOwnerPrivateKey).connect(ethers.provider);

    // Get the ACL contract: its owner is the owner of all host contracts
    const contractName = 'ACL';
    const aclContractAddress = getRequiredEnvVar(`${contractName}_CONTRACT_ADDRESS`);
    const aclContract = await ethers.getContractAt('ACL', aclContractAddress);

    // Accept the ownership of the ACL contract.
    const tx = await aclContract.connect(newOwner).acceptOwnership();

    await tx.wait();

    console.log(`Ownership of ACL contract ${aclContractAddress} successfully accepted by EOA ${newOwner.address}`);
  });
