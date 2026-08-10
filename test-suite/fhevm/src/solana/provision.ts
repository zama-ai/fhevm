// provision — typed on-chain provisioning and balance probing for the live Solana token scenarios.
//
// This module replaces the Rust `poc-live-client` setup seams the two-holder transfer arc used to
// shell out to (`initialize_mint`, `CONSUME_WRAP`, `INITIALIZE_TOKEN_ACCOUNT`, and the
// `TOKEN_BALANCE_STATE` probe) plus the `spl-token`/`solana` CLI calls around them. Every step is
// now a `@solana/kit` transaction built from the demo dapp's typed vault module — the same builders
// `demo/seed.ts` live-verifies on every e2e run — and the shared hand-built SPL instructions in
// `demo/tokenAccounts.ts`. Binding to explicit signers (instead of the ambient Solana CLI identity
// the live-client read from `$HOME`) is what lets the arc target any stack the harness injects.

import { createHash } from "node:crypto";

import {
  appendTransactionMessageInstructions,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  fetchEncodedAccount,
  generateKeyPairSigner,
  getAddressEncoder,
  getProgramDerivedAddress,
  lamports,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
  type Address,
  type Instruction,
  type Rpc,
  type SolanaRpcApi,
  type TransactionSigner,
} from "@solana/kit";

import type { Bytes32Hex } from "@sdk-src/core/types/primitives.js";

import {
  SPL_MINT_ACCOUNT_SPACE,
  SPL_TOKEN_PROGRAM_ADDRESS,
  associatedTokenAddress,
  buildVaultUnderlyingEscrowAtaInstruction,
  createAccountInstruction,
  createIdempotentAtaInstruction,
  initializeMint2Instruction,
  mintToInstruction,
  setComputeUnitLimitInstruction,
} from "../../demo/tokenAccounts";

// The vault module and the SDK sources are loaded lazily, not statically. Their tsconfig-path
// aliases resolve to real paths outside test-suite/fhevm, so their own dependencies (`@noble/*`,
// …) only resolve where the SDK's dependency graph is installed — the solana-e2e workflow does
// that with `npm ci --workspace=@fhevm/sdk…` right before the scenario suite. This module,
// however, is also loaded by the OFFLINE `bun test src` run (through `two-holder-transfer.ts`,
// whose orchestration test injects fake dependencies), where that graph does not exist; the lazy
// seam keeps merely importing this file dependency-free, the same reason
// `deposit-arc.scenario.test.ts` reaches the SDK through a dynamic import.
type VaultModule = typeof import("@demo-dapp/vault/index.js");
let vaultModulePromise: Promise<VaultModule> | undefined;
const vaultModule = (): Promise<VaultModule> => (vaultModulePromise ??= import("@demo-dapp/vault/index.js"));

type SdkProofModule = typeof import("@sdk-src/solana/proof.js");
let sdkProofModulePromise: Promise<SdkProofModule> | undefined;
const sdkProofModule = (): Promise<SdkProofModule> => (sdkProofModulePromise ??= import("@sdk-src/solana/proof.js"));

type SdkHandleModule = typeof import("@sdk-src/core/handle/FhevmHandle.js");
let sdkHandleModulePromise: Promise<SdkHandleModule> | undefined;
const sdkHandleModule = (): Promise<SdkHandleModule> =>
  (sdkHandleModulePromise ??= import("@sdk-src/core/handle/FhevmHandle.js"));

// Every provisioning transaction requests the validator's per-transaction CU ceiling: wrap_usdc
// runs several FHE steps in one instruction and needs ~1.4M CU (the same limit the retired
// live-client requested); the cheaper steps are simply unaffected by the higher ceiling.
const PROVISIONING_COMPUTE_UNIT_LIMIT = 1_400_000;
const LAMPORTS_PER_SOL = 1_000_000_000n;
const EUINT64_FHE_TYPE_ID = 5;
// Anchor account discriminator + layout anchors for the zama-host `HostConfig` singleton
// (`solana/programs/zama-host/src/state/host_config.rs`): `chain_id` is the u64 right after the
// 32-byte `admin` pubkey. The discriminator check keeps a layout drift from being read as garbage.
const HOST_CONFIG_DISCRIMINATOR = createHash("sha256").update("account:HostConfig").digest().subarray(0, 8);
const HOST_CONFIG_CHAIN_ID_OFFSET = 8 + 32;

const addressEncoder = getAddressEncoder();
const encodeAddress = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));

