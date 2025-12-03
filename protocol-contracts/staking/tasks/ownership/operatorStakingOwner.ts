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
    const { ethers, network, getNamedAccounts } = hre;

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
    const tx = await operatorStaking.transferOwnership(DAO_ADDRESS);
    await tx.wait();

    console.log(
      [
        `🔑 Transferred ownership of OperatorStaking contract:`,
        `  - Operator staking address:   ${operatorStakingAddress}`,
        `  - New owner (DAO):        ${DAO_ADDRESS}`,
        `  - Initiated by owner (deployer): ${deployer}`,
        `  - Network:           ${network.name}`,
        '',
      ].join('\n'),
    );
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
    const { ethers, network, getNamedAccounts } = hre;

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
    const tx = await operatorRewarder.transferOwnership(DAO_ADDRESS);
    await tx.wait();

    console.log(
      [
        `🔑 Transferred ownership of OperatorRewarder contract:`,
        `  - Operator rewarder address: ${operatorRewarderAddress}`,
        `  - New owner (DAO):  ${DAO_ADDRESS}`,
        `  - Initiated by owner (deployer): ${deployer}`,
        `  - Network: ${network.name}`,
        '',
      ].join('\n'),
    );
  });

// Transfer the all operator staking and rewarder contracts' ownerships from the deployer to the DAO
// Example usage:
// npx hardhat task:transferAllOperatorStakingRewarderOwnershipsToDAO --network testnet
task('task:transferAllOperatorStakingRewarderOwnershipsToDAO').setAction(async function (
  _,
  hre: HardhatRuntimeEnvironment,
) {
  console.log('Transferring ownership of all operator staking contracts to the DAO...\n');

  // Get the addresses of all operator staking contracts
  const operatorStakingAddresses = await getAllOperatorStakingAddresses(hre);

  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    await hre.run('task:transferOperatorStakingOwnershipToDAO', {
      operatorStakingAddress: operatorStakingAddresses[i],
    });
  }

  console.log('Transferring ownership of all operator rewarder contracts to the DAO...\n');

  // Get the addresses of all operator rewarder contracts
  const operatorRewarderAddresses = await getAllOperatorRewarderAddresses(hre);

  for (let i = 0; i < operatorRewarderAddresses.length; i++) {
    await hre.run('task:transferOperatorRewarderOwnershipToDAO', {
      operatorRewarderAddress: operatorRewarderAddresses[i],
    });
  }

  console.log('✅ Ownership of all operator staking and rewarder contracts have been transferred to the DAO');
});
