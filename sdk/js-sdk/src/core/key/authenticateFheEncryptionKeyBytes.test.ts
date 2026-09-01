import type { FheEncryptionKeyBytes, FheEncryptionKeyTrust } from '../types/fheEncryptionKey.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { BytesHex } from '../types/primitives.js';
import type { FheEncryptionKeyProvider } from './FheEncryptionKeyProvider-p.js';
import { Buffer } from 'node:buffer';
import { runInNewContext } from 'node:vm';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { EncryptionError } from '../errors/EncryptionError.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { globalFheEncryptionKeyCache, globalFheEncryptionKeyWasmCache } from './FheEncryptionKeyCache-p.js';
import { computeFheEncryptionKeyDigests } from './authenticateFheEncryptionKeyBytes.js';
import { createFheEncryptionKeyProvider } from './FheEncryptionKeyProvider-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createCoreFhevm, setFrozenContext } from '../runtime/CoreFhevm-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';

const deserializeFheEncryptionKey = vi.hoisted(() =>
  vi.fn(async (context: { readonly runtime?: { readonly uid?: string } }, keyBytes: FheEncryptionKeyBytes) => ({
    keyBytes,
    metadata: keyBytes.metadata,
    runtimeUid: context.runtime?.uid,
  })),
);

vi.mock('./deserializeFheEncryptionKey.js', () => ({ deserializeFheEncryptionKey }));
import { fetchFheEncryptionKeyWasm } from './fetchFheEncryptionKey.js';
import { fetchFheEncryptionKeyBytes } from './fetchFheEncryptionKeyBytes.js';
import { fetchFheEncryptionKeyBytes as fetchFheEncryptionKeyBytesAction } from '../actions/chain/fetchFheEncryptionKeyBytes.js';
import { buildWithProofPacked } from '../modules/encrypt/module/api-p.js';
import { createCleartextFheEncryptionKeyPolicy, createFheEncryptionKeyPolicy } from './FheEncryptionKeyPolicy-p.js';
import { mainnet } from '../chains/definitions/mainnet.js';
import { polygonAmoy } from '../chains/definitions/polygonAmoy.js';
import { sepolia } from '../chains/definitions/sepolia.js';

const RELAYER_URL = 'https://relayer.example.com';
const SEPOLIA_KMS_GENERATION_ADDRESS = '0x77389113d7000EcBCfc2bDed57202f5f46109934';
const MAINNET_KMS_GENERATION_ADDRESS = '0xf102cC9A9D2174630c394f5b7B7D63104E348daa';
const GENUINE_KEY = keyBundle(1, new Uint8Array([1, 2, 3, 4]), new Uint8Array([5, 6, 7, 8]));
const ATTACKER_KEY = keyBundle(1, new Uint8Array([9, 9, 9, 9]), new Uint8Array([8, 8, 8, 8]));
const GENUINE_DIGESTS = computeFheEncryptionKeyDigests(GENUINE_KEY);

beforeEach(() => {
  globalFheEncryptionKeyCache.clear();
  globalFheEncryptionKeyWasmCache.clear();
  vi.clearAllMocks();
});

afterEach(() => {
  globalFheEncryptionKeyCache.clear();
  globalFheEncryptionKeyWasmCache.clear();
});

