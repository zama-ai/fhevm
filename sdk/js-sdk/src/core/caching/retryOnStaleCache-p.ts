import type { FhevmCacheItem } from '../types/cache.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { UsedCacheVersions } from './usedCacheVersions-p.js';
import { CachedValueInvalidError } from '../errors/CachedValueInvalidError.js';
import { RelayerResponseInputProofRejectedError } from '../errors/RelayerResponseInputProofRejectedError.js';
import { RelayerAbortError } from '../errors/RelayerAbortError.js';
import { DuplicateSignerError, ThresholdSignerError, UnknownSignerError } from '../errors/SignersError.js';
import {
  readCoprocessorSignersVersion,
  refreshCoprocessorContextSignersAndThreshold,
} from '../host-contracts/getCoprocessorContextSignersAndThreshold-p.js';
import { readCurrentKmsContextVersion, refreshCurrentKmsContext } from '../host-contracts/readKmsSignersContext-p.js';
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
  // A coprocessor signature check failure: the signer set may have changed.
  coprocessorSigners: {
    mayBeStale: (_context, error) =>
      error instanceof UnknownSignerError ||
      error instanceof ThresholdSignerError ||
      error instanceof DuplicateSignerError,
    refresh: (context, _fhevmContext, invalidVersion) =>
      refreshCoprocessorContextSignersAndThreshold(context, invalidVersion),
    readVersion: (context) => readCoprocessorSignersVersion(context),
  },
  // Any failure of a KMS request handed to the relayer (an error, a timeout,
  // an abort): the KMS reports a refused context no other way. The request
  // records the context only then, so earlier errors (bad input, an ACL
  // denial) never get here.
  kmsContext: {
    mayBeStale: () => true,
    refresh: (context, _fhevmContext, invalidVersion) => refreshCurrentKmsContext(context, invalidVersion),
    readVersion: (context) => readCurrentKmsContextVersion(context),
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
      ? `Cached ${stale.item} (version ${stale.version}) was stale and has been refreshed.`
      : `Cached ${stale.item} (version ${stale.version}) is stale; not refreshed (autoRefreshOnInvalidity: false).`,
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
