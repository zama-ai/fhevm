import { describe, expect, test } from "bun:test";

import { AccountRole, generateKeyPairSigner, type Address } from "@solana/kit";
import { SYSTEM_PROGRAM_ADDRESS } from "@solana-program/system";
import {
  findAssociatedTokenPda,
  ASSOCIATED_TOKEN_PROGRAM_ADDRESS,
  TOKEN_PROGRAM_ADDRESS as SPL_TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";

import {
  buildVaultUnderlyingEscrowAtaInstruction,
  vaultAuthorityAddress,
} from "./spl";

// Fixed, realistic inputs so the derived escrow is a stable golden: the deployed confidential-token
// program id (matches the on-chain WrapUsdc failure that motivated this escrow) and two valid mints.
const TOKEN_PROGRAM = "FAWs7E52LZmXR5YzFy4aXanfBjNtXV2qooQVtkmBa3cL" as Address;
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
    const [expected] = await findAssociatedTokenPda({
      owner: vaultAuthority,
      tokenProgram: SPL_TOKEN_PROGRAM_ADDRESS,
      mint: UNDERLYING_MINT,
    });
    expect(escrow).toBe(expected);
    // Golden: pins the vault_authority PDA + ATA derivation the seed must match the program/SDK on.
    expect(vaultAuthority).toBe("Y5emEtkuiaUP9HgUujdsyWHrqrYyBkYox9E58ZX9kHc" as Address);
    expect(escrow).toBe("ErpU2FQWEbT1ESXxEC4J8SKdsHgzwcYDtBCxS5YDGjBt" as Address);
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
    expect(instruction.accounts?.map(({ address, role }) => ({ address, role }))).toEqual([
      { address: payer.address, role: AccountRole.WRITABLE_SIGNER },
      { address: escrow, role: AccountRole.WRITABLE },
      { address: vaultAuthority, role: AccountRole.READONLY },
      { address: UNDERLYING_MINT, role: AccountRole.READONLY },
      { address: SYSTEM_PROGRAM_ADDRESS as Address, role: AccountRole.READONLY },
      { address: SPL_TOKEN_PROGRAM_ADDRESS as Address, role: AccountRole.READONLY },
    ]);
    expect(instruction.accounts?.[0]).toHaveProperty("signer", payer);
    // The escrow is owned by the vault_authority PDA, holding the underlying mint — exactly the
    // constraints wrap_usdc enforces (vault_usdc.owner == vault_authority, vault_usdc.mint == underlying).
    expect(instruction.accounts?.[2]?.address).toBe(vaultAuthority);
    expect(instruction.accounts?.[3]?.address).toBe(UNDERLYING_MINT);
  });
});
