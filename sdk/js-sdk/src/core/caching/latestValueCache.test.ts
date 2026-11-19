import type { Logger } from '../types/logger.js';
import { describe, expect, it, vi } from 'vitest';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { LatestValueCache, type CachedValue } from './latestValueCache.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/caching/latestValueCache.test.ts
////////////////////////////////////////////////////////////////////////////////

const SLOT = 'slot';

type TestContext = { readonly options: object; readonly ttlMs: number; readonly misconfigured?: boolean };

/** A client context with TTL `ttlMs`. */
const ctx = (ttlMs = 1_000, misconfigured = false): TestContext => ({ options: {}, ttlMs, misconfigured });
type TestValue = { readonly data: string };
type TestFetch = (
  context: TestContext,
  parameters: object,
  cached: CachedValue<TestValue> | undefined,
) => Promise<{ readonly version: string; readonly value: TestValue }>;

/** `vN` versions are ordered by N; any other version has no order. */
function compareTestVersions(a: string, b: string): number | undefined {
  const n = (v: string): number | undefined => (/^v\d+$/.test(v) ? Number(v.slice(1)) : undefined);
  const x = n(a);
  const y = n(b);
  if (x === undefined || y === undefined) {
    return a === b ? 0 : undefined;
  }
  return x - y;
}

/** A cache with one slot; reads and the clock are given by the test. */
class TestCache extends LatestValueCache<TestContext, object, TestValue> {
  public readonly cacheName = 'keyMaterial';
  readonly #fetchFn: TestFetch;
  readonly #nowFn: () => number;

  constructor(fetchFn: TestFetch, nowFn: () => number) {
    super();
    this.#fetchFn = fetchFn;
    this.#nowFn = nowFn;
  }

  protected override getSlotKey(): string {
    return SLOT;
  }

  protected override fetch(
    context: TestContext,
    parameters: object,
    cached: CachedValue<TestValue> | undefined,
  ): ReturnType<TestFetch> {
    return this.#fetchFn(context, parameters, cached);
  }

  protected override now(): number {
    return this.#nowFn();
  }

  public readonly warn = vi.fn();

  protected override getLogger(): Logger {
    return { debug: undefined, warn: this.warn, error: undefined };
  }

  protected override compareVersions(a: string, b: string): number | undefined {
    return compareTestVersions(a, b);
  }

  protected override ttl(context: TestContext): number {
    return context.ttlMs;
  }

