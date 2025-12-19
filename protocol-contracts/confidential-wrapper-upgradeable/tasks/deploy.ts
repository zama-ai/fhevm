import { getRequiredEnvVar } from './utils/loadVariables';
import { task, types } from 'hardhat/config';

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

// Deploy a confidential wrapper contract
// Example usage:
// npx hardhat task:deployConfidentialWrapper --network testnet
task('task:deployConfidentialWrapper', 'Deploy a confidential wrapper contract')
  .addOptionalParam('name', 'The name of the wrapped token', undefined, types.string)
  .addOptionalParam('symbol', 'The symbol of the wrapped token', undefined, types.string)
  .addOptionalParam('contractUri', 'The contract URI for the wrapped token', undefined, types.string)
  .addOptionalParam('underlying', 'The address of the underlying ERC20 token', undefined, types.string)
  .addOptionalParam('owner', 'The owner address of the wrapper contract', undefined, types.string)
  .setAction(async function (taskArgs, hre) {
    const { ethers, upgrades, deployments, getNamedAccounts } = hre;
    const { save } = deployments;
    const { deployer } = await getNamedAccounts();

    // Get parameters from task args or environment variables
    const name = taskArgs.name || getRequiredEnvVar('CONFIDENTIAL_WRAPPER_NAME');
    const symbol = taskArgs.symbol || getRequiredEnvVar('CONFIDENTIAL_WRAPPER_SYMBOL');
    const contractUri = taskArgs.contractUri || getRequiredEnvVar('CONFIDENTIAL_WRAPPER_CONTRACT_URI');
    const underlyingAddress = taskArgs.underlying || getRequiredEnvVar('CONFIDENTIAL_WRAPPER_UNDERLYING_ADDRESS');
    const ownerAddress = taskArgs.owner || getRequiredEnvVar('CONFIDENTIAL_WRAPPER_OWNER_ADDRESS', deployer);

    console.log(`Deploying ConfidentialWrapper with:`);
    console.log(`  Name: ${name}`);
    console.log(`  Symbol: ${symbol}`);
    console.log(`  Contract URI: ${contractUri}`);
    console.log(`  Underlying: ${underlyingAddress}`);
    console.log(`  Owner: ${ownerAddress}`);

    // Deploy the proxy contract
    const ConfidentialWrapper = await ethers.getContractFactory('ConfidentialWrapper');
    const proxy = await upgrades.deployProxy(
      ConfidentialWrapper,
      [name, symbol, contractUri, underlyingAddress, ownerAddress],
      {
        initializer: 'initialize',
        kind: 'uups',
      },
    );

    await proxy.waitForDeployment();
    const proxyAddress = await proxy.getAddress();
    const implementationAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

    console.log(`ConfidentialWrapper proxy deployed at: ${proxyAddress}`);
    console.log(`ConfidentialWrapper implementation deployed at: ${implementationAddress}`);

    // Save the deployment artifacts
    const artifact = await deployments.getExtendedArtifact('ConfidentialWrapper');

    await save(getConfidentialWrapperProxyName(symbol), {
      address: proxyAddress,
      ...artifact,
    });

    await save(getConfidentialWrapperImplName(symbol), {
      address: implementationAddress,
      ...artifact,
    });

    return { proxyAddress, implementationAddress };
  });

// Upgrade a confidential wrapper contract
// Example usage:
// npx hardhat task:upgradeConfidentialWrapper --proxy 0x1234... --network testnet
task('task:upgradeConfidentialWrapper', 'Upgrade a confidential wrapper contract')
  .addParam('proxy', 'The proxy address of the confidential wrapper to upgrade', undefined, types.string)
  .setAction(async function ({ proxy }, hre) {
    const { ethers, upgrades } = hre;

    console.log(`Upgrading ConfidentialWrapper at proxy: ${proxy}`);

    const ConfidentialWrapper = await ethers.getContractFactory('ConfidentialWrapper');
    const upgraded = await upgrades.upgradeProxy(proxy, ConfidentialWrapper);

    await upgraded.waitForDeployment();
    const newImplementationAddress = await upgrades.erc1967.getImplementationAddress(proxy);

    console.log(`ConfidentialWrapper upgraded successfully`);
    console.log(`New implementation deployed at: ${newImplementationAddress}`);

    return { proxyAddress: proxy, implementationAddress: newImplementationAddress };
  });
