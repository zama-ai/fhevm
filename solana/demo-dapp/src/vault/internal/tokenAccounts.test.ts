import { describe, expect, it } from 'vitest';
import { address, getProgramDerivedAddress, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import {
  CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from './generated/confidentialToken/programAddress.js';
import { tokenStateAddress } from './tokenAccounts.js';

const utf8 = (value: string): Uint8Array => new TextEncoder().encode(value);
const addr = (fill: number): Address => address(base58.encode(new Uint8Array(32).fill(fill)));

describe('tokenStateAddress', () => {
  // The state identity does not contain a slot key: balance and burned amount share it.
  it('is the host PDA of (token program, token account, mint)', async () => {
    const mint = addr(3);
    const tokenAccount = addr(4);
    const [expected] = await getProgramDerivedAddress({
      programAddress: ZAMA_HOST_PROGRAM_ADDRESS,
      seeds: [
        utf8('encrypted-state'),
        base58.decode(CONFIDENTIAL_TOKEN_PROGRAM_ADDRESS),
        base58.decode(tokenAccount),
        base58.decode(mint),
      ],
    });
    expect(await tokenStateAddress(mint, tokenAccount)).toBe(expected);
  });
});
