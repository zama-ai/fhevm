import { findVaultAuthorityPda } from '@fhevm/confidential-token';
// spl — pure SPL/associated-token/system-program helpers shared by the typed scenario
// provisioning (`./provision.ts`) and the live demo entrypoints (`demo/seed.ts`,
// `demo/operator-server.ts`). No top-level side effects, so this module is importable by offline
// tests (unlike the demo entrypoints, which run `await main()` against a live validator on
// import).

import {
  AccountRole,
  getAddressEncoder,
  type AccountMeta,
  type Address,
  type Instruction,
  type TransactionSigner,
} from "@solana/kit";
import {
  findAssociatedTokenPda,
  getCreateAssociatedTokenIdempotentInstruction,
  TOKEN_PROGRAM_ADDRESS as SPL_TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";

const SYSTEM_PROGRAM_ADDRESS = "11111111111111111111111111111111" as Address;
const COMPUTE_BUDGET_PROGRAM_ADDRESS = "ComputeBudget111111111111111111111111111111" as Address;

const addressEncoder = getAddressEncoder();
const encodeAddress = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

/**
 * A signer account meta: the `signer` field rides along at runtime so `signTransactionMessageWithSigners`
 * produces the signature, while the meta stays typed as a plain `AccountMeta`. The hand-built
 * System `CreateAccount` needs its new keypair to sign its own creation.
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

/** System `Transfer` (index 2): moves `lamports` from the signing `from` to `to`. */
export const transferSolInstruction = (parameters: {
  readonly from: TransactionSigner;
  readonly to: Address;
  readonly lamports: bigint;
}): Instruction => {
  const data = new Uint8Array(4 + 8);
  const view = new DataView(data.buffer);
  view.setUint32(0, 2, true);
  view.setBigUint64(4, parameters.lamports, true);
  return {
    programAddress: SYSTEM_PROGRAM_ADDRESS,
    accounts: [
      signerMeta(parameters.from, AccountRole.WRITABLE_SIGNER),
      { address: parameters.to, role: AccountRole.WRITABLE },
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
// program declares `custom-heap`, and none of ours do — so a larger heap frame is never used, and
// lifting the real ceiling needs a program that owns its allocator (fhevm-internal#1872).

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
