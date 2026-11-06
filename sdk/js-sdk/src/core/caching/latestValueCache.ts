import { FhevmConfigError } from '../errors/FhevmConfigError.js';

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
  reading: Promise<CachedValue<T>> | undefined;
};

type LatestValueCacheOptions<TContext, TParams, TValue> = {
  /** Derives the slot (one current value) from context and parameters. */
  readonly slotKeyFn: (context: TContext, parameters: TParams) => string;
  /**
   * Reads the current value and its version. Given the cached value, it may
   * read the version first and return the cached value when unchanged.
   */
  readonly fetchFn: (
    context: TContext,
    parameters: TParams,
    cached: CachedValue<TValue> | undefined,
  ) => Promise<{ readonly version: string; readonly value: TValue }>;
  /** Clock function for TTL checks. Defaults to `Date.now`. Useful for testing. */
  readonly nowFn?: () => number;
};

export type LatestValueCache<TContext, TParams, TValue> = {
  /** The current value: the cached one while younger than `ttlMs` (`0`: never), else read. */
  readonly get: (context: TContext, parameters: TParams, ttlMs: number) => Promise<CachedValue<TValue>>;
  /**
   * Reads the value again (or joins the read in flight). With
   * `invalidVersion`: returns the cached value unread when it has another
   * version, and reads once more when a joined read (started earlier) returns
   * `invalidVersion`. The result may still have `invalidVersion`: the caller
   * decides what that means.
   */
  readonly refresh: (context: TContext, parameters: TParams, invalidVersion?: string) => Promise<CachedValue<TValue>>;
  /** The value cached in slot `slotKey`, whatever its age. Never reads. */
  readonly peek: (slotKey: string) => CachedValue<TValue> | undefined;
  /** Stores a value read by the caller; a read in flight no longer stores its own. */
  readonly set: (slotKey: string, version: string, value: TValue) => void;
  /** Empties the slot if it still holds `value`: the next call reads again. */
  readonly evict: (slotKey: string, value: TValue) => void;
  /** Empties the cache. For tests. */
  readonly clear: () => void;
};

// When exceeded, every slot with no read in flight is evicted at once (as in
// `createCachedFetch`).
const MAX_SLOTS = 100;

/**
 * Creates a cache of one current value per slot, with its version and the
 * time it was read. Like `createCachedFetch`, for data that is **not
 * user-specific** only: the cache is shared by every caller in the thread.
 *
 * - The TTL is given on each read, so each client applies its own. Nothing
 *   expires in the background.
 * - Concurrent reads of a slot share one; a read that ends stores its value
 *   (an unchanged one only renews the read time). On error nothing is stored.
 * - A request keeps the value it read. When it fails because that value went
 *   stale, it passes the value's version to `refresh`.
 */
export function createLatestValueCache<TContext, TParams, TValue>(
  options: LatestValueCacheOptions<TContext, TParams, TValue>,
): LatestValueCache<TContext, TParams, TValue> {
  const { slotKeyFn, fetchFn, nowFn = (): number => Date.now() } = options;
  let slots = new Map<string, Slot<TValue>>();

  function stamp(version: string, value: TValue): CachedValue<TValue> {
    return Object.freeze({ version, value, timestamp: nowFn() });
  }

  function getSlot(key: string): Slot<TValue> {
    let slot = slots.get(key);
    if (slot === undefined) {
      if (slots.size >= MAX_SLOTS) {
        slots = new Map([...slots].filter(([, s]) => s.reading !== undefined));
      }
      slot = { latest: undefined, reading: undefined };
      slots.set(key, slot);
    }
    return slot;
  }

  function read(slot: Slot<TValue>, context: TContext, parameters: TParams): Promise<CachedValue<TValue>> {
    if (slot.reading !== undefined) {
      return slot.reading;
    }
    const cachedAtStart = slot.latest;
    const reading = fetchFn(context, parameters, cachedAtStart).then(
      ({ version, value }) => {
        const cached = stamp(version, value);
        // Not stored if `set` replaced the value meanwhile, nor if it is the
        // value at the start, unchanged, which `evict` removed meanwhile.
        if (slot.reading === reading) {
          slot.reading = undefined;
          if (value !== cachedAtStart?.value || slot.latest?.value === value) {
            slot.latest = cached;
          }
        }
        return cached;
      },
      (e: unknown) => {
        if (slot.reading === reading) {
          slot.reading = undefined;
        }
        throw e;
      },
    );
    slot.reading = reading;
    return reading;
  }

  // A joined read runs with the context of the caller that started it (e.g.
  // its RPC client): if it fails on that caller's configuration, the caller
  // reads once more with its own, never joining another read, so one
  // misconfigured client does not fail the others.
  function readOrRetry(slot: Slot<TValue>, context: TContext, parameters: TParams): Promise<CachedValue<TValue>> {
    const joined = slot.reading !== undefined;
    const reading = read(slot, context, parameters);
    return joined
      ? reading.catch(async (e: unknown) => {
          if (!(e instanceof FhevmConfigError)) {
            throw e;
          }
          const latestAtRetry = slot.latest;
          const { version, value } = await fetchFn(context, parameters, latestAtRetry);
          const cached = stamp(version, value);
          // Not stored if `set` or `evict` changed the slot meanwhile.
          if (slot.reading === undefined && slot.latest === latestAtRetry) {
            slot.latest = cached;
          }
          return cached;
        })
      : reading;
  }

  return {
    get: (context, parameters, ttlMs) => {
      const slot = getSlot(slotKeyFn(context, parameters));
      const latest = slot.latest;
      if (latest !== undefined && ttlMs > 0 && nowFn() - latest.timestamp <= ttlMs) {
        return Promise.resolve(latest);
      }
      return readOrRetry(slot, context, parameters);
    },

    refresh: async (context, parameters, invalidVersion) => {
      const slot = getSlot(slotKeyFn(context, parameters));
      if (invalidVersion !== undefined && slot.latest !== undefined && slot.latest.version !== invalidVersion) {
        return slot.latest;
      }
      const joined = slot.reading !== undefined;
      const cached = await readOrRetry(slot, context, parameters);
      return joined && cached.version === invalidVersion ? readOrRetry(slot, context, parameters) : cached;
    },

    peek: (slotKey) => {
      return slots.get(slotKey)?.latest;
    },

    set: (slotKey, version, value) => {
      const slot = getSlot(slotKey);
      slot.reading = undefined;
      slot.latest = stamp(version, value);
    },

    evict: (slotKey, value) => {
      const slot = slots.get(slotKey);
      if (slot?.latest?.value === value) {
        slot.latest = undefined;
      }
    },

    clear: () => {
      slots = new Map();
    },
  };
}
