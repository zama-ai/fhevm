import { prepareTransientStore } from '@fhevm/sdk/solana';
import { ZAMA_HOST_PROGRAM_ADDRESS } from '@fhevm/solana-zama-host';
import { address, createNoopSigner } from '@solana/kit';
import { beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  buildClaim: vi.fn(),
  buildInitialize: vi.fn(),
  getBatch: vi.fn(),
  getJoinRecord: vi.fn(),
  createClient: vi.fn(),
  send: vi.fn(),
}));

vi.mock('@fhevm/solana-zama-host', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@fhevm/solana-zama-host')>()),
  createFinalizedRpc: () => ({}),
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
  programs: { host: ZAMA_HOST_PROGRAM_ADDRESS },
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

  test('initializes the payout account in the claim transaction, sent by the keeper', async () => {
    await claimBatchPayout({ config, keeper } as never, position, 'deposit', user);

    expect(mocks.buildInitialize).toHaveBeenCalledWith(
      expect.objectContaining({ payer: keeper, owner: user, mint: config.mints.payoutConfidential }),
    );
    expect(mocks.buildClaim).toHaveBeenCalledWith(
      expect.objectContaining({ payer: keeper, user, batch }),
    );
    expect(mocks.createClient).toHaveBeenCalledWith(config, keeper);
    expect(mocks.send.mock.calls[0]?.[0]).toMatchObject({ address: keeperStore });
    expect(mocks.send.mock.calls[0]?.[1]).toEqual([initializeInstruction, claimInstruction]);
  });

  test('treats an already claimed join as idempotent success', async () => {
    mocks.getJoinRecord.mockResolvedValue({ batch, user, claimed: true });

    await claimBatchPayout({ config, keeper } as never, position, 'deposit', user);

    expect(mocks.send).not.toHaveBeenCalled();
  });
});
