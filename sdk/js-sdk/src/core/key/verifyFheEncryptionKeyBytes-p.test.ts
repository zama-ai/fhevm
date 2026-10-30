import type { EthCallParameters, EthCallResult, EthereumModule, TrustedClient } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { EncryptModuleFactory } from '../modules/encrypt/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { Fhevm, FhevmKeyVerification } from '../types/coreFhevmClient.js';
import type { WithEncrypt } from '../types/coreFhevmRuntime.js';
import type { Bytes, BytesHex, ChecksummedAddress, UintNumber } from '../types/primitives.js';
import type { Auth } from '../types/auth.js';
import type { Logger } from '../types/logger.js';
import { readFileSync } from 'node:fs';
import { beforeAll, beforeEach, describe, expect, it, vi } from 'vitest';
import { AbiCoder, Interface, id as ethersId } from 'ethers';
import { shake256 } from '@noble/hashes/sha3.js';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { encode as ethersEncode } from '../../ethers/internal/ethereum.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { polygonAmoy } from '../chains/definitions/polygonAmoy.js';
import { bytesToHex } from '../base/bytes.js';
import { verifyTrustedValue } from '../base/trustedValue.js';
import {
  createCoreFhevm,
  getProvidedFheEncryptionKey,
  getProvidedFheEncryptionKeyError,
  setFrozenContext,
} from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { FheEncryptionKeyVerificationError } from '../errors/FheEncryptionKeyVerificationError.js';
import { assertKeyVerificationConfig, verifyFheEncryptionKeyBytes } from './verifyFheEncryptionKeyBytes-p.js';
import { fetchFheEncryptionKeyBytes } from './fetchFheEncryptionKeyBytes.js';
import { fetchFheEncryptionKeyBytes as fetchFheEncryptionKeyBytesAction } from '../actions/chain/fetchFheEncryptionKeyBytes.js';
import { globalFheEncryptionKeyCache, type CacheEntry } from './FheEncryptionKeyCache-p.js';
import { encrypt } from '../coprocessor/encrypt.js';
import { createTypedValue } from '../base/typedValue.js';
import { getHostContractVersion, invalidateVersionCache } from '../host-contracts/HostContractVersion-p.js';
import { getCrsMaterialsAbi, getKeyMaterialsAbi } from '../host-contracts/abi-fragments/fragments.js';
import { fetchFheEncryptionKeyWasm } from './fetchFheEncryptionKey.js';
import { encryptActionsWithModule } from '../clients/decorators/encrypt.js';
import * as tfhe from '../../wasm/tfhe/v1.8.1/tfhe.js';
import { isBlobWorkerSupported, runCodeInIsomorphicWorker } from '../base/isomorphicWorker.js';
import { FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE } from './fheEncryptionKeyDigestsWorkerCode-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/key/verifyFheEncryptionKeyBytes-p.test.ts
////////////////////////////////////////////////////////////////////////////////

// Counts SHAKE256 hashers on the calling thread, test helpers included: count
// around the call under test.
const shake = vi.hoisted(() => ({ hashers: 0 }));
vi.mock('@noble/hashes/sha3.js', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@noble/hashes/sha3.js')>();
  const create: typeof actual.shake256.create = (options) => {
    shake.hashers += 1;
    return actual.shake256.create(options);
  };
  return {
    ...actual,
    shake256: Object.assign((...args: Parameters<typeof actual.shake256>) => actual.shake256(...args), {
      ...actual.shake256,
      create,
    }),
  };
});

// The real worker functions, spied on.
vi.mock('../base/isomorphicWorker.js', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../base/isomorphicWorker.js')>();
  return {
    ...actual,
    isBlobWorkerSupported: vi.fn(actual.isBlobWorkerSupported),
    runCodeInIsomorphicWorker: vi.fn(actual.runCodeInIsomorphicWorker),
  };
});

/** The digests worker calls so far. */
const digestsWorkerCalls = (): unknown[][] =>
  vi.mocked(runCodeInIsomorphicWorker).mock.calls.filter(([code]) => code === FHE_ENCRYPTION_KEY_DIGESTS_WORKER_CODE);

/** The digests worker runs so far. */
const digestsWorkerRuns = (): number => digestsWorkerCalls().length;

const KEY_ID = (4n << 248n) | 3n;
const CRS_ID = (5n << 248n) | 4n;
const id32 = (id: bigint): string => `0x${id.toString(16).padStart(64, '0')}`;

const PUBLIC_KEY = new Uint8Array([1, 2, 3, 4, 5]) as Bytes;
const CRS = new Uint8Array([6, 7, 8, 9]) as Bytes;

function digest(domain: string, bytes: Uint8Array): BytesHex {
  const hasher = shake256.create({ dkLen: 32 });
  hasher.update(new TextEncoder().encode(domain));
  hasher.update(bytes);
  return bytesToHex(hasher.digest());
}

function keyBytes(
  overrides: { publicKey?: Bytes; crs?: Bytes; publicKeyId?: string; crsId?: string } = {},
): FheEncryptionKeyBytes {
  return {
    publicKeyBytes: { id: overrides.publicKeyId ?? id32(KEY_ID), bytes: overrides.publicKey ?? PUBLIC_KEY },
    crsBytes: { id: overrides.crsId ?? id32(CRS_ID), capacity: 2048 as UintNumber, bytes: overrides.crs ?? CRS },
    metadata: { relayerUrl: 'https://relayer.test', chainId: 0 },
  } as unknown as FheEncryptionKeyBytes;
}

// Ethereum: has kmsGeneration; Polygon-like: does not.
const ethereumChain: FhevmChain = { ...sepolia, fhevm: { ...sepolia.fhevm, relayerUrl: 'https://relayer.test' } };
const otherHostChain: FhevmChain = {
  ...polygonAmoy,
  fhevm: { ...polygonAmoy.fhevm, relayerUrl: 'https://relayer.test' },
};

type Handlers = Record<string, (parameters: { readonly args: readonly unknown[] }) => unknown>;

function kmsGenerationHandlers(overrides: Partial<Handlers> = {}): Handlers {
  return {
    getVersion: () => 'KMSGeneration v0.1.0',
    getActiveKeyId: () => KEY_ID,
    getActiveCrsId: () => CRS_ID,
    getKeyMaterials: () => [
      ['https://storage.test'],
      [
        { keyType: 0, digest: digest('PDAT_KEY', new Uint8Array([0xff])) },
        { keyType: 1, digest: digest('PDAT_KEY', PUBLIC_KEY) },
      ],
    ],
    getCrsMaterials: () => [['https://storage.test'], digest('PDAT_CRS', CRS)],
    ...overrides,
  } as Handlers;
}

/**
 * A real core client whose EthereumModule dispatches on the function name and
 * records which native client each read went through.
 */
function makeClient(parameters: {
  readonly chain: FhevmChain;
  readonly handlers: Handlers;
  readonly keyVerification?: FhevmKeyVerification | undefined;
  readonly keyVerificationChainId?: bigint | undefined;
  readonly fheEncryptionKey?: FheEncryptionKeyBytes | undefined;
  readonly relayer?: Partial<RelayerModule> | undefined;
  readonly batchRpcCalls?: boolean | undefined;
  readonly call?: ((parameters: EthCallParameters) => Promise<EthCallResult>) | undefined;
  readonly auth?: Auth | undefined;
  readonly logger?: Logger | undefined;
}) {
  const readClients: unknown[] = [];
  // 'start:<fn>' / 'end:<fn>' per RPC call; each call takes one macrotask.
  const rpcLog: string[] = [];
  const logged = async <T>(name: string, fn: () => T): Promise<T> => {
    rpcLog.push(`start:${name}`);
    await new Promise((resolve) => setTimeout(resolve, 0));
    rpcLog.push(`end:${name}`);
    return fn();
  };
  const readContract = vi.fn(
    async (trustedClient: TrustedClient, p: { readonly functionName: string; readonly args: readonly unknown[] }) => {
      readClients.push(verifyTrustedValue(trustedClient, PRIVATE_ETHERS_TOKEN));
      return logged(p.functionName, () => {
        const handler = parameters.handlers[p.functionName];
        if (handler === undefined) {
          throw new Error(`No mocked handler for ${p.functionName}`);
        }
        return handler(p);
      });
    },
  );
  // A native client `{ chainId }` overrides the network it is connected to.
  const getChainId = vi.fn(async (trustedClient: TrustedClient) => {
    const native = verifyTrustedValue(trustedClient, PRIVATE_ETHERS_TOKEN) as { readonly chainId?: bigint };
    return logged('getChainId', () => native.chainId ?? parameters.keyVerificationChainId ?? BigInt(sepolia.id));
  });
  const call = vi.fn(async (_trustedClient: TrustedClient, p: EthCallParameters) => {
    if (parameters.call === undefined) {
      throw new Error('No mocked call');
    }
    return parameters.call(p);
  });
  const ethereum = { readContract, getChainId, call, encode: ethersEncode } as unknown as EthereumModule;
  const fetchFheEncryptionKeyBytesFn = vi.fn(async () => keyBytes());

  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum,
    relayer: { fetchFheEncryptionKeyBytes: fetchFheEncryptionKeyBytesFn, ...parameters.relayer } as RelayerModule,
    config: { auth: parameters.auth, logger: parameters.logger },
  });

  const hostClient = { name: 'host' };
  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: parameters.chain,
    client: hostClient,
    runtime,
    options: {
      fheEncryptionKey: parameters.fheEncryptionKey,
      batchRpcCalls: parameters.batchRpcCalls,
      keyVerification: parameters.keyVerification,
    },
  });

  return { client, hostClient, readClients, rpcLog, readContract, getChainId, call, fetchFheEncryptionKeyBytesFn };
}

function flipFirstByte(bytes: Uint8Array): void {
  bytes[0] = (bytes[0] ?? 0) ^ 1;
}

/** The bytes of the cache entry under `key`, if any. */
function cachedBytes(key: string): Promise<FheEncryptionKeyBytes> | undefined {
  const entry = globalFheEncryptionKeyCache.get(key);
  return entry === undefined ? undefined : globalFheEncryptionKeyCache.entryBytes(entry);
}

beforeEach(() => {
  globalFheEncryptionKeyCache.clear();
  invalidateVersionCache({ includeInflight: true });
});

////////////////////////////////////////////////////////////////////////////////
// assertKeyVerificationConfig
////////////////////////////////////////////////////////////////////////////////

