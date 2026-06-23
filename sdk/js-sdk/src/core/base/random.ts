import type { Bytes } from '../types/primitives.js';

////////////////////////////////////////////////////////////////////////////////
// Random
////////////////////////////////////////////////////////////////////////////////

/**
 * Minimum seed length (in bytes) accepted by TFHE-rs seeded public encryption.
 *
 * The seed is prefixed with an 8-byte domain separator and expanded through an
 * AES-CTR XOF inside TFHE-rs. TFHE-rs requires the raw seed to be at least
 * 16 bytes; the WASM bindings do not validate this for us, so callers must.
 */
export const MIN_ENCRYPTION_SEED_BYTES = 16;

/**
 * Default length (in bytes) of a freshly generated encryption seed.
 *
 * 32 bytes provides a comfortable margin above {@link MIN_ENCRYPTION_SEED_BYTES}.
 */
export const DEFAULT_ENCRYPTION_SEED_BYTES = 32;

/**
 * Returns `length` cryptographically secure random bytes.
 *
 * Isomorphic: relies on the Web Crypto `crypto.getRandomValues`, available in
 * browsers, web workers, and Node.js (globalThis.crypto, Node 18+).
 *
 * @param length - Number of random bytes to generate. Must be a positive integer.
 * @throws If `length` is not a positive integer.
 */
export function randomBytes(length: number): Bytes {
  if (!Number.isInteger(length) || length <= 0) {
    throw new Error(`randomBytes: length must be a positive integer, got ${String(length)}`);
  }
  const out = new Uint8Array(length);
  crypto.getRandomValues(out);
  return out;
}

/**
 * Generates a fresh CSPRNG seed suitable for {@link https://docs.zama.ai | seeded encryption}.
 *
 * The returned seed is the secret shared between an encrypting machine and an
 * independent verifier: both feed the same seed (plus the same plaintext, public
 * key, and metadata) into seeded encryption and compare the resulting bytes. The
 * seed must be kept secret from the outside world — knowledge of it can break the
 * encryption — and is never transmitted with the ciphertext to the relayer.
 *
 * @param length - Seed length in bytes. Defaults to {@link DEFAULT_ENCRYPTION_SEED_BYTES}.
 *   Must be at least {@link MIN_ENCRYPTION_SEED_BYTES}.
 * @throws If `length` is below {@link MIN_ENCRYPTION_SEED_BYTES}.
 */
export function generateEncryptionSeed(length: number = DEFAULT_ENCRYPTION_SEED_BYTES): Bytes {
  if (!Number.isInteger(length) || length < MIN_ENCRYPTION_SEED_BYTES) {
    throw new Error(
      `generateEncryptionSeed: length must be an integer >= ${String(MIN_ENCRYPTION_SEED_BYTES)}, got ${String(length)}`,
    );
  }
  return randomBytes(length);
}

/**
 * Asserts that `seed` is a valid encryption seed (a `Uint8Array` of at least
 * {@link MIN_ENCRYPTION_SEED_BYTES} bytes). Returns nothing; throws on failure.
 *
 * @throws If `seed` is not a `Uint8Array` or is shorter than the minimum length.
 */
export function assertIsEncryptionSeed(seed: unknown): asserts seed is Bytes {
  if (!(seed instanceof Uint8Array)) {
    throw new Error('Encryption seed must be a Uint8Array');
  }
  if (seed.length < MIN_ENCRYPTION_SEED_BYTES) {
    throw new Error(
      `Encryption seed must be at least ${String(MIN_ENCRYPTION_SEED_BYTES)} bytes, got ${String(seed.length)}`,
    );
  }
}
