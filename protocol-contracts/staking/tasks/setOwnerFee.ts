import { OPERATOR_REWARDER_CONTRACT_NAME } from './deployment';
import { getAllOperatorRewarderCoproAddresses, getAllOperatorRewarderKMSAddresses } from './utils/getAddresses';
import { getRequiredEnvVar } from './utils/loadVariables';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Set the owner fee for the operator rewarder contract using the deployer account
// This task only works if the deployer account is the owner of the operator rewarder contract
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
    const { ethers, network, getNamedAccounts } = hre;

    console.log('Setting owner fee for operator rewarder contract...');

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

    // Set the owner fee
    const tx = await operatorRewarder.setOwnerFee(ownerFee);
    await tx.wait();

    console.log(
      [
        `👉 Set owner fee:`,
        `  - Owner fee in basis points: ${ownerFee}`,
        `  - Operator rewarder address: ${operatorRewarderAddress}`,
        `  - Initiated by owner (deployer): ${deployer}`,
        `  - Network: ${network.name}`,
        '',
      ].join('\n'),
    );
  });

// Set the owner fees for the all coprocessor operator rewarder contracts using the deployer account
// This task only works if the deployer account is the owner of all the coprocessor operator rewarder contracts
// Example usage:
// npx hardhat task:setAllCoprocessorOwnerFees --network testnet
task('task:setAllCoprocessorOwnerFees').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting owner fee for all coprocessor operator rewarder contracts...\n');

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
    // Get the owner fee for the operator rewarder contracts
    const ownerFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_COPRO_OWNER_FEE_${i}`)));
    await hre.run('task:setOwnerFee', { ownerFee, operatorRewarderAddress: operatorRewarderAddresses[i] });
  }
});

// Set the owner fees for the all KMS operator rewarder contracts using the deployer account
// This task only works if the deployer account is the owner of all the KMS operator rewarder contracts
// Example usage:
// npx hardhat task:setAllKMSOwnerFees --network testnet
task('task:setAllKMSOwnerFees').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting owner fee for all KMS operator rewarder contracts...\n');

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
    // Get the owner fee for the operator rewarder contracts
    const ownerFee = BigInt(parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_OWNER_FEE_${i}`)));
    await hre.run('task:setOwnerFee', { ownerFee, operatorRewarderAddress: operatorRewarderAddresses[i] });
  }
});

// Set the owner fee for the all operator rewarder contracts using the deployer account
// This task only works if the deployer account is the owner of all the operator rewarder contracts
// Example usage:
// npx hardhat task:setAllOwnerFees --network testnet
task('task:setAllOwnerFees').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Setting owner fee for all operator rewarder contracts...\n');

  await hre.run('task:setAllCoprocessorOwnerFees');
  await hre.run('task:setAllKMSOwnerFees');

  console.log('✅ Owner fees for all operator rewarder contracts have been set\n');
});
