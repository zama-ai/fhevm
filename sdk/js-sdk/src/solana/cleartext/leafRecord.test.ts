import { describe, expect, it } from 'vitest';
import {
  address,
  getAddressDecoder,
  getAddressEncoder,
  getBase58Decoder,
  getU32Encoder,
  getU64Encoder,
  type Address,
  type ReadonlyUint8Array,
} from '@solana/kit';
import type { SolanaRpc } from '../encryptedStore.js';
import { getFheExecuteInstructionDataEncoder } from '../internal/generated/zamaHost/instructions/fheExecute.js';
import { getMakeStoreHandlePublicInstructionDataEncoder } from '../internal/generated/zamaHost/instructions/makeStoreHandlePublic.js';
import { getFheExecutedEventEncoder } from '../internal/generated/zamaHost/types/fheExecutedEvent.js';
import { createRetainedMmr, storeLeafCommitment, type SolanaStoreHistoryEvent } from './mmr.js';
import { EVENT_IX_TAG, EVENT_VERSION } from './hostConstants.js';
import { createSolanaLeafRecord } from './leafRecord.js';

const host = address('11111111111111111111111111111112');
const store = address('SysvarC1ock11111111111111111111111111111111');
const other = address('SysvarRent111111111111111111111111111111111');
const app = address('SysvarEpochSchedu1e111111111111111111111111');
const storeBytes = new Uint8Array(getAddressEncoder().encode(store));
/** `sha256("account:EncryptedStore")[..8]`. */
const ENCRYPTED_STORE_DISCRIMINATOR = [161, 143, 137, 73, 233, 30, 46, 118];

const bytes32 = (fill: number): Uint8Array => new Uint8Array(32).fill(fill);
const keyOf = (fill: number): Address => getAddressDecoder().decode(bytes32(fill));
const base58 = (data: ReadonlyUint8Array): string => getBase58Decoder().decode(data);

const u32 = (value: number): ReadonlyUint8Array => getU32Encoder().encode(value);
const u64 = (value: bigint): ReadonlyUint8Array => getU64Encoder().encode(value);

type CompiledInstruction = { programIdIndex: number; accounts: number[]; data: string };

/** One transaction that touches the store: the leaves it appends at `at`, and how it appends them. */
type Write = {
  readonly at: bigint;
  readonly events: readonly SolanaStoreHistoryEvent[];
  readonly accountKeys: readonly Address[];
  readonly loadedWritable: readonly Address[];
  readonly instructions: readonly CompiledInstruction[];
  readonly inner: readonly { index: number; instructions: CompiledInstruction[] }[];
  readonly failed?: boolean;
};

/** A top-level `make_store_handle_public` of the handle filled with `handle`, appending leaf `at`. */
const makePublic = (at: bigint, handle: number): Write => ({
  at,
  events: [{ kind: 'markedPublic', handle: bytes32(handle) }],
  accountKeys: [other, host, store],
  loadedWritable: [],
  instructions: [
    {
      programIdIndex: 1,
      // payer, authority, encryptedStore, hostConfig, denyScopeRecord, systemProgram
      accounts: [0, 0, 2, 0, 0, 0],
      data: base58(
        getMakeStoreHandlePublicInstructionDataEncoder().encode({
          key: bytes32(1),
          handle: bytes32(handle),
          previousLeafCount: at,
        }),
      ),
    },
  ],
  inner: [],
});

/**
 * An application's call into `fhe_execute`, whose one result is the handle filled with `handle`.
 * It writes `other`, then the store, loaded from a lookup table: the result allowed to each of
 * `allowed` and made public, appended at `at`.
 */
