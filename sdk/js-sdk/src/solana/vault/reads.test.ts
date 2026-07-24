import { describe, expect, it, vi } from 'vitest';

const fetchEncodedAccount = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  fetchEncodedAccount: (...args: unknown[]) => fetchEncodedAccount(...args),
}));

import { address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import { getJoinRecordEncoder } from './internal/generated/confidentialBatcher/accounts/joinRecord.js';
import { getEncryptedValueState, getJoinRecord } from './reads.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}

function u32le(value: number): Uint8Array {
  const bytes = new Uint8Array(4);
  new DataView(bytes.buffer).setUint32(0, value, true);
  return bytes;
}

function u64le(value: bigint): Uint8Array {
  const bytes = new Uint8Array(8);
  new DataView(bytes.buffer).setBigUint64(0, value, true);
  return bytes;
}

function concat(...parts: Uint8Array[]): Uint8Array {
  const out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0));
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
}

/** Builds an `EncryptedValue` account image: 8-byte discriminator + borsh body, mirroring the crate. */
function encryptedValueAccount(input: {
  currentHandle: Uint8Array;
  subjects: Uint8Array[];
  leafCount: bigint;
  peaks: Uint8Array[];
  bump: number;
}): Uint8Array {
  return concat(
    new Uint8Array(8).fill(0xaa), // discriminator (skipped by the reader)
    new Uint8Array(32).fill(0x01), // acl_domain_key
    new Uint8Array(32).fill(0x02), // app_account
    new Uint8Array(32).fill(0x03), // encrypted_value_label
    input.currentHandle,
    u32le(input.subjects.length),
    ...input.subjects,
    u64le(input.leafCount),
    u32le(input.peaks.length),
    ...input.peaks,
    new Uint8Array([input.bump]),
  );
}

describe('getEncryptedValueState', () => {
  it('decodes handle, leaf count, and peaks from the account borsh body past the discriminator', async () => {
    const currentHandle = new Uint8Array(32).fill(0x77);
    const peakA = new Uint8Array(32).fill(0x0a);
    const peakB = new Uint8Array(32).fill(0x0b);
    const data = encryptedValueAccount({
      currentHandle,
      subjects: [new Uint8Array(32).fill(0x05)], // a subject the reader must skip
      leafCount: 3n,
      peaks: [peakA, peakB],
      bump: 254,
    });
    fetchEncodedAccount.mockResolvedValue({ exists: true, address: addr(9), data });

    const state = await getEncryptedValueState({} as never, addr(9));
    expect(Array.from(state.currentHandle)).toEqual(Array.from(currentHandle));
    expect(state.leafCount).toBe(3n);
    expect(state.peaks.map((p) => Array.from(p))).toEqual([Array.from(peakA), Array.from(peakB)]);
  });

  it('handles an empty value account (no subjects, no peaks)', async () => {
    const data = encryptedValueAccount({
      currentHandle: new Uint8Array(32),
      subjects: [],
      leafCount: 0n,
      peaks: [],
      bump: 255,
    });
    fetchEncodedAccount.mockResolvedValue({ exists: true, address: addr(9), data });

    const state = await getEncryptedValueState({} as never, addr(9));
    expect(state.leafCount).toBe(0n);
    expect(state.peaks).toEqual([]);
  });

  it('throws when the account does not exist', async () => {
    fetchEncodedAccount.mockResolvedValue({ exists: false, address: addr(9) });
    await expect(getEncryptedValueState({} as never, addr(9))).rejects.toThrow('does not exist');
  });
});

describe('getJoinRecord', () => {
  it('decodes the record fields and forwards the fetch config (e.g. commitment)', async () => {
    const data = getJoinRecordEncoder().encode({
      batch: addr(4),
      user: addr(100),
      joinedEncryptedValue: addr(5),
      claimed: true,
      bump: 253,
    });
    fetchEncodedAccount.mockResolvedValue({ exists: true, address: addr(9), data });

    const record = await getJoinRecord({} as never, addr(9), { commitment: 'confirmed' });
    expect(record.batch).toBe(addr(4));
    expect(record.user).toBe(addr(100));
    expect(record.joinedEncryptedValue).toBe(addr(5));
    expect(record.claimed).toBe(true);
    // The commitment rides through to the underlying account fetch — the demo reads a fresh claim
    // at 'confirmed' because finalization lags it by ~31 slots.
    expect(fetchEncodedAccount).toHaveBeenLastCalledWith({}, addr(9), { commitment: 'confirmed' });
  });

  it('throws when the user never joined the batch (no record)', async () => {
    fetchEncodedAccount.mockResolvedValue({ exists: false, address: addr(9) });
    await expect(getJoinRecord({} as never, addr(9))).rejects.toThrow();
  });
});
