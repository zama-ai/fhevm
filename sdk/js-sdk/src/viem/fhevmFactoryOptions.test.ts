import type { createFhevmCleartextDecryptClient } from './cleartext/createFhevmCleartextDecryptClient.js';
import type { createFhevmCleartextEncryptClient } from './cleartext/createFhevmCleartextEncryptClient.js';
import type { createFhevmBaseClient } from './clients/createFhevmBaseClient.js';
import type { createFhevmClient } from './clients/createFhevmClient.js';
import type { createFhevmDecryptClient } from './clients/createFhevmDecryptClient.js';
import type { createFhevmEncryptClient } from './clients/createFhevmEncryptClient.js';
import { describe, expect, it } from 'vitest';

type BaseOptions = NonNullable<Parameters<typeof createFhevmBaseClient>[0]['options']>;
type Options = NonNullable<Parameters<typeof createFhevmClient>[0]['options']>;
type EncryptOptions = NonNullable<Parameters<typeof createFhevmEncryptClient>[0]['options']>;
type DecryptOptions = NonNullable<Parameters<typeof createFhevmDecryptClient>[0]['options']>;
type CleartextEncryptOptions = NonNullable<Parameters<typeof createFhevmCleartextEncryptClient>[0]['options']>;
type CleartextDecryptOptions = NonNullable<Parameters<typeof createFhevmCleartextDecryptClient>[0]['options']>;

export const baseAcceptsBoth: BaseOptions = { moduleVersions: { tfhe: '1.6.2', kms: '0.13.20-0' } };
export const clientAcceptsBoth: Options = { moduleVersions: { tfhe: '1.6.2', kms: '0.13.20-0' } };
export const encryptAcceptsTfhe: EncryptOptions = { moduleVersions: { tfhe: '1.6.2' } };
export const decryptAcceptsKms: DecryptOptions = { moduleVersions: { kms: '0.13.20-0' } };
export const decryptRejectsFheEncryptionKeyTrust: DecryptOptions = {
  // @ts-expect-error Decrypt-only options must not accept FHE encryption-key trust anchors.
  fheEncryptionKeyTrust: {
    publicKeyDigest: '0x452298f972e0848bb511d582524a5c516067ee4c662f33a1ef1110d26d6d0ff1',
    crsDigest: '0x1ee0d74e24ad79124e48c2073599ae15b89b1866cfc20dfa80d15807cee1cc62',
  },
};

export const decryptRejectsFheEncryptionKey: DecryptOptions = {
  // @ts-expect-error Decrypt-only options must not accept caller-pinned FHE encryption keys.
  fheEncryptionKey: undefined,
};

export const encryptRejectsKms: EncryptOptions = {
  moduleVersions: {
    // @ts-expect-error Encrypt-only options must not accept KMS module versions.
    kms: '0.13.20-0',
  },
};

export const decryptRejectsTfhe: DecryptOptions = {
  moduleVersions: {
    // @ts-expect-error Decrypt-only options must not accept TFHE module versions.
    tfhe: '1.6.1',
  },
};

export const cleartextEncryptAcceptsTfhe: CleartextEncryptOptions = { moduleVersions: { tfhe: '1.6.2' } };
export const cleartextDecryptAcceptsKms: CleartextDecryptOptions = { moduleVersions: { kms: '0.13.20-0' } };

export const cleartextEncryptRejectsTrust: CleartextEncryptOptions = {
  // @ts-expect-error Cleartext clients must not accept real-key trust material.
  fheEncryptionKeyTrust: undefined,
};

export const cleartextEncryptRejectsPinnedKey: CleartextEncryptOptions = {
  // @ts-expect-error Cleartext clients must not accept pinned real-key material.
  fheEncryptionKey: undefined,
};

export const cleartextEncryptRejectsKms: CleartextEncryptOptions = {
  moduleVersions: {
    // @ts-expect-error Cleartext encrypt-only options must not accept KMS module versions.
    kms: '0.13.20-0',
  },
};

export const cleartextDecryptRejectsTfhe: CleartextDecryptOptions = {
  moduleVersions: {
    // @ts-expect-error Cleartext decrypt-only options must not accept TFHE module versions.
    tfhe: '1.6.1',
  },
};

describe('viem fhevm factory option types', () => {
  it('keeps compile-time assertions active', () => {
    expect(true).toBe(true);
  });
});
