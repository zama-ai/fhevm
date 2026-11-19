import type { FhevmCacheName } from '../types/cache.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
import type { Logger } from '../types/logger.js';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { getCacheTtl } from './cacheOptions-p.js';

/** A value as cached, with its version and the time it was read. */
export type CachedValue<T> = {
  readonly version: string;
  readonly value: T;
  readonly timestamp: number;
};

type Slot<T> = {
  // The current value: the one new requests use.
  latest: CachedValue<T> | undefined;
  // The read in flight, shared by concurrent callers.
  pendingRead: Promise<CachedValue<T>> | undefined;
};

// When exceeded, every slot with no read in flight is evicted at once (as in
// `createCachedFetch`).
const MAX_SLOTS = 100;

////////////////////////////////////////////////////////////////////////////////
// LatestValueCache
////////////////////////////////////////////////////////////////////////////////

/**
 * A cache of the latest value per slot, with its version and the time it was
 * read. Like `createCachedFetch`, for data that is **not user-specific** only:
 * the cache is shared by every caller in the thread.
 *
 * Only the latest value is kept. A request holds the value it read (and
 * records its version), so it keeps using it after the slot moves on; the old
 * value is freed with the last request holding it. No older version needs to
 * be stored or evicted.
 *
 * Nothing is persisted, and the SDK never resumes a relayer request: after a
 * crash or a restart, a retried request is a new one, built from the value
 * read then. A value a request is built from goes into its payload (e.g. the
 * KMS context in `extraData`), and the relayer deduplicates on a hash of the
 * payload, so it never answers the retry with the result of a request built
 * from an older value.
 *
 * - Each client applies its own TTL (`options.cache.<cacheName>.ttl`).
 *   Nothing expires in the background.
 * - Concurrent reads of a slot share one; a read that ends stores its value
 *   (an unchanged one only renews the read time). On error nothing is stored.
 * - A slot never moves backwards. A read older than the stored value (e.g.
 *   from a client whose RPC lags) is not stored: the caller gets the stored,
 *   newer value. "Older" is the order of {@link compareVersions}; versions
 *   with no order are stored as read.
 * - A request that fails because its value went stale passes the value's
 *   version to {@link refresh}.
 */
export abstract class LatestValueCache<TContext extends { readonly options: object }, TParams, TValue> {
  #slots = new Map<string, Slot<TValue>>();

  public abstract readonly cacheName: FhevmCacheName;

  /** The slot (one latest value) for this context and parameters. */
  protected abstract getSlotKey(context: TContext, parameters: TParams): string;

  /**
   * Reads the current value and its version. Given the cached value, it may
   * read the version first and return the cached value when unchanged. When
   * the version read is older (e.g. an RPC that lags), it returns
   * `cached.value` with the version read: the cache keeps `cached`, its read
   * time unchanged, and logs a warning.
   */
  protected abstract fetch(
    context: TContext,
    parameters: TParams,
    cached: CachedValue<TValue> | undefined,
  ): Promise<{ readonly version: string; readonly value: TValue }>;

  /**
   * Orders two versions: negative when `a` is older than `b`, `0` when equal,
   * positive when newer; `undefined` when they have no order. Override it
   * when versions follow on-chain ids that only increase. By default, only
   * equal versions are ordered.
   */
  protected compareVersions(a: string, b: string): number | undefined {
    return a === b ? 0 : undefined;
  }

  /** Whether `version` is known to be newer than `than` (see {@link compareVersions}). */
  public isNewerVersion(version: string, than: string): boolean {
    return (this.compareVersions(version, than) ?? 0) > 0;
  }

  /** Where to log, e.g. an older read ignored. None by default. */
  protected getLogger(_context: TContext): Logger | undefined {
    return undefined;
  }

  /** Clock for TTL checks, in milliseconds. */
  protected now(): number {
    return Date.now();
  }

  /** The client's TTL for this cache, in milliseconds (`0`: no cache). */
  protected ttl(context: TContext): number {
    return getCacheTtl(context, this.cacheName);
  }

  /**
   * Reads only the current version, caching nothing: cheaper than
   * {@link fetch} (e.g. the key ids, not the key).
   */
  protected abstract fetchVersion(context: TContext, parameters: TParams): Promise<string>;

