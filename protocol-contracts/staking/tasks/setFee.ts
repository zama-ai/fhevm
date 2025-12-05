import { OPERATOR_REWARDER_CONTRACT_NAME } from './deployment';
import { getAllOperatorRewarderCoproAddresses, getAllOperatorRewarderKMSAddresses } from './utils/getAddresses';
import { getRequiredEnvVar } from './utils/loadVariables';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Set the fee for the operator rewarder contract using the deployer account
// This task only works if the deployer account is the owner of the operator rewarder contract
// Note: The fee is in basis points (in 1/100th of a percent, so 10000 = 100.00%)
// Example usage:
// npx hardhat task:setFee --fee 2000 --operatorRewarderAddress 0x1234567890123456789012345678901234567890 --network testnet
task('task:setFee')
  .addParam('fee', 'The fee to set for the operator rewarder contract (in 1/100th of a percent)', 0n, types.bigint)
  .addParam(
    'operatorRewarderAddress',
    'The address of the operator rewarder contract to set the fee for',
    '',
    types.string,
  )
  .setAction(async function ({ fee, operatorRewarderAddress }, hre: HardhatRuntimeEnvironment) {
    const { ethers, network, getNamedAccounts } = hre;

    console.log('Setting fee for operator rewarder contract...');

    // Get the deployer account
    const { deployer } = await getNamedAccounts();
    const deployerSigner = await ethers.getSigner(deployer);

    // Load the operator rewarder contract
    // This task should only be used if the owner is the deployer account
    const operatorRewarder = await ethers.getContractAt(
      OPERATOR_REWARDER_CONTRACT_NAME,
      operatorRewarderAddress,
      deployerSigner,
    );

    // Set the fee
    const tx = await operatorRewarder.setFee(fee);
    await tx.wait();

    console.log(
      [
        `👉 Set fee:`,
        `  - Fee in basis points: ${fee}`,
        `  - Operator rewarder address: ${operatorRewarderAddress}`,
        `  - Initiated by owner (deployer): ${deployer}`,
        `  - Network: ${network.name}`,
        '',
      ].join('\n'),
    );
  });

// Set the fees for the all coprocessor operator rewarder contracts using the deployer account
// This task only works if the deployer account is the owner of all the coprocessor operator rewarder contracts
// Example usage:
// npx hardhat task:setAllCoprocessorFees --network testnet
task('task:setAllCoprocessorFees').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting fee for all coprocessor operator rewarder contracts...\n');

  // Get the addresses of all coprocessor operator rewarder contracts
  const operatorRewarderAddresses = await getAllOperatorRewarderCoproAddresses(hre);

  // Get the number of coprocessor operator rewarder contracts and check if the number of addresses is correct
  const numOperatorRewarderCopro = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_COPRO'));

  if (operatorRewarderAddresses.length !== numOperatorRewarderCopro) {
    throw new Error(
      `The number of operator rewarder contracts (${operatorRewarderAddresses.length}) does not 
      match the number of coprocessor operator staking contracts (${numOperatorRewarderCopro})`,
    );
  }

  for (let i = 0; i < operatorRewarderAddresses.length; i++) {
    // Get the fee for the operator rewarder contracts
    const fee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_FEE_${i}`)));
    await hre.run('task:setFee', { fee, operatorRewarderAddress: operatorRewarderAddresses[i] });
  }
});

// Set the fees for the all KMS operator rewarder contracts using the deployer account
// This task only works if the deployer account is the owner of all the KMS operator rewarder contracts
// Example usage:
// npx hardhat task:setAllKMSFees --network testnet
task('task:setAllKMSFees').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting fee for all KMS operator rewarder contracts...\n');

  // Get the addresses of all KMS operator rewarder contracts
  const operatorRewarderAddresses = await getAllOperatorRewarderKMSAddresses(hre);

  // Get the number of KMS operator rewarder contracts and check if the number of addresses is correct
  const numOperatorRewarderKms = parseInt(getRequiredEnvVar('NUM_OPERATOR_STAKING_KMS'));

  if (operatorRewarderAddresses.length !== numOperatorRewarderKms) {
    throw new Error(
      `The number of operator rewarder contracts (${operatorRewarderAddresses.length}) does not 
      match the number of KMS operator staking contracts (${numOperatorRewarderKms})`,
    );
  }

  for (let i = 0; i < operatorRewarderAddresses.length; i++) {
    // Get the fee for the operator rewarder contracts
    const fee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_FEE_${i}`)));
    await hre.run('task:setFee', { fee, operatorRewarderAddress: operatorRewarderAddresses[i] });
  }
});

// Set the fee for the all operator rewarder contracts using the deployer account
// This task only works if the deployer account is the owner of all the operator rewarder contracts
// Example usage:
// npx hardhat task:setAllFees --network testnet
task('task:setAllFees').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting fee for all operator rewarder contracts...\n');

  await hre.run('task:setAllCoprocessorFees');
  await hre.run('task:setAllKMSFees');

  console.log('✅ Fees for all operator rewarder contracts have been set\n');
});
