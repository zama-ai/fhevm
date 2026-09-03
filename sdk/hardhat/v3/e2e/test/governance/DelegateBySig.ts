import type { HardhatEthersSigner } from '@nomicfoundation/hardhat-ethers/types';
import { network } from 'hardhat';

import type { ConfidentialERC20Votes } from '../../types/ethers-contracts/index.ts';

const { ethers } = await network.getOrCreate();

/**
 *
 * @param delegator                   Signer from ethers.
 * @param delegatee                   Delegatee address.
 * @param confidentialERC20Votes      ConfidentialERC20Votes token.
 * @param nonce                       Nonce to sign.
 * @param expiry                      Expiry timestamp.
 * @returns                           The signature.
 */
export const delegateBySig = async (
  delegator: HardhatEthersSigner,
  delegatee: string,
  confidentialERC20Votes: ConfidentialERC20Votes,
  nonce: number,
  expiry: number,
): Promise<string> => {
  const { chainId } = await ethers.provider.getNetwork();

  const domain = {
    name: await confidentialERC20Votes.name(),
    version: '1.0',
    chainId: chainId,
    verifyingContract: await confidentialERC20Votes.getAddress(),
  };

  // @dev Delegation(address delegatee,uint256 nonce,uint256 expiry)
  const types = {
    Delegation: [
      {
        name: 'delegatee',
        type: 'address',
      },
      {
        name: 'nonce',
        type: 'uint256',
      },
      {
        name: 'expiry',
        type: 'uint256',
      },
    ],
  };

  const message = {
    delegatee: delegatee,
    nonce: nonce,
    expiry: expiry,
  };

  return await delegator.signTypedData(domain, types, message);
};
