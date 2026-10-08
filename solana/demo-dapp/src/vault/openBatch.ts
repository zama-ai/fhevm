import type { Address, Instruction } from '@solana/kit';
import {
  getOpenBatchInstructionAsync,
  type OpenBatchAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/openBatch.js';
import { tokenApp, withDenyRecords, type DenyListParameters } from './internal/denyRecords.js';

export type SolanaVaultOpenBatchParameters = DenyListParameters & {
  /**
   * Accounts, `index` + `authorityFundingLamports` for the batcher `open_batch` instruction. The
   * mints are plain addresses: the deny records are derived from them.
   */
  readonly openBatch: OpenBatchAsyncInput & {
    readonly joinConfidentialMint: Address;
    readonly payoutConfidentialMint: Address;
  };
};

/** Builds the `open_batch` instruction with the deny records of both of the batch's mints. */
export async function openBatch(parameters: SolanaVaultOpenBatchParameters): Promise<Instruction> {
  return withDenyRecords(
    await getOpenBatchInstructionAsync(parameters.openBatch),
    parameters.denyListEnabled,
    [tokenApp(parameters.openBatch.joinConfidentialMint), tokenApp(parameters.openBatch.payoutConfidentialMint)],
  );
}
