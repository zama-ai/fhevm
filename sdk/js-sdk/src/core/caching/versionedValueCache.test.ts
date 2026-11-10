import type { Logger } from '../types/logger.js';
import { describe, expect, it, vi } from 'vitest';
import { FhevmConfigError } from '../errors/FhevmConfigError.js';
import { VersionedValueCache, type CachedValue } from './versionedValueCache.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/caching/versionedValueCache.test.ts
////////////////////////////////////////////////////////////////////////////////

const SLOT = 'slot';

/** A client: its configuration, its logger, and (by default) the one slot. */
type TestContext = {
  readonly misconfigured?: boolean;
  readonly logger?: Logger | undefined;
  readonly slot?: string | undefined;
};
type TestValue = { readonly data: string };
type TestFetch = VersionedValueCache<TestContext, object, TestValue>['fetch'];

/** A cache with one slot; reads and the clock are given by the test. */
class TestCache extends VersionedValueCache<TestContext, object, TestValue> {
  readonly #fetchFn: TestFetch;
  readonly #nowFn: () => number;

  constructor(fetchFn: TestFetch, nowFn: () => number) {
    super();
    this.#fetchFn = fetchFn;
    this.#nowFn = nowFn;
  }

  protected override readonly itemName = 'testValue';

  protected override getSlotKey(context: TestContext): string {
    return context.slot ?? SLOT;
  }

  protected override getLogger(context: TestContext): Logger | undefined {
    return context.logger;
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
}

/** A {@link TestCache} whose versions only increase: `v1` < `v2` < `v3`. */
class OrderedTestCache extends TestCache {
  protected override isOlderVersion(version: string, than: string): boolean {
    return Number(version.slice(1)) < Number(than.slice(1));
  }
}

/** A cache over an on-chain value `onChain`, whose reads can be held back with `hold()`. */
function makeCache(options: { readonly ordered?: boolean } = {}) {
  let now = 1_000;
  let onChain = 'v1';
  let gate: Promise<void> | undefined;
  const fetchFn = vi.fn(
    async (
      context: TestContext,
      _parameters: object,
      _cached: CachedValue<TestValue> | undefined,
    ): Promise<{ readonly version: string; readonly value: TestValue }> => {
      const version = onChain;
      await gate;
      if (context.misconfigured === true) {
        throw new FhevmConfigError({ message: 'wrong network' });
      }
      return { version, value: { data: version } };
    },
  );
  const cache = options.ordered === true ? new OrderedTestCache(fetchFn, () => now) : new TestCache(fetchFn, () => now);
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

describe('VersionedValueCache', () => {
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
    const pendingRead = cache.get({}, {}, 1_000);
    const provided = { data: 'provided' };
    cache.set(SLOT, 'p', provided);
    release();
    await pendingRead;
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
    const bad = await cache.get({}, {}, 1_000);
    const pendingRead = cache.get({}, {}, 0);
    cache.evict(SLOT, bad.value);
    release();
    await expect(pendingRead).resolves.toMatchObject({ value: bad.value });
    expect(cache.peek(SLOT)).toBeUndefined();
    // The slot is not stuck on that read: the next call reads again.
    const next = cache.get({}, {}, 1_000);
    await expect(next).resolves.toMatchObject({ version: 'v1' });
    expect(cache.peek(SLOT)).toBeDefined();
  });
});

