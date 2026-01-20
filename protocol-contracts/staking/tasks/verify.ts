import {
  getProtocolStakingKMSProxyAddress,
  getAllOperatorStakingAddresses,
  getAllOperatorRewarderAddresses,
  getProtocolStakingCoproProxyAddress,
} from './utils/getAddresses';
import { getRequiredEnvVar } from './utils/loadVariables';
import { task, types } from 'hardhat/config';

// Verify a protocol staking contract
// Example usage:
// npx hardhat task:verifyProtocolStaking --proxy-address 0x1234567890123456789012345678901234567890 --network testnet
task('task:verifyProtocolStaking')
  .addParam('proxyAddress', 'The address of the protocol staking proxy contract to verify', '', types.string)
  .setAction(async function ({ proxyAddress }, hre) {
    const { upgrades, run } = hre;

    // Get the implementation address
    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

    console.log(`Verifying protocol staking proxy contract at ${proxyAddress}...\n`);
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });

    console.log(`Verifying protocol staking implementation contract at ${implementationAddress}...\n`);
    await run('verify:verify', {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

// Verify both protocol staking contracts
// Since both protocol staking contracts share the same implementation, we normally only have to
// verify one of them. However, since they are proxied, verifying both has the benefit of linking
// the proxy with its implementation on Etherscan.
// Example usage:
// npx hardhat task:verifyAllProtocolStakingContracts --network testnet
task('task:verifyAllProtocolStakingContracts').setAction(async function (_, hre) {
  // Verify the protocol staking coprocessor contract
  // The try catch block is used to not panic if the contracts are already verified
  try {
    console.log('Verifying protocol staking coprocessor contract...');
    const protocolStakingCoproProxyAddress = await getProtocolStakingCoproProxyAddress(hre);
    await hre.run('task:verifyProtocolStaking', { proxyAddress: protocolStakingCoproProxyAddress });
  } catch (error) {
    console.error('An error occurred:', error);
  }

  // Verify the protocol staking KMS contract
  // The try catch block is used to not panic if the contracts are already verified
  try {
    console.log('Verifying protocol staking KMS contract...');
    const protocolStakingKMSProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
    await hre.run('task:verifyProtocolStaking', { proxyAddress: protocolStakingKMSProxyAddress });
  } catch (error) {
    console.error('An error occurred:', error);
  }
});

// Verify a single operator staking contract by proxy address
// Example usage:
// npx hardhat task:verifyOperatorStaking --proxy-address 0x1234567890123456789012345678901234567890 --network testnet
task('task:verifyOperatorStaking')
  .addParam('proxyAddress', 'The address of the operator staking proxy contract to verify', '', types.string)
  .setAction(async function ({ proxyAddress }, hre) {
    const { run, upgrades } = hre;

    // Get the implementation address
    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

    console.log(`Verifying operator staking proxy contract at ${proxyAddress}...\n`);
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });

    console.log(`Verifying operator staking implementation contract at ${implementationAddress}...\n`);
    await run('verify:verify', {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

// Verify all operator staking contracts (proxies and implementations)
// Since all operator staking contracts share the same implementation, we normally only have to
// verify one of them. However, since they are proxied, verifying all of them has the benefit of linking
// their proxies with their implementations on Etherscan.
// Example usage:
// npx hardhat task:verifyAllOperatorStakingContracts --network testnet
task('task:verifyAllOperatorStakingContracts').setAction(async function (_, hre) {
  // Get all operator staking proxy addresses
  const operatorStakingProxyAddresses = await getAllOperatorStakingAddresses(hre);

  for (const proxyAddress of operatorStakingProxyAddresses) {
    try {
      await hre.run('task:verifyOperatorStaking', { proxyAddress });
    } catch (error) {
      console.error('An error occurred:', error);
    }
  }
});

// Verify an operator rewarder contract
// Example usage:
// npx hardhat task:verifyOperatorRewarder --network testnet
task('task:verifyOperatorRewarder').setAction(async function (_, hre) {
  const { run } = hre;

  // Get the first operator rewarder address
  // Since all operator rewarder contracts share the same implementation, we only have to
  // verify one of them
  const operatorRewarderAddress = (await getAllOperatorRewarderAddresses(hre))[0];

  // Get the constructor arguments for the first KMS operator staking contract
  const kmsOwnerAddress = getRequiredEnvVar(`OPERATOR_REWARDER_KMS_BENEFICIARY_0`);
  const protocolStakingKMSProxyAddress = await getProtocolStakingKMSProxyAddress(hre);
  const operatorStakingAddress = (await getAllOperatorStakingAddresses(hre))[0];
  const kmsInitialMaxFee = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_MAX_FEE_0`));
  const kmsInitialFee = parseInt(getRequiredEnvVar(`OPERATOR_REWARDER_KMS_FEE_0`));

  console.log(`Verifying operator rewarder contract at ${operatorRewarderAddress}...\n`);
  await run('verify:verify', {
    address: operatorRewarderAddress,
    constructorArguments: [
      kmsOwnerAddress,
      protocolStakingKMSProxyAddress,
      operatorStakingAddress,
      kmsInitialMaxFee,
      kmsInitialFee,
    ],
  });
});
