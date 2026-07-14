import { describe, expect, it } from 'vitest';
import { asBytes32Hex } from '../base/bytes.js';
import type { FhevmSolanaChain } from '../types/fhevmSolanaChain.js';
import { defineFhevmSolanaChain } from './utilsSolana.js';

const chain = {
  id: 9223372036854788153n,
  fhevm: {
    relayerUrl: 'http://localhost:3000',
    acl: { domainKeys: [asBytes32Hex('0x1111111111111111111111111111111111111111111111111111111111111111')] },
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
