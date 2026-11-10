import type { FhevmCacheName, FhevmCacheOptions, ResolvedFhevmCacheOptions } from '../types/cache.js';
import { CACHE_TTL_24H } from '../base/cachedFetch.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';

////////////////////////////////////////////////////////////////////////////////

export const FHEVM_CACHE_NAMES: readonly FhevmCacheName[] = Object.freeze([
  'keyMaterial',
  'kmsContext',
  'coprocessorSigners',
]);

/**
 * The default and maximum TTL of every cache. A stale value is detected when a
 * request fails, so the TTL only matters when no request fails.
 */
export const MAX_CACHE_TTL = CACHE_TTL_24H;

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves `options.cache`, applying the defaults.
 *
 * @throws {FhevmConfigError} on a `cache` (or cache entry) that is not an
 * object, an unknown key, a TTL that is not an integer between 0 and 24 h, or a
 * non-boolean `autoRefreshOnInvalidity`.
 */
export function resolveCacheOptions(cache: FhevmCacheOptions | undefined): ResolvedFhevmCacheOptions {
  if (cache === undefined) {
    return DEFAULT_CACHE_OPTIONS;
  }
  // `null`, an array or a primitive from a JavaScript caller.
  const untyped: unknown = cache;
  if (typeof untyped !== 'object' || untyped === null || Array.isArray(untyped)) {
    throw new FhevmConfigError({ message: `'options.cache' must be an object.` });
  }

  for (const key of Object.keys(cache)) {
    if (key !== 'autoRefreshOnInvalidity' && !FHEVM_CACHE_NAMES.includes(key as FhevmCacheName)) {
      throw new FhevmConfigError({
        message: `Unknown 'options.cache.${key}': expected ${FHEVM_CACHE_NAMES.join(', ')} or autoRefreshOnInvalidity.`,
      });
    }
  }

  const ttl = {} as Record<FhevmCacheName, number>;
  for (const cacheName of FHEVM_CACHE_NAMES) {
    const singleCacheOptions: unknown = cache[cacheName];
    if (singleCacheOptions !== undefined) {
      if (typeof singleCacheOptions !== 'object' || singleCacheOptions === null || Array.isArray(singleCacheOptions)) {
        throw new FhevmConfigError({ message: `'options.cache.${cacheName}' must be an object, such as { ttl }.` });
      }
      for (const key of Object.keys(singleCacheOptions)) {
        if (key !== 'ttl') {
          throw new FhevmConfigError({ message: `Unknown 'options.cache.${cacheName}.${key}': expected ttl.` });
        }
      }
    }
    const cacheTtl = cache[cacheName]?.ttl;
    if (cacheTtl === undefined) {
      ttl[cacheName] = MAX_CACHE_TTL;
      continue;
    }
    if (!Number.isSafeInteger(cacheTtl) || cacheTtl < 0 || cacheTtl > MAX_CACHE_TTL) {
      throw new FhevmConfigError({
        message: `'options.cache.${cacheName}.ttl' must be an integer number of milliseconds between 0 and ${MAX_CACHE_TTL} (24 h), got ${String(cacheTtl)}.`,
      });
    }
    ttl[cacheName] = cacheTtl;
  }

  const autoRefreshOnInvalidity = cache.autoRefreshOnInvalidity ?? true;
  if (typeof autoRefreshOnInvalidity !== 'boolean') {
    throw new FhevmConfigError({
      message: `'options.cache.autoRefreshOnInvalidity' must be a boolean.`,
    });
  }

  return Object.freeze({ ttl: Object.freeze(ttl), autoRefreshOnInvalidity });
}

const DEFAULT_CACHE_OPTIONS: ResolvedFhevmCacheOptions = resolveCacheOptions({});

/** The client's cache options; the defaults for a context with no `options.cache`. */
export function getCacheOptions(context: { readonly options: object }): ResolvedFhevmCacheOptions {
  return (context.options as { readonly cache?: ResolvedFhevmCacheOptions | undefined }).cache ?? DEFAULT_CACHE_OPTIONS;
}

/** The client's TTL for `cacheName`, in milliseconds. */
export function getCacheTtl(context: { readonly options: object }, cacheName: FhevmCacheName): number {
  return getCacheOptions(context).ttl[cacheName];
}