describe('assertKeyVerificationConfig', () => {
  it('accepts an Ethereum host chain without keyVerification', () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: {} });
    expect(() => assertKeyVerificationConfig(client)).not.toThrow();
  });

  it('rejects another host chain without keyVerification', () => {
    const { client } = makeClient({ chain: otherHostChain, handlers: {} });
    expect(() => assertKeyVerificationConfig(client)).toThrow(FhevmConfigError);
  });

  it('rejects a keyVerification chain without kmsGeneration', () => {
    const { client } = makeClient({
      chain: otherHostChain,
      handlers: {},
      keyVerification: { chain: otherHostChain, client: {} },
    });
    expect(() => assertKeyVerificationConfig(client)).toThrow(/has no KMSGeneration contract/);
  });
});

////////////////////////////////////////////////////////////////////////////////
// verifyFheEncryptionKeyBytes
////////////////////////////////////////////////////////////////////////////////

describe('verifyFheEncryptionKeyBytes', () => {
  it('accepts matching key and CRS on Ethereum, reading through the host client', async () => {
    const { client, hostClient, readClients } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes();

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).resolves.toBe(bytes);
    expect(readClients.every((c) => c === hostClient)).toBe(true);
  });

  it('accepts ethers-style array results with named keys', async () => {
    const pkDigest = digest('PDAT_KEY', PUBLIC_KEY);
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getKeyMaterials: () => [[], [Object.assign([1n, pkDigest], { keyType: 1n, digest: pkDigest })]],
      }),
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).resolves.toBeDefined();
  });

  it('rejects a public key whose digest does not match', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ publicKey: new Uint8Array([9, 9, 9]) as Bytes });

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).rejects.toThrow(FheEncryptionKeyVerificationError);
    await expect(verifyFheEncryptionKeyBytes(client, bytes)).rejects.toThrow(/FHE public key digest mismatch/);
  });

  it('rejects a CRS whose digest does not match', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ crs: new Uint8Array([9]) as Bytes });

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).rejects.toThrow(/CRS digest mismatch/);
  });

  it('rejects a key not active in KMSGeneration (digest of the Server key, not the Public one)', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ publicKey: new Uint8Array([0xff]) as Bytes });

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).rejects.toThrow(FheEncryptionKeyVerificationError);
  });

  it('rejects a 32-byte id that is not the active one, before reading the materials', async () => {
    const { client, rpcLog } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ publicKeyId: id32(KEY_ID - 1n) });

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).rejects.toThrow(/FHE public key id mismatch/);
    expect(rpcLog.filter((e) => /Materials$/.test(e))).toEqual([]);
  });

  it('rejects a 32-byte CRS id that is not the active one', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ crsId: id32(CRS_ID + 1n) });

    const error = await verifyFheEncryptionKeyBytes(client, bytes).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((error as Error).message).toMatch(/CRS id mismatch/);
  });

  // The guard: bytes one bit off the active ones never verify, whatever their id.
  it.each([
    ['public key', { publicKey: PUBLIC_KEY.map((b, i) => (i === PUBLIC_KEY.length - 1 ? b ^ 1 : b)) as Bytes }],
    ['CRS', { crs: CRS.map((b, i) => (i === CRS.length - 1 ? b ^ 1 : b)) as Bytes }],
  ])('rejects a tampered %s served with the active id', async (subject, overrides) => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });

    const error = await verifyFheEncryptionKeyBytes(client, keyBytes(overrides)).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((error as Error).message).toMatch(new RegExp(`${subject} digest mismatch`));
  });

  it('accepts a relayer label id when the digests match', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ publicKeyId: 'fhe-public-key-data-id', crsId: 'crs-data-id' });

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).resolves.toBe(bytes);
  });

  // A label id skips the id check: the digest check alone must catch a wrong key.
  it.each([
    ['an older key', new Uint8Array([0xff])],
    ['tampered bytes', PUBLIC_KEY.map((b, i) => (i === 0 ? b ^ 1 : b))],
  ])('rejects a public key served with a label id: %s', async (_, publicKey) => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ publicKey: publicKey as Bytes, publicKeyId: 'fhe-public-key-data-id' });

    const error = await verifyFheEncryptionKeyBytes(client, bytes).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((error as Error).message).toMatch(/FHE public key digest mismatch/);
  });

  it.each([
    ['an older CRS', new Uint8Array([0xee])],
    ['tampered bytes', CRS.map((b, i) => (i === 0 ? b ^ 1 : b))],
  ])('rejects a CRS served with a label id: %s', async (_, crs) => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ crs: crs as Bytes, crsId: 'crs-data-id' });

    const error = await verifyFheEncryptionKeyBytes(client, bytes).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((error as Error).message).toMatch(/CRS digest mismatch/);
  });

  it('rejects when KMSGeneration has no active key', async () => {
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({ getActiveKeyId: () => 0n }),
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow(FheEncryptionKeyVerificationError);
    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow(/No active FHE public key/);
  });

  it('rejects when KMSGeneration has no active CRS', async () => {
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({ getActiveCrsId: () => 0n }),
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow(/No active CRS/);
  });

  it('rethrows a CRS materials revert as a verification error, with the cause', async () => {
    const revert = new Error('execution reverted: CrsNotGenerated(uint256)');
    const { client, call } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getCrsMaterials: () => {
          throw revert;
        },
      }),
      call: async () => ({ success: false, reason: 'CrsNotGenerated(uint256)' }),
    });

    const error = await verifyFheEncryptionKeyBytes(client, keyBytes()).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((error as Error).message).toMatch(/Cannot read the active CRS materials/);
    expect((error as Error).message).toMatch(/CrsNotGenerated/);
    expect((error as Error).cause).toBe(revert);
    // The failed read is replayed as the same raw eth_call.
    expect(call).toHaveBeenCalledOnce();
    expect(call.mock.calls[0]?.[1]).toEqual({
      to: ethereumChain.fhevm.contracts.kmsGeneration?.address,
      data: new Interface(getCrsMaterialsAbi).encodeFunctionData('getCrsMaterials', [CRS_ID]),
    });
  });

  it('rethrows a key materials revert as a verification error, with the cause', async () => {
    const revert = new Error('execution reverted: KeyNotGenerated(uint256)');
    const { client, call } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getKeyMaterials: () => {
          throw revert;
        },
      }),
      call: async () => ({ success: false, reason: 'KeyNotGenerated(uint256)' }),
    });

    const error = await verifyFheEncryptionKeyBytes(client, keyBytes()).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((error as Error).message).toMatch(/Cannot read the active FHE public key materials/);
    expect((error as Error).message).toMatch(/KeyNotGenerated/);
    expect((error as Error).cause).toBe(revert);
    // The failed read is replayed as the same raw eth_call.
    expect(call).toHaveBeenCalledOnce();
    expect(call.mock.calls[0]?.[1]).toEqual({
      to: ethereumChain.fhevm.contracts.kmsGeneration?.address,
      data: new Interface(getKeyMaterialsAbi).encodeFunctionData('getKeyMaterials', [KEY_ID]),
    });
  });

  it.each([
    { fn: 'getKeyMaterials', signature: 'getKeyMaterials(uint256)', activeId: KEY_ID },
    { fn: 'getCrsMaterials', signature: 'getCrsMaterials(uint256)', activeId: CRS_ID },
  ])(
    'replays a failed $fn read with the $signature selector and the active id',
    async ({ fn, signature, activeId }) => {
      const { client, call } = makeClient({
        chain: ethereumChain,
        handlers: kmsGenerationHandlers({
          [fn]: () => {
            throw new Error('execution reverted');
          },
        }),
        call: async () => ({ success: false, reason: 'reverted' }),
      });

      await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow(FheEncryptionKeyVerificationError);
      const selector = ethersId(signature).slice(0, 10);
      const encodedId = AbiCoder.defaultAbiCoder().encode(['uint256'], [activeId]).slice(2);
      expect(call.mock.calls[0]?.[1].data).toBe(`${selector}${encodedId}`);
    },
  );

  it('rejects an active key with no public key digest', async () => {
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getKeyMaterials: () => [[], [{ keyType: 0, digest: digest('PDAT_KEY', PUBLIC_KEY) }]],
      }),
    });

    const error = await verifyFheEncryptionKeyBytes(client, keyBytes()).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((error as Error).message).toMatch(/has no public key digest/);
  });

  // A malformed result is a bug or an ABI drift, not a key mismatch: a plain Error.
  it.each([
    [
      'getKeyMaterials not an array',
      { getKeyMaterials: () => 'oops' },
      /Invalid KMSGeneration\.getKeyMaterials result/,
    ],
    [
      'getKeyMaterials without digests',
      { getKeyMaterials: () => [[]] },
      /Invalid KMSGeneration\.getKeyMaterials result/,
    ],
    [
      'a public key digest not a string',
      { getKeyMaterials: () => [[], [{ keyType: 1, digest: 42 }]] },
      /Invalid KMSGeneration public key digest/,
    ],
    [
      'getCrsMaterials not an array',
      { getCrsMaterials: () => 'oops' },
      /Invalid KMSGeneration\.getCrsMaterials result/,
    ],
    ['a CRS digest not a string', { getCrsMaterials: () => [[], 42] }, /Invalid KMSGeneration\.getCrsMaterials result/],
  ])('rejects a malformed KMSGeneration result: %s', async (_, overrides, message) => {
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(overrides as Partial<Handlers>),
    });

    const error = await verifyFheEncryptionKeyBytes(client, keyBytes()).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(Error);
    expect(error).not.toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((error as Error).message).toMatch(message);
  });

  it('rethrows a network failure of the key materials read as is', async () => {
    const networkError = new Error('fetch failed');
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getKeyMaterials: () => {
          throw networkError;
        },
      }),
      call: async () => {
        throw new Error('fetch failed again');
      },
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toBe(networkError);
  });

  it('rethrows a network failure of the CRS materials read as is', async () => {
    const networkError = new Error('fetch failed');
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getCrsMaterials: () => {
          throw networkError;
        },
      }),
      call: async () => {
        throw new Error('fetch failed again');
      },
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toBe(networkError);
  });

  it('rethrows a materials read error as is when the replayed call does not revert', async () => {
    const transientError = new Error('rate limited');
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getKeyMaterials: () => {
          throw transientError;
        },
      }),
      call: async () => ({ success: true, data: '0x' as BytesHex }),
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toBe(transientError);
  });

  it('rejects a contract that is not KMSGeneration', async () => {
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({ getVersion: () => 'ACL v0.1.0' }),
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow(/Invalid contract name/);
  });

  it('reads KMSGeneration through the keyVerification client on another host chain', async () => {
    const ethereumClient = { name: 'ethereum' };
    const { client, readClients } = makeClient({
      chain: otherHostChain,
      handlers: kmsGenerationHandlers(),
      keyVerification: { chain: sepolia, client: ethereumClient },
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).resolves.toBeDefined();
    expect(readClients.length).toBeGreaterThan(0);
    expect(readClients.every((c) => c === ethereumClient)).toBe(true);
  });

  it('rejects a keyVerification client connected to another chain as a config error', async () => {
    const { client, readContract } = makeClient({
      chain: otherHostChain,
      handlers: kmsGenerationHandlers(),
      keyVerification: { chain: sepolia, client: {} },
      keyVerificationChainId: 1n,
    });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow(FhevmConfigError);
    expect(readContract).not.toHaveBeenCalled();
  });
});

