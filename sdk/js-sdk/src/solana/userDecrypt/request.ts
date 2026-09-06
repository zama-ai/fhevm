// The v3 user-decryption request: one signed permit, and the handles asked for under it.
//
// The permit is the reusable object and the request is not. Every request cites the one signature the
// wallet produced, and this builder never signs, never asks a wallet for anything, and never mutates
// the permit — which is what makes a retry cost no further signature.
//
// The JSON this produces is the relayer's v3 wire shape, pinned by
// `solana/test-fixtures/user-decrypt/relayer_envelope_v1.json` and read from both sides of the seam.
// Two conventions in it are load-bearing: every byte string is `0x`-hex, and every 64-bit number is a
// decimal string — a JSON number would arrive at the relayer as a double and lose the chain id.
//
// An entry names three things: the handle, the key it was allowed to (the requester on a direct
// entry, the delegator on a delegated one), and the encrypted value account the handle lives in. It
// carries no proof and no leaf count: the Connector reads the account itself and asks the
// coprocessors for the allow leaf (RFC 035), so the client has nothing to fetch and nothing to
// verify before submitting.
//
// What this builder checks is what a layer before the Gateway can check without reading host state:
// the handle count cap and the bit budget, the handles' host chain, the width of each identity
// field, and that there is at least one handle. Everything else — the allow itself, delegation, the
// validity window, the revocation watermark — is authorization, and belongs to the Connector
// against a state snapshot no client can hold.

import type { SolanaSignedPermit } from '../permit/index.js';
import {
  MAX_DECRYPTION_REQUEST_BITS,
  decryptionRequestBitsOfHandle,
} from '../../core/handle/decryptionRequestBudget.js';
import { isBytes32 } from '../../core/base/bytes.js';
import { bytes32ToHandle } from '../../core/handle/FhevmHandle.js';
import { encodeSolanaKmsRouting } from '../permit/index.js';
import { bytesToHex } from '../proof.js';

/** The attestation type that selects this envelope at the relayer. */
export const SOLANA_SRFC38_ATTESTATION_TYPE = 'solana-srfc38-user-decrypt-v1';

/**
 * The handle-count cap: `MAX_SOLANA_USER_DECRYPT_HANDLES` in the Gateway's `Decryption.sol`. The
 * Connector refuses the same count terminally, so a request past it can only ever be paid for and
 * lost — the parity test beside this module pins the two constants to each other.
 */
export const MAX_SOLANA_USER_DECRYPT_HANDLES = 33;

/** One handle to decrypt: the handle, the key it was allowed to, and the account it lives in. */
export interface SolanaUserDecryptHandleEntry {
  /** The 32-byte ciphertext handle. */
  readonly handle: Uint8Array;
  /**
   * The 32-byte key whose allow on the handle authorizes this entry: the requester itself on a
   * direct entry, the delegator on a delegated one.
   */
  readonly allowedKey: Uint8Array;
  /** The 32-byte address of the `EncryptedValue` account the handle lives in. */
  readonly encryptedValueAccount: Uint8Array;
}

/** One handle entry, as it travels. */
export interface SolanaUserDecryptHandleJson {
  readonly handle: string;
  readonly allowedKey: string;
  readonly encryptedValueAccount: string;
}

/** The attested payload: the eight signed permit fields, plus the unsigned handle entries. */
export interface SolanaUserDecryptPayloadJson {
  readonly userPubkey: string;
  readonly transportKey: string;
  readonly allowedScopes: readonly string[];
  readonly requestValidity: { readonly startTimestamp: string; readonly durationSeconds: string };
  readonly verifyingProgramId: string;
  readonly chainId: string;
  readonly extraData: string;
  readonly handles: readonly SolanaUserDecryptHandleJson[];
}

