import { beforeEach, describe, expect, test, vi } from 'vitest';

const mocks = vi.hoisted(() => ({
  buildInputProof: vi.fn(),
  buildWrap: vi.fn(),
  joinBatch: vi.fn(),
  readHandle: vi.fn(),
  submitInputProof: vi.fn(),
}));

const rpc = {
  getAccountInfo: vi.fn(() => ({ send: vi.fn().mockResolvedValue({ value: null }) })),
};

vi.mock('@solana/kit', () => ({
  createSolanaRpc: () => rpc,
  createSolanaRpcSubscriptions: () => ({}),
  getAddressEncoder: () => ({ encode: () => new Uint8Array(32) }),
  sendAndConfirmTransactionFactory: vi.fn(),
}));

vi.mock('@fhevm/sdk/solana', () => ({
  createFhevmEncryptClient: () => ({
    buildInputProof: mocks.buildInputProof,
    submitInputProof: mocks.submitInputProof,
  }),
  defineFhevmSolanaChain: (chain: unknown) => chain,
  setFhevmRuntimeConfig: vi.fn(),
}));

vi.mock('@fhevm/sdk/solana/vault', () => ({
  buildInitializeTokenAccountInstruction: vi.fn(),
  buildWrapUsdcInstruction: mocks.buildWrap,
  computeSignerAddress: vi.fn().mockResolvedValue('11111111111111111111111111111111'),
  deriveBatchAddresses: vi.fn(),
  deriveJoinRecordAddress: vi.fn().mockResolvedValue('11111111111111111111111111111111'),
  getBatchByIndex: vi.fn(),
  getBatcher: vi.fn(),
  getCurrentBatch: vi.fn().mockResolvedValue({
    index: 1n,
    addresses: { batch: '11111111111111111111111111111111' },
    state: { status: 0 },
  }),
  getJoinRecord: vi.fn(),
  joinBatch: mocks.joinBatch,
  tokenAccountAddress: vi.fn(),
}));

vi.mock('./encryptionKey', () => ({
  loadDemoEncryptionKey: vi.fn().mockResolvedValue(new Uint8Array()),
}));
vi.mock('./evidenceStore', () => ({ recordTransactionEvidence: vi.fn() }));
vi.mock('./revealShares', () => ({ readClaimedUsdcHandle: mocks.readHandle }));
vi.mock('./transactionSimulation', () => ({
  simulateSignedTransactionLocally: vi.fn(),
  simulateUnsignedTransactionLocally: vi.fn(),
}));
vi.mock('./vaultRoots', () => ({
  vaultRoots: () => ({
    batcher: '11111111111111111111111111111111',
    joinConfidentialMint: '11111111111111111111111111111111',
  }),
}));

import type { DemoSession } from './demoSession';
import { depositToVault } from './deposit';

const session = {
  config: {
    chainId: '2147483648',
    rpcUrl: 'http://127.0.0.1:8899',
    wsUrl: 'ws://127.0.0.1:8900',
    relayerUrl: 'http://127.0.0.1:3000',
    aclProgram: '11111111111111111111111111111111',
    hostConfig: '11111111111111111111111111111111',
    mints: { joinConfidential: '11111111111111111111111111111111' },
    batchers: { deposit: { batcher: '11111111111111111111111111111111' } },
  },
  signer: { address: '11111111111111111111111111111111' },
  assertActive: vi.fn(),
} as unknown as DemoSession;

beforeEach(() => {
  vi.clearAllMocks();
  Object.assign(globalThis, {
    localStorage: {
      getItem: () => null,
      removeItem: vi.fn(),
      setItem: vi.fn(),
    },
  });
  mocks.buildInputProof.mockResolvedValue({ proof: true });
  mocks.submitInputProof.mockResolvedValue({ result: true });
  mocks.joinBatch.mockResolvedValue(undefined);
});

describe('direct cUSDC deposit', () => {
  test('rejects a stale handle before creating a proof', async () => {
    mocks.readHandle.mockResolvedValue('0xstale');

    await expect(depositToVault(session, 5, vi.fn(), undefined, 'cusdc', '0xexpected')).rejects.toThrow(
      'Reveal it again',
    );

    expect(mocks.buildInputProof).not.toHaveBeenCalled();
    expect(mocks.joinBatch).not.toHaveBeenCalled();
  });

  test('rejects a handle change after proof creation and before joining', async () => {
    mocks.readHandle.mockResolvedValueOnce('0xexpected').mockResolvedValueOnce('0xchanged');

    await expect(depositToVault(session, 5, vi.fn(), undefined, 'cusdc', '0xexpected')).rejects.toThrow(
      'Reveal it again',
    );

    expect(mocks.buildInputProof).toHaveBeenCalledOnce();
    expect(mocks.submitInputProof).toHaveBeenCalledOnce();
    expect(mocks.joinBatch).not.toHaveBeenCalled();
  });

  test('skips shielding and sends one join when the handle stays current', async () => {
    mocks.readHandle.mockResolvedValue('0xexpected');

    await expect(depositToVault(session, 5, vi.fn(), undefined, 'cusdc', '0xexpected')).resolves.toMatchObject({
      batchIndex: 1n,
      amountBaseUnits: 5_000_000n,
    });

    expect(mocks.readHandle).toHaveBeenCalledTimes(2);
    expect(mocks.buildWrap).not.toHaveBeenCalled();
    expect(mocks.joinBatch).toHaveBeenCalledOnce();
  });
});
