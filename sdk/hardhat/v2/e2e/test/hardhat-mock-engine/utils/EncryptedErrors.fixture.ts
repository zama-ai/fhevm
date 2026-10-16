import type { Signer } from 'ethers';
import * as hre from 'hardhat';

import type { TestEncryptedErrors } from '../../../typechain-types';

export async function deployEncryptedErrors(account: Signer, numberErrors: number): Promise<TestEncryptedErrors> {
  const contractFactory = await hre.ethers.getContractFactory('TestEncryptedErrors');
  const contract = await contractFactory.connect(account).deploy(numberErrors);
  await contract.waitForDeployment();
  return contract;
}