  /**
   * For `retryOnStaleCache`: whether a failed request's error may come from a
   * stale value of this cache. By default never: the request is not run again.
   */
  public mayBeStale(_context: TContext, _error: unknown): boolean {
    return false;
  }

  /** The parameters of the value the client uses now. */
  public abstract currentParameters(
    context: TContext & { readonly chain: FhevmChain },
    fhevmContext: FhevmClientFrozenContext,
  ): TParams;

  /** The current value: the cached one while younger than the client's TTL, else read. */
  public get(context: TContext, parameters: TParams): Promise<CachedValue<TValue>> {
    const ttlMs = this.ttl(context);
    const slot = this.#getSlot(this.getSlotKey(context, parameters));
    const latest = slot.latest;
    if (latest !== undefined && ttlMs > 0 && this.now() - latest.timestamp <= ttlMs) {
      return Promise.resolve(latest);
    }
    return this.#readOrRetry(slot, context, parameters);
  }

  /**
   * Reads the value again (or joins the read in flight). With
   * `staleVersion`:
   * - returns the cached value unread when it is newer than `staleVersion`;
   * - reads once more when a joined read (started earlier) returns
   *   `staleVersion` or an older version;
   * - reads at most twice, and never throws for an older read: a read older
   *   than `staleVersion` (e.g. an RPC that lags) is logged and returned, and
   *   the caller treats it as unchanged.
   *
   * The result may still have `staleVersion`: the caller decides what that
   * means.
   */
  public async refresh(context: TContext, parameters: TParams, staleVersion?: string): Promise<CachedValue<TValue>> {
    const slot = this.#getSlot(this.getSlotKey(context, parameters));
    if (staleVersion === undefined) {
      return this.#readOrRetry(slot, context, parameters);
    }
    if (slot.latest !== undefined && this.isNewerVersion(slot.latest.version, staleVersion)) {
      return slot.latest;
    }
    const joined = slot.pendingRead !== undefined;
    let cached = await this.#readOrRetry(slot, context, parameters);
    if (joined && (this.compareVersions(cached.version, staleVersion) ?? 1) <= 0) {
      cached = await this.#readOrRetry(slot, context, parameters);
    }
    if (this.isNewerVersion(staleVersion, cached.version)) {
      this.warnOlderRead(context, cached.version, staleVersion);
    }
    return cached;
  }

  /** {@link refresh} of the value the client uses now. */
  public refreshCurrent(
    context: TContext & { readonly chain: FhevmChain },
    fhevmContext: FhevmClientFrozenContext,
    staleVersion?: string,
  ): Promise<CachedValue<TValue>> {
    return this.refresh(context, this.currentParameters(context, fhevmContext), staleVersion);
  }

  /** The version of the value the client uses now, read again; caches nothing. */
  public readCurrentVersion(
    context: TContext & { readonly chain: FhevmChain },
    fhevmContext: FhevmClientFrozenContext,
  ): Promise<string> {
    return this.fetchVersion(context, this.currentParameters(context, fhevmContext));
  }

  /** The value cached in slot `slotKey`, whatever its age. Never reads. */
  public peek(slotKey: string): CachedValue<TValue> | undefined {
    return this.#slots.get(slotKey)?.latest;
  }

  /**
   * Stores a value read by the caller; a read in flight no longer stores its
   * own. A value older than the stored one is not stored.
   */
  public set(slotKey: string, version: string, value: TValue): void {
    const slot = this.#getSlot(slotKey);
    if (slot.latest !== undefined && this.isNewerVersion(slot.latest.version, version)) {
      return;
    }
    slot.pendingRead = undefined;
    slot.latest = this.#stamp(version, value);
  }

  /** Empties the slot if it still holds `value`: the next call reads again. */
  public evict(slotKey: string, value: TValue): void {
    const slot = this.#slots.get(slotKey);
    if (slot?.latest?.value === value) {
      slot.latest = undefined;
    }
  }

  /** Empties the cache. For tests. */
  public clear(): void {
    this.#slots = new Map();
  }

