import type { FhevmSolanaChain } from '../types/fhevmSolanaChain.js';
import { describe, expect, it } from 'vitest';
import { defineFhevmSolanaChain } from './utilsSolana.js';
import { asBytes32Hex } from '../base/bytes.js';

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    programs: { host: { address: asBytes32Hex(`0x${'22'.repeat(32)}`) } },
  },
} as const satisfies FhevmSolanaChain;

describe('defineFhevmSolanaChain', () => {
  it('preserves an exact high-bit bigint chain id', () => {
    expect(defineFhevmSolanaChain(chain).id).toBe(9223372036854788153n);
  });

  it.each([0n, 12345n, 1n << 64n])('rejects invalid Solana chain id %s', (id) => {
    expect(() => defineFhevmSolanaChain({ ...chain, id })).toThrow(
      'Solana chain id must be a u64 bigint with bit 63 set',
    );
  });
});
