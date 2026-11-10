import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmCacheOptions } from '../types/cache.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { RelayerApiError } from '../types/relayer-p.js';
import type { UsedCacheVersions } from './usedCacheVersions-p.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { invalidateVersionCache } from '../host-contracts/HostContractVersion-p.js';
import { ThresholdSignerError } from '../errors/SignersError.js';
import { RelayerResponseApiError } from '../errors/RelayerResponseApiError.js';
import { CachedValueInvalidError } from '../errors/CachedValueInvalidError.js';
import {
  coprocessorSignersVersion,
  getCoprocessorContextSignersAndThreshold,
} from '../host-contracts/getCoprocessorContextSignersAndThreshold-p.js';
import { kmsContextVersion, readCurrentKmsSignersContext } from '../host-contracts/readKmsSignersContext-p.js';
import { recordUsedCacheVersion } from './usedCacheVersions-p.js';
import { RelayerAbortError } from '../errors/RelayerAbortError.js';
import { bytes32ToHandle, FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION } from '../handle/FhevmHandle.js';
import { asBytes32 } from '../base/bytes.js';
import { publicDecryptWithGateway as publicDecrypt } from '../kms/publicDecryptWithGateway.js';
import { createKmsExtraDataV2 } from '../kms/kmsExtraData-p.js';
import { forceRefreshCachedValue, retryOnStaleCache } from './retryOnStaleCache-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/caching/retryOnStaleCache-p.test.ts
//
// Refresh on failure: a request failing on a stale cached value refreshes it
// and runs once more (or throws CachedValueInvalidError when automatic
// refresh is off). The "request" here is a stand-in that reads the cached
// value as encrypt / publicDecrypt do, and fails the way the relayer or the
// signature check fails when that value is stale.
////////////////////////////////////////////////////////////////////////////////

const SIGNER_A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;
const SIGNER_B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;

const fhevmContext = createFhevmClientFrozenContext({});

const INPUT_VERIFIER = sepolia.fhevm.contracts.inputVerifier.address as ChecksummedAddress;
const KMS_VERIFIER = sepolia.fhevm.contracts.kmsVerifier.address as ChecksummedAddress;

/** The on-chain state: coprocessor signer, KMS context; and a logger. */
function makeWorld(cache?: FhevmCacheOptions, relayer: RelayerModule = {} as RelayerModule) {
  const chainState = { coprocessorSigner: SIGNER_A, kmsContextId: 5n, rpcDown: false };
  const readContract = vi.fn(async (_client: unknown, p: { readonly functionName: string }) => {
    await Promise.resolve();
    if (chainState.rpcDown) {
      throw new Error('RPC down');
    }
    switch (p.functionName) {
      case 'getThreshold':
        return 1n;
      case 'getCoprocessorSigners':
        return [chainState.coprocessorSigner];
      case 'getCurrentKmsContextAndEpoch':
        return [chainState.kmsContextId, 1n];
      case 'getContextSignersAndThresholdFromExtraData':
        return [[SIGNER_A], 1n];
      case 'isAllowedForDecryption':
        return true;
      default:
        throw new Error(`No mocked handler for ${p.functionName}`);
    }
  });
  const warn = vi.fn();
  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum: { readContract } as unknown as EthereumModule,
    relayer,
    config: { logger: { debug: undefined, warn, error: undefined } },
  });
  const client = createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain: sepolia, client: {}, runtime, options: { cache } });
  const reads = (functionName: string) =>
    readContract.mock.calls.filter((call) => (call[1] as { functionName: string }).functionName === functionName)
      .length;
  return { client, chainState, reads, warn };
}

type World = ReturnType<typeof makeWorld>;

/**
 * An input proof request: the coprocessors sign with the on-chain signer; the
 * SDK checks against the cached set.
 */
function inputProofRequest(world: World, options: { readonly badSignature?: boolean } = {}) {
  const run = vi.fn(async (usedVersions: UsedCacheVersions) => {
    const signedBy = options.badSignature === true ? SIGNER_B : world.chainState.coprocessorSigner;
    const cached = await getCoprocessorContextSignersAndThreshold(world.client, { address: INPUT_VERIFIER });
    recordUsedCacheVersion(usedVersions, 'coprocessorSigners', coprocessorSignersVersion(cached));
    if (!cached.signers.includes(signedBy)) {
      throw new ThresholdSignerError({ type: 'coprocessor' });
    }
    return signedBy;
  });
  const send = () => retryOnStaleCache(world.client, fhevmContext, run);
  return { run, send };
}

