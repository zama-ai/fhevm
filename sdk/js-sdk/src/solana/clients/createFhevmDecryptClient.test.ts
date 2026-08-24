// The decrypt client's action surface, pinned.
//
// One absence here is load-bearing: the client must not offer `generateTransportKeyPair`. That
// action produced a pair from the core EVM-generation TKMS blob, and a permit built over such a pair
// is broken twice — the permit commits to the 869-byte MlKem512 container only the Solana blob
// serializes, and response verification checks wasm class identity, so the pair fails there even
// when its width happens to fit. A generator whose output the permit path cannot consume must not be
// reachable from the Solana surface at all; the permit path's own generator is
// `generateSolanaTransportKeyPair` in `solana/userDecrypt`.

import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { describe, expect, it } from 'vitest';
import { asBytes32Hex } from '../../core/base/bytes.js';
import { createFhevmDecryptClient } from './createFhevmDecryptClient.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    acl: { domainKeys: [asBytes32Hex('0x1111111111111111111111111111111111111111111111111111111111111111')] },
  },
} as const satisfies FhevmSolanaChain;

describe('createFhevmDecryptClient', () => {
  it('offers the public-decrypt set, and no EVM-blob transport key generator', async () => {
    setFhevmRuntimeConfig({});

    const client = createFhevmDecryptClient({ chain });

    expect(client.publicDecryptCertificate).toBeTypeOf('function');
    expect('generateTransportKeyPair' in client).toBe(false);
    await expect(client.ready).resolves.toBeUndefined();
  });
});
