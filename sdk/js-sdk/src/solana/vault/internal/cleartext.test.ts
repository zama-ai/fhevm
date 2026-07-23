import { describe, expect, it } from 'vitest';

import { settleTotalFromCleartext } from './cleartext.js';

function uint256BE(low: bigint, highByte0 = 0): Uint8Array {
  const out = new Uint8Array(32);
  out[0] = highByte0;
  const view = new DataView(out.buffer);
  view.setBigUint64(24, low, false); // low 8 bytes, big-endian
  return out;
}

describe('settleTotalFromCleartext', () => {
  it('extracts the low 8 bytes big-endian', () => {
    expect(settleTotalFromCleartext(uint256BE(0n))).toBe(0n);
    expect(settleTotalFromCleartext(uint256BE(42n))).toBe(42n);
    expect(settleTotalFromCleartext(uint256BE(800n))).toBe(800n);
    expect(settleTotalFromCleartext(uint256BE(0xffffffffffffffffn))).toBe(0xffffffffffffffffn);
  });

  it('reads big-endian, not little-endian', () => {
    const bytes = new Uint8Array(32);
    bytes[31] = 0x01; // big-endian value 1
    expect(settleTotalFromCleartext(bytes)).toBe(1n);
    const other = new Uint8Array(32);
    other[24] = 0x01; // most-significant of the low 8 bytes → 2^56
    expect(settleTotalFromCleartext(other)).toBe(1n << 56n);
  });

  it('rejects a total that does not fit u64 (any high byte non-zero)', () => {
    expect(() => settleTotalFromCleartext(uint256BE(1n, 0x01))).toThrow('exceeds u64');
    const justAboveU64 = new Uint8Array(32);
    justAboveU64[23] = 0x01; // byte just above the low 8 → 2^64
    expect(() => settleTotalFromCleartext(justAboveU64)).toThrow('exceeds u64');
  });

  it('rejects a non-32-byte cleartext', () => {
    expect(() => settleTotalFromCleartext(new Uint8Array(8))).toThrow('32-byte');
  });
});
