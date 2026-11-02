import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import { readFileSync } from 'node:fs';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { encryptModule } from '../modules/encrypt/module/index.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';
import { fetchFheEncryptionKeyBytes } from './fetchFheEncryptionKeyBytes.js';
import { fetchFheEncryptionKeyWasm } from './fetchFheEncryptionKey.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/key/fetchFheEncryptionKeyBytes.test.ts
////////////////////////////////////////////////////////////////////////////////

const relayerUrl = 'https://relayer.test';
const chain = { id: 12345, fhevm: { relayerUrl } } as unknown as FhevmChain;
const fhevmContext = createFhevmClientFrozenContext({});

type FixtureVersion = '1.5.4' | '1.6.1';
const fixtures = new Map<FixtureVersion, FheEncryptionKeyBytes>();

/** Key and CRS from `test/keys`. Read once per version. */
function loadFixture(tfheVersion: FixtureVersion): FheEncryptionKeyBytes {
  let keyBytes = fixtures.get(tfheVersion);
  if (keyBytes === undefined) {
    const json = JSON.parse(
      readFileSync(new URL(`../../../test/keys/key.${tfheVersion}.json`, import.meta.url), 'utf-8'),
    ) as {
      publicKeyBytes: { id: string; bytes: string };
      crsBytes: { id: string; capacity: number; bytes: string };
    };
    keyBytes = {
      publicKeyBytes: {
        id: json.publicKeyBytes.id,
        bytes: Uint8Array.from(Buffer.from(json.publicKeyBytes.bytes, 'base64')),
      },
      crsBytes: {
        id: json.crsBytes.id,
        capacity: json.crsBytes.capacity,
        bytes: Uint8Array.from(Buffer.from(json.crsBytes.bytes, 'base64')),
      },
      metadata: { relayerUrl, chainId: chain.id },
    } as unknown as FheEncryptionKeyBytes;
    fixtures.set(tfheVersion, keyBytes);
  }
  return keyBytes;
}

/** A fresh copy of the fixture, as the relayer serves it on each download. */
function downloadKeyBytes(tfheVersion: FixtureVersion): FheEncryptionKeyBytes {
  const fixture = loadFixture(tfheVersion);
  return {
    ...fixture,
    publicKeyBytes: { ...fixture.publicKeyBytes, bytes: new Uint8Array(fixture.publicKeyBytes.bytes) },
    crsBytes: { ...fixture.crsBytes, bytes: new Uint8Array(fixture.crsBytes.bytes) },
  };
}

/** Byte for byte: a re-serialized pre-tfhe-rs-1.6 public key differs by one byte. */
function expectBytesAsServed(bytes: FheEncryptionKeyBytes, tfheVersion: FixtureVersion): void {
  const served = loadFixture(tfheVersion);
  expect(bytes.publicKeyBytes.bytes).toEqual(served.publicKeyBytes.bytes);
  // CRS: ~6 MB, too slow for a deep equal.
  expect(Buffer.from(bytes.crsBytes.bytes).equals(served.crsBytes.bytes)).toBe(true);
}

// One runtime for all tests: the encrypt WASM module belongs to a single runtime.
const downloadFn = vi.fn<() => Promise<FheEncryptionKeyBytes>>();
const context: { chain: FhevmChain; runtime: WithEncrypt } = {
  chain,
  runtime: createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum: {} as EthereumModule,
    relayer: { fetchFheEncryptionKeyBytes: downloadFn } as unknown as RelayerModule,
    config: { singleThread: true },
  }).extend(encryptModule),
};

/** A download that settles when the test says so. */
function makePendingDownload() {
  let succeed!: (bytes: FheEncryptionKeyBytes) => void;
  let fail!: (error: Error) => void;
  const promise = new Promise<FheEncryptionKeyBytes>((resolve, reject) => {
    succeed = resolve;
    fail = reject;
  });
  downloadFn.mockImplementation(() => promise);
  return { succeed, fail };
}

beforeEach(() => {
  globalFheEncryptionKeyCache.clear();
  downloadFn.mockReset();
});

