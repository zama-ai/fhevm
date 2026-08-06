import type { Instruction } from '@solana/kit';

import {
  getInitializeBatcherInstruction,
  type InitializeBatcherInput,
} from './internal/generated/confidentialBatcher/instructions/initializeBatcher.js';
import { BatchDirection } from './internal/generated/confidentialBatcher/types/batchDirection.js';

export { BatchDirection };

/**
 * Accounts + args for `confidential_batcher::initialize_batcher`. The caller supplies the `payer`,
 * a fresh `batcher` keypair signer (the config account created here), the join/payout confidential
 * mints, the `vault` the batcher fronts, `minBatchAgeSlots`, and the `direction` (deposit = join
 * cUSDC → payout cShares, redeem = the swap). The system program defaults to its compiled address.
 */
export type SolanaVaultInitializeBatcherParameters = InitializeBatcherInput;

/**
 * Builds the `initialize_batcher` instruction. `min_batch_age` is measured in SLOTS, not seconds:
 * the seeder converts its desired live window (e.g. ~10 s) to slots (~400 ms/slot on the local
 * test validator) before calling. The seeder assembles and sends the returned instruction.
 */
export function buildInitializeBatcherInstruction(parameters: SolanaVaultInitializeBatcherParameters): Instruction {
  return getInitializeBatcherInstruction(parameters);
}
