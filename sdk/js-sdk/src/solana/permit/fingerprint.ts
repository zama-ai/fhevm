// The transport-key fingerprint the canonical text commits to.
//
// The key itself does not fit a wallet screen, so the text shows a digest of it. The digest is
// always recomputed from the full key and is never accepted as an input — an implementation that
// took a fingerprint as a parameter would let an attacker pair someone else's signed text with
// their own transport key.

import { shake256 } from '@noble/hashes/sha3.js';

/**
 * The 32-byte SHAKE256 digest of a transport key, plain and untagged (no domain separator).
 *
 * @param transportKey - The full serialized transport key, exactly as the permit carries it.
 */
export function transportKeyFingerprint(transportKey: Uint8Array): Uint8Array {
  return shake256(transportKey, { dkLen: 32 });
}