const execute = (at: bigint, handle: number, allowed: readonly number[]): Write => {
  // Static keys: payer, other, app, host. Loaded writable: store.
  const [payer, otherIndex, appIndex, hostIndex, storeIndex] = [0, 1, 2, 3, 4];
  const effect = (index: number, previousLeafCount: bigint) => ({
    result: { stepIndex: 0, outputIndex: 0 },
    storeIndex: index,
    previousLeafCount,
    slot: null,
    allowIndexes: Uint8Array.from(allowed.map((_, key) => key)),
    makePublic: true,
    grants: [],
  });
  const execution = getFheExecuteInstructionDataEncoder().encode({
    executionStoreIndex: 1,
    accountCount: 2,
    dictionary: allowed.map(bytes32),
    steps: [{ __kind: 'TrivialEncrypt', plaintext: new Uint8Array(1), fheType: 0 }],
    effects: [effect(0, 0n), effect(1, at)],
    returnedResults: [],
  });
  const event = getFheExecutedEventEncoder().encode({
    version: EVENT_VERSION,
    previousBankHash: bytes32(0),
    unixTimestamp: 0,
    results: [bytes32(handle)],
    seeds: [],
  });
  return {
    at,
    events: [
      ...allowed.map(
        (key): SolanaStoreHistoryEvent => ({ kind: 'allowed', handle: bytes32(handle), key: bytes32(key) }),
      ),
      { kind: 'markedPublic', handle: bytes32(handle) },
    ],
    accountKeys: [keyOf(0x70), other, app, host],
    loadedWritable: [store],
    instructions: [{ programIdIndex: appIndex, accounts: [payer, otherIndex, storeIndex], data: '' }],
    inner: [
      {
        index: 0,
        instructions: [
          {
            programIdIndex: hostIndex,
            // payer, authority, hostConfig, systemProgram, three absent optional accounts,
            // transientStore, instructions, eventAuthority, program; then the stores.
            accounts: [
              payer,
              payer,
              payer,
              payer,
              hostIndex,
              hostIndex,
              hostIndex,
              payer,
              payer,
              payer,
              hostIndex,
            ].concat([otherIndex, storeIndex]),
            data: base58(execution),
          },
          { programIdIndex: hostIndex, accounts: [payer], data: base58(new Uint8Array([...EVENT_IX_TAG, ...event])) },
        ],
      },
    ],
  };
};

/** An application's transaction that names the store and succeeds, but writes it nothing. */
const touch = (): Write => ({
  at: 0n,
  events: [],
  accountKeys: [other, app, store],
  loadedWritable: [],
  instructions: [{ programIdIndex: 1, accounts: [0, 2], data: '' }],
  inner: [],
});

/** The store's leaf count, peaks and proofs, from its leaves in the order the host appended them. */
const history = (writes: readonly Write[]) => {
  const tree = createRetainedMmr();
  writes
    .filter((write) => write.failed !== true)
    .sort((left, right) => Number(left.at - right.at))
    .flatMap((write) => write.events)
    .forEach((event, index) => {
      tree.append(storeLeafCommitment(storeBytes, BigInt(index), event));
    });
  const leafCount = tree.leafCount();
  return { tree, leafCount: BigInt(leafCount), peaks: tree.peaks(leafCount) };
};

/** The borsh bytes of the store account with `leafCount` leaves under `peaks`. */
const storeAccount = (leafCount: bigint, peaks: readonly Uint8Array[]): Uint8Array =>
  new Uint8Array([
    ...ENCRYPTED_STORE_DISCRIMINATOR,
    ...bytes32(0x11), // program
    ...bytes32(0x22), // authority
    ...bytes32(0x33), // scope
    ...u32(0), // slots
    ...u64(leafCount),
    ...u32(peaks.length),
    ...peaks.flatMap((peak) => [...peak]),
    0xfe, // bump
  ]);

/** Signatures per `getSignaturesForAddress` page, so a listing of more than two writes pages. */
const PAGE = 2;

/**
 * A validator whose writes to the store are `writes`, one transaction each, listed newest last.
 * Write `index` lands in slot `index + 1`. The store account holds the leaves of every successful
 * write, listed or not, and is read at the slot of the last write it holds.
 */
