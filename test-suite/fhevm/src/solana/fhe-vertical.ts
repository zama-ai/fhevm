// fhe-vertical — shared arc helpers for the fhe_execute scenarios ported from
// `solana/scripts/e2e/full-vertical.sh`: stand up a persistent handle, release it for public
// decryption, and prove its cleartext through the relayer/KMS — all typed, no stdout scraping.
//
// The decrypt side deliberately reuses the strict, unit-tested request builders the fhevm-cli
// already ships (`./public-decrypt`, `./current-user-decrypt`); this module only composes the
// on-chain facts (peaks, leaf count, canonical ids) they consume.

import path from "node:path";

import { getAddressEncoder, type Address, type TransactionSigner } from "@solana/kit";

import {
  hexToBytes as proofHexToBytes,
  verifyHistoricalAccessProof,
  verifyPublicDecryptProof,
  MAX_MMR_SIBLINGS,
  MMR_PROOF_MODE_HISTORICAL,
  type MmrProof,
} from "@sdk-src/solana/proof.js";

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
import { certificateCleartext, runSolanaPublicDecrypt, type PublicDecryptCertificate } from "./public-decrypt";
import type { SolanaProvisioningContext } from "./provision";
import { vaultModule } from "./lazy-modules";

const HISTORICAL_USER_DECRYPT_WORKER = path.join(REPO_ROOT, "test-suite/fhevm/solana-userdecrypt-historical.ts");
const WORKER_DIR = path.join(REPO_ROOT, "test-suite/fhevm");

const hex = (bytes: Uint8Array): string => `0x${Buffer.from(bytes).toString("hex")}`;
const hexCsv = (values: readonly Uint8Array[]): string => values.map(hex).join(",");
const addressEncoder = getAddressEncoder();
const addressBytes = (value: Address): Uint8Array => new Uint8Array(addressEncoder.encode(value));
const addressHex = (value: Address): string => hex(addressBytes(value));

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
 * (create or update — `persistentOutput` reads the previous state). The wallet is domain,
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

/** A proven public decrypt: the interpreted cleartext plus the raw KMS certificate. */
export type PublicDecryptOutcome = {
  readonly cleartext: bigint;
  /** The full certificate — what on-chain consume steps (redeem/disclose) verify. */
  readonly certificate: PublicDecryptCertificate;
};

/**
 * Runs the certified public decrypt of `handle`: live peaks/leaf-count from the on-chain value,
 * the inclusion proof from solana-proof-service (leaf resolved by the service, cross-checked
 * against the on-chain leaf count), then the KMS certificate through the SDK's public-decrypt
 * action. Returns the cleartext together with the certificate; asserting the value is the
 * scenario's job.
 */
