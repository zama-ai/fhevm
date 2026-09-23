import { ethers } from 'ethers';

export type ManagedWallet = ethers.NonceManager & { address: string; reset: () => void };

export function wrapWithNonceManager(wallet: ethers.Wallet): ManagedWallet {
  return Object.assign(new ethers.NonceManager(wallet), { address: wallet.address });
}

/** A new shared CREATE nonce domain, independent of earlier activity on either chain. */
export async function createFundedDeployers(
  funders: readonly ethers.Signer[],
  amount: bigint = ethers.parseEther('1'),
): Promise<ManagedWallet[]> {
  if (funders.length < 2 || amount <= 0n || funders.some((funder) => !funder.provider)) {
    throw new Error('Fresh multi-chain deployers require connected funders on both chains and positive funding');
  }
  const key = ethers.Wallet.createRandom().privateKey;
  const deployers = funders.map((funder) => wrapWithNonceManager(new ethers.Wallet(key, funder.provider)));
  // Funding uses the existing account's nonce; deployment uses the fresh key's
  // nonce zero on each chain. Finish both funding receipts before deploying.
  for (const [index, funder] of funders.entries()) {
    const deployer = deployers[index];
    const transaction = await funder.sendTransaction({ to: deployer.address, value: amount });
    const receipt = await transaction.wait();
    if (receipt?.status !== 1 || (await funder.provider!.getBalance(deployer.address)) < amount) {
      throw new Error(`Fresh multi-chain deployer funding was not confirmed on chain ${index}`);
    }
    if ((await deployer.getNonce('pending')) !== 0) {
      throw new Error(`Fresh multi-chain deployer already has transaction history on chain ${index}`);
    }
  }
  return deployers;
}
