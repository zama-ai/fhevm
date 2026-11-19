import type { FhevmCacheName } from '../types/cache.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { CacheUsage } from './cacheUsage-p.js';
import type { LatestValueCache } from './latestValueCache.js';
import { CachedValueStaleError } from '../errors/CachedValueStaleError.js';
import { RelayerAbortError } from '../errors/RelayerAbortError.js';
import { coprocessorSignersCache } from '../host-contracts/getCoprocessorContextSignersAndThreshold-p.js';
import { kmsContextCache } from '../host-contracts/readKmsSignersContext-p.js';
import { keyMaterialCache } from '../key/ensureFheEncryptionKeyDownload.js';
import { getCacheOptions } from './cacheOptions-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain>;

const CACHES: Readonly<Record<FhevmCacheName, LatestValueCache<Context, unknown, unknown>>> = {
  keyMaterial: keyMaterialCache,
  coprocessorSigners: coprocessorSignersCache,
  kmsContext: kmsContextCache,
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Fetches `cacheName` again into the cache (see `LatestValueCache.refresh` for
 * `staleVersion`). Concurrent calls share one fetch. It only reads again: an
 * unchanged value, or an older one (logged by the cache: the RPC lags), is no
 * error.
 *
 * @throws the fetch error.
 */
export async function refreshCachedValue(
  context: Context,
  parameters: {
    readonly cacheName: FhevmCacheName;
    readonly staleVersion?: string | undefined;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<void> {
  const { cacheName, staleVersion, fhevmContext } = parameters;
  await CACHES[cacheName].refreshCurrent(context, fhevmContext, staleVersion);
}

/** A cached value a failed request used, found stale (read again, its version changed). */
export type StaleValue = { readonly cacheName: FhevmCacheName; readonly version: string };

export type RetryOnStaleCacheOptions = {
  /**
   * For a request bound to the value it used (e.g. a user decryption, bound to
   * its permit's KMS context): on a stale value, the request is not run again;
   * returns the error to throw.
   */
  readonly createStaleError: (stale: StaleValue, error: unknown) => unknown;
};

/**
 * Runs a request. When it fails on a cached value that turns out stale (read
 * again, its version changed), the new value is cached and the request runs
 * once more, after `options.cache.refreshRetryDelay`, except:
 * - a request bound to the stale value (e.g. a user decryption, bound to its
 *   permit's KMS context) is not run again: `createStaleError` gives the error to throw;
 * - an aborted request throws its abort error;
 * - with `autoRefreshOnStale: false`, only the current version is read,
 *   the cache is left as is and {@link CachedValueStaleError} is thrown.
 *
 * A retry failing on a stale value again throws {@link CachedValueStaleError}.
 * Any other failure passes through as is: no value it used may explain it, or
 * each such value is unchanged or cannot be read again.
 */
export async function retryOnStaleCache<T>(
  context: Context,
  fhevmContext: FhevmClientFrozenContext,
  run: (usage: CacheUsage) => Promise<T>,
  runOptions?: RetryOnStaleCacheOptions,
): Promise<T> {
  // 1. The request.
  const firstUsage: CacheUsage = {};
  try {
    return await run(firstUsage);
  } catch (e) {
    // Throws unless a stale value was refreshed and the request may run again.
    await _refreshForRetry(context, fhevmContext, e, firstUsage, runOptions);
  }

  // 2. Once more, on the refreshed value, after a short wait (e.g. for nodes
  // still serving the old value). A stale value again is not retried.
  await _sleep(getCacheOptions(context).refreshRetryDelay);
  const secondUsage: CacheUsage = {};
  try {
    return await run(secondUsage);
  } catch (e) {
    const stale = await _refreshForRetry(context, fhevmContext, e, secondUsage, runOptions);
    throw _staleValueError(stale, e);
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * After a failed request: finds the stale value it used (refreshed, with
 * `autoRefreshOnStale`) and returns it when the request may run again.
 *
 * @throws the request error when no value it used is stale, or it was
 * aborted (the caller gave up: it is not run again).
 * @throws {CachedValueStaleError} with `autoRefreshOnStale: false`.
 * @throws the error from `runOptions.createStaleError`, for a request bound to
 * the value it used.
 */
async function _refreshForRetry(
  context: Context,
  fhevmContext: FhevmClientFrozenContext,
  error: unknown,
  usage: CacheUsage,
  runOptions: RetryOnStaleCacheOptions | undefined,
): Promise<StaleValue> {
  const autoRefresh = getCacheOptions(context).autoRefreshOnStale;
  const stale = await _findStaleValue(context, fhevmContext, error, usage, autoRefresh);
  if (stale === undefined) {
    throw error;
  }
  context.runtime.config.logger?.warn?.(
    autoRefresh
      ? `Cached ${stale.cacheName} (version ${stale.version}) was stale and has been refreshed.`
      : `Cached ${stale.cacheName} (version ${stale.version}) is stale; not refreshed (autoRefreshOnStale: false).`,
  );
  if (_isAbort(error)) {
    throw error;
  }
  if (!autoRefresh) {
    throw _staleValueError(stale, error);
  }
  if (runOptions !== undefined) {
    throw runOptions.createStaleError(stale, error);
  }
  return stale;
}

function _sleep(ms: number): Promise<void> {
  return ms > 0 ? new Promise((resolve) => setTimeout(resolve, ms)) : Promise.resolve();
}

/**
 * Reads again each value the failed request used whose cache may explain the
 * error. The first one whose version changed is the stale one: with
 * `refresh`, the new value is cached; without, only its version is read. A
 * failed read, or one older than the value used, proves nothing: it is
 * logged (failed) or skipped (older), and the request's error stands.
 */
async function _findStaleValue(
  context: Context,
  fhevmContext: FhevmClientFrozenContext,
  error: unknown,
  usage: CacheUsage,
  refresh: boolean,
): Promise<StaleValue | undefined> {
  for (const [cacheName, used] of Object.entries(usage) as Array<[FhevmCacheName, string]>) {
    const cache = CACHES[cacheName];
    if (!cache.mayBeStale(context, error)) {
      continue;
    }
    try {
      const current = refresh
        ? (await cache.refreshCurrent(context, fhevmContext, used)).version
        : await cache.readCurrentVersion(context, fhevmContext);
      // An older version (e.g. a lagging RPC) proves nothing; with `refresh`,
      // the cache has logged it.
      if (cache.isNewerVersion(used, current)) {
        if (!refresh) {
          cache.warnOlderRead(context, current, used);
        }
        continue;
      }
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

function _staleValueError(stale: StaleValue, cause: unknown): CachedValueStaleError {
  return new CachedValueStaleError({
    cacheName: stale.cacheName,
    staleVersion: stale.version,
    cause: cause instanceof Error ? cause : undefined,
  });
}
