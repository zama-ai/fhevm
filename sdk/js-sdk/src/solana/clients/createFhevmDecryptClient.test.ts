import { createSolanaRpc } from '@solana/kit';
// The decrypt client's action surface, pinned.
//
// One absence here is load-bearing: the client must not offer `generateTransportKeyPair`. That core
// action returns the EVM decrypt module's opaque private key, which the permit path cannot consume:
// a permit commits to the serialized public key, and response verification needs the raw key pair.
// `signPermit` makes its own pair with `generateSolanaTransportKeyPair` in `solana/userDecrypt`, so a
// second generator would only offer a key no Solana action accepts.

import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { describe, expect, it } from 'vitest';
import { createFhevmPublicDecryptClient } from './createFhevmPublicDecryptClient.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { asBytes32Hex } from '../../core/base/bytes.js';

const chain = {
  id: 72057594037940281n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
  },
} as const satisfies FhevmSolanaChain;

describe('createFhevmPublicDecryptClient', () => {
  it('offers the public-decrypt set, and no core transport key generator', async () => {
    setFhevmRuntimeConfig({});

    const client = createFhevmPublicDecryptClient({ chain, rpc: createSolanaRpc('http://localhost:8899') });

    expect(client.publicDecryptCertificate).toBeTypeOf('function');
    expect('generateTransportKeyPair' in client).toBe(false);
    await expect(client.ready).resolves.toBeUndefined();
  });
});
