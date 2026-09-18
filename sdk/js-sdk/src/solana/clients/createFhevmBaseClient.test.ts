import { createSolanaRpc } from '@solana/kit';
import type { FhevmSolanaChain } from '../../core/types/fhevmSolanaChain.js';
import { describe, expect, it } from 'vitest';
import { setFhevmRuntimeConfig } from '../internal/config.js';
import { createFhevmBaseClient } from './createFhevmBaseClient.js';
import { asBytes32Hex } from '../../core/base/bytes.js';

const rpc = createSolanaRpc('http://localhost:8899');

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
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

  it.each([0n, 12345n, 1n << 64n])('rejects invalid Solana chain id %s', (id) => {
    expect(() => createFhevmBaseClient({ rpc, chain: { ...chain, id } })).toThrow(
      'Solana chain id must be a u64 bigint with bit 63 set',
    );
  });
});
