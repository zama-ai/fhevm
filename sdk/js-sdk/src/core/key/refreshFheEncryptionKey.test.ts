import type { EthereumModule } from '../modules/ethereum/types.js';
import type { FetchFheEncryptionKeyBytesParameters, RelayerModule } from '../modules/relayer/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FheEncryptionKeyBytes, FheEncryptionKeySource } from '../types/fheEncryptionKey.js';
import type { FhevmCacheOptions } from '../types/cache.js';
import type { Bytes, BytesHex, UintNumber } from '../types/primitives.js';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { shake256 } from '@noble/hashes/sha3.js';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { bytesToHex } from '../base/bytes.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { invalidateVersionCache } from '../host-contracts/HostContractVersion-p.js';
import { fetchFheEncryptionKeyBytes } from './fetchFheEncryptionKeyBytes.js';
import {
  ensureFheEncryptionKeyDownload,
  fheEncryptionKeyCacheForTests,
  getFheEncryptionKeySlot,
  keyMaterialCache,
} from './ensureFheEncryptionKeyDownload.js';
import { RelayerResponseInputProofRejectedError } from '../errors/RelayerResponseInputProofRejectedError.js';
import { recordUsedCacheVersion } from '../caching/usedCacheVersions-p.js';
import { forceRefreshCachedValue, retryOnStaleCache } from '../caching/retryOnStaleCache-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/key/refreshFheEncryptionKey.test.ts
//
// Key material in the unified cache: per-client TTL, refresh after a stale
// key (rotation), and the cheap active-id check before any download.
////////////////////////////////////////////////////////////////////////////////

const id32 = (id: bigint): string => `0x${id.toString(16).padStart(64, '0')}`;

function digest(domain: string, bytes: Uint8Array): BytesHex {
  return bytesToHex(shake256.create({ dkLen: 32 }).update(new TextEncoder().encode(domain)).update(bytes).digest());
}

// Two key generations: the active one before and after a rotation.
type Generation = { readonly keyId: bigint; readonly crsId: bigint; readonly publicKey: Bytes; readonly crs: Bytes };
const GEN1: Generation = {
  keyId: (4n << 248n) | 1n,
  crsId: (5n << 248n) | 1n,
  publicKey: new Uint8Array([1, 1, 1]) as Bytes,
  crs: new Uint8Array([2, 2, 2]) as Bytes,
};
const GEN2: Generation = {
  keyId: (4n << 248n) | 2n,
  crsId: (5n << 248n) | 2n,
  publicKey: new Uint8Array([3, 3, 3]) as Bytes,
  crs: new Uint8Array([4, 4, 4]) as Bytes,
};
const version = (gen: Generation): string => `${id32(gen.keyId)}:${id32(gen.crsId)}`;

const RELAYER_URL = 'https://relayer.test';
const chain: FhevmChain = { ...sepolia, fhevm: { ...sepolia.fhevm, relayerUrl: RELAYER_URL } };
const fhevmContext = createFhevmClientFrozenContext({});

/**
 * A runtime whose chain (KMSGeneration) and relayer each serve one generation,
 * switched independently; the relayer serves 32-byte hex ids, or label ids.
 */
