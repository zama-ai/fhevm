import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * A copy of the key and CRS bytes, so the verified bytes and the caller's
 * arrays never share a buffer: changing one never changes the other.
 * `new Uint8Array(…)` always copies, even a Node.js `Buffer` (whose `slice`
 * is a view).
 */
export function copyFheEncryptionKeyBytes(fheEncryptionKey: FheEncryptionKeyBytes): FheEncryptionKeyBytes {
  return {
    publicKeyBytes: {
      ...fheEncryptionKey.publicKeyBytes,
      bytes: new Uint8Array(fheEncryptionKey.publicKeyBytes.bytes),
    },
    crsBytes: {
      ...fheEncryptionKey.crsBytes,
      bytes: new Uint8Array(fheEncryptionKey.crsBytes.bytes),
    },
    metadata: { ...fheEncryptionKey.metadata },
  };
}