function ledger(initial: readonly Write[]) {
  const writes = [...initial];
  const hidden = new Set<number>();
  const unavailable = new Set<number>();
  const fetched: string[] = [];
  const gates = new Map<Address, Promise<void>>();
  const account: {
    shape: 'store' | 'missing' | 'foreign' | 'notStore';
    /** The account as of this many writes, as a read that lags the listing sees it. */
    asOf: number | undefined;
    tamper: boolean;
  } = { shape: 'store', asOf: undefined, tamper: false };
  const signatureOf = (index: number): string => `sig${index}`;
  const indexOf = (signature: string): number => Number(signature.slice('sig'.length));

  // A write that loads no account from a lookup table is sent as v1, as any client may.
  const versionOf = (write: Write): number => (write.loadedWritable.length > 0 ? 0 : 1);
  const transaction = (write: Write) => ({
    version: versionOf(write),
    transaction: {
      message: {
        header: { numRequiredSignatures: 1, numReadonlySignedAccounts: 0, numReadonlyUnsignedAccounts: 1 },
        accountKeys: write.accountKeys,
        instructions: write.instructions,
      },
    },
    meta: {
      err: null,
      innerInstructions: write.inner,
      loadedAddresses: { writable: write.loadedWritable, readonly: [] as Address[] },
    },
  });

  const accountInfo = () => {
    if (account.shape === 'missing') return null;
    const { leafCount, peaks } = history(writes.slice(0, account.asOf));
    const data =
      account.shape === 'notStore'
        ? new Uint8Array(64)
        : storeAccount(leafCount, account.tamper ? peaks.map(() => bytes32(0xee)) : peaks);
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
    // Any other address holds no account, once its gate, if any, opens.
    getAccountInfo: (address: Address) => ({
      send: async () => {
        await gates.get(address);
        return {
          context: { slot: BigInt(account.asOf ?? writes.length) },
          value: address === store ? accountInfo() : null,
        };
      },
    }),
    // Newest first, `PAGE` at a time.
    getSignaturesForAddress: (_: Address, { before, until }: { before?: string; until?: string }) => ({
      send: () => {
        const listed = writes
          .map((_, index) => index)
          .filter((index) => !hidden.has(index))
          .filter((index) => until === undefined || index > indexOf(until))
          .filter((index) => before === undefined || index < indexOf(before))
          .reverse();
        return Promise.resolve(
          listed.slice(0, PAGE).map((index) => ({
            signature: signatureOf(index),
            slot: BigInt(index + 1),
            err: writes[index]?.failed === true ? { InstructionError: [0, 'Custom'] } : null,
          })),
        );
      },
    }),
    // Refuses, as a node does, a transaction of a later version than the request supports.
    getTransaction: (
      signature: string,
      { maxSupportedTransactionVersion }: { maxSupportedTransactionVersion?: number },
    ) => ({
      send: () => {
        fetched.push(signature);
        const index = indexOf(signature);
        const write = writes[index];
        if (write === undefined || unavailable.has(index)) return Promise.resolve(null);
        if ((maxSupportedTransactionVersion ?? -1) < versionOf(write))
          return Promise.reject(new Error(`transaction version (${versionOf(write)}) is not supported`));
        return Promise.resolve(transaction(write));
      },
    }),
  } as unknown as SolanaRpc;

  return {
    read: createSolanaLeafRecord(rpc, host),
    fetched,
    account,
    append: (write: Write) => writes.push(write),
    /** Holds the account read of `address` until the returned function is called. */
    gate: (address: Address): (() => void) => {
      let open = (): void => undefined;
      gates.set(address, new Promise((resolve) => (open = resolve)));
      return () => {
        open();
      };
    },
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
      const { tree, leafCount } = history(writes);
      return {
        status: 'found',
        leafIndex: BigInt(leafIndex),
        leafCount,
        siblings: tree.proof(leafIndex, tree.leafCount()).siblings,
      };
    },
  };
}

const publicLeaf = (handle: number) => ({ encryptedStore: store, handle: bytes32(handle) });
const allowedLeaf = (handle: number, key: number) => ({ ...publicLeaf(handle), key: keyOf(key) });

