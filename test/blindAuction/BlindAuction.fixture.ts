import { AddressLike, BigNumberish, Signer } from 'ethers';
import { ethers } from 'hardhat';

import type { BlindAuction } from '../../types';

export async function deployBlindAuctionFixture(
  account: Signer,
  tokenContract: AddressLike,
  biddingTime: BigNumberish,
  isStoppable: boolean,
): Promise<BlindAuction> {
  const contractFactory = await ethers.getContractFactory('BlindAuction');
  const contract = await contractFactory
    .connect(account)
    .deploy(account.getAddress(), tokenContract, biddingTime, isStoppable);
  await contract.waitForDeployment();
  return contract;
}
