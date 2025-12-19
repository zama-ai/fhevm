import { CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME } from './deploy';
import { task, types } from 'hardhat/config';

// Verify a confidential wrapper contract
// Example usage:
// npx hardhat task:verifyConfidentialTokensRegistry --proxyAddress 0x1234567890123456789012345678901234567890 --network testnet
task('task:verifyConfidentialTokensRegistry')
  .addParam(
    'proxyAddress',
    'The address of the confidential tokens registry proxy contract to verify',
    '',
    types.string,
  )
  .setAction(async function ({ proxyAddress }, hre) {
    const { upgrades, run } = hre;

    // Get the implementation address
    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

    console.log(`Verifying confidential tokens registry proxy contract at ${proxyAddress}...\n`);
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });

    console.log(`Verifying confidential tokens registry implementation contract at ${implementationAddress}...\n`);
    await run('verify:verify', {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

// Verify the confidential tokens registry contract using environment variables
// Example usage:
// npx hardhat task:verifyConfidentialTokensRegistryFromEnv --network testnet
task('task:verifyConfidentialTokensRegistryFromEnv').setAction(async function (_, hre) {
  const { get } = hre.deployments;

  // Get the proxy address from deployments
  const proxyAddress = await get(CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME);

  // Verify the contract
  try {
    console.log('Verifying confidential tokens registry contract...');
    await hre.run('task:verifyConfidentialTokensRegistry', { proxyAddress });
  } catch (error) {
    console.error('An error occurred:', error);
  }
});
