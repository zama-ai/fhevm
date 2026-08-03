import { describe, expect, it } from 'vitest';
import { AccountRole, address, type Address, type TransactionSigner } from '@solana/kit';
import { base58 } from '@scure/base';

import {
  ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS,
  LOOKUP_TABLE_STILL_ACTIVE,
  MAX_EXTEND_ADDRESSES_PER_TRANSACTION,
  decodeLookupTableDeactivationSlot,
  deriveAddressLookupTableAddress,
  getCloseLookupTableInstruction,
  getCreateLookupTableInstruction,
  getDeactivateLookupTableInstruction,
  getExtendLookupTableInstruction,
  getExtendLookupTableInstructions,
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

  it('refuses a single extend that cannot fit the transaction wire limit', () => {
    const authority = signer(addr(1));
    const addresses = Array.from({ length: MAX_EXTEND_ADDRESSES_PER_TRANSACTION + 1 }, (_, i) => addr(i + 1));
    expect(() =>
      getExtendLookupTableInstruction({ lookupTable: addr(2), authority, payer: authority, addresses }),
    ).toThrow('cannot fit');
  });

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
    const counts = instructions.map((instruction) =>
      new DataView(instruction.data!.buffer, instruction.data!.byteOffset).getBigUint64(4, true),
    );
    expect(counts).toEqual([BigInt(MAX_EXTEND_ADDRESSES_PER_TRANSACTION), 12n]);
    // Order carries across the chunk boundary: the last address of chunk 0 is followed by the
    // next address as chunk 1's first entry.
    const first = instructions[0]!.data!;
    const boundary = 12 + (MAX_EXTEND_ADDRESSES_PER_TRANSACTION - 1) * 32;
    expect(Array.from(first.subarray(boundary, boundary + 32))).toEqual(
      Array.from(base58.decode(addr(MAX_EXTEND_ADDRESSES_PER_TRANSACTION))),
    );
    expect(Array.from(instructions[1]!.data!.subarray(12, 44))).toEqual(
      Array.from(base58.decode(addr(MAX_EXTEND_ADDRESSES_PER_TRANSACTION + 1))),
    );
  });

  it('encodes DeactivateLookupTable with the table and its signing authority only', () => {
    const authority = signer(addr(1));
    const instruction = getDeactivateLookupTableInstruction({ lookupTable: addr(2), authority });
    expect(instruction.programAddress).toBe(ADDRESS_LOOKUP_TABLE_PROGRAM_ADDRESS);
    expect(instruction.data).toHaveLength(4);
    const view = new DataView(instruction.data!.buffer, instruction.data!.byteOffset);
    expect(view.getUint32(0, true)).toBe(3); // DeactivateLookupTable discriminant
    expect(instruction.accounts!.map((a) => a.role)).toEqual([AccountRole.WRITABLE, AccountRole.READONLY_SIGNER]);
  });

  it('encodes CloseLookupTable refunding rent to the recipient', () => {
    const authority = signer(addr(1));
    const instruction = getCloseLookupTableInstruction({ lookupTable: addr(2), authority, recipient: addr(3) });
    expect(instruction.data).toHaveLength(4);
    const view = new DataView(instruction.data!.buffer, instruction.data!.byteOffset);
    expect(view.getUint32(0, true)).toBe(4); // CloseLookupTable discriminant
    expect(instruction.accounts!.map((a) => a.address)).toEqual([addr(2), addr(1), addr(3)]);
    expect(instruction.accounts![2]!.role).toBe(AccountRole.WRITABLE); // rent recipient is credited
  });

  it('reads deactivation_slot out of raw lookup-table account data', () => {
    const data = new Uint8Array(56);
    const view = new DataView(data.buffer);
    view.setBigUint64(4, LOOKUP_TABLE_STILL_ACTIVE, true);
    expect(decodeLookupTableDeactivationSlot(data)).toBe(LOOKUP_TABLE_STILL_ACTIVE);
    view.setBigUint64(4, 12_345n, true);
    expect(decodeLookupTableDeactivationSlot(data)).toBe(12_345n);
    expect(() => decodeLookupTableDeactivationSlot(new Uint8Array(8))).toThrow('too short');
  });
});
