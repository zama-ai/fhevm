import { describe, expect, it } from 'vitest';
import { AccountRole, address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import {
  ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
  deriveAddressLookupTableAddress,
  getCreateLookupTableInstruction,
  getExtendLookupTableInstruction,
} from './addressLookupTable.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function signer(a: Address): TransactionSigner {
  return { address: a, signTransactions: async () => [] } as unknown as TransactionSigner;
}

describe('address lookup table instructions', () => {
  it('encodes CreateLookupTable with the recent slot and derived bump', async () => {
    const authority = signer(addr(1));
    const recentSlot = 123n;
    const { instruction, lookupTableAddress } = await getCreateLookupTableInstruction({
      authority,
      payer: authority,
      recentSlot,
    });
    const derived = await deriveAddressLookupTableAddress(authority.address, recentSlot);
    expect(lookupTableAddress).toBe(derived.address);

    expect(instruction.programAddress).toBe(ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS);
    const data = instruction.data!;
    expect(data).toHaveLength(4 + 8 + 1);
    const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
    expect(view.getUint32(0, true)).toBe(0); // CreateLookupTable discriminant
    expect(view.getBigUint64(4, true)).toBe(recentSlot);
    expect(data[12]).toBe(derived.bump);

    const roles = instruction.accounts!.map((a) => a.role);
    expect(instruction.accounts![0]!.address).toBe(lookupTableAddress);
    expect(roles[0]).toBe(AccountRole.WRITABLE); // uninitialized table
    expect(roles[1]).toBe(AccountRole.READONLY_SIGNER); // authority
    expect(roles[2]).toBe(AccountRole.WRITABLE_SIGNER); // payer
  });

  it('encodes ExtendLookupTable with a length-prefixed address vector', () => {
    const authority = signer(addr(1));
    const addresses = [addr(9), addr(10), addr(11)];
    const instruction = getExtendLookupTableInstruction({
      lookupTable: addr(2),
      authority,
      payer: authority,
      addresses,
    });
    const data = instruction.data!;
    const view = new DataView(data.buffer, data.byteOffset, data.byteLength);
    expect(view.getUint32(0, true)).toBe(2); // ExtendLookupTable discriminant
    expect(view.getBigUint64(4, true)).toBe(3n); // vec length
    expect(data).toHaveLength(4 + 8 + 3 * 32);
    // First appended address round-trips.
    expect(base58.encode(data.slice(12, 44))).toBe(addresses[0]);
  });

  it('rejects an empty extend', () => {
    const authority = signer(addr(1));
    expect(() =>
      getExtendLookupTableInstruction({ lookupTable: addr(2), authority, payer: authority, addresses: [] }),
    ).toThrow('at least one address');
  });
});
