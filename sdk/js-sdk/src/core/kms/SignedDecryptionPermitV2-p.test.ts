import { describe, expect, it } from 'vitest';
import { isSignedDecryptionPermitV2, parseSignedDecryptionPermitV2 } from './SignedDecryptionPermitV2-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/kms/SignedDecryptionPermitV2-p.test.ts
//
// Simple, mock-free unit tests. Signing, chain reads (readCurrentKmsSignersContext)
// and KMS signature verification are exercised elsewhere (integration tests), so
// here we only cover the synchronous, self-contained surface.
////////////////////////////////////////////////////////////////////////////////

describe('isSignedDecryptionPermitV2', () => {
  it('returns false for non-object values', () => {
    expect(isSignedDecryptionPermitV2(undefined)).toBe(false);
    expect(isSignedDecryptionPermitV2(null)).toBe(false);
    expect(isSignedDecryptionPermitV2(0)).toBe(false);
    expect(isSignedDecryptionPermitV2('')).toBe(false);
    expect(isSignedDecryptionPermitV2('permit')).toBe(false);
    expect(isSignedDecryptionPermitV2(true)).toBe(false);
  });

  it('returns false for plain objects and arrays', () => {
    expect(isSignedDecryptionPermitV2({})).toBe(false);
    expect(isSignedDecryptionPermitV2([])).toBe(false);
  });

  it('returns false for an object that merely looks like a signed permit', () => {
    // A permit is only recognized when it is a genuine internal instance
    // (created by the SDK), never a structurally-similar plain object.
    const lookalike = {
      version: 2,
      signature: `0x${'11'.repeat(65)}`,
      signerAddress: '0x1111111111111111111111111111111111111111',
      eip712: { primaryType: 'UserDecryptRequestVerification', message: {} },
    };
    expect(isSignedDecryptionPermitV2(lookalike)).toBe(false);
  });
});

describe('parseSignedDecryptionPermitV2', () => {
  it('rejects an invalid transport key pair before doing anything else', async () => {
    const permit = {
      eip712: { primaryType: 'UserDecryptRequestVerification', message: {} },
      signature: `0x${'11'.repeat(65)}`,
      signerAddress: '0x1111111111111111111111111111111111111111',
    };
    // The first thing parse does is validate the transport key pair; a plain
    // object is not a genuine TransportKeyPair instance, so it must throw
    // without touching the chain.
    await expect(
      parseSignedDecryptionPermitV2({} as never, { transportKeyPair: {} as never, permit, fhevmContext: {} as never }),
    ).rejects.toThrow();
  });
});
