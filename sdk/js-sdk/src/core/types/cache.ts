/**
 * A value the SDK reads from the chain or the relayer and caches:
 * - `'keyMaterial'`: the FHE public key and CRS;
 * - `'coprocessorSigners'`: the coprocessor signers and threshold.
 */
export type FhevmCacheItem = 'keyMaterial' | 'coprocessorSigners';

export type FhevmCacheItemOptions = {
  /**
   * How long a cached value is used, in milliseconds: at most (and by
   * default) 24 hours. `0` turns the cache off for the client.
   */
  readonly ttl?: number | undefined;
};

export type FhevmCacheOptions = {
  readonly keyMaterial?: FhevmCacheItemOptions | undefined;
  readonly coprocessorSigners?: FhevmCacheItemOptions | undefined;
  /**
   * When a request fails because a cached value went stale, the SDK refreshes
   * it and retries the request once (default `true`). With `false`, the SDK throws
   * `CachedValueInvalidError` and leaves the cache as is: call
   * `forceRefresh(item, { invalidVersion })` and retry.
   */
  readonly autoRefreshOnInvalidity?: boolean | undefined;
};

export type ResolvedFhevmCacheOptions = {
  readonly ttl: Readonly<Record<FhevmCacheItem, number>>;
  readonly autoRefreshOnInvalidity: boolean;
};
