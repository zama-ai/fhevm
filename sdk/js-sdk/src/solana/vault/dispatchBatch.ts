import type { Instruction } from '@solana/kit';

import {
  getDispatchInstructionAsync,
  type DispatchAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/dispatch.js';

/**
 * Accounts for the batcher `dispatch` instruction. `batchAuthority` defaults to its PDA and the
 * program ids default to their compiled addresses.
 */
export type SolanaVaultDispatchParameters = DispatchAsyncInput;

/**
 * Builds the permissionless `dispatch` instruction: once a batch is old enough, it burns the batch
 * account's full encrypted balance and records the born-public burned handle the KMS will certify
 * at settle. No arguments.
 */
export async function buildDispatchBatchInstruction(parameters: SolanaVaultDispatchParameters): Promise<Instruction> {
  return getDispatchInstructionAsync(parameters);
}
