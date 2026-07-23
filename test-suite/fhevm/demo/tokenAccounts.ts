// tokenAccounts — pure SPL/associated-token helpers shared by the live demo entrypoints (seed,
// faucet). No top-level side effects, so this module is importable by offline tests (unlike
// `seed.ts` / `faucet-server.ts`, which run `await main()` against a live validator on import).
//
// Layouts cited inline:
//   - Associated-Token `CreateIdempotent` (tag 1): no data args beyond the tag; accounts
//     [payer(ws), ata(w), owner, mint, systemProgram, tokenProgram]. Idempotent = a no-op if the ATA
//     already exists. https://github.com/solana-program/associated-token-account.

import {
  AccountRole,
  getAddressEncoder,
  getProgramDerivedAddress,
  type Address,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";

// Well-known program ids (same literals the SDK's vault `derive.ts` and the other demo scripts use).
const SPL_TOKEN_PROGRAM_ADDRESS = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" as Address;
const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" as Address;
const SYSTEM_PROGRAM_ADDRESS = "11111111111111111111111111111111" as Address;
// confidential_token's `vault_authority` PDA seed prefix ([b"vault-authority", confidential_mint]).
const VAULT_AUTHORITY_SEED = "vault-authority";

const addressEncoder = getAddressEncoder();
const encodeAddress = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

/** Derives the canonical associated token account for `owner` and SPL `mint` (classic token program). */
export const associatedTokenAddress = async (owner: Address, mint: Address): Promise<Address> => {
  const [ata] = await getProgramDerivedAddress({
    programAddress: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
    seeds: [encodeAddress(owner), encodeAddress(SPL_TOKEN_PROGRAM_ADDRESS), encodeAddress(mint)],
  });
  return ata;
};

/** Associated-Token `CreateIdempotent` (tag 1): a no-op when the ATA already exists. */
export const createIdempotentAtaInstruction = (params: {
  readonly payer: TransactionSigner;
  readonly ata: Address;
  readonly owner: Address;
  readonly mint: Address;
}): Instruction => ({
  programAddress: ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
  accounts: [
    { address: params.payer.address, role: AccountRole.WRITABLE_SIGNER },
    { address: params.ata, role: AccountRole.WRITABLE },
    { address: params.owner, role: AccountRole.READONLY },
    { address: params.mint, role: AccountRole.READONLY },
    { address: SYSTEM_PROGRAM_ADDRESS, role: AccountRole.READONLY },
    { address: SPL_TOKEN_PROGRAM_ADDRESS, role: AccountRole.READONLY },
  ],
  data: new Uint8Array([1]),
});

/** The confidential_token `vault_authority` PDA for a confidential `mint` ([b"vault-authority", mint]). */
export const vaultAuthorityAddress = async (tokenProgram: Address, confidentialMint: Address): Promise<Address> => {
  const [vaultAuthority] = await getProgramDerivedAddress({
    programAddress: tokenProgram,
    seeds: [new TextEncoder().encode(VAULT_AUTHORITY_SEED), encodeAddress(confidentialMint)],
  });
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
  const escrow = await associatedTokenAddress(vaultAuthority, params.underlyingMint);
  return {
    escrow,
    instruction: createIdempotentAtaInstruction({
      payer: params.payer,
      ata: escrow,
      owner: vaultAuthority,
      mint: params.underlyingMint,
    }),
  };
};