////////////////////////////////////////////////////////////////////////////////
// verifyFheEncryptionKeyBytes — RPC rounds
////////////////////////////////////////////////////////////////////////////////

describe('verifyFheEncryptionKeyBytes RPC rounds', () => {
  const ethereumClient = { name: 'ethereum' };

  it('reads one call at a time by default (batchRpcCalls: false)', async () => {
    const { client, rpcLog } = makeClient({
      chain: otherHostChain,
      handlers: kmsGenerationHandlers(),
      keyVerification: { chain: sepolia, client: ethereumClient },
    });

    await verifyFheEncryptionKeyBytes(client, keyBytes());
    expect(rpcLog).toEqual(
      ['getChainId', 'getVersion', 'getActiveKeyId', 'getActiveCrsId', 'getKeyMaterials', 'getCrsMaterials'].flatMap(
        (fn) => [`start:${fn}`, `end:${fn}`],
      ),
    );
  });

  it('reads in two rounds with batchRpcCalls: true', async () => {
    const { client, rpcLog } = makeClient({
      chain: otherHostChain,
      handlers: kmsGenerationHandlers(),
      keyVerification: { chain: sepolia, client: ethereumClient },
      batchRpcCalls: true,
    });

    await verifyFheEncryptionKeyBytes(client, keyBytes());
    const round1 = ['getChainId', 'getVersion', 'getActiveKeyId', 'getActiveCrsId'];
    const round2 = ['getKeyMaterials', 'getCrsMaterials'];
    expect(rpcLog.slice(0, 4).sort()).toEqual(round1.map((fn) => `start:${fn}`).sort());
    expect(rpcLog.slice(4, 8).sort()).toEqual(round1.map((fn) => `end:${fn}`).sort());
    expect(rpcLog.slice(8, 10).sort()).toEqual(round2.map((fn) => `start:${fn}`).sort());
    expect(rpcLog).toHaveLength(12);
  });

  it.each([false, true])(
    'reports a wrong network, not the read error it causes (batchRpcCalls: %s)',
    async (batchRpcCalls) => {
      const { client } = makeClient({
        chain: otherHostChain,
        handlers: kmsGenerationHandlers({
          getActiveKeyId: () => {
            throw new Error('execution reverted');
          },
        }),
        keyVerification: { chain: sepolia, client: ethereumClient },
        keyVerificationChainId: 1n,
        batchRpcCalls,
      });

      await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow(FhevmConfigError);
    },
  );

  it.each([false, true])(
    'reports a contract that is not KMSGeneration, not the read error it causes (batchRpcCalls: %s)',
    async (batchRpcCalls) => {
      const { client } = makeClient({
        chain: ethereumChain,
        handlers: kmsGenerationHandlers({
          getVersion: () => 'ACL v0.1.0',
          getActiveKeyId: () => {
            throw new Error('execution reverted');
          },
        }),
        batchRpcCalls,
      });

      await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow(/Invalid contract name/);
    },
  );

  it('does not read the chain id of the host client on Ethereum', async () => {
    const { client, getChainId } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });

    await verifyFheEncryptionKeyBytes(client, keyBytes());
    expect(getChainId).not.toHaveBeenCalled();
  });
});

////////////////////////////////////////////////////////////////////////////////
// verifyFheEncryptionKeyBytes — KMSGeneration version read
////////////////////////////////////////////////////////////////////////////////

describe('verifyFheEncryptionKeyBytes version read', () => {
  const ethereumClient = { name: 'ethereum' };
  const countReads = (readContract: { mock: { calls: unknown[][] } }, fn: string): number =>
    readContract.mock.calls.filter((c) => (c[1] as { functionName: string }).functionName === fn).length;

  it('reads the version through keyVerification on every verification, never from the cache', async () => {
    const { client, readContract, getChainId } = makeClient({
      chain: otherHostChain,
      handlers: kmsGenerationHandlers(),
      keyVerification: { chain: sepolia, client: ethereumClient },
    });

    await verifyFheEncryptionKeyBytes(client, keyBytes());
    await verifyFheEncryptionKeyBytes(client, keyBytes());
    expect(countReads(readContract, 'getVersion')).toBe(2);
    expect(getChainId).toHaveBeenCalledTimes(2);
  });

  it.each([false, true])(
    'fails only the client whose keyVerification client is on another network (batchRpcCalls: %s)',
    async (batchRpcCalls) => {
      const good = makeClient({
        chain: otherHostChain,
        handlers: kmsGenerationHandlers(),
        keyVerification: { chain: sepolia, client: ethereumClient },
        batchRpcCalls,
      });
      // Same runtime and KMSGeneration, but its client is on mainnet.
      const misconfigured = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
        chain: otherHostChain,
        client: {},
        runtime: good.client.runtime,
        options: { batchRpcCalls, keyVerification: { chain: sepolia, client: { chainId: 1n } } },
      });

      const [bad, ok] = await Promise.allSettled([
        verifyFheEncryptionKeyBytes(misconfigured, keyBytes()),
        verifyFheEncryptionKeyBytes(good.client, keyBytes()),
      ]);
      expect(bad.status).toBe('rejected');
      expect((bad as PromiseRejectedResult).reason).toBeInstanceOf(FhevmConfigError);
      expect(ok.status).toBe('fulfilled');
    },
  );

  it('shares the version cache with host contract reads on Ethereum', async () => {
    const { client, readContract } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const address = ethereumChain.fhevm.contracts.kmsGeneration?.address as ChecksummedAddress;

    await getHostContractVersion(client, { address });
    await verifyFheEncryptionKeyBytes(client, keyBytes());
    expect(countReads(readContract, 'getVersion')).toBe(1);
  });
});

////////////////////////////////////////////////////////////////////////////////
// fetchFheEncryptionKeyBytes — only verified bytes reach the cache
////////////////////////////////////////////////////////////////////////////////

