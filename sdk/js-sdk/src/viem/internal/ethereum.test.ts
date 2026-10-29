import { describe, expect, it } from 'vitest';
import { decode as ethersDecode, encode as ethersEncode } from '../../ethers/internal/ethereum.js';
import { decode, encode } from './ethereum.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/viem/internal/ethereum.test.ts
//
// The viem adapter's encode/decode accept the same Solidity type strings as the ethers adapter's.
////////////////////////////////////////////////////////////////////////////////

const BYTES32 = `0x${'11'.repeat(32)}`;

const CASES: ReadonlyArray<{ name: string; types: readonly string[]; values: readonly unknown[] }> = [
  { name: 'elementary types', types: ['uint256', 'address', 'bytes'], values: [7n, `0x${'ab'.repeat(20)}`, '0x00aa'] },
  { name: 'an array', types: ['bytes32[]'], values: [[BYTES32, BYTES32]] },
  {
    name: 'a tuple array inside a tuple',
    types: ['((bytes32,bytes,uint32)[])'],
    values: [[[[BYTES32, '0x00aabb', 3]]]],
  },
  { name: 'no types', types: [], values: [] },
];

describe('viem ethereum adapter: encode/decode', () => {
  for (const { name, types, values } of CASES) {
    it(`encodes ${name} exactly as the ethers adapter does`, () => {
      expect(encode({ types, values })).toBe(ethersEncode({ types, values }));
    });

    it(`decodes ${name} as the ethers adapter does`, () => {
      const encodedData = ethersEncode({ types, values });
      expect(_normalize(decode({ types, encodedData }))).toEqual(_normalize(ethersDecode({ types, encodedData })));
    });
  }

  it('rejects an invalid type', () => {
    expect(() => encode({ types: ['uint7'], values: [1n] })).toThrow();
  });
});

/** ethers returns `Result`s and bigints, viem plain arrays, numbers and checksummed addresses. */
function _normalize(value: unknown): unknown {
  if (Array.isArray(value)) {
    return Array.from(value, _normalize);
  }
  if (typeof value === 'number') {
    return BigInt(value);
  }
  return typeof value === 'string' ? value.toLowerCase() : value;
}
