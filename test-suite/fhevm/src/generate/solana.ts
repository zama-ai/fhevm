import { SOLANA_MERKLE_PROOF_PORT } from "../layout";
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
 * Reads the base58 public key of a Solana keypair file (a 64-byte JSON array, the
 * `[secret(32) || public(32)]` ed25519 layout): a program id or a wallet address, what
 * `solana address -k` prints, computed without invoking the CLI.
 */
export const solanaPubkeyFromKeypairFile = (keypairPath: string): string => {
  const bytes = Uint8Array.from(JSON.parse(fs.readFileSync(keypairPath, "utf8")) as number[]);
  if (bytes.length !== 64) {
    throw new Error(`${keypairPath}: expected a 64-byte solana keypair, got ${bytes.length} bytes`);
  }
  return base58Encode(bytes.subarray(32, 64));
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
 * The Merkle proof endpoint as reached from INSIDE the docker network — same host-process problem
 * as {@link solanaValidatorUrl}: the proof server runs natively next to the validator, the
 * connector runs in a container.
 *
 * One URL, because the demo runs one `solana_merkle_proof_server` (on `SOLANA_MERKLE_PROOF_PORT`).
 * The connector asks every URL at once, so a topology with several coprocessors would list one URL
 * per Merkle proof server.
 */
export const solanaMerkleProofUrl = (): string => `http://host.docker.internal:${SOLANA_MERKLE_PROOF_PORT}`;

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
 * Number.MAX_SAFE_INTEGER, so `JSON.stringify(Number(id))` would corrupt it),
 * `solana_host_program_id` and the Merkle proof URLs the connector requires for a Solana chain,
 * and no `acl_address`. The connector takes the kind from the chain id's type byte and refuses an
 * entry carrying the other kind's settings.
 */
export const serializeKmsHostChains = (entries: readonly KmsHostChainEntry[]): string => {
  const parts = entries.map((e) => {
    if (e.kind === "solana") {
      return (
        `{"url":${JSON.stringify(e.url)},"chain_id":${BigInt(e.chainId).toString()},` +
        `"solana_host_program_id":${JSON.stringify(e.solanaProgramId ?? "")},` +
        `"solana_proof_urls":${JSON.stringify([solanaMerkleProofUrl()])}}`
      );
    }
    return JSON.stringify({ url: e.url, chain_id: Number(e.chainId), acl_address: e.aclAddress ?? "" });
  });
  return `[${parts.join(",")}]`;
};
