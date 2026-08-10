// fhe-vertical — shared arc helpers for the fhe_execute scenarios ported from
// `solana/scripts/e2e/full-vertical.sh`: stand up a persistent handle, release it for public
// decryption, and prove its cleartext through the relayer/KMS — all typed, no stdout scraping.
//
// The decrypt side deliberately reuses the strict, unit-tested request builders the fhevm-cli
// already ships (`./public-decrypt`, `./current-user-decrypt`); this module only composes the
// on-chain facts (peaks, leaf count, canonical ids) they consume.

import path from "node:path";

import { getAddressEncoder, type Address, type TransactionSigner } from "@solana/kit";

import { REPO_ROOT } from "../layout";
import { run } from "../utils/process";
import {
  ExecutionDictionary,
  FHE_TYPE,
  allowForPublicDecryption,
  persistentOutput,
  persistentValueTarget,
  scalarBytes,
  sendFheExecute,
  type PersistentValueTarget,
} from "./fhe-execute";
import { runSolanaPublicDecrypt } from "./public-decrypt";
import type { SolanaProvisioningContext } from "./provision";

type VaultModule = typeof import("@demo-dapp/vault/index.js");
let vaultModulePromise: Promise<VaultModule> | undefined;
const vaultModule = (): Promise<VaultModule> => (vaultModulePromise ??= import("@demo-dapp/vault/index.js"));

const HISTORICAL_USER_DECRYPT_WORKER = path.join(REPO_ROOT, "test-suite/fhevm/solana-userdecrypt-historical.ts");
const WORKER_DIR = path.join(REPO_ROOT, "test-suite/fhevm");

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;
const hexCsv = (values: readonly Uint8Array[]): string => values.map(hex).join(",");
const addressEncoder = getAddressEncoder();
const addressHex = (value: Address): string => hex(new Uint8Array(addressEncoder.encode(value)));

/** The environment facts every vertical decrypt binds to. */
export type FheVerticalConfig = {
  readonly relayerUrl: string;
  readonly proofServiceUrl: string;
  /** The Solana host chain id (`HostConfig.chain_id`, high bit set). */
  readonly chainId: bigint;
  /** KMS public-decrypt context id, 0x-hex bytes32. */
  readonly publicDecryptContextId: string;
  /** Gateway user-decrypt context id, unsigned decimal string. */
  readonly userDecryptContextId: string;
};

/** A 32-byte encrypted-value label from a short name, underscore-padded (the house label idiom). */
export const paddedLabel = (name: string): Uint8Array => {
  const bytes = new TextEncoder().encode(name.padEnd(32, "_"));
  if (bytes.length !== 32) throw new Error(`label name too long: ${name}`);
  return bytes;
};

export type PersistentHandle = {
  readonly target: PersistentValueTarget;
  /** The value's current handle right after the execution, as 32 raw bytes. */
  readonly handle: Uint8Array;
};

/** Reads the current handle bytes of a persistent value at `confirmed`. */
export const currentHandle = async (
  context: SolanaProvisioningContext,
  encryptedValue: Address,
): Promise<Uint8Array> => {
  const vault = await vaultModule();
  const state = await vault.getEncryptedValueState(context.rpc, encryptedValue, { commitment: "confirmed" });
  return state.currentHandle;
};

/**
 * One single-step `fhe_execute` TrivialEncrypt bound to a persistent scenario-owned value
 * (create or rotate — `persistentOutput` reads the previous state). The wallet is domain,
 * account authority, and sole ACL subject, mirroring the live-client's wallet-driven phases.
 */
export const trivialEncryptPersistent = async (
  context: SolanaProvisioningContext,
  params: {
    readonly payer: TransactionSigner;
    readonly value: bigint;
    readonly label: Uint8Array;
    readonly fheType?: number;
  },
): Promise<PersistentHandle> => {
  const target = await persistentValueTarget(params.payer.address, params.payer.address, params.label);
  const dictionary = new ExecutionDictionary();
  const output = await persistentOutput(context, dictionary, {
    target,
    encryptedValueIndex: 0,
    subjects: [params.payer.address],
  });
  await sendFheExecute(context, {
    payer: params.payer,
    dictionary,
    steps: [
      {
        __kind: "TrivialEncrypt",
        plaintext: scalarBytes(params.value),
        fheType: params.fheType ?? FHE_TYPE.euint64,
        output,
      },
    ],
    remainingAccounts: [{ address: target.encryptedValue, writable: true }],
  });
  return { target, handle: await currentHandle(context, target.encryptedValue) };
};

