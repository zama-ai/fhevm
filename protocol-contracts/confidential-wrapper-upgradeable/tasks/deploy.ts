import { getRequiredEnvVar } from './utils/loadVariables';
import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

export const CONTRACT_NAME = 'ConfidentialWrapper';

// Get the deployment name for a confidential wrapper
export function getConfidentialWrapperName(tokenName: string): string {
  return `ConfidentialWrapper_${tokenName}`;
}

// Get the implementation deployment name for a confidential wrapper
export function getConfidentialWrapperImplName(tokenName: string): string {
  return `ConfidentialWrapper_${tokenName}_Impl`;
}

// Get the proxy deployment name for a confidential wrapper
export function getConfidentialWrapperProxyName(tokenName: string): string {
  return `ConfidentialWrapper_${tokenName}_Proxy`;
}

// Deploy a confidential wrapper contract as a function
async function deployConfidentialWrapper(
  name: string,
  symbol: string,
  contractUri: string,
  underlying: string,
  owner: string,
  hre: HardhatRuntimeEnvironment,
) {
  const { ethers, upgrades, deployments, getNamedAccounts } = hre;
  const { save, getArtifact } = deployments;
  const { deployer } = await getNamedAccounts();

  // Deploy the proxy contract
  const confidentialWrapperFactory = await ethers.getContractFactory(CONTRACT_NAME);
  const proxy = await upgrades.deployProxy(confidentialWrapperFactory, [name, symbol, contractUri, underlying, owner], {
    initializer: 'initialize',
    kind: 'uups',
  });

  await proxy.waitForDeployment();
  const proxyAddress = await proxy.getAddress();

  console.log(
    [
      `✅ Deployed ${name} ConfidentialWrapper:`,
      `  - Confidential wrapper proxy address:  ${proxyAddress}`,
      `  - name: ${name}`,
      `  - symbol: ${symbol}`,
      `  - contract URI: ${contractUri}`,
      `  - underlying: ${underlying}`,
      `  - owner: ${owner}`,
      `  - Deployed by deployer account: ${deployer}`,
      `  - Network: ${hre.network.name}`,
      '',
    ].join('\n'),
  );

  // Save the deployment artifacts
  const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);
  const artifact = await getArtifact(CONTRACT_NAME);
  await save(getConfidentialWrapperProxyName(name), { address: proxyAddress, abi: artifact.abi });
  await save(getConfidentialWrapperImplName(name), { address: implementationAddress, abi: artifact.abi });
}

// Deploy all confidential wrapper contracts
// Example usage:
// npx hardhat task:deployAllConfidentialWrappers --network testnet
task('task:deployAllConfidentialWrappers').setAction(async function (_, hre) {
  console.log('Deploying confidential wrapper contracts...');

  // Get the number of confidential wrappers from environment variable
  const numWrappers = parseInt(getRequiredEnvVar('NUM_CONFIDENTIAL_WRAPPERS'));

  for (let i = 0; i < numWrappers; i++) {
    // Get the name from environment variable
    const name = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_NAME_${i}`);
    const symbol = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_SYMBOL_${i}`);
    const contractUri = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_CONTRACT_URI_${i}`);
    const underlying = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_UNDERLYING_ADDRESS_${i}`);
    const owner = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_OWNER_ADDRESS_${i}`);

    await deployConfidentialWrapper(name, symbol, contractUri, underlying, owner, hre);
  }

  console.log('✅ All confidential wrapper contracts deployed\n');
});