describe('FHE encryption key authentication', () => {
  it('uses the KMSGeneration digest domains', () => {
    expect(GENUINE_DIGESTS).toEqual({
      publicKeyDigest: '0x452298f972e0848bb511d582524a5c516067ee4c662f33a1ef1110d26d6d0ff1',
      crsDigest: '0x1ee0d74e24ad79124e48c2073599ae15b89b1866cfc20dfa80d15807cee1cc62',
    });
  });

  it('fails closed before fetching when no trust anchor is configured', async () => {
    const warn = vi.fn();
    const fixture = makeFixture(GENUINE_KEY, undefined, 1, undefined, false, undefined, 1n, 'runtime-default', warn);

    await expect(fetchWasm(fixture)).rejects.toThrow('no KMSGeneration contract or application trust anchor');
    expect(warn).not.toHaveBeenCalled();
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();
  });

  it('selects chain-configured KMSGeneration trust from the Sepolia definition', () => {
    expect(createFheEncryptionKeyPolicy(undefined, sepolia)).toEqual({
      mode: 'kmsGeneration',
      trust: {
        chainId: 11_155_111n,
        kmsGenerationAddress: SEPOLIA_KMS_GENERATION_ADDRESS,
      },
    });
  });

  it('selects chain-configured KMSGeneration trust from the Mainnet definition', () => {
    expect(createFheEncryptionKeyPolicy(undefined, mainnet)).toEqual({
      mode: 'kmsGeneration',
      trust: {
        chainId: 1n,
        kmsGenerationAddress: MAINNET_KMS_GENERATION_ADDRESS,
      },
    });
  });

  it('requires explicit trust for Polygon Amoy until a KMSGeneration anchor is configured', () => {
    expect(createFheEncryptionKeyPolicy(undefined, polygonAmoy).mode).toBe('missingTrust');
    expect(createFheEncryptionKeyPolicy({ fheEncryptionKeyTrust: GENUINE_DIGESTS }, polygonAmoy).mode).toBe('relayer');
  });

  it('uses chain-configured active on-chain digests', async () => {
    const key = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(key, undefined, sepolia.id, undefined, false, sepolia);
    serveOnChainDigests(fixture.readContract, computeFheEncryptionKeyDigests(key));

    await expect(fetchWasm(fixture)).resolves.toBeDefined();

    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(fixture.getChainId).toHaveBeenCalledTimes(6);
    expect(fixture.readContract).toHaveBeenCalledTimes(10);
    for (const [parameters] of fixture.readContract.mock.calls) {
      expect(parameters).toEqual(expect.objectContaining({ address: SEPOLIA_KMS_GENERATION_ADDRESS }));
    }
    expect(fixture.readContract).toHaveBeenCalledWith(
      expect.objectContaining({
        address: SEPOLIA_KMS_GENERATION_ADDRESS,
        functionName: 'getKeyMaterials',
        args: [101n],
      }),
    );
    expect(fixture.readContract).toHaveBeenCalledWith(
      expect.objectContaining({
        address: SEPOLIA_KMS_GENERATION_ADDRESS,
        functionName: 'getCrsMaterials',
        args: [202n],
      }),
    );
  });

  it('fails closed if KMSGeneration active ids rotate before cache admission', async () => {
    const key = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(key, undefined, sepolia.id, undefined, false, sepolia);
    const digests = computeFheEncryptionKeyDigests(key);
    let activeKeyId = 101n;
    fixture.fetchKeyBytes.mockImplementationOnce(async () => {
      activeKeyId = 303n;
      return key;
    });
    fixture.readContract.mockImplementation(async (parameters: { readonly functionName: string }) => {
      switch (parameters.functionName) {
        case 'getActiveKeyId':
          return activeKeyId;
        case 'getActiveCrsId':
          return 202n;
        case 'getKeyMaterials':
          return [[], [{ keyType: 1, digest: digests.publicKeyDigest }]];
        case 'getCrsMaterials':
          return [[], digests.crsDigest];
        default:
          throw new Error(`Unexpected contract read: ${parameters.functionName}`);
      }
    });

    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(FhevmConfigError);
    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(fixture.getChainId).toHaveBeenCalledTimes(3);
    expect(fixture.readContract).toHaveBeenCalledTimes(8);
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();
  });

  it('revalidates KMSGeneration host trust before every byte-cache lookup', async () => {
    const key = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(key, undefined, sepolia.id, undefined, false, sepolia);
    serveOnChainDigests(fixture.readContract, computeFheEncryptionKeyDigests(key));

    await fetchWasm(fixture);
    await fetchWasm(fixture);

    expect(fixture.getChainId).toHaveBeenCalledTimes(10);
    expect(fixture.readContract).toHaveBeenCalledTimes(18);
    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(deserializeFheEncryptionKey).toHaveBeenCalledOnce();
  });

  it('rejects relayer bytes that do not match KMSGeneration on-chain digests', async () => {
    const attacker = keyBundle(
      sepolia.id,
      ATTACKER_KEY.publicKeyBytes.bytes,
      ATTACKER_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(attacker, undefined, sepolia.id, undefined, false, sepolia);
    serveOnChainDigests(fixture.readContract, GENUINE_DIGESTS);

    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(EncryptionError);
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();
  });

  it('accepts legacy opaque relayer data ids when bytes match KMSGeneration digests', async () => {
    const legacy = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
      'fhe-public-key-data-id',
      'fhe-crs-data-id',
    );
    const fixture = makeFixture(legacy, undefined, sepolia.id, undefined, false, sepolia);
    serveOnChainDigests(fixture.readContract, computeFheEncryptionKeyDigests(legacy));

    await expect(fetchWasm(fixture)).resolves.toBeDefined();
    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(deserializeFheEncryptionKey).toHaveBeenCalledOnce();
  });

  it('fails closed when KMSGeneration trust material is unavailable', async () => {
    const key = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(key, undefined, sepolia.id, undefined, false, sepolia);
    fixture.readContract.mockRejectedValue(new Error('RPC unavailable'));

    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(FhevmConfigError);
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();
  });

  it('fails closed when a KMSGeneration preset is connected to the wrong chain', async () => {
    const key = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(key, undefined, sepolia.id, undefined, false, sepolia, 80_002n);
    serveOnChainDigests(fixture.readContract, computeFheEncryptionKeyDigests(key));

    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(FhevmConfigError);
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(fixture.readContract).not.toHaveBeenCalled();
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();
  });

  it('fails closed if a KMSGeneration provider changes chain while trust material is read', async () => {
    const key = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(key, undefined, sepolia.id, undefined, false, sepolia);
    fixture.getChainId.mockResolvedValueOnce(BigInt(sepolia.id)).mockResolvedValueOnce(80_002n);
    serveOnChainDigests(fixture.readContract, computeFheEncryptionKeyDigests(key));

    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(FhevmConfigError);
    expect(fixture.getChainId).toHaveBeenCalledTimes(2);
    expect(fixture.readContract).toHaveBeenCalledTimes(6);
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();
  });

  it('does not allow application digest overrides when KMSGeneration is configured', () => {
    expect(() => createFheEncryptionKeyPolicy({ fheEncryptionKeyTrust: GENUINE_DIGESTS }, sepolia)).toThrow(
      FhevmConfigError,
    );

    const pinned = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    expect(createFheEncryptionKeyPolicy({ fheEncryptionKey: pinned }, sepolia)).toMatchObject({
      mode: 'pinned',
      configuredTrust: {
        chainId: 11_155_111n,
        kmsGenerationAddress: SEPOLIA_KMS_GENERATION_ADDRESS,
      },
    });
  });

  it('authenticates pinned bytes against chain-configured KMSGeneration trust', async () => {
    const pinned = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(ATTACKER_KEY, undefined, sepolia.id, pinned, false, sepolia);
    serveOnChainDigests(fixture.readContract, computeFheEncryptionKeyDigests(pinned));

    await expect(fetchWasm(fixture)).resolves.toBeDefined();

    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(fixture.readContract).toHaveBeenCalledWith(
      expect.objectContaining({
        address: SEPOLIA_KMS_GENERATION_ADDRESS,
        functionName: 'getKeyMaterials',
        args: [101n],
      }),
    );
  });

  it('rejects pinned bytes that disagree with chain-configured KMSGeneration trust', async () => {
    const attacker = keyBundle(
      sepolia.id,
      ATTACKER_KEY.publicKeyBytes.bytes,
      ATTACKER_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(GENUINE_KEY, undefined, sepolia.id, attacker, false, sepolia);
    serveOnChainDigests(fixture.readContract, GENUINE_DIGESTS);

    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(FhevmConfigError);
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();
  });

  it('rejects attacker bytes before deserialization and retries cleanly', async () => {
    const fixture = makeFixture(ATTACKER_KEY, GENUINE_DIGESTS);

    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(EncryptionError);
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();

    fixture.served.current = GENUINE_KEY;
    await expect(fetchWasm(fixture)).resolves.toBeDefined();
    expect(fixture.fetchKeyBytes).toHaveBeenCalledTimes(2);
    expect(deserializeFheEncryptionKey).toHaveBeenCalledOnce();
  });

  it('keeps cleartext, pinned, and relayer clients in separate cache identities', async () => {
    const cleartext = makeFixture(ATTACKER_KEY, undefined, 1, GENUINE_KEY, true);
    const pinned = makeFixture(ATTACKER_KEY, undefined, 1, GENUINE_KEY);
    const relayer = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);

    await fetchWasm(cleartext);
    await fetchWasm(pinned);
    await fetchWasm(relayer);

    expect(cleartext.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(pinned.fetchKeyBytes).not.toHaveBeenCalled();
    expect(relayer.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(deserializeFheEncryptionKey).toHaveBeenCalledTimes(3);
  });

  it('isolates chains that share a relayer and trust anchor', async () => {
    const first = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);
    const second = makeFixture(
      keyBundle(2, GENUINE_KEY.publicKeyBytes.bytes, GENUINE_KEY.crsBytes.bytes),
      GENUINE_DIGESTS,
      2,
    );

    await fetchWasm(first);
    await fetchWasm(second);

    expect(first.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(second.fetchKeyBytes).toHaveBeenCalledOnce();
  });

  it('isolates authenticated byte cache entries by per-call relayer auth options', async () => {
    const fixture = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);

    await fetchFheEncryptionKeyBytes(fixture.provider, {
      options: { auth: { type: 'ApiKeyHeader', value: 'first-test-key' } },
    });
    await fetchFheEncryptionKeyBytes(fixture.provider, {
      options: { auth: { type: 'ApiKeyHeader', value: 'second-test-key' } },
    });

    expect(fixture.fetchKeyBytes).toHaveBeenCalledTimes(2);
  });

  it('isolates authenticated byte cache entries when per-call relayer auth mutates in place', async () => {
    const fixture = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);
    const auth = { type: 'ApiKeyHeader' as const, value: 'first-test-key' };
    const parameters = { options: { auth } };

    await fetchFheEncryptionKeyBytes(fixture.provider, parameters);
    auth.value = 'second-test-key';
    await fetchFheEncryptionKeyBytes(fixture.provider, parameters);

    expect(fixture.fetchKeyBytes).toHaveBeenCalledTimes(2);
  });

  it('fails closed if pinned KMSGeneration trust rotates after a cached lookup', async () => {
    const pinned = keyBundle(
      sepolia.id,
      GENUINE_KEY.publicKeyBytes.bytes,
      GENUINE_KEY.crsBytes.bytes,
      sepolia.fhevm.relayerUrl,
    );
    const fixture = makeFixture(GENUINE_KEY, undefined, sepolia.id, pinned, false, sepolia);
    const digests = computeFheEncryptionKeyDigests(pinned);
    let activeKeyIdReads = 0;
    fixture.readContract.mockImplementation(async (parameters: { readonly functionName: string }) => {
      switch (parameters.functionName) {
        case 'getActiveKeyId':
          activeKeyIdReads += 1;
          return activeKeyIdReads === 5 ? 303n : 101n;
        case 'getActiveCrsId':
          return 202n;
        case 'getKeyMaterials':
          return [[], [{ keyType: 1, digest: digests.publicKeyDigest }]];
        case 'getCrsMaterials':
          return [[], digests.crsDigest];
        default:
          throw new Error(`Unexpected contract read: ${parameters.functionName}`);
      }
    });

    await expect(fetchWasm(fixture)).resolves.toBeDefined();
    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(FhevmConfigError);

    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(deserializeFheEncryptionKey).toHaveBeenCalledOnce();
  });

  it('isolates WASM cache entries by TFHE version', async () => {
    const fixture = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);

    await fetchWasm(fixture, false, '1.5.3');
    await fetchWasm(fixture, false, '1.6.2');

    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(deserializeFheEncryptionKey).toHaveBeenCalledTimes(2);
  });

  it('isolates WASM cache entries by runtime while sharing authenticated bytes', async () => {
    const first = makeFixture(GENUINE_KEY, GENUINE_DIGESTS, 1, undefined, false, undefined, 1n, 'runtime-a');
    const second = makeFixture(GENUINE_KEY, GENUINE_DIGESTS, 1, undefined, false, undefined, 1n, 'runtime-b');

    const firstWasm = await fetchWasm(first);
    const secondWasm = await fetchWasm(second);

    expect(first.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(second.fetchKeyBytes).not.toHaveBeenCalled();
    expect(deserializeFheEncryptionKey).toHaveBeenCalledTimes(2);
    expect(firstWasm).not.toBe(secondWasm);
    expect(firstWasm).toEqual(expect.objectContaining({ runtimeUid: 'runtime-a' }));
    expect(secondWasm).toEqual(expect.objectContaining({ runtimeUid: 'runtime-b' }));
  });

  it('keeps authenticated bytes after a version-specific WASM conversion failure', async () => {
    const fixture = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);
    deserializeFheEncryptionKey.mockRejectedValueOnce(new Error('unsupported TFHE version'));

    await expect(fetchWasm(fixture, false, '1.5.3')).rejects.toThrow('unsupported TFHE version');
    await expect(fetchWasm(fixture, false, '1.6.2')).resolves.toBeDefined();

    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(deserializeFheEncryptionKey).toHaveBeenCalledTimes(2);
  });

  it('invalidates same-version WASM when raw bytes are refreshed', async () => {
    const fixture = makeFixture(GENUINE_KEY, undefined, 1, undefined, true);
    await fetchWasm(fixture, false, '1.6.2');

    fixture.served.current = ATTACKER_KEY;
    await fetchFheEncryptionKeyBytes(fixture.provider, { ignoreCache: true });
    await fetchWasm(fixture, false, '1.6.2');

    expect(fixture.fetchKeyBytes).toHaveBeenCalledTimes(2);
    expect(deserializeFheEncryptionKey).toHaveBeenCalledTimes(2);
    expect(deserializeFheEncryptionKey.mock.calls[1]?.[1]).toEqual(ATTACKER_KEY);
  });

  it('rejects pinned bytes that disagree with an explicit trust anchor', async () => {
    const fixture = makeFixture(GENUINE_KEY, GENUINE_DIGESTS, 1, ATTACKER_KEY);

    await expect(fetchWasm(fixture)).rejects.toBeInstanceOf(FhevmConfigError);
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
    expect(deserializeFheEncryptionKey).not.toHaveBeenCalled();
  });

  it('takes ownership of pinned bytes without contacting the relayer', async () => {
    const pinned = keyBundle(1, new Uint8Array([1, 2, 3, 4]), new Uint8Array([5, 6, 7, 8]));
    const fixture = makeFixture(GENUINE_KEY, undefined, 1, pinned);
    pinned.publicKeyBytes.bytes.fill(9);

    await expect(fetchWasm(fixture)).resolves.toBeDefined();
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
  });

  it('admits the policy-owned pinned identity and reuses it when ignoreCache is requested', async () => {
    const pinned = keyBundle(1, new Uint8Array([1, 2, 3, 4]), new Uint8Array([5, 6, 7, 8]));
    const fixture = makeFixture(GENUINE_KEY, undefined, 1, pinned);

    const first = await fetchFheEncryptionKeyBytes(fixture.provider);
    const ignored = await fetchFheEncryptionKeyBytes(fixture.provider, { ignoreCache: true });

    expect(fixture.fheEncryptionKeyPolicy.mode).toBe('pinned');
    if (fixture.fheEncryptionKeyPolicy.mode !== 'pinned') {
      throw new Error('Expected pinned policy');
    }
    expect(first).toBe(fixture.fheEncryptionKeyPolicy.key);
    expect(first).not.toBe(pinned);
    expect(first.publicKeyBytes.bytes).not.toBe(pinned.publicKeyBytes.bytes);
    expect(first.crsBytes.bytes).not.toBe(pinned.crsBytes.bytes);
    expect(ignored).toBe(first);
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
  });

  it('takes ownership of Buffer-backed pinned bytes', async () => {
    const publicKey = Buffer.from([1, 2, 3, 4]);
    const crs = Buffer.from([5, 6, 7, 8]);
    const fixture = makeFixture(GENUINE_KEY, undefined, 1, keyBundle(1, publicKey, crs));
    publicKey.fill(9);
    crs.fill(9);

    await expect(fetchWasm(fixture)).resolves.toBeDefined();

    const deserialized = deserializeFheEncryptionKey.mock.calls[0]?.[1];
    expect(deserialized?.publicKeyBytes.bytes).toEqual(new Uint8Array([1, 2, 3, 4]));
    expect(deserialized?.crsBytes.bytes).toEqual(new Uint8Array([5, 6, 7, 8]));
    expect(Buffer.isBuffer(deserialized?.publicKeyBytes.bytes)).toBe(false);
    expect(Buffer.isBuffer(deserialized?.crsBytes.bytes)).toBe(false);
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
  });

  it('takes ownership of Buffer-backed relayer bytes before cache admission', async () => {
    const publicKey = Buffer.from([1, 2, 3, 4]);
    const crs = Buffer.from([5, 6, 7, 8]);
    const relayerKey = keyBundle(1, publicKey, crs);
    const fixture = makeFixture(relayerKey, computeFheEncryptionKeyDigests(relayerKey));

    await fetchFheEncryptionKeyBytes(fixture.provider);
    publicKey.fill(9);
    crs.fill(9);
    await expect(fetchWasm(fixture)).resolves.toBeDefined();

    const deserialized = deserializeFheEncryptionKey.mock.calls[0]?.[1];
    expect(deserialized?.publicKeyBytes.bytes).toEqual(new Uint8Array([1, 2, 3, 4]));
    expect(deserialized?.crsBytes.bytes).toEqual(new Uint8Array([5, 6, 7, 8]));
    expect(Buffer.isBuffer(deserialized?.publicKeyBytes.bytes)).toBe(false);
    expect(Buffer.isBuffer(deserialized?.crsBytes.bytes)).toBe(false);
    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
  });

  it('accepts and takes ownership of cross-realm Uint8Array bytes', async () => {
    const publicKey = runInNewContext('new Uint8Array([1, 2, 3, 4])') as Uint8Array;
    const crs = runInNewContext('new Uint8Array([5, 6, 7, 8])') as Uint8Array;
    expect(publicKey).not.toBeInstanceOf(Uint8Array);
    expect(crs).not.toBeInstanceOf(Uint8Array);

    const fixture = makeFixture(GENUINE_KEY, undefined, 1, keyBundle(1, publicKey, crs));
    publicKey.fill(9);
    crs.fill(9);
    await expect(fetchWasm(fixture)).resolves.toBeDefined();

    const deserialized = deserializeFheEncryptionKey.mock.calls[0]?.[1];
    expect(deserialized?.publicKeyBytes.bytes).toEqual(new Uint8Array([1, 2, 3, 4]));
    expect(deserialized?.crsBytes.bytes).toEqual(new Uint8Array([5, 6, 7, 8]));
    expect(fixture.fetchKeyBytes).not.toHaveBeenCalled();
  });

  it('uses a new cache identity after resolver-backed rotation', async () => {
    const rotated = keyBundle(1, new Uint8Array([10, 11]), new Uint8Array([12, 13]));
    let activeDigests = GENUINE_DIGESTS;
    const fixture = makeFixture(GENUINE_KEY, async () => activeDigests);

    await fetchWasm(fixture);
    activeDigests = computeFheEncryptionKeyDigests(rotated);
    fixture.served.current = rotated;
    await fetchWasm(fixture);

    expect(fixture.fetchKeyBytes).toHaveBeenCalledTimes(2);
    expect(deserializeFheEncryptionKey).toHaveBeenCalledTimes(2);
  });

  it('re-authenticates an ignored cache entry', async () => {
    const fixture = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);
    await fetchWasm(fixture);

    fixture.served.current = ATTACKER_KEY;
    await expect(fetchWasm(fixture, true)).rejects.toBeInstanceOf(EncryptionError);
    expect(fixture.fetchKeyBytes).toHaveBeenCalledTimes(2);
  });

  it('keeps an in-flight caller alive while ignoreCache refreshes the same key', async () => {
    const fixture = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);
    let resolveFirst!: (value: FheEncryptionKeyBytes) => void;
    const firstFetch = new Promise<FheEncryptionKeyBytes>((resolve) => {
      resolveFirst = resolve;
    });
    fixture.fetchKeyBytes.mockImplementationOnce(() => firstFetch);

    const first = fetchFheEncryptionKeyBytes(fixture.provider);
    await vi.waitFor(() => expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce());

    const refreshed = fetchFheEncryptionKeyBytes(fixture.provider, {
      ignoreCache: true,
    });
    await vi.waitFor(() => expect(fixture.fetchKeyBytes).toHaveBeenCalledTimes(2));
    resolveFirst(GENUINE_KEY);

    await expect(Promise.all([first, refreshed])).resolves.toHaveLength(2);
  });

  it('does not expose mutable cached bytes through the public action or reflected client symbols', async () => {
    const fixture = makeFixture(GENUINE_KEY, GENUINE_DIGESTS);
    const token = Symbol('test-owner');
    const runtime = createFhevmRuntime(token, {
      config: {},
      ethereum: {} as never,
      relayer: { fetchFheEncryptionKeyBytes: fixture.fetchKeyBytes } as never,
    });
    const client = createCoreFhevm(token, {
      chain: fixture.context.chain,
      runtime,
      client: {},
      options: { fheEncryptionKeyTrust: GENUINE_DIGESTS },
    });
    setFrozenContext(client, createFhevmClientFrozenContext({}));
    const returned = await fetchFheEncryptionKeyBytesAction(client);
    returned.publicKeyBytes.bytes.fill(0);
    returned.crsBytes.bytes.fill(0);

    const reflectedProviders = getReflectedZeroArgumentProviders(client);
    for (const provider of reflectedProviders) {
      const cached = await provider.getAuthenticatedBytes();
      cached.publicKeyBytes.bytes.fill(9);
      cached.crsBytes.bytes.fill(9);
    }

    await fetchWasm(fixture);

    const deserialized = deserializeFheEncryptionKey.mock.calls[0]?.[1];
    expect(reflectedProviders).toHaveLength(0);
    expect(deserialized?.publicKeyBytes.bytes).toEqual(GENUINE_KEY.publicKeyBytes.bytes);
    expect(deserialized?.crsBytes.bytes).toEqual(GENUINE_KEY.crsBytes.bytes);
    expect(fixture.fetchKeyBytes).toHaveBeenCalledOnce();
    expect(deserializeFheEncryptionKey).toHaveBeenCalledOnce();
  });

  it('rejects structurally assembled FHE key handles at the raw proof-building boundary', async () => {
    const structuralKey = Object.freeze({
      publicKey: Object.freeze({ id: 'relayer-public-key' }),
      crs: Object.freeze({ id: 'relayer-crs', capacity: 2048 }),
      metadata: Object.freeze({ chainId: sepolia.id, relayerUrl: sepolia.fhevm.relayerUrl }),
      tfheVersion: '1.6.2',
    });

    await expect(
      buildWithProofPacked({} as never, {
        fheEncryptionKey: structuralKey as never,
        typedValues: [],
        metaData: new Uint8Array(),
        extraData: '0x' as BytesHex,
        tfheVersion: '1.6.2',
      }),
    ).rejects.toThrow('Invalid FheEncryptionKey instance');
  });
});

