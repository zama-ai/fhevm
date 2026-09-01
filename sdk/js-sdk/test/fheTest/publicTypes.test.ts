import type {
  FheEncryptionCrsBytes,
  FheEncryptionKeyBytes,
  FheEncryptionKeyDigests,
  FheEncryptionKeyMetadata,
  FheEncryptionKeyTrust,
  FheEncryptionPublicKeyBytes,
} from '@fhevm/sdk/types';
import type { createFhevmEncryptClient } from '@fhevm/sdk/viem';
import { describe, expect, it } from 'vitest';

type EncryptOptions = NonNullable<Parameters<typeof createFhevmEncryptClient>[0]['options']>;

const metadata: FheEncryptionKeyMetadata = {
  chainId: 1,
  relayerUrl: 'https://relayer.example',
};
const publicKeyBytes: FheEncryptionPublicKeyBytes = {
  id: 'public-key',
  bytes: new Uint8Array([1]),
};
const crsBytes: FheEncryptionCrsBytes = {
  id: 'crs',
  capacity: 2048,
  bytes: new Uint8Array([2]),
};
const key: FheEncryptionKeyBytes = {
  publicKeyBytes,
  crsBytes,
  metadata,
};
const digests: FheEncryptionKeyDigests = {
  publicKeyDigest: `0x${'11'.repeat(32)}`,
  crsDigest: `0x${'22'.repeat(32)}`,
};
const trust: FheEncryptionKeyTrust = (_metadata) => digests;
const pinnedOptions: EncryptOptions = { fheEncryptionKey: key, fheEncryptionKeyTrust: trust };

describe('@fhevm/sdk/types encryption-key exports', () => {
  it('type-checks every encryption-key DTO through the external entrypoint', () => {
    expect(pinnedOptions.fheEncryptionKey?.metadata).toEqual(metadata);
    expect(publicKeyBytes.id).toBe('public-key');
    expect(crsBytes.capacity).toBe(2048);
  });
});