function relayerApiError(relayerApiError: RelayerApiError): RelayerResponseApiError {
  return new RelayerResponseApiError({
    url: 'https://relayer.test',
    fetchMethod: 'GET',
    operation: 'PUBLIC_DECRYPT',
    retryCount: 0,
    elapsed: 0,
    status: 400,
    state: {
      aborted: false,
      canceled: false,
      failed: true,
      fetching: false,
      running: false,
      succeeded: false,
      terminated: true,
      timeout: false,
    },
    relayerApiError,
  } as unknown as ConstructorParameters<typeof RelayerResponseApiError>[0]);
}

function relayerAbortError(): RelayerAbortError {
  return new RelayerAbortError({ url: 'https://relayer.test', operation: 'PUBLIC_DECRYPT' });
}

type KmsFailure = 'apiError' | 'timeout' | 'abort';

/**
 * A public decryption request, as publicDecrypt runs it: reads the cached KMS
 * context, records it once the request is sent, and fails the way the relayer
 * reports a refused context (`failure`) when the context is not the current
 * one. With `failLocally`, it fails before sending (an ACL denial).
 */
function publicDecryptRequest(
  world: World,
  options: { readonly failure?: KmsFailure; readonly failLocally?: boolean } = {},
) {
  const run = vi.fn(async (usedVersions: UsedCacheVersions) => {
    const context = await readCurrentKmsSignersContext(world.client, {
      kmsVerifierAddress: KMS_VERIFIER,
      // As publicDecrypt does: the chain's addresses name the cache slot.
      protocolConfigAddress: sepolia.fhevm.contracts.protocolConfig?.address as ChecksummedAddress | undefined,
    });
    if (options.failLocally === true) {
      throw new Error('ACL denied');
    }
    recordUsedCacheVersion(usedVersions, 'kmsContext', kmsContextVersion(context.id, context.epochId));
    if (context.id !== world.chainState.kmsContextId) {
      switch (options.failure ?? 'timeout') {
        case 'apiError':
          throw relayerApiError({ label: 'internal_server_error', message: 'Processing Failed' });
        case 'timeout':
          throw relayerApiError({ label: 'response_timed_out', message: 'timed out' });
        case 'abort':
          throw relayerAbortError();
      }
    }
    return context.id;
  });
  const send = () => retryOnStaleCache(world.client, fhevmContext, run);
  return { run, send };
}

beforeEach(() => {
  invalidateVersionCache({ includeInflight: true });
});

////////////////////////////////////////////////////////////////////////////////

