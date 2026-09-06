import { describe, expect, it, vi } from 'vitest';

const fetchEncodedAccount = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  fetchEncodedAccount: (...args: unknown[]) => fetchEncodedAccount(...args),
}));

import { address, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import { getJoinRecordEncoder } from './internal/generated/confidentialBatcher/accounts/joinRecord.js';
import { getJoinRecord } from './reads.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}




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
