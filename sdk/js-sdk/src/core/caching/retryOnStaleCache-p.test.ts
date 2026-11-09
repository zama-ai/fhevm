import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmCacheOptions } from '../types/cache.js';
import type { ChecksummedAddress } from '../types/primitives.js';
import type { UsedCacheVersions } from './usedCacheVersions-p.js';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { createFhevmClientFrozenContext } from '../frozenContext/fhevmClientFrozenContext-p.js';
import { invalidateVersionCache } from '../host-contracts/HostContractVersion-p.js';
import { ThresholdSignerError } from '../errors/SignersError.js';
import { CachedValueInvalidError } from '../errors/CachedValueInvalidError.js';
import {
  coprocessorSignersVersion,
  getCoprocessorContextSignersAndThreshold,
} from '../host-contracts/getCoprocessorContextSignersAndThreshold-p.js';
import { recordUsedCacheVersion } from './usedCacheVersions-p.js';
import { forceRefreshCachedValue, retryOnStaleCache } from './retryOnStaleCache-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/caching/retryOnStaleCache-p.test.ts
//
// Refresh on failure: a request failing on a stale cached value refreshes it
// and runs once more (or throws CachedValueInvalidError when automatic
// refresh is off). The "request" here is a stand-in that reads the cached
// value as encrypt does, and fails the way the relayer or the
// signature check fails when that value is stale.
////////////////////////////////////////////////////////////////////////////////

const SIGNER_A = '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266' as ChecksummedAddress;
const SIGNER_B = '0x70997970C51812dc3A010C7d01b50e0d17dc79C8' as ChecksummedAddress;

const fhevmContext = createFhevmClientFrozenContext({});

const INPUT_VERIFIER = sepolia.fhevm.contracts.inputVerifier.address as ChecksummedAddress;

/** The on-chain state: coprocessor signer; and a logger. */
function makeWorld(cache?: FhevmCacheOptions) {
  const chainState = { coprocessorSigner: SIGNER_A, rpcDown: false };
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
      default:
        throw new Error(`No mocked handler for ${p.functionName}`);
    }
  });
  const warn = vi.fn();
  const runtime = createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
    ethereum: { readContract } as unknown as EthereumModule,
    relayer: {} as RelayerModule,
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

beforeEach(() => {
  invalidateVersionCache({ includeInflight: true });
});

////////////////////////////////////////////////////////////////////////////////

// The rules are the same for every item: tested once, on a coprocessor
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

    await expect(request.send()).rejects.toMatchObject({ name: 'CachedValueInvalidError', item: 'coprocessorSigners' });
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
    expect(error).toMatchObject({ item: 'coprocessorSigners', invalidVersion: usedVersion });
    expect((error as Error).cause).toBeInstanceOf(ThresholdSignerError);
    expect(request.run).toHaveBeenCalledTimes(2);
    // Nothing was cached: the next request still uses the old set.
    await expect(request.send()).rejects.toBeInstanceOf(CachedValueInvalidError);
    // forceRefresh reads the new set; then the request succeeds.
    const reads = world.reads('getCoprocessorSigners');
    await forceRefreshCachedValue(world.client, {
      item: 'coprocessorSigners',
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
      forceRefreshCachedValue(world.client, { item: 'coprocessorSigners', invalidVersion: v1, fhevmContext });

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
      createStaleError: (stale, cause) => new Error(`stale ${stale.item}`, { cause }),
    }).catch((e: unknown) => e);
    expect((error as Error).message).toBe('stale coprocessorSigners');
    expect((error as Error).cause).toBeInstanceOf(ThresholdSignerError);
    expect(request.run).toHaveBeenCalledTimes(2);
    expect(world.warn).toHaveBeenCalledTimes(1);
    await expect(request.send()).resolves.toBe(SIGNER_B);
  });
});
