import type { FhevmCacheItem } from '../types/cache.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { UsedCacheVersions } from './usedCacheVersions-p.js';
import { CachedValueInvalidError } from '../errors/CachedValueInvalidError.js';
import { RelayerResponseInputProofRejectedError } from '../errors/RelayerResponseInputProofRejectedError.js';
import { RelayerAbortError } from '../errors/RelayerAbortError.js';
import { readFheEncryptionKeyVersion, refreshFheEncryptionKey } from '../key/ensureFheEncryptionKeyDownload.js';
import { requiresKeyVerification } from '../key/verifyFheEncryptionKeyBytes-p.js';
import { getCacheOptions } from './cacheOptions-p.js';

////////////////////////////////////////////////////////////////////////////////

type Context = FhevmBase<FhevmChain>;

/**
 * Per cached item: which request errors may come from a stale value, how to
 * read the value again (resolving to its version), and how to read only its
 * current version, caching nothing.
 */
const ITEMS: Readonly<
  Record<
    FhevmCacheItem,
    {
      readonly mayBeStale: (context: Context, error: unknown) => boolean;
      readonly refresh: (
        context: Context,
        fhevmContext: FhevmClientFrozenContext,
        invalidVersion: string | undefined,
      ) => Promise<string>;
      readonly readVersion: (context: Context, fhevmContext: FhevmClientFrozenContext) => Promise<string>;
    }
  >
> = {
  // An input proof rejected by the coprocessors: the key may have rotated.
  // Not for clients that do not check keys (the cleartext mock relayer): their
  // key never changes, and its labels cannot tell.
  keyMaterial: {
    mayBeStale: (context, error) =>
      error instanceof RelayerResponseInputProofRejectedError && requiresKeyVerification(context.runtime),
    refresh: (context, fhevmContext, invalidVersion) =>
      refreshFheEncryptionKey(context, { fhevmContext, invalidVersion }),
    readVersion: (context) => readFheEncryptionKeyVersion(context),
  },
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Fetches `item` again into the cache (see `VersionedValueCache.refresh` for
 * `invalidVersion`). Concurrent calls share one fetch.
 *
 * @throws {CachedValueInvalidError} when the value read again still has
 * `invalidVersion`: refreshing cannot fix the failure.
 * @throws the fetch error.
 */
export async function forceRefreshCachedValue(
  context: Context,
  parameters: {
    readonly item: FhevmCacheItem;
    readonly invalidVersion?: string | undefined;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<void> {
  const { item, invalidVersion, fhevmContext } = parameters;
  const version = await ITEMS[item].refresh(context, fhevmContext, invalidVersion);
  if (invalidVersion !== undefined && version === invalidVersion) {
    throw new CachedValueInvalidError({
      item,
      invalidVersion,
      message: `The ${item} read again (version ${invalidVersion}) is the cached one: refreshing it cannot help.`,
    });
  }
}

/** A cached value a failed request used, found stale (read again, its version changed). */
export type StaleValue = { readonly item: FhevmCacheItem; readonly version: string };

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
  options?: {
    /**
     * For a request bound to the value it used (e.g. a user decryption, bound
     * to its permit's KMS context): on a stale value, the request is not run
     * again; returns the error to throw.
     */
    readonly createStaleError: (stale: StaleValue, error: unknown) => unknown;
  },
): Promise<T> {
  const autoRefresh = getCacheOptions(context).autoRefreshOnInvalidity;
  for (let attempt = 0; ; attempt++) {
    const usedVersions: UsedCacheVersions = {};
    try {
      return await request(usedVersions);
    } catch (e) {
      const stale = await _findStaleValue(context, fhevmContext, e, usedVersions, autoRefresh);
      if (stale === undefined) {
        throw e;
      }
      context.runtime.config.logger?.warn?.(
        autoRefresh
          ? `Cached ${stale.item} (version ${stale.version}) was stale and has been refreshed.`
          : `Cached ${stale.item} (version ${stale.version}) is stale; not refreshed (autoRefreshOnInvalidity: false).`,
      );
      // The caller gave up on an aborted request: it is not run again.
      if (_isAbort(e)) {
        throw e;
      }
      if (!autoRefresh) {
        throw _staleValueError(stale, e);
      }
      if (options !== undefined) {
        throw options.createStaleError(stale, e);
      }
      if (attempt > 0) {
        throw _staleValueError(stale, e);
      }
    }
  }
}

////////////////////////////////////////////////////////////////////////////////

/**
 * Reads again each value the failed request used whose item may explain the
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
  for (const [item, used] of Object.entries(usedVersions) as Array<[FhevmCacheItem, string]>) {
    if (!ITEMS[item].mayBeStale(context, error)) {
      continue;
    }
    try {
      const current = refresh
        ? await ITEMS[item].refresh(context, fhevmContext, used)
        : await ITEMS[item].readVersion(context, fhevmContext);
      if (current !== used) {
        return { item, version: used };
      }
    } catch (refreshError) {
      context.runtime.config.logger?.warn?.(
        `Could not read the ${item} again (${String(refreshError)}): passing the request error on.`,
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
    item: stale.item,
    invalidVersion: stale.version,
    cause: cause instanceof Error ? cause : undefined,
  });
}
