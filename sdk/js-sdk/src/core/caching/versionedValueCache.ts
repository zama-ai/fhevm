import { FhevmConfigError } from '../errors/FhevmConfigError.js';

/** A value as cached, with its version and the time it was read. */
export type CachedValue<T> = {
  readonly version: string;
  readonly value: T;
  readonly timestamp: number;
};

type Slot<T> = {
  // The current value: the one new requests use.
  current: CachedValue<T> | undefined;
  // The read in flight, shared by concurrent callers.
  pendingRead: Promise<CachedValue<T>> | undefined;
};

// When exceeded, every slot with no read in flight is evicted at once (as in
// `createCachedFetch`).
const MAX_SLOTS = 100;

////////////////////////////////////////////////////////////////////////////////
// VersionedValueCache class
////////////////////////////////////////////////////////////////////////////////

/**
 * A cache of one current value per slot, with its version and the time it was
 * read. Like `createCachedFetch`, for data that is **not user-specific** only:
 * the cache is shared by every caller in the thread.
 *
 * A subclass names the slot ({@link getSlotKey}) and reads the value
 * ({@link fetch}); it may override the clock ({@link now}), e.g. for tests.
 *
 * - The TTL is given on each read, so each client applies its own. Nothing
 *   expires in the background.
 * - Concurrent reads of a slot share one; a read that ends stores its value
 *   (an unchanged one only renews the read time). On error nothing is stored.
 * - A request keeps the value it read. When it fails because that value went
 *   stale, it passes the value's version to {@link refresh}.
 * - When versions only increase ({@link isOlderVersion}), an older value read
 *   (e.g. from a lagging RPC node) never replaces a newer cached one, unless
 *   a {@link refresh} reads it: the newer value failed, or the read is forced.
 */
export abstract class VersionedValueCache<TContext, TParams, TValue> {
  #slots = new Map<string, Slot<TValue>>();

  //////////////////////////////////////////////////////////////////////////////
  // Overridable
  //////////////////////////////////////////////////////////////////////////////

  /** Derives the slot (one current value) from context and parameters. */
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

  /** Clock for TTL checks, in milliseconds. Defaults to `Date.now()`. */
  protected now(): number {
    return Date.now();
  }

  /**
   * Whether `version` is older than `than`. Defaults to `false`: versions are
   * not ordered, and the last value read wins. Override it when versions only
   * increase.
   */
  protected isOlderVersion(_version: string, _than: string): boolean {
    return false;
  }

  //////////////////////////////////////////////////////////////////////////////
  // Public API
  //////////////////////////////////////////////////////////////////////////////

  /** The current value: the cached one while younger than `ttlMs` (`0`: never), else read. */
  public get(context: TContext, parameters: TParams, ttlMs: number): Promise<CachedValue<TValue>> {
    const slot = this.#getSlot(this.getSlotKey(context, parameters));
    const current = slot.current;
    if (current !== undefined && ttlMs > 0 && this.now() - current.timestamp <= ttlMs) {
      return Promise.resolve(current);
    }
    return this.#readOrRetry(slot, context, parameters, false);
  }

  /**
   * Reads the value again (or joins the read in flight). With
   * `invalidVersion`: returns the cached value unread when it has another
   * version (not an older one), and reads once more when a joined read
   * (started earlier) returns `invalidVersion`. The result may still have
   * `invalidVersion`: the caller decides what that means.
   *
   * The value read replaces the cached one even when older: the cached one
   * failed (`invalidVersion`), or the read is forced (no `invalidVersion`).
   */
  public async refresh(context: TContext, parameters: TParams, invalidVersion?: string): Promise<CachedValue<TValue>> {
    const slot = this.#getSlot(this.getSlotKey(context, parameters));
    const current = slot.current;
    if (
      invalidVersion !== undefined &&
      current !== undefined &&
      current.version !== invalidVersion &&
      !this.isOlderVersion(current.version, invalidVersion)
    ) {
      return current;
    }
    // A joined read keeps the rule of the caller that started it: a `get` keeps
    // a newer cached value, so the value read may still be `invalidVersion`.
    const joined = slot.pendingRead !== undefined;
    const cached = await this.#readOrRetry(slot, context, parameters, true);
    return joined && cached.version === invalidVersion ? this.#readOrRetry(slot, context, parameters, true) : cached;
  }