////////////////////////////////////////////////////////////////////////////////
// Bytes as served after the key is loaded for encryption (planning-blockchain#1757)
////////////////////////////////////////////////////////////////////////////////

describe('fetchFheEncryptionKeyBytes', () => {
  it.each([
    { tfheVersion: '1.5.4', format: 'pre-tfhe-rs-1.6' },
    { tfheVersion: '1.6.1', format: 'tfhe-rs 1.6' },
  ] as const)(
    'returns the key bytes as served before and after the key is loaded (tfhe-rs $tfheVersion key, $format format)',
    async ({ tfheVersion }) => {
      downloadFn.mockImplementation(async () => downloadKeyBytes(tfheVersion));

      expectBytesAsServed(await fetchFheEncryptionKeyBytes(context, { fhevmContext }), tfheVersion);
      await fetchFheEncryptionKeyWasm(context, { fhevmContext });
      expect(globalFheEncryptionKeyCache.get(relayerUrl)?.resolvedKind).toBe('wasm');
      expectBytesAsServed(await fetchFheEncryptionKeyBytes(context, { fhevmContext }), tfheVersion);
    },
  );

  it('returns the key bytes as served to a runtime without the encrypt module once another runtime loaded the key', async () => {
    downloadFn.mockImplementation(async () => downloadKeyBytes('1.5.4'));
    await fetchFheEncryptionKeyWasm(context, { fhevmContext });
    expect(globalFheEncryptionKeyCache.get(relayerUrl)?.resolvedKind).toBe('wasm');

    const runtimeWithoutEncrypt = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
      ethereum: {} as EthereumModule,
      relayer: { fetchFheEncryptionKeyBytes: downloadFn } as unknown as RelayerModule,
      config: {},
    });
    expectBytesAsServed(
      await fetchFheEncryptionKeyBytes({ chain, runtime: runtimeWithoutEncrypt }, { fhevmContext }),
      '1.5.4',
    );
  });

  it('returns the key bytes as served to every concurrent caller while the key is loading, from one download', async () => {
    const pending = makePendingDownload();

    const bytesCalls = [
      fetchFheEncryptionKeyBytes(context, { fhevmContext }),
      fetchFheEncryptionKeyBytes(context, { fhevmContext }),
    ];
    const wasmCall = fetchFheEncryptionKeyWasm(context, { fhevmContext });
    pending.succeed(downloadKeyBytes('1.5.4'));

    for (const bytes of await Promise.all(bytesCalls)) {
      expectBytesAsServed(bytes, '1.5.4');
    }
    await wasmCall;
    expect(downloadFn).toHaveBeenCalledTimes(1);
  });

  it('gives every concurrent caller the download error when the download fails while the key is loading, then downloads again', async () => {
    const downloadError = new Error('download failed');
    const pending = makePendingDownload();

    const bytesCall = fetchFheEncryptionKeyBytes(context, { fhevmContext });
    const wasmCall = fetchFheEncryptionKeyWasm(context, { fhevmContext });
    pending.fail(downloadError);
    await expect(bytesCall).rejects.toBe(downloadError);
    await expect(wasmCall).rejects.toBe(downloadError);

    downloadFn.mockImplementation(async () => downloadKeyBytes('1.5.4'));
    expectBytesAsServed(await fetchFheEncryptionKeyBytes(context, { fhevmContext }), '1.5.4');
    expect(downloadFn).toHaveBeenCalledTimes(2);
  });

  it('gives a bytes-only caller the download error, without leaving it unhandled', async () => {
    const downloadError = new Error('download failed');
    downloadFn.mockRejectedValue(downloadError);

    const unhandledRejection = vi.fn();
    process.on('unhandledRejection', unhandledRejection);
    try {
      await expect(fetchFheEncryptionKeyBytes(context, { fhevmContext })).rejects.toBe(downloadError);
      // Node reports unhandled rejections after the microtask queue drains.
      await new Promise((resolve) => setTimeout(resolve, 0));
      expect(unhandledRejection).not.toHaveBeenCalled();
    } finally {
      process.off('unhandledRejection', unhandledRejection);
    }
  });
});
