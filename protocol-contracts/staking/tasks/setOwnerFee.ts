import { OPERATOR_REWARDER_CONTRACT_NAME } from './deployment';
import { getAllOperatorRewarderAddresses } from './utils/getAddresses';
import { getRequiredEnvVar } from './utils/loadVariables';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Set the owner fee for the operator rewarder contract
// Note: The owner fee is in basis points (in 1/100th of a percent, so 10000 = 100.00%)
// Example usage:
// npx hardhat task:setOwnerFee --ownerFee 2000 --operatorRewarderAddress 0x1234567890123456789012345678901234567890 --network testnet
task('task:setOwnerFee')
  .addParam(
    'ownerFee',
    'The owner fee to set for the operator rewarder contract (in 1/100th of a percent)',
    0n,
    types.bigint,
  )
  .addParam(
    'operatorRewarderAddress',
    'The address of the operator rewarder contract to set the owner fee for',
    '',
    types.string,
  )
  .setAction(async function ({ ownerFee, operatorRewarderAddress }, hre: HardhatRuntimeEnvironment) {
    const { ethers, deployments, network, getNamedAccounts } = hre;
    const { log } = deployments;

    log('Setting owner fee for operator rewarder contract...');

    // Get the deployer account
    const { deployer } = await getNamedAccounts();
    const deployerSigner = await ethers.getSigner(deployer);

    // Load the protocol staking contract
    // This task should only be used if the owner is the deployer account
    const operatorRewarder = await ethers.getContractAt(
      OPERATOR_REWARDER_CONTRACT_NAME,
      operatorRewarderAddress,
      deployerSigner,
    );

    // Set the owner fee
    await operatorRewarder.setOwnerFee(ownerFee);

    log(`Owner fee set to ${ownerFee} for operator rewarder contract at 
          address ${operatorRewarderAddress} on network ${network.name}`);
  });

// Set the owner fee for the all operator rewarder contracts
// Example usage:
// npx hardhat task:setAllOwnerFees --network testnet
task('task:setAllOwnerFees').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Setting owner fee for all operator rewarder contracts...');

  // Get the owner fee for the operator rewarder contracts
  const ownerFee = getRequiredEnvVar('OPERATOR_REWARDER_OWNER_FEE');

  // Get the addresses of all operator rewarder contracts
  const operatorRewarderAddresses = await getAllOperatorRewarderAddresses(hre);

  for (let i = 0; i < operatorRewarderAddresses.length; i++) {
    await hre.run('task:setOwnerFee', { ownerFee, operatorRewarderAddress: operatorRewarderAddresses[i] });
  }
});
