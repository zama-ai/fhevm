import { describe, expect, it } from 'vitest';
import { address, getAddressEncoder, getBase58Decoder, type Address } from '@solana/kit';
import type { SolanaRpc } from '../encryptedStore.js';
import { getMakeStoreHandlePublicInstructionDataEncoder } from '../internal/generated/zamaHost/instructions/makeStoreHandlePublic.js';
import { mmrBuildProof, reconstructSolanaStoreHistory, type SolanaStoreHistoryEvent } from '../proof.js';
import { createSolanaLeafRecord } from './leafRecord.js';

const host = address('11111111111111111111111111111112');
const store = address('SysvarC1ock11111111111111111111111111111111');
const other = address('SysvarRent111111111111111111111111111111111');
const storeBytes = new Uint8Array(getAddressEncoder().encode(store));
/** `sha256("account:EncryptedStore")[..8]`. */
const ENCRYPTED_STORE_DISCRIMINATOR = [161, 143, 137, 73, 233, 30, 46, 118];

/** A `make_store_handle_public` that appends leaf `at`, making public the handle filled with `handle`. */
type Write = { readonly at: bigint; readonly handle: number };

const handleOf = (fill: number): Uint8Array => new Uint8Array(32).fill(fill);

const u32 = (value: number): Uint8Array => new Uint8Array(new Uint32Array([value]).buffer);
const u64 = (value: bigint): Uint8Array => new Uint8Array(new BigUint64Array([value]).buffer);

/** The store's leaves in the order the host appended them, and the leaves they commit to. */
const history = (writes: readonly Write[]) => {
  const events = [...writes]
    .sort((left, right) => Number(left.at - right.at))
    .map((write): SolanaStoreHistoryEvent => ({ kind: 'markedPublic', handle: handleOf(write.handle) }));
  return reconstructSolanaStoreHistory(storeBytes, events);
};

/** The borsh bytes of the store account with `leafCount` leaves under `peaks`. */
const storeAccount = (leafCount: bigint, peaks: readonly Uint8Array[]): Uint8Array =>
  new Uint8Array([
    ...ENCRYPTED_STORE_DISCRIMINATOR,
    ...handleOf(0x11), // program
    ...handleOf(0x22), // authority
    ...handleOf(0x33), // scope
    ...u32(0), // slots
    ...u64(leafCount),
    ...u32(peaks.length),
    ...peaks.flatMap((peak) => [...peak]),
    0xfe, // bump
  ]);

/**
 * A validator whose only writes to the store are `write`s, one transaction each, listed newest
 * last. The store account holds the leaves of every write, listed or not.
 */
function ledger(initial: readonly Write[]) {
  const writes = [...initial];
  const hidden = new Set<number>();
  const unavailable = new Set<number>();
  const fetched: string[] = [];
  const account: {
    shape: 'store' | 'missing' | 'foreign' | 'notStore';
    /** The account as of this many writes, as a read that lags the listing sees it. */
    asOf: number | undefined;
    tamper: boolean;
  } = { shape: 'store', asOf: undefined, tamper: false };
  const signatureOf = (index: number): string => `sig${index}`;
  const indexOf = (signature: string): number => Number(signature.slice('sig'.length));

  const transaction = ({ at, handle }: Write) => ({
    transaction: {
      message: {
        header: { numRequiredSignatures: 1, numReadonlySignedAccounts: 0, numReadonlyUnsignedAccounts: 1 },
        accountKeys: [host, other, other, store],
        instructions: [
          {
            programIdIndex: 0,
            // payer, authority, encryptedStore, hostConfig, denyScopeRecord, systemProgram
            accounts: [1, 2, 3, 1, 1, 1],
            data: getBase58Decoder().decode(
              getMakeStoreHandlePublicInstructionDataEncoder().encode({
                key: handleOf(1),
                handle: handleOf(handle),
                previousLeafCount: at,
              }),
            ),
          },
        ],
      },
    },
    meta: {
      err: null,
      innerInstructions: [],
      loadedAddresses: { writable: [] as Address[], readonly: [] as Address[] },
    },
  });

  const accountInfo = () => {
    if (account.shape === 'missing') return null;
    const { leafCount, peaks } = history(writes.slice(0, account.asOf));
    const data =
      account.shape === 'notStore'
        ? new Uint8Array(64)
        : storeAccount(leafCount, account.tamper ? peaks.map(() => handleOf(0xee)) : peaks);
    return {
      data: [Buffer.from(data).toString('base64'), 'base64'],
      executable: false,
      lamports: 1_000_000n,
      owner: account.shape === 'foreign' ? other : host,
      rentEpoch: 0n,
      space: BigInt(data.length),
    };
  };

  const rpc = {
    getAccountInfo: () => ({ send: () => Promise.resolve({ context: { slot: 0n }, value: accountInfo() }) }),
    // Newest first, in one page.
    getSignaturesForAddress: (_: Address, { before, until }: { before?: string; until?: string }) => ({
      send: () => {
        const after = until === undefined ? -1 : indexOf(until);
        const newer = writes.map((_, index) => index).filter((index) => index > after && !hidden.has(index));
        return Promise.resolve(
          before === undefined ? newer.reverse().map((index) => ({ signature: signatureOf(index), err: null })) : [],
        );
      },
    }),
    getTransaction: (signature: string) => ({
      send: () => {
        fetched.push(signature);
        const index = indexOf(signature);
        const write = writes[index];
        return Promise.resolve(write === undefined || unavailable.has(index) ? null : transaction(write));
      },
    }),
  } as unknown as SolanaRpc;

  return {
    read: createSolanaLeafRecord(rpc, host),
    fetched,
    account,
    append: (write: Write) => writes.push(write),
    /** Leaves write `index` out of the signature listing, as an RPC that has not indexed it yet. */
    hide: (index: number) => hidden.add(index),
    /** Serves no transaction for write `index`, as an RPC that lists it before it serves it. */
    withhold: (index: number) => unavailable.add(index),
    release: (index: number) => {
      hidden.delete(index);
      unavailable.delete(index);
    },
    /** The answer a record over every write gives for leaf `leafIndex`. */
    found: (leafIndex: number) => {
      const { leaves } = history(writes);
      return {
        status: 'found',
        leafIndex: BigInt(leafIndex),
        leafCount: BigInt(leaves.length),
        siblings: mmrBuildProof(leaves, BigInt(leafIndex))?.siblings,
      };
    },
  };
}