/** The request body posted to the relayer's v3 user-decrypt endpoint. */
export interface SolanaUserDecryptRequestJson {
  readonly attestationType: typeof SOLANA_SRFC38_ATTESTATION_TYPE;
  readonly attestedPayload: SolanaUserDecryptPayloadJson;
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
  | { readonly reason: 'too-many-handles'; readonly count: number; readonly max: number }
  | { readonly reason: 'handle-without-a-width'; readonly index: number }
  | { readonly reason: 'budget-exceeded'; readonly bits: number; readonly budget: number }
  | { readonly reason: 'foreign-host-chain'; readonly index: number; readonly chainId: bigint }
  | {
      readonly reason: 'entry-field-width';
      readonly index: number;
      readonly field: 'allowedKey' | 'encryptedValueAccount';
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
    case 'too-many-handles':
      return `the request names ${failure.count} handles, above the cap of ${failure.max}`;
    case 'handle-without-a-width':
      return `handle ${failure.index} is not a handle of a type this protocol assigns a bit width to`;
    case 'budget-exceeded':
      return `the request sums to ${failure.bits} bits, above the decryption budget of ${failure.budget}`;
    case 'foreign-host-chain':
      return `handle ${failure.index} belongs to host chain ${failure.chainId}, which the permit was not signed for`;
    case 'entry-field-width':
      return `entry ${failure.index}: ${failure.field} is not 32 bytes`;
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The admission a request must pass: every rule checkable from the entries and the permit alone —
 * the handle count cap, the bit budget, each handle's host chain, the width of each identity field.
 * A request that can never be submitted is refused before it costs a relayer fee.
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

  // Widths and the budget first, over the whole list: the sum is a property of the request, not of
  // any one entry, so it is settled before the per-entry rules name individual positions.
  solanaUserDecryptRequestBits(entries.map((entry) => entry.handle));

  for (const [index, entry] of entries.entries()) {
    // The handle parses — the width check above proved that much — so this narrowing cannot fail;
    // it re-states the width failure only to convince the type system, not the reader.
    if (!isBytes32(entry.handle)) {
      throw new SolanaUserDecryptRequestError({ reason: 'handle-without-a-width', index });
    }
    // The chain id the handle embeds must be the one chain the permit was signed for.
    const embeddedChainId = bytes32ToHandle(entry.handle).chainId;
    if (embeddedChainId !== chainId) {
      throw new SolanaUserDecryptRequestError({ reason: 'foreign-host-chain', index, chainId: embeddedChainId });
    }

    if (entry.allowedKey.length !== 32) {
      throw new SolanaUserDecryptRequestError({ reason: 'entry-field-width', index, field: 'allowedKey' });
    }
    if (entry.encryptedValueAccount.length !== 32) {
      throw new SolanaUserDecryptRequestError({ reason: 'entry-field-width', index, field: 'encryptedValueAccount' });
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
    attestedPayload: {
      userPubkey: bytesToHex(fields.userPubkey),
      transportKey: bytesToHex(fields.transportKey),
      allowedScopes: fields.allowedScopes.map((scope) => bytesToHex(scope)),
      requestValidity: {
        startTimestamp: fields.startTimestamp.toString(),
        durationSeconds: fields.durationSeconds.toString(),
      },
      verifyingProgramId: bytesToHex(fields.verifyingProgramId),
      chainId: fields.chainId.toString(),
      extraData: bytesToHex(encodeSolanaKmsRouting(fields.kmsRouting)),
      handles: entries.map((entry) => ({
        handle: bytesToHex(entry.handle),
        allowedKey: bytesToHex(entry.allowedKey),
        encryptedValueAccount: bytesToHex(entry.encryptedValueAccount),
      })),
    },
    signature: bytesToHex(signedPermit.signature),
  };
}

/**
 * The volume admission the Gateway runs statelessly: the handle-count cap, then the bit cost of the
 * set as the Gateway sums it.
 *
 * @param handles - The 32-byte handles, in any order.
 * @throws SolanaUserDecryptRequestError - If the list is over the cap, a handle has no width, or the
 * sum is above the budget.
 */
export function solanaUserDecryptRequestBits(handles: readonly Uint8Array[]): number {
  // The count first: it is a property of the list, so it is settled before any one handle is named.
  if (handles.length > MAX_SOLANA_USER_DECRYPT_HANDLES) {
    throw new SolanaUserDecryptRequestError({
      reason: 'too-many-handles',
      count: handles.length,
      max: MAX_SOLANA_USER_DECRYPT_HANDLES,
    });
  }

  let bits = 0;
  for (const [index, handle] of handles.entries()) {
    const width = decryptionRequestBitsOfHandle(handle);
    if (width === undefined) {
      throw new SolanaUserDecryptRequestError({ reason: 'handle-without-a-width', index });
    }
    bits += width;
  }
  if (bits > MAX_DECRYPTION_REQUEST_BITS) {
    throw new SolanaUserDecryptRequestError({
      reason: 'budget-exceeded',
      bits,
      budget: MAX_DECRYPTION_REQUEST_BITS,
    });
  }
  return bits;
}