describe('fetchFheEncryptionKeyBytes', () => {
  const fhevmContext = createFhevmClientFrozenContext({});

  it('caches downloaded bytes once verified', async () => {
    const { client, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
    });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toBeDefined();
    expect(fetchFheEncryptionKeyBytesFn).toHaveBeenCalledOnce();
    expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(true);
  });

  it('never caches bytes that fail the check', async () => {
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({ getCrsMaterials: () => [[], digest('PDAT_CRS', new Uint8Array([0]))] }),
    });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toThrow(
      FheEncryptionKeyVerificationError,
    );
    expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(false);
  });

  it('checks caller-provided bytes instead of downloading', async () => {
    const provided = keyBytes({ publicKey: new Uint8Array([9]) as Bytes });
    const { client, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
      fheEncryptionKey: provided,
    });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toThrow(/digest mismatch/);
    expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
    expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(false);
  });

  it('uses caller-provided bytes once verified, then refetches from the relayer', async () => {
    const provided = keyBytes();
    const { client, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
      fheEncryptionKey: provided,
    });
    expect('fheEncryptionKey' in client.options).toBe(false);

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toEqual(provided);
    expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
    expect(getProvidedFheEncryptionKey(client)).toBeUndefined();

    globalFheEncryptionKeyCache.remove('https://relayer.test');
    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.not.toBe(provided);
    expect(fetchFheEncryptionKeyBytesFn).toHaveBeenCalledOnce();
  });

  it.each(['https://relayer.test/', 'https://other.test'])(
    'rejects caller-provided bytes from relayer "%s" at client creation, leaving the cache as is',
    async (relayerUrl) => {
      const { client: other } = makeClient({ chain: ethereumChain, handlers: {} });
      const cached = keyBytes();
      globalFheEncryptionKeyCache.setBytes(other.runtime, 'https://relayer.test', 'https://relayer.test', cached);
      const entry = globalFheEncryptionKeyCache.get('https://relayer.test');
      const provided = { ...keyBytes(), metadata: { relayerUrl, chainId: 0 } };

      expect(() => makeClient({ chain: ethereumChain, handlers: {}, fheEncryptionKey: provided })).toThrow(
        FhevmConfigError,
      );
      expect(() => makeClient({ chain: ethereumChain, handlers: {}, fheEncryptionKey: provided })).toThrow(
        /must originate from the same relayer/,
      );
      expect(globalFheEncryptionKeyCache.get('https://relayer.test')).toBe(entry);
      await expect(cachedBytes('https://relayer.test')).resolves.toBe(cached);
    },
  );

  it('keeps its own copy of caller-provided bytes: changing them after creation changes nothing', async () => {
    const provided = keyBytes({ publicKey: PUBLIC_KEY.slice() as Bytes, crs: CRS.slice() as Bytes });
    const { client, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
      fheEncryptionKey: provided,
    });
    // Before the check: the client still checks (and caches) the bytes it was given.
    flipFirstByte(provided.publicKeyBytes.bytes);

    const cached = await fetchFheEncryptionKeyBytes(client, { fhevmContext });
    expect(cached.publicKeyBytes.bytes).toEqual(PUBLIC_KEY);
    expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();

    // After the check: the cache does not share the caller's arrays.
    flipFirstByte(provided.crsBytes.bytes);
    await expect(cachedBytes('https://relayer.test')).resolves.toEqual(keyBytes());
  });

  it('returns a copy from the fetchFheEncryptionKeyBytes action: changing it leaves the cache as is', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    setFrozenContext(client, fhevmContext);

    const returned = await fetchFheEncryptionKeyBytesAction(client);
    expect(returned).toEqual(keyBytes());
    flipFirstByte(returned.publicKeyBytes.bytes);
    flipFirstByte(returned.crsBytes.bytes);

    await expect(cachedBytes('https://relayer.test')).resolves.toEqual(keyBytes());
    await expect(fetchFheEncryptionKeyBytesAction(client)).resolves.toEqual(keyBytes());
  });

  it('keeps rejecting stale caller-provided bytes with the same error: no RPC call, no download', async () => {
    // A base client (no encrypt module).
    const { client, readContract, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
      fheEncryptionKey: keyBytes({ publicKey: new Uint8Array([9]) as Bytes }),
    });

    const error = await fetchFheEncryptionKeyBytes(client, { fhevmContext }).catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    const reads = readContract.mock.calls.length;
    // The client drops its copy and keeps only the error.
    expect(getProvidedFheEncryptionKey(client)).toBeUndefined();
    expect(getProvidedFheEncryptionKeyError(client)).toBe(error);

    // Never falls back to the relayer key: the client keeps failing.
    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toBe(error);
    expect(readContract).toHaveBeenCalledTimes(reads);
    expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
    expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(false);
  });

  it('keeps caller-provided bytes after a network error: the next call checks them again', async () => {
    const networkError = new Error('fetch failed');
    let fail = true;
    const { client, readContract, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getActiveKeyId: () => {
          if (fail) {
            throw networkError;
          }
          return KEY_ID;
        },
      }),
      fheEncryptionKey: keyBytes(),
    });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toBe(networkError);
    expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(false);
    const reads = readContract.mock.calls.length;

    fail = false;
    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toEqual(keyBytes());
    expect(readContract.mock.calls.length).toBeGreaterThan(reads);
    expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
    await expect(cachedBytes('https://relayer.test')).resolves.toEqual(keyBytes());
  });

  describe('caller-provided bytes when the cache already holds the key', () => {
    async function warmCache(): Promise<FheEncryptionKeyBytes> {
      const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
      return fetchFheEncryptionKeyBytes(client, { fhevmContext });
    }

    it('rejects stale provided bytes, as on an empty cache, and never falls back to the cached key', async () => {
      await warmCache();
      const entry = globalFheEncryptionKeyCache.get('https://relayer.test');
      const { client, fetchFheEncryptionKeyBytesFn } = makeClient({
        chain: ethereumChain,
        handlers: kmsGenerationHandlers(),
        fheEncryptionKey: keyBytes({ publicKey: new Uint8Array([9]) as Bytes }),
      });

      const error = await fetchFheEncryptionKeyBytes(client, { fhevmContext }).catch((e: unknown) => e);
      expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
      expect(globalFheEncryptionKeyCache.get('https://relayer.test')).toBe(entry);

      await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toBe(error);
      expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
    });

    it('checks the provided bytes once for concurrent calls on a client', async () => {
      const { client: other } = makeClient({ chain: ethereumChain, handlers: {} });
      const stale = keyBytes({ crs: CRS.slice(1) as Bytes });
      globalFheEncryptionKeyCache.setBytes(other.runtime, 'https://relayer.test', 'https://relayer.test', stale);
      const { client, readContract } = makeClient({
        chain: ethereumChain,
        handlers: kmsGenerationHandlers(),
        fheEncryptionKey: keyBytes(),
      });

      const results = await Promise.all([
        fetchFheEncryptionKeyBytes(client, { fhevmContext }),
        fetchFheEncryptionKeyBytes(client, { fhevmContext }),
      ]);
      // Checked once, then it replaces the stale cached key.
      expect(results[0]).toEqual(keyBytes());
      expect(results[1]).toBe(results[0]);
      const reads = readContract.mock.calls.map(([, p]) => p.functionName);
      expect(reads.filter((name) => name === 'getKeyMaterials')).toHaveLength(1);
      expect(reads.filter((name) => name === 'getCrsMaterials')).toHaveLength(1);
    });

    it('rejects concurrent and later calls with the same error when the check fails', async () => {
      await warmCache();
      const { client, readContract } = makeClient({
        chain: ethereumChain,
        handlers: kmsGenerationHandlers(),
        fheEncryptionKey: keyBytes({ publicKey: new Uint8Array([9]) as Bytes }),
      });

      const results = await Promise.allSettled([
        fetchFheEncryptionKeyBytes(client, { fhevmContext }),
        fetchFheEncryptionKeyBytes(client, { fhevmContext }),
      ]);
      const errors = results.map((r) => (r as PromiseRejectedResult).reason as unknown);
      expect(results.every((r) => r.status === 'rejected')).toBe(true);
      expect(errors[0]).toBeInstanceOf(FheEncryptionKeyVerificationError);
      expect(errors[1]).toBe(errors[0]);
      const keyMaterialReads = (): number =>
        readContract.mock.calls.filter(([, p]) => p.functionName === 'getKeyMaterials').length;
      expect(keyMaterialReads()).toBe(1);

      // Remembered: the next call fails the same, with no RPC call.
      await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toBe(errors[0]);
      expect(keyMaterialReads()).toBe(1);
    });

    it('uses the cached entry for provided bytes identical to the cached ones, still checked, then releases them', async () => {
      const cached = await warmCache();
      const entry = globalFheEncryptionKeyCache.get('https://relayer.test');
      const { client, readContract, fetchFheEncryptionKeyBytesFn } = makeClient({
        chain: ethereumChain,
        handlers: kmsGenerationHandlers(),
        fheEncryptionKey: keyBytes(),
      });

      await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toBe(cached);
      // Checked by this client, with its own reads.
      expect(readContract).toHaveBeenCalled();
      expect(globalFheEncryptionKeyCache.get('https://relayer.test')).toBe(entry);
      expect(getProvidedFheEncryptionKey(client)).toBeUndefined();

      // Released: a refetch after an eviction downloads from the relayer.
      globalFheEncryptionKeyCache.remove('https://relayer.test');
      await fetchFheEncryptionKeyBytes(client, { fhevmContext });
      expect(fetchFheEncryptionKeyBytesFn).toHaveBeenCalledOnce();
    });

    it('evicts the cached entry for provided bytes identical to it that fail the check, and remembers the error', async () => {
      const { client: other } = makeClient({ chain: ethereumChain, handlers: {} });
      globalFheEncryptionKeyCache.setBytes(other.runtime, 'https://relayer.test', 'https://relayer.test', keyBytes());
      // Compared only once its bytes are available.
      await globalFheEncryptionKeyCache.get('https://relayer.test')?.ready;
      // Another active CRS for this client.
      const { client, readContract, fetchFheEncryptionKeyBytesFn } = makeClient({
        chain: ethereumChain,
        handlers: kmsGenerationHandlers({ getCrsMaterials: () => [[], digest('PDAT_CRS', new Uint8Array([0]))] }),
        fheEncryptionKey: keyBytes(),
      });

      const error = await fetchFheEncryptionKeyBytes(client, { fhevmContext }).catch((e: unknown) => e);
      expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
      expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(false);
      const reads = readContract.mock.calls.length;

      // Remembered: the same error, with no RPC call and no relayer download.
      await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toBe(error);
      expect(readContract).toHaveBeenCalledTimes(reads);
      expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
      expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(false);
    });

    it('replaces a stale cached key with different provided bytes that pass the check', async () => {
      const { client: other } = makeClient({ chain: ethereumChain, handlers: {} });
      const stale = keyBytes({ crs: CRS.slice(1) as Bytes });
      globalFheEncryptionKeyCache.setBytes(other.runtime, 'https://relayer.test', 'https://relayer.test', stale);
      const entry = globalFheEncryptionKeyCache.get('https://relayer.test');
      const { client, readContract, fetchFheEncryptionKeyBytesFn } = makeClient({
        chain: ethereumChain,
        handlers: kmsGenerationHandlers(),
        fheEncryptionKey: keyBytes(),
      });

      await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toEqual(keyBytes());
      expect(readContract).toHaveBeenCalled();
      expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
      // A new entry takes the slot: callers holding the old one keep the old key.
      expect(globalFheEncryptionKeyCache.get('https://relayer.test')).not.toBe(entry);
      await entry?.ready;
      expect(entry?.value).toBe(stale);

      // Released: a refetch after an eviction downloads from the relayer.
      globalFheEncryptionKeyCache.remove('https://relayer.test');
      await fetchFheEncryptionKeyBytes(client, { fhevmContext });
      expect(fetchFheEncryptionKeyBytesFn).toHaveBeenCalledOnce();
    });
  });

  it('requires keyVerification on another host chain, for any client', async () => {
    // A base client (no encrypt module): creating it does not need keyVerification,
    // fetching the key does.
    const { client, fetchFheEncryptionKeyBytesFn } = makeClient({ chain: otherHostChain, handlers: {} });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toThrow(FhevmConfigError);
    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toThrow(/options\.keyVerification/);
    expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(false);
    // Rejected before the download.
    expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
  });

  it('requires keyVerification on another host chain even when another client filled the cache', async () => {
    const { client: other } = makeClient({ chain: ethereumChain, handlers: {} });
    globalFheEncryptionKeyCache.setBytes(other.runtime, 'https://relayer.test', 'https://relayer.test', keyBytes());
    const { client } = makeClient({ chain: otherHostChain, handlers: {} });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toThrow(/options\.keyVerification/);
  });

  it('verifies through keyVerification on another host chain, for any client', async () => {
    const ethereumClient = { name: 'ethereum' };
    const { client, readClients } = makeClient({
      chain: otherHostChain,
      handlers: kmsGenerationHandlers(),
      keyVerification: { chain: sepolia, client: ethereumClient },
    });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toBeDefined();
    expect(readClients.every((c) => c === ethereumClient)).toBe(true);
  });

  it('does not check placeholder bytes from a cleartext relayer module, nor require keyVerification', async () => {
    const { client, readContract, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: otherHostChain,
      handlers: {},
      relayer: { servesPlaceholderFheEncryptionKey: true },
    });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toBeDefined();
    expect(readContract).not.toHaveBeenCalled();
    expect(fetchFheEncryptionKeyBytesFn).toHaveBeenCalledOnce();
  });
});

////////////////////////////////////////////////////////////////////////////////
// fetchFheEncryptionKeyBytes — placeholder bytes never reach the shared cache
////////////////////////////////////////////////////////////////////////////////

