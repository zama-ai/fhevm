import { beforeEach, describe, expect, test, vi } from 'vitest';

// The lookup-table lifecycle is the demo's only unbounded rent risk: every batch derives a table
// from a recent slot, and once that slot has passed the address cannot be re-derived. These tests
// cover the paths that leave a table active with nobody to retry it — a settlement whose separate
// deactivation transaction failed, a canceled batch that never settles at all, and a crash between
// creating a table and recording its address.

const mocks = vi.hoisted(() => ({
  getAccountInfoSend: vi.fn(),
  getSlotSend: vi.fn(),
  sendTransaction: vi.fn(),
  getBatchByIndex: vi.fn(),
  getCurrentBatch: vi.fn(),
  openBatchForBatcher: vi.fn(),
  deactivate: vi.fn((input: { lookupTable: string }) => ({ kind: 'deactivate', ...input })),
  close: vi.fn((input: { lookupTable: string }) => ({ kind: 'close', ...input })),
  extend: vi.fn((input: { lookupTable: string }) => [{ kind: 'extend', ...input }]),
  readFile: vi.fn(),
  writeFile: vi.fn(),
  rename: vi.fn(),
}));

const rpc = {
  getAccountInfo: (address: string) => ({ send: () => mocks.getAccountInfoSend(address) }),
  getSlot: () => ({ send: () => mocks.getSlotSend() }),
};

vi.mock('@solana/kit', () => ({
  createSolanaRpc: () => rpc,
  getAddressEncoder: () => ({ encode: () => new Uint8Array(32) }),
}));

vi.mock('./vault/index.js', () => ({
  LOOKUP_TABLE_DEACTIVATION_COOLDOWN_SLOTS: 513n,
  LOOKUP_TABLE_STILL_ACTIVE: 18446744073709551615n,
  decodeLookupTableDeactivationSlot: (bytes: Buffer) => BigInt(bytes.readBigUInt64LE(4)),
  getCloseLookupTableInstruction: mocks.close,
  getDeactivateLookupTableInstruction: mocks.deactivate,
  getExtendLookupTableInstructions: mocks.extend,
  getBatchByIndex: mocks.getBatchByIndex,
  getCurrentBatch: mocks.getCurrentBatch,
  openBatchForBatcher: mocks.openBatchForBatcher,
}));

vi.mock('./sendTransaction', () => ({ sendTransaction: mocks.sendTransaction }));
vi.mock('./vaultRoots', () => ({ vaultRoots: () => ({ batcher: 'batcher-1' }) }));
vi.mock('node:fs/promises', () => ({
  readFile: mocks.readFile,
  writeFile: mocks.writeFile,
  rename: mocks.rename,
}));

import { prepareNextBatch } from './batchProvisioning';
import { BATCH_CANCELED, BATCH_DISPATCHED, BATCH_PENDING, BATCH_SETTLED } from './batchTypes';

const REGISTRY_PATH = '/tmp/registry.json';
const LOOKUP_TABLE_PROGRAM = 'AddressLookupTab1e1111111111111111111111111';

const config = {
  chainId: 42,
  rpcUrl: 'http://rpc',
  authorityFundingLamports: '1000',
  batchers: { deposit: { batcher: 'batcher-1', lookupTable: 'table-batch-0' }, redeem: { batcher: 'batcher-2', lookupTable: 'table-redeem-0' } },
} as never;

const keeper = { address: 'keeper' } as never;

/** A lookup-table account whose stored deactivation slot is `deactivationSlot`. */
const lookupTableAccount = (deactivationSlot: bigint) => {
  const bytes = Buffer.alloc(56);
  bytes.writeBigUInt64LE(deactivationSlot, 4);
  return { value: { owner: LOOKUP_TABLE_PROGRAM, data: [bytes.toString('base64'), 'base64'] } };
};

const STILL_ACTIVE = 18446744073709551615n;

/**
 * An active lookup table holding one address that is not the one this batch expects. The mocked
 * address encoder returns zeroes, so a non-zero stored address is what makes the prefix mismatch.
 */
const lookupTableWithForeignAddress = () => {
  const bytes = Buffer.alloc(56 + 32, 0);
  bytes.writeBigUInt64LE(STILL_ACTIVE, 4);
  bytes.fill(0xab, 56);
  return { value: { owner: LOOKUP_TABLE_PROGRAM, data: [bytes.toString('base64'), 'base64'] } };
};

/** The last registry object handed to `writeFile`, parsed. */
const writtenRegistry = (call = -1): Record<string, { table: string; retired?: string[] }> => {
  const calls = mocks.writeFile.mock.calls;
  const chosen = call < 0 ? calls[calls.length + call] : calls[call];
  return JSON.parse(chosen![1] as string);
};

const setRegistryFile = (registry: unknown): void => {
  mocks.readFile.mockResolvedValue(JSON.stringify(registry));
};

