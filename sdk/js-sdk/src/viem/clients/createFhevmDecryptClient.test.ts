import type { FheEncryptionKeyBytes } from '../../core/types/fheEncryptionKey.js';
import { describe, expect, it } from 'vitest';
import { createPublicClient, http } from 'viem';
import { sepolia } from '../../core/chains/definitions/sepolia.js';
import { getProvidedFheEncryptionKey } from '../../core/runtime/CoreFhevm-p.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { createFhevmDecryptClient } from './createFhevmDecryptClient.js';
import { createFhevmEncryptClient } from './createFhevmEncryptClient.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/viem/clients/createFhevmDecryptClient.test.ts
////////////////////////////////////////////////////////////////////////////////

const fheEncryptionKey = {
  publicKeyBytes: { id: 'key', bytes: new Uint8Array([1, 2, 3]) },
  crsBytes: { id: 'crs', capacity: 2048, bytes: new Uint8Array([4, 5, 6]) },
  metadata: { relayerUrl: sepolia.fhevm.relayerUrl, chainId: sepolia.id },
} as unknown as FheEncryptionKeyBytes;

describe('viem createFhevmDecryptClient', () => {
  setFhevmRuntimeConfig({});
  const publicClient = createPublicClient({ transport: http('http://127.0.0.1:1') });
  // One options object for every client, as callers sharing it do.
  const options = { batchRpcCalls: false, fheEncryptionKey };

  it('keeps no copy of options.fheEncryptionKey', () => {
    const client = createFhevmDecryptClient({ publicClient, chain: sepolia, options });
    expect(getProvidedFheEncryptionKey(client)).toBeUndefined();
  });

  it('leaves the base and encrypt clients their copy', () => {
    const base = createFhevmBaseClient({ publicClient, chain: sepolia, options });
    const encrypt = createFhevmEncryptClient({ publicClient, chain: sepolia, options });
    expect(getProvidedFheEncryptionKey(base)).toStrictEqual(fheEncryptionKey);
    expect(getProvidedFheEncryptionKey(encrypt)).toStrictEqual(fheEncryptionKey);
  });
});
