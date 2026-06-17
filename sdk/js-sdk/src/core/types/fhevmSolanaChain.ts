import type { Bytes32Hex } from './primitives.js';

/**
 * A Solana host chain definition for the user-decrypt request path.
 *
 * Unlike {@link FhevmChain} (EVM), the Solana host has no on-chain `ACL` / `KMSVerifier` contracts
 * the SDK reads over RPC, and the SDK stops at the aggregated signcrypted shares (de-signcryption is
 * out of scope until the Solana keccak-link path lands in the TKMS WASM). The request therefore
 * needs only:
 *
 * - `id` — the Solana host chain id embedded in each ciphertext handle (`contracts_chain_id`).
 *   It is a `bigint`: the RFC-021 Solana host id (e.g. `9223372036854788153n`) exceeds
 *   `Number.MAX_SAFE_INTEGER`, so a `number` would silently lose precision.
 * - `relayerUrl` — the relayer base URL the ed25519 user-decrypt request is POSTed to.
 * - `acl` — the 32-byte (bytes32) ACL domain key(s) the request is scoped to.
 */
export type FhevmSolanaChain = {
  readonly id: bigint;
  readonly fhevm: {
    readonly relayerUrl: string;
    readonly acl: FhevmSolanaAcl;
  };
};

export type FhevmSolanaAcl = {
  /** The authorized ACL domain keys (each a bytes32 0x-hex), the signed `allowedContracts` scope. */
  readonly domainKeys: readonly Bytes32Hex[];
};
