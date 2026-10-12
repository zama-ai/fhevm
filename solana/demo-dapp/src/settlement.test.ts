import { beforeEach, describe, expect, test, vi } from 'vitest';
import { address, createNoopSigner } from '@solana/kit';

const mocks = vi.hoisted(() => ({
  sendFheTransaction: vi.fn(),
  getBatchByIndex: vi.fn(),
  getBatcher: vi.fn(),
  getBatchJoinRecords: vi.fn(),
  buildQuit: vi.fn((input: { user: string }) => Promise.resolve({ kind: 'quit', user: input.user })),
  buildCancel: vi.fn(() => Promise.resolve({ kind: 'cancel' })),
  clock: vi.fn(),
  reclaim: vi.fn(),
}));

vi.mock('./demoClient', () => ({
  createDemoClient: () => ({ rpc: {}, sendFheTransaction: mocks.sendFheTransaction }),
}));
vi.mock('./vault/index.js', () => ({
  buildCancelDispatchInstruction: mocks.buildCancel,
  buildQuitInstruction: mocks.buildQuit,
  getBatchByIndex: mocks.getBatchByIndex,
  getBatcher: mocks.getBatcher,
  getBatchJoinRecords: mocks.getBatchJoinRecords,
  readHostPolicy: () => Promise.resolve({}),
  settleDeadline: () => 100n,
}));
vi.mock('@fhevm/sdk/solana', () => ({ prepareTransientStore: () => Promise.resolve({}) }));
vi.mock('@solana/sysvars', () => ({ fetchSysvarClock: mocks.clock }));
vi.mock('./batchProvisioning', () => ({ reclaimFinishedBatchAuthorities: mocks.reclaim }));
vi.mock('./vaultRoots', () => ({ vaultRoots: () => ({ batcher: 'batcher-1', joinConfidentialMint: 'mint-1' }) }));

import { settleOrCancelVaultBatch } from './settlement';
import { BatchStatus } from './batchTypes';

const config = { rpcUrl: 'http://rpc', authorityFundingLamports: '0', programs: { host: 'host' } };
const keeper = createNoopSigner(address('5bV6jUfhDHCQVA1WfKBUnXUsboJgoKgkzkKcxr3joew5'));
const session = { relayerApiKey: 'key', config, keeper } as never;
const position = { batch: 'batch-0', batchIndex: 0n } as never;
const batchIn = (status: BatchStatus) => ({ index: 0n, addresses: { batch: 'batch-0' }, state: { status } });
const sent = (): { kind: string; user?: string }[] =>
  mocks.sendFheTransaction.mock.calls.flatMap((call) => call[1] as { kind: string; user?: string }[]);

beforeEach(() => {
  vi.clearAllMocks();
  mocks.reclaim.mockResolvedValue(0);
  mocks.getBatchJoinRecords.mockResolvedValue([{ user: 'alice' }, { user: 'bob' }, { user: 'carol' }]);
  mocks.sendFheTransaction.mockImplementation((_store: unknown, [instruction]: { user?: string }[]) =>
    instruction.user === 'bob'
      ? Promise.reject(new Error('quit rejected'))
      : Promise.resolve({ context: { signature: 'signature' } }),
  );
});

describe('the keeper refunds every participant of a refunding batch', () => {
  test('one failed quit does not hold back the others, and names who is left', async () => {
    mocks.getBatchByIndex.mockResolvedValue(batchIn(BatchStatus.Refunding));
    await expect(settleOrCancelVaultBatch(session, position, 'deposit')).rejects.toThrow('Refunds failed for bob');
    expect(sent().map((instruction) => instruction.user)).toEqual(['alice', 'bob', 'carol']);
  });

  test('a cancel at the settle deadline is followed by the refunds', async () => {
    mocks.getBatchByIndex
      .mockResolvedValueOnce(batchIn(BatchStatus.Dispatched))
      .mockResolvedValue(batchIn(BatchStatus.Refunding));
    mocks.getBatcher.mockResolvedValue({});
    mocks.clock.mockResolvedValue({ unixTimestamp: 100n });
    mocks.getBatchJoinRecords.mockResolvedValue([{ user: 'alice' }]);
    expect(await settleOrCancelVaultBatch(session, position, 'deposit')).toBe('signature');
    expect(sent()).toEqual([{ kind: 'cancel' }, { kind: 'quit', user: 'alice' }]);
    expect(mocks.reclaim).toHaveBeenCalledWith(config, keeper, 'deposit');
  });

  test('a failed reclaim of the finished authorities does not fail the pass', async () => {
    mocks.getBatchByIndex.mockResolvedValue(batchIn(BatchStatus.Refunding));
    mocks.getBatchJoinRecords.mockResolvedValue([{ user: 'alice' }]);
    mocks.reclaim.mockRejectedValue(new Error('rpc down'));
    expect(await settleOrCancelVaultBatch(session, position, 'deposit')).toBe('signature');
    expect(mocks.reclaim).toHaveBeenCalledOnce();
  });
});