  protected override async fetchVersion(context: TestContext, parameters: object): Promise<string> {
    return (await this.#fetchFn(context, parameters, undefined)).version;
  }

  public override currentParameters(): object {
    return {};
  }
}

/** A cache over an on-chain value `onChain`, whose reads can be held back with `hold()`. */
function makeCache() {
  let now = 1_000;
  let onChain = 'v1';
  let gate: Promise<void> | undefined;
  const fetchFn = vi.fn(
    async (
      context: { readonly misconfigured?: boolean },
      _parameters: object,
      _cached: CachedValue<{ readonly data: string }> | undefined,
    ): Promise<{ readonly version: string; readonly value: { readonly data: string } }> => {
      const version = onChain;
      await gate;
      if (context.misconfigured === true) {
        throw new FhevmConfigError({ message: 'wrong network' });
      }
      return { version, value: { data: version } };
    },
  );
  const cache = new TestCache(fetchFn, () => now);
  return {
    cache,
    fetchFn,
    change: (version: string) => {
      onChain = version;
    },
    advance: (ms: number) => {
      now += ms;
    },
    /** Holds back the reads started from now on, until the returned function is called. */
    hold: (): (() => void) => {
      let release!: () => void;
      gate = new Promise<void>((resolve) => (release = resolve));
      return () => {
        gate = undefined;
        release();
      };
    },
  };
}

describe('LatestValueCache', () => {
  it('reuses the value within each caller TTL; 0 always reads', async () => {
    const { cache, fetchFn, advance } = makeCache();
    await cache.get(ctx(1_000), {});
    advance(500);
    await cache.get(ctx(1_000), {});
    expect(fetchFn).toHaveBeenCalledTimes(1);

    await cache.get(ctx(100), {});
    await cache.get(ctx(0), {});
    expect(fetchFn).toHaveBeenCalledTimes(3);
  });

  it('shares one read between concurrent callers, and stores nothing on error', async () => {
    const { cache, fetchFn } = makeCache();
    const [a, b] = await Promise.all([cache.get(ctx(1_000), {}), cache.get(ctx(1_000), {})]);
    expect(b).toBe(a);
    expect(fetchFn).toHaveBeenCalledTimes(1);

    fetchFn.mockRejectedValueOnce(new Error('rpc down'));
    await expect(cache.refresh(ctx(), {})).rejects.toThrow('rpc down');
    expect(cache.peek(SLOT)).toBe(a);
  });

  it('passes the cached value to the read, and stores what it returns with a new read time', async () => {
    const { cache, fetchFn, advance } = makeCache();
    const first = await cache.get(ctx(1_000), {});
    fetchFn.mockImplementationOnce(async (_context, _parameters, cached) => Promise.resolve(cached ?? first));
    advance(2_000);
    const again = await cache.get(ctx(1_000), {});
    expect(again.value).toBe(first.value);
    expect(again.timestamp).toBe(first.timestamp + 2_000);
  });

  describe('refresh', () => {
    it('with staleVersion: reads once for concurrent calls, none once replaced', async () => {
      const { cache, fetchFn, change } = makeCache();
      await cache.get(ctx(1_000), {});
      change('v2');
      const results = await Promise.all([cache.refresh(ctx(), {}, 'v1'), cache.refresh(ctx(), {}, 'v1')]);
      expect(results.map((r) => r.version)).toEqual(['v2', 'v2']);
      await expect(cache.refresh(ctx(), {}, 'v1')).resolves.toMatchObject({ version: 'v2' });
      expect(fetchFn).toHaveBeenCalledTimes(2);
    });

    it('may return staleVersion when the value did not change', async () => {
      const { cache } = makeCache();
      await cache.get(ctx(1_000), {});
      await expect(cache.refresh(ctx(), {}, 'v1')).resolves.toMatchObject({ version: 'v1' });
    });

    it('reads once more when a joined read, started before the change, returns staleVersion', async () => {
      const { cache, fetchFn, change, hold } = makeCache();
      await cache.get(ctx(1_000), {});
      const release = hold();
      const before = cache.get(ctx(0), {});
      await vi.waitFor(() => expect(fetchFn).toHaveBeenCalledTimes(2));
      change('v2');
      const refreshed = cache.refresh(ctx(), {}, 'v1');
      release();
      await expect(before).resolves.toMatchObject({ version: 'v1' });
      await expect(refreshed).resolves.toMatchObject({ version: 'v2' });
      expect(cache.peek(SLOT)).toMatchObject({ version: 'v2' });
    });
  });

  it('retries with its own context a joined read that failed on the starter configuration', async () => {
    const { cache, fetchFn, hold } = makeCache();
    const release = hold();
    const bad = cache.get(ctx(1_000, true), {});
    const good = cache.get(ctx(1_000), {});
    release();
    await expect(bad).rejects.toThrow(FhevmConfigError);
    await expect(good).resolves.toMatchObject({ version: 'v1' });
    expect(fetchFn).toHaveBeenCalledTimes(2);
  });

  it('a retry after a configuration error does not overwrite a value set meanwhile', async () => {
    const { cache, hold } = makeCache();
    const release = hold();
    const bad = cache.get(ctx(1_000, true), {});
    const good = cache.get(ctx(1_000), {});
    const provided = { data: 'provided' };
    release();
    // Hold the joiner's own read, then set a value meanwhile.
    const releaseRetry = hold();
    await expect(bad).rejects.toThrow(FhevmConfigError);
    cache.set(SLOT, 'p', provided);
    releaseRetry();
    await good;
    expect(cache.peek(SLOT)?.value).toBe(provided);
  });

  it('set wins over a read in flight; evict empties the slot only if it holds the value', async () => {
    const { cache, hold } = makeCache();
    const release = hold();
    const reading = cache.get(ctx(1_000), {});
    const provided = { data: 'provided' };
    cache.set(SLOT, 'p', provided);
    release();
    await reading;
    expect(cache.peek(SLOT)?.value).toBe(provided);

    cache.evict(SLOT, { data: 'other' });
    expect(cache.peek(SLOT)?.value).toBe(provided);
    cache.evict(SLOT, provided);
    expect(cache.peek(SLOT)).toBeUndefined();
  });

  it('does not store again a value evicted while a read that reuses it was in flight', async () => {
    let release!: () => void;
    const gate = new Promise<void>((resolve) => (release = resolve));
    let first = true;
    // Unchanged version: the cached value is reused.
    const cache = new TestCache(
      async (_context, _parameters, cached) => {
        if (first) {
          first = false;
          return { version: 'v1', value: { data: 'v1' } };
        }
        await gate;
        return cached ?? { version: 'v1', value: { data: 'v1' } };
      },
      () => Date.now(),
    );
    const bad = await cache.get(ctx(1_000), {});
    const reading = cache.get(ctx(0), {});
    cache.evict(SLOT, bad.value);
    release();
    await expect(reading).resolves.toMatchObject({ value: bad.value });
    expect(cache.peek(SLOT)).toBeUndefined();
    // The slot is not stuck on that read: the next call reads again.
    const next = cache.get(ctx(1_000), {});
    await expect(next).resolves.toMatchObject({ version: 'v1' });
    expect(cache.peek(SLOT)).toBeDefined();
  });

  describe('a slot never moves backwards', () => {
    /** The next read returns `v1`, as through a client whose RPC lags. */
    const lagOnce = (fetchFn: ReturnType<typeof makeCache>['fetchFn']): void => {
      fetchFn.mockImplementationOnce(async () => ({ version: 'v1', value: { data: 'v1' } }));
    };

    it('does not store a read older than the stored value: the caller gets the stored one', async () => {
      const { cache, fetchFn, change } = makeCache();
      change('v2');
      const stored = await cache.get(ctx(1_000), {});
      lagOnce(fetchFn);
      await expect(cache.get(ctx(0), {})).resolves.toBe(stored);
      expect(cache.peek(SLOT)).toBe(stored);
      expect(cache.warn).toHaveBeenCalledWith(
        "keyMaterial: this client's RPC returned version v1, older than the cached v2, which is kept. " +
          'The RPC node lags behind the chain; if this repeats, check the RPC endpoint.',
      );
    });

    it('an older read the fetch answers with the cached value: kept as is, its read time not renewed', async () => {
      const { cache, fetchFn, change, advance } = makeCache();
      change('v2');
      const stored = await cache.get(ctx(1_000), {});
      fetchFn.mockImplementationOnce(async (_c, _p, cached) => ({ version: 'v1', value: cached!.value }));
      advance(500);
      await expect(cache.get(ctx(0), {})).resolves.toBe(stored);
      expect(cache.peek(SLOT)).toBe(stored);
      expect(cache.peek(SLOT)?.timestamp).toBe(stored.timestamp);
      expect(cache.warn).toHaveBeenCalledTimes(1);
    });

    it('a request that fails on v2 never gets v1 back from refresh(v2)', async () => {
      // B stores v2; A, whose RPC lags, reads v1.
      const { cache, fetchFn, change } = makeCache();
      change('v2');
      await cache.get(ctx(1_000), {});
      lagOnce(fetchFn);
      await cache.get(ctx(0), {});
      // B's request fails: refresh(v2) reads again, through B's RPC.
      await expect(cache.refresh(ctx(), {}, 'v2')).resolves.toMatchObject({ version: 'v2' });
      expect(fetchFn).toHaveBeenCalledTimes(3);
    });

    it('refresh(staleVersion) reads again when the stored value is older than staleVersion', async () => {
      const { cache, fetchFn, change } = makeCache();
      change('v2');
      const b = await cache.get(ctx(1_000), {});
      // The slot loses v2, then A stores v1.
      cache.evict(SLOT, b.value);
      lagOnce(fetchFn);
      await cache.get(ctx(0), {});
      expect(cache.peek(SLOT)).toMatchObject({ version: 'v1' });

      await expect(cache.refresh(ctx(), {}, 'v2')).resolves.toMatchObject({ version: 'v2' });
      expect(cache.peek(SLOT)).toMatchObject({ version: 'v2' });
    });

    it('refresh(staleVersion) returns a read older than staleVersion with a warning, no error', async () => {
      const { cache, fetchFn } = makeCache();
      lagOnce(fetchFn);
      await expect(cache.refresh(ctx(), {}, 'v2')).resolves.toMatchObject({ version: 'v1' });
      expect(cache.warn).toHaveBeenCalledWith(expect.stringContaining('lags behind the chain'));
    });

    it('set does not store a value older than the stored one', async () => {
      const { cache, change } = makeCache();
      change('v2');
      const stored = await cache.get(ctx(1_000), {});
      cache.set(SLOT, 'v1', { data: 'v1' });
      expect(cache.peek(SLOT)).toBe(stored);
    });

    it('versions with no order: stored as read, and refresh reads again', async () => {
      const { cache, fetchFn, change } = makeCache();
      change('a');
      await cache.get(ctx(1_000), {});
      change('b');
      await cache.get(ctx(0), {});
      expect(cache.peek(SLOT)).toMatchObject({ version: 'b' });
      await expect(cache.refresh(ctx(), {}, 'a')).resolves.toMatchObject({ version: 'b' });
      expect(fetchFn).toHaveBeenCalledTimes(3);
    });
  });
});