function makeWorld(options: { readonly labelIds?: boolean; readonly unverified?: boolean } = {}) {
  // `gate`: when set, a download waits for it after the active-id check (a slow download).
  // `failNextActiveIdRead`: the next read of the active key id fails (a flaky RPC).
  const state = {
    chain: GEN1,
    relayer: GEN1,
    rpcDown: false,
    failNextActiveIdRead: false,
    gate: undefined as Promise<void> | undefined,
    // When set, the bytes downloaded after the gate (a rotation between the
    // active-id check and the download).
    relayerAfterGate: undefined as Generation | undefined,
  };
  const readContract = vi.fn(async (_client: unknown, p: { readonly functionName: string }) => {
    await Promise.resolve();
    if (state.rpcDown) {
      throw new Error('RPC down');
    }
    switch (p.functionName) {
      case 'getVersion':
        return 'KMSGeneration v0.1.0';
      case 'getActiveKeyId':
        if (state.failNextActiveIdRead) {
          state.failNextActiveIdRead = false;
          throw new Error('RPC flaky');
        }
        return state.chain.keyId;
      case 'getActiveCrsId':
        return state.chain.crsId;
      case 'getKeyMaterials':
        return [[], [{ keyType: 1, digest: digest('PDAT_KEY', state.chain.publicKey) }]];
      case 'getCrsMaterials':
        return [[], digest('PDAT_CRS', state.chain.crs)];
      default:
        throw new Error(`No mocked handler for ${p.functionName}`);
    }
  });
  const downloads = vi.fn();
  const fetchFheEncryptionKeyBytesFn = vi.fn(
    async (_relayerClient: unknown, parameters: FetchFheEncryptionKeyBytesParameters) => {
      const gen = state.relayer;
      const source = {
        publicKeySource: { id: options.labelIds === true ? 'fhe-public-key-data-id' : id32(gen.keyId), url: 'k' },
        crsSource: { id: options.labelIds === true ? 'crs-data-id' : id32(gen.crsId), url: 'c', capacity: 2048 },
        metadata: { relayerUrl: RELAYER_URL, chainId: chain.id },
      } as unknown as FheEncryptionKeySource;
      await parameters.beforeDownload?.(source);
      await state.gate;
      downloads();
      const served = state.relayerAfterGate ?? gen;
      return {
        publicKeyBytes: { id: source.publicKeySource.id, bytes: served.publicKey },
        crsBytes: { id: source.crsSource.id, capacity: 2048 as UintNumber, bytes: served.crs },
        metadata: { relayerUrl: RELAYER_URL, chainId: chain.id },
      } as unknown as FheEncryptionKeyBytes;
    },
  );
  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum: { readContract } as unknown as EthereumModule,
    relayer: {
      fetchFheEncryptionKeyBytes: fetchFheEncryptionKeyBytesFn,
      // As the cleartext mock relayer: keys are not checked against KMSGeneration.
      skipsFheEncryptionKeyVerification: options.unverified === true,
    } as unknown as RelayerModule,
    config: {},
  });
  const newClient = (cache?: FhevmCacheOptions, fheEncryptionKey?: FheEncryptionKeyBytes) =>
    createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain, client: {}, runtime, options: { cache, fheEncryptionKey } });
  const activeIdReads = () =>
    readContract.mock.calls.filter((call) => (call[1] as { functionName: string }).functionName === 'getActiveKeyId')
      .length;
  return { state, runtime, newClient, downloads, activeIdReads };
}

type Client = ReturnType<ReturnType<typeof makeWorld>['newClient']>;
const fetchBytes = (client: Client) => fetchFheEncryptionKeyBytes(client, { fhevmContext });
const fetchVersion = async (client: Client) => (await ensureFheEncryptionKeyDownload(client, { fhevmContext })).version;
const refresh = async (client: Client, invalidVersion?: string) =>
  (await keyMaterialCache.refresh(client, { fhevmContext }, invalidVersion)).version;

beforeEach(() => {
  fheEncryptionKeyCacheForTests.clear();
  invalidateVersionCache({ includeInflight: true });
});

afterEach(() => {
  vi.useRealTimers();
});