describe('fetchFheEncryptionKeyBytes with a cleartext client on the same relayer URL', () => {
  const fhevmContext = createFhevmClientFrozenContext({});
  const placeholder = keyBytes({
    publicKey: new Uint8Array([0xde, 0xad]) as Bytes,
    crs: new Uint8Array([0xbe, 0xef]) as Bytes,
    publicKeyId: 'fhe-public-key-data-id',
    crsId: 'crs-data-id',
  });

  function makeCleartextClient() {
    const fetchPlaceholder = vi.fn(async () => placeholder);
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: {},
      relayer: { servesPlaceholderFheEncryptionKey: true, fetchFheEncryptionKeyBytes: fetchPlaceholder },
    });
    return { client, fetchPlaceholder };
  }

  it('keeps the placeholder bytes out of the shared cache: a real client still downloads and verifies', async () => {
    const cleartext = makeCleartextClient();
    await expect(fetchFheEncryptionKeyBytes(cleartext.client, { fhevmContext })).resolves.toBe(placeholder);
    expect(globalFheEncryptionKeyCache.has('https://relayer.test')).toBe(false);

    const real = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = await fetchFheEncryptionKeyBytes(real.client, { fhevmContext });
    expect(bytes).not.toBe(placeholder);
    expect(bytes.publicKeyBytes.bytes).toEqual(PUBLIC_KEY);
    expect(real.fetchFheEncryptionKeyBytesFn).toHaveBeenCalledOnce();
    expect(real.readContract).toHaveBeenCalled();

    // The cleartext client keeps its own bytes.
    await expect(fetchFheEncryptionKeyBytes(cleartext.client, { fhevmContext })).resolves.toBe(placeholder);
    expect(cleartext.fetchPlaceholder).toHaveBeenCalledOnce();
  });

  it('gives a cleartext client its own placeholder bytes when the shared cache holds the real key', async () => {
    const real = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = await fetchFheEncryptionKeyBytes(real.client, { fhevmContext });

    const cleartext = makeCleartextClient();
    await expect(fetchFheEncryptionKeyBytes(cleartext.client, { fhevmContext })).resolves.toBe(placeholder);
    expect(cleartext.fetchPlaceholder).toHaveBeenCalledOnce();
    await expect(cachedBytes('https://relayer.test')).resolves.toBe(bytes);
  });

  it('shares one cache entry between cleartext clients on the same relayer URL', async () => {
    const first = makeCleartextClient();
    const second = makeCleartextClient();
    await expect(fetchFheEncryptionKeyBytes(first.client, { fhevmContext })).resolves.toBe(placeholder);
    await expect(fetchFheEncryptionKeyBytes(second.client, { fhevmContext })).resolves.toBe(placeholder);
    expect(first.fetchPlaceholder).toHaveBeenCalledOnce();
    expect(second.fetchPlaceholder).not.toHaveBeenCalled();
    await expect(cachedBytes('placeholder:https://relayer.test')).resolves.toBe(placeholder);
  });

  it.each(['cleartext first', 'real first'])(
    'never caches placeholder bytes under the plain relayer URL (%s)',
    async (order) => {
      const cleartext = makeCleartextClient();
      const real = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
      const clients = order === 'cleartext first' ? [cleartext.client, real.client] : [real.client, cleartext.client];
      for (const client of clients) {
        await fetchFheEncryptionKeyBytes(client, { fhevmContext });
        const plain = await cachedBytes('https://relayer.test');
        expect(plain).not.toBe(placeholder);
        expect(plain?.publicKeyBytes.bytes).not.toEqual(placeholder.publicKeyBytes.bytes);
      }
      const plain = await cachedBytes('https://relayer.test');
      expect(plain?.publicKeyBytes.bytes).toEqual(PUBLIC_KEY);
    },
  );
});

////////////////////////////////////////////////////////////////////////////////
// fetchFheEncryptionKeyBytes — one shared download, one check per client
////////////////////////////////////////////////////////////////////////////////

describe('fetchFheEncryptionKeyBytes with several clients', () => {
  const fhevmContext = createFhevmClientFrozenContext({});
  const KEY = 'https://relayer.test';
  const ethereumClient = { name: 'ethereum' };

  // Digests computed once, so that a read hashes nothing.
  const PUBLIC_KEY_DIGEST = digest('PDAT_KEY', PUBLIC_KEY);
  const CRS_DIGEST = digest('PDAT_CRS', CRS);
  function precomputedHandlers(overrides: Partial<Handlers> = {}): Handlers {
    return kmsGenerationHandlers({
      getKeyMaterials: () => [[], [{ keyType: 1, digest: PUBLIC_KEY_DIGEST }]],
      getCrsMaterials: () => [[], CRS_DIGEST],
      ...overrides,
    });
  }

  function deferred(): { readonly promise: Promise<void>; readonly resolve: () => void } {
    let resolve!: () => void;
    const promise = new Promise<void>((res) => {
      resolve = res;
    });
    return { promise, resolve };
  }

  const countReads = (readContract: { mock: { calls: unknown[][] } }, fn: string): number =>
    readContract.mock.calls.filter((c) => (c[1] as { functionName: string }).functionName === fn).length;

  it('downloads and hashes the key once, and checks it once per client', async () => {
    const a = makeClient({ chain: ethereumChain, handlers: precomputedHandlers() });
    const b = makeClient({ chain: ethereumChain, handlers: precomputedHandlers() });
    const hashers = shake.hashers;
    const workerRuns = digestsWorkerRuns();

    const [bytesA, bytesB] = await Promise.all([
      fetchFheEncryptionKeyBytes(a.client, { fhevmContext }),
      fetchFheEncryptionKeyBytes(b.client, { fhevmContext }),
    ]);
    expect(bytesB).toBe(bytesA);
    expect(a.fetchFheEncryptionKeyBytesFn.mock.calls.length + b.fetchFheEncryptionKeyBytesFn.mock.calls.length).toBe(1);
    // One worker run for the public key and the CRS, none on the calling thread.
    expect(digestsWorkerRuns() - workerRuns).toBe(1);
    expect(shake.hashers - hashers).toBe(0);
    expect(countReads(a.readContract, 'getKeyMaterials')).toBe(1);
    expect(countReads(b.readContract, 'getKeyMaterials')).toBe(1);

    // Each client checked it already: no more RPC call.
    await fetchFheEncryptionKeyBytes(a.client, { fhevmContext });
    await fetchFheEncryptionKeyBytes(b.client, { fhevmContext });
    expect(countReads(a.readContract, 'getKeyMaterials')).toBe(1);
    expect(countReads(b.readContract, 'getKeyMaterials')).toBe(1);
  });

  it.each(['first', 'second'])(
    'fails only the client whose keyVerification client is on another network (%s to fetch)',
    async (order) => {
      const good = makeClient({
        chain: otherHostChain,
        handlers: precomputedHandlers(),
        keyVerification: { chain: sepolia, client: ethereumClient },
      });
      const bad = makeClient({
        chain: otherHostChain,
        handlers: precomputedHandlers(),
        keyVerification: { chain: sepolia, client: { chainId: 1n } },
      });
      const clients = order === 'first' ? [bad.client, good.client] : [good.client, bad.client];

      const results = await Promise.allSettled(clients.map((c) => fetchFheEncryptionKeyBytes(c, { fhevmContext })));
      const [badResult, goodResult] = order === 'first' ? results : [results[1], results[0]];
      expect(badResult?.status).toBe('rejected');
      expect((badResult as PromiseRejectedResult).reason).toBeInstanceOf(FhevmConfigError);
      expect(goodResult?.status).toBe('fulfilled');
      // Not a key mismatch: the downloaded key stays cached.
      expect(globalFheEncryptionKeyCache.has(KEY)).toBe(true);

      // Not remembered: the next call of the failed client checks again, with
      // no new download.
      const chainIdReads = bad.getChainId.mock.calls.length;
      await expect(fetchFheEncryptionKeyBytes(bad.client, { fhevmContext })).rejects.toThrow(FhevmConfigError);
      expect(bad.getChainId).toHaveBeenCalledTimes(chainIdReads + 1);
      expect(
        good.fetchFheEncryptionKeyBytesFn.mock.calls.length + bad.fetchFheEncryptionKeyBytesFn.mock.calls.length,
      ).toBe(1);
    },
  );

  it('uses the entry it checked when a provided key replaces the slot during its check', async () => {
    const OLD = keyBytes();
    const NEW = keyBytes({ publicKey: new Uint8Array([0xb0]) as Bytes, crs: new Uint8Array([0xb1]) as Bytes });
    const gate = deferred();
    // Client A still sees OLD as active, client B sees NEW (a key rotation).
    const a = makeClient({
      chain: ethereumChain,
      handlers: precomputedHandlers({
        getCrsMaterials: async () => {
          await gate.promise;
          return [[], CRS_DIGEST];
        },
      }),
      relayer: { fetchFheEncryptionKeyBytes: vi.fn(async () => OLD) },
    });
    const b = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getKeyMaterials: () => [[], [{ keyType: 1, digest: digest('PDAT_KEY', NEW.publicKeyBytes.bytes) }]],
        getCrsMaterials: () => [[], digest('PDAT_CRS', NEW.crsBytes.bytes)],
      }),
      fheEncryptionKey: NEW,
    });
    const deserialize = vi.fn(async ({ publicKeyBytes }: { readonly publicKeyBytes: { readonly bytes: Bytes } }) => ({
      bytes: publicKeyBytes.bytes,
    }));
    a.client.runtime.extend((() => ({
      encrypt: { deserializeFheEncryptionPublicKey: deserialize, deserializeFheEncryptionCrs: async () => ({}) },
    })) as unknown as EncryptModuleFactory);

    const bytesA = fetchFheEncryptionKeyBytes(a.client, { fhevmContext });
    const wasmA = fetchFheEncryptionKeyWasm(a.client as unknown as Fhevm<FhevmChain, WithEncrypt>, { fhevmContext });
    await vi.waitFor(() => expect(countReads(a.readContract, 'getCrsMaterials')).toBe(1));
    const oldEntry = globalFheEncryptionKeyCache.get(KEY);

    await expect(fetchFheEncryptionKeyBytes(b.client, { fhevmContext })).resolves.toEqual(NEW);
    expect(globalFheEncryptionKeyCache.get(KEY)).not.toBe(oldEntry);

    gate.resolve();
    await expect(bytesA).resolves.toBe(OLD);
    const wasm = (await wasmA) as unknown as { readonly publicKey: { readonly bytes: Bytes } };
    expect(wasm.publicKey.bytes).toBe(OLD.publicKeyBytes.bytes);
    // The slot keeps B's key.
    await expect(cachedBytes(KEY)).resolves.toEqual(NEW);
  });

  it("uses a provided key without waiting for another client's download in flight", async () => {
    const gate = deferred();
    const download = vi.fn(async () => {
      await gate.promise;
      return keyBytes();
    });
    const a = makeClient({
      chain: ethereumChain,
      handlers: precomputedHandlers(),
      relayer: { fetchFheEncryptionKeyBytes: download },
    });
    const b = makeClient({ chain: ethereumChain, handlers: precomputedHandlers(), fheEncryptionKey: keyBytes() });

    const bytesA = fetchFheEncryptionKeyBytes(a.client, { fhevmContext });
    await vi.waitFor(() => expect(download).toHaveBeenCalledOnce());
    const pending = globalFheEncryptionKeyCache.get(KEY);

    // A's download is still pending: B checks its own key right away.
    await expect(fetchFheEncryptionKeyBytes(b.client, { fhevmContext })).resolves.toEqual(keyBytes());
    expect(b.fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
    expect(globalFheEncryptionKeyCache.get(KEY)).not.toBe(pending);

    gate.resolve();
    await expect(bytesA).resolves.toEqual(keyBytes());
  });

  it('shares one entry between concurrent clients providing equal keys: no orphaned entry', async () => {
    const a = makeClient({ chain: ethereumChain, handlers: precomputedHandlers(), fheEncryptionKey: keyBytes() });
    const b = makeClient({ chain: ethereumChain, handlers: precomputedHandlers(), fheEncryptionKey: keyBytes() });
    const deserialize = vi.fn(async ({ publicKeyBytes }: { readonly publicKeyBytes: { readonly bytes: Bytes } }) => ({
      bytes: publicKeyBytes.bytes,
    }));
    for (const c of [a, b]) {
      c.client.runtime.extend((() => ({
        encrypt: { deserializeFheEncryptionPublicKey: deserialize, deserializeFheEncryptionCrs: async () => ({}) },
      })) as unknown as EncryptModuleFactory);
    }
    const replaceBytes = vi.spyOn(globalFheEncryptionKeyCache, 'replaceBytes');

    try {
      const [bytesA, bytesB] = await Promise.all([
        fetchFheEncryptionKeyBytes(a.client, { fhevmContext }),
        fetchFheEncryptionKeyBytes(b.client, { fhevmContext }),
      ]);
      expect(bytesB).toBe(bytesA);
      expect(replaceBytes).toHaveBeenCalledOnce();
      await expect(cachedBytes(KEY)).resolves.toBe(bytesA);

      // Both clients use the slot's entry: one wasm conversion.
      await Promise.all(
        [a, b].map((c) =>
          fetchFheEncryptionKeyWasm(c.client as unknown as Fhevm<FhevmChain, WithEncrypt>, { fhevmContext }),
        ),
      );
      expect(deserialize).toHaveBeenCalledOnce();
      expect(replaceBytes).toHaveBeenCalledOnce();
    } finally {
      replaceBytes.mockRestore();
    }
  });

  it('evicts a downloaded key that fails the check, leaving the other holders of its entry as they are', async () => {
    const good = makeClient({ chain: ethereumChain, handlers: precomputedHandlers() });
    const gate = deferred();
    const deserialize = vi.fn(async ({ publicKeyBytes }: { readonly publicKeyBytes: { readonly bytes: Bytes } }) => {
      await gate.promise;
      return { bytes: publicKeyBytes.bytes };
    });
    good.client.runtime.extend((() => ({
      encrypt: { deserializeFheEncryptionPublicKey: deserialize, deserializeFheEncryptionCrs: async () => ({}) },
    })) as unknown as EncryptModuleFactory);
    const cached = await fetchFheEncryptionKeyBytes(good.client, { fhevmContext });
    const entry = globalFheEncryptionKeyCache.get(KEY);
    // A holder of the entry, converting it to wasm.
    const wasmGood = fetchFheEncryptionKeyWasm(good.client as unknown as Fhevm<FhevmChain, WithEncrypt>, {
      fhevmContext,
    });
    await vi.waitFor(() => expect(deserialize).toHaveBeenCalledOnce());
    // Another active key for this client (e.g. a wrong KMSGeneration).
    const bad = makeClient({
      chain: ethereumChain,
      handlers: precomputedHandlers({ getCrsMaterials: () => [[], digest('PDAT_CRS', new Uint8Array([0]))] }),
    });

    await expect(fetchFheEncryptionKeyBytes(bad.client, { fhevmContext })).rejects.toThrow(
      FheEncryptionKeyVerificationError,
    );
    expect(globalFheEncryptionKeyCache.has(KEY)).toBe(false);
    // Not invalidated: its holders still read the bytes, and the pending
    // conversion still gives the wasm key.
    await expect(globalFheEncryptionKeyCache.entryBytes(entry as CacheEntry)).resolves.toBe(cached);
    gate.resolve();
    const wasm = (await wasmGood) as unknown as { readonly publicKey: { readonly bytes: Bytes } };
    expect(wasm.publicKey.bytes).toBe(cached.publicKeyBytes.bytes);
    expect(entry?.resolvedKind).toBe('wasm');

    // The next call downloads again.
    await expect(fetchFheEncryptionKeyBytes(good.client, { fhevmContext })).resolves.toEqual(cached);
    expect(
      good.fetchFheEncryptionKeyBytesFn.mock.calls.length + bad.fetchFheEncryptionKeyBytesFn.mock.calls.length,
    ).toBe(2);
  });

  it('evicts a downloaded key that fails the check only if the slot still holds it', async () => {
    const gate = deferred();
    const bad = makeClient({
      chain: ethereumChain,
      handlers: precomputedHandlers({
        getCrsMaterials: async () => {
          await gate.promise;
          return [[], digest('PDAT_CRS', new Uint8Array([0]))];
        },
      }),
    });
    const fetched = fetchFheEncryptionKeyBytes(bad.client, { fhevmContext });
    await vi.waitFor(() => expect(countReads(bad.readContract, 'getCrsMaterials')).toBe(1));

    // A provided key (another one) takes the slot meanwhile.
    const NEW = keyBytes({ publicKey: new Uint8Array([0xb0]) as Bytes, crs: new Uint8Array([0xb1]) as Bytes });
    const provider = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers({
        getKeyMaterials: () => [[], [{ keyType: 1, digest: digest('PDAT_KEY', NEW.publicKeyBytes.bytes) }]],
        getCrsMaterials: () => [[], digest('PDAT_CRS', NEW.crsBytes.bytes)],
      }),
      fheEncryptionKey: NEW,
    });
    await fetchFheEncryptionKeyBytes(provider.client, { fhevmContext });
    const entry = globalFheEncryptionKeyCache.get(KEY);

    gate.resolve();
    await expect(fetched).rejects.toThrow(FheEncryptionKeyVerificationError);
    expect(globalFheEncryptionKeyCache.get(KEY)).toBe(entry);
  });

  it('keeps a downloaded key after a network error during the check: the next call checks it again', async () => {
    const networkError = new Error('fetch failed');
    let fail = true;
    const { client, readContract, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: precomputedHandlers({
        getActiveCrsId: () => {
          if (fail) {
            throw networkError;
          }
          return CRS_ID;
        },
      }),
    });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).rejects.toBe(networkError);
    expect(globalFheEncryptionKeyCache.has(KEY)).toBe(true);

    fail = false;
    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toEqual(keyBytes());
    expect(fetchFheEncryptionKeyBytesFn).toHaveBeenCalledOnce();
    expect(countReads(readContract, 'getActiveCrsId')).toBe(2);
  });
});

