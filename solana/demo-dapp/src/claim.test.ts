import { SYSTEM_PROGRAM_ADDRESS } from '@solana-program/system';
import { prepareTransientStore } from '@fhevm/sdk/solana';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { address, createNoopSigner } from '@solana/kit';
import { beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  accountInfo: vi.fn(),
  buildClaim: vi.fn(),
  buildInitialize: vi.fn(),
  getBatch: vi.fn(),
  getJoinRecord: vi.fn(),
  createClient: vi.fn(),
  send: vi.fn(),
}));

vi.mock('@fhevm/solana-zama-host', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@fhevm/solana-zama-host')>()),
  createFinalizedRpc: () => ({
      getAccountInfo: () => ({ send: mocks.accountInfo }),
    }),
}));
vi.mock('./vault/index.js', () => ({
  TOKEN_PROGRAM_ADDRESS: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  buildClaimInstruction: mocks.buildClaim,
  buildInitializeTokenAccountInstruction: mocks.buildInitialize,
  findJoinRecordPda: vi.fn(async () => [address('SysvarC1ock11111111111111111111111111111111'), 255]),
  getBatchByIndex: mocks.getBatch,
  getJoinRecord: mocks.getJoinRecord,
}));
vi.mock('./demoClient', () => ({ createDemoClient: mocks.createClient }));

import type { DemoConfig } from './demoConfig';
import { claimBatchPayout } from './claim';

const batch = address('11111111111111111111111111111111');
const user = address('SysvarC1ock11111111111111111111111111111111');
const tokenProgram = address('SysvarRent111111111111111111111111111111111');
const keeper = createNoopSigner(address('SysvarRecentB1ockHashes11111111111111111111'));
const config = {
  rpcUrl: 'http://127.0.0.1:8899',
  wsUrl: 'ws://127.0.0.1:8900',
  programs: { token: tokenProgram, host: ZAMA_HOST_PROGRAM_ADDRESS },
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
const initializeInstruction = { programAddress: tokenProgram, accounts: [], data: new Uint8Array([1]) };
const claimInstruction = { programAddress: tokenProgram, accounts: [], data: new Uint8Array([2]) };
let keeperStore: string;
const sentBody = (call: number) => mocks.send.mock.calls[call]?.[1];

describe('sponsored payout claim', () => {
  beforeEach(async () => {
    keeperStore = (await prepareTransientStore({ payer: keeper, host: ZAMA_HOST_PROGRAM_ADDRESS })).address;
    vi.clearAllMocks();
    mocks.createClient.mockReturnValue({ sendFheTransaction: mocks.send });
    mocks.getBatch.mockResolvedValue({ index: 1n, addresses: { batch }, state: { status: 2 } });
    mocks.getJoinRecord.mockResolvedValue({ batch, user, claimed: false });
    mocks.buildInitialize.mockResolvedValue(initializeInstruction);
    mocks.buildClaim.mockResolvedValue(claimInstruction);
    mocks.send.mockResolvedValue({ context: { signature: 'claim-signature' } });
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
    expect(mocks.createClient).toHaveBeenCalledWith(config, keeper);
    expect(mocks.send.mock.calls[0]?.[0]).toMatchObject({ address: keeperStore });
    expect(sentBody(0)).toEqual([initializeInstruction, claimInstruction]);
  });

  test('claims directly when the canonical payout account already exists', async () => {
    mocks.accountInfo.mockResolvedValue({ value: { owner: tokenProgram } });

    await claimBatchPayout({ config, keeper } as never, position, 'redeem', user);

    expect(mocks.buildInitialize).not.toHaveBeenCalled();
    expect(sentBody(0)).toEqual([claimInstruction]);
  });

  test('initializes and claims a pre-funded System-owned payout account', async () => {
    mocks.accountInfo.mockResolvedValue({ value: { owner: SYSTEM_PROGRAM_ADDRESS } });

    await claimBatchPayout({ config, keeper } as never, position, 'deposit', user);

    expect(mocks.buildInitialize).toHaveBeenCalledOnce();
    expect(sentBody(0)).toEqual([initializeInstruction, claimInstruction]);
  });

  test('re-reads state and retries once after an initialization race', async () => {
    mocks.accountInfo
      .mockResolvedValueOnce({ value: null })
      .mockResolvedValueOnce({ value: { owner: tokenProgram } });
    mocks.send
      .mockRejectedValueOnce(new Error('account already in use'))
      .mockResolvedValueOnce({ context: { signature: 'claim-signature' } });

    await claimBatchPayout({ config, keeper } as never, position, 'deposit', user);

    expect(mocks.getJoinRecord).toHaveBeenCalledTimes(2);
    expect(mocks.send).toHaveBeenCalledTimes(2);
    expect(sentBody(0)).toEqual([initializeInstruction, claimInstruction]);
    expect(sentBody(1)).toEqual([claimInstruction]);
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
