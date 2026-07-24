import type { Instruction } from '@solana/kit';

import {
  getQuitInstructionAsync,
  type QuitAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/quit.js';

/**
 * Accounts for the batcher `quit` instruction. `batchAuthority` and `joinRecord` default to their
 * PDAs; the batcher/token/system program ids default to their compiled addresses.
 */
export type SolanaVaultQuitParameters = QuitAsyncInput;

/**
 * Builds the batcher `quit` instruction: the user leaves a pending batch and is refunded the exact
 * recorded amount. On-chain this spends the user's joined encrypted value account via
 * `confidential_transfer_from_value` (the from-value arm) and resets it to zero — the SDK only
 * builds the batcher instruction; the from-value transfer is a CPI the program makes internally.
 */
export async function buildQuitInstruction(parameters: SolanaVaultQuitParameters): Promise<Instruction> {
  return getQuitInstructionAsync(parameters);
}