const bytesEqual = (a: Uint8Array, b: Uint8Array): boolean =>
  a.length === b.length && a.every((byte, index) => byte === b[index]);

/** The zama-host singleton `HostConfig` PDA (`[b"host-config"]`) every host-CPI instruction takes. */
export const hostConfigAddress = async (): Promise<Address> => {
  const vault = await vaultModule();
  const [hostConfig] = await getProgramDerivedAddress({
    programAddress: vault.ZAMA_HOST_PROGRAM_ADDRESS,
    seeds: [new TextEncoder().encode("host-config")],
  });
  return hostConfig;
};

export type SolanaProvisioningContext = {
  readonly rpc: Rpc<SolanaRpcApi>;
  /** Signs `instructions` with `payer` plus any account-embedded signers, sends, and confirms. */
  sendTransaction(payer: TransactionSigner, instructions: readonly Instruction[]): Promise<void>;
  /** Airdrops `sol` SOL to `recipient` and waits for the confirmation. Local validators only. */
  airdropSol(recipient: Address, sol: bigint): Promise<void>;
};

/** Binds RPC endpoints into the send/confirm/airdrop closures every provisioning step shares. */
export const createProvisioningContext = (rpcUrl: string, wsUrl: string): SolanaProvisioningContext => {
  const rpc = createSolanaRpc(rpcUrl);
  const rpcSubscriptions = createSolanaRpcSubscriptions(wsUrl);
  const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
  return {
    rpc,
    async sendTransaction(payer, instructions) {
      const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
      const base = setTransactionMessageFeePayerSigner(payer, createTransactionMessage({ version: 0 }));
      const withLifetime = setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, base);
      const message = appendTransactionMessageInstructions(
        [setComputeUnitLimitInstruction(PROVISIONING_COMPUTE_UNIT_LIMIT), ...instructions],
        withLifetime,
      );
      const signedTransaction = await signTransactionMessageWithSigners(message);
      assertIsTransactionWithBlockhashLifetime(signedTransaction);
      await sendAndConfirm(signedTransaction, { commitment: "confirmed" });
    },
    async airdropSol(recipient, sol) {
      const signature = await rpc
        .requestAirdrop(recipient, lamports(sol * LAMPORTS_PER_SOL), { commitment: "confirmed" })
        .send();
      const deadline = Date.now() + 30_000;
      for (;;) {
        const { value } = await rpc.getSignatureStatuses([signature]).send();
        const status = value[0];
        if (status?.err) throw new Error(`airdrop to ${recipient} failed: ${JSON.stringify(status.err)}`);
        const level = status?.confirmationStatus;
        if (level === "confirmed" || level === "finalized") return;
        if (Date.now() >= deadline) throw new Error(`airdrop to ${recipient} did not confirm within 30s`);
        await Bun.sleep(500);
      }
    },
  };
};

export type GeneratedKeypair = {
  readonly signer: TransactionSigner;
  /** The standard 64-byte Solana keypair encoding: 32-byte seed followed by the 32-byte pubkey. */
  readonly bytes: Uint8Array;
};

/**
 * Generates a fresh Ed25519 keypair whose secret is extractable — unlike kit's
 * `generateKeyPairSigner`, whose WebCrypto key can never leave the runtime. The scenarios need the
 * raw bytes twice: written as a keypair file for the SDK transfer worker subprocess, and as the
 * user-decrypt secret key.
 */
export const generateSolanaKeypair = async (): Promise<GeneratedKeypair> => {
  const pair = (await crypto.subtle.generateKey("Ed25519", true, ["sign", "verify"])) as CryptoKeyPair;
  const pkcs8 = new Uint8Array(await crypto.subtle.exportKey("pkcs8", pair.privateKey));
  const publicKey = new Uint8Array(await crypto.subtle.exportKey("raw", pair.publicKey));
  const bytes = new Uint8Array(64);
  // An Ed25519 PKCS#8 blob is a fixed 16-byte DER header followed by the 32-byte seed.
  bytes.set(pkcs8.subarray(pkcs8.length - 32), 0);
  bytes.set(publicKey, 32);
  return { signer: await createKeyPairSignerFromBytes(bytes), bytes };
};

