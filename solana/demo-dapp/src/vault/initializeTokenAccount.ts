import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getInitializeTokenAccountInstructionAsync } from './internal/generated/confidentialToken/instructions/initializeTokenAccount.js';
import { findTokenAccountPda } from './internal/generated/confidentialToken/pdas/tokenAccount.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from './internal/generated/confidentialToken/programAddress.js';
import {
  balanceValueAddress,
  tokenEventAuthorityAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenValueAccount.js';

export type SolanaVaultInitializeTokenAccountParameters = {
  /** Signer funding the new confidential account and encrypted balance. */
  readonly payer: TransactionSigner;
  /** Owner of the confidential account. This address does not need to sign. */
  readonly owner: Address;
  /** The confidential mint this account belongs to. */
  readonly mint: Address;
  /** zama-host config PDA used for handle derivation. */
  readonly hostConfig: Address;
};

/**
 * Builds `confidential_token::initialize_token_account`: creates the owner's confidential token
 * account PDA for `mint` and its zero balance handle. The account PDA, its balance value_account, and
 * the two Anchor event authorities are derived here from `(mint, owner)`. The seeder assembles and
 * sends the returned instruction.
 */
export async function buildInitializeTokenAccountInstruction(
  parameters: SolanaVaultInitializeTokenAccountParameters,
): Promise<Instruction> {
  const [tokenAccount] = await findTokenAccountPda({ mint: parameters.mint, owner: parameters.owner });
  return getInitializeTokenAccountInstructionAsync({
    payer: parameters.payer,
    owner: parameters.owner,
    mint: parameters.mint,
    tokenAccount,
    balanceEncryptedValue: await balanceValueAddress(parameters.mint, tokenAccount),
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    hostConfig: parameters.hostConfig,
    eventAuthority: await tokenEventAuthorityAddress(),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  });
}
