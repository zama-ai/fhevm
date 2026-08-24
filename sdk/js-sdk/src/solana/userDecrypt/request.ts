// The v3 user-decryption request: one signed permit, and the handles asked for under it.
//
// The permit is the reusable object and the request is not. Every request cites the one signature the
// wallet produced, and this builder never signs, never asks a wallet for anything, and never mutates
// the permit — which is what makes a retry, a rebuilt proof and the switch to a historical proof cost
// no further signature.
//
// The JSON this produces is the relayer's v3 wire shape, pinned by
// `solana/test-fixtures/user-decrypt/relayer_envelope_v1.json` and read from both sides of the seam.
// Two conventions in it are load-bearing: every byte string is `0x`-hex, and every 64-bit number is a
// decimal string — a JSON number would arrive at the relayer as a double and lose the chain id.
//
// What this builder checks is what a layer before the Gateway can check without reading host state
// itself: the handle count cap and the bit budget, the handles' host chain, each entry's leaf count
// agreeing with its access proof about current-versus-historical, the form of a non-empty proof and
// that it verifies against the peaks its evidence carries, and that there is at least one handle.
// Everything else — ownership, delegation, the validity window, the revocation watermark — is
// authorization, and belongs to the Connector against a state snapshot no client can hold.

import type { SolanaAccessEvidence, SolanaHandleRequest } from './evidence.js';
import type { SolanaSignedPermit } from '../permit/index.js';
import {
  MAX_DECRYPTION_REQUEST_BITS,
  decryptionRequestBitsOfHandle,
} from '../../core/handle/decryptionRequestBudget.js';
import { isBytes32 } from '../../core/base/bytes.js';
import { bytes32ToHandle } from '../../core/handle/FhevmHandle.js';
import { encodeSolanaKmsRouting } from '../permit/index.js';
import { bytesToHex, decodeMmrProof, verifyHistoricalAccessProof } from '../proof.js';

/** The attestation type that selects this envelope at the relayer. */
export const SOLANA_SRFC38_ATTESTATION_TYPE = 'solana-srfc38-user-decrypt-v1';

/**
 * The handle-count cap: `MAX_SOLANA_USER_DECRYPT_HANDLES` in the Gateway's `Decryption.sol`. The
 * Connector refuses the same count terminally, so a request past it can only ever be paid for and
 * lost — the parity test beside this module pins the two constants to each other.
 */
export const MAX_SOLANA_USER_DECRYPT_HANDLES = 33;

/** The widest value `proofLeafCount` may carry: the wire field is an unsigned 64-bit decimal. */
const MAX_PROOF_LEAF_COUNT = 0xffff_ffff_ffff_ffffn;

/** One handle entry, as it travels. */
export interface SolanaUserDecryptHandleJson {
  readonly handle: string;
  readonly subject: string;
  readonly encryptedValueId: string;
  readonly proofLeafCount: string;
  readonly accessProof: string;
}