describe('key material cache', () => {
  it('versions a download by the active ids, also when the relayer serves label ids', async () => {
    const { newClient } = makeWorld({ labelIds: true });
    expect(await fetchVersion(newClient())).toBe(version(GEN1));
  });

  it('applies each client its own TTL, and does not download again while the chain has the same key', async () => {
    vi.useFakeTimers();
    const { newClient, downloads, activeIdReads } = makeWorld();
    const short = newClient({ keyMaterial: { ttl: 1_000 } });
    const long = newClient();
    await fetchBytes(short);
    await fetchBytes(long);
    expect(downloads).toHaveBeenCalledTimes(1);
    const readsBefore = activeIdReads();

    vi.advanceTimersByTime(2_000);
    // Fresh for the long-TTL client: no read at all (its check is memoized).
    await fetchBytes(long);
    expect(activeIdReads()).toBe(readsBefore);
    // Expired for the short-TTL client: one cheap read, no download.
    await fetchBytes(short);
    expect(activeIdReads()).toBe(readsBefore + 1);
    expect(downloads).toHaveBeenCalledTimes(1);
  });

  it('with a TTL of 0, reads the active ids on every call and downloads only after a rotation', async () => {
    const { state, newClient, downloads, activeIdReads } = makeWorld();
    const client = newClient({ keyMaterial: { ttl: 0 } });
    await fetchBytes(client);
    const readsBefore = activeIdReads();
    await fetchBytes(client);
    expect(activeIdReads()).toBe(readsBefore + 1);
    expect(downloads).toHaveBeenCalledTimes(1);

    state.chain = GEN2;
    state.relayer = GEN2;
    expect(await fetchVersion(client)).toBe(version(GEN2));
    expect(downloads).toHaveBeenCalledTimes(2);
  });

  it('fails when the active ids cannot be read after the TTL, and keeps the cached key for the next call', async () => {
    vi.useFakeTimers();
    const { state, newClient, downloads } = makeWorld();
    const client = newClient({ keyMaterial: { ttl: 1_000 } });
    const before = await fetchBytes(client);
    vi.advanceTimersByTime(2_000);
    state.rpcDown = true;
    await expect(fetchBytes(client)).rejects.toThrow('RPC down');
    state.rpcDown = false;
    await expect(fetchBytes(client)).resolves.toEqual(before);
    expect(downloads).toHaveBeenCalledTimes(1);
  });

  it('label ids and a rotation between the id read and the download: versioned by the ids its digests match', async () => {
    const { state, newClient } = makeWorld({ labelIds: true });
    const client = newClient();
    let release: (() => void) | undefined;
    state.gate = new Promise<void>((resolve) => (release = resolve));
    state.relayerAfterGate = GEN2;
    const fetching = fetchVersion(client);
    await vi.waitFor(() => expect(release).toBeDefined());
    await new Promise((resolve) => setTimeout(resolve, 0));
    state.chain = GEN2;
    release?.();
    // Read as GEN1 before the download; the digests match GEN2.
    expect(await fetching).toBe(version(GEN2));
    expect(fheEncryptionKeyCacheForTests.peek(getFheEncryptionKeySlot(client))?.version).toBe(version(GEN2));
  });

  it('versions a provided key served with label ids by its check', async () => {
    const { newClient } = makeWorld();
    const provided = {
      publicKeyBytes: { id: 'fhe-public-key-data-id', bytes: GEN1.publicKey },
      crsBytes: { id: 'crs-data-id', capacity: 2048 as UintNumber, bytes: GEN1.crs },
      metadata: { relayerUrl: RELAYER_URL, chainId: chain.id },
    } as unknown as FheEncryptionKeyBytes;
    const checked = await ensureFheEncryptionKeyDownload(newClient(undefined, provided), { fhevmContext });
    expect(checked.version).toBe(version(GEN1));
  });

  describe('refresh after a key rotation', () => {
    it('downloads the new key once for concurrent refreshes, none when already replaced', async () => {
      const { state, newClient, downloads } = makeWorld();
      const client = newClient();
      const before = await fetchBytes(client);
      state.chain = GEN2;
      state.relayer = GEN2;

      await Promise.all([
        refresh(client, version(GEN1)),
        refresh(client, version(GEN1)),
        refresh(client, version(GEN1)),
      ]);
      expect(downloads).toHaveBeenCalledTimes(2);
      await refresh(client, version(GEN1));
      expect(downloads).toHaveBeenCalledTimes(2);

      expect(await fetchVersion(client)).toBe(version(GEN2));
      // A request that read the old key keeps it.
      expect(before.publicKeyBytes.bytes).toEqual(GEN1.publicKey);
    });

    it('fails without a download when the chain still has the cached key', async () => {
      const { newClient, downloads } = makeWorld();
      const client = newClient();
      await fetchBytes(client);
      const error = await forceRefreshCachedValue(client, {
        cacheName: 'keyMaterial',
        invalidVersion: version(GEN1),
        fhevmContext,
      }).catch((e: unknown) => e);
      expect(error).toMatchObject({
        name: 'CachedValueInvalidError',
        cacheName: 'keyMaterial',
        invalidVersion: version(GEN1),
      });
      expect(downloads).toHaveBeenCalledTimes(1);
    });

    it('does not download while the relayer and the chain disagree, and keeps the cached key meanwhile', async () => {
      const { state, newClient, downloads } = makeWorld();
      const client = newClient();
      await fetchBytes(client);
      state.chain = GEN2; // the relayer still serves GEN1

      for (let i = 0; i < 3; i++) {
        await expect(refresh(client, version(GEN1))).rejects.toMatchObject({
          name: 'FheEncryptionKeyVerificationError',
        });
        // Other requests keep the cached key: the relayer may still accept it.
        await expect(fetchBytes(client)).resolves.toMatchObject({ publicKeyBytes: { bytes: GEN1.publicKey } });
      }
      expect(downloads).toHaveBeenCalledTimes(1);

      state.relayer = GEN2;
      await refresh(client, version(GEN1));
      await expect(fetchBytes(client)).resolves.toMatchObject({ publicKeyBytes: { bytes: GEN2.publicKey } });
      expect(downloads).toHaveBeenCalledTimes(2);
    });

    it('after the TTL, fails while the relayer cannot serve the active key, then downloads it', async () => {
      vi.useFakeTimers();
      const { state, newClient, downloads } = makeWorld();
      const client = newClient({ keyMaterial: { ttl: 1_000 } });
      await fetchBytes(client);
      state.chain = GEN2; // the relayer still serves GEN1
      vi.advanceTimersByTime(2_000);
      await expect(fetchBytes(client)).rejects.toMatchObject({ name: 'FheEncryptionKeyVerificationError' });
      expect(downloads).toHaveBeenCalledTimes(1);

      state.relayer = GEN2;
      await expect(fetchBytes(client)).resolves.toMatchObject({ publicKeyBytes: { bytes: GEN2.publicKey } });
      expect(downloads).toHaveBeenCalledTimes(2);
    });

    it('without invalidVersion, reads the active ids once for concurrent refreshes, downloading only a new key', async () => {
      const { state, newClient, downloads, activeIdReads } = makeWorld();
      const client = newClient();
      await fetchBytes(client);
      const reads = activeIdReads();
      await Promise.all([refresh(client), refresh(client), refresh(client)]);
      expect(activeIdReads()).toBe(reads + 1);
      expect(downloads).toHaveBeenCalledTimes(1);

      state.chain = GEN2;
      state.relayer = GEN2;
      await expect(refresh(client)).resolves.toBe(version(GEN2));
      expect(downloads).toHaveBeenCalledTimes(2);
    });
  });

  it('a download in flight across a rotation: the refresh downloads the new key, no CachedValueInvalidError', async () => {
    const { state, newClient, downloads } = makeWorld();
    const client = newClient();
    let release: (() => void) | undefined;
    state.gate = new Promise<void>((resolve) => (release = resolve));
    // Passes the active-id check on GEN1, then waits.
    const slow = ensureFheEncryptionKeyDownload(client, { fhevmContext });
    await vi.waitFor(() => expect(release).toBeDefined());
    await new Promise((resolve) => setTimeout(resolve, 0));
    state.chain = GEN2;
    state.relayer = GEN2;

    const refreshing = refresh(client, version(GEN1));
    state.gate = undefined;
    release?.();
    await slow.catch(() => undefined);

    await expect(refreshing).resolves.toBe(version(GEN2));
    expect(await fetchVersion(client)).toBe(version(GEN2));
    expect(downloads).toHaveBeenCalledTimes(2);
  });

  it('never uses a refreshed key before the client checked it', async () => {
    const { state, newClient } = makeWorld();
    const client = newClient();
    await fetchBytes(client);
    // The relayer and chain id agree, but the chain's digest differs: the bytes fail the check.
    state.chain = { ...GEN2, publicKey: new Uint8Array([9]) as Bytes };
    state.relayer = GEN2;
    await refresh(client, version(GEN1));
    await expect(fetchBytes(client)).rejects.toMatchObject({ name: 'FheEncryptionKeyVerificationError' });
  });
});

