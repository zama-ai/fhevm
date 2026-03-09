import { ethers } from 'hardhat';

import type { EncryptedERC20 } from '../../types/contracts';
import { getSigners } from '../signers';
import { waitForTransactionReceipt } from '../utils';

export async function deployEncryptedERC20Fixture(): Promise<EncryptedERC20> {
  const signers = await getSigners();

  const contractFactory = await ethers.getContractFactory('EncryptedERC20');
  const deployTx = await contractFactory.getDeployTransaction('Naraggara', 'NARA'); // City of Zama's battle
  const tx = await signers.alice.sendTransaction({ ...deployTx, gasLimit: 10_000_000 });
  const receipt = await waitForTransactionReceipt(tx.hash);
  if (!receipt.contractAddress || receipt.status !== 1) {
    throw new Error(`EncryptedERC20 deployment failed: ${tx.hash}`);
  }

  return contractFactory.attach(receipt.contractAddress) as EncryptedERC20;
}
