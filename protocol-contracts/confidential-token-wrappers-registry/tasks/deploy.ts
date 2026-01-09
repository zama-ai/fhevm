import { task } from 'hardhat/config';
import { HardhatRuntimeEnvironment } from 'hardhat/types';
import { getRequiredEnvVar } from './utils/loadVariables';

export const CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_CONTRACT_NAME = 'ConfidentialTokenWrappersRegistry';
export const CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_PROXY_NAME = `${CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_CONTRACT_NAME}_Proxy`;
export const CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_IMPL_NAME = `${CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_CONTRACT_NAME}_Impl`;

// Deploy the ConfidentialTokenWrappersRegistry contract
async function deployConfidentialTokenWrappersRegistry(hre: HardhatRuntimeEnvironment) {
  const { getNamedAccounts, ethers, deployments, network, upgrades } = hre;
  const { save, getArtifact } = deployments;

  // Get the deployer account
  const { deployer } = await getNamedAccounts();
  const deployerSigner = await ethers.getSigner(deployer);

  // Get the initial owner address
  const initialOwner = getRequiredEnvVar('INITIAL_OWNER');

  // Get the contract factory and deploy the proxy + the implementation
  const confidentialTokenWrappersRegistryFactory = await ethers.getContractFactory(
    CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_CONTRACT_NAME,
    deployerSigner,
  );
  const proxy = await upgrades.deployProxy(confidentialTokenWrappersRegistryFactory, [initialOwner], {
    kind: 'uups',
    initializer: 'initialize',
  });
  await proxy.waitForDeployment();

  // Get the proxy address
  const proxyAddress = await proxy.getAddress();

  console.log(
    [
      `✅ Deployed ConfidentialTokenWrappersRegistry:`,
      `  - Proxy address: ${proxyAddress}`,
      `  - Initial owner: ${initialOwner}`,
      `  - Deployed by deployer account: ${deployer}`,
      `  - Network: ${network.name}`,
      '',
    ].join('\n'),
  );

  // Save the proxy and implementation contract artifacts
  const artifact = await getArtifact(CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_CONTRACT_NAME);
  const implAddress = await upgrades.erc1967.getImplementationAddress(proxyAddress);

  await save(CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_PROXY_NAME, {
    address: proxyAddress,
    abi: artifact.abi,
  });
  await save(CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_IMPL_NAME, {
    address: implAddress,
    abi: artifact.abi,
  });

  return proxyAddress;
}

// Deploy the ConfidentialTokenWrappersRegistry contract
// Example usage:
// npx hardhat task:deployConfidentialTokenWrappersRegistry --network testnet
task('task:deployConfidentialTokenWrappersRegistry').setAction(async function (_, hre) {
  console.log('Deploying ConfidentialTokenWrappersRegistry contract...\n');

  await deployConfidentialTokenWrappersRegistry(hre);

  console.log('✅ ConfidentialTokenWrappersRegistry contract deployed\n');
});
