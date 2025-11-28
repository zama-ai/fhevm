import { PROTOCOL_STAKING_CONTRACT_NAME } from './deployment';
import {
  getAllOperatorStakingCoproAddresses,
  getProtocolStakingCoproProxyAddress,
  getProtocolStakingKMSProxyAddress,
  getAllOperatorStakingKMSAddresses,
} from './utils/getAddresses';
import { wait } from './utils/time';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Add an operator as eligible in the protocol staking contract
// Example usage:
// npx hardhat task:addOperatorAsEligibleInProtocolStaking \
//   --operatorStakingAddress 0x1234567890123456789012345678901234567890 \
//   --protocolStakingProxyAddress 0x1234567890123456789012345678901234567890 \
//   --network testnet
task('task:addOperatorAsEligibleInProtocolStaking')
  .addParam(
    'operatorStakingAddress',
    'The address of the operator staking contract to add as eligible',
    '',
    types.string,
  )
  .addParam(
    'protocolStakingProxyAddress',
    'The address of the protocol staking contract to add the eligible operator to',
    '',
    types.string,
  )
  .setAction(async function ({ operatorStakingAddress, protocolStakingProxyAddress }, hre: HardhatRuntimeEnvironment) {
    const { ethers, network, getNamedAccounts } = hre;

    // Get the deployer account
    const { deployer } = await getNamedAccounts();
    const deployerSigner = await ethers.getSigner(deployer);

    // Load the protocol staking contract
    const protocolStaking = await ethers.getContractAt(
      PROTOCOL_STAKING_CONTRACT_NAME,
      protocolStakingProxyAddress,
      deployerSigner,
    );

    // Add the operator as eligible
    await protocolStaking.addEligibleAccount(operatorStakingAddress);

    console.log(
      [
        `👉 Added operator as eligible:`,
        `  - Operator staking address: ${operatorStakingAddress}`,
        `  - Protocol staking proxy address: ${protocolStakingProxyAddress}`,
        `  - Added by manager (deployer): ${deployer}`,
        `  - Network: ${network.name}`,
        '',
      ].join('\n'),
    );
  });

// Add all coprocessor operators as eligible in the coprocessor protocol staking contract
// Example usage:
// npx hardhat task:addAllCoproOperatorsAsEligible --network testnet
task('task:addAllCoproOperatorsAsEligible').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Adding all coprocessor operators as eligible in the coprocessor protocol staking contract...\n');

  const operatorStakingAddresses = await getAllOperatorStakingCoproAddresses(hre);
  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const operatorStakingAddress = operatorStakingAddresses[i];
    await hre.run('task:addOperatorAsEligibleInProtocolStaking', {
      operatorStakingAddress,
      protocolStakingProxyAddress: protocolStakingCoproProxyAddress,
    });

    if (i < operatorStakingAddresses.length - 1) {
      // Wait for 5 seconds before adding the next operator as eligible in order to avoid underpriced transaction issues
      await wait(5);
    }
  }
});

// Add all KMS operators as eligible in the KMS protocol staking contract
// Example usage:
// npx hardhat task:addAllKMSOperatorsAsEligible --network testnet
task('task:addAllKMSOperatorsAsEligible').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Adding all KMS operators as eligible in the KMS protocol staking contract...\n');

  const operatorStakingAddresses = await getAllOperatorStakingKMSAddresses(hre);
  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const operatorStakingAddress = operatorStakingAddresses[i];
    await hre.run('task:addOperatorAsEligibleInProtocolStaking', {
      operatorStakingAddress,
      protocolStakingProxyAddress: protocolStakingKmsProxyAddress,
    });

    if (i < operatorStakingAddresses.length - 1) {
      // Wait for 5 seconds before adding the next operator as eligible in order to avoid underpriced transaction issues
      await wait(5);
    }
  }
});

// Add all operators as eligible in the relevant protocol staking contracts (coprocessor and KMS)
// Example usage:
// npx hardhat task:addAllOperatorsAsEligible --network testnet
task('task:addAllOperatorsAsEligible').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  console.log('Adding all operators as eligible in the protocol staking contract...\n');

  await hre.run('task:addAllCoproOperatorsAsEligible');

  await wait(5);

  await hre.run('task:addAllKMSOperatorsAsEligible');
});
