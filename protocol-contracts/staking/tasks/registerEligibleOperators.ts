import { PROTOCOL_STAKING_CONTRACT_NAME } from './deployment';
import {
  getAllOperatorStakingCoproAddresses,
  getProtocolStakingCoproProxyAddress,
  getProtocolStakingKMSProxyAddress,
  getAllOperatorStakingKMSAddresses,
} from './utils/getAddresses';
import { task, types } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

// Register an operator as eligible in the protocol staking contract
// Example usage:
// npx hardhat task:registerOperatorInProtocolStaking \
//   --operatorStakingAddress 0x1234567890123456789012345678901234567890 \
//   --protocolStakingProxyAddress 0x1234567890123456789012345678901234567890 \
//   --network ethereum-testnet
task('task:registerOperatorInProtocolStaking')
  .addParam(
    'operatorStakingAddress',
    'The address of the operator staking contract to register as eligible',
    '',
    types.string,
  )
  .addParam(
    'protocolStakingProxyAddress',
    'The address of the protocol staking contract to register the eligible operator for',
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

    // Register the eligible operator
    await protocolStaking.addEligibleAccount(operatorStakingAddress);

    log(`Eligible operator ${operatorStakingAddress} registered in protocol staking contract at 
        address ${protocolStakingProxyAddress} on network ${network.name}`);
  });

// Register all coprocessor operators in the coprocessor protocol staking contract
// Example usage:
// npx hardhat task:registerAllCoproOperatorsInProtocolStaking --network ethereum-testnet
task('task:registerAllCoproOperatorsInProtocolStaking').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Registering all coprocessor operators in the coprocessor protocol staking contract...');

  const operatorStakingAddresses = await getAllOperatorStakingCoproAddresses(hre);
  const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);

  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const operatorStakingAddress = operatorStakingAddresses[i];
    await hre.run('task:registerOperatorInProtocolStaking', {
      operatorStakingAddress,
      protocolStakingProxyAddress: protocolStakingCoproProxyAddress,
    });
  }
});

// Register all KMS operators in the KMS protocol staking contract
// Example usage:
// npx hardhat task:registerAllKMSOperatorsInProtocolStaking --network ethereum-testnet
task('task:registerAllKMSOperatorsInProtocolStaking').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Registering all KMS operators in the KMS protocol staking contract...');

  const operatorStakingAddresses = await getAllOperatorStakingKMSAddresses(hre);
  const protocolStakingKmsProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  for (let i = 0; i < operatorStakingAddresses.length; i++) {
    const operatorStakingAddress = operatorStakingAddresses[i];
    await hre.run('task:registerOperatorInProtocolStaking', {
      operatorStakingAddress,
      protocolStakingProxyAddress: protocolStakingKmsProxyAddress,
    });
  }
});

// Register all operators in the relevant protocol staking contracts (coprocessor and KMS)
// Example usage:
// npx hardhat task:registerAllOperatorsInProtocolStaking --network ethereum-testnet
task('task:registerAllOperatorsInProtocolStaking').setAction(async function (_, hre: HardhatRuntimeEnvironment) {
  const { log } = hre.deployments;

  log('Registering all operators in the protocol staking contract...');

  await hre.run('task:registerAllCoproOperatorsInProtocolStaking');
  await hre.run('task:registerAllKMSOperatorsInProtocolStaking');
});
