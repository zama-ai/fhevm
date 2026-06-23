import { describe, it, expect } from 'vitest';
import {
  randomBytes,
  generateEncryptionSeed,
  assertIsEncryptionSeed,
  MIN_ENCRYPTION_SEED_BYTES,
  DEFAULT_ENCRYPTION_SEED_BYTES,
} from './random.js';

describe('randomBytes', () => {
  it('returns a Uint8Array of the requested length', () => {
    const b = randomBytes(24);
    expect(b).toBeInstanceOf(Uint8Array);
    expect(b.length).toBe(24);
  });

  it('is overwhelmingly unlikely to repeat', () => {
    const a = randomBytes(32);
    const b = randomBytes(32);
    expect(Array.from(a)).not.toEqual(Array.from(b));
  });

  it('rejects non-positive / non-integer lengths', () => {
    expect(() => randomBytes(0)).toThrow();
    expect(() => randomBytes(-1)).toThrow();
    expect(() => randomBytes(1.5)).toThrow();
  });
});

describe('generateEncryptionSeed', () => {
  it('defaults to DEFAULT_ENCRYPTION_SEED_BYTES', () => {
    expect(generateEncryptionSeed().length).toBe(DEFAULT_ENCRYPTION_SEED_BYTES);
  });

  it('accepts a custom length at or above the minimum', () => {
    expect(generateEncryptionSeed(MIN_ENCRYPTION_SEED_BYTES).length).toBe(MIN_ENCRYPTION_SEED_BYTES);
  });

  it('rejects lengths below the minimum', () => {
    expect(() => generateEncryptionSeed(MIN_ENCRYPTION_SEED_BYTES - 1)).toThrow();
  });
});

describe('assertIsEncryptionSeed', () => {
  it('passes for a Uint8Array of at least the minimum length', () => {
    expect(() => assertIsEncryptionSeed(new Uint8Array(MIN_ENCRYPTION_SEED_BYTES))).not.toThrow();
  });

  it('throws for too-short seeds', () => {
    expect(() => assertIsEncryptionSeed(new Uint8Array(MIN_ENCRYPTION_SEED_BYTES - 1))).toThrow();
  });

  it('throws for non-Uint8Array input', () => {
    expect(() => assertIsEncryptionSeed('not-bytes')).toThrow();
    expect(() => assertIsEncryptionSeed(undefined)).toThrow();
  });
});
