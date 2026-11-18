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
 * The default and maximum TTL of every cached value. A stale value is detected when a
 * request fails, so the TTL only matters when no request fails.
 */
export const MAX_CACHE_TTL = CACHE_TTL_24H;

export const DEFAULT_REFRESH_RETRY_DELAY = 1_000;
export const MAX_REFRESH_RETRY_DELAY = 10_000;

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves `options.cache`, applying the defaults.
 *
 * @throws {FhevmConfigError} on a `cache` (or per-cache entry) that is not an
 * object, an unknown key, a TTL that is not an integer between 0 and 24 h, a
 * non-boolean `autoRefreshOnStale`, or a `refreshRetryDelay` that is not an integer
 * between 0 and 10 s.
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
    if (
      key !== 'autoRefreshOnStale' &&
      key !== 'refreshRetryDelay' &&
      !FHEVM_CACHE_NAMES.includes(key as FhevmCacheName)
    ) {
      throw new FhevmConfigError({
        message: `Unknown 'options.cache.${key}': expected ${FHEVM_CACHE_NAMES.join(', ')}, autoRefreshOnStale or refreshRetryDelay.`,
      });
    }
  }

  const ttl = {} as Record<FhevmCacheName, number>;
  for (const cacheName of FHEVM_CACHE_NAMES) {
    const valueOptions: unknown = cache[cacheName];
    if (valueOptions !== undefined) {
      if (typeof valueOptions !== 'object' || valueOptions === null || Array.isArray(valueOptions)) {
        throw new FhevmConfigError({ message: `'options.cache.${cacheName}' must be an object, such as { ttl }.` });
      }
      for (const key of Object.keys(valueOptions)) {
        if (key !== 'ttl') {
          throw new FhevmConfigError({ message: `Unknown 'options.cache.${cacheName}.${key}': expected ttl.` });
        }
      }
    }
    const valueTtl = cache[cacheName]?.ttl;
    if (valueTtl === undefined) {
      ttl[cacheName] = MAX_CACHE_TTL;
      continue;
    }
    if (!Number.isSafeInteger(valueTtl) || valueTtl < 0 || valueTtl > MAX_CACHE_TTL) {
      throw new FhevmConfigError({
        message: `'options.cache.${cacheName}.ttl' must be an integer number of milliseconds between 0 and ${MAX_CACHE_TTL} (24 h), got ${String(valueTtl)}.`,
      });
    }
    ttl[cacheName] = valueTtl;
  }

  const autoRefreshOnStale = cache.autoRefreshOnStale ?? true;
  if (typeof autoRefreshOnStale !== 'boolean') {
    throw new FhevmConfigError({
      message: `'options.cache.autoRefreshOnStale' must be a boolean.`,
    });
  }

  const refreshRetryDelay = cache.refreshRetryDelay ?? DEFAULT_REFRESH_RETRY_DELAY;
  if (
    !Number.isSafeInteger(refreshRetryDelay) ||
    refreshRetryDelay < 0 ||
    refreshRetryDelay > MAX_REFRESH_RETRY_DELAY
  ) {
    throw new FhevmConfigError({
      message: `'options.cache.refreshRetryDelay' must be an integer number of milliseconds between 0 and ${MAX_REFRESH_RETRY_DELAY}, got ${String(refreshRetryDelay)}.`,
    });
  }

  return Object.freeze({ ttl: Object.freeze(ttl), autoRefreshOnStale, refreshRetryDelay });
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
