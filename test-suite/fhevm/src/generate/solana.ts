import fs from "node:fs";

import type { Discovery } from "../types";

const BASE58_ALPHABET = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/** Encodes bytes as base58 (Bitcoin/Solana alphabet). */
const base58Encode = (bytes: Uint8Array): string => {
  let n = 0n;
  for (const b of bytes) n = n * 256n + BigInt(b);
  let out = "";
  while (n > 0n) {
    const rem = Number(n % 58n);
    n /= 58n;
    out = BASE58_ALPHABET[rem] + out;
  }
  // Preserve leading-zero bytes as leading '1's.
  for (const b of bytes) {
    if (b === 0) out = "1" + out;
    else break;
  }
  return out;
};

/**
 * Resolves a Solana program's base58 id from its keypair file (a 64-byte JSON array, the
 * `[secret(32) || public(32)]` ed25519 layout) — the deterministic id `solana address -k` prints,
 * computed without invoking the CLI.
 */
export const solanaProgramIdFromKeypairFile = (keypairPath: string): string => {
  const bytes = Uint8Array.from(JSON.parse(fs.readFileSync(keypairPath, "utf8")) as number[]);
  if (bytes.length !== 64) {
    throw new Error(`${keypairPath}: expected a 64-byte solana keypair, got ${bytes.length} bytes`);
  }
  return base58Encode(bytes.subarray(32, 64));
};

// RFC-021 Solana host chain ids occupy the high half of u64 (chain-type high bit set), so they
// exceed both i63 and Number.MAX_SAFE_INTEGER. Two consequences this module centralizes:
//   1. the coprocessor DB stores chain_id as a signed BIGINT, so the u64 is mapped to its
//      two's-complement i64;
//   2. any JSON/number context must carry the id as a raw integer or string literal — never a
//      lossy JS `Number`.

const TWO_POW_64 = 1n << 64n;
const I64_MAX = (1n << 63n) - 1n;

/** Maps a u64 Solana host chain id (decimal string) to the two's-complement i64 the DB stores. */
export const solanaHostChainIdI64 = (chainId: string): string => {
  const u = BigInt(chainId);
  return (u > I64_MAX ? u - TWO_POW_64 : u).toString();
};

/**
 * The Solana host's program id (base58) — its ACL identity, discovered post-deploy like an EVM
 * host's ACL address and stored under the same `ACL_CONTRACT_ADDRESS` discovery key.
 */
export const solanaProgramId = (discovery: Pick<Discovery, "hosts"> | undefined, key: string): string =>
  discovery?.hosts[key]?.ACL_CONTRACT_ADDRESS ?? "";

/** A kms-connector `KMS_CONNECTOR_HOST_CHAINS` entry. */
export type KmsHostChainEntry = {
  readonly url: string;
  readonly chainId: string;
  readonly aclAddress: string;
  readonly kind: "evm" | "solana";
  readonly solanaProgramId?: string;
};

/**
 * Serializes `KMS_CONNECTOR_HOST_CHAINS`. EVM entries use a numeric `chain_id`; Solana entries
 * emit `chain_id` as a raw integer literal (RFC-021 ids exceed Number.MAX_SAFE_INTEGER, so
 * `JSON.stringify(Number(id))` would corrupt it) plus `chain_kind` + `solana_host_program_id`.
 */
export const serializeKmsHostChains = (entries: readonly KmsHostChainEntry[]): string => {
  const parts = entries.map((e) => {
    if (e.kind === "solana") {
      return (
        `{"url":${JSON.stringify(e.url)},"chain_id":${BigInt(e.chainId).toString()},` +
        `"chain_kind":"solana","acl_address":${JSON.stringify(e.aclAddress)},` +
        `"solana_host_program_id":${JSON.stringify(e.solanaProgramId ?? "")}}`
      );
    }
    return JSON.stringify({ url: e.url, chain_id: Number(e.chainId), acl_address: e.aclAddress });
  });
  return `[${parts.join(",")}]`;
};
