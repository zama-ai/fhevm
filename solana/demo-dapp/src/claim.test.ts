import { address } from '@solana/kit';
import { beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  accountInfo: vi.fn(),
  buildClaim: vi.fn(),
  buildInitialize: vi.fn(),
  getBatch: vi.fn(),
  getJoinRecord: vi.fn(),
  send: vi.fn(),
}));

vi.mock('@solana/kit', async (importOriginal) => {
  const original = await importOriginal<typeof import('@solana/kit')>();
  return {
    ...original,
    createSolanaRpc: () => ({
      getAccountInfo: () => ({ send: mocks.accountInfo }),
    }),
  };
});
vi.mock('./vault/index.js', () => ({
  buildClaimInstruction: mocks.buildClaim,
  buildInitializeTokenAccountInstruction: mocks.buildInitialize,
  deriveJoinRecordAddress: vi.fn(async () => address('SysvarC1ock11111111111111111111111111111111')),
  getBatchByIndex: mocks.getBatch,
  getJoinRecord: mocks.getJoinRecord,
  tokenAccountAddress: vi.fn(async () => address('SysvarRent111111111111111111111111111111111')),
}));
vi.mock('./sendTransaction', () => ({ sendTransaction: mocks.send }));

import type { DemoConfig } from './demoConfig';
import { claimBatchPayout } from './claim';

const batch = address('11111111111111111111111111111111');
const user = address('SysvarC1ock11111111111111111111111111111111');
const tokenProgram = address('SysvarRent111111111111111111111111111111111');
const keeper = { address: address('SysvarRecentB1ockHashes11111111111111111111') };
const config = {
  rpcUrl: 'http://127.0.0.1:8899',
  wsUrl: 'ws://127.0.0.1:8900',
  hostConfig: address('SysvarS1otHashes111111111111111111111111111'),
  programs: { token: tokenProgram },
  mints: {
    joinUnderlying: address('SysvarStakeHistory1111111111111111111111111'),
    payoutUnderlying: address('Stake11111111111111111111111111111111111111'),
    joinConfidential: address('Vote111111111111111111111111111111111111111'),
    payoutConfidential: address('Config1111111111111111111111111111111111111'),
  },
  batchers: {
    deposit: { batcher: address('AddressLookupTab1e1111111111111111111111111') },
    redeem: { batcher: address('ComputeBudget111111111111111111111111111111') },
  },
} as unknown as DemoConfig;
const position = { batchIndex: 1n, batch, amountBaseUnits: 100_000_000n };

describe('sponsored payout claim', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.getBatch.mockResolvedValue({ index: 1n, addresses: { batch }, state: { status: 2 } });
    mocks.getJoinRecord.mockResolvedValue({ batch, user, claimed: false });
    mocks.buildInitialize.mockResolvedValue({ programAddress: tokenProgram, accounts: [], data: new Uint8Array() });
    mocks.buildClaim.mockResolvedValue({ programAddress: tokenProgram, accounts: [], data: new Uint8Array() });
    mocks.send.mockResolvedValue(undefined);
  });

  test('atomically initializes a missing payout account and claims with the keeper', async () => {
    mocks.accountInfo.mockResolvedValue({ value: null });

    await claimBatchPayout({ config, keeper } as never, position, 'deposit', user);

    expect(mocks.buildInitialize).toHaveBeenCalledWith(
      expect.objectContaining({ payer: keeper, owner: user, mint: config.mints.payoutConfidential }),
    );
    expect(mocks.buildClaim).toHaveBeenCalledWith(
      expect.objectContaining({ payer: keeper, user, batch }),
    );
    expect(mocks.send.mock.calls[0]?.[2]).toHaveLength(2);
  });

  test('claims directly when the canonical payout account already exists', async () => {
    mocks.accountInfo.mockResolvedValue({ value: { owner: tokenProgram } });

    await claimBatchPayout({ config, keeper } as never, position, 'redeem', user);

    expect(mocks.buildInitialize).not.toHaveBeenCalled();
    expect(mocks.send.mock.calls[0]?.[2]).toHaveLength(1);
  });

  test('initializes and claims a pre-funded System-owned payout account', async () => {
    mocks.accountInfo.mockResolvedValue({ value: { owner: address('11111111111111111111111111111111') } });

    await claimBatchPayout({ config, keeper } as never, position, 'deposit', user);

    expect(mocks.buildInitialize).toHaveBeenCalledOnce();
    expect(mocks.send.mock.calls[0]?.[2]).toHaveLength(2);
  });

  test('re-reads state and retries once after an initialization race', async () => {
    mocks.accountInfo
      .mockResolvedValueOnce({ value: null })
      .mockResolvedValueOnce({ value: { owner: tokenProgram } });
    mocks.send.mockRejectedValueOnce(new Error('account already in use')).mockResolvedValueOnce(undefined);

    await claimBatchPayout({ config, keeper } as never, position, 'deposit', user);

    expect(mocks.getJoinRecord).toHaveBeenCalledTimes(2);
    expect(mocks.send).toHaveBeenCalledTimes(2);
    expect(mocks.send.mock.calls[1]?.[2]).toHaveLength(1);
  });

  test('does not retry a permanent failure', async () => {
    mocks.accountInfo.mockResolvedValue({ value: null });
    mocks.send.mockRejectedValue(new Error('claim failed'));

    await expect(claimBatchPayout({ config, keeper } as never, position, 'deposit', user)).rejects.toThrow(
      'claim failed',
    );

    expect(mocks.send).toHaveBeenCalledTimes(1);
  });

  test('treats an already claimed join as idempotent success', async () => {
    mocks.getJoinRecord.mockResolvedValue({ batch, user, claimed: true });

    await claimBatchPayout({ config, keeper } as never, position, 'deposit', user);

    expect(mocks.send).not.toHaveBeenCalled();
  });

  test('rejects a payout PDA owned by an unexpected program', async () => {
    mocks.accountInfo.mockResolvedValue({ value: { owner: user } });

    await expect(claimBatchPayout({ config, keeper } as never, position, 'deposit', user)).rejects.toThrow(
      'unexpected program',
    );
    expect(mocks.send).not.toHaveBeenCalled();
  });
});
