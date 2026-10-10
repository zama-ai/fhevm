import { describe, expect, it, vi } from 'vitest';

const fetchEncodedAccount = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  fetchEncodedAccount: (...args: unknown[]) => fetchEncodedAccount(...args),
}));

import { address, getBase58Encoder, getBase64Decoder, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import { getBatchEncoder } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { getJoinRecordEncoder } from './internal/generated/confidentialBatcher/accounts/joinRecord.js';
import { getBatchJoinRecords, getJoinRecord } from './reads.js';

function addr(fill: number): Address {
  return address(base58.encode(new Uint8Array(32).fill(fill)));
}




describe('getJoinRecord', () => {
  it('decodes the record fields using the RPC defaults', async () => {
    const data = getJoinRecordEncoder().encode({
      batch: addr(4),
      user: addr(100),
      claimed: true,
      bump: 253,
    });
    fetchEncodedAccount.mockResolvedValue({ exists: true, address: addr(9), data });

    const record = await getJoinRecord({} as never, addr(9));
    expect(record.batch).toBe(addr(4));
    expect(record.user).toBe(addr(100));
    expect('joinedEncryptedValue' in record).toBe(false);
    expect(record.claimed).toBe(true);
    expect(fetchEncodedAccount).toHaveBeenLastCalledWith({}, addr(9), undefined);
  });

  it('throws when the user never joined the batch (no record)', async () => {
    fetchEncodedAccount.mockResolvedValue({ exists: false, address: addr(9) });
    await expect(getJoinRecord({} as never, addr(9))).rejects.toThrow();
  });
});

type Filter =
  | { readonly dataSize: bigint }
  | { readonly memcmp: { readonly offset: bigint; readonly bytes: string; readonly encoding: 'base58' } };

/** A program-accounts RPC that applies the request's filters to `accounts`, as a validator does. */
const programAccountsRpc = (accounts: readonly Uint8Array[]) => ({
  getProgramAccounts: (_program: Address, config: { readonly filters: readonly Filter[] }) => ({
    send: async () =>
      accounts
        .filter((data) =>
          config.filters.every((filter) => {
            if ('dataSize' in filter) return BigInt(data.length) === filter.dataSize;
            const bytes = getBase58Encoder().encode(filter.memcmp.bytes);
            const offset = Number(filter.memcmp.offset);
            return bytes.every((byte, index) => data[offset + index] === byte);
          }),
        )
        .map((data) => ({ account: { data: [getBase64Decoder().decode(data), 'base64'] } })),
  }),
});

describe('getBatchJoinRecords', () => {
  it("returns only the batch's join records", async () => {
    const record = (batch: Address, user: Address) =>
      new Uint8Array(getJoinRecordEncoder().encode({ batch, user, claimed: false, bump: 255 }));
    const otherAccount = new Uint8Array(
      getBatchEncoder().encode({
        batcher: addr(4),
        index: 0n,
        status: 0,
        openedAt: 0n,
        dispatchedAt: 0n,
        joinCount: 0n,
        totalJoined: 0n,
        payoutReceived: 0n,
        payoutRate: 0n,
        burnedTotalHandle: new Uint8Array(32),
        authorityBump: 0,
        bump: 0,
      }),
    );
    const rpc = programAccountsRpc([record(addr(4), addr(100)), record(addr(5), addr(101)), record(addr(4), addr(102)), otherAccount]);

    const records = await getBatchJoinRecords(rpc as never, addr(4));

    expect(records.map(({ user }) => user)).toEqual([addr(100), addr(102)]);
  });
});
