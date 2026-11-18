import type { FhevmCacheName } from '../types/cache.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmBase } from '../types/coreFhevmClient.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { CacheUsage } from './cacheUsage-p.js';
import { CachedValueStaleError } from '../errors/CachedValueStaleError.js';
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
 * Per cache: which request errors may come from a stale value, how to
 * read the value again (resolving to its version), and how to read only its
 * current version, caching nothing.
 */
const CACHES: Readonly<
  Record<
    FhevmCacheName,
    {
      readonly mayBeStale: (context: Context, error: unknown) => boolean;
      readonly refresh: (
        context: Context,
        fhevmContext: FhevmClientFrozenContext,
        staleVersion: string | undefined,
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
    refresh: (context, fhevmContext, staleVersion) => refreshFheEncryptionKey(context, { fhevmContext, staleVersion }),
    readVersion: (context) => readFheEncryptionKeyVersion(context),
  },
  // A coprocessor signature check failure: the signer set may have changed.
  coprocessorSigners: {
    mayBeStale: (_context, error) =>
      error instanceof UnknownSignerError ||
      error instanceof ThresholdSignerError ||
      error instanceof DuplicateSignerError,
    refresh: (context, _fhevmContext, staleVersion) =>
      refreshCoprocessorContextSignersAndThreshold(context, staleVersion),
    readVersion: (context) => readCoprocessorSignersVersion(context),
  },
  // Any failure of a KMS request handed to the relayer (an error, a timeout,
  // an abort): the KMS reports a refused context no other way. The request
  // records the context only then, so earlier errors (bad input, an ACL
  // denial) never get here.
  kmsContext: {
    mayBeStale: () => true,
    refresh: (context, _fhevmContext, staleVersion) => refreshCurrentKmsContext(context, staleVersion),
    readVersion: (context) => readCurrentKmsContextVersion(context),
  },
};

////////////////////////////////////////////////////////////////////////////////

/**
 * Fetches `cacheName` again into the cache (see `LatestValueCache.refresh` for
 * `staleVersion`). Concurrent calls share one fetch.
 *
 * @throws {CachedValueStaleError} when the value read again still has
 * `staleVersion`: refreshing cannot fix the failure.
 * @throws the fetch error.
 */
export async function forceRefreshCachedValue(
  context: Context,
  parameters: {
    readonly cacheName: FhevmCacheName;
    readonly staleVersion?: string | undefined;
    readonly fhevmContext: FhevmClientFrozenContext;
  },
): Promise<void> {
  const { cacheName, staleVersion, fhevmContext } = parameters;
  const version = await CACHES[cacheName].refresh(context, fhevmContext, staleVersion);
  if (staleVersion !== undefined && version === staleVersion) {
    throw new CachedValueStaleError({
      cacheName,
      staleVersion,
      message: `The ${cacheName} read again (version ${staleVersion}) is the cached one: refreshing it cannot help.`,
    });
  }
}

/** A cached value a failed request used, found stale (read again, its version changed). */
export type StaleValue = { readonly cacheName: FhevmCacheName; readonly version: string };

/**
 * Runs a request. When it fails on a cached value that turns out stale (read
 * again, its version changed), the new value is cached and the request runs
 * once more, except:
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
  runOptions?: { readonly createStaleError: (stale: StaleValue, error: unknown) => unknown },
): Promise<T> {
  const autoRefresh = getCacheOptions(context).autoRefreshOnStale;
  for (let attempt = 0; ; attempt++) {
    const usage: CacheUsage = {};
    try {
      return await run(usage);
    } catch (e) {
      const stale = await _findStaleValue(context, fhevmContext, e, usage, autoRefresh);
      if (stale === undefined) {
        throw e;
      }
      context.runtime.config.logger?.warn?.(
        autoRefresh
          ? `Cached ${stale.cacheName} (version ${stale.version}) was stale and has been refreshed.`
          : `Cached ${stale.cacheName} (version ${stale.version}) is stale; not refreshed (autoRefreshOnStale: false).`,
      );
      // The caller gave up on an aborted request: it is not run again.
      if (_isAbort(e)) {
        throw e;
      }
      if (!autoRefresh) {
        throw _staleValueError(stale, e);
      }
      if (runOptions !== undefined) {
        throw runOptions.createStaleError(stale, e);
      }
      if (attempt > 0) {
        throw _staleValueError(stale, e);
      }
    }
  }
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
  usage: CacheUsage,
  refresh: boolean,
): Promise<StaleValue | undefined> {
  for (const [cacheName, used] of Object.entries(usage) as Array<[FhevmCacheName, string]>) {
    if (!CACHES[cacheName].mayBeStale(context, error)) {
      continue;
    }
    try {
      const current = refresh
        ? await CACHES[cacheName].refresh(context, fhevmContext, used)
        : await CACHES[cacheName].readVersion(context, fhevmContext);
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
