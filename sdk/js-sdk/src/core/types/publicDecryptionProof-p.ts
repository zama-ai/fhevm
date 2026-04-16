import type { ClearValue } from './encryptedTypes-p.js';
import type { BytesHex } from './primitives.js';
import type { NonEmptyReadonlyArray } from './utils.js';

////////////////////////////////////////////////////////////////////////////////
//
// PublicDecryptionProof
//
////////////////////////////////////////////////////////////////////////////////

/**
 * Result of a public decryption request.
 *
 * **Order matters.** All fields are bound to the same ordering as the input
 * `encryptedValues`. The `decryptionProof` is cryptographically tied to the
 * exact order of `orderedClearValues` and `orderedAbiEncodedClearValues`.
 * Reordering any field independently will cause on-chain proof verification
 * (`FHE.checkSignatures`) to fail silently.
 */
export type PublicDecryptionProof = {
  /**
   * Cryptographic proof that `orderedClearValues` are the correct decryptions.
   * Valid only for the exact ordering of `orderedClearValues` and
   * `orderedAbiEncodedClearValues`. Pass all three to `FHE.checkSignatures`
   * in Solidity for on-chain verification.
   */
  readonly decryptionProof: BytesHex;
  /** Decrypted clear values, in the same order as the input `encryptedValues`. Do not reorder. */
  readonly orderedClearValues: NonEmptyReadonlyArray<ClearValue>;
  /** ABI-encoded clear values, in the same order as the input `encryptedValues`. Do not reorder. */
  readonly orderedAbiEncodedClearValues: BytesHex;
  readonly extraData: BytesHex;
};
