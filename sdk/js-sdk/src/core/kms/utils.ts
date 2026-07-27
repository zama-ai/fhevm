import type { Handle } from '../types/encryptedTypes-p.js';
import type { UintNumber } from '../types/primitives.js';

const MAX_KMS_DECRYPT_DECRYPTION_BIT_LIMIT = 2048 as UintNumber;

/**
 * Asserts that the total encrypted bits across the given handles does not
 * exceed the maximum number of bits the KMS is able to decrypt in a single call.
 *
 * @param handles - Handles to sum encrypted bits for
 * @returns The total encrypted bits across all handles
 * @throws {Error} If the total exceeds the limit
 */
export function assertKmsDecryptionBitLimit(handles: readonly Handle[]): UintNumber {
  let total: number = 0;

  for (const handle of handles) {
    total += handle.encryptionBits;

    if (total > MAX_KMS_DECRYPT_DECRYPTION_BIT_LIMIT) {
      throw new Error(
        `Cannot decrypt more than ${MAX_KMS_DECRYPT_DECRYPTION_BIT_LIMIT} encrypted bits in a single request`,
      );
    }
  }

  return total as UintNumber;
}
