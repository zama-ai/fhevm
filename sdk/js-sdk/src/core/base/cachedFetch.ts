type CacheEntry<T> = {
  promise: Promise<T>;
  readonly timestamp: number;
  settled: boolean;
};

type CachedFetchOptions<TContext, TParams, TResult> = {
  /** The function that performs the actual async work. */
  readonly executeFn: (
    context: TContext,
    parameters: TParams,
  ) => Promise<TResult>;
  /** Derives the cache key from context and parameters. */
  readonly cacheKeyFn: (context: TContext, parameters: TParams) => string;
  /** TTL in milliseconds. `0` disables caching. Omit for permanent cache. */
  readonly ttlMs?: number;
  /**
   * Maximum number of cache entries. When exceeded, **all** settled entries are
   * evicted at once. This is all-or-nothing by design — an LRU or per-entry
   * eviction would add complexity disproportionate to the SDK's use case
   * (typically a handful of contract addresses). Defaults to 100.
   */
  readonly maxSize?: number;
  /** Clock function for TTL checks. Defaults to `Date.now`. Useful for testing. */
  readonly nowFn?: () => number;
};

export const CACHE_TTL_24H = 24 * 60 * 60 * 1000; // 24 hours

/**
 * Creates a cached, deduplicating wrapper around an async fetch function.
 *
 * **Important:** This utility is designed for caching data that is **not
 * user-specific** (e.g. on-chain contract versions, signers, thresholds).
 * The cache is shared across all callers within the same thread — in a server
 * context, all requests see the same cached entries. Never use this for
 * user-scoped data. In a clustered or worker-threads setup, each worker holds
 * its own independent cache — there is no cross-worker sharing.
 *
 * - Concurrent calls with the same cache key share a single in-flight request.
 * - On success, the in-flight promise is replaced by a resolved one to release
 *   the `.then()` closure (which captures context and parameters) for GC.
 * - On error, the cache entry is removed so the next call retries. Retry
 *   backoff is intentionally out of scope — it should be handled by the
 *   `executeFn` or higher in the call stack.
 * - `forceRefresh` treats everything before the call as invalid: the cached
 *   entry (whether settled or in-flight) is discarded, a new fetch is issued,
 *   and the fresh result is stored back in the cache. This may cause a
 *   duplicate RPC if an in-flight request was already pending — that is
 *   intentional, since the caller explicitly does not trust prior results.
 * - If the cache exceeds `maxSize` entries, all settled entries are evicted
 *   as a safety net against unbounded memory growth in long-running processes.
 *
 * TTL behavior:
 * - `ttlMs` omitted → permanent cache (entries never expire).
 * - `ttlMs: 0` → caching disabled (every call goes to the fetch function).
 * - `ttlMs: N` → entries expire after N milliseconds.
 */
type CachedFetchResult<TContext, TParams, TResult> = {
  /** Fetch with caching and deduplication. */
  readonly execute: (
    context: TContext,
    parameters: TParams & { readonly forceRefresh?: boolean | undefined },
  ) => Promise<TResult>;
  /**
   * Remove entries from the cache. By default, only settled entries are removed
   * — in-flight requests are kept since they represent fresh data being fetched.
   *
   * Pass `{ includeInflight: true }` to also discard in-flight entries. Their
   * promises still resolve for existing callers, but results are not cached.
   *
   * @example
   * ```ts
   * cached.clear();                              // evict all settled entries
   * cached.clear({ includeInflight: true });     // evict everything
   * cached.clear({ key: 'uid:0xABC' });         // evict one settled entry
   * cached.clear({ key: 'uid:0xABC', includeInflight: true }); // evict one entry regardless
   * ```
   */
  readonly clear: (options?: {
    readonly key?: string;
    readonly includeInflight?: boolean;
  }) => void;
};

export function createCachedFetch<TContext, TParams, TResult>(
  options: CachedFetchOptions<TContext, TParams, TResult>,
): CachedFetchResult<TContext, TParams, TResult> {
  let cache = new Map<string, CacheEntry<TResult>>();

  const {
    executeFn,
    cacheKeyFn,
    ttlMs,
    maxSize = 100,
    nowFn = Date.now,
  } = options;

  function execute(
    context: TContext,
    parameters: TParams & { readonly forceRefresh?: boolean | undefined },
  ): Promise<TResult> {
    // TTL of 0 disables caching entirely.
    if (ttlMs === 0) {
      return executeFn(context, parameters);
    }

    const key = cacheKeyFn(context, parameters);

    const entry = cache.get(key);
    if (entry !== undefined) {
      // undefined TTL means permanent caching.
      const permanent = ttlMs === undefined;

      const expired = !permanent && nowFn() - entry.timestamp > ttlMs;

      if (expired || parameters.forceRefresh === true) {
        cache.delete(key);
      } else {
        return entry.promise;
      }
    }

    const inflightPromise = executeFn(context, parameters).then(
      (res) => {
        const cacheEntry = cache.get(key);
        if (cacheEntry?.promise === inflightPromise) {
          // Replace the in-flight promise with a resolved one to release
          // the .then() closure (which captures context and parameters) for GC.
          cache.set(key, {
            promise: Promise.resolve(res),
            timestamp: cacheEntry.timestamp,
            settled: true,
          });
        }
        return res;
      },
      (err: unknown) => {
        if (cache.get(key)?.promise === inflightPromise) {
          cache.delete(key);
        }
        throw err;
      },
    );

    // Safety net: evict settled entries if cache grows beyond maxSize.
    // Rebuilds the Map to reclaim internal hash table memory.
    if (cache.size >= maxSize) {
      cache = rebuildWithoutSettled(cache);
    }

    cache.set(key, {
      promise: inflightPromise,
      timestamp: nowFn(),
      settled: false,
    });

    return inflightPromise;
  }

  function clear(clearOptions?: {
    readonly key?: string;
    readonly includeInflight?: boolean;
  }): void {
    const includeInflight = clearOptions?.includeInflight === true;
    const key = clearOptions?.key;

    if (key !== undefined) {
      const entry = cache.get(key);
      if (entry !== undefined && (includeInflight || entry.settled)) {
        cache.delete(key);
      }
    } else if (includeInflight) {
      // Drop the entire Map to reclaim internal hash table memory.
      cache = new Map();
    } else {
      // Rebuild without settled entries to reclaim internal hash table memory
      // (deleting keys from a Map doesn't shrink its backing storage).
      cache = rebuildWithoutSettled(cache);
    }
  }

  return { execute, clear };
}

/**
 * Rebuilds a cache Map keeping only unsettled (in-flight) entries.
 * Creates a new Map to reclaim the internal hash table memory that
 * would otherwise stay inflated after deletions.
 */
function rebuildWithoutSettled<T>(
  cache: Map<string, CacheEntry<T>>,
): Map<string, CacheEntry<T>> {
  const next = new Map<string, CacheEntry<T>>();
  for (const [key, entry] of cache) {
    if (!entry.settled) {
      next.set(key, entry);
    }
  }
  return next;
}
