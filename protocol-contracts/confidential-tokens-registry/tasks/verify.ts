import { CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME } from './deploy';
import { task, types } from 'hardhat/config';

// Verify a confidential wrapper contract
// Example usage:
// npx hardhat task:verifyConfidentialTokensRegistry --proxyAddress 0x1234567890123456789012345678901234567890 --network testnet
task('task:verifyConfidentialTokensRegistry')
  .addOptionalParam(
    'proxyAddress',
    'The address of the confidential tokens registry proxy contract to verify. If not provided, the proxy address will be fetched from deployments.',
    false,
    types.string,
  )
  .setAction(async function ({ proxyAddress }, hre) {
    const { upgrades, run, deployments } = hre;
    const { get } = deployments;

    if (!proxyAddress) {
      // Get the proxy address from deployments
      proxyAddress = await get(CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME);
    }

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
