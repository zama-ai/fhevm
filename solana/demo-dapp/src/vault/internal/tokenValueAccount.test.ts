import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import { CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS, ZAMA_HOST_PROGRAM_ADDRESS } from './generated/confidentialToken/programAddress.js';
import { balanceValueAddress } from './tokenValueAccount.js';

const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const addr = (fill: number): Address => address(base58.encode(new Uint8Array(32).fill(fill)));

describe('balanceValueAddress', () => {
  // The seeds the host derives an encrypted value account from, in the crate's order: the tag,
  // the token program, the token account (authority), the mint (scope), the fixed balance label.
  it('is the host PDA of (token program, token account, mint, balance label)', async () => {
    const mint = addr(3);
    const tokenAccount = addr(4);
    const [expected] = await getProgramDerivedAddress({
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
      seeds: [
        utf8('encrypted-value'),
        base58.decode(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
        base58.decode(tokenAccount),
        base58.decode(mint),
        utf8('balance_________________________'),
      ],
    });
    expect(await balanceValueAddress(mint, tokenAccount)).toBe(expected);
  });
});
