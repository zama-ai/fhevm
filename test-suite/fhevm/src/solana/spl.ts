// spl — pure SPL/associated-token/system-program helpers shared by the typed scenario
// provisioning (`./provision.ts`) and the live demo entrypoints (`demo/seed.ts`,
// `demo/faucet-server.ts`). No top-level side effects, so this module is importable by offline
// tests (unlike the demo entrypoints, which run `await main()` against a live validator on
// import).
//
// The instructions are hand-built with `@solana/kit` primitives on purpose: the test-suite carries
// no `@solana-program/token` dependency. Layouts cited inline:
//   - Associated-Token `CreateIdempotent` (tag 1): no data args beyond the tag; accounts
//     [payer(ws), ata(w), owner, mint, systemProgram, tokenProgram]. Idempotent = a no-op if the ATA
//     already exists. https://github.com/solana-program/associated-token-account.
//   - System `CreateAccount` (tag 0 as u32): data = [0:u32, lamports:u64, space:u64, owner:32];
//     accounts [payer(ws), newAccount(ws)] — both sign.
//   - SPL Token `InitializeMint2` (tag 20): data = [20, decimals, mintAuthority:32, freezeOption];
//     accounts [mint(w)]. https://github.com/solana-program/token — `Instruction::InitializeMint2`.
//   - SPL Token `MintTo` (tag 7): data = [7, amount:u64-le]; accounts
//     [mint(w), destination(w), authority(s)]. Same source, `Instruction::MintTo`.
//   - ComputeBudget `SetComputeUnitLimit` (tag 2): data = [2, units:u32-le]; no accounts.

import {
  AccountRole,
  getAddressEncoder,
  getProgramDerivedAddress,
  type AccountMeta,
  type Address,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";

// Well-known program ids (same literals the SDK's vault `derive.ts` and the other demo scripts use).
export const SPL_TOKEN_PROGRAM_ADDRESS = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" as Address;
const ASSOCIATED_TOKEN_PROGRAM_ADDRESS = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" as Address;
const SYSTEM_PROGRAM_ADDRESS = "11111111111111111111111111111111" as Address;
const COMPUTE_BUDGET_PROGRAM_ADDRESS = "ComputeBudget111111111111111111111111111111" as Address;
/** SPL Token `Mint` account length — the `space` for `CreateAccount` before `InitializeMint2`. */
export const SPL_MINT_ACCOUNT_SPACE = 82n;
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

/**
 * A signer account meta: the `signer` field rides along at runtime so `signTransactionMessageWithSigners`
 * produces the signature, while the meta stays typed as a plain `AccountMeta` (same shape the demo
 * dapp's address-lookup-table builder uses). Used for the hand-built System `CreateAccount`, whose
 * keypair must sign its own creation.
 */
const signerMeta = (signer: TransactionSigner, role: AccountRole): AccountMeta =>
  ({ address: signer.address, role, signer }) as unknown as AccountMeta;

/** System `CreateAccount` (tag 0), signed by both the payer and the new account's keypair. */
export const createAccountInstruction = (parameters: {
  readonly payer: TransactionSigner;
  readonly newAccount: TransactionSigner;
  readonly lamports: bigint;
  readonly space: bigint;
  readonly owner: Address;
}): Instruction => {
  const data = new Uint8Array(4 + 8 + 8 + 32);
  const view = new DataView(data.buffer);
  view.setUint32(0, 0, true); // instruction index 0 = CreateAccount
  view.setBigUint64(4, parameters.lamports, true);
  view.setBigUint64(12, parameters.space, true);
  data.set(encodeAddress(parameters.owner), 20);
  return {
    programAddress: SYSTEM_PROGRAM_ADDRESS,
    accounts: [
      signerMeta(parameters.payer, AccountRole.WRITABLE_SIGNER),
      signerMeta(parameters.newAccount, AccountRole.WRITABLE_SIGNER),
    ],
    data,
  };
};

/** SPL Token `InitializeMint2` (tag 20): sets decimals + mint authority, no freeze authority. */
export const initializeMint2Instruction = (parameters: {
  readonly mint: Address;
  readonly decimals: number;
  readonly mintAuthority: Address;
}): Instruction => {
  const data = new Uint8Array(1 + 1 + 32 + 1);
  data[0] = 20;
  data[1] = parameters.decimals;
  data.set(encodeAddress(parameters.mintAuthority), 2);
  data[34] = 0; // freeze authority COption::None
  return {
    programAddress: SPL_TOKEN_PROGRAM_ADDRESS,
    accounts: [{ address: parameters.mint, role: AccountRole.WRITABLE }],
    data,
  };
};

/** SPL Token `MintTo` (tag 7): mints `baseUnits` to `destination`, signed by the mint authority. */
export const mintToInstruction = (params: {
  readonly mint: Address;
  readonly destination: Address;
  readonly authority: TransactionSigner;
  readonly baseUnits: bigint;
}): Instruction => {
  const data = new Uint8Array(9);
  data[0] = 7;
  new DataView(data.buffer).setBigUint64(1, params.baseUnits, true);
  return {
    programAddress: SPL_TOKEN_PROGRAM_ADDRESS,
    accounts: [
      { address: params.mint, role: AccountRole.WRITABLE },
      { address: params.destination, role: AccountRole.WRITABLE },
      { address: params.authority.address, role: AccountRole.READONLY_SIGNER },
    ],
    data,
  };
};

/** ComputeBudget `SetComputeUnitLimit` (tag 2): raises the per-tx CU ceiling for the FHE-heavy CPIs. */
export const setComputeUnitLimitInstruction = (units: number): Instruction => {
  const data = new Uint8Array(5);
  data[0] = 2;
  new DataView(data.buffer).setUint32(1, units, true);
  return { programAddress: COMPUTE_BUDGET_PROGRAM_ADDRESS, data };
};

// No `RequestHeapFrame` helper: the request is granted and then ignored. Anchor's entrypoint
// installs an allocator hard-wired to `solana_program_entrypoint::HEAP_LENGTH` (32 KB) unless the
// program declares `custom-heap`, and none of ours do — so a larger frame is never used, and
// lifting the real ceiling needs a program that owns its allocator (fhevm-internal#1872).

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
