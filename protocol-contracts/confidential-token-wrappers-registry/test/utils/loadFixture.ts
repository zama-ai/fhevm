import { CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_PROXY_NAME } from '../../tasks/deploy';
import { ethers } from 'hardhat';
import hre from 'hardhat';

export async function getRegistryFixture() {
  const registryDeployment = await hre.deployments.get(CONFIDENTIAL_TOKEN_WRAPPERS_REGISTRY_PROXY_NAME);
  const registry = await ethers.getContractAt('ConfidentialTokenWrappersRegistry', registryDeployment.address);
  return { registry, registryAddress: registryDeployment.address };
}
