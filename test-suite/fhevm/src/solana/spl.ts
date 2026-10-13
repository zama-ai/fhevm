import { findVaultAuthorityPda } from '@fhevm/confidential-token';
// spl — the SPL/associated-token helper shared by the typed scenario provisioning (`./provision.ts`)
// and the demo seed (`demo/seed.ts`).

import type { Address, Instruction, TransactionSigner } from "@solana/kit";
import {
  findAssociatedTokenPda,
  getCreateAssociatedTokenIdempotentInstruction,
  TOKEN_PROGRAM_ADDRESS as SPL_TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";

/**
 * Builds the `CreateIdempotent` for a confidential mint's underlying-token escrow: the associated
 * token account owned by that mint's `vault_authority` PDA and holding the underlying SPL mint —
 * `ATA(vault_authority(confidentialMint), underlyingMint)`.
 *
 * This escrow is exactly the `vault_usdc` account both `wrap_usdc` and `redeem_burned_amount`
 * require, and both REQUIRE it to already exist (they have no `init`; a missing escrow fails on-chain
 * with AnchorError 3012 AccountNotInitialized on `vault_usdc`). The seed must therefore create it
 * before any wrap/redeem — `initialize_vault`/`initialize_mint` do not.
 */
export const buildVaultUnderlyingEscrowAtaInstruction = async (params: {
  readonly payer: TransactionSigner;
  readonly tokenProgram: Address;
  readonly confidentialMint: Address;
  readonly underlyingMint: Address;
}): Promise<Instruction> => {
  const [vaultAuthority] = await findVaultAuthorityPda(
    { mint: params.confidentialMint },
    { programAddress: params.tokenProgram },
  );
  const [escrow] = await findAssociatedTokenPda({
    owner: vaultAuthority,
    tokenProgram: SPL_TOKEN_PROGRAM_ADDRESS,
    mint: params.underlyingMint,
  });
  return getCreateAssociatedTokenIdempotentInstruction({
    payer: params.payer,
    ata: escrow,
    owner: vaultAuthority,
    mint: params.underlyingMint,
  });
};