describe('encryption after a key rotation', () => {
  // A stand-in for encrypt: the coprocessors reject a proof made with a key
  // other than the active one.
  function encryptRequest(world: ReturnType<typeof makeWorld>, client: Client) {
    const run = vi.fn(async (usedVersions: Parameters<Parameters<typeof retryOnStaleCache>[2]>[0]) => {
      const checked = await ensureFheEncryptionKeyDownload(client, { fhevmContext });
      recordUsedCacheVersion(usedVersions, 'keyMaterial', checked.version);
      if (checked.version !== version(world.state.chain)) {
        throw new RelayerResponseInputProofRejectedError({
          url: RELAYER_URL,
          fetchMethod: 'GET',
          operation: 'INPUT_PROOF',
          retryCount: 0,
          elapsed: 0,
          status: 200,
        } as unknown as ConstructorParameters<typeof RelayerResponseInputProofRejectedError>[0]);
      }
      return checked.version;
    });
    const send = () => retryOnStaleCache(client, fhevmContext, run);
    return { run, send };
  }

  it('the next encryption succeeds with the new key, after one download', async () => {
    const world = makeWorld();
    const client = world.newClient();
    const request = encryptRequest(world, client);
    await request.send();
    world.state.chain = GEN2;
    world.state.relayer = GEN2;

    await expect(request.send()).resolves.toBe(version(GEN2));
    expect(request.run).toHaveBeenCalledTimes(3);
    expect(world.downloads).toHaveBeenCalledTimes(2);
  });

  it('autoRefreshOnInvalidity: false: CachedValueInvalidError with no download; forceRefresh downloads once', async () => {
    const world = makeWorld();
    const client = world.newClient({ autoRefreshOnInvalidity: false });
    const request = encryptRequest(world, client);
    await request.send();
    world.state.chain = GEN2;
    world.state.relayer = GEN2;

    await expect(request.send()).rejects.toMatchObject({
      name: 'CachedValueInvalidError',
      cacheName: 'keyMaterial',
      invalidVersion: version(GEN1),
    });
    // Only the active ids were read: the new key was not downloaded.
    expect(world.downloads).toHaveBeenCalledTimes(1);
    await forceRefreshCachedValue(client, { cacheName: 'keyMaterial', invalidVersion: version(GEN1), fhevmContext });
    expect(world.downloads).toHaveBeenCalledTimes(2);
    await expect(request.send()).resolves.toBe(version(GEN2));
  });

  it('a rejected proof with the active key: the rejection as is, no refresh, no download', async () => {
    const world = makeWorld();
    const client = world.newClient();
    const request = encryptRequest(world, client);
    await request.send();
    request.run.mockClear();
    request.run.mockImplementationOnce(async (usedVersions) => {
      recordUsedCacheVersion(usedVersions, 'keyMaterial', version(GEN1));
      throw new RelayerResponseInputProofRejectedError({
        url: RELAYER_URL,
        operation: 'INPUT_PROOF',
      } as unknown as ConstructorParameters<typeof RelayerResponseInputProofRejectedError>[0]);
    });
    await expect(request.send()).rejects.toBeInstanceOf(RelayerResponseInputProofRejectedError);
    expect(request.run).toHaveBeenCalledTimes(1);
    expect(world.downloads).toHaveBeenCalledTimes(1);
  });

  it('a client that does not check keys (mock relayer): a rejected proof as is, no refresh, no download', async () => {
    const world = makeWorld({ labelIds: true, unverified: true });
    const client = world.newClient();
    const request = encryptRequest(world, client);
    await ensureFheEncryptionKeyDownload(client, { fhevmContext });
    const reads = world.activeIdReads();
    request.run.mockImplementationOnce(async (usedVersions) => {
      recordUsedCacheVersion(
        usedVersions,
        'keyMaterial',
        (await ensureFheEncryptionKeyDownload(client, { fhevmContext })).version,
      );
      throw new RelayerResponseInputProofRejectedError({
        url: RELAYER_URL,
        operation: 'INPUT_PROOF',
      } as unknown as ConstructorParameters<typeof RelayerResponseInputProofRejectedError>[0]);
    });

    await expect(request.send()).rejects.toBeInstanceOf(RelayerResponseInputProofRejectedError);
    expect(request.run).toHaveBeenCalledTimes(1);
    expect(world.downloads).toHaveBeenCalledTimes(1);
    expect(world.activeIdReads()).toBe(reads);
  });

  it('label ids, the relayer still serves the old key: nothing cached, the old key stays, then the new one', async () => {
    const world = makeWorld({ labelIds: true });
    const client = world.newClient();
    const request = encryptRequest(world, client);
    await request.send();
    world.state.chain = GEN2; // the relayer still serves GEN1, under label ids

    await expect(request.send()).rejects.toBeInstanceOf(RelayerResponseInputProofRejectedError);
    expect(fheEncryptionKeyCacheForTests.peek(getFheEncryptionKeySlot(client))?.version).toBe(version(GEN1));

    world.state.relayer = GEN2;
    await expect(request.send()).resolves.toBe(version(GEN2));
  });

  it('the relayer still serves the old key: the request error as is, and the cached key stays', async () => {
    const world = makeWorld();
    const client = world.newClient();
    const request = encryptRequest(world, client);
    await request.send();
    world.state.chain = GEN2; // the relayer still serves GEN1

    await expect(request.send()).rejects.toBeInstanceOf(RelayerResponseInputProofRejectedError);
    expect(request.run).toHaveBeenCalledTimes(2);
    await expect(fetchBytes(client)).resolves.toMatchObject({ publicKeyBytes: { bytes: GEN1.publicKey } });
  });
});
