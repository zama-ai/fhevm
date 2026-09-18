import { createSolanaRpc } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { describe, expect, it } from 'vitest';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';

const rpc = createSolanaRpc('http://localhost:8899');

const chain = {
  id: 72057594037940281n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
  },
} as const satisfies FhevmSolanaChain;

describe('createFhevmBaseClient', () => {
  it('keeps the exact Solana chain while leaving EVM chain and native client absent', async () => {
    setFhevmRuntimeConfig({});

    const client = createFhevmBaseClient({ rpc, chain });

    expect(client.chain).toBe(chain);
    expect(client.rpc).toBe(rpc);
    for (const member of ['ethereum', 'runtime', 'protocolVersion', 'options', 'extend'])
      expect(member in client).toBe(false);
    await expect(client.ready).resolves.toBeUndefined();
  });

  it.each([0n, 12345n, 1n << 63n, 1n << 64n])('rejects invalid Solana chain id %s', (id) => {
    expect(() => createFhevmBaseClient({ rpc, chain: { ...chain, id } })).toThrow(
      'Solana chain id must be a u64 bigint with type byte 0x01',
    );
  });
});
