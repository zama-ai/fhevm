import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { Auth } from '../types/auth.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { encryptModule } from '../modules/encrypt/mock.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import { fetchFheEncryptionKeyWasm } from './fetchFheEncryptionKey.js';

// The key check is not under test here (see verifyFheEncryptionKeyBytes-p.test.ts).
vi.mock('./verifyFheEncryptionKeyBytes-p.js', async (importOriginal) => ({
  ...(await importOriginal<typeof import('./verifyFheEncryptionKeyBytes-p.js')>()),
  assertKeyVerificationConfig: () => undefined,
  verifyFheEncryptionKeyBytes: async (_context: unknown, bytes: FheEncryptionKeyBytes) => bytes,
}));

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/key/fetchFheEncryptionKey.test.ts
////////////////////////////////////////////////////////////////////////////////

const chain = { id: 12345, fhevm: { relayerUrl: 'https://relayer.test' } } as unknown as FhevmChain;
const fhevmContext = createFhevmClientFrozenContext({});

const runtimeAuth: Auth = { type: 'ApiKeyHeader', value: 'runtime-secret' };
// Stop after the download: the key is not under test.
const stopAfterDownload = () => Promise.reject(new Error('stop after the download'));
const fetchFheEncryptionKeyBytes = vi.fn<RelayerModule['fetchFheEncryptionKeyBytes']>(stopAfterDownload);
const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
  ethereum: {} as EthereumModule,
  relayer: { fetchFheEncryptionKeyBytes } as unknown as RelayerModule,
  config: { auth: runtimeAuth },
}).extend(encryptModule);
const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain, runtime });

beforeEach(() => {
  globalFheEncryptionKeyCache.clear();
  fetchFheEncryptionKeyBytes.mockReset();
  fetchFheEncryptionKeyBytes.mockImplementation(stopAfterDownload);
});

describe('fetchFheEncryptionKeyWasm', () => {
  it('sends the runtime auth when it starts the key download', async () => {
    await expect(fetchFheEncryptionKeyWasm(client, { fhevmContext })).rejects.toThrow('stop after the download');
    expect(fetchFheEncryptionKeyBytes).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({ options: { auth: runtimeAuth } }),
    );
  });

  it('gives a second runtime the key another runtime loaded, from one download', async () => {
    const keyBytes = {
      publicKeyBytes: { id: 'key-id', bytes: new Uint8Array([1, 2, 3]) },
      crsBytes: { id: 'crs-id', capacity: 2048, bytes: new Uint8Array([4, 5, 6]) },
      metadata: { relayerUrl: chain.fhevm.relayerUrl, chainId: chain.id },
    } as unknown as FheEncryptionKeyBytes;
    fetchFheEncryptionKeyBytes.mockImplementationOnce(async () => keyBytes);
    const otherRuntime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
      ethereum: {} as EthereumModule,
      relayer: { fetchFheEncryptionKeyBytes } as unknown as RelayerModule,
      config: {},
    }).extend(encryptModule);

    const key = await fetchFheEncryptionKeyWasm(client, { fhevmContext });
    expect(
      await fetchFheEncryptionKeyWasm(createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain, runtime: otherRuntime }), {
        fhevmContext,
      }),
    ).toBe(key);
    expect(fetchFheEncryptionKeyBytes).toHaveBeenCalledTimes(1);
  });

  it('sends the caller auth instead of the runtime auth when the caller passes one', async () => {
    const callerAuth: Auth = { type: 'BearerToken', token: 'caller-token' };

    await expect(fetchFheEncryptionKeyWasm(client, { fhevmContext, options: { auth: callerAuth } })).rejects.toThrow(
      'stop after the download',
    );
    expect(fetchFheEncryptionKeyBytes).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({ options: { auth: callerAuth } }),
    );
  });
});