/**
 * `prepareNextBatch` for batch 1, with batch 0 already settled. The table for batch 1 is fully
 * provisioned already unless `tableAccounts` says otherwise, so each test only has to describe the
 * tables it cares about.
 */
const prepare = async (tableAccounts: Record<string, ReturnType<typeof lookupTableAccount> | { value: null }>) => {
  mocks.getCurrentBatch.mockResolvedValue({
    index: 0n,
    addresses: { batch: 'batch-0' },
    state: { status: BATCH_SETTLED },
  });
  mocks.openBatchForBatcher.mockResolvedValue({
    lookupTableAddress: 'table-fresh',
    lookupTableAddresses: [],
    instructions: [{ kind: 'open' }, { kind: 'create' }],
  });
  mocks.getAccountInfoSend.mockImplementation((address: string) => tableAccounts[address] ?? { value: null });
  return prepareNextBatch(config, keeper, 'deposit', REGISTRY_PATH);
};

const deactivatedTables = (): string[] =>
  mocks.sendTransaction.mock.calls
    .flatMap((call) => call[2] as { kind: string; lookupTable?: string }[])
    .filter((instruction) => instruction.kind === 'deactivate')
    .map((instruction) => instruction.lookupTable!);

const closedTables = (): string[] =>
  mocks.sendTransaction.mock.calls
    .flatMap((call) => call[2] as { kind: string; lookupTable?: string }[])
    .filter((instruction) => instruction.kind === 'close')
    .map((instruction) => instruction.lookupTable!);

beforeEach(() => {
  vi.clearAllMocks();
  mocks.getSlotSend.mockResolvedValue(10_000n);
  mocks.sendTransaction.mockResolvedValue('signature');
  mocks.readFile.mockRejectedValue(new Error('no registry yet'));
  mocks.writeFile.mockResolvedValue(undefined);
  mocks.rename.mockResolvedValue(undefined);
});

describe('the lookup-table crank retries what settlement could not finish', () => {
  test('deactivates a settled batch table that is still active', async () => {
    // The state a failed post-settlement deactivation leaves behind: batch settled, table live.
    setRegistryFile({ '42:batcher-1:0:batch-0': 'table-batch-0-recorded' });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_SETTLED } });

    await prepare({
      'table-batch-0-recorded': lookupTableAccount(STILL_ACTIVE),
      'table-fresh': { value: null },
    });

    expect(deactivatedTables()).toContain('table-batch-0-recorded');
    // Closing waits out the cooldown, so this crank must not attempt it.
    expect(closedTables()).not.toContain('table-batch-0-recorded');
  });

  test('deactivates a canceled batch table, which settlement never touches', async () => {
    setRegistryFile({ '42:batcher-1:0:batch-0': 'table-canceled' });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_CANCELED } });

    await prepare({ 'table-canceled': lookupTableAccount(STILL_ACTIVE), 'table-fresh': { value: null } });

    expect(deactivatedTables()).toContain('table-canceled');
  });

  test('leaves a dispatched batch table alone — settlement still needs it', async () => {
    setRegistryFile({ '42:batcher-1:0:batch-0': 'table-settling' });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_DISPATCHED } });

    await prepare({ 'table-settling': lookupTableAccount(STILL_ACTIVE), 'table-fresh': { value: null } });

    expect(deactivatedTables()).toEqual([]);
  });

  test('never deactivates twice: an already-deactivated table is only closed once cooled', async () => {
    setRegistryFile({ '42:batcher-1:0:batch-0': 'table-cooled' });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_SETTLED } });

    await prepare({ 'table-cooled': lookupTableAccount(100n), 'table-fresh': { value: null } });

    expect(deactivatedTables()).toEqual([]);
    expect(closedTables()).toContain('table-cooled');
  });

  test('a deactivation that throws is retried by the next crank, not swallowed into the record', async () => {
    setRegistryFile({ '42:batcher-1:0:batch-0': 'table-stubborn' });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_SETTLED } });
    mocks.sendTransaction.mockImplementation((_config: unknown, _keeper: unknown, instructions: { kind: string }[]) => {
      if (instructions.some((instruction) => instruction.kind === 'deactivate')) {
        throw new Error('blockhash expired');
      }
      return Promise.resolve('signature');
    });

    await expect(prepare({ 'table-stubborn': lookupTableAccount(STILL_ACTIVE), 'table-fresh': { value: null } })).resolves.toBeDefined();

    // Still recorded: the address is the only handle on that rent, and the next crank will retry.
    const registry = writtenRegistry();
    expect(Object.values(registry).flatMap((entry) => [entry.table, ...(entry.retired ?? [])])).toContain(
      'table-stubborn',
    );
  });
});

