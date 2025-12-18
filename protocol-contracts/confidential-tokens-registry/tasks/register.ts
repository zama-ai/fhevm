import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

import { getRequiredEnvVar } from './utils/loadVariables';
import { CONFIDENTIAL_TOKENS_REGISTRY_CONTRACT_NAME, CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME } from './deploy';

// Register a token with its confidential token in the ConfidentialTokensRegistry
async function registerConfidentialToken(
  tokenAddress: string,
  confidentialTokenAddress: string,
  hre: HardhatRuntimeEnvironment,
) {
  const { getNamedAccounts, ethers, deployments, network } = hre;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Get the ConfidentialTokensRegistry proxy address from deployments
  const registryDeployment = await deployments.get(CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME);
  const registryAddress = registryDeployment.address;

  // Get the ConfidentialTokensRegistry contract instance
  const registry = await ethers.getContractAt(
    CONFIDENTIAL_TOKENS_REGISTRY_CONTRACT_NAME,
    registryAddress,
    deployerSigner,
  );

  // Register the token with its confidential token
  const tx = await registry.registerConfidentialToken(tokenAddress, confidentialTokenAddress);
  await tx.wait();

  console.log(
    [
      `✅ Registered confidential token:`,
      `  - Token address: ${tokenAddress}`,
      `  - Confidential token address: ${confidentialTokenAddress}`,
      `  - Registry address: ${registryAddress}`,
      `  - Registered by: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );
}

// Register a token with its confidential token in the ConfidentialTokensRegistry
// Example usage:
// npx hardhat task:registerConfidentialToken --token 0x1234... --confidential-token 0x5678... --network testnet
task('task:registerConfidentialToken')
  .addParam('token', 'The address of the ERC20 token to register')
  .addParam('confidentialToken', 'The address of the ERC7984 confidential token to associate')
  .setAction(async function ({ token, confidentialToken }, hre) {
    console.log('Registering confidential token...\n');

    await registerConfidentialToken(token, confidentialToken, hre);

    console.log('✅ Confidential token registered\n');
  });

// Register all initial tokens from environment variables
// Example usage:
// npx hardhat task:registerAllInitialConfidentialTokens --network testnet
task('task:registerAllInitialConfidentialTokens').setAction(async function (_, hre) {
  console.log('Registering all initial confidential tokens from environment...\n');

  // Get the number of tokens to register
  const numTokens = parseInt(getRequiredEnvVar('INITIAL_NUM_TOKENS'));

  for (let i = 0; i < numTokens; i++) {
    const tokenAddress = getRequiredEnvVar(`INITIAL_TOKEN_ADDRESS_${i}`);
    const confidentialTokenAddress = getRequiredEnvVar(`INITIAL_CONFIDENTIAL_TOKEN_ADDRESS_${i}`);

    await hre.run('task:registerConfidentialToken', {
      token: tokenAddress,
      confidentialToken: confidentialTokenAddress,
    });
  }

  console.log(`✅ All ${numTokens} initial confidential tokens registered\n`);
});