////////////////////////////////////////////////////////////////////////////////
// encrypt client — a tampered provided key makes `ready` reject
////////////////////////////////////////////////////////////////////////////////

describe('encrypt client with a tampered options.fheEncryptionKey', () => {
  const encryptModuleFactory = (() => ({
    encrypt: {
      initTfheModule: vi.fn(async () => undefined),
      deserializeFheEncryptionPublicKey: vi.fn(async () => ({})),
      deserializeFheEncryptionCrs: vi.fn(async () => ({})),
    },
  })) as unknown as EncryptModuleFactory;

  it.each([
    ['FHE public key', { publicKey: PUBLIC_KEY.map((b, i) => (i === 0 ? b ^ 1 : b)) as Bytes }],
    ['CRS', { crs: CRS.map((b, i) => (i === 0 ? b ^ 1 : b)) as Bytes }],
  ])('rejects ready and encryption on a %s one byte off', async (subject, overrides) => {
    const {
      client: base,
      readContract,
      fetchFheEncryptionKeyBytesFn,
    } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
      fheEncryptionKey: keyBytes(overrides),
    });
    // The version basis is not under test: skip its on-chain resolution.
    setFrozenContext(base, createFhevmClientFrozenContext({}));
    const client = base.extend((c) => encryptActionsWithModule(c, encryptModuleFactory));
    const mismatch = new RegExp(`${subject} digest mismatch`);

    const error = await client.ready.catch((e: unknown) => e);
    expect(error).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect(error).toHaveProperty('message', expect.stringMatching(mismatch));
    const reads = readContract.mock.calls.length;
    // The client drops its copy and keeps only the error.
    expect(getProvidedFheEncryptionKey(client)).toBeUndefined();
    expect(getProvidedFheEncryptionKeyError(client)).toBe(error);

    // The same error on later calls, with no RPC call and no relayer download.
    await expect(client.ready).rejects.toBe(error);
    await expect(
      client.encryptValue({
        value: { type: 'uint64', value: 42n },
        contractAddress: '0xa5e1defb98EFe38EBb2D958CEe052410247F4c80',
        userAddress: '0x8ba1f109551bD432803012645Ac136ddd64DBA72',
      }),
    ).rejects.toBe(error);
    expect(readContract).toHaveBeenCalledTimes(reads);
    expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
  });
});

////////////////////////////////////////////////////////////////////////////////
// encrypt — a key cache miss verifies through the client instance
////////////////////////////////////////////////////////////////////////////////