function keyBundle(
  chainId: number,
  publicKey: Uint8Array,
  crs: Uint8Array,
  relayerUrl = RELAYER_URL,
  publicKeyId = '101',
  crsId = '202',
): FheEncryptionKeyBytes {
  return {
    publicKeyBytes: { id: publicKeyId, bytes: publicKey },
    crsBytes: { id: crsId, capacity: 2048, bytes: crs },
    metadata: { chainId, relayerUrl },
  };
}

function makeFixture(
  initialKey: FheEncryptionKeyBytes,
  trust?: FheEncryptionKeyTrust,
  chainId = 1,
  pinnedKey?: FheEncryptionKeyBytes,
  allowUntrustedFheEncryptionKey = false,
  chain?: FhevmChain,
  connectedChainId = BigInt(chain?.id ?? chainId),
  runtimeUid = 'runtime-default',
  warn?: (message: string) => void,
) {
  const served = { current: initialKey };
  const fetchKeyBytes = vi.fn(async () => served.current);
  const getChainId = vi.fn(async () => connectedChainId);
  const fheEncryptionKeyPolicy = allowUntrustedFheEncryptionKey
    ? createCleartextFheEncryptionKeyPolicy()
    : createFheEncryptionKeyPolicy({ fheEncryptionKey: pinnedKey, fheEncryptionKeyTrust: trust }, chain);
  const readContract = vi.fn();
  const fixtureChain =
    chain ??
    ({
      id: chainId,
      fhevm: {
        contracts: { kmsGeneration: undefined },
        relayerUrl: RELAYER_URL,
      },
    } as FhevmChain);
  const runtime = {
    uid: runtimeUid,
    config: { auth: undefined, logger: warn === undefined ? undefined : { warn } },
    relayer: { fetchFheEncryptionKeyBytes: fetchKeyBytes },
  } as unknown as WithEncrypt;
  const provider = createFheEncryptionKeyProvider({
    chain: fixtureChain,
    runtime,
    policy: fheEncryptionKeyPolicy,
    configuredTrustReader: {
      getChainId,
      readContract: (parameters) => readContract(parameters),
    },
  });
  const context = {
    chain: fixtureChain,
    runtime,
    tfheVersion: '1.6.2',
    fheEncryptionKeyProvider: provider,
  } as const;
  return { context, provider, fheEncryptionKeyPolicy, fetchKeyBytes, getChainId, served, readContract };
}