/** The attested payload: the eight signed permit fields, plus the unsigned per-handle evidence. */
export interface SolanaUserDecryptPayloadJson {
  readonly userPubkey: string;
  readonly transportKey: string;
  readonly allowedAclDomainKeys: readonly string[];
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
  | { readonly reason: 'evidence-field-width'; readonly index: number; readonly field: 'subject' | 'encryptedValueId' }
  | { readonly reason: 'proof-leaf-count-range'; readonly index: number }
  | { readonly reason: 'proof-mode-mismatch'; readonly index: number }
  | { readonly reason: 'access-proof-form'; readonly index: number }
  | { readonly reason: 'access-proof-refuted'; readonly index: number };

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
    case 'evidence-field-width':
      return `entry ${failure.index}: ${failure.field} is not 32 bytes`;
    case 'proof-leaf-count-range':
      return `entry ${failure.index}: the proof leaf count does not fit an unsigned 64-bit integer`;
    case 'proof-mode-mismatch':
      return `entry ${failure.index}: the leaf count and the access proof disagree about whether the access is current or historical`;
    case 'access-proof-form':
      return `entry ${failure.index}: the access proof is not a bare borsh MMR proof`;
    case 'access-proof-refuted':
      return `entry ${failure.index}: the access proof does not prove this handle's access against the peaks it came with`;
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * The admission a request must pass before anything else is spent on it. Every rule here is
 * checkable from the requests and the permit alone — the handle count cap, the bit budget, each
 * handle's host chain, the width of each identity field — so a request that can never be submitted
 * is refused before evidence is fetched for it: no host RPC, no proof-service work, no relayer fee.
 *
 * The request builder runs the same rules again over the resolved evidence; running them twice is
 * cheaper than a seam through which an unadmitted request could reach the wire.
 *
 * @param admission.chainId - The one host chain the permit was signed for.
 * @param admission.requests - The handles, in the order they will be requested.
 * @throws SolanaUserDecryptRequestError - On the first rule the request breaks.
 */
export function admitSolanaUserDecryptRequest(admission: {
  readonly chainId: bigint;
  readonly requests: readonly SolanaHandleRequest[];
}): void {
  const { chainId, requests } = admission;

  if (requests.length === 0) {
    throw new SolanaUserDecryptRequestError({ reason: 'no-handles' });
  }

  // Widths and the budget first, over the whole list: the sum is a property of the request, not of
  // any one entry, so it is settled before the per-entry rules name individual positions.
  solanaUserDecryptRequestBits(requests.map((request) => request.handle));

  for (const [index, request] of requests.entries()) {
    // The handle parses — the width check above proved that much — so this narrowing cannot fail;
    // it re-states the width failure only to convince the type system, not the reader.
    if (!isBytes32(request.handle)) {
      throw new SolanaUserDecryptRequestError({ reason: 'handle-without-a-width', index });
    }
    // The chain id the handle embeds must be the one chain the permit was signed for.
    const embeddedChainId = bytes32ToHandle(request.handle).chainId;
    if (embeddedChainId !== chainId) {
      throw new SolanaUserDecryptRequestError({ reason: 'foreign-host-chain', index, chainId: embeddedChainId });
    }

    if (request.subject.length !== 32) {
      throw new SolanaUserDecryptRequestError({ reason: 'evidence-field-width', index, field: 'subject' });
    }
    if (request.encryptedValueId.length !== 32) {
      throw new SolanaUserDecryptRequestError({ reason: 'evidence-field-width', index, field: 'encryptedValueId' });
    }
  }
}

/**
 * Assembles the request body for a signed permit and the resolved entries.
 *
 * Duplicates and their order are preserved exactly as given: each occurrence is authorized on its
 * own, counts toward the budget on its own, and is bound by the response linker at its position.
 * Trimming an oversize request to fit would be answered as a request the caller never made, so an
 * oversize one is refused instead.
 *
 * @param request.signedPermit - The permit and its one signature.
 * @param request.entries - The resolved entries, in the order they will be requested.
 * @throws SolanaUserDecryptRequestError - On the first entry or budget rule the request breaks.
 */
export function buildSolanaUserDecryptRequest(request: {
  readonly signedPermit: SolanaSignedPermit;
  readonly entries: readonly SolanaAccessEvidence[];
}): SolanaUserDecryptRequestJson {
  const { signedPermit, entries } = request;
  const fields = signedPermit.fields;

  admitSolanaUserDecryptRequest({ chainId: fields.chainId, requests: entries });

  for (const [index, entry] of entries.entries()) {
    // The leaf count travels as an unsigned 64-bit decimal; a bigint outside that range has no wire
    // form, and serializing it anyway would put a request on the network only the server can refuse.
    if (entry.proofLeafCount < 0n || entry.proofLeafCount > MAX_PROOF_LEAF_COUNT) {
      throw new SolanaUserDecryptRequestError({ reason: 'proof-leaf-count-range', index });
    }
    // Current access is an empty proof AND a zero leaf count; historical is a proof AND the count it
    // was built against. An entry claiming one mode in each field is not a request either mode makes,
    // and the Connector is bound to refuse it — after the fee.
    if ((entry.accessProof.length === 0) !== (entry.proofLeafCount === 0n)) {
      throw new SolanaUserDecryptRequestError({ reason: 'proof-mode-mismatch', index });
    }

    // An empty proof is the current-access mode, not a malformed proof: it never sees the decoder.
    // A non-empty one must be a bare borsh MMR proof exactly — the relayer and the Connector both
    // refuse anything else, and refusing it here costs no submission.
    if (entry.accessProof.length > 0) {
      let proof;
      try {
        proof = decodeMmrProof(entry.accessProof);
      } catch {
        throw new SolanaUserDecryptRequestError({ reason: 'access-proof-form', index });
      }
      // A well-formed proof that proves nothing — a proof service answering for the wrong handle,
      // or a snapshot the proof does not belong to — would be refused by the Connector after the
      // fee, and from here that refusal is indistinguishable from an unanswered request. Verifying
      // against the peaks the evidence came with catches it before it costs the attempt budget.
      // The leaf binds the account's own pubkey — the PDA, not the wire identity.
      if (
        !verifyHistoricalAccessProof(
          entry.encryptedValueAccount,
          entry.peaks,
          entry.proofLeafCount,
          entry.handle,
          entry.subject,
          proof,
        )
      ) {
        throw new SolanaUserDecryptRequestError({ reason: 'access-proof-refuted', index });
      }
    }
  }

  return {
    attestationType: SOLANA_SRFC38_ATTESTATION_TYPE,
    attestedPayload: {
      userPubkey: bytesToHex(fields.userPubkey),
      transportKey: bytesToHex(fields.transportKey),
      allowedAclDomainKeys: fields.allowedAclDomainKeys.map((key) => bytesToHex(key)),
      requestValidity: {
        startTimestamp: fields.startTimestamp.toString(),
        durationSeconds: fields.durationSeconds.toString(),
      },
      verifyingProgramId: bytesToHex(fields.verifyingProgramId),
      chainId: fields.chainId.toString(),
      extraData: bytesToHex(encodeSolanaKmsRouting(fields.kmsRouting)),
      handles: entries.map((entry) => ({
        handle: bytesToHex(entry.handle),
        subject: bytesToHex(entry.subject),
        encryptedValueId: bytesToHex(entry.encryptedValueId),
        proofLeafCount: entry.proofLeafCount.toString(),
        accessProof: bytesToHex(entry.accessProof),
      })),
    },
    signature: bytesToHex(signedPermit.signature),
  };
}

/**
 * The volume admission the Gateway runs statelessly: the handle-count cap, then the bit cost of the
 * set as the Gateway sums it.
 *
 * Exported because both rules are worth checking before the evidence is fetched: resolving proofs
 * for a request that cannot be submitted spends the proof service's work for nothing.
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
