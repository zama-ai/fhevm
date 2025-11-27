import { OPERATOR_STAKING_CONTRACT_NAME, OPERATOR_REWARDER_CONTRACT_NAME } from '../deployment';
import { getAllOperatorStakingAddresses, getAllOperatorRewarderAddresses } from '../utils/getAddresses';
import { getRequiredEnvVar } from '../utils/loadVariables';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Transfer the operator staking contract's ownership from the deployer to the DAO
// Example usage:
// npx hardhat task:transferOperatorStakingOwnershipToDAO --network testnet
task('task:transferOperatorStakingOwnershipToDAO')
  .addParam(
    'operatorStakingAddress',
    'The address of the operator staking contract to transfer the owner role from',
    '',
    types.string,
  )
  .setAction(async function ({ operatorStakingAddress }, hre: HardhatRuntimeEnvironment) {
    const { ethers, deployments, network, getNamedAccounts } = hre;
    const { log } = deployments;

    // Get the deployer account
    const { deployer } = await getNamedAccounts();
    const deployerSigner = await ethers.getSigner(deployer);

    // Load the operator staking contract
    const operatorStaking = await ethers.getContractAt(
      OPERATOR_STAKING_CONTRACT_NAME,
      operatorStakingAddress,
      deployerSigner,
    );

    // Get the DAO address
    const DAO_ADDRESS = getRequiredEnvVar('DAO_ADDRESS');

    // Transfer the operator staking contract's ownership to the DAO
    await operatorStaking.transferOwnership(DAO_ADDRESS);

    log(`Operator staking contract at address ${operatorStakingAddress} 
      ownership transferred to DAO address ${DAO_ADDRESS} on network ${network.name}`);
  });

// Transfer the operator rewarder contract's ownership from the deployer to the DAO
// Example usage:
// npx hardhat task:transferOperatorRewarderOwnershipToDAO --network testnet
task('task:transferOperatorRewarderOwnershipToDAO')
  .addParam(
    'operatorRewarderAddress',
    'The address of the operator rewarder contract to transfer the owner role from',
    '',
    types.string,
  )
  .setAction(async function ({ operatorRewarderAddress }, hre: HardhatRuntimeEnvironment) {
    const { ethers, deployments, network, getNamedAccounts } = hre;
    const { log } = deployments;

    // Get the deployer account
    const { deployer } = await getNamedAccounts();
    const deployerSigner = await ethers.getSigner(deployer);

    // Load the operator rewarder contract
    const operatorRewarder = await ethers.getContractAt(
      OPERATOR_REWARDER_CONTRACT_NAME,
      operatorRewarderAddress,
      deployerSigner,
    );

    // Get the DAO address
    const DAO_ADDRESS = getRequiredEnvVar('DAO_ADDRESS');

    // Transfer the operator rewarder contract's ownership to the DAO
    await operatorRewarder.transferOwnership(DAO_ADDRESS);

    log(`Operator rewarder contract at address ${operatorRewarderAddress} 
      ownership transferred to DAO address ${DAO_ADDRESS} on network ${network.name}`);
  });

// Transfer the all operator staking and rewarder contracts' ownerships from the deployer to the DAO
// Example usage:
// npx hardhat task:transferAllOperatorStakingRewarderOwnershipsToDAO --network testnet
task('task:transferAllOperatorStakingRewarderOwnershipsToDAO').setAction(async function (
  _,
  hre: HardhatRuntimeEnvironment,
) {
  const { log } = hre.deployments;

  log('Transferring ownership of all operator staking contracts to the DAO...');

  // Get the addresses of all operator staking contracts
  const operatorStakingAddresses = await getAllOperatorStakingAddresses(hre);

  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    await hre.run('task:transferOperatorStakingOwnershipToDAO', {
      operatorStakingAddress: operatorStakingAddresses[i],
    });
  }

  log('Transferring ownership of all operator rewarder contracts to the DAO...');

  // Get the addresses of all operator rewarder contracts
  const operatorRewarderAddresses = await getAllOperatorRewarderAddresses(hre);

  for (let i = 0; i < operatorRewarderAddresses.length; i++) {
    await hre.run('task:transferOperatorRewarderOwnershipToDAO', {
      operatorRewarderAddress: operatorRewarderAddresses[i],
    });
  }

  log('Ownership of all operator staking and rewarder contracts have been transferred to the DAO');
});
