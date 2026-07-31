import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address } from '@solana/kit';
import { base58 } from '@scure/base';
import { sha256 } from '@noble/hashes/sha2.js';

import { ZAMA_HOST_PROGRAM_ADDRESS } from '../../internal/generated/confidentialToken/programAddress.js';
import { confidentialBalanceValueAccount } from './tokenValueAccount.js';

const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const concat = (...parts: Uint8Array[]): Uint8Array => {
  const result = new Uint8Array(parts.reduce((size, part) => size + part.length, 0));
  let offset = 0;
  for (const part of parts) {
    result.set(part, offset);
    offset += part.length;
  }
  return result;
};
const addr = (fill: number): Address => address(base58.encode(new Uint8Array(32).fill(fill)));

describe('confidentialBalanceValueAccount', () => {
  it('derives the canonical encrypted value ID and host PDA for the balance label', async () => {
    const mint = addr(3);
    const tokenAccount = addr(4);
    const expectedKey = sha256(
      concat(
        utf8('zama-encrypted-value-key-v1'),
        base58.decode(mint),
        base58.decode(tokenAccount),
        utf8('balance_________________________'),
      ),
    );
    const expectedAddress = (
      await getProgramDerivedAddress({
        programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
        seeds: [utf8('encrypted-value'), expectedKey],
      })
    )[0];

    const actual = await confidentialBalanceValueAccount(mint, tokenAccount);
    expect(Array.from(actual.aclValueKey)).toEqual(Array.from(expectedKey));
    expect(actual.encryptedValueAddress).toBe(expectedAddress);
  });
});
