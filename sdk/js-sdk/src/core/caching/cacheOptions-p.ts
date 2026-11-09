import type { FhevmCacheItem, FhevmCacheOptions, ResolvedFhevmCacheOptions } from '../types/cache.js';
import { CACHE_TTL_24H } from '../base/cachedFetch.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';

////////////////////////////////////////////////////////////////////////////////

export const FHEVM_CACHE_ITEMS: readonly FhevmCacheItem[] = Object.freeze([
  'keyMaterial',
  'kmsContext',
  'coprocessorSigners',
]);

/**
 * The default and maximum TTL of every item. A stale value is detected when a
 * request fails, so the TTL only matters when no request fails.
 */
export const MAX_CACHE_TTL = CACHE_TTL_24H;

////////////////////////////////////////////////////////////////////////////////

/**
 * Resolves `options.cache`, applying the defaults.
 *
 * @throws {FhevmConfigError} on a `cache` (or item entry) that is not an
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
    if (key !== 'autoRefreshOnInvalidity' && !FHEVM_CACHE_ITEMS.includes(key as FhevmCacheItem)) {
      throw new FhevmConfigError({
        message: `Unknown 'options.cache.${key}': expected ${FHEVM_CACHE_ITEMS.join(', ')} or autoRefreshOnInvalidity.`,
      });
    }
  }

  const ttl = {} as Record<FhevmCacheItem, number>;
  for (const item of FHEVM_CACHE_ITEMS) {
    const itemOptions: unknown = cache[item];
    if (itemOptions !== undefined) {
      if (typeof itemOptions !== 'object' || itemOptions === null || Array.isArray(itemOptions)) {
        throw new FhevmConfigError({ message: `'options.cache.${item}' must be an object, such as { ttl }.` });
      }
      for (const key of Object.keys(itemOptions)) {
        if (key !== 'ttl') {
          throw new FhevmConfigError({ message: `Unknown 'options.cache.${item}.${key}': expected ttl.` });
        }
      }
    }
    const itemTtl = cache[item]?.ttl;
    if (itemTtl === undefined) {
      ttl[item] = MAX_CACHE_TTL;
      continue;
    }
    if (!Number.isSafeInteger(itemTtl) || itemTtl < 0 || itemTtl > MAX_CACHE_TTL) {
      throw new FhevmConfigError({
        message: `'options.cache.${item}.ttl' must be an integer number of milliseconds between 0 and ${MAX_CACHE_TTL} (24 h), got ${String(itemTtl)}.`,
      });
    }
    ttl[item] = itemTtl;
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

/** The client's TTL for `item`, in milliseconds. */
export function getCacheTtl(context: { readonly options: object }, item: FhevmCacheItem): number {
  return getCacheOptions(context).ttl[item];
}
