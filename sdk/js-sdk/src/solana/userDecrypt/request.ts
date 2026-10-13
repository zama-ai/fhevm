// The user-decryption request: one signed permit, and the handles asked for under it.
//
// The permit is the reusable object and the request is not. Every request cites the one signature the
// wallet produced, and this builder never signs, never asks a wallet for anything, and never mutates
// the permit — which is what makes a retry cost no further signature.
//
// The JSON this produces is `SolanaUserDecryptionRequest` of `kms-connector-api` as it serializes,
// which the decryption relayer forwards unchanged: byte strings are `0x`-hex and the two timestamps
// are JSON numbers. The permit's typed rules cap both far below 2^53, so a number carries them
// exactly.
//
// An entry names three things: the handle, the key it was allowed to (the requester on a direct
// entry, the delegator on a delegated one), and the encrypted value account the handle lives in. It
// carries no proof and no leaf count: the Connector reads the account itself and asks the
// coprocessors for the allow leaf (RFC 035), so the client has nothing to fetch and nothing to
// verify before submitting.
//
// What this builder checks is what a layer before the Connector can check without reading host
// state: the KMS bit budget, the handles' host chain, the width of each identity field, and that
// there is at least one handle. The handle count cap is `zama-solana-request`'s, which relayer-http
// and the Connector both run. Everything else — the allow itself, delegation, the validity window,
// the revocation watermark — is authorization, and belongs to the Connector against a state
// snapshot no client can hold.

import type { SolanaSignedPermit } from '../permit/index.js';
import { assertKmsDecryptionBitLimit } from '../../core/kms/utils.js';
import { toFhevmHandle } from '../../core/handle/FhevmHandle.js';
import { encodeSolanaKmsRouting } from '../permit/index.js';
import { bytesToHex } from '../../core/base/bytes.js';

/** The attestation type that selects the Solana permit scheme at the Connector. */
export const SOLANA_SRFC38_ATTESTATION_TYPE = 'solana-srfc38-user-decrypt-v1';

/** One handle to decrypt: the handle, the owner address whose allow authorizes it, and its store. */
export interface SolanaUserDecryptHandleEntry {
  /** The 32-byte ciphertext handle. */
  readonly handle: Uint8Array;
  /**
   * The 32-byte key whose allow on the handle authorizes this entry: the requester itself on a
   * direct entry, the delegator on a delegated one.
   */
  readonly ownerAddress: Uint8Array;
  /** The 32-byte address of the `EncryptedStore` account the handle lives in. */
  readonly encryptedStore: Uint8Array;
}

/** One handle entry, as it travels. */
export interface SolanaUserDecryptHandleJson {
  readonly handle: string;
  readonly ownerAddress: string;
  readonly encryptedStore: string;
}

/**
 * The payload: the signed permit fields, plus the unsigned handle entries. The permit's chain id does
 * not travel: it is the one every handle embeds. `publicKey` is the permit's transport key.
 */
export interface SolanaUserDecryptPayloadJson {
  readonly userAddress: string;
  readonly publicKey: string;
  readonly allowedScopes: readonly string[];
  readonly requestValidity: { readonly startTimestamp: number; readonly durationSeconds: number };
  readonly verifyingProgramId: string;
  readonly extraData: string;
  readonly handles: readonly SolanaUserDecryptHandleJson[];
}

/** The request body posted to the decryption relayer's `v4/user-decrypt`. */
export interface SolanaUserDecryptRequestJson {
  readonly attestationType: typeof SOLANA_SRFC38_ATTESTATION_TYPE;
  readonly payload: SolanaUserDecryptPayloadJson;
  readonly signature: string;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Why a request could not be assembled.
 *
 * Every member is a rejection this layer can reach without reading host state, and each names the
 * entry it came from: a request of thirty-three handles that is refused without saying which one is
 * malformed sends the caller looking through all of them.
 */
export type SolanaUserDecryptRequestFailure =
  | { readonly reason: 'no-handles' }
  | { readonly reason: 'foreign-host-chain'; readonly index: number; readonly chainId: bigint }
  | {
      readonly reason: 'entry-field-width';
      readonly index: number;
      readonly field: 'ownerAddress' | 'encryptedStore';
    };

/** A request that was refused before it reached the network. */
export class SolanaUserDecryptRequestError extends Error {
  readonly failure: SolanaUserDecryptRequestFailure;

