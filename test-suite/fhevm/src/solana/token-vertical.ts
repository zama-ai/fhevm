// token-vertical — the typed confidential-token consume arc the token scenario drives:
// burn (attested external amount) -> seal -> KMS-certified public decrypt -> redeem + disclose.
//
// The burn/redeem instructions come from the test-suite's own Codama confidential-token client
// (they are deliberately absent from the demo-dapp's client — the demo never burns or redeems
// directly); the seal and disclose steps reuse the vault module's product builders. Everything
// the retired live-client derived with anchor-client (token account, pending burn, escrow ATAs,
// KMS context) derives here from the same seeds.

import { getProgramDerivedAddress, type Address, type TransactionSigner } from "@solana/kit";

import { associatedTokenAddress, requestHeapFrameInstruction } from "../../demo/tokenAccounts";
import {
  getConfidentialBurnInstructionAsync,
  getRedeemBurnedAmountInstructionAsync,
} from "./internal/generated/confidentialToken/instructions/index.js";
import { findTotalSupplyAuthorityPda } from "./internal/generated/confidentialToken/pdas/totalSupplyAuthority.js";
import { findVaultAuthorityPda } from "./internal/generated/confidentialToken/pdas/vaultAuthority.js";
import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from "./internal/generated/confidentialToken/programAddress.js";
import type { CoprocessorInputAttestationArgs } from "./internal/generated/confidentialToken/types/index.js";
import type { PublicDecryptClaim } from "./public-decrypt";
import { hostConfigAddress, type SolanaProvisioningContext } from "./provision";

type VaultModule = typeof import("@demo-dapp/vault/index.js");
let vaultModulePromise: Promise<VaultModule> | undefined;
const vaultModule = (): Promise<VaultModule> => (vaultModulePromise ??= import("@demo-dapp/vault/index.js"));

type SdkVerifyModule = typeof import("@sdk-src/solana/actions/verifyPublicDecrypt.js");
let sdkVerifyModulePromise: Promise<SdkVerifyModule> | undefined;
const sdkVerifyModule = (): Promise<SdkVerifyModule> =>
  (sdkVerifyModulePromise ??= import("@sdk-src/solana/actions/verifyPublicDecrypt.js"));

/** The KMS context id `fhevm-cli up` provisions (`demo/seed.ts` BRINGUP_KMS_CONTEXT_ID). */
export const BRINGUP_KMS_CONTEXT_ID = 1n;

/** The zama-host KMS-context PDA for `contextId` (`["kms-context", u64-le id]`). */
export const kmsContextAddress = async (contextId: bigint = BRINGUP_KMS_CONTEXT_ID): Promise<Address> => {
  const idBytes = new Uint8Array(8);
  new DataView(idBytes.buffer).setBigUint64(0, contextId, true);
  const [kmsContext] = await getProgramDerivedAddress({
    programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [new TextEncoder().encode("kms-context"), idBytes],
  });
  return kmsContext;
};

const eventAuthority = async (programAddress: Address): Promise<Address> => {
  const [address] = await getProgramDerivedAddress({
    programAddress,
    seeds: [new TextEncoder().encode("__event_authority")],
  });
  return address;
};

// Byte-identical to `confidential_token::state`: the labels of the token-account encrypted value
// accounts a burn writes (balance in place, burned amount created-public).
const TOTAL_SUPPLY_LABEL = new TextEncoder().encode("total_supply____________________");

/** The identities a burn touches for `(mint, owner)`, all derived from the same on-chain seeds. */
export type ConfidentialBurnTarget = {
  readonly tokenAccount: Address;
  readonly pendingBurn: Address;
  /** The burned-amount encrypted value account: the certificate's aclValueKey + its PDA. */
  readonly burnedAmount: { readonly aclValueKey: Uint8Array; readonly encryptedValueAddress: Address };
};

/** Derives the burn-facing accounts for `owner`'s confidential token account under `mint`. */
export const confidentialBurnTarget = async (mint: Address, owner: Address): Promise<ConfidentialBurnTarget> => {
  const vault = await vaultModule();
  const tokenAccount = await vault.tokenAccountAddress(mint, owner);
  return {
    tokenAccount,
    pendingBurn: await vault.pendingBurnAddress(mint, tokenAccount),
    burnedAmount: await vault.burnedAmountValueAccount(mint, tokenAccount),
  };
};

/**
 * Burns an attested external amount from `owner`'s confidential balance (`confidential_burn`).
 * The attestation binds (user = owner, contract = the mint's compute-signer PDA) — the token and
 * host programs require contract == compute_signer for transfer/burn amounts. Five FHE steps in
 * one instruction: raised CU limit, a 256 KiB heap frame, and no preflight (SlotHashes entropy is
 * only populated in real execution), exactly the shape the retired live-client sent.
 */
