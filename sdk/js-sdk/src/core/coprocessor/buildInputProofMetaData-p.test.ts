import { describe, it, expect } from 'vitest';

import { buildInputProofMetaData, isSolanaHostChainId, SOLANA_CHAIN_TYPE_BIT } from './buildInputProofMetaData-p.js';

describe('buildInputProofMetaData', () => {
  it('assembles the 92-byte EVM layout (contract||user||acl||chainId)', () => {
    const meta = buildInputProofMetaData({
      chainId: 1,
      contractAddress: `0x${'11'.repeat(20)}`,
      userAddress: `0x${'22'.repeat(20)}`,
      aclContractAddress: `0x${'33'.repeat(20)}`,
    });

    expect(meta.length).toBe(92);
    expect([...meta.slice(0, 20)]).toEqual(Array(20).fill(0x11));
    expect([...meta.slice(20, 40)]).toEqual(Array(20).fill(0x22));
    expect([...meta.slice(40, 60)]).toEqual(Array(20).fill(0x33));
    expect(meta[91]).toBe(1); // chainId 1, big-endian
  });

  it('assembles the 128-byte Solana layout matching zkproof-worker', () => {
    const chainId = SOLANA_CHAIN_TYPE_BIT | 12345n;
    const meta = buildInputProofMetaData({
      chainId,
      contractAddress: `0x${'11'.repeat(32)}`,
      userAddress: `0x${'22'.repeat(32)}`,
      aclContractAddress: `0x${'33'.repeat(32)}`,
    });

    expect(meta.length).toBe(128);
    expect([...meta.slice(0, 32)]).toEqual(Array(32).fill(0x11));
    expect([...meta.slice(32, 64)]).toEqual(Array(32).fill(0x22));
    expect([...meta.slice(64, 96)]).toEqual(Array(32).fill(0x33));
    // chainId word (bytes 96..128) is a 32-byte big-endian uint; the u64 occupies
    // its low 8 bytes (96+24..96+32), so the chain-type high bit lands at byte 120
    // and the logical id 0x3039 (12345) at the final two bytes.
    expect(meta[96]).toBe(0x00);
    expect(meta[120]).toBe(0x80);
    expect(meta[126]).toBe(0x30);
    expect(meta[127]).toBe(0x39);
  });

  it('rejects a 20-byte address on the Solana path', () => {
    expect(() =>
      buildInputProofMetaData({
        chainId: SOLANA_CHAIN_TYPE_BIT | 1n,
        contractAddress: `0x${'11'.repeat(20)}`, // not bytes32
        userAddress: `0x${'22'.repeat(32)}`,
        aclContractAddress: `0x${'33'.repeat(32)}`,
      }),
    ).toThrow();
  });

  it('isSolanaHostChainId reads the chain-type high bit', () => {
    expect(isSolanaHostChainId(12345)).toBe(false);
    expect(isSolanaHostChainId(SOLANA_CHAIN_TYPE_BIT | 12345n)).toBe(true);
  });
});