describe('encrypt', () => {
  const fhevmContext = createFhevmClientFrozenContext({});

  it('sends the runtime auth when it downloads the key into an empty cache slot', async () => {
    const auth: Auth = { type: 'ApiKeyHeader', value: 'secret' };
    const { client, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
      auth,
    });
    const encryptModuleFactory = (() => ({
      encrypt: {
        deserializeFheEncryptionPublicKey: vi.fn(async () => ({})),
        deserializeFheEncryptionCrs: vi.fn(async () => ({})),
        buildWithProofPacked: vi.fn(() => Promise.reject(new Error('stop after key'))),
      },
    })) as unknown as EncryptModuleFactory;
    client.runtime.extend(encryptModuleFactory);

    await expect(
      encrypt(client as unknown as Fhevm<FhevmChain, WithEncrypt>, {
        contractAddress: '0xa5e1defb98EFe38EBb2D958CEe052410247F4c80' as ChecksummedAddress,
        userAddress: '0x8ba1f109551bD432803012645Ac136ddd64DBA72' as ChecksummedAddress,
        values: [createTypedValue({ type: 'uint64', value: 42n })],
        fhevmContext,
      }),
    ).rejects.toThrow('stop after key');
    expect(fetchFheEncryptionKeyBytesFn).toHaveBeenCalledOnce();
    expect(fetchFheEncryptionKeyBytesFn.mock.calls[0]).toEqual([
      expect.anything(),
      expect.objectContaining({ options: expect.objectContaining({ auth }) }),
    ]);
  });

  it('verifies a re-fetched key through the host client after a cache eviction', async () => {
    const { client, hostClient, readClients, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
    });
    // Stops right after the key is fetched, verified and deserialized.
    const buildWithProofPacked = vi.fn(() => Promise.reject(new Error('stop after key')));
    const encryptModuleFactory = (() => ({
      encrypt: {
        deserializeFheEncryptionPublicKey: vi.fn(async () => ({})),
        deserializeFheEncryptionCrs: vi.fn(async () => ({})),
        buildWithProofPacked,
      },
    })) as unknown as EncryptModuleFactory;
    client.runtime.extend(encryptModuleFactory);

    await fetchFheEncryptionKeyBytes(client, { fhevmContext });
    globalFheEncryptionKeyCache.remove('https://relayer.test');
    readClients.length = 0;

    await expect(
      encrypt(client as unknown as Fhevm<FhevmChain, WithEncrypt>, {
        contractAddress: '0xa5e1defb98EFe38EBb2D958CEe052410247F4c80' as ChecksummedAddress,
        userAddress: '0x8ba1f109551bD432803012645Ac136ddd64DBA72' as ChecksummedAddress,
        values: [createTypedValue({ type: 'uint64', value: 42n })],
        fhevmContext,
      }),
    ).rejects.toThrow('stop after key');
    expect(fetchFheEncryptionKeyBytesFn).toHaveBeenCalledTimes(2);
    expect(buildWithProofPacked).toHaveBeenCalledOnce();
    expect(readClients.length).toBeGreaterThan(0);
    expect(readClients.every((c) => c === hostClient)).toBe(true);
  });
});

////////////////////////////////////////////////////////////////////////////////
// fetchFheEncryptionKeyBytes — returns the verified bytes, never a wasm round trip
////////////////////////////////////////////////////////////////////////////////

describe('fetchFheEncryptionKeyBytes after the wasm upgrade', () => {
  const fhevmContext = createFhevmClientFrozenContext({});
  const SIZE_LIMIT = BigInt(1024 * 1024 * 512);

  // The public key served by relayers today, in an older tfhe serialization
  // version: the part a wasm round trip re-encodes. The CRS goes through a
  // pass-through (de)serializer below, so a small one stands in for the
  // fixture's ~4.6MB CRS, whose SHAKE256 takes seconds under coverage.
  let fetched: FheEncryptionKeyBytes;

  beforeAll(() => {
    const json = JSON.parse(readFileSync(new URL('../../../test/keys/key.1.5.4.json', import.meta.url), 'utf-8')) as {
      readonly publicKeyBytes: { readonly id: string; readonly bytes: string };
    };
    fetched = {
      publicKeyBytes: {
        id: json.publicKeyBytes.id,
        bytes: new Uint8Array(Buffer.from(json.publicKeyBytes.bytes, 'base64')) as Bytes,
      },
      crsBytes: { id: 'crs-data-id', capacity: 2048 as UintNumber, bytes: CRS },
      metadata: { relayerUrl: 'https://relayer.test', chainId: 0 },
    } as unknown as FheEncryptionKeyBytes;
    tfhe.initSync({ module: readFileSync(new URL('../../wasm/tfhe/v1.8.1/tfhe_bg.wasm', import.meta.url)) });
  });

  // Real tfhe for the public key (the part re-encoded), pass-through CRS.
  const tfheEncryptModuleFactory = (() => ({
    encrypt: {
      deserializeFheEncryptionPublicKey: async ({ publicKeyBytes }: { readonly publicKeyBytes: { bytes: Bytes } }) =>
        tfhe.TfheCompactPublicKey.safe_deserialize(publicKeyBytes.bytes, SIZE_LIMIT),
      serializeFheEncryptionPublicKey: async ({ publicKey }: { readonly publicKey: tfhe.TfheCompactPublicKey }) => ({
        id: 'fhe-public-key-data-id',
        bytes: publicKey.safe_serialize(SIZE_LIMIT),
      }),
      deserializeFheEncryptionCrs: async ({ crsBytes }: { readonly crsBytes: unknown }) => crsBytes,
      serializeFheEncryptionCrs: async ({ crs }: { readonly crs: unknown }) => crs,
    },
  })) as unknown as EncryptModuleFactory;

  function kmsGenerationHandlersFor(bytes: FheEncryptionKeyBytes): Handlers {
    return kmsGenerationHandlers({
      getKeyMaterials: () => [[], [{ keyType: 1, digest: digest('PDAT_KEY', bytes.publicKeyBytes.bytes) }]],
      getCrsMaterials: () => [[], digest('PDAT_CRS', bytes.crsBytes.bytes)],
    });
  }

  it('a tfhe round trip re-encodes the key.1.5.4 public key', () => {
    const original = fetched.publicKeyBytes.bytes;
    const roundTrip = tfhe.TfheCompactPublicKey.safe_deserialize(original, SIZE_LIMIT).safe_serialize(SIZE_LIMIT);

    expect(Buffer.from(roundTrip).equals(Buffer.from(original))).toBe(false);
  });

  it('returns the bytes as fetched once the entry holds the wasm key, and they verify when provided back', async () => {
    const handlers = kmsGenerationHandlersFor(fetched);
    const { client } = makeClient({
      chain: ethereumChain,
      handlers,
      relayer: { fetchFheEncryptionKeyBytes: vi.fn(async () => fetched) },
    });
    client.runtime.extend(tfheEncryptModuleFactory);

    await fetchFheEncryptionKeyWasm(client as unknown as Fhevm<FhevmChain, WithEncrypt>, { fhevmContext });

    const returned = await fetchFheEncryptionKeyBytes(client, { fhevmContext });
    expect(returned).toBe(fetched);

    // Passed back as options.fheEncryptionKey (e.g. after persisting it).
    globalFheEncryptionKeyCache.clear();
    const { client: next, fetchFheEncryptionKeyBytesFn } = makeClient({
      chain: ethereumChain,
      handlers,
      fheEncryptionKey: returned,
    });
    await expect(fetchFheEncryptionKeyBytes(next, { fhevmContext })).resolves.toEqual(returned);
    expect(fetchFheEncryptionKeyBytesFn).not.toHaveBeenCalled();
  });
});

////////////////////////////////////////////////////////////////////////////////
// FheEncryptionKeyCache — entries never change in place
////////////////////////////////////////////////////////////////////////////////

describe('FheEncryptionKeyCache entries', () => {
  const fhevmContext = createFhevmClientFrozenContext({});
  const KEY = 'https://relayer.test';
  const OLD = keyBytes({ publicKey: new Uint8Array([0xa0]) as Bytes, crs: new Uint8Array([0xa1]) as Bytes });
  const NEW = keyBytes({ publicKey: new Uint8Array([0xb0]) as Bytes, crs: new Uint8Array([0xb1]) as Bytes });

  function deferred<T>(): {
    readonly promise: Promise<T>;
    readonly resolve: (value: T) => void;
    readonly reject: (error: unknown) => void;
  } {
    let resolve!: (value: T) => void;
    let reject!: (error: unknown) => void;
    const promise = new Promise<T>((res, rej) => {
      resolve = res;
      reject = rej;
    });
    return { promise, resolve, reject };
  }

  // KMSGeneration whose active key is whatever `active()` returns.
  function activeKeyHandlers(active: () => FheEncryptionKeyBytes): Handlers {
    return kmsGenerationHandlers({
      getKeyMaterials: () => [[], [{ keyType: 1, digest: digest('PDAT_KEY', active().publicKeyBytes.bytes) }]],
      getCrsMaterials: () => [[], digest('PDAT_CRS', active().crsBytes.bytes)],
    });
  }

  // The deserialized public key carries its bytes: `wasm.publicKey.bytes`.
  function withEncrypt(
    client: Fhevm<FhevmChain>,
    deserialize: (publicKeyBytes: Bytes) => Promise<{ readonly bytes: Bytes }>,
  ): Fhevm<FhevmChain, WithEncrypt> {
    client.runtime.extend((() => ({
      encrypt: {
        deserializeFheEncryptionPublicKey: ({
          publicKeyBytes,
        }: {
          readonly publicKeyBytes: { readonly bytes: Bytes };
        }) => deserialize(publicKeyBytes.bytes),
        deserializeFheEncryptionCrs: async () => ({}),
      },
    })) as unknown as EncryptModuleFactory);
    return client as unknown as Fhevm<FhevmChain, WithEncrypt>;
  }

  it('gives bytes callers the bytes while the wasm conversion never settles', async () => {
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: activeKeyHandlers(() => OLD),
      relayer: { fetchFheEncryptionKeyBytes: vi.fn(async () => OLD) },
    });
    const deserialize = vi.fn(() => new Promise<never>(() => {}));
    const encryptClient = withEncrypt(client, deserialize);

    void fetchFheEncryptionKeyWasm(encryptClient, { fhevmContext });
    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toBe(OLD);
    expect(deserialize).toHaveBeenCalledOnce();
  });

  it('rejects every concurrent bytes and wasm caller with the same verification error', async () => {
    const fetched = deferred<FheEncryptionKeyBytes>();
    const { client } = makeClient({
      chain: ethereumChain,
      // The relayer serves OLD, KMSGeneration has NEW.
      handlers: activeKeyHandlers(() => NEW),
      relayer: { fetchFheEncryptionKeyBytes: vi.fn(() => fetched.promise) },
    });
    const deserialize = vi.fn(async (bytes: Bytes) => ({ bytes }));
    const encryptClient = withEncrypt(client, deserialize);

    // Cold fetch, with wasm conversions chained while it is in flight.
    const calls = [
      fetchFheEncryptionKeyBytes(client, { fhevmContext }),
      fetchFheEncryptionKeyWasm(encryptClient, { fhevmContext }),
      fetchFheEncryptionKeyBytes(client, { fhevmContext }),
      fetchFheEncryptionKeyWasm(encryptClient, { fhevmContext }),
    ];
    fetched.resolve(OLD);
    const results = await Promise.allSettled(calls);

    const errors = results.map((r) => (r as PromiseRejectedResult).reason as unknown);
    expect(results.every((r) => r.status === 'rejected')).toBe(true);
    expect(errors[0]).toBeInstanceOf(FheEncryptionKeyVerificationError);
    expect((errors[0] as Error).message).toMatch(/digest mismatch/);
    expect(errors.every((e) => e === errors[0])).toBe(true);
    expect(deserialize).not.toHaveBeenCalled();
    expect(globalFheEncryptionKeyCache.has(KEY)).toBe(false);
  });

  it('takes provided bytes identical to the cached ones but with other ids as cached: checked, no replacement', async () => {
    const labelIdBytes = keyBytes({ publicKeyId: 'fhe-public-key-data-id', crsId: 'crs-data-id' });
    const warm = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
      relayer: { fetchFheEncryptionKeyBytes: vi.fn(async () => labelIdBytes) },
    });
    await fetchFheEncryptionKeyBytes(warm.client, { fhevmContext });
    const entry = globalFheEncryptionKeyCache.get(KEY);

    // Same bytes, with the 32-byte hex ids.
    const { client, readContract } = makeClient({
      chain: ethereumChain,
      handlers: kmsGenerationHandlers(),
      fheEncryptionKey: keyBytes(),
    });
    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toBe(labelIdBytes);
    expect(readContract).toHaveBeenCalled();
    expect(globalFheEncryptionKeyCache.get(KEY)).toBe(entry);
  });

  it('rejects bytes from another relayer than the one passed, leaving the slot as is', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: {} });
    const other = { ...keyBytes(), metadata: { relayerUrl: 'https://other.test', chainId: 0 } };

    expect(() => globalFheEncryptionKeyCache.setBytes(client.runtime, KEY, KEY, other)).toThrow(/relayerUrl mismatch/);
    expect(globalFheEncryptionKeyCache.has(KEY)).toBe(false);

    globalFheEncryptionKeyCache.setBytes(client.runtime, KEY, KEY, OLD);
    const entry = globalFheEncryptionKeyCache.get(KEY);
    expect(() => globalFheEncryptionKeyCache.replaceBytes(client.runtime, KEY, KEY, other)).toThrow(
      /relayerUrl mismatch/,
    );
    expect(globalFheEncryptionKeyCache.get(KEY)).toBe(entry);
  });

  it('rejects downloaded bytes from another relayer than the one passed', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: {} });
    const other = { ...keyBytes(), metadata: { relayerUrl: 'https://other.test', chainId: 0 } };

    const entry = globalFheEncryptionKeyCache.ensureBytes({
      owner: client.runtime,
      key: KEY,
      fetcher: async () => other,
      metadata: { chainId: 0, relayerUrl: KEY },
    });
    await expect(entry.ready).rejects.toThrow(/relayerUrl mismatch/);
  });
});

