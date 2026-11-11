import type { EncryptedERC20 } from '../../typechain-types';
import { EncryptedERC20__factory } from '../../typechain-types';
import { getSigners } from '../signers';

export async function deployEncryptedERC20Fixture(): Promise<EncryptedERC20> {
  const signers = await getSigners();

  const contract = await new EncryptedERC20__factory(signers.alice).deploy('Naraggara', 'NARA'); // City of Zama's battle
  await contract.waitForDeployment();

  return contract;
}