/** Creates a classic SPL mint with `authority` as payer + mint authority; returns its address. */
export const createSplMint = async (
  context: SolanaProvisioningContext,
  params: { readonly authority: TransactionSigner; readonly decimals: number },
): Promise<Address> => {
  const mint = await generateKeyPairSigner();
  const rent = await context.rpc.getMinimumBalanceForRentExemption(SPL_MINT_ACCOUNT_SPACE).send();
  await context.sendTransaction(params.authority, [
    createAccountInstruction({
      payer: params.authority,
      newAccount: mint,
      lamports: rent,
      space: SPL_MINT_ACCOUNT_SPACE,
      owner: SPL_TOKEN_PROGRAM_ADDRESS,
    }),
    initializeMint2Instruction({ mint: mint.address, decimals: params.decimals, mintAuthority: params.authority.address }),
  ]);
  return mint.address;
};

/** Mints `baseUnits` of `mint` to `recipient`'s ATA (created idempotently); returns the ATA. */
export const mintSplTo = async (
  context: SolanaProvisioningContext,
  params: {
    readonly authority: TransactionSigner;
    readonly mint: Address;
    readonly recipient: Address;
    readonly baseUnits: bigint;
  },
): Promise<Address> => {
  const ata = await associatedTokenAddress(params.recipient, params.mint);
  await context.sendTransaction(params.authority, [
    createIdempotentAtaInstruction({ payer: params.authority, ata, owner: params.recipient, mint: params.mint }),
    mintToInstruction({ mint: params.mint, destination: ata, authority: params.authority, baseUnits: params.baseUnits }),
  ]);
  return ata;
};

/**
 * Creates a confidential mint wrapping `underlyingMint` plus its underlying-token escrow — the
 * `vault_usdc` ATA `wrap_usdc` requires to pre-exist. Two transactions, mirroring the seeder's
 * live-verified sequence (steps 3 and 3b in `demo/seed.ts`). Returns the mint and its
 * compute-signer PDA (the contract identity input proofs bind to).
 */
export const createConfidentialMint = async (
  context: SolanaProvisioningContext,
  params: { readonly authority: TransactionSigner; readonly underlyingMint: Address },
): Promise<{ readonly mint: Address; readonly computeSigner: Address }> => {
  const vault = await vaultModule();
  const mint = await generateKeyPairSigner();
  const hostConfig = await hostConfigAddress();
  await context.sendTransaction(params.authority, [
    await vault.buildInitializeMintInstruction({
      authority: params.authority,
      mint,
      underlyingMint: params.underlyingMint,
      hostConfig,
    }),
  ]);
  const escrow = await buildVaultUnderlyingEscrowAtaInstruction({
    payer: params.authority,
    tokenProgram: vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
    confidentialMint: mint.address,
    underlyingMint: params.underlyingMint,
  });
  await context.sendTransaction(params.authority, [escrow.instruction]);
  return { mint: mint.address, computeSigner: await vault.computeSignerAddress(mint.address) };
};

/** Creates `owner`'s canonical zero-balance confidential token account if it does not exist yet. */
export const initializeConfidentialTokenAccount = async (
  context: SolanaProvisioningContext,
  params: { readonly payer: TransactionSigner; readonly owner: Address; readonly mint: Address },
): Promise<void> => {
  const vault = await vaultModule();
  const instruction = await vault.getOrCreateConfidentialTokenAccountInstruction(context.rpc, {
    payer: params.payer,
    owner: params.owner,
    mint: params.mint,
    hostConfig: await hostConfigAddress(),
  });
  if (instruction) await context.sendTransaction(params.payer, [instruction]);
};

/** Escrows a public `amount` of the underlying and rotates it into `owner`'s confidential balance. */
export const wrapUnderlying = async (
  context: SolanaProvisioningContext,
  params: {
    readonly owner: TransactionSigner;
    readonly mint: Address;
    readonly underlyingMint: Address;
    readonly amount: bigint;
  },
): Promise<void> => {
  const vault = await vaultModule();
  await context.sendTransaction(params.owner, [
    await vault.buildWrapUsdcInstruction({
      owner: params.owner,
      mint: params.mint,
      underlyingMint: params.underlyingMint,
      hostConfig: await hostConfigAddress(),
      amount: params.amount,
    }),
  ]);
};

/** A holder's live confidential balance identity, as proven by `readTokenBalanceState`. */
export type BalanceState = {
  version: 1;
  mint: string;
  owner: string;
  tokenAccount: string;
  encryptedValueAccount: string;
  encryptedValueId: string;
  currentHandle: string;
  chainId: string;
};