////////////////////////////////////////////////////////////////////////////////
// verifyFheEncryptionKeyBytes — hashing the key and CRS
////////////////////////////////////////////////////////////////////////////////

describe('verifyFheEncryptionKeyBytes hashing', () => {
  // Digests computed once, so that a read hashes nothing.
  const PUBLIC_KEY_DIGEST = digest('PDAT_KEY', PUBLIC_KEY);
  const CRS_DIGEST = digest('PDAT_CRS', CRS);
  function hashedHandlers(overrides: Partial<Handlers> = {}): Handlers {
    return kmsGenerationHandlers({
      getKeyMaterials: () => [[], [{ keyType: 1, digest: PUBLIC_KEY_DIGEST }]],
      getCrsMaterials: () => [[], CRS_DIGEST],
      ...overrides,
    });
  }

  // Holds getActiveKeyId, so that round 1 stays in flight.
  function gatedRoundOne(): { readonly handlers: Handlers; readonly release: () => void } {
    let release!: () => void;
    const gate = new Promise<void>((resolve) => {
      release = resolve;
    });
    const handlers = hashedHandlers({
      getActiveKeyId: async () => {
        await gate;
        return KEY_ID;
      },
    });
    return { handlers, release };
  }

  it('hashes in a worker, leaving the cached bytes attached', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: hashedHandlers() });
    const bytes = keyBytes();
    const hashers = shake.hashers;
    const workerRuns = digestsWorkerRuns();

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).resolves.toBe(bytes);
    expect(digestsWorkerRuns() - workerRuns).toBe(1);
    expect(shake.hashers - hashers).toBe(0);
    const [, input] = digestsWorkerCalls().at(-1) ?? [];
    expect(input).toEqual({ publicKey: PUBLIC_KEY, crs: CRS });
    // Cloned, not transferred.
    expect(bytes.publicKeyBytes.bytes.byteLength).toBe(PUBLIC_KEY.length);
    expect(bytes.crsBytes.bytes.byteLength).toBe(CRS.length);
    expect(bytes.publicKeyBytes.bytes).toEqual(new Uint8Array([1, 2, 3, 4, 5]));
  });

  it('hashes on the calling thread when no blob / eval worker can run', async () => {
    vi.mocked(isBlobWorkerSupported).mockResolvedValueOnce(false);
    const { client } = makeClient({ chain: ethereumChain, handlers: hashedHandlers() });
    const bytes = keyBytes();
    const hashers = shake.hashers;
    const workerRuns = digestsWorkerRuns();

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).resolves.toBe(bytes);
    expect(digestsWorkerRuns() - workerRuns).toBe(0);
    // One hasher for the public key, one for the CRS.
    expect(shake.hashers - hashers).toBe(2);
  });

  it('hashes on the calling thread when the worker fails', async () => {
    const debug = vi.fn();
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: hashedHandlers(),
      logger: { debug, warn: undefined, error: undefined },
    });
    vi.mocked(runCodeInIsomorphicWorker).mockRejectedValueOnce(new Error('Mocked worker failure'));
    const hashers = shake.hashers;
    const workerRuns = digestsWorkerRuns();

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).resolves.toBeDefined();
    expect(digestsWorkerRuns() - workerRuns).toBe(1);
    expect(shake.hashers - hashers).toBe(2);
    expect(debug).toHaveBeenCalledWith(expect.stringMatching(/Mocked worker failure.*calling thread/));
  });

  it('hashes on the calling thread when the worker returns invalid digests', async () => {
    const debug = vi.fn();
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: hashedHandlers(),
      logger: { debug, warn: undefined, error: undefined },
    });
    vi.mocked(runCodeInIsomorphicWorker).mockResolvedValueOnce({
      publicKey: new Uint8Array(1),
      crs: new Uint8Array(32),
    });
    const hashers = shake.hashers;
    const workerRuns = digestsWorkerRuns();

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).resolves.toBeDefined();
    expect(digestsWorkerRuns() - workerRuns).toBe(1);
    expect(shake.hashers - hashers).toBe(2);
    expect(debug).toHaveBeenCalledWith(expect.stringMatching(/Invalid FHE public key and CRS digests.*calling thread/));
  });

  it('rejects a digest mismatch when hashing on the calling thread', async () => {
    vi.mocked(isBlobWorkerSupported).mockResolvedValueOnce(false);
    const { client } = makeClient({ chain: ethereumChain, handlers: hashedHandlers() });

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes({ crs: new Uint8Array([9]) as Bytes }))).rejects.toThrow(
      /CRS digest mismatch/,
    );
  });

  it('starts the worker hash before round 1 resolves', async () => {
    const { handlers, release } = gatedRoundOne();
    const { client, rpcLog } = makeClient({ chain: ethereumChain, handlers });
    const workerRuns = digestsWorkerRuns();

    const verified = verifyFheEncryptionKeyBytes(client, keyBytes());
    await vi.waitFor(() => {
      expect(digestsWorkerRuns() - workerRuns).toBe(1);
    });
    expect(rpcLog).not.toContain('end:getActiveCrsId');

    release();
    await expect(verified).resolves.toBeDefined();
  });

  it('hashes on the calling thread only at the digest compare, after both rounds', async () => {
    vi.mocked(isBlobWorkerSupported).mockResolvedValueOnce(false);
    let release!: () => void;
    const gate = new Promise<void>((resolve) => {
      release = resolve;
    });
    const { client, rpcLog } = makeClient({
      chain: ethereumChain,
      handlers: hashedHandlers({
        getCrsMaterials: async () => {
          await gate;
          return [[], CRS_DIGEST];
        },
      }),
    });
    const hashers = shake.hashers;

    const verified = verifyFheEncryptionKeyBytes(client, keyBytes());
    await vi.waitFor(() => {
      expect(rpcLog).toContain('end:getCrsMaterials');
    });
    expect(shake.hashers - hashers).toBe(0);

    release();
    await expect(verified).resolves.toBeDefined();
    expect(shake.hashers - hashers).toBe(2);
  });

  it('does not hash on the calling thread when round 1 fails', async () => {
    vi.mocked(isBlobWorkerSupported).mockResolvedValueOnce(false);
    const { client } = makeClient({
      chain: ethereumChain,
      handlers: hashedHandlers({
        getActiveCrsId: () => {
          throw new Error('Mocked read failure');
        },
      }),
    });
    const hashers = shake.hashers;

    await expect(verifyFheEncryptionKeyBytes(client, keyBytes())).rejects.toThrow();
    expect(shake.hashers - hashers).toBe(0);
  });

  it('hashes a bytes object once for concurrent and later checks', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: hashedHandlers() });
    const bytes = keyBytes();
    const workerRuns = digestsWorkerRuns();

    await Promise.all([verifyFheEncryptionKeyBytes(client, bytes), verifyFheEncryptionKeyBytes(client, bytes)]);
    await verifyFheEncryptionKeyBytes(client, bytes);
    expect(digestsWorkerRuns() - workerRuns).toBe(1);
  });

  it('hashes a bytes object once on the calling thread for concurrent checks', async () => {
    // The worker probe answers false once every check awaits it.
    let release!: () => void;
    const gate = new Promise<void>((resolve) => {
      release = resolve;
    });
    vi.mocked(isBlobWorkerSupported).mockImplementationOnce(async () => {
      await gate;
      return false;
    });
    const { client, rpcLog } = makeClient({ chain: ethereumChain, handlers: hashedHandlers() });
    const bytes = keyBytes();
    const hashers = shake.hashers;

    const verified = Promise.all(Array.from({ length: 3 }, () => verifyFheEncryptionKeyBytes(client, bytes)));
    await vi.waitFor(() => {
      expect(rpcLog.filter((entry) => entry === 'end:getCrsMaterials')).toHaveLength(3);
    });
    release();
    await verified;
    // One hasher for the public key, one for the CRS.
    expect(shake.hashers - hashers).toBe(2);
  });
});