function fetchWasm(
  fixture: ReturnType<typeof makeFixture>,
  ignoreCache = false,
  tfheVersion: '1.5.3' | '1.6.2' = '1.6.2',
) {
  return fetchFheEncryptionKeyWasm({ ...fixture.context, tfheVersion }, { ignoreCache });
}

function getReflectedZeroArgumentProviders(client: object): FheEncryptionKeyProvider[] {
  const providers: FheEncryptionKeyProvider[] = [];
  for (const symbol of Object.getOwnPropertySymbols(client)) {
    const candidateAccessor = Object.getOwnPropertyDescriptor(client, symbol)?.value;
    if (typeof candidateAccessor !== 'function' || candidateAccessor.length !== 0) {
      continue;
    }

    try {
      const candidate: unknown = Reflect.apply(candidateAccessor, client, []);
      if (
        candidate !== null &&
        typeof candidate === 'object' &&
        'getAuthenticatedBytes' in candidate &&
        typeof candidate.getAuthenticatedBytes === 'function'
      ) {
        providers.push(candidate as FheEncryptionKeyProvider);
      }
    } catch {
      // A reflected internal accessor is safe here only when zero-argument use is rejected.
    }
  }
  return providers;
}

function serveOnChainDigests(
  readContract: ReturnType<typeof vi.fn>,
  digests: ReturnType<typeof computeFheEncryptionKeyDigests>,
): void {
  readContract.mockImplementation(async (parameters: { readonly functionName: string }) => {
    switch (parameters.functionName) {
      case 'getActiveKeyId':
        return 101n;
      case 'getActiveCrsId':
        return 202n;
      case 'getKeyMaterials':
        return [[], [{ keyType: 1, digest: digests.publicKeyDigest }]];
      case 'getCrsMaterials':
        return [[], digests.crsDigest];
      default:
        throw new Error(`Unexpected contract read: ${parameters.functionName}`);
    }
  });
}
