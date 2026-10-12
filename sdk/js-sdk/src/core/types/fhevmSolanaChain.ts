import type { Bytes32Hex } from './primitives.js';

/**
 * A Solana host chain definition: the deployment's identity and where to reach it, the Solana
 * counterpart of `FhevmChain`.
 *
 * - `id` — the Solana host chain id embedded in each ciphertext handle (`contracts_chain_id`).
 *   It is a `bigint`: the RFC-021 Solana host id (e.g. `72057594037940281n`) exceeds
 *   `Number.MAX_SAFE_INTEGER`, so a `number` would silently lose precision.
 * - `relayerUrl` — the relayer that serves input proofs and the FHE public key.
 * - `decryptionRelayerUrl` — the relayer that answers user and public decryptions, synchronously
 *   (`relayer-http`, `/v4/user-decrypt` and `/v4/public-decrypt`).
 * - `programs.host.address` — the zama-host program id of the deployment as a canonical 32-byte
 *   identity, the analogue of the EVM chain's `contracts.acl.address`. Every action derives PDAs,
 *   checks account ownership and signs permits (`verifyingProgramId`) against it; nothing falls
 *   back to the program id of the bundled IDL.
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
    readonly decryptionRelayerUrl: string;
    readonly programs: {
      readonly host: {
        readonly address: Bytes32Hex;
      };
    };
  };
};
