import { beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  confidentialBalanceValueAccount: vi.fn(),
  createFhevmDecryptClient: vi.fn(),
  decryptPosition: vi.fn(),
  getEncryptedValueState: vi.fn(),
  tokenAccountAddress: vi.fn(),
}));

vi.mock('@solana/kit', () => ({
  createSolanaRpc: vi.fn(() => ({})),
  getAddressEncoder: vi.fn(() => ({ encode: () => new Uint8Array(32) })),
}));
vi.mock('@fhevm/sdk/solana', () => ({
  createFhevmDecryptClient: mocks.createFhevmDecryptClient,
  defineFhevmSolanaChain: vi.fn((chain) => chain),
  setFhevmRuntimeConfig: vi.fn(),
}));
vi.mock('@fhevm/sdk/solana/vault', () => ({
  confidentialBalanceValueAccount: mocks.confidentialBalanceValueAccount,
  decryptPosition: mocks.decryptPosition,
  getEncryptedValueState: mocks.getEncryptedValueState,
  tokenAccountAddress: mocks.tokenAccountAddress,
}));

import type { DemoSession } from './demoSession';
import { readDecryptionEvidence } from './evidenceStore';
import { revealClaimedShares } from './revealShares';

const session = {
  config: {
    chainId: '31337',
    demoBootId: 'boot-a',
    mints: {
      joinConfidential: '11111111111111111111111111111111',
      payoutConfidential: '11111111111111111111111111111111',
    },
    relayerUrl: 'http://127.0.0.1:3000',
    rpcUrl: 'http://127.0.0.1:8899',
    userDecryptContextId: '0',
  },
  signer: {
    address: '11111111111111111111111111111111',
    signMessageExact: vi.fn(),
  },
} as unknown as DemoSession;

describe('confidential balance reveal evidence', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    const storage = new Map<string, string>();
    Object.assign(globalThis, {
      localStorage: {
        getItem: (key: string) => storage.get(key) ?? null,
        setItem: (key: string, value: string) => storage.set(key, value),
      },
    });
    mocks.createFhevmDecryptClient.mockReturnValue({ ready: Promise.resolve(), runtime: {} });
    mocks.tokenAccountAddress.mockResolvedValue('token-account');
    mocks.confidentialBalanceValueAccount.mockResolvedValue({
      aclValueKey: new Uint8Array(32),
      encryptedValueAddress: 'encrypted-value-account',
    });
    mocks.getEncryptedValueState.mockResolvedValue({ currentHandle: new Uint8Array(32).fill(0x12) });
  });

  test('records successful SDK correlation without persisting the clear value', async () => {
    mocks.decryptPosition.mockImplementation(async (_client, _signer, parameters) => {
      parameters.options.onProgress({
        type: 'queued',
        method: 'POST',
        jobId: 'job-1',
        requestId: 'post-request',
        elapsed: 50,
      });
      parameters.options.onProgress({
        type: 'succeeded',
        jobId: 'job-1',
        requestId: 'get-request',
        elapsed: 1_250,
      });
      return [{ value: 72n }];
    });
    vi.spyOn(performance, 'now')
      .mockReturnValueOnce(1_000)
      .mockReturnValueOnce(1_100)
      .mockReturnValueOnce(2_300)
      .mockReturnValueOnce(2_500);
    vi.spyOn(Date, 'now').mockReturnValue(1_700_000_000_000);

    await expect(revealClaimedShares(session)).resolves.toEqual({
      handle: `0x${'12'.repeat(32)}`,
      value: 72n,
    });
    expect(readDecryptionEvidence(session)).toEqual([
      {
        completedAt: 1_700_000_000_000,
        handle: `0x${'12'.repeat(32)}`,
        jobId: 'job-1',
        label: 'cShares',
        queueToResponseMs: 1_200,
        totalElapsedMs: 1_500,
      },
    ]);
    expect(JSON.stringify(readDecryptionEvidence(session))).not.toContain('72');
  });
});
