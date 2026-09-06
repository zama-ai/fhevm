import type { Bytes32Hex } from './primitives.js';

/**
 * A Solana host chain definition: the deployment's identity and where to reach it.
 *
 * - `id` — the Solana host chain id embedded in each ciphertext handle (`contracts_chain_id`).
 *   It is a `bigint`: the RFC-021 Solana host id (e.g. `9223372036854788153n`) exceeds
 *   `Number.MAX_SAFE_INTEGER`, so a `number` would silently lose precision.
 * - `relayerUrl` — the relayer base URL requests are POSTed to.
 * - `verifyingProgramId` — the 32-byte host program id permits are signed for: the deployment
 *   identity, the Solana analogue of a `verifyingContract`. Required by the permit-path decrypt
 *   actions, unused elsewhere.
 *
 * Everything here describes *where* a deployment is, never *whom to trust* — the trust
 * configuration (KMS signer set, routing, gateway domain) travels separately, as a client
 * parameter. Nor does it carry a default permit scope: a permit is permissive unless the signer
 * names the `(program, scope)` pairs it covers.
 */
export type FhevmSolanaChain = {
  readonly id: bigint;
  readonly fhevm: {
    readonly relayerUrl: string;
    readonly verifyingProgramId?: Bytes32Hex | undefined;
  };
};
