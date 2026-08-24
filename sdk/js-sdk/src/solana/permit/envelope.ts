// Envelope construction and signature verification.
//
// The wallet signs the canonical text inside an offchain-message envelope whose first byte is
// illegal as a transaction's first byte — that is the structural guarantee that a permit signature
// can never be replayed as a transaction.
//
// Verification is reconstruction: the envelope is rebuilt locally from validated typed fields and
// the signature is checked over those bytes. Neither the text nor the envelope is ever accepted
// from a caller, which is why the signed bytes are never parsed for a security decision — and why
// neither function here has a variant taking a text or an envelope.
//
// What "verifies" means: the signature scalar must be reduced and neither R nor A may be of small
// order (`verify_strict`, not the permissive entry point), the user pubkey must be an on-curve point
// and canonically encoded. Five implementations verify these permits on five libraries, and
// libraries disagree about exactly these cases; the strict reading is spelled out rather than
// inherited from whichever library each side links.
//
// That is also why the group equation is checked here on the curve library's point arithmetic
// rather than through its `verify` entry point: this library's strictest mode still admits a
// small-order R and checks the cofactored equation, either of which would let it accept a signature
// the Rust canon's `verify_strict` refuses. The equation checked below is the cofactorless
// `[s]B = R + [k]A`, over the wire encodings of R and A exactly as they were signed.

import type { SolanaPermitFields } from './types.js';
import { ed25519 } from '@noble/curves/ed25519.js';
import { sha512 } from '@noble/hashes/sha2.js';
import { SolanaPermitError } from './errors.js';
import { renderSolanaPermitText } from './render.js';
import { PERMIT_SIGNATURE_LEN } from './types.js';

/** The offchain-message preamble. Its leading `0xff` cannot begin a transaction. */
export const PERMIT_ENVELOPE_PREAMBLE = new Uint8Array([
  0xff, 0x73, 0x6f, 0x6c, 0x61, 0x6e, 0x61, 0x20, 0x6f, 0x66, 0x66, 0x63, 0x68, 0x61, 0x69, 0x6e,
]);

/** Envelope format version. */
export const PERMIT_ENVELOPE_VERSION = 1;

/** A permit envelope always has exactly one signer, the permit's own user. */
export const PERMIT_ENVELOPE_SIGNER_COUNT = 1;

/** Half of a signature: the width of the encoded point R, and of the scalar s after it. */
const SIGNATURE_POINT_LEN = PERMIT_SIGNATURE_LEN / 2;

/**
 * Reconstructs the envelope bytes the wallet signed.
 *
 * @param fields - Validated permit fields.
 */
export function buildSolanaPermitEnvelope(fields: SolanaPermitFields): Uint8Array {
  const text = new TextEncoder().encode(renderSolanaPermitText(fields));

  const envelope = new Uint8Array(PERMIT_ENVELOPE_PREAMBLE.length + 2 + fields.userPubkey.length + text.length);
  envelope.set(PERMIT_ENVELOPE_PREAMBLE, 0);
  envelope[PERMIT_ENVELOPE_PREAMBLE.length] = PERMIT_ENVELOPE_VERSION;
  envelope[PERMIT_ENVELOPE_PREAMBLE.length + 1] = PERMIT_ENVELOPE_SIGNER_COUNT;
  // The sole signer is the permit's own user, which is also what the text's `User:` line names —
  // so the screen a human read and the bytes their wallet signed cannot disagree about who is
  // consenting.
  envelope.set(fields.userPubkey, PERMIT_ENVELOPE_PREAMBLE.length + 2);
  // No length prefix and no application domain: the text runs to the end of the message.
  envelope.set(text, PERMIT_ENVELOPE_PREAMBLE.length + 2 + fields.userPubkey.length);
  return envelope;
}

/**
 * Verifies a signature over the locally reconstructed envelope.
 *
 * Returns nothing and throws `SolanaPermitError` on any failure: a boolean would invite a caller to
 * carry on with an unverified permit.
 *
 * A signature of the wrong width is a mismatch like any other, not a separate failure: there is no
 * envelope it verifies over, and the shared rule set names no width rule for it.
 *
 * @param fields - Validated permit fields.
 * @param signature - The claimed 64-byte Ed25519 signature.
 */
