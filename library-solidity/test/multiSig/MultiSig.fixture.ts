import type { SimpleMultiSig } from '../../typechain-types/index.js';
import { ethers } from '../hardhat.js';
import { getSigners } from '../signers.js';

export async function deploySimpleMultiSigFixture(): Promise<SimpleMultiSig> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('SimpleMultiSig');
  const contract = await contractFactory.connect(signers.alice).deploy([signers.alice, signers.bob, signers.carol]); // City of Zama's battle
  await contract.waitForDeployment();

  return contract;
}
