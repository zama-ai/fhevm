import type { Instruction } from '@solana/kit';

import {
  getClaimInstructionAsync,
  type ClaimAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/claim.js';

/**
 * Accounts for the batcher `claim` instruction. `batchAuthority` and `joinRecord` default to their
 * PDAs; the program ids default to their compiled addresses. `user` is an unchecked account (not a
 * signer): claim is a permissionless pull, and the payout can only land in the user's own account.
 *
 * The user's payout token account (`userPayoutTokenAccount`) must already exist — build and send
 * `initializeTokenAccount` (confidential-token client) for the user once before the first claim.
 */
export type SolanaVaultClaimParameters = ClaimAsyncInput;

/**
 * Builds the permissionless `claim` instruction: computes the user's exact proportional payout
 * (`encrypted(joined) * payout_received / total_joined`, one MulDiv frame) and transfers it to the
 * user. No arguments.
 */
export async function buildClaimInstruction(parameters: SolanaVaultClaimParameters): Promise<Instruction> {
  return getClaimInstructionAsync(parameters);
}
