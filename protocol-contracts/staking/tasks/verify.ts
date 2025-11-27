import {
  getProtocolStakingKMSProxyAddress,
  getAllOperatorStakingAddresses,
  getAllOperatorRewarderAddresses,
} from './utils/getAddresses';
import { getRequiredEnvVar } from './utils/loadVariables';
import { task } from 'hardhat/config';

// Verify a protocol staking contract
// Example usage:
// npx hardhat task:verifyProtocolStaking --network testnet
task('task:verifyProtocolStaking').setAction(async function (_, hre) {
  const { upgrades, run } = hre;

  // Get a protocol staking proxy address
  // Since both both protocol staking contracts share the same implementation, we only have to
  // verify one of them
  const proxyAddress = await getProtocolStakingKMSProxyAddress(hre);

  // Get the implementation address
  const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

  await run('verify:verify', {
    address: proxyAddress,
    constructorArguments: [],
  });
  await run('verify:verify', {
    address: implementationAddress,
    constructorArguments: [],
  });
});

// Verify a operator staking contract
// Example usage:
// npx hardhat task:verifyOperatorStaking --network testnet
task('task:verifyOperatorStaking').setAction(async function (_, hre) {
  const { run } = hre;

  // Get the first operator staking address
  // Since all operator staking contracts share the same implementation, we only have to
  // verify one of them
  const operatorStakingAddress = (await getAllOperatorStakingAddresses(hre))[0];

  // Get the constructor arguments for the first KMS operator staking contract
  const kmsTokenName = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_NAME_0`);
  const kmsTokenSymbol = getRequiredEnvVar(`OPERATOR_STAKING_KMS_TOKEN_SYMBOL_0`);
  const kmsOwnerAddress = getRequiredEnvVar(`OPERATOR_STAKING_KMS_OWNER_ADDRESS_0`);

  // Get the protocol staking KMS proxy address
  const protocolStakingKMSProxyAddress = await getProtocolStakingKMSProxyAddress(hre);

  await run('verify:verify', {
    address: operatorStakingAddress,
    constructorArguments: [kmsTokenName, kmsTokenSymbol, protocolStakingKMSProxyAddress, kmsOwnerAddress],
  });
});

// Verify a operator rewarder contract
// Example usage:
// npx hardhat task:verifyOperatorRewarder --network testnet
task('task:verifyOperatorRewarder').setAction(async function (_, hre) {
  const { run } = hre;

  // Get the first operator rewarder address
  // Since all operator rewarder contracts share the same implementation, we only have to
  // verify one of them
  const operatorRewarderAddress = (await getAllOperatorRewarderAddresses(hre))[0];

  // Get the constructor arguments for the first KMS operator staking contract
  const kmsOwnerAddress = getRequiredEnvVar(`OPERATOR_STAKING_KMS_OWNER_ADDRESS_0`);

  // Get the protocol staking KMS proxy address
  const protocolStakingKMSProxyAddress = await getProtocolStakingKMSProxyAddress(hre);

  // Get the first operator staking address
  const operatorStakingAddress = (await getAllOperatorStakingAddresses(hre))[0];

  await run('verify:verify', {
    address: operatorRewarderAddress,
    constructorArguments: [kmsOwnerAddress, protocolStakingKMSProxyAddress, operatorStakingAddress],
  });
});
