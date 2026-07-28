import { beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  appendInstructions: vi.fn(() => 'message'),
  compileTransaction: vi.fn(() => 'unsigned-transaction'),
  createRpc: vi.fn(() => ({ rpc: true })),
  createRpcSubscriptions: vi.fn(() => ({ subscriptions: true })),
  createTransactionMessage: vi.fn(() => 'base-message'),
  sendAndConfirm: vi.fn(),
  setComputeLimit: vi.fn(() => 'compute-message'),
  setFeePayer: vi.fn(() => 'payer-message'),
  setLifetime: vi.fn(() => 'lifetime-message'),
  sign: vi.fn(() => Promise.resolve('signed-transaction')),
  simulateSigned: vi.fn(),
  simulateUnsigned: vi.fn(),
}));

vi.mock('@solana/kit', () => ({
  appendTransactionMessageInstructions: mocks.appendInstructions,
  assertIsFullySignedTransaction: vi.fn(),
  assertIsTransactionWithBlockhashLifetime: vi.fn(),
  assertIsTransactionWithinSizeLimit: vi.fn(),
  compileTransaction: mocks.compileTransaction,
  createSolanaRpc: mocks.createRpc,
  createSolanaRpcSubscriptions: mocks.createRpcSubscriptions,
  createTransactionMessage: mocks.createTransactionMessage,
  sendAndConfirmTransactionFactory: vi.fn(() => mocks.sendAndConfirm),
  setTransactionMessageComputeUnitLimit: mocks.setComputeLimit,
  setTransactionMessageFeePayerSigner: mocks.setFeePayer,
  setTransactionMessageLifetimeUsingBlockhash: mocks.setLifetime,
  signTransactionMessageWithSigners: mocks.sign,
}));

vi.mock('./transactionSimulation', () => ({
  simulateSignedTransactionLocally: mocks.simulateSigned,
  simulateUnsignedTransactionLocally: mocks.simulateUnsigned,
}));

import { sendTransaction } from './sendTransaction';

const config = {
  rpcUrl: 'http://127.0.0.1:8899',
  wsUrl: 'ws://127.0.0.1:8900',
} as Parameters<typeof sendTransaction>[0];
const payer = {} as Parameters<typeof sendTransaction>[1];
const rpc = {
  getLatestBlockhash: vi.fn(() => ({
    send: vi.fn().mockResolvedValue({
      value: { blockhash: 'latest-blockhash', lastValidBlockHeight: 1_000n },
    }),
  })),
};

describe('sendTransaction simulation boundary', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.createRpc.mockReturnValue(rpc as never);
    mocks.simulateSigned.mockResolvedValue(undefined);
    mocks.simulateUnsigned.mockResolvedValue(undefined);
    mocks.sign.mockResolvedValue('signed-transaction');
  });

  test('does not open the wallet when unsigned local simulation fails', async () => {
    mocks.simulateUnsigned.mockRejectedValue(new Error('unsigned simulation failed'));

    await expect(sendTransaction(config, payer, [], 100_000)).rejects.toThrow('unsigned simulation failed');

    expect(mocks.sign).not.toHaveBeenCalled();
    expect(mocks.simulateSigned).not.toHaveBeenCalled();
    expect(mocks.sendAndConfirm).not.toHaveBeenCalled();
  });

  test('does not submit when signed local simulation fails', async () => {
    mocks.simulateSigned.mockRejectedValue(new Error('signed simulation failed'));

    await expect(sendTransaction(config, payer, [], 100_000)).rejects.toThrow('signed simulation failed');

    expect(mocks.sign).toHaveBeenCalledOnce();
    expect(mocks.sendAndConfirm).not.toHaveBeenCalled();
    expect(mocks.simulateUnsigned.mock.invocationCallOrder[0]).toBeLessThan(mocks.sign.mock.invocationCallOrder[0]);
    expect(mocks.sign.mock.invocationCallOrder[0]).toBeLessThan(mocks.simulateSigned.mock.invocationCallOrder[0]);
  });

  test('submits without a duplicate RPC preflight after both explicit simulations pass', async () => {
    await sendTransaction(config, payer, [], 100_000);

    expect(mocks.sendAndConfirm).toHaveBeenCalledWith('signed-transaction', {
      commitment: 'confirmed',
      skipPreflight: true,
    });
    expect(mocks.simulateSigned.mock.invocationCallOrder[0]).toBeLessThan(mocks.sendAndConfirm.mock.invocationCallOrder[0]);
  });
});
