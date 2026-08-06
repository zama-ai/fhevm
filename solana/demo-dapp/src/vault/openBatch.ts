import type { Address, Instruction } from '@solana/kit';

import {
  getOpenBatchInstructionAsync,
  type OpenBatchAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/openBatch.js';
import { getCreateLookupTableInstruction, getExtendLookupTableInstructions } from './internal/addressLookupTable.js';

export type SolanaVaultOpenBatchParameters = {
  /** Accounts + `authorityFundingLamports` for the batcher `open_batch` instruction. */
  readonly openBatch: OpenBatchAsyncInput;
  /**
   * A recent, finalized slot used to derive the per-batch settle lookup table address. The table's
   * entries become usable from the NEXT slot, so a table created here is always usable by the later
   * `settle` (which runs at least `min_batch_age_slots` after).
   */
  readonly recentSlot: bigint;
  /**
   * The addresses to seed the settle table with — every one of settle's 34 accounts EXCEPT the fee
   * payer (always static) and `redemption_record` (seeded by the burned handle, which only exists
   * after `dispatch`, so it cannot be in a table frozen now). All 32 are derivable at open time.
   */
  readonly settleLookupTableAddresses: readonly Address[];
};

export type SolanaVaultOpenBatchResult = {
  /**
   * `[open_batch, create_lookup_table, ...extend chunks]`, in submission order. The extend is
   * pre-chunked so no instruction can overflow the transaction wire limit: send `open_batch`
   * alone (it carries ~24 accounts), pair the create with the first extend chunk, then send and
   * confirm each later chunk on its own.
   */
  readonly instructions: readonly Instruction[];
  /** The derived settle lookup table address; pass it to {@link settleBatch}. */
  readonly lookupTableAddress: Address;
  /** The addresses the table now holds; pass them to {@link settleBatch}. */
  readonly lookupTableAddresses: readonly Address[];
};

/**
 * Builds the `open_batch` instruction and the instructions that stand up the batch's settle
 * address lookup table (create + wire-limit-chunked extends). The batch's `payer` doubles as the
 * lookup table authority. The returned table address + addresses feed `settleBatch`.
 */
export async function openBatch(parameters: SolanaVaultOpenBatchParameters): Promise<SolanaVaultOpenBatchResult> {
  const payer = parameters.openBatch.payer;
  const openBatchInstruction = await getOpenBatchInstructionAsync(parameters.openBatch);
  const { instruction: createInstruction, lookupTableAddress } = await getCreateLookupTableInstruction({
    authority: payer,
    payer,
    recentSlot: parameters.recentSlot,
  });
  const extendInstructions = getExtendLookupTableInstructions({
    lookupTable: lookupTableAddress,
    authority: payer,
    payer,
    addresses: parameters.settleLookupTableAddresses,
  });
  return {
    instructions: [openBatchInstruction, createInstruction, ...extendInstructions],
    lookupTableAddress,
    lookupTableAddresses: parameters.settleLookupTableAddresses,
  };
}
