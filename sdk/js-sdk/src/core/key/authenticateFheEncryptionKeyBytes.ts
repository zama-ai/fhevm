import type {
  FheEncryptionKeyBytes,
  FheEncryptionKeyDigests,
  FheEncryptionKeyMetadata,
  FheEncryptionKeyTrust,
} from '../types/fheEncryptionKey.js';
import { shake256 } from '@noble/hashes/sha3.js';
import { asBytes32Hex, bytes32ToHex, isBytes32Hex } from '../base/bytes.js';
import { EncryptionError } from '../errors/EncryptionError.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { cloneFheEncryptionKeyBytes, isFheEncryptionKeyBytesShape } from './cloneFheEncryptionKeyBytes.js';

const PUBLIC_KEY_DIGEST_DOMAIN = new TextEncoder().encode('PDAT_KEY');
const CRS_DIGEST_DOMAIN = new TextEncoder().encode('PDAT_CRS');

export type ExpectedFheEncryptionKeyDigests = FheEncryptionKeyDigests & {
  // Active KMSGeneration ids identify the trust snapshot and cache entry. Relayer
  // dataId values are opaque storage identifiers; the digests authenticate bytes.
  readonly publicKeyId?: bigint | undefined;
  readonly crsId?: bigint | undefined;
};

////////////////////////////////////////////////////////////////////////////////

export function normalizeFheEncryptionKeyTrust(
  trust: FheEncryptionKeyTrust | undefined,
): FheEncryptionKeyTrust | undefined {
  return trust === undefined || typeof trust === 'function' ? trust : normalizeFheEncryptionKeyDigests(trust);
}

export async function resolveFheEncryptionKeyTrust(
  trust: FheEncryptionKeyTrust,
  metadata: FheEncryptionKeyMetadata,
): Promise<FheEncryptionKeyDigests> {
  return normalizeFheEncryptionKeyDigests(typeof trust === 'function' ? await trust(metadata) : trust);
}

export function normalizeFheEncryptionKeyDigests(value: unknown): FheEncryptionKeyDigests {
  if (
    value === null ||
    typeof value !== 'object' ||
    !('publicKeyDigest' in value) ||
    !('crsDigest' in value) ||
    !isBytes32Hex(value.publicKeyDigest) ||
    !isBytes32Hex(value.crsDigest)
  ) {
    throw new FhevmConfigError({
      message: 'fheEncryptionKeyTrust must resolve to 32-byte public-key and CRS digests.',
    });
  }
  return Object.freeze({
    publicKeyDigest: asBytes32Hex(value.publicKeyDigest.toLowerCase()),
    crsDigest: asBytes32Hex(value.crsDigest.toLowerCase()),
  });
}

/** Computes the digests stored by KMSGeneration for serialized FHE material. */
export function computeFheEncryptionKeyDigests(keyBytes: FheEncryptionKeyBytes): FheEncryptionKeyDigests {
  if (!isFheEncryptionKeyBytesShape(keyBytes)) {
    throw new FhevmConfigError({
      message: 'fheEncryptionKey must contain serialized public-key and CRS bytes.',
    });
  }
  return Object.freeze({
    publicKeyDigest: _digest(PUBLIC_KEY_DIGEST_DOMAIN, keyBytes.publicKeyBytes.bytes),
    crsDigest: _digest(CRS_DIGEST_DOMAIN, keyBytes.crsBytes.bytes),
  });
}

/** Takes ownership of raw bytes and authenticates them before cache admission. */
export function authenticateFheEncryptionKeyBytes(
  keyBytes: FheEncryptionKeyBytes,
  expected: ExpectedFheEncryptionKeyDigests,
  chainId: number,
): FheEncryptionKeyBytes {
  if (!isFheEncryptionKeyBytesShape(keyBytes)) {
    throw new EncryptionError({ message: 'The relayer returned malformed FHE public-key or CRS bytes.' });
  }

  const owned = cloneFheEncryptionKeyBytes(keyBytes);
  const actual = computeFheEncryptionKeyDigests(owned);
  assertFheEncryptionKeyDigestsMatch(actual, expected, chainId);
  return owned;
}

export function assertFheEncryptionKeyDigestsMatch(
  actual: FheEncryptionKeyDigests,
  expected: FheEncryptionKeyDigests,
  chainId: number,
): void {
  if (actual.publicKeyDigest !== expected.publicKeyDigest || actual.crsDigest !== expected.crsDigest) {
    throw new EncryptionError({
      message: `FHE encryption key mismatch for chain ${chainId.toString()}. Nothing was encrypted.`,
    });
  }
}

function _digest(domain: Uint8Array, bytes: Uint8Array): `0x${string}` {
  const hash = shake256.create({ dkLen: 32 });
  hash.update(domain);
  hash.update(bytes);
  return bytes32ToHex(hash.digest());
}