const publicLeaf = (handle: number) => ({ encryptedStore: store, handle: handleOf(handle) });

describe('createSolanaLeafRecord', () => {
  it('proves each leaf where its write placed it, whatever order the RPC lists the writes in', async () => {
    const chain = ledger([
      { at: 1n, handle: 3 },
      { at: 0n, handle: 2 },
      { at: 2n, handle: 4 },
    ]);
    expect(await chain.read([publicLeaf(2), publicLeaf(3), publicLeaf(4), publicLeaf(9)])).toEqual([
      chain.found(0),
      chain.found(1),
      chain.found(2),
      { status: 'notFound', leafCount: 3n },
    ]);
  });

  it('answers the first leaf of a handle made public twice, as the listener does', async () => {
    const chain = ledger([
      { at: 0n, handle: 2 },
      { at: 1n, handle: 2 },
    ]);
    expect(await chain.read([publicLeaf(2)])).toEqual([chain.found(0)]);
  });

  it('proves at its own leaf count when the account it read lags the listing', async () => {
    const chain = ledger([
      { at: 0n, handle: 2 },
      { at: 1n, handle: 3 },
    ]);
    chain.account.asOf = 1;
    expect(await chain.read([publicLeaf(3)])).toEqual([chain.found(1)]);
  });

  it('throws when two writes claim one leaf', async () => {
    const chain = ledger([
      { at: 0n, handle: 2 },
      { at: 0n, handle: 3 },
    ]);
    await expect(chain.read([publicLeaf(2)])).rejects.toThrow(/claim leaf 0/);
  });

  it('stays at its leaf count while a write is unlisted or unserved, then catches up', async () => {
    const chain = ledger([
      { at: 0n, handle: 2 },
      { at: 1n, handle: 3 },
      { at: 2n, handle: 4 },
    ]);
    chain.hide(1);
    expect(await chain.read([publicLeaf(2)])).toEqual([{ status: 'notFound', leafCount: 0n }]);
    chain.release(1);
    chain.withhold(2);
    expect(await chain.read([publicLeaf(2)])).toEqual([{ status: 'notFound', leafCount: 0n }]);
    chain.release(2);
    expect(await chain.read([publicLeaf(2), publicLeaf(4)])).toEqual([chain.found(0), chain.found(2)]);
  });

  it('reads each transaction once: only newer ones on a later read, and one catch-up at a time', async () => {
    const chain = ledger([
      { at: 0n, handle: 2 },
      { at: 1n, handle: 3 },
    ]);
    await Promise.all([chain.read([publicLeaf(2)]), chain.read([publicLeaf(3), publicLeaf(2)])]);
    chain.append({ at: 2n, handle: 4 });
    expect(await chain.read([publicLeaf(4)])).toEqual([chain.found(2)]);
    expect(chain.fetched).toEqual(['sig1', 'sig0', 'sig2']);
  });

  it('throws on peaks that are not the account’s, then rebuilds the store from its first transaction', async () => {
    const chain = ledger([
      { at: 0n, handle: 2 },
      { at: 1n, handle: 3 },
    ]);
    await chain.read([publicLeaf(2)]);
    chain.account.tamper = true;
    await expect(chain.read([publicLeaf(2)])).rejects.toThrow(/do not match its peaks at leaf count 2/);
    chain.account.tamper = false;
    expect(await chain.read([publicLeaf(3)])).toEqual([chain.found(1)]);
    expect(chain.fetched).toEqual(['sig1', 'sig0', 'sig1', 'sig0']);
  });

  it('knows no store at an address the host does not hold one at', async () => {
    for (const shape of ['missing', 'foreign', 'notStore'] as const) {
      const chain = ledger([{ at: 0n, handle: 2 }]);
      chain.account.shape = shape;
      expect(await chain.read([publicLeaf(2)])).toEqual([{ status: 'unknownAccount' }]);
      expect(chain.fetched).toEqual([]);
    }
  });
});
