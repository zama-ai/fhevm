import type { FheEncryptionKeyWasm } from '../../types/fheEncryptionKey.js';
import type { BytesHex, UintNumber } from '../../types/primitives.js';
import { describe, it, expect } from 'vitest';
import { bytesToHexLarge } from '../../base/bytes.js';
import { createTypedValue } from '../../base/typedValue.js';
import { buildWithProofPacked, deserializeFheEncryptionPublicKey, deserializeFheEncryptionCrs } from './mock.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/encrypt/mock.seeded.test.ts
//
// Exercises the seeded-encryption branch threaded through the (cleartext) mock
// module: gating, validation, and the core determinism contract — same seed +
// inputs => identical bytes; no seed => non-deterministic.
////////////////////////////////////////////////////////////////////////////////

const TFHE_VERSION = '1.6.1' as const;

async function makeKey(): Promise<FheEncryptionKeyWasm> {
  const publicKey = await deserializeFheEncryptionPublicKey({
    publicKeyBytes: { id: 'pk', bytes: new Uint8Array([1, 2, 3]) },
    tfheVersion: TFHE_VERSION,
  });
  const crs = await deserializeFheEncryptionCrs({
    crsBytes: { id: 'crs', capacity: 2048 as UintNumber, bytes: new Uint8Array([4, 5, 6]) },
    tfheVersion: TFHE_VERSION,
  });
  return { publicKey, crs, metadata: { relayerUrl: 'http://mock', chainId: 1234 }, tfheVersion: TFHE_VERSION };
}

function commonParams(fheEncryptionKey: FheEncryptionKeyWasm) {
  return {
    fheEncryptionKey,
    typedValues: [createTypedValue({ type: 'uint64', value: 42n }), createTypedValue({ type: 'bool', value: true })],
    metaData: new Uint8Array(92).fill(7),
    extraData: '0x00' as BytesHex,
    tfheVersion: TFHE_VERSION,
  };
}

describe('mock seeded encryption', () => {
  it('is deterministic: same seed + inputs => identical ciphertext bytes', async () => {
    const key = await makeKey();
    const seed = new Uint8Array(32).fill(9);

    const a = await buildWithProofPacked({ ...commonParams(key), seed });
    const b = await buildWithProofPacked({ ...commonParams(key), seed });

    expect(bytesToHexLarge(a.ciphertextWithZKProofBytes)).toBe(bytesToHexLarge(b.ciphertextWithZKProofBytes));
  });

  it('different seeds => different ciphertext bytes', async () => {
    const key = await makeKey();

    const a = await buildWithProofPacked({ ...commonParams(key), seed: new Uint8Array(32).fill(1) });
    const b = await buildWithProofPacked({ ...commonParams(key), seed: new Uint8Array(32).fill(2) });

    expect(bytesToHexLarge(a.ciphertextWithZKProofBytes)).not.toBe(bytesToHexLarge(b.ciphertextWithZKProofBytes));
  });

  it('no seed => non-deterministic (random nonce path unchanged)', async () => {
    const key = await makeKey();

    const a = await buildWithProofPacked(commonParams(key));
    const b = await buildWithProofPacked(commonParams(key));

    expect(bytesToHexLarge(a.ciphertextWithZKProofBytes)).not.toBe(bytesToHexLarge(b.ciphertextWithZKProofBytes));
  });

  it('rejects seeds shorter than 16 bytes', async () => {
    const key = await makeKey();
    await expect(buildWithProofPacked({ ...commonParams(key), seed: new Uint8Array(15) })).rejects.toThrow(
      /at least 16 bytes/,
    );
  });

  it('rejects seeded encryption on non-1.6.1 TFHE versions', async () => {
    const key = await makeKey();
    await expect(
      buildWithProofPacked({
        ...commonParams(key),
        tfheVersion: '1.5.3',
        seed: new Uint8Array(32).fill(3),
      }),
    ).rejects.toThrow(/requires TFHE version 1\.6\.1/);
  });
});
