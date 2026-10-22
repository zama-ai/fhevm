import type { EthereumModule, TrustedClient } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FheEncryptionKeyBytes } from '../types/fheEncryptionKey.js';
import type { FhevmKeyVerification } from '../types/coreFhevmClient.js';
import type { Bytes, BytesHex, UintNumber } from '../types/primitives.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { shake256 } from '@noble/hashes/sha3.js';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { polygonAmoy } from '../chains/definitions/polygonAmoy.js';
import { bytesToHex } from '../base/bytes.js';
import { verifyTrustedValue } from '../base/trustedValue.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { FheEncryptionKeyVerificationError } from '../errors/FheEncryptionKeyVerificationError.js';
import { assertKeyVerificationConfig, verifyFheEncryptionKeyBytes } from './verifyFheEncryptionKeyBytes-p.js';
import { fetchFheEncryptionKeyBytes } from './fetchFheEncryptionKeyBytes.js';
import { globalFheEncryptionKeyCache } from './FheEncryptionKeyCache-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/key/verifyFheEncryptionKeyBytes-p.test.ts
////////////////////////////////////////////////////////////////////////////////

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
}) {
  const readClients: unknown[] = [];
  const readContract = vi.fn(
    async (trustedClient: TrustedClient, p: { readonly functionName: string; readonly args: readonly unknown[] }) => {
      readClients.push(verifyTrustedValue(trustedClient, PRIVATE_ETHERS_TOKEN));
      const handler = parameters.handlers[p.functionName];
      if (handler === undefined) {
        throw new Error(`No mocked handler for ${p.functionName}`);
      }
      return handler(p);
    },
  );
  const getChainId = vi.fn(async () => parameters.keyVerificationChainId ?? BigInt(sepolia.id));
  const ethereum = { readContract, getChainId } as unknown as EthereumModule;
  const fetchFheEncryptionKeyBytesFn = vi.fn(async () => keyBytes());

  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum,
    relayer: { fetchFheEncryptionKeyBytes: fetchFheEncryptionKeyBytesFn, ...parameters.relayer } as RelayerModule,
    config: {},
  });

  const hostClient = { name: 'host' };
  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
    chain: parameters.chain,
    client: hostClient,
    runtime,
    options: { fheEncryptionKey: parameters.fheEncryptionKey },
    keyVerification: parameters.keyVerification,
  });

  return { client, hostClient, readClients, readContract, getChainId, fetchFheEncryptionKeyBytesFn };
}

beforeEach(() => {
  globalFheEncryptionKeyCache.clear();
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

  it('rejects a 32-byte id that is not the active one', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ publicKeyId: id32(KEY_ID - 1n) });

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).rejects.toThrow(/FHE public key id mismatch/);
  });

  it('accepts a relayer label id when the digests match', async () => {
    const { client } = makeClient({ chain: ethereumChain, handlers: kmsGenerationHandlers() });
    const bytes = keyBytes({ publicKeyId: 'fhe-public-key-data-id', crsId: 'crs-data-id' });

    await expect(verifyFheEncryptionKeyBytes(client, bytes)).resolves.toBe(bytes);
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
    expect(globalFheEncryptionKeyCache.get('https://relayer.test')?.resolvedKind).toBe('bytes');
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

  it('does not check placeholder bytes from a cleartext relayer module', async () => {
    const { client, readContract } = makeClient({
      chain: otherHostChain,
      handlers: {},
      relayer: { servesPlaceholderFheEncryptionKey: true },
    });

    await expect(fetchFheEncryptionKeyBytes(client, { fhevmContext })).resolves.toBeDefined();
    expect(readContract).not.toHaveBeenCalled();
  });
});
