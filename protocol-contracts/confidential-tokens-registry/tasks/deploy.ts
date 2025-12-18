import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';

export const CONFIDENTIAL_TOKENS_REGISTRY_CONTRACT_NAME = 'ConfidentialTokensRegistry';
export const CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME = `${CONFIDENTIAL_TOKENS_REGISTRY_CONTRACT_NAME}_Proxy`;
export const CONFIDENTIAL_TOKENS_REGISTRY_IMPL_NAME = `${CONFIDENTIAL_TOKENS_REGISTRY_CONTRACT_NAME}_Impl`;

// Deploy the ConfidentialTokensRegistry contract
async function deployConfidentialTokensRegistry(hre: HardhatRuntimeEnvironment) {
  const { getNamedAccounts, ethers, deployments, network, upgrades } = hre;
  const { save, getArtifact } = deployments;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Get the contract factory and deploy the proxy + the implementation
  const confidentialTokensRegistryFactory = await ethers.getContractFactory(
    CONFIDENTIAL_TOKENS_REGISTRY_CONTRACT_NAME,
    deployerSigner,
  );
  const proxy = await upgrades.deployProxy(confidentialTokensRegistryFactory, [deployer], {
    kind: 'uups',
    initializer: 'initialize',
  });
  await proxy.waitForDeployment();

  // Get the proxy address
  const proxyAddress = await proxy.getAddress();

  console.log(
    [
      `✅ Deployed ConfidentialTokensRegistry:`,
      `  - Proxy address: ${proxyAddress}`,
      `  - Initial owner (deployer): ${deployer}`,
      `  - Deployed by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );

  // Save the proxy and implementation contract artifacts
  const artifact = await getArtifact(CONFIDENTIAL_TOKENS_REGISTRY_CONTRACT_NAME);
  const implAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

  await save(CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME, {
    address: proxyAddress,
    abi: artifact.abi,
  });
  await save(CONFIDENTIAL_TOKENS_REGISTRY_IMPL_NAME, {
    address: implAddress,
    abi: artifact.abi,
  });

  return proxyAddress;
}

// Deploy the ConfidentialTokensRegistry contract
// Example usage:
// npx hardhat task:deployConfidentialTokensRegistry --network testnet
task('task:deployConfidentialTokensRegistry').setAction(async function (_, hre) {
  console.log('Deploying ConfidentialTokensRegistry contract...\n');

  await deployConfidentialTokensRegistry(hre);

  console.log('✅ ConfidentialTokensRegistry contract deployed\n');
});