export function verifySolanaPermitSignature(fields: SolanaPermitFields, signature: Uint8Array): void {
  const userPoint = usableUserPubkeyPoint(fields.userPubkey);

  if (signature.length !== PERMIT_SIGNATURE_LEN) {
    throw new SolanaPermitError({ code: 'SignatureMismatch' });
  }
  const encodedR = signature.subarray(0, SIGNATURE_POINT_LEN);

  // R under the same strict decoding as the key: canonical encoding, on the curve. Unlike the key's
  // failures, R's are properties of the signature, so they all fail as a mismatch — including a
  // small-order R, which `verify_strict` refuses and permissive verifiers accept.
  let pointR;
  try {
    pointR = ed25519.Point.fromBytes(encodedR, false);
  } catch {
    throw new SolanaPermitError({ code: 'SignatureMismatch' });
  }
  if (pointR.isSmallOrder()) {
    throw new SolanaPermitError({ code: 'SignatureMismatch' });
  }

  // The scalar must arrive reduced: admitting s and s + l as two spellings of one signature is the
  // malleability the strict reading exists to remove.
  const scalarS = littleEndianToBigint(signature.subarray(SIGNATURE_POINT_LEN));
  if (scalarS >= ed25519.Point.Fn.ORDER) {
    throw new SolanaPermitError({ code: 'SignatureMismatch' });
  }

  // The challenge hashes the wire encodings exactly as they were signed: R from the signature, A
  // from the permit. Both are canonical here — the strict decodes above made sure — so re-encoding
  // would produce the same bytes; hashing the originals keeps that a fact rather than a hope.
  const envelope = buildSolanaPermitEnvelope(fields);
  const challenge = new Uint8Array(encodedR.length + fields.userPubkey.length + envelope.length);
  challenge.set(encodedR, 0);
  challenge.set(fields.userPubkey, encodedR.length);
  challenge.set(envelope, encodedR.length + fields.userPubkey.length);
  const scalarK = littleEndianToBigint(sha512(challenge)) % ed25519.Point.Fn.ORDER;

  // The cofactorless group equation, checked exactly: no `clearCofactor`, so a signature that only
  // balances up to a torsion component does not verify.
  const left = ed25519.Point.BASE.multiplyUnsafe(scalarS);
  const right = pointR.add(userPoint.multiplyUnsafe(scalarK));
  if (!left.equals(right)) {
    throw new SolanaPermitError({ code: 'SignatureMismatch' });
  }
}

/**
 * Decodes a permit's user pubkey into a point that can verify, or says it cannot.
 *
 * Three ways a key is unusable — a non-canonical encoding, a point off the curve, a point of small
 * order — all reported alike, because they are all "this permit names something that is not a
 * wallet key", none of them a statement about the signature that arrived with it. Small order is
 * refused here rather than left to the group equation, where a cofactored verifier would let an
 * all-zero signature pass against such a key: a permit carrying the consent of a wallet nobody owns
 * is a fact about the permit, and every verifier has to report it the same way.
 *
 * @param userPubkey - The permit's 32-byte user pubkey, exactly as signed.
 */
function usableUserPubkeyPoint(userPubkey: Uint8Array): InstanceType<typeof ed25519.Point> {
  let point;
  try {
    // Strict decoding: the y-coordinate must sit below the field modulus and the point must be on
    // the curve — the same boundary the Rust canon draws, RFC 8032's rather than ZIP-215's.
    point = ed25519.Point.fromBytes(userPubkey, false);
  } catch {
    throw new SolanaPermitError({ code: 'UnusableUserPubkey' });
  }
  if (point.isSmallOrder()) {
    throw new SolanaPermitError({ code: 'UnusableUserPubkey' });
  }
  return point;
}

/**
 * Reads little-endian bytes as an unsigned bigint.
 *
 * @param bytes - The little-endian encoding, any length.
 */
function littleEndianToBigint(bytes: Uint8Array): bigint {
  let value = 0n;
  for (const [index, byte] of bytes.entries()) {
    value |= BigInt(byte) << (8n * BigInt(index));
  }
  return value;
}
