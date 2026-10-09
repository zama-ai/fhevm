import { SYSTEM_PROGRAM_ADDRESS } from '@solana-program/system';
import { beforeEach, describe, expect, test, vi } from 'vitest';
import { generateKeyPairSigner, getCompiledTransactionMessageDecoder, decompileTransactionMessage, type Blockhash } from '@solana/kit';
import {
  CLOSE_TRANSIENT_STORE_DISCRIMINATOR,
  OPEN_TRANSIENT_STORE_DISCRIMINATOR,
  ZAMA_HOST_PROGRAM_ADDRESS,
} from '@fhevm/solana-zama-host';

const mocks = vi.hoisted(() => ({
  encryptValues: vi.fn(),
  buildWrap: vi.fn(),
  buildInitialize: vi.fn(),
  send: vi.fn(),
  joinBatch: vi.fn(),
  readHandle: vi.fn(),
}));

const rpc = {
  getLatestBlockhash: vi.fn(() => ({ send: async () => ({ value: { blockhash: "11111111111111111111111111111111" as Blockhash, lastValidBlockHeight: 1_000n } }) })),
  getAccountInfo: vi.fn(() => ({ send: vi.fn().mockResolvedValue({ value: null }) })),
  simulateTransaction: vi.fn(() => ({
    send: async () => ({ value: { err: null, logs: [], unitsConsumed: 200_000n, loadedAccountsDataSize: 100_000 } }),
  })),
};

vi.mock('@fhevm/solana-zama-host', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@fhevm/solana-zama-host')>()),
  createFinalizedRpc: () => rpc,
}));
vi.mock('@solana/kit', async (importOriginal) => ({
  ...await importOriginal<typeof import('@solana/kit')>(),
  createSolanaRpcSubscriptions: () => ({}),
  sendAndConfirmTransactionFactory: () => mocks.send,
}));

vi.mock('@fhevm/sdk/solana', async (importOriginal) => ({
  ...await importOriginal<typeof import('@fhevm/sdk/solana')>(),
  createFhevmEncryptClient: () => ({
    encryptValues: mocks.encryptValues,
  }),
  defineFhevmSolanaChain: (chain: unknown) => chain,
  setFhevmRuntimeConfig: vi.fn(),
}));

vi.mock('./vault/index.js', () => ({
  TOKEN_PROGRAM_ADDRESS: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  buildInitializeTokenAccountInstruction: mocks.buildInitialize,
  buildWrapUsdcInstruction: mocks.buildWrap,
  deriveBatchAddresses: vi.fn(),
  findJoinRecordPda: vi.fn().mockResolvedValue(['11111111111111111111111111111111', 255]),
  getBatchByIndex: vi.fn(),
  getBatcher: vi.fn(),
  getCurrentBatch: vi.fn().mockResolvedValue({
    index: 1n,
    addresses: { batch: '11111111111111111111111111111111' },
    state: { status: 0 },
  }),
  getJoinRecord: vi.fn(),
  joinBatch: mocks.joinBatch,
}));

vi.mock('./encryptionKey', () => ({
  loadDemoEncryptionKey: vi.fn().mockResolvedValue(new Uint8Array()),
}));
vi.mock('./evidenceStore', () => ({ recordTransactionEvidence: vi.fn() }));
vi.mock('./revealShares', () => ({ readClaimedUsdcHandle: mocks.readHandle }));
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
    mints: { joinConfidential: '11111111111111111111111111111111' },
    programs: { token: '11111111111111111111111111111111', host: ZAMA_HOST_PROGRAM_ADDRESS },
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
  mocks.encryptValues.mockResolvedValue({ inputProof: { proof: true }, result: true });
  mocks.joinBatch.mockResolvedValue(undefined);
});

describe('direct cUSDC deposit', () => {
  test('rejects a stale handle before creating a proof', async () => {
    mocks.readHandle.mockResolvedValue('0xstale');

    await expect(depositToVault(session, 5, vi.fn(), undefined, 'cusdc', '0xexpected')).rejects.toThrow(
      'Reveal it again',
    );

    expect(mocks.encryptValues).not.toHaveBeenCalled();
    expect(mocks.joinBatch).not.toHaveBeenCalled();
  });

  test('rejects a handle change after proof creation and before joining', async () => {
    mocks.readHandle.mockResolvedValueOnce('0xexpected').mockResolvedValueOnce('0xchanged');

    await expect(depositToVault(session, 5, vi.fn(), undefined, 'cusdc', '0xexpected')).rejects.toThrow(
      'Reveal it again',
    );

    expect(mocks.encryptValues).toHaveBeenCalledOnce();
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


describe('public USDC deposit', () => {
  test('initializes the join account and wraps in one transaction context', async () => {
    const signer = await generateKeyPairSigner();
    const init = { programAddress: SYSTEM_PROGRAM_ADDRESS, data: new Uint8Array([1]) };
    const wrap = { ...init, data: new Uint8Array([2]) };
    mocks.buildInitialize.mockResolvedValue(init);
    mocks.buildWrap.mockResolvedValue(wrap);
    await depositToVault({ ...session, signer }, 5, vi.fn());
    expect(mocks.send).toHaveBeenCalledOnce();
    const sent = mocks.send.mock.calls[0]![0];
    const message = decompileTransactionMessage(getCompiledTransactionMessageDecoder().decode(sent.messageBytes));
    // All FHE work shares one exact lifecycle; v1 carries the compute limit in the message.
    expect(message.version).toBe(1);
    const body = [...message.instructions];
    expect([...body[0]!.data!]).toEqual([...OPEN_TRANSIENT_STORE_DISCRIMINATOR]);
    expect(body.slice(1, -1).map(ix => [...ix.data!])).toEqual([[1], [2]]);
    expect([...body.at(-1)!.data!]).toEqual([...CLOSE_TRANSIENT_STORE_DISCRIMINATOR]);
    const initContext = mocks.buildInitialize.mock.calls[0]![0].transientStore;
    const wrapContext = mocks.buildWrap.mock.calls[0]![0].transientStore;
    expect(initContext).toEqual(wrapContext);
    expect(body[0]!.accounts?.[1]?.address).toBe(wrapContext.address);
    expect(body.at(-1)!.accounts?.[1]?.address).toBe(wrapContext.address);
    expect(mocks.joinBatch).toHaveBeenCalledOnce();
  });
});
