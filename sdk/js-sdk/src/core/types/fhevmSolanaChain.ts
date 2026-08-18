import type { Bytes32Hex } from './primitives.js';

/**
 * A Solana host chain definition: the deployment's identity and where to reach it.
 *
 * - `id` — the Solana host chain id embedded in each ciphertext handle (`contracts_chain_id`).
 *   It is a `bigint`: the RFC-021 Solana host id (e.g. `9223372036854788153n`) exceeds
 *   `Number.MAX_SAFE_INTEGER`, so a `number` would silently lose precision.
 * - `relayerUrl` — the relayer base URL requests are POSTed to.
 * - `acl` — the 32-byte (bytes32) ACL domain key(s) requests are scoped to by default.
 * - `rpcUrl` — the Solana RPC the permit path reads host state through (the `EncryptedValue`
 *   account behind each handle). Required by the permit-path decrypt actions, unused elsewhere.
 * - `proofServiceUrl` — the standalone proof service historical-access proofs come from.
 *   Required by the permit-path decrypt actions, unused elsewhere.
 * - `verifyingProgramId` — the 32-byte host program id permits are signed for: the deployment
 *   identity, the Solana analogue of a `verifyingContract`. Required by the permit-path decrypt
 *   actions, unused elsewhere.
 *
 * Everything here describes *where* a deployment is, never *whom to trust* — the trust
 * configuration (KMS signer set, routing, gateway domain) travels separately, as a client
 * parameter.
 */
export type FhevmSolanaChain = {
  readonly id: bigint;
  readonly fhevm: {
    readonly relayerUrl: string;
    readonly acl: FhevmSolanaAcl;
    readonly rpcUrl?: string | undefined;
    readonly proofServiceUrl?: string | undefined;
    readonly verifyingProgramId?: Bytes32Hex | undefined;
  };
};

export type FhevmSolanaAcl = {
  /** The signed ACL-domain scope. Empty accepts any ACL domain; subject and handle authorization still apply. */
  readonly domainKeys: readonly Bytes32Hex[];
};
