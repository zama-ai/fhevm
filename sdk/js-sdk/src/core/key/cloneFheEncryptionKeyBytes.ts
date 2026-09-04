import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';

////////////////////////////////////////////////////////////////////////////////

/**
 * Checks the public DTO shape needed for safe ownership transfer.
 *
 * This does not verify that the byte arrays are valid TFHE serializations;
 * native deserialization remains the serialization verifier.
 */
export function isFheEncryptionKeyBytesShape(value: unknown): value is FheEncryptionKeyBytes {
  if (value === null || typeof value !== 'object') {
    return false;
  }
  const candidate = value as {
    readonly publicKeyBytes?: { readonly id?: unknown; readonly bytes?: unknown };
    readonly crsBytes?: { readonly id?: unknown; readonly capacity?: unknown; readonly bytes?: unknown };
    readonly metadata?: { readonly chainId?: unknown; readonly relayerUrl?: unknown };
  };
  return (
    typeof candidate.publicKeyBytes?.id === 'string' &&
    candidate.publicKeyBytes.id.length > 0 &&
    isUint8Array(candidate.publicKeyBytes.bytes) &&
    candidate.publicKeyBytes.bytes.byteLength > 0 &&
    typeof candidate.crsBytes?.id === 'string' &&
    candidate.crsBytes.id.length > 0 &&
    candidate.crsBytes.capacity === 2048 &&
    isUint8Array(candidate.crsBytes.bytes) &&
    candidate.crsBytes.bytes.byteLength > 0 &&
    typeof candidate.metadata?.chainId === 'number' &&
    Number.isSafeInteger(candidate.metadata.chainId) &&
    candidate.metadata.chainId >= 0 &&
    typeof candidate.metadata.relayerUrl === 'string' &&
    candidate.metadata.relayerUrl.length > 0
  );
}

function isUint8Array(value: unknown): value is Uint8Array {
  return ArrayBuffer.isView(value) && Object.prototype.toString.call(value) === '[object Uint8Array]';
}

/** Returns an independent SDK-owned copy of serialized FHE material. */
export function cloneFheEncryptionKeyBytes(keyBytes: FheEncryptionKeyBytes): FheEncryptionKeyBytes {
  if (!isFheEncryptionKeyBytesShape(keyBytes)) {
    throw new FhevmConfigError({
      message: 'fheEncryptionKey must contain serialized public-key and CRS bytes.',
    });
  }
  return Object.freeze({
    publicKeyBytes: Object.freeze({
      ...keyBytes.publicKeyBytes,
      bytes: new Uint8Array(keyBytes.publicKeyBytes.bytes),
    }),
    crsBytes: Object.freeze({ ...keyBytes.crsBytes, bytes: new Uint8Array(keyBytes.crsBytes.bytes) }),
    metadata: Object.freeze({ ...keyBytes.metadata }),
  });
}