// The rules are the same for every cache: tested once, on a coprocessor
// signer change (signer errors are its stale signal).
describe('refresh and retry', () => {
  it('a signer change: refreshes once, retries once, succeeds, and logs it', async () => {
    const world = makeWorld();
    const request = inputProofRequest(world);
    await request.send();
    world.chainState.coprocessorSigner = SIGNER_B;

    await expect(request.send()).resolves.toBe(SIGNER_B);
    expect(request.run).toHaveBeenCalledTimes(3);
    expect(world.warn).toHaveBeenCalledTimes(1);
    expect(world.warn.mock.calls[0]?.[0]).toContain('coprocessorSigners');
  });

  it('a bad signature with an unchanged set: today’s signer error, no refresh, no retry', async () => {
    const world = makeWorld();
    const request = inputProofRequest(world, { badSignature: true });
    await expect(request.send()).rejects.toBeInstanceOf(ThresholdSignerError);
    expect(request.run).toHaveBeenCalledTimes(1);
    expect(world.warn).not.toHaveBeenCalled();
  });

  it('a failure to read the set again: the request error as is, not the read error', async () => {
    const world = makeWorld();
    const request = inputProofRequest(world, { badSignature: true });
    await expect(request.send()).rejects.toBeInstanceOf(ThresholdSignerError);
    world.chainState.rpcDown = true;
    await expect(request.send()).rejects.toBeInstanceOf(ThresholdSignerError);
    expect(world.warn).toHaveBeenCalledWith(expect.stringContaining('RPC down'));
  });

  it('many concurrent requests failing on the same stale set: one refresh, all succeed', async () => {
    const world = makeWorld();
    await inputProofRequest(world).send();
    world.chainState.coprocessorSigner = SIGNER_B;
    const readsBefore = world.reads('getCoprocessorSigners');

    const results = await Promise.all(Array.from({ length: 5 }, () => inputProofRequest(world).send()));
    expect(results).toEqual(Array.from({ length: 5 }, () => SIGNER_B));
    expect(world.reads('getCoprocessorSigners') - readsBefore).toBe(1);
  });

  it('never loops: a retry failing on a stale value again throws CachedValueInvalidError', async () => {
    const world = makeWorld();
    const request = inputProofRequest(world);
    await request.send();
    request.run.mockClear();
    // The set changes again between the refresh and the retry check.
    let calls = 0;
    request.run.mockImplementation(async (usedVersions: UsedCacheVersions) => {
      calls++;
      const cached = await getCoprocessorContextSignersAndThreshold(world.client, { address: INPUT_VERIFIER });
      recordUsedCacheVersion(usedVersions, 'coprocessorSigners', coprocessorSignersVersion(cached));
      world.chainState.coprocessorSigner = calls % 2 === 1 ? SIGNER_B : SIGNER_A;
      throw new ThresholdSignerError({ type: 'coprocessor' });
    });

    await expect(request.send()).rejects.toMatchObject({
      name: 'CachedValueInvalidError',
      cacheName: 'coprocessorSigners',
    });
    expect(request.run).toHaveBeenCalledTimes(2);
  });

  it('autoRefreshOnInvalidity: false → CachedValueInvalidError → forceRefresh → retry succeeds', async () => {
    const world = makeWorld({ autoRefreshOnInvalidity: false });
    const request = inputProofRequest(world);
    const usedVersion = coprocessorSignersVersion({ threshold: 1, signers: [SIGNER_A] });
    await request.send();
    world.chainState.coprocessorSigner = SIGNER_B;

    const error = await request.send().catch((e: unknown) => e);
    expect(error).toBeInstanceOf(CachedValueInvalidError);
    expect(error).toMatchObject({ cacheName: 'coprocessorSigners', invalidVersion: usedVersion });
    expect((error as Error).cause).toBeInstanceOf(ThresholdSignerError);
    expect(request.run).toHaveBeenCalledTimes(2);
    // Nothing was cached: the next request still uses the old set.
    await expect(request.send()).rejects.toBeInstanceOf(CachedValueInvalidError);
    // forceRefresh reads the new set; then the request succeeds.
    const reads = world.reads('getCoprocessorSigners');
    await forceRefreshCachedValue(world.client, {
      cacheName: 'coprocessorSigners',
      invalidVersion: usedVersion,
      fhevmContext,
    });
    expect(world.reads('getCoprocessorSigners')).toBe(reads + 1);
    await expect(request.send()).resolves.toBe(SIGNER_B);
  });

  it('forceRefresh: one fetch for concurrent calls, none once replaced, CachedValueInvalidError if unchanged', async () => {
    const world = makeWorld();
    await inputProofRequest(world).send();
    const v1 = coprocessorSignersVersion({ threshold: 1, signers: [SIGNER_A] });
    const refresh = () =>
      forceRefreshCachedValue(world.client, { cacheName: 'coprocessorSigners', invalidVersion: v1, fhevmContext });

    await expect(refresh()).rejects.toMatchObject({ name: 'CachedValueInvalidError', invalidVersion: v1 });

    world.chainState.coprocessorSigner = SIGNER_B;
    const readsBefore = world.reads('getCoprocessorSigners');
    await Promise.all([refresh(), refresh(), refresh()]);
    await refresh();
    expect(world.reads('getCoprocessorSigners') - readsBefore).toBe(1);
  });

  it('createStaleError (a user decryption): refreshed, not run again, the createStaleError error with the request error as cause', async () => {
    const world = makeWorld();
    const request = inputProofRequest(world);
    await request.send();
    world.chainState.coprocessorSigner = SIGNER_B;

    const error = await retryOnStaleCache(world.client, fhevmContext, request.run, {
      createStaleError: (stale, cause) => new Error(`stale ${stale.cacheName}`, { cause }),
    }).catch((e: unknown) => e);
    expect((error as Error).message).toBe('stale coprocessorSigners');
    expect((error as Error).cause).toBeInstanceOf(ThresholdSignerError);
    expect(request.run).toHaveBeenCalledTimes(2);
    expect(world.warn).toHaveBeenCalledTimes(1);
    await expect(request.send()).resolves.toBe(SIGNER_B);
  });
});

////////////////////////////////////////////////////////////////////////////////

