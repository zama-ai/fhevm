import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import { shake256 } from '@noble/hashes/sha3.js';
import { runCodeInIsomorphicWorker } from '../base/isomorphicWorker.js';
import { computeFheEncryptionKeyDigests, type FheEncryptionKeyDigests } from './fheEncryptionKeyDigests-p.js';
import { FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE } from './fheEncryptionKeyDigestsWorkerCode-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/key/fheEncryptionKeyDigests-p.test.ts
////////////////////////////////////////////////////////////////////////////////

// SHAKE256 absorbs 136-byte blocks.
const RATE = 136;
// The ~4.6MB CRS.
const CRS_SIZE = 4_600_000;

function nobleDigest(domain: string, bytes: Uint8Array): Uint8Array {
  return shake256.create({ dkLen: 32 }).update(new TextEncoder().encode(domain)).update(bytes).digest();
}

function bytesOf(length: number, seed: number): Uint8Array {
  const bytes = new Uint8Array(length);
  let x = seed;
  for (let i = 0; i < length; i++) {
    x = (Math.imul(x, 1103515245) + 12345) >>> 0;
    bytes[i] = x >>> 24;
  }
  return bytes;
}

/** Runs the worker code on the calling thread, as the worker does. */
function runWorkerCodeHere(data: {
  readonly publicKey: Uint8Array;
  readonly crs: Uint8Array;
}): FheEncryptionKeyDigests {
  // eslint-disable-next-line @typescript-eslint/no-implied-eval
  return new Function('data', FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE)(data) as FheEncryptionKeyDigests;
}

// The KMS prepends 8 domain bytes: these sizes put domain ‖ bytes on both
// sides of the block boundaries.
const SIZES = [
  0,
  1,
  RATE - 9,
  RATE - 8,
  RATE - 7,
  RATE - 1,
  RATE,
  RATE + 1,
  2 * RATE - 8,
  2 * RATE,
  2 * RATE + 1,
  ...Array.from({ length: 8 }, (_, i) => 1 + ((i * 7919 + 13) % 20_000)),
];

describe('FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE', () => {
  it.each(SIZES)('equals @noble/hashes for %i bytes', (size) => {
    const publicKey = bytesOf(size, size + 1);
    const crs = bytesOf(size, size + 2);

    const expected = { publicKey: nobleDigest('PDAT_KEY', publicKey), crs: nobleDigest('PDAT_CRS', crs) };
    expect(runWorkerCodeHere({ publicKey, crs })).toEqual(expected);
    expect(computeFheEncryptionKeyDigests({ publicKey, crs })).toEqual(expected);
  });

  it('equals @noble/hashes for a CRS-sized buffer', () => {
    const publicKey = bytesOf(33_000, 3);
    const crs = bytesOf(CRS_SIZE, 4);

    const actual = runWorkerCodeHere({ publicKey, crs });
    expect(actual.publicKey).toEqual(nobleDigest('PDAT_KEY', publicKey));
    expect(actual.crs).toEqual(nobleDigest('PDAT_CRS', crs));
  });

  it('returns the same digests in an isomorphic worker, leaving the input attached', async () => {
    const publicKey = bytesOf(33_000, 5);
    const crs = bytesOf(CRS_SIZE, 6);

    const actual = await runCodeInIsomorphicWorker<FheEncryptionKeyDigests>(
      FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE,
      { publicKey, crs },
      5_000,
    );
    expect(actual.publicKey).toBeInstanceOf(Uint8Array);
    expect(actual).toEqual(computeFheEncryptionKeyDigests({ publicKey, crs }));
    expect(crs.byteLength).toBe(CRS_SIZE);
  });

  it('is a self-contained script', () => {
    // Keep in sync with FORBIDDEN_PATTERNS in scripts/build/codegen-key-digests-worker.mjs.
    expect(FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE).not.toMatch(
      /\bimport\s*[\s({*'"]|\bimport\.meta\b|\brequire\s*\(|['"`]node:/,
    );
    expect(FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE).toContain('Copyright (c) 2022 Paul Miller');
  });
});

describe('codegen-key-digests-worker --check', () => {
  const codegen = new URL('../../../scripts/build/codegen-key-digests-worker.mjs', import.meta.url);
  const source = readFileSync(new URL('./fheEncryptionKeyDigestsWorkerCode-p.ts', import.meta.url), 'utf-8');
  const check = async (committed: string): Promise<string | undefined> => {
    const { checkKeyDigestsWorkerCodeSource } = (await import(codegen.href)) as {
      readonly checkKeyDigestsWorkerCodeSource: (source: string) => string | undefined;
    };
    return checkKeyDigestsWorkerCodeSource(committed);
  };

  it('accepts the committed file', async () => {
    await expect(check(source)).resolves.toBeUndefined();
  });

  it('rejects a modified code string', async () => {
    const modified = source.replace('computeFheEncryptionKeyDigests(data);', 'computeFheEncryptionKeyDigests(data );');
    expect(modified).not.toBe(source);
    await expect(check(modified)).resolves.toBe('its code does not match its recorded hash');
  });
});
