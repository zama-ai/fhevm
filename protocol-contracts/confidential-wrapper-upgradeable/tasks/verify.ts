import { getConfidentialWrapperProxyName } from './deploy';
import { getRequiredEnvVar } from './utils/loadVariables';
import { task, types } from 'hardhat/config';

// Verify a confidential wrapper contract
// Example usage:
// npx hardhat task:verifyConfidentialWrapper --proxyAddress 0x1234567890123456789012345678901234567890 --network testnet
task('task:verifyConfidentialWrapper')
  .addParam('proxyAddress', 'The address of the confidential wrapper proxy contract to verify', '', types.string)
  .setAction(async function ({ proxyAddress }, hre) {
    const { upgrades, run } = hre;

    // Get the implementation address
    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

    console.log(`Verifying confidential wrapper proxy contract at ${proxyAddress}...\n`);
    await run('verify:verify', {
      address: proxyAddress,
      constructorArguments: [],
    });

    console.log(`Verifying confidential wrapper implementation contract at ${implementationAddress}...\n`);
    await run('verify:verify', {
      address: implementationAddress,
      constructorArguments: [],
    });
  });

// Verify the confidential wrapper contract using environment variables
// Example usage:
// npx hardhat task:verifyConfidentialWrapperFromEnv --network testnet
task('task:verifyConfidentialWrapperFromEnv').setAction(async function (_, hre) {
  const { get } = hre.deployments;

  // Get the symbol from environment variable
  const symbol = getRequiredEnvVar('CONFIDENTIAL_WRAPPER_SYMBOL');

  // Get the proxy address from deployments
  const proxyAddress = await get(getConfidentialWrapperProxyName(symbol));

  // Verify the contract
  try {
    console.log('Verifying confidential wrapper contract...');
    await hre.run('task:verifyConfidentialWrapper', { proxyAddress });
  } catch (error) {
    console.error('An error occurred:', error);
  }
});
