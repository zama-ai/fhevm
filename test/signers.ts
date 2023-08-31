import type { SignerWithAddress } from '@nomicfoundation/hardhat-ethers/dist/src/signer-with-address';
import { ethers } from 'hardhat';

export interface Signers {
  alice: SignerWithAddress;
  bob: SignerWithAddress;
  carol: SignerWithAddress;
  dave: SignerWithAddress;
}

export const getSigners = async (): Promise<Signers> => {
  const signers = await ethers.getSigners();
  return {
    alice: signers[0],
    bob: signers[1],
    carol: signers[2],
    dave: signers[3],
  };
};
