import { address, type Signature } from '@solana/kit';
import { beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  deriveBatchAddresses: vi.fn(),
  deriveJoinRecordAddress: vi.fn(),
  getAccountInfo: vi.fn(),
  getCurrentBatch: vi.fn(),
  getJoinRecord: vi.fn(),
  getSignatureStatuses: vi.fn(),
}));

vi.mock('@solana/kit', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@solana/kit')>()),
  createSolanaRpc: () => ({
    getAccountInfo: mocks.getAccountInfo,
    getSignatureStatuses: mocks.getSignatureStatuses,
  }),
}));
vi.mock('@fhevm/sdk/solana/vault', () => ({
  deriveBatchAddresses: mocks.deriveBatchAddresses,
  deriveJoinRecordAddress: mocks.deriveJoinRecordAddress,
  getCurrentBatch: mocks.getCurrentBatch,
  getJoinRecord: mocks.getJoinRecord,
}));
vi.mock('./vaultRoots', () => ({ vaultRoots: () => ({}) }));

import type { DemoSession } from './demoSession';
import { readTransactionEvidence } from './evidenceStore';
import {
  assertBalanceHandleIsCurrent,
  findCompletedRedeem,
  findExistingRedeem,
  joinRedeemBatch,
  redactRedeemPosition,
} from './redeem';

const nextBatch = address('SysvarRent111111111111111111111111111111111');
const joinRecord = address('SysvarC1ock11111111111111111111111111111111');
const transactionSignature =
  '5h6xBEauJ3PK6WsSJZuHmEdmHdGzXJnbccQkWc9s3E8fPJ8mgLPJgGbu49Bv4J2M5z7yV1ycK4XoLZ4qsVQxHzDP' as Signature;

