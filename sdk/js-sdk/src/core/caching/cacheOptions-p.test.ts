import type { EthereumModule } from '../modules/ethereum/types.js';
import type { RelayerModule } from '../modules/relayer/types.js';
import type { FhevmCacheOptions } from '../types/cache.js';
import { describe, expect, it } from 'vitest';
import { PRIVATE_ETHERS_TOKEN } from '../../ethers/internal/ethers-p.js';
import { sepolia } from '../chains/definitions/sepolia.js';
import { createCoreFhevm } from '../runtime/CoreFhevm-p.js';
import { createFhevmRuntime } from '../runtime/CoreFhevmRuntime-p.js';
import { getCacheOptions, getCacheTtl, MAX_CACHE_TTL, resolveCacheOptions } from './cacheOptions-p.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/caching/cacheOptions-p.test.ts
////////////////////////////////////////////////////////////////////////////////

describe('resolveCacheOptions', () => {
  it('defaults every TTL to 24 h and turns automatic refresh on', () => {
    expect(resolveCacheOptions(undefined)).toEqual({
      ttl: { keyMaterial: MAX_CACHE_TTL, coprocessorSigners: MAX_CACHE_TTL },
      autoRefreshOnInvalidity: true,
    });
    expect(MAX_CACHE_TTL).toBe(24 * 60 * 60 * 1000);
  });

  it('takes a shorter TTL per item, and 0', () => {
    const resolved = resolveCacheOptions({
      keyMaterial: { ttl: 60_000 },
      coprocessorSigners: { ttl: 0 },
      autoRefreshOnInvalidity: false,
    });
    expect(resolved.ttl).toEqual({ keyMaterial: 60_000, coprocessorSigners: 0 });
    expect(resolved.autoRefreshOnInvalidity).toBe(false);
  });

  it.each([MAX_CACHE_TTL + 1, -1, 1.5, Number.NaN, '1000'])('rejects a TTL of %s', (ttl) => {
    expect(() => resolveCacheOptions({ keyMaterial: { ttl: ttl as number } })).toThrow(
      "'options.cache.keyMaterial.ttl' must be an integer",
    );
  });

  it('rejects a non-boolean autoRefreshOnInvalidity', () => {
    expect(() => resolveCacheOptions({ autoRefreshOnInvalidity: 'yes' as unknown as boolean })).toThrow(
      "'options.cache.autoRefreshOnInvalidity' must be a boolean",
    );
  });

  it.each([null, [], 'yes'])('rejects a cache that is not an object (%j) with FhevmConfigError', (cache) => {
    expect(() => resolveCacheOptions(cache as unknown as FhevmCacheOptions)).toThrow(
      expect.objectContaining({ name: 'FhevmConfigError' }),
    );
  });
});

describe('client cache options', () => {
  const runtime = () =>
    createFhevmRuntime(PRIVATE_ETHERS_TOKEN, {
      ethereum: {} as EthereumModule,
      relayer: {} as RelayerModule,
      config: {},
    });

  it('are resolved per client', () => {
    const a = createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
      chain: sepolia,
      client: {},
      runtime: runtime(),
      options: { cache: { keyMaterial: { ttl: 1_000 } } },
    });
    const b = createCoreFhevm(PRIVATE_ETHERS_TOKEN, { chain: sepolia, client: {}, runtime: runtime() });
    expect(getCacheTtl(a, 'keyMaterial')).toBe(1_000);
    expect(getCacheTtl(b, 'keyMaterial')).toBe(MAX_CACHE_TTL);
  });

  it('fail client creation on an invalid TTL', () => {
    expect(() =>
      createCoreFhevm(PRIVATE_ETHERS_TOKEN, {
        chain: sepolia,
        client: {},
        runtime: runtime(),
        options: { cache: { coprocessorSigners: { ttl: MAX_CACHE_TTL * 2 } } },
      }),
    ).toThrow("'options.cache.coprocessorSigners.ttl'");
  });

  it('default for an internal context that is not a client', () => {
    expect(getCacheOptions({ options: { batchRpcCalls: false } }).autoRefreshOnInvalidity).toBe(true);
  });

  it('rejects an item entry that is not an object, and an unknown key, with FhevmConfigError', () => {
    expect(() => resolveCacheOptions({ coprocessorSigners: 3_600_000 } as unknown as FhevmCacheOptions)).toThrow(
      "'options.cache.coprocessorSigners' must be an object",
    );
    expect(() => resolveCacheOptions({ coprocessorSigner: { ttl: 0 } } as unknown as FhevmCacheOptions)).toThrow(
      "Unknown 'options.cache.coprocessorSigner'",
    );
    expect(() => resolveCacheOptions({ coprocessorSigners: { TTL: 0 } } as unknown as FhevmCacheOptions)).toThrow(
      "Unknown 'options.cache.coprocessorSigners.TTL'",
    );
    expect(() => resolveCacheOptions({ coprocessorSigners: [] } as unknown as FhevmCacheOptions)).toThrow(
      "'options.cache.coprocessorSigners' must be an object",
    );
  });
});
