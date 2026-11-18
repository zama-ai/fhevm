import type { FhevmErrorBaseParams } from './FhevmErrorBase.js';
import type { FhevmCacheName } from '../types/cache.js';
import type { Prettify } from '../types/utils.js';
import { FhevmErrorBase } from './FhevmErrorBase.js';

////////////////////////////////////////////////////////////////////////////////
// CachedValueStaleError
////////////////////////////////////////////////////////////////////////////////

export type CachedValueStaleErrorType = CachedValueStaleError & {
  name: 'CachedValueStaleError';
};

export type CachedValueStaleErrorParams = Prettify<
  Omit<FhevmErrorBaseParams, 'name' | 'message'> & {
    readonly cacheName: FhevmCacheName;
    readonly staleVersion: string;
    readonly message?: string | undefined;
  }
>;

/**
 * Thrown when a request failed because a cached value went stale (e.g. an
 * input proof rejected after a key rotation) and the SDK did not recover:
 * `autoRefreshOnStale` is `false`, or the retry failed on a stale value
 * again. Call `forceRefresh(cacheName, { staleVersion })`, then retry.
 *
 * `forceRefresh` also throws it when the value read again still has
 * `staleVersion`: refreshing cannot help.
 */
export class CachedValueStaleError extends FhevmErrorBase {
  readonly #cacheName: FhevmCacheName;
  readonly #staleVersion: string;

  constructor(params: CachedValueStaleErrorParams) {
    const { cacheName, staleVersion, ...baseParams } = params;
    super({
      ...baseParams,
      name: 'CachedValueStaleError',
      message: params.message ?? `Cached ${cacheName} (version ${staleVersion}) is stale.`,
    });
    this.#cacheName = cacheName;
    this.#staleVersion = staleVersion;
  }

  /** The cached value that went stale. */
  public get cacheName(): FhevmCacheName {
    return this.#cacheName;
  }

  /**
   * The version of the cached value that went stale, to pass to `forceRefresh`.
   * Not the SDK version (`version`).
   */
  public get staleVersion(): string {
    return this.#staleVersion;
  }
}
