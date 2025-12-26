import { getConfidentialWrapperProxyName } from './deploy';
import { getRequiredEnvVar } from './utils/loadVariables';
import { task, types } from 'hardhat/config';

// Verify a confidential wrapper contract
// Example usage:
// npx hardhat task:verifyConfidentialWrapper --proxy-address 0x1234567890123456789012345678901234567890 --network testnet
task('task:verifyConfidentialWrapper')
  .addParam('proxyAddress', 'The address of the confidential wrapper proxy contract to verify', '', types.string)
  .setAction(async function ({ proxyAddress }, hre) {
    const { upgrades, run } = hre;

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

// Verify all confidential wrapper contracts
// Since all confidential wrapper contracts share the same implementation, we normally only have to
// verify one of them. However, since they are proxied, verifying all of them has the benefit of linking
// the proxies with their implementation on Etherscan.
// Example usage:
// npx hardhat task:verifyAllConfidentialWrappers --network testnet
task('task:verifyAllConfidentialWrappers').setAction(async function (_, hre) {
  const { run, deployments } = hre;
  const { get } = deployments;

  // Get the number of confidential wrappers from environment variable
  const numWrappers = parseInt(getRequiredEnvVar('NUM_CONFIDENTIAL_WRAPPERS'));

  for (let i = 0; i < numWrappers; i++) {
    // Get the name from environment variable
    const name = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_NAME_${i}`);

    try {
      // Get the proxy address from deployments
      const proxyAddress = await get(getConfidentialWrapperProxyName(name));

      // Verify the confidential wrapper contract
      await run('task:verifyConfidentialWrapper', { proxyAddress: proxyAddress.address });
    } catch (error) {
      console.error('An error occurred:', error);
    }
  }
});