// Values whose versions only increase (e.g. the KMS context). A source may lag
// (e.g. a load-balanced RPC node): an older value read must not replace a newer
// one, unless the newer one failed.
describe('VersionedValueCache with ordered versions', () => {
  it('a read returning an older version keeps the newer value, and returns it', async () => {
    const { cache, change } = makeCache({ ordered: true });
    change('v2');
    await cache.get({}, {}, 1_000);
    change('v1'); // a lagging source

    await expect(cache.get({}, {}, 0)).resolves.toMatchObject({ version: 'v2' });
    expect(cache.peek(SLOT)?.version).toBe('v2');
  });

  it('versions that are not ordered: the last read wins', async () => {
    const { cache, change } = makeCache();
    change('v2');
    await cache.get({}, {}, 1_000);
    change('v1');

    await expect(cache.get({}, {}, 0)).resolves.toMatchObject({ version: 'v1' });
    expect(cache.peek(SLOT)?.version).toBe('v1');
  });

  it('refresh(invalidVersion) with the cached version: the newer value failed, the older one read replaces it', async () => {
    const { cache, change } = makeCache({ ordered: true });
    change('v2');
    await cache.get({}, {}, 1_000);
    change('v1'); // e.g. a dev chain restarted from scratch

    await expect(cache.refresh({}, {}, 'v2')).resolves.toMatchObject({ version: 'v1' });
    expect(cache.peek(SLOT)?.version).toBe('v1');
  });

  it('refresh() with no invalidVersion (forced): the older value read replaces the newer one', async () => {
    const { cache, change } = makeCache({ ordered: true });
    change('v2');
    await cache.get({}, {}, 1_000);
    change('v1');

    await expect(cache.refresh({}, {})).resolves.toMatchObject({ version: 'v1' });
  });

  it('refresh(invalidVersion) when the cache holds an older version than the one that failed: reads again', async () => {
    const { cache, change, fetchFn } = makeCache({ ordered: true });
    change('v2');
    await cache.get({}, {}, 1_000);
    cache.set(SLOT, 'v1', { data: 'v1' }); // older than what the request used
    change('v3');
    fetchFn.mockClear();

    await expect(cache.refresh({}, {}, 'v2')).resolves.toMatchObject({ version: 'v3' });
    expect(fetchFn).toHaveBeenCalledOnce();
  });

  it('a refresh joining a read that kept the newer value reads once more, and takes the older one', async () => {
    const { cache, change, hold } = makeCache({ ordered: true });
    change('v2');
    await cache.get({}, {}, 1_000);
    change('v1');
    const release = hold();
    const reading = cache.get({}, {}, 0); // keeps v2: an older read does not replace it
    const refreshing = cache.refresh({}, {}, 'v2'); // v2 failed: takes v1
    release();

    await expect(reading).resolves.toMatchObject({ version: 'v2' });
    await expect(refreshing).resolves.toMatchObject({ version: 'v1' });
    expect(cache.peek(SLOT)?.version).toBe('v1');
  });
});

// The cache logs its decisions, never a cache hit, to the logger of the client
// whose call caused them.
describe('VersionedValueCache logs', () => {
  function makeLogger() {
    return { debug: vi.fn(), warn: vi.fn(), error: vi.fn() } satisfies Logger;
  }

  it('a version change: debug, with both versions; nothing on a hit or an unchanged read', async () => {
    const { cache, change } = makeCache();
    const logger = makeLogger();
    await cache.get({ logger }, {}, 1_000);
    await cache.get({ logger }, {}, 1_000); // a hit
    await cache.get({ logger }, {}, 0); // unchanged
    expect(logger.debug).not.toHaveBeenCalled();

    change('v2');
    await cache.get({ logger }, {}, 0);
    expect(logger.debug).toHaveBeenCalledOnce();
    expect(logger.debug).toHaveBeenCalledWith('testValue: version changed, v1 -> v2.');
    expect(logger.warn).not.toHaveBeenCalled();
  });

  it('an older value read and ignored: warn, with both versions', async () => {
    const { cache, change } = makeCache({ ordered: true });
    const logger = makeLogger();
    change('v2');
    await cache.get({ logger }, {}, 1_000);
    change('v1');

    await cache.get({ logger }, {}, 0);
    expect(logger.warn).toHaveBeenCalledOnce();
    expect(logger.warn.mock.calls[0]?.[0]).toContain('older version (v1) than the cached one (v2)');
  });

  it('a joined read failing on another client configuration: warn on the client that reads again', async () => {
    const { cache, hold } = makeCache();
    const misconfigured = makeLogger();
    const good = makeLogger();
    const release = hold();
    const badRead = cache.get({ misconfigured: true, logger: misconfigured }, {}, 1_000);
    const goodRead = cache.get({ logger: good }, {}, 1_000);
    release();
    await expect(badRead).rejects.toThrow(FhevmConfigError);
    await goodRead;

    expect(good.warn).toHaveBeenCalledOnce();
    expect(good.warn.mock.calls[0]?.[0]).toContain('failed on its configuration (wrong network)');
    expect(misconfigured.warn).not.toHaveBeenCalled();
  });

  it('more than 100 slots: warn, with the number of idle slots evicted', async () => {
    const { cache } = makeCache();
    const logger = makeLogger();
    for (let i = 0; i < 100; i++) {
      await cache.get({ slot: `slot-${i}` }, {}, 1_000);
    }
    expect(logger.warn).not.toHaveBeenCalled();

    await cache.get({ slot: 'slot-100', logger }, {}, 1_000);
    expect(logger.warn).toHaveBeenCalledOnce();
    expect(logger.warn.mock.calls[0]?.[0]).toContain('more than 100 cache slots, evicted 100 idle ones');
  });
});
