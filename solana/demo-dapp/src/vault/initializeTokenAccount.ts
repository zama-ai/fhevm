import type { Address, GetAccountInfoApi, Instruction, Rpc, TransactionSigner } from '@solana/kit';

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

const SYSTEM_PROGRAM_ADDRESS = '11111111111111111111111111111111';

/** Returns whether the canonical account is absent or only carries attacker-pre-funded lamports. */
export function needsConfidentialTokenAccountInitialization(accountOwner: Address | null): boolean {
  if (accountOwner === null || accountOwner === SYSTEM_PROGRAM_ADDRESS) return true;
  if (accountOwner === CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS) return false;
  throw new Error('The canonical confidential token account is owned by an unexpected program');
}

/**
 * Builds `confidential_token::initialize_token_account`: creates the owner's confidential token
 * account PDA for `mint` and its zero balance handle. The account PDA, its balance encrypted value account, and
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

/**
 * ATA-like get-or-create helper for a confidential token account. It derives and reads the
 * canonical `(mint, owner)` PDA, returns `null` when it already exists with the expected owner, and
 * otherwise returns the permissionless create-for instruction for the caller to submit. A
 * System-owned pre-funded PDA remains creatable by the on-chain instruction.
 */
export async function getOrCreateConfidentialTokenAccountInstruction(
  rpc: Rpc<GetAccountInfoApi>,
  parameters: SolanaVaultInitializeTokenAccountParameters,
): Promise<Instruction | null> {
  const [tokenAccount] = await findTokenAccountPda({ mint: parameters.mint, owner: parameters.owner });
  const account = await rpc.getAccountInfo(tokenAccount, { commitment: 'confirmed', encoding: 'base64' }).send();
  if (!needsConfidentialTokenAccountInitialization(account.value?.owner ?? null)) return null;
  return buildInitializeTokenAccountInstruction(parameters);
}
