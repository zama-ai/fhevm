/**
 * A value the SDK reads from the chain or the relayer and caches:
 * - `'keyMaterial'`: the FHE public key and CRS;
 * - `'kmsContext'`: the KMS context, with its epoch, signers and threshold;
 * - `'coprocessorSigners'`: the coprocessor signers and threshold.
 */
export type FhevmCacheName = 'keyMaterial' | 'kmsContext' | 'coprocessorSigners';

export type FhevmSingleCacheOptions = {
  /**
   * How long a cached value is used, in milliseconds: at most (and by
   * default) 24 hours. `0` turns the cache off for the client.
   */
  readonly ttl?: number | undefined;
};

export type FhevmCacheOptions = {
  readonly keyMaterial?: FhevmSingleCacheOptions | undefined;
  readonly kmsContext?: FhevmSingleCacheOptions | undefined;
  readonly coprocessorSigners?: FhevmSingleCacheOptions | undefined;
  /**
   * When a request fails because a cached value went stale, the SDK refreshes
   * it and retries the request once (default `true`). A request signed for the
   * old value (e.g. a user decryption's permit) is not retried: the new value
   * is cached for the next permit. With `false`, the SDK throws
   * `CachedValueInvalidError` and leaves the cache as is: call
   * `forceRefresh(cacheName, { invalidVersion })` and retry.
   */
  readonly autoRefreshOnInvalidity?: boolean | undefined;
};

export type ResolvedFhevmCacheOptions = {
  readonly ttl: Readonly<Record<FhevmCacheName, number>>;
  readonly autoRefreshOnInvalidity: boolean;
};