// The KMS context check end to end is tested through publicDecrypt below.
describe('KMS context', () => {
  it('a fetch AbortError (not RelayerAbortError) on an old context: not retried, the abort error', async () => {
    const world = makeWorld();
    const request = publicDecryptRequest(world, { failure: 'abort' });
    await request.send();
    world.chainState.kmsContextId = 6n;
    const fetchAbort = Object.assign(new Error('This operation was aborted'), { name: 'AbortError' });
    request.run.mockImplementationOnce(async (usedVersions: UsedCacheVersions) => {
      recordUsedCacheVersion(usedVersions, 'kmsContext', kmsContextVersion(5n, 1n));
      return Promise.reject(fetchAbort);
    });

    await expect(request.send()).rejects.toBe(fetchAbort);
    expect(request.run).toHaveBeenCalledTimes(2);
  });

  it('an aborted request on an old context: refreshed, not retried, the abort error', async () => {
    const world = makeWorld();
    const request = publicDecryptRequest(world, { failure: 'abort' });
    await request.send();
    world.chainState.kmsContextId = 6n;

    await expect(request.send()).rejects.toBeInstanceOf(RelayerAbortError);
    expect(request.run).toHaveBeenCalledTimes(2);
    // The next request uses the new context without reading it again.
    const signerReads = world.reads('getContextSignersAndThresholdFromExtraData');
    await expect(request.send()).resolves.toBe(6n);
    expect(world.reads('getContextSignersAndThresholdFromExtraData') - signerReads).toBe(0);
  });

  it('an aborted request on an old context (autoRefreshOnInvalidity: false): not refreshed, the abort error', async () => {
    const world = makeWorld({ autoRefreshOnInvalidity: false });
    const request = publicDecryptRequest(world, { failure: 'abort' });
    await request.send();
    world.chainState.kmsContextId = 6n;

    const signerReads = world.reads('getContextSignersAndThresholdFromExtraData');
    await expect(request.send()).rejects.toBeInstanceOf(RelayerAbortError);
    expect(request.run).toHaveBeenCalledTimes(2);
    // Only the ids were read: the new context's signers were not loaded.
    expect(world.reads('getContextSignersAndThresholdFromExtraData')).toBe(signerReads);
    await forceRefreshCachedValue(world.client, {
      cacheName: 'kmsContext',
      invalidVersion: kmsContextVersion(5n, 1n),
      fhevmContext,
    });
    await expect(request.send()).resolves.toBe(6n);
  });

  it('a request bound to its permit (autoRefreshOnInvalidity: false): CachedValueInvalidError, not createStaleError', async () => {
    const world = makeWorld({ autoRefreshOnInvalidity: false });
    const request = publicDecryptRequest(world, { failure: 'apiError' });
    await request.send();
    world.chainState.kmsContextId = 6n;
    const createStaleError = vi.fn(() => new Error('createStaleError'));

    const error = await retryOnStaleCache(world.client, fhevmContext, request.run, { createStaleError }).catch(
      (e: unknown) => e,
    );
    expect(error).toMatchObject({ name: 'CachedValueInvalidError', cacheName: 'kmsContext' });
    expect(createStaleError).not.toHaveBeenCalled();
  });
});

////////////////////////////////////////////////////////////////////////////////

describe('KMS context check through publicDecrypt', () => {
  // A computed euint8 handle on sepolia.
  function sepoliaHandle() {
    const bytes = new Uint8Array(32);
    bytes.fill(7, 0, 21);
    bytes[21] = 0xff;
    new DataView(bytes.buffer).setBigUint64(22, BigInt(sepolia.id));
    bytes[30] = 2;
    bytes[31] = FHEVM_HANDLE_CURRENT_CIPHERTEXT_VERSION;
    return bytes32ToHandle(asBytes32(bytes));
  }

  /** A relayer that never answers: every request times out. */
  function timingOutWorld() {
    const sent: string[] = [];
    const fetchPublicDecrypt = vi.fn(
      async (_context: unknown, p: { readonly payload: { readonly extraData: string } }) => {
        sent.push(p.payload.extraData);
        return Promise.reject(relayerApiError({ label: 'response_timed_out', message: 'timed out' }));
      },
    );
    const world = makeWorld(undefined, { fetchPublicDecrypt } as unknown as RelayerModule);
    const decrypt = (handles = [sepoliaHandle()]) =>
      publicDecrypt(world.client as never, { originToken: Symbol('test'), handles, fhevmContext });
    return { world, sent, decrypt };
  }

  it('a local error before sending never reads the context', async () => {
    const { world, decrypt } = timingOutWorld();
    await expect(decrypt([])).rejects.toThrow('handles must not be empty');
    // Only the read that built the request.
    expect(world.reads('getCurrentKmsContextAndEpoch')).toBe(1);
  });

  it('a timeout on the current context: one check, no retry, the timeout', async () => {
    const { world, sent, decrypt } = timingOutWorld();
    await expect(decrypt()).rejects.toBeInstanceOf(RelayerResponseApiError);
    expect(sent).toEqual([createKmsExtraDataV2({ kmsContextId: 5n as never, kmsEpochId: 1n as never }).bytesHex]);
    expect(world.reads('getCurrentKmsContextAndEpoch')).toBe(2);
  });

  it('a switch: the request runs once more with the new context', async () => {
    const { world, sent, decrypt } = timingOutWorld();
    await decrypt().catch(() => undefined);
    world.chainState.kmsContextId = 6n;
    sent.length = 0;

    await expect(decrypt()).rejects.toBeInstanceOf(RelayerResponseApiError);
    expect(sent).toEqual([
      createKmsExtraDataV2({ kmsContextId: 5n as never, kmsEpochId: 1n as never }).bytesHex,
      createKmsExtraDataV2({ kmsContextId: 6n as never, kmsEpochId: 1n as never }).bytesHex,
    ]);
    expect(world.warn).toHaveBeenCalledTimes(1);
  });
});
