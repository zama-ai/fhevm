import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { describe, expect, it } from 'vitest';
import { asBytes32Hex } from '../../core/base/bytes.js';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    acl: { domainKeys: [asBytes32Hex('0x1111111111111111111111111111111111111111111111111111111111111111')] },
  },
} as const satisfies FhevmSolanaChain;

describe('createFhevmBaseClient', () => {
  it('keeps the exact Solana chain while leaving EVM chain and native client absent', async () => {
    setFhevmRuntimeConfig({});

    const client = createFhevmBaseClient({ chain });

    expect(client.chain).toBeUndefined();
    expect(client.client).toBeUndefined();
    expect(client.solanaChain).toBe(chain);
    expect(client.solanaChain.id).toBe(9223372036854788153n);
    await expect(client.ready).resolves.toBeUndefined();
  });

  it.each([0n, 12345n, 1n << 64n])('rejects invalid Solana chain id %s', (id) => {
    expect(() => createFhevmBaseClient({ chain: { ...chain, id } })).toThrow(
      'Solana chain id must be a u64 bigint with bit 63 set',
    );
  });
});