describe('createSolanaLeafRecord', () => {
  it('proves each leaf where its write placed it, whatever order the RPC lists the writes in', async () => {
    const chain = ledger([makePublic(1n, 3), makePublic(0n, 2), makePublic(2n, 4)]);
    expect(await chain.read([publicLeaf(2), publicLeaf(3), publicLeaf(4), publicLeaf(9)])).toEqual([
      chain.found(0),
      chain.found(1),
      chain.found(2),
      { status: 'notFound', leafCount: 3n },
    ]);
  });

  it('records the allow and public leaves of an fhe_execute an application calls, on its store only', async () => {
    const chain = ledger([makePublic(0n, 2), execute(1n, 5, [6, 7])]);
    expect(
      await chain.read([allowedLeaf(5, 6), allowedLeaf(5, 7), publicLeaf(5), allowedLeaf(5, 8), allowedLeaf(2, 6)]),
    ).toEqual([
      chain.found(1),
      chain.found(2),
      chain.found(3),
      { status: 'notFound', leafCount: 4n },
      { status: 'notFound', leafCount: 4n },
    ]);
  });

  it('answers the first leaf of a handle made public twice, as the listener does', async () => {
    const chain = ledger([makePublic(0n, 2), makePublic(1n, 2)]);
    expect(await chain.read([publicLeaf(2)])).toEqual([chain.found(0)]);
  });

  it('answers at the leaf count of the account it read, and reads the later writes on a later read', async () => {
    const chain = ledger([makePublic(0n, 2), makePublic(1n, 3)]);
    chain.account.asOf = 1;
    expect(await chain.read([publicLeaf(3)])).toEqual([{ status: 'notFound', leafCount: 1n }]);
    expect(chain.fetched).toEqual(['sig0']);
    chain.account.asOf = undefined;
    expect(await chain.read([publicLeaf(3)])).toEqual([chain.found(1)]);
  });

  it('skips failed transactions, and reads every page of the listing', async () => {
    const chain = ledger([
      makePublic(0n, 2),
      { ...makePublic(0n, 9), failed: true },
      makePublic(1n, 3),
      makePublic(2n, 4),
      makePublic(3n, 5),
    ]);
    expect(await chain.read([publicLeaf(2), publicLeaf(5), publicLeaf(9)])).toEqual([
      chain.found(0),
      chain.found(3),
      { status: 'notFound', leafCount: 4n },
    ]);
    expect(chain.fetched).not.toContain('sig1');
  });

  it('throws when two writes claim one leaf', async () => {
    const chain = ledger([makePublic(0n, 2), makePublic(0n, 3)]);
    await expect(chain.read([publicLeaf(2)])).rejects.toThrow(/claim leaf 0/);
  });

  it('stays at its leaf count while a write is unlisted or unserved, then catches up', async () => {
    const chain = ledger([makePublic(0n, 2), makePublic(1n, 3), makePublic(2n, 4)]);
    chain.hide(1);
    expect(await chain.read([publicLeaf(2)])).toEqual([{ status: 'notFound', leafCount: 0n }]);
    chain.release(1);
    chain.withhold(2);
    expect(await chain.read([publicLeaf(2)])).toEqual([{ status: 'notFound', leafCount: 0n }]);
    chain.release(2);
    expect(await chain.read([publicLeaf(2), publicLeaf(4)])).toEqual([chain.found(0), chain.found(2)]);
  });

  // A newer transaction that appends nothing, listed before the store's last write is, must not
  // carry the record past that write for good.
  it.each([
    ['a failed write', { ...makePublic(0n, 9), failed: true }],
    ['a transaction that writes nothing', touch()],
  ])('stays behind a last write the listing does not show yet, under %s listed after it', async (_name, newer) => {
    const chain = ledger([makePublic(0n, 2), makePublic(1n, 3), newer]);
    chain.hide(1);
    expect(await chain.read([publicLeaf(2)])).toEqual([{ status: 'notFound', leafCount: 0n }]);
    chain.release(1);
    expect(await chain.read([publicLeaf(3)])).toEqual([chain.found(1)]);
  });

  // The account read is older than a write the listing leaves out, and than a newer transaction it
  // shows: the record must not move past the write for good.
  it('lists a write after the account it read again, though the listing showed a newer transaction first', async () => {
    const chain = ledger([makePublic(0n, 2), makePublic(1n, 3), touch()]);
    chain.account.asOf = 1;
    chain.hide(1);
    expect(await chain.read([publicLeaf(2)])).toEqual([{ ...chain.found(0), leafCount: 1n, siblings: [] }]);
    chain.account.asOf = undefined;
    chain.release(1);
    expect(await chain.read([publicLeaf(3)])).toEqual([chain.found(1)]);
  });

  it('reads each transaction once: only newer ones on a later read, and one catch-up at a time', async () => {
    const chain = ledger([makePublic(0n, 2), makePublic(1n, 3)]);
    await Promise.all([chain.read([publicLeaf(2)]), chain.read([publicLeaf(3), publicLeaf(2)])]);
    chain.append(makePublic(2n, 4));
    expect(await chain.read([publicLeaf(4)])).toEqual([chain.found(2)]);
    expect(chain.fetched).toEqual(['sig1', 'sig0', 'sig2']);
  });

  it('answers from the leaves its own catch-up checked, whatever a later one appends meanwhile', async () => {
    const chain = ledger([makePublic(0n, 2)]);
    const elsewhere = keyOf(0x55);
    const open = chain.gate(elsewhere);
    const waiting = chain.read([publicLeaf(3), { encryptedStore: elsewhere, handle: bytes32(3) }]);
    await new Promise((resolve) => setTimeout(resolve, 0));
    chain.append(makePublic(1n, 3));
    expect(await chain.read([publicLeaf(3)])).toEqual([chain.found(1)]);
    open();
    expect(await waiting).toEqual([{ status: 'notFound', leafCount: 1n }, { status: 'unknownAccount' }]);
  });

  it('throws on peaks that are not the account’s, then rebuilds the store from its first transaction', async () => {
    const chain = ledger([makePublic(0n, 2), makePublic(1n, 3)]);
    await chain.read([publicLeaf(2)]);
    chain.account.tamper = true;
    await expect(chain.read([publicLeaf(2)])).rejects.toThrow(/do not match its peaks at leaf count 2/);
    chain.account.tamper = false;
    expect(await chain.read([publicLeaf(3)])).toEqual([chain.found(1)]);
    expect(chain.fetched).toEqual(['sig1', 'sig0', 'sig1', 'sig0']);
  });

  it('knows no store at an address the host does not hold one at', async () => {
    for (const shape of ['missing', 'foreign', 'notStore'] as const) {
      const chain = ledger([makePublic(0n, 2)]);
      chain.account.shape = shape;
      expect(await chain.read([publicLeaf(2)])).toEqual([{ status: 'unknownAccount' }]);
      expect(chain.fetched).toEqual([]);
    }
  });
});