  constructor(failure: SolanaUserDecryptRequestFailure) {
    super(describeRequestFailure(failure));
    this.name = 'SolanaUserDecryptRequestError';
    this.failure = failure;
  }
}

/**
 * Renders an assembly failure as a sentence. Exhaustive by construction — no fallback arm.
 *
 * @param failure - The structured reason.
 */
function describeRequestFailure(failure: SolanaUserDecryptRequestFailure): string {
  switch (failure.reason) {
    case 'no-handles':
      return 'a user-decryption request must name at least one handle; a permit on its own authorizes nothing';
    case 'foreign-host-chain':
      return `handle ${failure.index} belongs to host chain ${failure.chainId}, which the permit was not signed for`;
    case 'entry-field-width':
      return `entry ${failure.index}: ${failure.field} is not 32 bytes`;
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The admission a request must pass: every rule checkable from the entries and the permit alone —
 * the KMS bit budget, each handle's host chain, the width of each identity field.
 * A request that can never be answered is refused before it reaches the network.
 *
 * @param admission.chainId - The one host chain the permit was signed for.
 * @param admission.entries - The handles, in the order they will be requested.
 * @throws SolanaUserDecryptRequestError - On the first rule the request breaks.
 */
export function admitSolanaUserDecryptRequest(admission: {
  readonly chainId: bigint;
  readonly entries: readonly SolanaUserDecryptHandleEntry[];
}): void {
  const { chainId, entries } = admission;

  if (entries.length === 0) {
    throw new SolanaUserDecryptRequestError({ reason: 'no-handles' });
  }

  const handles = entries.map((entry) => toFhevmHandle(entry.handle));
  assertKmsDecryptionBitLimit(handles);

  // The chain id each handle embeds must be the one chain the permit was signed for.
  for (const [index, handle] of handles.entries()) {
    if (handle.chainId !== chainId) {
      throw new SolanaUserDecryptRequestError({ reason: 'foreign-host-chain', index, chainId: handle.chainId });
    }
  }

  for (const [index, entry] of entries.entries()) {
    if (entry.ownerAddress.length !== 32) {
      throw new SolanaUserDecryptRequestError({ reason: 'entry-field-width', index, field: 'ownerAddress' });
    }
    if (entry.encryptedStore.length !== 32) {
      throw new SolanaUserDecryptRequestError({ reason: 'entry-field-width', index, field: 'encryptedStore' });
    }
  }
}

/**
 * Assembles the request body for a signed permit and the entries.
 *
 * Duplicates and their order are preserved exactly as given: each occurrence is authorized on its
 * own, counts toward the budget on its own, and is bound by the response linker at its position.
 * Trimming an oversize request to fit would be answered as a request the caller never made, so an
 * oversize one is refused instead.
 *
 * @param request.signedPermit - The permit and its one signature.
 * @param request.entries - The entries, in the order they will be requested.
 * @throws SolanaUserDecryptRequestError - On the first entry or budget rule the request breaks.
 */
export function buildSolanaUserDecryptRequest(request: {
  readonly signedPermit: SolanaSignedPermit;
  readonly entries: readonly SolanaUserDecryptHandleEntry[];
}): SolanaUserDecryptRequestJson {
  const { signedPermit, entries } = request;
  const fields = signedPermit.fields;

  admitSolanaUserDecryptRequest({ chainId: fields.chainId, entries });

  return {
    attestationType: SOLANA_SRFC38_ATTESTATION_TYPE,
    payload: {
      userAddress: bytesToHex(fields.userAddress),
      publicKey: bytesToHex(fields.transportKey),
      allowedScopes: fields.allowedScopes.map((scope) => bytesToHex(scope)),
      requestValidity: {
        startTimestamp: Number(fields.startTimestamp),
        durationSeconds: Number(fields.durationSeconds),
      },
      verifyingProgramId: bytesToHex(fields.verifyingProgramId),
      extraData: bytesToHex(encodeSolanaKmsRouting(fields.kmsRouting)),
      handles: entries.map((entry) => ({
        handle: bytesToHex(entry.handle),
        ownerAddress: bytesToHex(entry.ownerAddress),
        encryptedStore: bytesToHex(entry.encryptedStore),
      })),
    },
    signature: bytesToHex(signedPermit.signature),
  };
}