/**
 * Proves `handle` decrypts publicly to the expected cleartext: live peaks/leaf-count from the
 * on-chain value, the inclusion proof from solana-proof-service (leaf resolved by the service,
 * cross-checked against the on-chain leaf count), then the KMS certificate through the SDK's
 * public-decrypt action. Returns the decrypted cleartext.
 */
export const publicDecryptExpect = async (
  context: SolanaProvisioningContext,
  config: FheVerticalConfig,
  params: {
    readonly target: Pick<PersistentValueTarget, "encryptedValue" | "encryptedValueId">;
    readonly handle: Uint8Array;
    /** Exact expected cleartext, or an exclusive upper bound for random outputs. */
    readonly expect: bigint | { readonly lessThan: bigint };
    readonly expectedLeafCount?: bigint;
    readonly expectedLeafIndex?: bigint;
  },
): Promise<bigint> => {
  const vault = await vaultModule();
  const state = await vault.getEncryptedValueState(context.rpc, params.target.encryptedValue, {
    commitment: "confirmed",
  });
  const proof = await vault.fetchSolanaPublicDecryptProof(
    { proofServiceUrl: config.proofServiceUrl },
    params.target.encryptedValue,
    params.handle,
  );
  if (proof.leafCount !== state.leafCount) {
    throw new Error(
      `proof-service leaf count ${proof.leafCount} does not match the on-chain leaf count ${state.leafCount}`,
    );
  }
  if (params.expectedLeafCount !== undefined && proof.leafCount !== params.expectedLeafCount) {
    throw new Error(`public proof leaf count ${proof.leafCount} != expected ${params.expectedLeafCount}`);
  }
  if (params.expectedLeafIndex !== undefined && proof.proof.leafIndex !== params.expectedLeafIndex) {
    throw new Error(`public proof leaf index ${proof.proof.leafIndex} != expected ${params.expectedLeafIndex}`);
  }

  const claim = await runSolanaPublicDecrypt({
    PD_RELAYER_URL: config.relayerUrl,
    PD_CONTRACTS_CHAIN_ID: config.chainId.toString(),
    PD_HANDLE: hex(params.handle),
    PD_CONTEXT_ID: config.publicDecryptContextId,
    PD_ACL_VALUE_KEY: hex(params.target.encryptedValueId),
    PD_MMR_PROOF_SLOT: state.leafCount.toString(),
    PD_MMR_ENCRYPTED_VALUE_ACCOUNT: addressHex(params.target.encryptedValue),
    PD_MMR_PEAKS: hexCsv(state.peaks),
    PD_MMR_LEAF_COUNT: state.leafCount.toString(),
    PD_MMR_PROOF_BYTES: hex(proof.mmrProofBytes),
  });
  const cleartext = BigInt(claim.abiEncodedCleartext);
  if (typeof params.expect === "bigint") {
    if (cleartext !== params.expect) throw new Error(`public-decrypt cleartext ${cleartext} != ${params.expect}`);
  } else if (cleartext >= params.expect.lessThan) {
    throw new Error(`public-decrypt cleartext ${cleartext} not < ${params.expect.lessThan}`);
  }
  return cleartext;
};

/**
 * Standard "release and prove" tail: allow_subjects + make_handle_public on the value, wait for
 * the SNS ciphertext commit, then public-decrypt and compare. The shape every operator row and
 * both compute phases share.
 */
export const releaseAndExpect = async (
  context: SolanaProvisioningContext,
  config: FheVerticalConfig,
  stack: { waitForSnsCommit(handle: string): Promise<void> },
  params: {
    readonly payer: TransactionSigner;
    readonly result: PersistentHandle;
    readonly expect: bigint | { readonly lessThan: bigint };
  },
): Promise<bigint> => {
  await allowForPublicDecryption(context, {
    payer: params.payer,
    encryptedValue: params.result.target.encryptedValue,
  });
  await stack.waitForSnsCommit(hex(params.result.handle));
  return publicDecryptExpect(context, config, {
    target: params.result.target,
    handle: params.result.handle,
    expect: params.expect,
  });
};

/** The normalized historical-access proof, mirroring the vault's public-proof client. */
export type HistoricalAccessProof = {
  readonly leafIndex: bigint;
  readonly siblings: readonly Uint8Array[];
  readonly leafCount: bigint;
  /** `0x01 || Borsh(MmrProof)` transport blob. */
  readonly mmrProofBytes: Uint8Array;
};

/**
 * Fetches the historical-access inclusion proof for `(encryptedValue, oldHandle, subject)` from
 * solana-proof-service. The service resolves the leaf semantically — the client supplies no leaf
 * index — and only `verified: true` responses are accepted; `503 lagging` is retried bounded,
 * exactly like the vault module's public-proof client.
 */
