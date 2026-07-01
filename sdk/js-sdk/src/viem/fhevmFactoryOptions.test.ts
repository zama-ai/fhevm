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

export const baseAcceptsBoth: BaseOptions = { moduleVersions: { tfhe: '1.6.1', kms: '0.13.20-0' } };
export const clientAcceptsBoth: Options = { moduleVersions: { tfhe: '1.6.1', kms: '0.13.20-0' } };
export const encryptAcceptsTfhe: EncryptOptions = { moduleVersions: { tfhe: '1.6.1' } };
export const decryptAcceptsKms: DecryptOptions = { moduleVersions: { kms: '0.13.20-0' } };

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

export const cleartextEncryptAcceptsTfhe: CleartextEncryptOptions = { moduleVersions: { tfhe: '1.6.1' } };
export const cleartextDecryptAcceptsKms: CleartextDecryptOptions = { moduleVersions: { kms: '0.13.20-0' } };

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
