import type { FhevmCacheName } from '../types/cache.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { UsedCacheVersions } from './usedCacheVersions-p.js';
import type { VersionedValueCache } from './versionedValueCache.js';
import { CachedValueInvalidError } from '../errors/CachedValueInvalidError.js';
import { RelayerAbortError } from '../errors/RelayerAbortError.js';
import { coprocessorSignersCache } from '../host-contracts/getCoprocessorContextSignersAndThreshold-p.js';
import { kmsContextCache } from '../host-contracts/readKmsSignersContext-p.js';
import { keyMaterialCache } from '../key/ensureFheEncryptionKeyDownload.js';
import { getCacheOptions } from './cacheOptions-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain>;

/**
 * The cache of each cache: it tells which request errors may come from a stale
 * value, reads the value again, or reads only its current version.
 */
const CACHES: Readonly<Record<FhevmCacheName, VersionedValueCache<Context, unknown, unknown>>> = {
  keyMaterial: keyMaterialCache,
  coprocessorSigners: coprocessorSignersCache,
  kmsContext: kmsContextCache,
};

/** Reads `cacheName` again into the cache (see `VersionedValueCache.refresh`); resolves to its version. */
async function _refreshCache(
  context: Context,
  fhevmContext: FhevmClientFrozenContext,
  cacheName: FhevmCacheName,
  invalidVersion: string | undefined,
): Promise<string> {
  const cache = CACHES[cacheName];
  const { version } = await cache.refresh(context, cache.currentParameters(context, fhevmContext), invalidVersion);
  return version;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Fetches `cacheName` again into the cache (see `VersionedValueCache.refresh` for
 * `invalidVersion`). Concurrent calls share one fetch.
 *
 * @throws {CachedValueInvalidError} when the value read again still has
 * `invalidVersion`: refreshing cannot fix the failure.
 * @throws the fetch error.
 */
export async function forceRefreshCachedValue(
  context: Context,
  parameters: {
    readonly cacheName: FhevmCacheName;
    readonly invalidVersion?: string | undefined;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<void> {
  const { cacheName, invalidVersion, fhevmContext } = parameters;
  const version = await _refreshCache(context, fhevmContext, cacheName, invalidVersion);
  if (invalidVersion !== undefined && version === invalidVersion) {
    throw new CachedValueInvalidError({
      cacheName,
      invalidVersion,
      message: `The ${cacheName} read again (version ${invalidVersion}) is the cached one: refreshing it cannot help.`,
    });
  }
}

/** A cached value a failed request used, found stale (read again, its version changed). */
export type StaleValue = { readonly cacheName: FhevmCacheName; readonly version: string };

export type RetryOnStaleCacheOptions = {
  /**
   * For a request bound to the value it used (e.g. a user decryption, bound
   * to its permit's KMS context): on a stale value, the request is not run
   * again; returns the error to throw.
   */
  readonly createStaleError: (stale: StaleValue, error: unknown) => unknown;
};

/**
 * Runs a request. When it fails on a cached value that turns out stale (read
 * again, its version changed), the new value is cached and the request runs
 * once more, except:
 * - a request bound to the stale value (e.g. a user decryption, bound to its
 *   permit's KMS context) is not run again: `createStaleError` gives the
 *   error to throw;
 * - an aborted request throws its abort error;
 * - with `autoRefreshOnInvalidity: false`, only the current version is read,
 *   the cache is left as is and {@link CachedValueInvalidError} is thrown.
 *
 * A retry failing on a stale value again throws {@link CachedValueInvalidError}.
 * Any other failure passes through as is: no value it used may explain it, or
 * each such value is unchanged or cannot be read again.
 */
export async function retryOnStaleCache<T>(
  context: Context,
  fhevmContext: FhevmClientFrozenContext,
  request: (usedVersions: UsedCacheVersions) => Promise<T>,
  options?: RetryOnStaleCacheOptions,
): Promise<T> {
  // 1. The request.
  const firstVersions: UsedCacheVersions = {};
  try {
    return await request(firstVersions);
  } catch (e) {
    // Throws unless a stale value was refreshed and the request may run again.
    await _refreshForRetry(context, fhevmContext, e, firstVersions, options);
  }

  // 2. Once more, on the refreshed value: a stale value again is not retried.
  const secondVersions: UsedCacheVersions = {};
  try {
    return await request(secondVersions);
  } catch (e) {
    const stale = await _refreshForRetry(context, fhevmContext, e, secondVersions, options);
    throw _staleValueError(stale, e);
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * After a failed request: finds the stale value it used (refreshed, with
 * `autoRefreshOnInvalidity`) and returns it when the request may run again.
 *
 * @throws the request error when no value it used is stale, or it was
 * aborted (the caller gave up: it is not run again).
 * @throws {CachedValueInvalidError} with `autoRefreshOnInvalidity: false`.
 * @throws the error from `options.createStaleError`, for a request bound to
 * the value it used.
 */
async function _refreshForRetry(
  context: Context,
  fhevmContext: FhevmClientFrozenContext,
  error: unknown,
  usedVersions: UsedCacheVersions,
  options: RetryOnStaleCacheOptions | undefined,
): Promise<StaleValue> {
  const autoRefresh = getCacheOptions(context).autoRefreshOnInvalidity;
  const stale = await _findStaleValue(context, fhevmContext, error, usedVersions, autoRefresh);
  if (stale === undefined) {
    throw error;
  }
  context.runtime.config.logger?.warn?.(
    autoRefresh
      ? `Cached ${stale.cacheName} (version ${stale.version}) was stale and has been refreshed.`
      : `Cached ${stale.cacheName} (version ${stale.version}) is stale; not refreshed (autoRefreshOnInvalidity: false).`,
  );
  if (_isAbort(error)) {
    throw error;
  }
  if (!autoRefresh) {
    throw _staleValueError(stale, error);
  }
  if (options !== undefined) {
    throw options.createStaleError(stale, error);
  }
  return stale;
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads again each value the failed request used whose cache may explain the
 * error. The first one whose version changed is the stale one: with
 * `refresh`, the new value is cached; without, only its version is read. A
 * failed read proves nothing: it is logged, and the request's error stands.
 */
async function _findStaleValue(
  context: Context,
  fhevmContext: FhevmClientFrozenContext,
  error: unknown,
  usedVersions: UsedCacheVersions,
  refresh: boolean,
): Promise<StaleValue | undefined> {
  for (const [cacheName, used] of Object.entries(usedVersions) as Array<[FhevmCacheName, string]>) {
    const cache = CACHES[cacheName];
    if (!cache.mayBeStale(context, error)) {
      continue;
    }
    try {
      const current = refresh
        ? await _refreshCache(context, fhevmContext, cacheName, used)
        : await cache.readCurrentVersion(context, cache.currentParameters(context, fhevmContext));
      if (current !== used) {
        return { cacheName, version: used };
      }
    } catch (refreshError) {
      context.runtime.config.logger?.warn?.(
        `Could not read the ${cacheName} again (${String(refreshError)}): passing the request error on.`,
      );
    }
  }
  return undefined;
}

/** An abort: the SDK's own, or the fetch API's `AbortError`. */
function _isAbort(error: unknown): boolean {
  return (
    error instanceof RelayerAbortError ||
    (typeof error === 'object' && error !== null && (error as { name?: unknown }).name === 'AbortError')
  );
}

function _staleValueError(stale: StaleValue, cause: unknown): CachedValueInvalidError {
  return new CachedValueInvalidError({
    cacheName: stale.cacheName,
    invalidVersion: stale.version,
    cause: cause instanceof Error ? cause : undefined,
  });
}
