import { ethers } from 'hardhat';
import hre from 'hardhat';
import { CONTRACT_NAME, getConfidentialWrapperProxyName } from '../../tasks/deploy';
import { getRequiredEnvVar } from '../../tasks/utils/loadVariables';
import { ConfidentialWrapper } from '../../types';

export async function getConfidentialWrappersFixture(): Promise<{ confidentialWrappers: ConfidentialWrapper[] }> {
  const numWrappers = parseInt(getRequiredEnvVar('NUM_CONFIDENTIAL_WRAPPERS'));
  const confidentialWrappers = [];
  for (let i = 0; i < numWrappers; i++) {
    const name = getRequiredEnvVar(`CONFIDENTIAL_WRAPPER_NAME_${i}`);
    const confidentialWrapperProxy = await hre.deployments.get(getConfidentialWrapperProxyName(name));
    confidentialWrappers.push(await ethers.getContractAt(CONTRACT_NAME, confidentialWrapperProxy.address));
  }
  return { confidentialWrappers };
}
