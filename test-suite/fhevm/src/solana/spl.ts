import { findVaultAuthorityPda } from '@fhevm/confidential-token';
// spl — pure SPL/associated-token helpers shared by the typed scenario
// provisioning (`./provision.ts`) and the live demo entrypoints (`demo/seed.ts`,
// `demo/operator-server.ts`). No top-level side effects, so this module is importable by offline
// tests (unlike the demo entrypoints, which run `await main()` against a live validator on
// import).

import type { Address, Instruction, TransactionSigner } from "@solana/kit";
import {
  findAssociatedTokenPda,
  getCreateAssociatedTokenIdempotentInstruction,
  TOKEN_PROGRAM_ADDRESS as SPL_TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";

/** The confidential_token `vault_authority` PDA for a confidential `mint` ([b"vault-authority", mint]). */
export const vaultAuthorityAddress = async (tokenProgram: Address, confidentialMint: Address): Promise<Address> => {
  const [vaultAuthority] = await findVaultAuthorityPda({ mint: confidentialMint }, { programAddress: tokenProgram });
  return vaultAuthority;
};

/**
 * Builds the `CreateIdempotent` for a confidential mint's underlying-token escrow: the associated
 * token account owned by that mint's `vault_authority` PDA and holding the underlying SPL mint —
 * `ATA(vault_authority(confidentialMint), underlyingMint)`.
 *
 * This escrow is exactly the `vault_usdc` account both `wrap_usdc` and `redeem_burned_amount`
 * require, and both REQUIRE it to already exist (they have no `init`; a missing escrow fails on-chain
 * with AnchorError 3012 AccountNotInitialized on `vault_usdc`). The seed must therefore create it
 * before any wrap/redeem — `initialize_vault`/`initialize_mint` do not.
 *
 * Returns the escrow address alongside the instruction so the caller can log/assert it.
 */
export const buildVaultUnderlyingEscrowAtaInstruction = async (params: {
  readonly payer: TransactionSigner;
  readonly tokenProgram: Address;
  readonly confidentialMint: Address;
  readonly underlyingMint: Address;
}): Promise<{ readonly escrow: Address; readonly instruction: Instruction }> => {
  const vaultAuthority = await vaultAuthorityAddress(params.tokenProgram, params.confidentialMint);
  const [escrow] = await findAssociatedTokenPda({
    owner: vaultAuthority,
    tokenProgram: SPL_TOKEN_PROGRAM_ADDRESS,
    mint: params.underlyingMint,
  });
  return {
    escrow,
    instruction: getCreateAssociatedTokenIdempotentInstruction({
      payer: params.payer,
      ata: escrow,
      owner: vaultAuthority,
      mint: params.underlyingMint,
    }),
  };
};