/**
 * The typed replacement for the live-client's `TOKEN_BALANCE_STATE` probe: resolves the canonical
 * current-balance handle for `(mint, owner)` and proves it is the real thing before any decrypt
 * consumes it. Ported assertions, in order:
 * - the confidential token account PDA exists and is owned by the confidential-token program (its
 *   `(mint, owner)` identity is pinned by the PDA derivation itself, so the body is not re-decoded);
 * - the balance `EncryptedValue` account decodes cleanly (realloc-aware, MMR invariant — enforced
 *   inside `getEncryptedValueState`) and its body matches the canonical derivation: `domain` is the
 *   mint, the authority is the token account, and the (domain, authority, label) triple re-derives
 *   the canonical encrypted value ID — which pins the balance label;
 * - the ACL subjects are exactly [owner, mint compute signer];
 * - the current handle is a version-0 euint64 handle (all-zero rejected: type 0 is not euint64);
 * - the handle's embedded chain id has the Solana high bit and matches the on-chain
 *   `HostConfig.chain_id` (discriminator-checked read at its pinned layout offset).
 * The retired Rust probe additionally re-read all three accounts in one `getMultipleAccounts` to
 * guard against the token account's balance pointer moving between reads; here every address is
 * derived client-side rather than followed from a pointer, so there is no indirection to race —
 * the reads are simply all taken at `confirmed`.
 */
export const readTokenBalanceState = async (
  context: SolanaProvisioningContext,
  params: { readonly mint: Address; readonly owner: Address },
): Promise<BalanceState> => {
  const vault = await vaultModule();
  const { deriveEncryptedValueId } = await sdkProofModule();
  const { bytes32HexToHandle } = await sdkHandleModule();
  const { mint, owner } = params;
  const tokenAccount = await vault.tokenAccountAddress(mint, owner);
  const { aclValueKey, encryptedValueAddress } = await vault.confidentialBalanceValueAccount(mint, tokenAccount);

  const tokenAccountInfo = await fetchEncodedAccount(context.rpc, tokenAccount, { commitment: "confirmed" });
  if (!tokenAccountInfo.exists || tokenAccountInfo.programAddress !== vault.CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS) {
    throw new Error(`confidential token account for (${mint}, ${owner}) is missing or not program-owned`);
  }

  const state = await vault.getEncryptedValueState(context.rpc, encryptedValueAddress, { commitment: "confirmed" });
  const derivedId = deriveEncryptedValueId(
    encodeAddress(state.domain),
    encodeAddress(state.encryptedValueAccountAuthority),
    state.label,
  );
  if (state.domain !== mint || state.encryptedValueAccountAuthority !== tokenAccount || !bytesEqual(derivedId, aclValueKey)) {
    throw new Error("balance encrypted value body does not match its canonical derivation");
  }
  const computeSigner = await vault.computeSignerAddress(mint);
  if (state.subjects.length !== 2 || state.subjects[0] !== owner || state.subjects[1] !== computeSigner) {
    throw new Error("balance encrypted value subjects are not exactly owner + mint compute signer");
  }

  const currentHandle = `0x${Buffer.from(state.currentHandle).toString("hex")}` as Bytes32Hex;
  const handle = bytes32HexToHandle(currentHandle); // throws on a bad handle version or FHE type id
  if (handle.fheTypeId !== EUINT64_FHE_TYPE_ID) {
    throw new Error(`balance handle is not a euint64 handle (FHE type id ${handle.fheTypeId})`);
  }

  const configInfo = await fetchEncodedAccount(context.rpc, await hostConfigAddress(), { commitment: "confirmed" });
  if (
    !configInfo.exists ||
    configInfo.programAddress !== vault.ZAMA_HOST_PROGRAM_ADDRESS ||
    !bytesEqual(configInfo.data.subarray(0, 8), HOST_CONFIG_DISCRIMINATOR)
  ) {
    throw new Error("HostConfig account is missing or has the wrong owner or discriminator");
  }
  const chainId = new DataView(configInfo.data.buffer, configInfo.data.byteOffset).getBigUint64(
    HOST_CONFIG_CHAIN_ID_OFFSET,
    true,
  );
  if ((chainId & (1n << 63n)) === 0n || handle.chainId !== chainId) {
    throw new Error("balance handle chain id does not match the Solana HostConfig");
  }

  return {
    version: 1,
    mint,
    owner,
    tokenAccount,
    encryptedValueAccount: encryptedValueAddress,
    encryptedValueId: `0x${Buffer.from(aclValueKey).toString("hex")}`,
    currentHandle,
    chainId: chainId.toString(),
  };
};
