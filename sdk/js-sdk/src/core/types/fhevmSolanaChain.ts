import type { Bytes32Hex } from './primitives.js';

/**
 * A Solana host chain definition for the user-decrypt request path.
 *
 * Unlike {@link FhevmChain} (EVM), the Solana host has no on-chain `ACL` / `KMSVerifier` contracts
 * the SDK reads over RPC, so everything the flow needs arrives in this definition:
 *
 * - `id` — the Solana host chain id embedded in each ciphertext handle (`contracts_chain_id`).
 *   It is a `bigint`: the RFC-021 Solana host id (e.g. `9223372036854788153n`) exceeds
 *   `Number.MAX_SAFE_INTEGER`, so a `number` would silently lose precision.
 * - `relayerUrl` — the relayer base URL the ed25519 user-decrypt request is POSTed to.
 * - `acl` — the 32-byte (bytes32) ACL domain key(s) the request is scoped to.
 * - `kms` — the trust anchor the in-SDK de-signcryption verifies the KMS responses against.
 *   Required by the user-decrypt action; chains used only for encryption/input proofs may omit it.
 */
export type FhevmSolanaChain = {
  readonly id: bigint;
  readonly fhevm: {
    readonly relayerUrl: string;
    readonly acl: FhevmSolanaAcl;
    readonly kms?: FhevmSolanaKms | undefined;
  };
};

export type FhevmSolanaAcl = {
  /** The signed ACL-domain scope. Empty accepts any ACL domain; subject and handle authorization still apply. */
  readonly domainKeys: readonly Bytes32Hex[];
};

/**
 * What the SDK verifies KMS user-decrypt responses against. Response verification is fail-closed:
 * a share is released only when it carries a valid signature from one of `signers` over the Solana
 * user-decryption link, so an empty or wrong signer set authenticates nothing. The Solana host has
 * no on-chain registry the SDK reads over RPC — the caller supplies the KMS-context signer set it
 * read from the host program (or the gateway) itself.
 */
export type FhevmSolanaKms = {
  /**
   * The registered KMS signer addresses (20-byte EVM hex), in KMS party-id order: the address at
   * index `i` is party `i + 1`. Same convention as the EVM path's `KMSVerifier` signer list.
   */
  readonly signers: readonly string[];
  /** The base58 address of the host program the handles and the decryption link are bound to. */
  readonly verifyingProgramId: string;
  /**
   * The EIP-712 domain each KMS node signed its response under. This is the GATEWAY `Decryption`
   * contract's domain (name `"Decryption"`, version `"1"`, the gateway chain id, the `Decryption`
   * contract address) — not the Solana host chain id. A mismatched domain rejects every share.
   */
  readonly responseDomain: {
    readonly name: string;
    readonly version: string;
    readonly chainId: number;
    readonly verifyingContract: string;
  };
  /** The FHE parameter choice the KMS keys were generated under. Defaults to `'default'`. */
  readonly fheParameter?: 'default' | 'test' | undefined;
};
