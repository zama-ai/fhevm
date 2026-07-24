import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getClaimInstructionAsync } from './internal/generated/confidentialBatcher/instructions/claim.js';
import {
  claimAmountLineage,
  findBatchAuthorityPda,
  pendingJoinLineage,
  tokenAccountAddress,
} from './internal/batcherPdas.js';
import {
  balanceValueAddress,
  computeSignerAddress,
  tokenEventAuthorityAddress,
  transferredAmountValueAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenLineage.js';

/**
 * Semantic roots for the batcher `claim` instruction. Every other account the on-chain handler
 * validates (`claim.rs`) — the batch authority, the join record, the pending-join and claim-amount
 * value accounts, the payout mint's compute signer, both payout token accounts with their balance /
 * transferred-amount value accounts, and both event authorities — is derived internally from these, so
 * callers never hand-build the account map.
 *
 * `user` is not a signer: claim is a permissionless pull, and the payout can only land in the
 * user's own account. That account (`token_account_address(payoutConfidentialMint, user)`) must
 * already exist — build and send `initializeTokenAccount` for the user once before the first claim.
 */
export type SolanaVaultClaimParameters = {
  /** Pays the rent for the claim-amount value account and the transfer output. Anyone — claim is a permissionless pull. */
  readonly payer: TransactionSigner;
  /** The user being claimed for (pins the join record). Not a signer. */
  readonly user: Address;
  /** Batcher config account. */
  readonly batcher: Address;
  /** The settled batch being claimed from. */
  readonly batch: Address;
  /** Confidential mint claims pay out in (`batcher.payout_confidential_mint`). */
  readonly payoutConfidentialMint: Address;
  /** ZamaHost config PDA (demo-config `hostConfig`). */
  readonly hostConfig: Address;
};

/**
 * Builds the permissionless `claim` instruction: computes the user's exact proportional payout
 * (`encrypted(joined) * payout_received / total_joined`, one MulDiv frame) and transfers it to the
 * user.
 */
export async function buildClaimInstruction(parameters: SolanaVaultClaimParameters): Promise<Instruction> {
  const { user, payoutConfidentialMint } = parameters;
  const [batchAuthority] = await findBatchAuthorityPda({ batch: parameters.batch });
  const batchPayoutTokenAccount = await tokenAccountAddress(payoutConfidentialMint, batchAuthority);
  const userPayoutTokenAccount = await tokenAccountAddress(payoutConfidentialMint, user);
  return getClaimInstructionAsync({
    payer: parameters.payer,
    user,
    batcher: parameters.batcher,
    batch: parameters.batch,
    batchAuthority,
    pendingJoinValue: (await pendingJoinLineage(parameters.batch, batchAuthority, user)).encryptedValueAddress,
    claimAmountValue: (await claimAmountLineage(parameters.batch, batchAuthority, user)).encryptedValueAddress,
    payoutConfidentialMint,
    payoutComputeSigner: await computeSignerAddress(payoutConfidentialMint),
    batchPayoutTokenAccount,
    userPayoutTokenAccount,
    batchPayoutBalanceValue: await balanceValueAddress(payoutConfidentialMint, batchPayoutTokenAccount),
    userPayoutBalanceValue: await balanceValueAddress(payoutConfidentialMint, userPayoutTokenAccount),
    batchPayoutTransferredValue: await transferredAmountValueAddress(payoutConfidentialMint, batchPayoutTokenAccount),
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    hostConfig: parameters.hostConfig,
    confidentialTokenEventAuthority: await tokenEventAuthorityAddress(),
  });
}