describe('the table address is recorded before it can exist', () => {
  test('writes the record before sending any create or extend', async () => {
    mocks.getCurrentBatch.mockResolvedValue({
      index: 0n,
      addresses: { batch: 'batch-0' },
      state: { status: BATCH_SETTLED },
    });
    mocks.openBatchForBatcher.mockResolvedValue({
      lookupTableAddress: 'table-fresh',
      lookupTableAddresses: ['account-a'],
      instructions: [{ kind: 'open' }, { kind: 'create' }],
    });
    mocks.getAccountInfoSend.mockResolvedValue({ value: null });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_SETTLED } });

    const order: string[] = [];
    mocks.writeFile.mockImplementation(() => {
      order.push('write-registry');
      return Promise.resolve();
    });
    mocks.sendTransaction.mockImplementation((_config: unknown, _keeper: unknown, instructions: { kind: string }[]) => {
      order.push(instructions.map((instruction) => instruction.kind).join('+'));
      return Promise.resolve('signature');
    });

    await prepareNextBatch(config, keeper, 'deposit', REGISTRY_PATH);

    // The open transaction may precede the record — it creates no table. Everything that can bring
    // a lookup table into existence must come after it.
    const firstWrite = order.indexOf('write-registry');
    const createIndex = order.findIndex((entry) => entry.includes('create'));
    expect(firstWrite).toBeGreaterThanOrEqual(0);
    expect(createIndex).toBeGreaterThan(firstWrite);
  });

  test('retires a recorded table it cannot reuse instead of overwriting it', async () => {
    // The crash case: `table-orphan` was recorded and created, but its contents do not match what
    // this run needs, so a fresh table is derived from a newer slot. Overwriting the record here is
    // what used to make `table-orphan` unreclaimable.
    setRegistryFile({ '42:batcher-1:1:batch-0': { table: 'table-orphan' } });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_DISPATCHED } });
    mocks.getCurrentBatch.mockResolvedValue({
      index: 0n,
      addresses: { batch: 'batch-0' },
      state: { status: BATCH_SETTLED },
    });
    mocks.openBatchForBatcher.mockResolvedValue({
      lookupTableAddress: 'table-fresh',
      lookupTableAddresses: ['account-a'],
      instructions: [{ kind: 'open' }, { kind: 'create' }],
    });
    // The orphan exists on chain but holds an address this batch does not want, so its contents
    // cannot serve as a prefix and a table derived from a newer slot has to take over.
    mocks.getAccountInfoSend.mockImplementation((address: string) =>
      address === 'table-orphan' ? lookupTableWithForeignAddress() : { value: null },
    );

    await prepareNextBatch(config, keeper, 'deposit', REGISTRY_PATH);

    const entry = writtenRegistry(0)['42:batcher-1:1:batch-0']!;
    expect(entry.table).toBe('table-fresh');
    expect(entry.retired).toEqual(['table-orphan']);
  });

  test('reads a legacy bare-string record as the batch table', async () => {
    setRegistryFile({ '42:batcher-1:1:batch-0': 'table-legacy' });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_SETTLED } });
    mocks.getCurrentBatch.mockResolvedValue({
      index: 0n,
      addresses: { batch: 'batch-0' },
      state: { status: BATCH_SETTLED },
    });
    mocks.openBatchForBatcher.mockResolvedValue({
      lookupTableAddress: 'table-fresh',
      lookupTableAddresses: [],
      instructions: [{ kind: 'open' }, { kind: 'create' }],
    });
    mocks.getAccountInfoSend.mockImplementation((address: string) =>
      address === 'table-legacy' ? lookupTableAccount(STILL_ACTIVE) : { value: null },
    );

    const prepared = await prepareNextBatch(config, keeper, 'deposit', REGISTRY_PATH);

    // An empty expected-address list means the recorded table already holds everything needed.
    expect(prepared.lookupTable).toBe('table-legacy');
  });
});

describe('a pending batch needs no open transaction', () => {
  test('reuses the pending batch and still records its table first', async () => {
    mocks.getCurrentBatch.mockResolvedValue({
      index: 3n,
      addresses: { batch: 'batch-3' },
      state: { status: BATCH_PENDING },
    });
    mocks.openBatchForBatcher.mockResolvedValue({
      lookupTableAddress: 'table-3',
      lookupTableAddresses: [],
      instructions: [{ kind: 'open' }, { kind: 'create' }],
    });
    mocks.getAccountInfoSend.mockResolvedValue({ value: null });
    mocks.getBatchByIndex.mockResolvedValue({ state: { status: BATCH_SETTLED } });

    const prepared = await prepareNextBatch(config, keeper, 'deposit', REGISTRY_PATH);

    expect(prepared.batchIndex).toBe(3n);
    expect(writtenRegistry(0)['42:batcher-1:3:batch-3']).toEqual({ table: 'table-3' });
    expect(mocks.sendTransaction.mock.calls.flatMap((call) => call[2] as { kind: string }[])).not.toContainEqual({
      kind: 'open',
    });
  });
});