  /** The value cached in slot `slotKey`, whatever its age. Never reads. */
  public peek(slotKey: string): CachedValue<TValue> | undefined {
    return this.#slots.get(slotKey)?.current;
  }

  /** Stores a value read by the caller; a read in flight no longer stores its own. */
  public set(slotKey: string, version: string, value: TValue): void {
    const slot = this.#getSlot(slotKey);
    slot.pendingRead = undefined;
    slot.current = this.#stamp(version, value);
  }

  /** Empties the slot if it still holds `value`: the next call reads again. */
  public evict(slotKey: string, value: TValue): void {
    const slot = this.#slots.get(slotKey);
    if (slot?.current?.value === value) {
      slot.current = undefined;
    }
  }

  /** Empties the cache. For tests. */
  public clear(): void {
    this.#slots = new Map();
  }

  //////////////////////////////////////////////////////////////////////////////
  // Private
  //////////////////////////////////////////////////////////////////////////////

  #stamp(version: string, value: TValue): CachedValue<TValue> {
    return Object.freeze({ version, value, timestamp: this.now() });
  }

  #getSlot(key: string): Slot<TValue> {
    let slot = this.#slots.get(key);
    if (slot === undefined) {
      if (this.#slots.size >= MAX_SLOTS) {
        this.#slots = new Map([...this.#slots].filter(([, s]) => s.pendingRead !== undefined));
      }
      slot = { current: undefined, pendingRead: undefined };
      this.#slots.set(key, slot);
    }
    return slot;
  }

  /** Whether a value read with `version` must not replace the cached `current`. */
  #keepsNewer(current: CachedValue<TValue> | undefined, version: string, acceptOlder: boolean): boolean {
    return !acceptOlder && current !== undefined && this.isOlderVersion(version, current.version);
  }

  /**
   * Starts a read, or joins the read in flight. With `acceptOlder` false, an
   * older value read does not replace a newer cached one, which is returned
   * instead.
   */
  #read(
    slot: Slot<TValue>,
    context: TContext,
    parameters: TParams,
    acceptOlder: boolean,
  ): Promise<CachedValue<TValue>> {
    if (slot.pendingRead !== undefined) {
      return slot.pendingRead;
    }
    const cachedAtStart = slot.current;
    const pendingRead = this.fetch(context, parameters, cachedAtStart).then(
      ({ version, value }) => {
        const cached = this.#stamp(version, value);
        // Not stored if `set` replaced the value meanwhile, nor if it is the
        // value at the start, unchanged, which `evict` removed meanwhile.
        if (slot.pendingRead === pendingRead) {
          slot.pendingRead = undefined;
          const current = slot.current;
          if (current !== undefined && this.#keepsNewer(current, version, acceptOlder)) {
            return current;
          }
          if (value !== cachedAtStart?.value || slot.current?.value === value) {
            slot.current = cached;
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
  #readOrRetry(
    slot: Slot<TValue>,
    context: TContext,
    parameters: TParams,
    acceptOlder: boolean,
  ): Promise<CachedValue<TValue>> {
    const joined = slot.pendingRead !== undefined;
    const pendingRead = this.#read(slot, context, parameters, acceptOlder);
    return joined
      ? pendingRead.catch(async (e: unknown) => {
          if (!(e instanceof FhevmConfigError)) {
            throw e;
          }
          const currentAtRetry = slot.current;
          const { version, value } = await this.fetch(context, parameters, currentAtRetry);
          if (currentAtRetry !== undefined && this.#keepsNewer(currentAtRetry, version, acceptOlder)) {
            return currentAtRetry;
          }
          const cached = this.#stamp(version, value);
          // Not stored if `set` or `evict` changed the slot meanwhile.
          if (slot.pendingRead === undefined && slot.current === currentAtRetry) {
            slot.current = cached;
          }
          return cached;
        })
      : pendingRead;
  }
}
