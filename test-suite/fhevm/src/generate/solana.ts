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

/**
 * The Solana validator's RPC URL as reached from INSIDE the docker network.
 *
 * The Solana host runs a NATIVE `solana-test-validator` on the host (the ecosystem norm on macOS;
 * the only published agave images are amd64, so a container would mean qemu emulation). Containers
 * reach the host via `host.docker.internal`; the validator publishes `rpcPort`. Docker Desktop
 * provides that name automatically; on Linux (CI) the connector container maps it to the host
 * gateway via `extra_hosts` (see kms-connector-docker-compose.yml).
 */
export const solanaValidatorUrl = (chain: { readonly rpcPort: number }): string =>
  `http://host.docker.internal:${chain.rpcPort}`;

/**
 * Port the Solana host listener serves its leaf-proof route on, and the bearer key that route
 * requires. Both sides of the same connection read these: `startHostListener` passes them to
 * `solana_host_listener` as `--http-port` / `--proof-api-key`, and `serializeKmsHostChains` puts
 * them in the connector's host-chain entry. Passed explicitly rather than relying on the
 * binary's own default, so the two cannot drift apart silently.
 */
export const SOLANA_LEAF_PROOF_PORT = 8080;
export const SOLANA_LEAF_PROOF_API_KEY = "00000000-0000-0000-0000-000000000000";

/**
 * The leaf-proof endpoint as reached from INSIDE the docker network — same host-process problem
 * as {@link solanaValidatorUrl}: the listener runs natively next to the validator, the connector
 * runs in a container.
 *
 * One entry, because the demo runs one `solana_host_listener`. The connector accepts a list and
 * merges every answer, so a topology with several coprocessors would list one URL per listener.
 */
export const solanaLeafProofUrl = (): string => `http://host.docker.internal:${SOLANA_LEAF_PROOF_PORT}`;

/** A kms-connector `KMS_CONNECTOR_HOST_CHAINS` entry. `aclAddress` is EVM-only. */
export type KmsHostChainEntry = {
  readonly url: string;
  readonly chainId: string;
  readonly kind: "evm" | "solana";
  readonly aclAddress?: string;
  readonly solanaProgramId?: string;
};

/**
 * Serializes `KMS_CONNECTOR_HOST_CHAINS`. EVM entries carry a numeric `chain_id` + `acl_address`.
 * Solana entries emit `chain_id` as a raw integer literal (RFC-021 ids exceed
 * Number.MAX_SAFE_INTEGER, so `JSON.stringify(Number(id))` would corrupt it), `chain_kind`,
 * `solana_host_program_id`, and the leaf-proof endpoints + bearer key the connector requires for
 * a Solana chain — and OMIT `acl_address` (the connector's schema makes it optional and ignores
 * it for Solana, whose ACL is verified via the program id).
 */
export const serializeKmsHostChains = (entries: readonly KmsHostChainEntry[]): string => {
  const parts = entries.map((e) => {
    if (e.kind === "solana") {
      return (
        `{"url":${JSON.stringify(e.url)},"chain_id":${BigInt(e.chainId).toString()},` +
        `"chain_kind":"solana","solana_host_program_id":${JSON.stringify(e.solanaProgramId ?? "")},` +
        `"solana_proof_endpoints":${JSON.stringify([solanaLeafProofUrl()])},` +
        `"solana_proof_api_key":${JSON.stringify(SOLANA_LEAF_PROOF_API_KEY)}}`
      );
    }
    return JSON.stringify({ url: e.url, chain_id: Number(e.chainId), acl_address: e.aclAddress ?? "" });
  });
  return `[${parts.join(",")}]`;
};