export const confidentialBurn = async (
  context: SolanaProvisioningContext,
  params: {
    readonly owner: TransactionSigner;
    readonly mint: Address;
    readonly amountAttestation: CoprocessorInputAttestationArgs;
  },
): Promise<void> => {
  const vault = await vaultModule();
  const target = await confidentialBurnTarget(params.mint, params.owner.address);
  const { encryptedValueAddress: balanceValue } = await vault.confidentialBalanceValueAccount(
    params.mint,
    target.tokenAccount,
  );
  const [totalSupplyAuthority] = await findTotalSupplyAuthorityPda({ mint: params.mint });
  const totalSupplyValue = await vault.encryptedValueAddress(params.mint, totalSupplyAuthority, TOTAL_SUPPLY_LABEL);
  const instruction = await getConfidentialBurnInstructionAsync({
    owner: params.owner,
    mint: params.mint,
    tokenAccount: target.tokenAccount,
    balanceValue,
    totalSupplyValue,
    burnedAmountValue: target.burnedAmount.encryptedValueAddress,
    pendingBurn: target.pendingBurn,
    zamaEventAuthority: await eventAuthority(ZAMA_HOST_PROGRAM_ADDRESS),
    hostConfig: await hostConfigAddress(),
    eventAuthority: await eventAuthority(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    amountAttestation: params.amountAttestation,
  });
  await context.sendTransaction(params.owner, [requestHeapFrameInstruction(256 * 1024), instruction], {
    skipPreflight: true,
  });
};

/**
 * Redeems the KMS-certified burned amount from the SPL vault (`redeem_burned_amount`): the host
 * verifier CPI checks the certificate against the live KMS context it names plus the burned
 * handle's MMR public-leaf inclusion proof, the token account's PendingBurn closes, and the
 * cleartext amount of underlying releases to the owner's associated token account.
 */
export const redeemBurnedAmount = async (
  context: SolanaProvisioningContext,
  params: {
    readonly owner: TransactionSigner;
    readonly mint: Address;
    readonly underlyingMint: Address;
    readonly claim: PublicDecryptClaim;
    readonly kmsContextId?: bigint;
  },
): Promise<void> => {
  const vault = await vaultModule();
  const { verifyPublicDecryptArgsFromClaim } = await sdkVerifyModule();
  const args = verifyPublicDecryptArgsFromClaim(params.claim);
  const target = await confidentialBurnTarget(params.mint, params.owner.address);
  const [vaultAuthority] = await findVaultAuthorityPda({ mint: params.mint });
  const instruction = await getRedeemBurnedAmountInstructionAsync({
    owner: params.owner,
    mint: params.mint,
    tokenAccount: target.tokenAccount,
    underlyingMint: params.underlyingMint,
    vaultUsdc: await associatedTokenAddress(vaultAuthority, params.underlyingMint),
    destinationUsdc: await associatedTokenAddress(params.owner.address, params.underlyingMint),
    burnedAmountValue: target.burnedAmount.encryptedValueAddress,
    hostConfig: await hostConfigAddress(),
    kmsContext: await kmsContextAddress(params.kmsContextId),
    eventAuthority: await eventAuthority(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
    program: CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    burnedHandle: args.handle,
    cleartextAmount: BigInt(params.claim.abiEncodedCleartext),
    signatures: [...args.signatures],
    extraData: args.extraData,
    proof: { leafIndex: args.leafIndex, siblings: [...args.siblings] },
  });
  await context.sendTransaction(params.owner, [instruction]);
};

/**
 * Seals a token-account handle publicly decryptable (`make_token_account_handle_public`): the
 * token wrapper signs the Host CPI as the encrypted value account authority. Since
 * fhevm-internal#1704 the sealed public-decrypt leaf IS the disclosure request — no witness.
 */
export const sealBurnedAmountHandle = async (
  context: SolanaProvisioningContext,
  params: { readonly owner: TransactionSigner; readonly mint: Address; readonly handle: Uint8Array },
): Promise<void> => {
  const vault = await vaultModule();
  const target = await confidentialBurnTarget(params.mint, params.owner.address);
  await context.sendTransaction(params.owner, [
    await vault.buildMakeTokenAccountHandlePublicInstruction({
      payer: params.owner,
      owner: params.owner,
      mint: params.mint,
      tokenAccount: target.tokenAccount,
      encryptedValue: target.burnedAmount.encryptedValueAddress,
      hostConfig: await hostConfigAddress(),
      kind: vault.DisclosedValueKind.BurnedAmount,
      handle: params.handle,
    }),
  ]);
};

/**
 * Publishes the KMS-certified burned-amount cleartext on-chain (`disclose_secp`): the same host
 * verifier CPI as redeem, then a token-scoped disclosure event. Idempotent by design.
 */
export const discloseBurnedAmount = async (
  context: SolanaProvisioningContext,
  params: {
    readonly owner: TransactionSigner;
    readonly mint: Address;
    readonly claim: PublicDecryptClaim;
    readonly kmsContextId?: bigint;
  },
): Promise<void> => {
  const vault = await vaultModule();
  const target = await confidentialBurnTarget(params.mint, params.owner.address);
  const instruction = await vault.buildDiscloseSecpInstruction(
    {
      mint: params.mint,
      tokenAccount: target.tokenAccount,
      kind: vault.DisclosedValueKind.BurnedAmount,
      encryptedValue: target.burnedAmount.encryptedValueAddress,
      kmsContext: await kmsContextAddress(params.kmsContextId),
      hostConfig: await hostConfigAddress(),
    },
    // Structurally the SDK's SolanaPublicDecryptCertificateClaim; the cast is the same seam the
    // deposit-arc scenario uses at the vault boundary.
    params.claim as never,
  );
  await context.sendTransaction(params.owner, [instruction]);
};
