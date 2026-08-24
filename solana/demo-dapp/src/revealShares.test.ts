import { beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  confidentialBalanceValueAccount: vi.fn(),
  createFhevmDecryptClient: vi.fn(),
  decryptPosition: vi.fn(),
  getAccountInfo: vi.fn(),
  getEncryptedValueState: vi.fn(),
  signPermit: vi.fn(),
  tokenAccountAddress: vi.fn(),
}));

vi.mock('@solana/kit', () => ({
  createSolanaRpc: vi.fn(() => ({ getAccountInfo: mocks.getAccountInfo })),
  getAddressEncoder: vi.fn(() => ({ encode: () => new Uint8Array(32) })),
}));
vi.mock('@fhevm/sdk/solana', () => ({
  createFhevmDecryptClient: mocks.createFhevmDecryptClient,
  defineFhevmSolanaChain: vi.fn((chain) => chain),
  setFhevmRuntimeConfig: vi.fn(),
}));
vi.mock('./vault/index.js', () => ({
  confidentialBalanceValueAccount: mocks.confidentialBalanceValueAccount,
  decryptPosition: mocks.decryptPosition,
  getEncryptedValueState: mocks.getEncryptedValueState,
  tokenAccountAddress: mocks.tokenAccountAddress,
}));

import type { DemoSession } from './demoSession';
import { readDecryptionEvidence } from './evidenceStore';
import { clearPermitCache } from './permitCache';
import { hasConfidentialBalanceAccount, revealClaimedShares } from './revealShares';

/** A sentinel the reveal hands to `signPermit` untouched; nothing in the test signs for real. */
const PERMIT_WALLET = {
  account: { address: 'A1iceWa11etAddress11111111111111111111111111', publicKey: new Uint8Array(32) },
  features: {},
};
/** The window covers the mocked clock below, so a second reveal may reuse the cached permit. */
const PERMIT_SESSION = {
  signedPermit: { fields: { startTimestamp: 1_699_999_000n, durationSeconds: 86_400n } },
  keyPair: {},
  warnings: [],
};

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
    proofServiceUrl: 'http://127.0.0.1:8080',
    aclProgram: `0x${'22'.repeat(32)}`,
    userDecryptContextId: '0',
    kmsSigners: [`0x${'01'.repeat(20)}`],
    kmsEpochId: `0x${'00'.repeat(32)}`,
    fheParameter: 'test',
    gatewayChainId: '31337',
    gatewayDecryptionContract: `0x${'aa'.repeat(20)}`,
    programs: { token: 'confidential-token-program' },
  },
  signer: {
    address: '11111111111111111111111111111111',
  },
  wallet: { kind: 'burner', name: 'Demo wallet' },
  permitWallet: PERMIT_WALLET,
} as unknown as DemoSession;

describe('confidential balance reveal evidence', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    clearPermitCache();
    const storage = new Map<string, string>();
    Object.assign(globalThis, {
      localStorage: {
        getItem: (key: string) => storage.get(key) ?? null,
        setItem: (key: string, value: string) => storage.set(key, value),
      },
    });
    mocks.signPermit.mockResolvedValue(PERMIT_SESSION);
    mocks.createFhevmDecryptClient.mockReturnValue({ ready: Promise.resolve(), signPermit: mocks.signPermit });
    mocks.tokenAccountAddress.mockResolvedValue('token-account');
    mocks.confidentialBalanceValueAccount.mockResolvedValue({
      aclValueKey: new Uint8Array(32),
      encryptedValueAddress: 'encrypted-value-account',
    });
    mocks.getEncryptedValueState.mockResolvedValue({ currentHandle: new Uint8Array(32).fill(0x12) });
  });

  test('records successful SDK correlation without persisting the clear value', async () => {
    mocks.decryptPosition.mockImplementation(async (_client, parameters) => {
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

    // The permit is minted once through the session's wallet, and the request runs under it.
    expect(mocks.signPermit).toHaveBeenCalledExactlyOnceWith({ wallet: PERMIT_WALLET, durationSeconds: 3_600n });
    const [client, parameters] = mocks.decryptPosition.mock.calls[0] ?? [];
    expect(client).toBe(mocks.createFhevmDecryptClient.mock.results[0]?.value);
    expect(parameters).toMatchObject({
      session: PERMIT_SESSION,
      entries: [{ handle: new Uint8Array(32).fill(0x12), encryptedValueId: new Uint8Array(32) }],
    });
  });

  // One wallet confirmation answers repeated views of the same balance: the permit is cached per
  // (wallet, domain, KMS route) and reused for its whole validity window.
  test('reuses one signed permit across two reveals of the same balance', async () => {
    mocks.decryptPosition.mockResolvedValue([{ value: 72n }]);
    vi.spyOn(Date, 'now').mockReturnValue(1_700_000_000_000);

    await expect(revealClaimedShares(session)).resolves.toEqual({ handle: `0x${'12'.repeat(32)}`, value: 72n });
    await expect(revealClaimedShares(session)).resolves.toEqual({ handle: `0x${'12'.repeat(32)}`, value: 72n });

    expect(mocks.decryptPosition).toHaveBeenCalledTimes(2);
    expect(mocks.signPermit).toHaveBeenCalledTimes(1);
  });

  // The permit channel is exclusive: a session kind that cannot provide the sRFC-38 wallet is told
  // to use the demo wallet, rather than falling back to raw message signing.
  test('refuses a session whose wallet cannot sign permits', async () => {
    const walletStandard = {
      ...(session as unknown as Record<string, unknown>),
      wallet: { kind: 'wallet-standard', name: 'Phantom', accountKey: 'a' },
      permitWallet: undefined,
    } as unknown as DemoSession;

    await expect(revealClaimedShares(walletStandard)).rejects.toThrow('solana:signOffchainMessage');
    expect(mocks.decryptPosition).not.toHaveBeenCalled();
  });
});

describe('confidential balance account discovery', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mocks.tokenAccountAddress.mockResolvedValue('token-account');
  });

  test('distinguishes absent and initialized canonical accounts', async () => {
    mocks.getAccountInfo.mockReturnValueOnce({
      send: vi.fn().mockResolvedValue({ value: null }),
    });
    await expect(
      hasConfidentialBalanceAccount(session, session.config.mints.joinConfidential),
    ).resolves.toBe(false);

    mocks.getAccountInfo.mockReturnValueOnce({
      send: vi.fn().mockResolvedValue({ value: { owner: session.config.programs.token } }),
    });
    await expect(
      hasConfidentialBalanceAccount(session, session.config.mints.joinConfidential),
    ).resolves.toBe(true);
  });

  test('rejects a canonical account owned by another program', async () => {
    mocks.getAccountInfo.mockReturnValue({
      send: vi.fn().mockResolvedValue({ value: { owner: 'unexpected-program' } }),
    });
    await expect(
      hasConfidentialBalanceAccount(session, session.config.mints.joinConfidential),
    ).rejects.toThrow('unexpected program');
  });
});
