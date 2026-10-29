import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { Auth } from '../types/auth.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import { readFileSync } from 'node:fs';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { encryptModule } from '../modules/encrypt/module/index.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import { fetchFheEncryptionKeyWasm } from './fetchFheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/key/fetchFheEncryptionKey.test.ts
////////////////////////////////////////////////////////////////////////////////

const chain = { id: 12345, fhevm: { relayerUrl: 'https://relayer.test' } } as unknown as FhevmChain;
const fhevmContext = createFhevmClientFrozenContext({});

const runtimeAuth: Auth = { type: 'ApiKeyHeader', value: 'runtime-secret' };
// Stop after the download: the key is not under test.
const fetchFheEncryptionKeyBytes = vi.fn<RelayerModule['fetchFheEncryptionKeyBytes']>(() =>
  Promise.reject(new Error('stop after the download')),
);
const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
  ethereum: {} as EthereumModule,
  relayer: { fetchFheEncryptionKeyBytes } as unknown as RelayerModule,
  config: { auth: runtimeAuth },
}).extend(encryptModule);

beforeEach(() => {
  globalFheEncryptionKeyCache.clear();
  fetchFheEncryptionKeyBytes.mockClear();
});

describe('fetchFheEncryptionKeyWasm', () => {
  it('sends the runtime auth when it starts the key download', async () => {
    await expect(fetchFheEncryptionKeyWasm({ chain, runtime }, { fhevmContext })).rejects.toThrow(
      'stop after the download',
    );
    expect(fetchFheEncryptionKeyBytes).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({ options: { auth: runtimeAuth } }),
    );
  });

  it('gives a second runtime the key another runtime loaded, from one download', async () => {
    const json = JSON.parse(readFileSync(new URL('../../../test/keys/key.1.6.1.json', import.meta.url), 'utf-8')) as {
      publicKeyBytes: { id: string; bytes: string };
      crsBytes: { id: string; capacity: number; bytes: string };
    };
    const keyBytes = {
      publicKeyBytes: { id: json.publicKeyBytes.id, bytes: Buffer.from(json.publicKeyBytes.bytes, 'base64') },
      crsBytes: { ...json.crsBytes, bytes: Buffer.from(json.crsBytes.bytes, 'base64') },
      metadata: { relayerUrl: chain.fhevm.relayerUrl, chainId: chain.id },
    } as unknown as FheEncryptionKeyBytes;
    fetchFheEncryptionKeyBytes.mockImplementationOnce(async () => keyBytes);
    const otherRuntime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
      ethereum: {} as EthereumModule,
      relayer: { fetchFheEncryptionKeyBytes } as unknown as RelayerModule,
      config: {},
    }).extend(encryptModule);

    const key = await fetchFheEncryptionKeyWasm({ chain, runtime }, { fhevmContext });
    expect(await fetchFheEncryptionKeyWasm({ chain, runtime: otherRuntime }, { fhevmContext })).toBe(key);
    expect(fetchFheEncryptionKeyBytes).toHaveBeenCalledTimes(1);
  });

  it('sends the caller auth instead of the runtime auth when the caller passes one', async () => {
    const callerAuth: Auth = { type: 'BearerToken', token: 'caller-token' };

    await expect(
      fetchFheEncryptionKeyWasm({ chain, runtime }, { fhevmContext, options: { auth: callerAuth } }),
    ).rejects.toThrow('stop after the download');
    expect(fetchFheEncryptionKeyBytes).toHaveBeenCalledWith(
      expect.anything(),
      expect.objectContaining({ options: { auth: callerAuth } }),
    );
  });
});
