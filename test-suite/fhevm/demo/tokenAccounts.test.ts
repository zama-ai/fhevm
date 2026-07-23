import { describe, expect, test } from "bun:test";

import { AccountRole, generateKeyPairSigner, type Address } from "@solana/kit";

import {
  associatedTokenAddress,
  buildVaultUnderlyingEscrowAtaInstruction,
  createIdempotentAtaInstruction,
  vaultAuthorityAddress,
} from "./tokenAccounts";

const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" as Address;
const SPL_TOKEN_PROGRAM_ADDRESS = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" as Address;
const SYSTEM_PROGRAM_ADDRESS = "11111111111111111111111111111111" as Address;

// Fixed, realistic inputs so the derived escrow is a stable golden: the deployed confidential-token
// program id (matches the on-chain WrapUsdc failure that motivated this escrow) and two valid mints.
const TOKEN_PROGRAM = "pS2gMMq6PNZKpjxiANeoN5XxJgwaFsUR6xaJkpUHcDg" as Address;
const CONFIDENTIAL_MINT = "So11111111111111111111111111111111111111112" as Address;
const UNDERLYING_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" as Address;

describe("vault underlying-token escrow (the wrap_usdc / redeem_burned_amount vault_usdc account)", () => {
  test("derives escrow = ATA(vault_authority(confidentialMint), underlyingMint)", async () => {
    const { escrow } = await buildVaultUnderlyingEscrowAtaInstruction({
      payer: await generateKeyPairSigner(),
      tokenProgram: TOKEN_PROGRAM,
      confidentialMint: CONFIDENTIAL_MINT,
      underlyingMint: UNDERLYING_MINT,
    });
    const vaultAuthority = await vaultAuthorityAddress(TOKEN_PROGRAM, CONFIDENTIAL_MINT);
    const expected = await associatedTokenAddress(vaultAuthority, UNDERLYING_MINT);
    expect(escrow).toBe(expected);
    // Golden: pins the vault_authority PDA + ATA derivation the seed must match the program/SDK on.
    expect(vaultAuthority).toBe("G2Pzm1TT4n9vwcViAMGCwH6of9SaLSkPfm25UdtrNfb4" as Address);
    expect(escrow).toBe("Dte8iCreFgzj26bwLLPgGt5MiyzBeAUcebYRpk5m9uj3" as Address);
  });

  test("builds a CreateIdempotent (tag 1) with the canonical account order and roles", async () => {
    const payer = await generateKeyPairSigner();
    const { escrow, instruction } = await buildVaultUnderlyingEscrowAtaInstruction({
      payer,
      tokenProgram: TOKEN_PROGRAM,
      confidentialMint: CONFIDENTIAL_MINT,
      underlyingMint: UNDERLYING_MINT,
    });
    const vaultAuthority = await vaultAuthorityAddress(TOKEN_PROGRAM, CONFIDENTIAL_MINT);

    expect(instruction.programAddress).toBe(ASSOCIATED_TOKEN_PROGRAM_ADDRESS);
    expect(Array.from(instruction.data ?? [])).toEqual([1]);
    expect(instruction.accounts).toEqual([
      { address: payer.address, role: AccountRole.WRITABLE_SIGNER },
      { address: escrow, role: AccountRole.WRITABLE },
      { address: vaultAuthority, role: AccountRole.READONLY },
      { address: UNDERLYING_MINT, role: AccountRole.READONLY },
      { address: SYSTEM_PROGRAM_ADDRESS as Address, role: AccountRole.READONLY },
      { address: SPL_TOKEN_PROGRAM_ADDRESS as Address, role: AccountRole.READONLY },
    ]);
    // The escrow is owned by the vault_authority PDA, holding the underlying mint — exactly the
    // constraints wrap_usdc enforces (vault_usdc.owner == vault_authority, vault_usdc.mint == underlying).
    expect(instruction.accounts?.[2]?.address).toBe(vaultAuthority);
    expect(instruction.accounts?.[3]?.address).toBe(UNDERLYING_MINT);
  });

  test("createIdempotentAtaInstruction is a no-data tag-1 instruction", () => {
    const payer = { address: "11111111111111111111111111111112" as Address } as never;
    const instruction = createIdempotentAtaInstruction({
      payer,
      ata: CONFIDENTIAL_MINT,
      owner: UNDERLYING_MINT,
      mint: TOKEN_PROGRAM,
    });
    expect(instruction.programAddress).toBe(ASSOCIATED_TOKEN_PROGRAM_ADDRESS);
    expect(Array.from(instruction.data ?? [])).toEqual([1]);
    expect(instruction.accounts).toHaveLength(6);
  });
});
