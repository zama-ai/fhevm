import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Instruction } from '@solana/kit';

import {
  getQuitInstructionAsync,
  type QuitAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/quit.js';
import { batchApp, tokenApp, withDenyRecords, type DenyListParameters } from './internal/denyRecords.js';

/**
 * Accounts for the batcher `quit` instruction. `batchAuthority`, `joinRecord`, `hostConfig` and
 * `zamaEventAuthority` default to their PDAs; the batcher/token/system program ids default to their
 * compiled addresses.
 */
export type SolanaVaultQuitParameters = Omit<QuitAsyncInput, 'transientStore' | 'instructions'> &
  DenyListParameters & {
  readonly transientStore: TransientStore;
};

/**
 * Builds the batcher `quit` instruction: the user leaves a pending batch and is refunded the exact
 * recorded amount. On-chain this spends the user's joined encrypted value account via
 * `confidential_transfer_from_value` (the from-value arm) and resets it to zero — the SDK only
 * builds the batcher instruction; the from-value transfer is a CPI the program makes internally.
 */
export async function buildQuitInstruction(parameters: SolanaVaultQuitParameters): Promise<Instruction> {
  const { transientStore, denyListEnabled, ...accounts } = parameters;
  const instruction = await getQuitInstructionAsync({
    ...accounts,
    transientStore: transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
  });
  const batch = batchApp(accounts.batch);
  return withDenyRecords(instruction, denyListEnabled, [tokenApp(accounts.joinConfidentialMint), batch, batch]);
}