describe('redeem recovery', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    const storage = new Map<string, string>();
    Object.assign(globalThis, {
      localStorage: {
        getItem: (key: string) => storage.get(key) ?? null,
        removeItem: (key: string) => storage.delete(key),
        setItem: (key: string, value: string) => storage.set(key, value),
      },
    });
  });

  test('redacts the confidential amount from persisted and recovered positions', () => {
    expect(redactRedeemPosition({ batchIndex: 8n, batch: nextBatch })).toEqual({
      batchIndex: 8n,
      batch: nextBatch,
      amountBaseUnits: 0n,
    });
  });

  test('rejects a decrypted amount after its encrypted handle changes', () => {
    expect(() => assertBalanceHandleIsCurrent('0xold', '0xnew')).toThrow('private share balance changed');
    expect(() => assertBalanceHandleIsCurrent('0xsame', '0xsame')).not.toThrow();
  });

  test('discards a claimed saved join without reconciling its confirmed transaction again', async () => {
    const signer = address('11111111111111111111111111111111');
    const session = {
      config: {
        demoBootId: 'test-boot',
        chainId: 'localnet',
        batchers: { redeem: { batcher: nextBatch } },
      },
      signer: { address: signer },
    } as unknown as DemoSession;
    localStorage.setItem(
      `fhevm-solana-demo:active-redeem:localnet:${nextBatch}:${signer}`,
      JSON.stringify({
        batchIndex: '1',
        batch: nextBatch,
        amountBaseUnits: '0',
        transaction: {
          signature: 'confirmed-signature',
          blockhash: 'blockhash',
          lastValidBlockHeight: '999',
        },
      }),
    );
    mocks.getCurrentBatch.mockResolvedValue({ index: 1n });
    mocks.deriveBatchAddresses.mockResolvedValue({ batch: nextBatch });
    mocks.deriveJoinRecordAddress.mockResolvedValue(joinRecord);
    mocks.getAccountInfo.mockReturnValue({ send: async () => ({ value: {} }) });
    mocks.getJoinRecord.mockResolvedValue({ claimed: true });

    await expect(findExistingRedeem(session)).resolves.toBe(null);
    expect(mocks.getSignatureStatuses).not.toHaveBeenCalled();
    expect(localStorage.getItem(`fhevm-solana-demo:active-redeem:localnet:${nextBatch}:${signer}`)).toBe(null);
    expect(localStorage.getItem(`fhevm-solana-demo:completed-redeem:localnet:${nextBatch}:${signer}`)).not.toBe(null);
  });

  test('records an ambiguously confirmed redeem when its unclaimed join is recovered', async () => {
    const signer = address('11111111111111111111111111111111');
    const session = {
      config: {
        demoBootId: 'test-boot',
        chainId: 'localnet',
        batchers: { redeem: { batcher: nextBatch } },
      },
      signer: { address: signer },
    } as unknown as DemoSession;
    localStorage.setItem(
      `fhevm-solana-demo:active-redeem:localnet:${nextBatch}:${signer}`,
      JSON.stringify({
        batchIndex: '1',
        batch: nextBatch,
        amountBaseUnits: '0',
        transaction: {
          signature: transactionSignature,
          blockhash: 'blockhash',
          lastValidBlockHeight: '999',
        },
      }),
    );
    mocks.getCurrentBatch.mockResolvedValue({ index: 1n });
    mocks.deriveBatchAddresses.mockResolvedValue({ batch: nextBatch });
    mocks.deriveJoinRecordAddress.mockResolvedValue(joinRecord);
    mocks.getAccountInfo.mockReturnValue({ send: async () => ({ value: {} }) });
    mocks.getJoinRecord.mockResolvedValue({ claimed: false });

    await expect(findExistingRedeem(session)).resolves.toEqual({
      batchIndex: 1n,
      batch: nextBatch,
      amountBaseUnits: 0n,
    });
    expect(readTransactionEvidence(session)).toEqual([{ label: 'Redeem', signature: transactionSignature }]);
  });

  test('records an ambiguously confirmed redeem when the in-page action retries', async () => {
    const signer = address('11111111111111111111111111111111');
    const session = {
      config: {
        demoBootId: 'test-boot',
        chainId: 'localnet',
        rpcUrl: 'http://127.0.0.1:8899',
        wsUrl: 'ws://127.0.0.1:8900',
        batchers: { redeem: { batcher: nextBatch } },
      },
      signer: { address: signer },
      assertActive: vi.fn(),
    } as unknown as DemoSession;
    localStorage.setItem(
      `fhevm-solana-demo:active-redeem:localnet:${nextBatch}:${signer}`,
      JSON.stringify({
        batchIndex: '1',
        batch: nextBatch,
        amountBaseUnits: '0',
        transaction: {
          signature: transactionSignature,
          blockhash: 'blockhash',
          lastValidBlockHeight: '999',
        },
      }),
    );
    mocks.deriveBatchAddresses.mockResolvedValue({ batch: nextBatch });
    mocks.deriveJoinRecordAddress.mockResolvedValue(joinRecord);
    mocks.getAccountInfo.mockReturnValue({ send: async () => ({ value: {} }) });
    mocks.getJoinRecord.mockResolvedValue({ claimed: false });

    await expect(joinRedeemBatch(session, 1n, '0xhandle', vi.fn())).resolves.toEqual({
      batchIndex: 1n,
      batch: nextBatch,
      amountBaseUnits: 0n,
    });
    expect(readTransactionEvidence(session)).toEqual([{ label: 'Redeem', signature: transactionSignature }]);
  });

  test('preserves completed activity when its RPC validation fails transiently', async () => {
    const signer = address('11111111111111111111111111111111');
    const session = {
      config: {
        demoBootId: 'test-boot',
        chainId: 'localnet',
        batchers: { redeem: { batcher: nextBatch } },
      },
      signer: { address: signer },
    } as unknown as DemoSession;
    const key = `fhevm-solana-demo:completed-redeem:localnet:${nextBatch}:${signer}`;
    localStorage.setItem(key, JSON.stringify({ batchIndex: '1', batch: nextBatch }));
    mocks.deriveBatchAddresses.mockResolvedValue({ batch: nextBatch });
    mocks.deriveJoinRecordAddress.mockResolvedValue(joinRecord);
    mocks.getAccountInfo.mockReturnValue({ send: async () => Promise.reject(new Error('RPC unavailable')) });

    await expect(findCompletedRedeem(session)).rejects.toThrow('RPC unavailable');
    expect(localStorage.getItem(key)).not.toBe(null);
  });
});
