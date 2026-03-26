import { Wallet } from 'ethers';
import { task, types } from 'hardhat/config';
import { HardhatEthersHelpers } from 'hardhat/types';

import { getRequiredEnvVar, loadHostAddresses } from './utils/loadVariables';

// Helper function to get a host contract and its proxy address
async function getHostContract(
  contractName: string,
  addressEnvVar: string,
  ethers: HardhatEthersHelpers,
  useInternalAddress: boolean,
  envVarPrivateKeyName: string,
) {
  const accountPrivateKey = getRequiredEnvVar(envVarPrivateKeyName);
  const account = new Wallet(accountPrivateKey).connect(ethers.provider);

  if (useInternalAddress) {
    loadHostAddresses();
  }

  const proxyAddress = getRequiredEnvVar(addressEnvVar);
  const contract = await ethers.getContractAt(contractName, proxyAddress, account);

  return { contract, proxyAddress };
}

// Helper function to pause a contract
async function pauseSingleContract(
  contractName: string,
  addressEnvVar: string,
  ethers: HardhatEthersHelpers,
  useInternalAddress: boolean,
) {
  const { contract, proxyAddress } = await getHostContract(
    contractName,
    addressEnvVar,
    ethers,
    useInternalAddress,
    'PAUSER_PRIVATE_KEY',
  );
  await contract.pause();
  console.log(`${contractName} contract successfully paused at address: ${proxyAddress}\n`);
}

// Helper function to unpause a contract
async function unpauseSingleContract(
  contractName: string,
  addressEnvVar: string,
  ethers: HardhatEthersHelpers,
  useInternalAddress: boolean,
) {
  // NOTE: this task won't work once ownership will be transferred from initial deployer to the multisig
  const { contract, proxyAddress } = await getHostContract(
    contractName,
    addressEnvVar,
    ethers,
    useInternalAddress,
    'DEPLOYER_PRIVATE_KEY',
  );
  await contract.unpause();
  console.log(`${contractName} contract successfully unpaused at address: ${proxyAddress}\n`);
}

// Pause the ACL contract
task('task:pauseACL')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    await pauseSingleContract('ACL', 'ACL_CONTRACT_ADDRESS', ethers, useInternalProxyAddress);
  });

// Unpause the ACL contract
task('task:unpauseACL')
  .addOptionalParam(
    'useInternalProxyAddress',
    'If proxy address from the /addresses directory should be used',
    false,
    types.boolean,
  )
  .setAction(async function ({ useInternalProxyAddress }, { ethers }) {
    await unpauseSingleContract('ACL', 'ACL_CONTRACT_ADDRESS', ethers, useInternalProxyAddress);
  });
