import type { Address, Instruction, TransactionSigner } from '@solana/kit';

import { getWrapUsdcInstructionAsync } from '../internal/generated/confidentialToken/instructions/wrapUsdc.js';
import { findTokenAccountPda } from '../internal/generated/confidentialToken/pdas/tokenAccount.js';
import { findVaultAuthorityPda as findMintVaultAuthorityPda } from '../internal/generated/confidentialToken/pdas/vaultAuthority.js';
import { findTotalSupplyAuthorityPda } from '../internal/generated/confidentialToken/pdas/totalSupplyAuthority.js';
import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS } from '../internal/generated/confidentialToken/programAddress.js';
import {
  associatedTokenAddress,
  balanceValueAddress,
  totalSupplyValueAddress,
  tokenEventAuthorityAddress,
  zamaEventAuthorityAddress,
} from './internal/tokenLineage.js';

export type SolanaVaultWrapUsdcParameters = {
  /** Token owner and transfer authority. */
  readonly owner: TransactionSigner;
  /** The confidential mint whose balance is increased (e.g. cUSDC). */
  readonly mint: Address;
  /** The underlying SPL mint being escrowed (e.g. mock USDC). */
  readonly underlyingMint: Address;
  /** zama-host config PDA used for handle derivation. */
  readonly hostConfig: Address;
  /** Public underlying amount to escrow and rotate into the confidential balance. */
  readonly amount: number | bigint;
};

/**
 * Builds `confidential_token::wrap_usdc`: escrows a PUBLIC `amount` of `underlyingMint` from the
 * owner's associated token account and rotates the owner's confidential balance by that amount. The
 * amount is public at the wrap boundary, so — unlike a confidential transfer — this needs NO input
 * proof. The owner's confidential token account, the program's underlying vault, both durable
 * lineage accounts, and the two Anchor event authorities are derived here from the mints and owner;
 * the seeder/scenario supplies only semantic roots and assembles/sends the returned instruction.
 */
export async function buildWrapUsdcInstruction(parameters: SolanaVaultWrapUsdcParameters): Promise<Instruction> {
  const { owner, mint, underlyingMint } = parameters;
  const [tokenAccount] = await findTokenAccountPda({ mint, owner: owner.address });
  const [mintVaultAuthority] = await findMintVaultAuthorityPda({ mint });
  const [totalSupplyAuthority] = await findTotalSupplyAuthorityPda({ mint });
  return getWrapUsdcInstructionAsync({
    owner,
    mint,
    tokenAccount,
    underlyingMint,
    userUsdc: await associatedTokenAddress(owner.address, underlyingMint),
    vaultUsdc: await associatedTokenAddress(mintVaultAuthority, underlyingMint),
    balanceValue: await balanceValueAddress(mint, tokenAccount),
    totalSupplyValue: await totalSupplyValueAddress(mint, totalSupplyAuthority),
    zamaEventAuthority: await zamaEventAuthorityAddress(),
    hostConfig: parameters.hostConfig,
    eventAuthority: await tokenEventAuthorityAddress(),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    amount: parameters.amount,
  });
}