  /** Logs a read older than the value kept: for the operator, the client's RPC lags. */
  public warnOlderRead(context: TContext, readVersion: string, newerVersion: string): void {
    this.getLogger(context)?.warn?.(
      `${this.cacheName}: this client's RPC returned version ${readVersion}, older than the cached ` +
        `${newerVersion}, which is kept. The RPC node lags behind the chain; if this repeats, check the RPC endpoint.`,
    );
  }

  #stamp(version: string, value: TValue): CachedValue<TValue> {
    return Object.freeze({ version, value, timestamp: this.now() });
  }

  // A value read. When `fetch` kept the value at the start after an older
  // read, that value stays as is: not read again, so its read time is not
  // renewed. Otherwise see `#keepNewest`.
  #settle(
    context: TContext,
    slot: Slot<TValue>,
    cachedAtStart: CachedValue<TValue> | undefined,
    read: { readonly version: string; readonly value: TValue },
    store: boolean,
  ): CachedValue<TValue> {
    if (read.value === cachedAtStart?.value && this.isNewerVersion(cachedAtStart.version, read.version)) {
      const kept = slot.latest ?? cachedAtStart;
      this.warnOlderRead(context, read.version, kept.version);
      return kept;
    }
    return this.#keepNewest(context, slot, this.#stamp(read.version, read.value), store);
  }

  // A value read: stored when `store`, unless older than the stored one,
  // which is then returned instead (the slot never moves backwards).
  #keepNewest(context: TContext, slot: Slot<TValue>, cached: CachedValue<TValue>, store: boolean): CachedValue<TValue> {
    if (slot.latest !== undefined && this.isNewerVersion(slot.latest.version, cached.version)) {
      this.warnOlderRead(context, cached.version, slot.latest.version);
      return slot.latest;
    }
    if (store) {
      slot.latest = cached;
    }
    return cached;
  }

  #getSlot(key: string): Slot<TValue> {
    let slot = this.#slots.get(key);
    if (slot === undefined) {
      if (this.#slots.size >= MAX_SLOTS) {
        this.#slots = new Map([...this.#slots].filter(([, s]) => s.pendingRead !== undefined));
      }
      slot = { latest: undefined, pendingRead: undefined };
      this.#slots.set(key, slot);
    }
    return slot;
  }

  #read(slot: Slot<TValue>, context: TContext, parameters: TParams): Promise<CachedValue<TValue>> {
    if (slot.pendingRead !== undefined) {
      return slot.pendingRead;
    }
    const cachedAtStart = slot.latest;
    const pendingRead: Promise<CachedValue<TValue>> = this.fetch(context, parameters, cachedAtStart).then(
      (read) => {
        // Not stored if `set` replaced the value meanwhile, nor if it is the
        // value at the start, unchanged, which `evict` removed meanwhile.
        const own = slot.pendingRead === pendingRead;
        if (own) {
          slot.pendingRead = undefined;
        }
        const store = own && (read.value !== cachedAtStart?.value || slot.latest?.value === read.value);
        return this.#settle(context, slot, cachedAtStart, read, store);
      },
      (e: unknown) => {
        if (slot.pendingRead === pendingRead) {
          slot.pendingRead = undefined;
        }
        throw e;
      },
    );
    slot.pendingRead = pendingRead;
    return pendingRead;
  }

  // A joined read runs with the context of the caller that started it (e.g.
  // its RPC client): if it fails on that caller's configuration, the caller
  // reads once more with its own, never joining another read, so one
  // misconfigured client does not fail the others.
  #readOrRetry(slot: Slot<TValue>, context: TContext, parameters: TParams): Promise<CachedValue<TValue>> {
    const joined = slot.pendingRead !== undefined;
    const pendingRead = this.#read(slot, context, parameters);
    return joined
      ? pendingRead.catch(async (e: unknown) => {
          if (!(e instanceof FhevmConfigError)) {
            throw e;
          }
          const latestAtRetry = slot.latest;
          const read = await this.fetch(context, parameters, latestAtRetry);
          // Not stored if `set` or `evict` changed the slot meanwhile.
          const store = slot.pendingRead === undefined && slot.latest === latestAtRetry;
          return this.#settle(context, slot, latestAtRetry, read, store);
        })
      : pendingRead;
  }
}
