import { describe, expect, it } from 'vitest';
import { address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';
import { getExtendLookupTableInstructionDataDecoder } from '@solana-program/address-lookup-table';

import { MAX_EXTEND_ADDRESSES_PER_TRANSACTION, getExtendLookupTableInstructions } from './addressLookupTable.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

describe('lookup table extend chunking', () => {
  it('chunks a full 32-address settle table into sendable extends, preserving order', () => {
    const authority = signer(addr(1));
    const addresses = Array.from({ length: 32 }, (_, i) => addr(i + 1));
    const instructions = getExtendLookupTableInstructions({
      lookupTable: addr(2),
      authority,
      payer: authority,
      addresses,
    });
    expect(instructions).toHaveLength(2);
    const chunks = instructions.map(
      (instruction) => getExtendLookupTableInstructionDataDecoder().decode(instruction.data!).addresses,
    );
    expect(chunks.map((chunk) => chunk.length)).toEqual([MAX_EXTEND_ADDRESSES_PER_TRANSACTION, 12]);
    expect(chunks.flat()).toEqual(addresses);
  });

  it('emits no extend for an empty address set', () => {
    const authority = signer(addr(1));
    expect(getExtendLookupTableInstructions({ lookupTable: addr(2), authority, payer: authority, addresses: [] })).toEqual([]);
  });

  it('retains the authority and payer signers on each chunk', () => {
    const authority = signer(addr(1));
    const payer = signer(addr(2));
    const instructions = getExtendLookupTableInstructions({
      lookupTable: addr(3),
      authority,
      payer,
      addresses: Array.from({ length: MAX_EXTEND_ADDRESSES_PER_TRANSACTION + 1 }, (_, index) => addr(index + 4)),
    });
    for (const instruction of instructions) {
      expect(instruction.accounts![1]).toHaveProperty('signer', authority);
      expect(instruction.accounts![2]).toHaveProperty('signer', payer);
    }
  });
});