export const certifiedPublicDecrypt = async (
  context: SolanaProvisioningContext,
  config: FheVerticalConfig,
  params: {
    readonly target: Pick<PersistentValueTarget, "encryptedValue" | "encryptedValueId">;
    readonly handle: Uint8Array;
    readonly expectedLeafCount?: bigint;
    readonly expectedLeafIndex?: bigint;
  },
): Promise<PublicDecryptOutcome> => {
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
  // Client-side verification against the LIVE peaks, the twin of the historical check below and of
  // what the retired Rust client did before the gateway ever saw the request. Matching leaf counts
  // only prove the service read the same account; this proves the leaf is actually in the tree.
  if (
    !verifyPublicDecryptProof(
      addressBytes(params.target.encryptedValue),
      state.peaks,
      state.leafCount,
      params.handle,
      proof.proof,
    )
  ) {
    throw new Error(
      `public-decrypt proof for handle ${hex(params.handle)} does not verify against the on-chain peaks ` +
        `(leaf index ${proof.proof.leafIndex}, leaf count ${state.leafCount})`,
    );
  }

  const certificate = await runSolanaPublicDecrypt({
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
  return { cleartext: certificateCleartext(certificate), certificate };
};

/**
 * Standard release-and-decrypt tail: allow_subjects + make_handle_public on the value, wait for
 * the SNS ciphertext commit, then the certified public decrypt. The shape every operator row and
 * both compute phases share.
 */
export const releaseAndDecrypt = async (
  context: SolanaProvisioningContext,
  config: FheVerticalConfig,
  stack: { waitForSnsCommit(handle: string): Promise<void> },
  params: {
    readonly payer: TransactionSigner;
    readonly result: PersistentHandle;
  },
): Promise<PublicDecryptOutcome> => {
  await allowForPublicDecryption(context, {
    payer: params.payer,
    encryptedValue: params.result.target.encryptedValue,
  });
  await stack.waitForSnsCommit(hex(params.result.handle));
  return certifiedPublicDecrypt(context, config, {
    target: params.result.target,
    handle: params.result.handle,
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
 * The proof-service wire shape for the semantic access-proof endpoint. Parsed in exactly one place
 * ({@link parseHistoricalAccessProof}); everything downstream consumes the normalized result.
 */
type AccessProofWire = {
  readonly mmr_proof: { readonly leaf_index: number; readonly siblings: readonly string[] } | null;
  readonly leaf_count: number;
  readonly verified: boolean;
  readonly status?: string;
};

/** `0x01 || Borsh(MmrProof)` — the historical-mode twin of the vault client's `0x02` encoder. */
const encodeHistoricalTransportBlob = (proof: MmrProof): Uint8Array => {
  const out = new Uint8Array(1 + 8 + 4 + proof.siblings.length * 32);
  const view = new DataView(out.buffer);
  out[0] = MMR_PROOF_MODE_HISTORICAL;
  view.setBigUint64(1, proof.leafIndex, true);
  view.setUint32(9, proof.siblings.length, true);
  proof.siblings.forEach((sibling, i) => out.set(sibling, 13 + i * 32));
  return out;
};

/**
 * Normalizes a success response, enforcing what the vault's public-proof client enforces:
 * `verified: true`, 32-byte siblings, and the sibling cap the on-chain verifier rejects past.
 */
export const parseHistoricalAccessProof = (body: unknown): HistoricalAccessProof => {
  if (typeof body !== "object" || body === null || !("mmr_proof" in body)) {
    throw new Error("proof-service response is not an MMR-proof envelope");
  }
  const wire = body as AccessProofWire;
  if (!wire.verified || wire.mmr_proof === null) {
    throw new Error(`proof-service returned an unverified access proof (status "${wire.status ?? "?"}")`);
  }
  const siblings = wire.mmr_proof.siblings.map((sibling) => {
    const bytes = proofHexToBytes(sibling);
    if (bytes.length !== 32) throw new Error(`access-proof sibling must be 32 bytes, got ${bytes.length}`);
    return bytes;
  });
  if (siblings.length > MAX_MMR_SIBLINGS) {
    throw new Error(`access-proof carries ${siblings.length} siblings, exceeding the cap of ${MAX_MMR_SIBLINGS}`);
  }
  // leafIndex is the service's resolved OUTPUT — the client never supplies one.
  const proof: MmrProof = { leafIndex: BigInt(wire.mmr_proof.leaf_index), siblings };
  return {
    leafIndex: proof.leafIndex,
    siblings,
    leafCount: BigInt(wire.leaf_count),
    mmrProofBytes: encodeHistoricalTransportBlob(proof),
  };
};

/**
 * Only `lagging` means "retry later" — the store catching up to the chain. Every other status
 * (`leaf_not_found`, a corrupt cache, an integrity failure, any 4xx) is terminal: retrying one of
 * those would bury a real integrity failure under a retry loop and report it as a timeout.
 */
export const isLaggingAccessProof = (body: unknown): boolean =>
  typeof body === "object" && body !== null && (body as { status?: string }).status === "lagging";

/** Per-request cap: a service that accepts the connection but never answers must not hang the poll. */
const ACCESS_PROOF_REQUEST_TIMEOUT_MS = 10_000;
/**
 * Bounded `lagging` retry budget. The retired Rust live-client tolerated ~28s (15 attempts, 2s
 * apart). The poll here is twice as fast so a recovering service is picked up sooner, and the
 * count is raised to match so the tolerance for a genuinely lagging service is unchanged.
 */
const ACCESS_PROOF_MAX_RETRIES = 28;
const ACCESS_PROOF_RETRY_DELAY_MS = 1_000;

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
  for (let attempt = 0; ; attempt++) {
    const response = await fetch(url, {
      method: "GET",
      headers: { accept: "application/json" },
      signal: AbortSignal.timeout(ACCESS_PROOF_REQUEST_TIMEOUT_MS),
    });
    const body: unknown = await response.json().catch(() => null);
    if (response.ok) return parseHistoricalAccessProof(body);
    if (response.status === 503 && isLaggingAccessProof(body) && attempt < ACCESS_PROOF_MAX_RETRIES) {
      await Bun.sleep(ACCESS_PROOF_RETRY_DELAY_MS);
      continue;
    }
    const status = (body as { status?: string } | null)?.status;
    throw new Error(`access-proof request failed (HTTP ${response.status}, status "${status ?? "?"}")`);
  }
};

/**
 * Runs the pure-SDK HISTORICAL user-decrypt of an old (updated-away) handle through the existing
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
  if (params.proof.leafCount !== state.leafCount) {
    throw new Error(
      `access-proof leaf count ${params.proof.leafCount} does not match the on-chain leaf count ${state.leafCount}`,
    );
  }
  // Client-side cryptographic verification against the LIVE peaks, restoring what the retired Rust
  // live-client did: recompute the historical-access leaf commitment and fold the sibling path.
  // The gateway verifies this again, so coverage does not hang on it — DIAGNOSIS does. Without it a
  // service that resolves a wrong-but-self-consistent leaf index fails much later as a generic KMS
  // decrypt rejection, with nothing pointing at the proof.
  const proofVerifies = verifyHistoricalAccessProof(
    addressBytes(params.target.encryptedValue),
    state.peaks,
    state.leafCount,
    params.oldHandle,
    addressBytes(params.subject),
    { leafIndex: params.proof.leafIndex, siblings: params.proof.siblings },
  );
  if (!proofVerifies) {
    throw new Error(
      `access proof for handle ${hex(params.oldHandle)} does not verify against the on-chain peaks ` +
        `(leaf index ${params.proof.leafIndex}, leaf count ${state.leafCount})`,
    );
  }
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
