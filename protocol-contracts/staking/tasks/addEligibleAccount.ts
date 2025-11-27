import { PROTOCOL_STAKING_CONTRACT_NAME } from './deployment';
import {
  getAllOperatorStakingCoproAddresses,
  getProtocolStakingCoproProxyAddress,
  getProtocolStakingKMSProxyAddress,
  getAllOperatorStakingKMSAddresses,
} from './utils/getAddresses';
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
    const { ethers, deployments, network, getNamedAccounts } = hre;
    const { log } = deployments;

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

    log(`Operator ${operatorStakingAddress} added as eligible in protocol staking contract at 
        address ${protocolStakingProxyAddress} on network ${network.name}`);
  });

// Add all coprocessor operators as eligible in the coprocessor protocol staking contract
// Example usage:
// npx hardhat task:addAllCoproOperatorsAsEligible --network testnet
task('task:addAllCoproOperatorsAsEligible').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Adding all coprocessor operators as eligible in the coprocessor protocol staking contract...');

  const operatorStakingAddresses = await getAllOperatorStakingCoproAddresses(hre);
  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const operatorStakingAddress = operatorStakingAddresses[i];
    await hre.run('task:addOperatorAsEligibleInProtocolStaking', {
      operatorStakingAddress,
      protocolStakingProxyAddress: protocolStakingCoproProxyAddress,
    });
  }
});

// Add all KMS operators as eligible in the KMS protocol staking contract
// Example usage:
// npx hardhat task:addAllKMSOperatorsAsEligible --network testnet
task('task:addAllKMSOperatorsAsEligible').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Adding all KMS operators as eligible in the KMS protocol staking contract...');

  const operatorStakingAddresses = await getAllOperatorStakingKMSAddresses(hre);
  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const operatorStakingAddress = operatorStakingAddresses[i];
    await hre.run('task:addOperatorAsEligibleInProtocolStaking', {
      operatorStakingAddress,
      protocolStakingProxyAddress: protocolStakingKmsProxyAddress,
    });
  }
});

// Add all operators as eligible in the relevant protocol staking contracts (coprocessor and KMS)
// Example usage:
// npx hardhat task:addAllOperatorsAsEligible --network testnet
task('task:addAllOperatorsAsEligible').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Adding all operators as eligible in the protocol staking contract...');

  await hre.run('task:addAllCoproOperatorsAsEligible');
  await hre.run('task:addAllKMSOperatorsAsEligible');
});
