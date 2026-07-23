import type { Instruction } from '@solana/kit';

import {
  getInitializeVaultInstructionAsync,
  type InitializeVaultAsyncInput,
} from './internal/generated/demoVault/instructions/initializeVault.js';

/**
 * Accounts for `demo_vault::initialize_vault`. The caller supplies the `payer`, a fresh `vault`
 * keypair signer (the account created here), and the `underlyingMint`; the vault-seeded PDAs
 * (`vaultAuthority`, `shareMint`, `vaultTokenAccount`) and the program ids default to their
 * derived/compiled values.
 */
export type SolanaVaultInitializeVaultParameters = InitializeVaultAsyncInput;

/**
 * Builds the demo-vault `initialize_vault` instruction: creates the vault state account, its
 * share mint (same decimals as the underlying, vault-authority PDA as mint authority), and the
 * program-owned underlying token account. The seeder assembles and sends it.
 */
export async function buildInitializeVaultInstruction(
  parameters: SolanaVaultInitializeVaultParameters,
): Promise<Instruction> {
  return getInitializeVaultInstructionAsync(parameters);
}
