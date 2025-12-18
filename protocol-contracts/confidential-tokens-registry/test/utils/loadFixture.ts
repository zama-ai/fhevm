import { CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME } from '../../tasks/deploy';
import { ethers } from 'hardhat';
import hre from 'hardhat';

export async function getRegistryFixture() {
  const registryDeployment = await hre.deployments.get(CONFIDENTIAL_TOKENS_REGISTRY_PROXY_NAME);
  const registry = await ethers.getContractAt('ConfidentialTokensRegistry', registryDeployment.address);
  return { registry, registryAddress: registryDeployment.address };
}
