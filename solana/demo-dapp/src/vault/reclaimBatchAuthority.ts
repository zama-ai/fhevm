import type { Instruction } from '@solana/kit';

import {
  getReclaimBatchAuthorityInstructionAsync,
  type ReclaimBatchAuthorityAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/reclaimBatchAuthority.js';

/**
 * Accounts for the batcher `reclaim_batch_authority` instruction. `batchAuthority` defaults to its
 * PDA; the batcher and system program ids default to their compiled addresses. `authority` must be
 * the join mint's wrapper authority (the demo keeper) and receives the lamports.
 */
export type SolanaVaultReclaimBatchAuthorityParameters = ReclaimBatchAuthorityAsyncInput;

/**
 * Builds the `reclaim_batch_authority` instruction: once a batch is settled, canceled or
 * refunding, the whole balance its authority PDA still holds from open/cancel/settle funding goes
 * back to the operator. Claims and quits pay their own rent, so nothing else needs it.
 */
export async function buildReclaimBatchAuthorityInstruction(
  parameters: SolanaVaultReclaimBatchAuthorityParameters,
): Promise<Instruction> {
  return getReclaimBatchAuthorityInstructionAsync(parameters);
}
