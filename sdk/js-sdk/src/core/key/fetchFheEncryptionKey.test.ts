import type { EncryptModuleFactory } from '../modules/encrypt/types.js';
import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { Auth } from '../types/auth.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { SolanaProofContext } from '../types/zkProofBuilder-p.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { encryptModule } from '../modules/encrypt/mock.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import { fetchFheEncryptionKeyWasm, fetchSolanaFheEncryptionKeyWasm } from './fetchFheEncryptionKey.js';

// The key check is not under test here (see verifyFheEncryptionKeyBytes-p.test.ts).
vi.mock('./verifyFheEncryptionKeyBytes-p.js', async (importOriginal) => ({
  ...(await importOriginal<typeof import('./verifyFheEncryptionKeyBytes-p.js')>()),
  resolveKmsGeneration: () => ({ chainId: 1, address: '0x0000000000000000000000000000000000000001' }),
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
// The mock encrypt module, counting the public keys it converts from bytes.
let publicKeyConversions = 0;
const countingEncryptModule: EncryptModuleFactory = (moduleRuntime) => {
  const { encrypt } = encryptModule(moduleRuntime);
  return {
    encrypt: {
      ...encrypt,
      deserializeFheEncryptionPublicKey: (parameters) => {
        publicKeyConversions += 1;
        return encrypt.deserializeFheEncryptionPublicKey(parameters);
      },
    },
  };
};
const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
  ethereum: {} as EthereumModule,
  relayer: { fetchFheEncryptionKeyBytes } as unknown as RelayerModule,
  config: { auth: runtimeAuth },
}).extend(countingEncryptModule);
const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain, runtime });

beforeEach(() => {
  globalFheEncryptionKeyCache.clear();
  publicKeyConversions = 0;
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

describe('fetchSolanaFheEncryptionKeyWasm', () => {
  const solanaChain = {
    id: 0x0100000000000001n,
    fhevm: {
      relayerUrl: 'https://solana-relayer.test',
      decryptionRelayerUrl: 'https://solana-decryption.test',
      programs: { host: { address: `0x${'22'.repeat(32)}` } },
    },
  } as SolanaProofContext['chain'];
  // A new object each call: two keys made from the same `seed` hold equal bytes.
  const keyBytes = (id: string, seed = id.length) =>
    ({
      publicKeyBytes: { id, bytes: new Uint8Array([seed, 2, 3]) },
      crsBytes: { id: `${id}-crs`, capacity: 2048, bytes: new Uint8Array([seed, 5, 6]) },
      metadata: { relayerUrl: solanaChain.fhevm.relayerUrl, chainId: solanaChain.id },
    }) as unknown as FheEncryptionKeyBytes;
  // A chainless core, as the Solana clients create theirs.
  const solanaContext = (fheEncryptionKey?: FheEncryptionKeyBytes): SolanaProofContext => ({
    fhevm: createCoreFhevm(PRIVATE_ETHERS_TOKEN, { runtime, options: { fheEncryptionKey } }),
    chain: solanaChain,
    aclProgramAddress: solanaChain.fhevm.programs.host.address,
    runtime,
  });

  it('uses the provided key without a download, and a later client its own provided key', async () => {
    const first = await fetchSolanaFheEncryptionKeyWasm(solanaContext(keyBytes('first')), { fhevmContext });
    const second = await fetchSolanaFheEncryptionKeyWasm(solanaContext(keyBytes('second')), { fhevmContext });

    expect(second).not.toBe(first);
    expect(fetchFheEncryptionKeyBytes).not.toHaveBeenCalled();
  });

  it('reuses the cached key for a provided key with the same bytes, and replaces it for other bytes', async () => {
    const first = await fetchSolanaFheEncryptionKeyWasm(solanaContext(keyBytes('key', 1)), { fhevmContext });
    const same = await fetchSolanaFheEncryptionKeyWasm(solanaContext(keyBytes('key', 1)), { fhevmContext });
    expect(same).toBe(first);
    expect(publicKeyConversions).toBe(1);

    const other = await fetchSolanaFheEncryptionKeyWasm(solanaContext(keyBytes('key', 2)), { fhevmContext });
    expect(other).not.toBe(first);
    expect(publicKeyConversions).toBe(2);
    expect(fetchFheEncryptionKeyBytes).not.toHaveBeenCalled();
  });

  it("downloads the chain relayer's key with the runtime auth when no key is provided", async () => {
    await expect(fetchSolanaFheEncryptionKeyWasm(solanaContext(), { fhevmContext })).rejects.toThrow(
      'stop after the download',
    );
    expect(fetchFheEncryptionKeyBytes).toHaveBeenCalledWith(
      { relayerUrl: solanaChain.fhevm.relayerUrl, chainId: solanaChain.id },
      expect.objectContaining({ options: { auth: runtimeAuth } }),
    );
  });
});
