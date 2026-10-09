import { INSTRUCTIONS_SYSVAR_ADDRESS, type TransientStore } from '@fhevm/sdk/solana';
import type { Address, Instruction } from '@solana/kit';

import {
  getQuitInstructionAsync,
  type QuitAsyncInput,
} from './internal/generated/confidentialBatcher/instructions/quit.js';
import { batchApp, tokenApp, withDenyRecords } from './internal/denyRecords.js';
import { type HostPolicyParameters } from './internal/hostPolicy.js';

/**
 * Accounts for the batcher `quit` instruction. `batchAuthority`, `joinRecord`, `hostConfig` and
 * `zamaEventAuthority` default to their PDAs; the batcher/token/system program ids default to their
 * compiled addresses.
 */
export type SolanaVaultQuitParameters = Omit<
  QuitAsyncInput,
  | 'transientStore'
  | 'instructions'
  | 'joinMintHcuBlockMeter'
  | 'joinMintHcuTrustedAppRecord'
  | 'batchHcuBlockMeter'
  | 'batchHcuTrustedAppRecord'
> &
  HostPolicyParameters & {
  readonly transientStore: TransientStore;
  /** Plain addresses: the deny records are derived from them. */
  readonly batch: Address;
  readonly joinConfidentialMint: Address;
};

/**
 * Builds the batcher `quit` instruction: the user leaves a pending batch and is refunded the exact
 * recorded amount. On-chain this spends the user's joined encrypted value account via
 * `confidential_transfer_from_value` (the from-value arm) and resets it to zero — the SDK only
 * builds the batcher instruction; the from-value transfer is a CPI the program makes internally.
 */
export async function buildQuitInstruction(parameters: SolanaVaultQuitParameters): Promise<Instruction> {
  const { transientStore, host, ...accounts } = parameters;
  const joinMint = tokenApp(accounts.joinConfidentialMint);
  const batch = batchApp(accounts.batch);
  const [joinMintHcu, batchHcu] = await Promise.all([host?.hcuAccounts(joinMint), host?.hcuAccounts(batch)]);
  const instruction = await getQuitInstructionAsync({
    ...accounts,
    transientStore: transientStore.address,
    instructions: INSTRUCTIONS_SYSVAR_ADDRESS,
    joinMintHcuBlockMeter: joinMintHcu?.hcuBlockMeter,
    joinMintHcuTrustedAppRecord: joinMintHcu?.hcuTrustedAppRecord,
    batchHcuBlockMeter: batchHcu?.hcuBlockMeter,
    batchHcuTrustedAppRecord: batchHcu?.hcuTrustedAppRecord,
  });
  return withDenyRecords(instruction, host?.denyListEnabled, [joinMint, batch, batch]);
}
