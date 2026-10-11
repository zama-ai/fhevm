import { beforeEach, describe, expect, test, vi } from 'vitest';
import { address, createNoopSigner } from '@solana/kit';

const mocks = vi.hoisted(() => ({
  getBalanceSend: vi.fn(),
  sendTransaction: vi.fn(),
  getBatcher: vi.fn(),
  getBatchByIndex: vi.fn(),
  getCurrentBatch: vi.fn(),
  reclaim: vi.fn((input: { batch: string }) => ({ kind: 'reclaim', ...input })),
}));

vi.mock('./demoClient', () => ({
  createDemoClient: () => ({
    rpc: { getBalance: (address: string) => ({ send: () => mocks.getBalanceSend(address) }) },
    sendTransaction: mocks.sendTransaction,
  }),
}));
vi.mock('./vault/index.js', () => ({
  getReclaimBatchAuthorityInstructionAsync: mocks.reclaim,
  getBatcher: mocks.getBatcher,
  getBatchByIndex: mocks.getBatchByIndex,
  getCurrentBatch: mocks.getCurrentBatch,
}));
vi.mock('./vaultRoots', () => ({ vaultRoots: () => ({ batcher: 'batcher-1' }) }));

import { RECLAIM_SCAN_WINDOW, prepareNextBatch, reclaimFinishedBatchAuthorities } from './batchProvisioning';
import { BatchStatus } from './batchTypes';

const config = {
  chainId: 42,
  rpcUrl: 'http://rpc',
  batchers: { deposit: { batcher: 'batcher-1' }, redeem: { batcher: 'batcher-2' } },
} as never;

const keeper = createNoopSigner(address('5bV6jUfhDHCQVA1WfKBUnXUsboJgoKgkzkKcxr3joew5'));

const reclaimedBatches = (): string[] =>
  mocks.sendTransaction.mock.calls
    .flatMap((call) => call[0] as { kind: string; batch?: string }[])
    .filter((instruction) => instruction.kind === 'reclaim')
    .map((instruction) => instruction.batch!);

beforeEach(() => {
  vi.clearAllMocks();
  // The reclaim pass is covered on its own below; the prepare tests run it over no batches.
  mocks.getBatcher.mockResolvedValue({ nextBatchIndex: 0n });
  mocks.getBalanceSend.mockResolvedValue({ value: 0n });
  mocks.sendTransaction.mockResolvedValue({});
});

describe('the authority-reclaim crank drains every finished batch once', () => {
  const batches: Record<string, { status: BatchStatus; lamports: bigint }> = {
    '0': { status: BatchStatus.Settled, lamports: 0n },
    '1': { status: BatchStatus.Canceled, lamports: 50_000n },
    '2': { status: BatchStatus.Refunding, lamports: 70_000n },
    '3': { status: BatchStatus.Dispatched, lamports: 100_000_000n },
    '4': { status: BatchStatus.Pending, lamports: 100_000_000n },
  };

  beforeEach(() => {
    mocks.getBatcher.mockResolvedValue({ nextBatchIndex: 5n });
    mocks.getBatchByIndex.mockImplementation((_rpc: unknown, _roots: unknown, index: bigint) =>
      Promise.resolve({
        index,
        addresses: { batch: `batch-${index}`, batchAuthority: `authority-${index}` },
        state: { status: batches[index.toString()]!.status },
      }),
    );
    mocks.getBalanceSend.mockImplementation((address: string) =>
      Promise.resolve({ value: batches[address.slice('authority-'.length)]!.lamports }),
    );
  });

  test('reclaims finished batches that still hold funding and skips live or drained ones', async () => {
    // Settled-and-drained (0) is idempotently skipped; dispatched (3) and pending (4) still need
    // their authority to pay settle's rent, so only the canceled and refunding batches are drained.
    await expect(reclaimFinishedBatchAuthorities(config, keeper, 'deposit')).resolves.toBe(2);
    expect(reclaimedBatches()).toEqual(['batch-1', 'batch-2']);
    expect(mocks.reclaim).toHaveBeenCalledWith(
      expect.objectContaining({ authority: keeper, batcher: 'batcher-1', batch: 'batch-1', batchAuthority: 'authority-1' }),
    );
  });

  test('a reclaim that throws is skipped and left for the next crank', async () => {
    mocks.sendTransaction.mockImplementation((instructions: { batch?: string }[]) => {
      if (instructions.some((instruction) => instruction.batch === 'batch-1')) throw new Error('blockhash expired');
      return Promise.resolve({});
    });
    await expect(reclaimFinishedBatchAuthorities(config, keeper, 'deposit')).resolves.toBe(1);
    expect(reclaimedBatches()).toEqual(['batch-1', 'batch-2']);
  });

  test('inspects only the most recent batches so the join path stays bounded', async () => {
    mocks.getBatcher.mockResolvedValue({ nextBatchIndex: RECLAIM_SCAN_WINDOW + 5n });
    mocks.getBatchByIndex.mockImplementation((_rpc: unknown, _roots: unknown, index: bigint) =>
      Promise.resolve({
        index,
        addresses: { batch: `batch-${index}`, batchAuthority: `authority-${index}` },
        state: { status: BatchStatus.Canceled },
      }),
    );
    mocks.getBalanceSend.mockResolvedValue({ value: 50_000n });
    await expect(reclaimFinishedBatchAuthorities(config, keeper, 'deposit')).resolves.toBe(Number(RECLAIM_SCAN_WINDOW));
    expect(mocks.getBatchByIndex).toHaveBeenCalledTimes(Number(RECLAIM_SCAN_WINDOW));
    expect(reclaimedBatches()[0]).toBe('batch-5');
  });

  test('prepareNextBatch returns the pending batch and runs the reclaim pass', async () => {
    mocks.getCurrentBatch.mockResolvedValue({ index: 4n, addresses: { batch: 'batch-4' }, state: { status: BatchStatus.Pending } });
    await expect(prepareNextBatch(config, keeper, 'deposit')).resolves.toEqual({ batchIndex: 4n, batch: 'batch-4' });
    expect(reclaimedBatches()).toEqual(['batch-1', 'batch-2']);
  });
});
