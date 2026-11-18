import type { FhevmCacheName } from '../types/cache.js';
import type { FhevmChain } from '../types/fhevmChain.js';
import type { FhevmClientFrozenContext } from '../types/fhevmClientFrozenContext-p.js';
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
   * read the version first and return the cached value when unchanged.
   */
  protected abstract fetch(
    context: TContext,
    parameters: TParams,
    cached: CachedValue<TValue> | undefined,
  ): Promise<{ readonly version: string; readonly value: TValue }>;

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
   * `staleVersion`: returns the cached value unread when it has another
   * version, and reads once more when a joined read (started earlier) returns
   * `staleVersion`. The result may still have `staleVersion`: the caller
   * decides what that means.
   */
  public async refresh(context: TContext, parameters: TParams, staleVersion?: string): Promise<CachedValue<TValue>> {
    const slot = this.#getSlot(this.getSlotKey(context, parameters));
    if (staleVersion !== undefined && slot.latest !== undefined && slot.latest.version !== staleVersion) {
      return slot.latest;
    }
    const joined = slot.pendingRead !== undefined;
    const cached = await this.#readOrRetry(slot, context, parameters);
    return joined && cached.version === staleVersion ? this.#readOrRetry(slot, context, parameters) : cached;
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

  /** Stores a value read by the caller; a read in flight no longer stores its own. */
  public set(slotKey: string, version: string, value: TValue): void {
    const slot = this.#getSlot(slotKey);
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

  #stamp(version: string, value: TValue): CachedValue<TValue> {
    return Object.freeze({ version, value, timestamp: this.now() });
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
    const pendingRead = this.fetch(context, parameters, cachedAtStart).then(
      ({ version, value }) => {
        const cached = this.#stamp(version, value);
        // Not stored if `set` replaced the value meanwhile, nor if it is the
        // value at the start, unchanged, which `evict` removed meanwhile.
        if (slot.pendingRead === pendingRead) {
          slot.pendingRead = undefined;
          if (value !== cachedAtStart?.value || slot.latest?.value === value) {
            slot.latest = cached;
          }
        }
        return cached;
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
          const { version, value } = await this.fetch(context, parameters, latestAtRetry);
          const cached = this.#stamp(version, value);
          // Not stored if `set` or `evict` changed the slot meanwhile.
          if (slot.pendingRead === undefined && slot.latest === latestAtRetry) {
            slot.latest = cached;
          }
          return cached;
        })
      : pendingRead;
  }
}
