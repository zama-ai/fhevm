import { describe, expect, test } from "bun:test";

import { AccountRole, generateKeyPairSigner, type Address } from "@solana/kit";

import {
  SPL_MINT_ACCOUNT_SPACE,
  associatedTokenAddress,
  buildVaultUnderlyingEscrowAtaInstruction,
  createAccountInstruction,
  createIdempotentAtaInstruction,
  initializeMint2Instruction,
  mintToInstruction,
  requestHeapFrameInstruction,
  setComputeUnitLimitInstruction,
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

describe("hand-built System/SPL/ComputeBudget instruction layouts", () => {
  test("createAccountInstruction encodes tag 0, lamports, space, and the owner program", async () => {
    const payer = await generateKeyPairSigner();
    const newAccount = await generateKeyPairSigner();
    const instruction = createAccountInstruction({
      payer,
      newAccount,
      lamports: 1_461_600n,
      space: SPL_MINT_ACCOUNT_SPACE,
      owner: SPL_TOKEN_PROGRAM_ADDRESS,
    });
    expect(instruction.programAddress).toBe(SYSTEM_PROGRAM_ADDRESS);
    const data = instruction.data ?? new Uint8Array();
    const view = new DataView(data.buffer, data.byteOffset);
    expect(view.getUint32(0, true)).toBe(0);
    expect(view.getBigUint64(4, true)).toBe(1_461_600n);
    expect(view.getBigUint64(12, true)).toBe(82n);
    // Both the payer and the created account must sign their own creation.
    expect(instruction.accounts?.map((meta) => meta.role)).toEqual([
      AccountRole.WRITABLE_SIGNER,
      AccountRole.WRITABLE_SIGNER,
    ]);
    expect(instruction.accounts?.map((meta) => meta.address)).toEqual([payer.address, newAccount.address]);
  });

  test("initializeMint2Instruction encodes tag 20, decimals, authority, and no freeze authority", () => {
    const instruction = initializeMint2Instruction({
      mint: CONFIDENTIAL_MINT,
      decimals: 9,
      mintAuthority: UNDERLYING_MINT,
    });
    expect(instruction.programAddress).toBe(SPL_TOKEN_PROGRAM_ADDRESS);
    const data = instruction.data ?? new Uint8Array();
    expect(data).toHaveLength(35);
    expect(data[0]).toBe(20);
    expect(data[1]).toBe(9);
    expect(data[34]).toBe(0); // freeze authority COption::None
    expect(instruction.accounts).toEqual([{ address: CONFIDENTIAL_MINT, role: AccountRole.WRITABLE }]);
  });

  test("mintToInstruction encodes tag 7 + u64-le amount with [mint(w), destination(w), authority(s)]", async () => {
    const authority = await generateKeyPairSigner();
    const instruction = mintToInstruction({
      mint: CONFIDENTIAL_MINT,
      destination: UNDERLYING_MINT,
      authority,
      baseUnits: 1_000_000n,
    });
    expect(instruction.programAddress).toBe(SPL_TOKEN_PROGRAM_ADDRESS);
    const data = instruction.data ?? new Uint8Array();
    expect(data[0]).toBe(7);
    expect(new DataView(data.buffer, data.byteOffset).getBigUint64(1, true)).toBe(1_000_000n);
    expect(instruction.accounts).toEqual([
      { address: CONFIDENTIAL_MINT, role: AccountRole.WRITABLE },
      { address: UNDERLYING_MINT, role: AccountRole.WRITABLE },
      { address: authority.address, role: AccountRole.READONLY_SIGNER },
    ]);
  });

  test("setComputeUnitLimitInstruction encodes tag 2 + u32-le units with no accounts", () => {
    const instruction = setComputeUnitLimitInstruction(1_400_000);
    expect(instruction.programAddress).toBe("ComputeBudget111111111111111111111111111111" as Address);
    const data = instruction.data ?? new Uint8Array();
    expect(data[0]).toBe(2);
    expect(new DataView(data.buffer, data.byteOffset).getUint32(1, true)).toBe(1_400_000);
    expect(instruction.accounts).toBeUndefined();
  });

  test("requestHeapFrameInstruction encodes tag 1 + u32-le bytes with no accounts", () => {
    const instruction = requestHeapFrameInstruction(256 * 1024);
    expect(instruction.programAddress).toBe("ComputeBudget111111111111111111111111111111" as Address);
    const data = instruction.data ?? new Uint8Array();
    expect(data[0]).toBe(1);
    expect(new DataView(data.buffer, data.byteOffset).getUint32(1, true)).toBe(262_144);
    expect(instruction.accounts).toBeUndefined();
  });
});