export const fetchHistoricalAccessProof = async (
  config: FheVerticalConfig,
  params: { readonly encryptedValue: Address; readonly oldHandle: Uint8Array; readonly subject: Address },
): Promise<HistoricalAccessProof> => {
  const base = config.proofServiceUrl.replace(/\/$/, "");
  const url =
    `${base}/internal/solana/access-proof?encrypted_value=${params.encryptedValue}` +
    `&handle=${Buffer.from(params.oldHandle).toString("hex")}&subject=${params.subject}`;
  const maxRetries = 10;
  for (let attempt = 0; ; attempt++) {
    const response = await fetch(url, { headers: { accept: "application/json" } });
    const body = (await response.json().catch(() => null)) as {
      mmr_proof: { leaf_index: number; siblings: string[] } | null;
      leaf_count: number;
      verified: boolean;
      status?: string;
    } | null;
    if (response.ok && body?.verified && body.mmr_proof) {
      const siblings = body.mmr_proof.siblings.map((sibling) => {
        const bytes = Uint8Array.from(Buffer.from(sibling.replace(/^0x/, ""), "hex"));
        if (bytes.length !== 32) throw new Error("access-proof sibling must be 32 bytes");
        return bytes;
      });
      const leafIndex = BigInt(body.mmr_proof.leaf_index);
      // 0x01 (historical mode) || borsh(MmrProof{leaf_index: u64-le, siblings: Vec<[u8;32]>}).
      const blob = new Uint8Array(1 + 8 + 4 + siblings.length * 32);
      const view = new DataView(blob.buffer);
      blob[0] = 0x01;
      view.setBigUint64(1, leafIndex, true);
      view.setUint32(9, siblings.length, true);
      siblings.forEach((sibling, i) => blob.set(sibling, 13 + i * 32));
      return { leafIndex, siblings, leafCount: BigInt(body.leaf_count), mmrProofBytes: blob };
    }
    if (response.status === 503 && body?.status === "lagging" && attempt < maxRetries) {
      await Bun.sleep(1000);
      continue;
    }
    throw new Error(`access-proof request failed (HTTP ${response.status}, status "${body?.status ?? "?"}")`);
  }
};

/**
 * Runs the pure-SDK HISTORICAL user-decrypt of an old (rotated-away) handle through the existing
 * worker (`solana-userdecrypt-historical.ts`): ML-KEM keygen, the v3 ed25519 request with the
 * historical MMR proof, and in-SDK de-signcryption. The worker asserts the cleartext equals
 * `expected` and exits non-zero otherwise.
 */
export const historicalUserDecryptExpect = async (
  context: SolanaProvisioningContext,
  config: FheVerticalConfig,
  params: {
    readonly target: PersistentValueTarget;
    readonly oldHandle: Uint8Array;
    readonly subject: Address;
    /** The wallet's 32-byte seed, 0x-hex — the ed25519 key the v3 request signs with. */
    readonly secretKey: string;
    /** Allowed ACL domain key (bytes32 hex) — the value's domain pubkey. */
    readonly allowedDomainKey: string;
    readonly expected: bigint;
    readonly proof: HistoricalAccessProof;
  },
): Promise<void> => {
  const vault = await vaultModule();
  const state = await vault.getEncryptedValueState(context.rpc, params.target.encryptedValue, {
    commitment: "confirmed",
  });
  const userDecryptContextIdHex = `0x${BigInt(config.userDecryptContextId).toString(16).padStart(64, "0")}`;
  await run(["bun", "run", HISTORICAL_USER_DECRYPT_WORKER], {
    cwd: WORKER_DIR,
    env: {
      UD_RELAYER_URL: config.relayerUrl,
      UD_CONTRACTS_CHAIN_ID: config.chainId.toString(),
      UD_HANDLE: hex(params.oldHandle),
      UD_SECRET_KEY: params.secretKey,
      UD_CONTEXT_ID: userDecryptContextIdHex,
      UD_ALLOWED_DOMAIN_KEYS: params.allowedDomainKey,
      UD_ACL_VALUE_KEY: hex(params.target.encryptedValueId),
      UD_EXPECTED: params.expected.toString(),
      UD_MMR_ENCRYPTED_VALUE_ACCOUNT: addressHex(params.target.encryptedValue),
      UD_MMR_PEAKS: hexCsv(state.peaks),
      UD_MMR_LEAF_COUNT: state.leafCount.toString(),
      UD_MMR_PROOF_SLOT: state.leafCount.toString(),
      UD_MMR_LEAF_INDEX: params.proof.leafIndex.toString(),
      UD_MMR_SIBLINGS: hexCsv(params.proof.siblings),
      UD_MMR_PROOF_BYTES: hex(params.proof.mmrProofBytes),
      UD_MMR_SUBJECT: addressHex(params.subject),
    },
  });
};
