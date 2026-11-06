import { describe, expect, it, vi } from 'vitest';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { createLatestValueCache, type CachedValue } from './latestValueCache.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/caching/latestValueCache.test.ts
////////////////////////////////////////////////////////////////////////////////

const SLOT = 'slot';

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
  const cache = createLatestValueCache<{ readonly misconfigured?: boolean }, object, { readonly data: string }>({
    slotKeyFn: () => SLOT,
    fetchFn,
    nowFn: () => now,
  });
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

describe('createLatestValueCache', () => {
  it('reuses the value within each caller TTL; 0 always reads', async () => {
    const { cache, fetchFn, advance } = makeCache();
    await cache.get({}, {}, 1_000);
    advance(500);
    await cache.get({}, {}, 1_000);
    expect(fetchFn).toHaveBeenCalledTimes(1);

    await cache.get({}, {}, 100);
    await cache.get({}, {}, 0);
    expect(fetchFn).toHaveBeenCalledTimes(3);
  });

  it('shares one read between concurrent callers, and stores nothing on error', async () => {
    const { cache, fetchFn } = makeCache();
    const [a, b] = await Promise.all([cache.get({}, {}, 1_000), cache.get({}, {}, 1_000)]);
    expect(b).toBe(a);
    expect(fetchFn).toHaveBeenCalledTimes(1);

    fetchFn.mockRejectedValueOnce(new Error('rpc down'));
    await expect(cache.refresh({}, {})).rejects.toThrow('rpc down');
    expect(cache.peek(SLOT)).toBe(a);
  });

  it('passes the cached value to the read, and stores what it returns with a new read time', async () => {
    const { cache, fetchFn, advance } = makeCache();
    const first = await cache.get({}, {}, 1_000);
    fetchFn.mockImplementationOnce(async (_context, _parameters, cached) => Promise.resolve(cached ?? first));
    advance(2_000);
    const again = await cache.get({}, {}, 1_000);
    expect(again.value).toBe(first.value);
    expect(again.timestamp).toBe(first.timestamp + 2_000);
  });

  describe('refresh', () => {
    it('with invalidVersion: reads once for concurrent calls, none once replaced', async () => {
      const { cache, fetchFn, change } = makeCache();
      await cache.get({}, {}, 1_000);
      change('v2');
      const results = await Promise.all([cache.refresh({}, {}, 'v1'), cache.refresh({}, {}, 'v1')]);
      expect(results.map((r) => r.version)).toEqual(['v2', 'v2']);
      await expect(cache.refresh({}, {}, 'v1')).resolves.toMatchObject({ version: 'v2' });
      expect(fetchFn).toHaveBeenCalledTimes(2);
    });

    it('may return invalidVersion when the value did not change', async () => {
      const { cache } = makeCache();
      await cache.get({}, {}, 1_000);
      await expect(cache.refresh({}, {}, 'v1')).resolves.toMatchObject({ version: 'v1' });
    });

    it('reads once more when a joined read, started before the change, returns invalidVersion', async () => {
      const { cache, fetchFn, change, hold } = makeCache();
      await cache.get({}, {}, 1_000);
      const release = hold();
      const before = cache.get({}, {}, 0);
      await vi.waitFor(() => expect(fetchFn).toHaveBeenCalledTimes(2));
      change('v2');
      const refreshed = cache.refresh({}, {}, 'v1');
      release();
      await expect(before).resolves.toMatchObject({ version: 'v1' });
      await expect(refreshed).resolves.toMatchObject({ version: 'v2' });
      expect(cache.peek(SLOT)).toMatchObject({ version: 'v2' });
    });
  });

  it('retries with its own context a joined read that failed on the starter configuration', async () => {
    const { cache, fetchFn, hold } = makeCache();
    const release = hold();
    const bad = cache.get({ misconfigured: true }, {}, 1_000);
    const good = cache.get({}, {}, 1_000);
    release();
    await expect(bad).rejects.toThrow(FhevmConfigError);
    await expect(good).resolves.toMatchObject({ version: 'v1' });
    expect(fetchFn).toHaveBeenCalledTimes(2);
  });

  it('a retry after a configuration error does not overwrite a value set meanwhile', async () => {
    const { cache, hold } = makeCache();
    const release = hold();
    const bad = cache.get({ misconfigured: true }, {}, 1_000);
    const good = cache.get({}, {}, 1_000);
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
    const reading = cache.get({}, {}, 1_000);
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
    const cache = createLatestValueCache<object, object, { readonly data: string }>({
      slotKeyFn: () => SLOT,
      // Unchanged version: the cached value is reused.
      fetchFn: async (_context, _parameters, cached) => {
        if (first) {
          first = false;
          return { version: 'v1', value: { data: 'v1' } };
        }
        await gate;
        return cached ?? { version: 'v1', value: { data: 'v1' } };
      },
    });
    const bad = await cache.get({}, {}, 1_000);
    const reading = cache.get({}, {}, 0);
    cache.evict(SLOT, bad.value);
    release();
    await expect(reading).resolves.toMatchObject({ value: bad.value });
    expect(cache.peek(SLOT)).toBeUndefined();
    // The slot is not stuck on that read: the next call reads again.
    const next = cache.get({}, {}, 1_000);
    await expect(next).resolves.toMatchObject({ version: 'v1' });
    expect(cache.peek(SLOT)).toBeDefined();
  });
});
