import { describe, expect, it, vi } from 'vitest';

const fetchEncodedAccount = vi.hoisted(() => vi.fn());
const fetchEncodedAccounts = vi.hoisted(() => vi.fn());
vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  fetchEncodedAccount: (...args: unknown[]) => fetchEncodedAccount(...args),
  fetchEncodedAccounts: (...args: unknown[]) => fetchEncodedAccounts(...args),
}));

import { address, getBase58Encoder, getBase64Decoder, type Address } from '@solana/kit';
import { base58 } from '@scure/base';

import { getBatchEncoder } from './internal/generated/confidentialBatcher/accounts/batch.js';
import { getBatcherEncoder } from './internal/generated/confidentialBatcher/accounts/batcher.js';
import { getJoinRecordEncoder } from './internal/generated/confidentialBatcher/accounts/joinRecord.js';
import { findBatchPda } from './internal/generated/confidentialBatcher/pdas/batch.js';
import { findJoinRecordPda } from './internal/generated/confidentialBatcher/pdas/joinRecord.js';
import type { VaultDemoRoots } from './derive.js';
import {
  findRecentJoin,
  getBatchJoinRecords,
  getJoinRecord,
  RECENT_BATCH_SCAN_LIMIT,
} from './reads.js';

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
        dispatchableAt: 0n,
        settleDeadlineSecs: 0n,
        settleDeadline: 0n,
        joinCount: 0n,
        totalJoined: 0n,
        payoutReceived: 0n,
        burnedTotalHandle: new Uint8Array(32),
        authorityBump: 0,
      }),
    );
    const rpc = programAccountsRpc([record(addr(4), addr(100)), record(addr(5), addr(101)), record(addr(4), addr(102)), otherAccount]);

    const records = await getBatchJoinRecords(rpc as never, addr(4));

    expect(records.map(({ user }) => user)).toEqual([addr(100), addr(102)]);
  });
});

describe('findRecentJoin', () => {
  const roots = { batcher: addr(2) } as VaultDemoRoots;
  const user = addr(100);
  const batchAt = async (index: bigint) => (await findBatchPda({ batcher: roots.batcher, index }))[0];
  const recordIn = async (index: bigint) => (await findJoinRecordPda({ batch: await batchAt(index), user }))[0];

  /** A batcher whose pending batch is `pending`, and a user who joined the batches at `joined`. */
  const chain = async (pending: bigint, joined: readonly bigint[]) => {
    const batcher = getBatcherEncoder().encode({
      direction: 0,
      joinConfidentialMint: addr(4),
      payoutConfidentialMint: addr(13),
      vault: addr(10),
      admin: addr(3),
      paused: false,
      minBatchAgeSecs: 0n,
      settleDeadlineSecs: 60n,
      nextBatchIndex: pending + 1n,
    });
    fetchEncodedAccount.mockResolvedValue({ exists: true, address: roots.batcher, data: batcher });
    const records = new Map(
      await Promise.all(
        joined.map(async (index) => [await recordIn(index), await batchAt(index)] as const),
      ),
    );
    fetchEncodedAccounts.mockImplementation(async (_rpc: unknown, addresses: Address[]) =>
      addresses.map((address) => {
        const batch = records.get(address);
        return batch === undefined
          ? { exists: false, address }
          : { exists: true, address, data: getJoinRecordEncoder().encode({ batch, user, claimed: false, bump: 255 }) };
      }),
    );
  };
  const scanned = (): Address[] => fetchEncodedAccounts.mock.lastCall![1];

  it('finds the newest join, including one in a batch already dispatched', async () => {
    await chain(4n, [1n, 3n]);
    await expect(findRecentJoin({} as never, roots, user)).resolves.toMatchObject({
      batchIndex: 3n,
      batch: await batchAt(3n),
      joinRecord: { user, claimed: false },
    });
    expect(scanned()).toEqual(await Promise.all([4n, 3n, 2n, 1n, 0n].map(recordIn)));
  });

  it("looks in the preferred batch first, unless it is not one of the batcher's", async () => {
    await chain(4n, [1n, 3n]);
    const preferred = { batchIndex: 1n, batch: await batchAt(1n) };
    await expect(findRecentJoin({} as never, roots, user, preferred)).resolves.toMatchObject({ batchIndex: 1n });
    expect(scanned()).toEqual(await Promise.all([1n, 4n, 3n, 2n, 0n].map(recordIn)));
    await expect(
      findRecentJoin({} as never, roots, user, { batchIndex: 1n, batch: await batchAt(2n) }),
    ).resolves.toMatchObject({ batchIndex: 3n });
  });

  it('looks back at most RECENT_BATCH_SCAN_LIMIT batches', async () => {
    const pending = RECENT_BATCH_SCAN_LIMIT + 5n;
    await chain(pending, [5n]);
    await expect(findRecentJoin({} as never, roots, user)).resolves.toBe(null);
    expect(scanned()).toHaveLength(Number(RECENT_BATCH_SCAN_LIMIT));
    expect(scanned().at(-1)).toBe(await recordIn(6n));
  });
});
