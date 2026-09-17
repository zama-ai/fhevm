import type { FhevmSolanaChain } from '../types/fhevmSolanaChain.js';
import { describe, expect, it } from 'vitest';
import { defineFhevmSolanaChain } from './utilsSolana.js';

const chain = {
  id: 72057594037940281n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
  },
} as const satisfies FhevmSolanaChain;

describe('defineFhevmSolanaChain', () => {
  it('preserves an exact type-byte bigint chain id', () => {
    expect(defineFhevmSolanaChain(chain).id).toBe(72057594037940281n);
  });

  it.each([0n, 12345n, 1n << 63n, 1n << 64n])('rejects invalid Solana chain id %s', (id) => {
    expect(() => defineFhevmSolanaChain({ ...chain, id })).toThrow(
      'Solana chain id must be a u64 bigint with type byte 0x01',
    );
  });
});
