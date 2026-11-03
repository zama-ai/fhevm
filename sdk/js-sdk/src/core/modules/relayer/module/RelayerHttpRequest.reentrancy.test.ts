import type { Mock } from 'vitest';
import type { Logger } from '../../../types/logger.js';
import type { RelayerHttpOperation } from './RelayerHttpAttempt.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from './RelayerHttpRequest.js';
import { RelayerHttpResponseError } from '../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpTimeoutError } from '../../../errors/RelayerHttpTimeoutError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/RelayerHttpRequest.reentrancy.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const OTHER_URL = 'https://relayer-http.example.com/v4/public-decrypt';
const VALID_REQUEST_ID = '018f4d2e-7c3a-7b1e-8c2d-abcdef123456';
const SHARES = [{ payload: 'ab'.repeat(32), signature: 'cd'.repeat(65), extraData: '0x00' }];
const ONE_SHOT_MESSAGE =
  'RelayerHttpRequest.run() was already called on this instance. Create a new instance per request.';

type RequestParams = ConstructorParameters<typeof RelayerHttpRequest<'USER_DECRYPT'>>[0];
type Options = NonNullable<RequestParams['options']>;
type Decide = (
  self: RelayerHttpRequest<RelayerHttpOperation>,
  error: RelayerHttpResponseError,
  attempt: number,
) => unknown;

// Test-local override (design §4.5): inherits the constructor unchanged, closes over `decide`.
function scriptedRequestClass(decide: Decide) {
  return class ScriptedRequest<O extends RelayerHttpOperation> extends RelayerHttpRequest<O> {
    protected override _retryDecision(error: RelayerHttpResponseError, attempt: number): number | undefined {
      return decide(this as RelayerHttpRequest<RelayerHttpOperation>, error, attempt) as number | undefined;
    }
  };
}

function scripted(
  decide: Decide,
  options: Options = {},
  logger?: Logger,
  url = RELAYER_URL,
): RelayerHttpRequest<'USER_DECRYPT'> {
  const ScriptedRequest = scriptedRequestClass(decide);
  return new ScriptedRequest({ relayerOperation: 'USER_DECRYPT', url, payload: {}, options, logger });
}

type TestLogger = Logger & { debug: Mock; warn: Mock; error: Mock };
function makeLogger(): TestLogger {
  return { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };
}

type Step = { status: number; body: string } | 'pending';

const S503: Step = {
  status: 503,
  body: JSON.stringify({ code: 'overloaded', message: 'busy', requestId: VALID_REQUEST_ID }),
};
const S200: Step = {
  status: 200,
  body: JSON.stringify({ status: 'succeeded', requestId: VALID_REQUEST_ID, result: { result: SHARES } }),
};

// One finite step list per url, a fresh Response per call; a call past the end fails loudly.
function fetchByUrl(stepsByUrl: Record<string, Step[]>): Mock {
  const indexes = new Map<string, number>();
  return vi.fn((url: string, init?: RequestInit) => {
    const index = indexes.get(url) ?? 0;
    indexes.set(url, index + 1);
    const step = stepsByUrl[url]?.[index];
    if (step === undefined) {
      return Promise.reject(new Error(`fetch script exhausted for ${url} (call ${index + 1})`));
    }
    if (step === 'pending') {
      return new Promise<Response>((_resolve, reject) => {
        init?.signal?.addEventListener('abort', () => reject(init.signal?.reason), { once: true });
      });
    }
    return Promise.resolve(new Response(step.body, { status: step.status }));
  });
}

function callsTo(url: string): number {
  return (global.fetch as Mock).mock.calls.filter(([u]) => u === url).length;
}

describe('RelayerHttpRequest reentrancy', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  describe('one-shot run()', () => {
    it('a second run() while the first fetch is in flight rejects; the first completes normally', async () => {
      vi.useFakeTimers();
      let resolveFetch: (response: Response) => void = () => undefined;
      global.fetch = vi.fn(
        () =>
          new Promise<Response>((resolve) => {
            resolveFetch = resolve;
          }),
      );
      const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });

      const first = request.run();
      await expect(request.run()).rejects.toThrow(ONE_SHOT_MESSAGE);
      await vi.advanceTimersByTimeAsync(0);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      resolveFetch(new Response(S200.body, { status: 200 }));

      await expect(first).resolves.toEqual(SHARES);
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    it.each([
      ['a success', [S200]],
      ['a failure', [S503]],
    ])('a second run() after %s rejects without fetching', async (_label, steps) => {
      global.fetch = fetchByUrl({ [RELAYER_URL]: steps });
      const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });

      await request.run().catch(() => undefined);
      await expect(request.run()).rejects.toThrow(ONE_SHOT_MESSAGE);
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    it('a second run() after a build throw also rejects (the instance is spent)', async () => {
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S200] });
      const request = new RelayerHttpRequest({
        relayerOperation: 'USER_DECRYPT',
        url: RELAYER_URL,
        payload: { big: 1n },
      });

      await expect(request.run()).rejects.toBeInstanceOf(TypeError);
      await expect(request.run()).rejects.toThrow(ONE_SHOT_MESSAGE);
      expect(global.fetch).not.toHaveBeenCalled();
    });
  });

  describe('decide overrides', () => {
    it('a decide that calls run() on the same instance gets the one-shot rejection; the outer run is unaffected', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200] });
      let inner: Promise<unknown> | undefined;
      const request = scripted((self, _error, attempt) => {
        inner = self.run().catch((e: unknown) => e);
        return attempt === 1 ? 10 : undefined;
      });

      const outer = request.run();
      await vi.advanceTimersByTimeAsync(10);

      await expect(outer).resolves.toEqual(SHARES);
      expect(((await inner) as Error).message).toBe(ONE_SHOT_MESSAGE);
      expect(global.fetch).toHaveBeenCalledTimes(2);
    });

    it('a decide that starts another (retrying) request: both complete independently', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200], [OTHER_URL]: [S503, S503, S200] });
      let nested: Promise<unknown> | undefined;
      const nestedDecide = vi.fn<Decide>((_self, _error, attempt) => (attempt <= 2 ? 20 : undefined));
      const request = scripted(() => {
        nested ??= scripted(nestedDecide, { timeout: 60_000 }, undefined, OTHER_URL).run();
        return 10;
      });

      // Outer: one 10 ms wait; nested: two 20 ms waits. Advance exactly the slowest.
      const outer = request.run();
      await vi.advanceTimersByTimeAsync(40);

      await expect(outer).resolves.toEqual(SHARES);
      await expect(nested).resolves.toEqual(SHARES);
      expect(callsTo(RELAYER_URL)).toBe(2);
      expect(callsTo(OTHER_URL)).toBe(3);
      expect(nestedDecide).toHaveBeenCalledTimes(2);
      expect(vi.getTimerCount()).toBe(0);
    });

    it('a decide that aborts the caller signal and returns a delay: the caller reason, no second fetch', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200] });
      const controller = new AbortController();
      const request = scripted(
        () => {
          controller.abort('aborted in decide');
          return 1000;
        },
        { signal: controller.signal, timeout: 60_000 },
      );

      expect(await request.run().catch((e: unknown) => e)).toBe('aborted in decide');
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // The stop logs the response error, then the abort before settlement wins over it.
    it('a decide that aborts the caller signal and returns undefined: the caller reason, error logged once', async () => {
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503] });
      const controller = new AbortController();
      const logger = makeLogger();
      const request = scripted(
        () => {
          controller.abort('aborted in decide');
          return undefined;
        },
        { signal: controller.signal },
        logger,
      );

      expect(await request.run().catch((e: unknown) => e)).toBe('aborted in decide');
      expect(logger.error).toHaveBeenCalledTimes(1);
      expect(logger.warn).toHaveBeenCalledTimes(1);
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    it.each([
      ['an Error', new Error('decide failed')],
      ['a string', 'decide failed'],
      ['null', null],
    ])('a decide that throws %s: that value propagates, cleanup runs, 1 fetch', async (_label, value) => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200] });
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
      const logger = makeLogger();
      const request = scripted(
        () => {
          throw value;
        },
        { signal: controller.signal, timeout: 60_000 },
        logger,
      );

      expect(
        await request.run().then(
          () => 'resolved',
          (e: unknown) => e,
        ),
      ).toBe(value);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
      expect(removeSpy).toHaveBeenCalledWith('abort', addSpy.mock.calls[0]![1]);
      // Not a stop (§5 guard 4): no relayer-http error line.
      expect(logger.error).not.toHaveBeenCalled();
    });

    it('a decide that throws on a retried attempt: that value propagates after 2 fetches', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S503] });
      const thrown = new Error('second decision failed');
      const request = scripted((_self, _error, attempt) => {
        if (attempt === 2) {
          throw thrown;
        }
        return 10;
      });

      const run = request.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);

      expect(await run).toBe(thrown);
      expect(global.fetch).toHaveBeenCalledTimes(2);
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  describe('logger hooks', () => {
    it('a warn hook that aborts the caller signal: the caller reason, no second fetch', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200] });
      const controller = new AbortController();
      const logger = makeLogger();
      logger.warn.mockImplementationOnce(() => controller.abort('aborted in warn'));

      const run = scripted(() => 1000, { signal: controller.signal, timeout: 60_000 }, logger)
        .run()
        .catch((e: unknown) => e);

      expect(await run).toBe('aborted in warn');
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    it('a warn hook that throws: that value propagates, cleanup runs, no second fetch', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200] });
      const controller = new AbortController();
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
      const hookError = new Error('warn hook failed');
      const logger = makeLogger();
      logger.warn.mockImplementationOnce(() => {
        throw hookError;
      });

      const run = scripted(() => 1000, { signal: controller.signal, timeout: 60_000 }, logger)
        .run()
        .catch((e: unknown) => e);

      expect(await run).toBe(hookError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
      expect(removeSpy).toHaveBeenCalledTimes(1);
    });

    it('an error hook that throws at the stop: that value propagates, cleanup runs', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503] });
      const hookError = new Error('error hook failed');
      const logger = makeLogger();
      logger.error.mockImplementationOnce(() => {
        throw hookError;
      });

      expect(
        await scripted(() => undefined, { timeout: 60_000 }, logger)
          .run()
          .catch((e: unknown) => e),
      ).toBe(hookError);
      expect(vi.getTimerCount()).toBe(0);
    });

    it('a warn hook that advances time past the deadline: RelayerHttpTimeoutError, no second fetch', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200] });
      const logger = makeLogger();
      logger.warn.mockImplementationOnce(() => {
        vi.advanceTimersByTime(1000);
      });

      const run = scripted(() => 500, { timeout: 1000 }, logger)
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);

      expect(await run).toBeInstanceOf(RelayerHttpTimeoutError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    it('a warn hook that calls run() on the same instance: one-shot rejection, the outer run retries', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200] });
      const logger = makeLogger();
      let request: RelayerHttpRequest<'USER_DECRYPT'> | undefined;
      let inner: Promise<unknown> | undefined;
      logger.warn.mockImplementationOnce(() => {
        inner = request!.run().catch((e: unknown) => e);
      });
      request = scripted((_self, _error, attempt) => (attempt === 1 ? 10 : undefined), {}, logger);

      const outer = request.run();
      await vi.advanceTimersByTimeAsync(10);

      await expect(outer).resolves.toEqual(SHARES);
      expect(((await inner) as Error).message).toBe(ONE_SHOT_MESSAGE);
    });
  });

  // An abort that lands after the response arrived: never retries, and the abort wins over the error.
  it('a caller abort fired from inside fetch (response still delivered): no retry, decide not called, the caller reason', async () => {
    const controller = new AbortController();
    global.fetch = vi.fn(() => {
      controller.abort('aborted in fetch');
      return Promise.resolve(new Response(S503.body, { status: 503 }));
    });
    const decide = vi.fn<Decide>(() => 10);

    const error = await scripted(decide, { signal: controller.signal })
      .run()
      .catch((e: unknown) => e);

    expect(decide).not.toHaveBeenCalled();
    expect(global.fetch).toHaveBeenCalledTimes(1);
    expect(error).toBe('aborted in fetch');
  });

  describe('two instances concurrently', () => {
    it('share no state: different scripts, timeouts and signals', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200], [OTHER_URL]: [S503, S503, S503] });
      const decideA = vi.fn<Decide>(() => 100);
      const decideB = vi.fn<Decide>((_self, _error, attempt) => (attempt < 3 ? 30 : undefined));
      const controllerA = new AbortController();
      const controllerB = new AbortController();

      const runA = scripted(decideA, { signal: controllerA.signal, timeout: 10_000 }).run();
      const runB = scripted(decideB, { signal: controllerB.signal, timeout: 500 }, undefined, OTHER_URL)
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(100);

      await expect(runA).resolves.toEqual(SHARES);
      expect(await runB).toBeInstanceOf(RelayerHttpResponseError);
      expect(decideA.mock.calls.map(([, , a]) => a)).toEqual([1]);
      expect(decideB.mock.calls.map(([, , a]) => a)).toEqual([1, 2, 3]);
      expect(callsTo(RELAYER_URL)).toBe(2);
      expect(callsTo(OTHER_URL)).toBe(3);
      expect(vi.getTimerCount()).toBe(0);
    });

    it("aborting one instance's signal during its wait does not affect the other", async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200], [OTHER_URL]: [S503, S200] });
      const controllerA = new AbortController();
      const controllerB = new AbortController();

      const runA = scripted(() => 1000, { signal: controllerA.signal, timeout: 60_000 })
        .run()
        .catch((e: unknown) => e);
      const runB = scripted(() => 1000, { signal: controllerB.signal, timeout: 60_000 }, undefined, OTHER_URL).run();
      await vi.advanceTimersByTimeAsync(10);
      controllerA.abort('only A');
      await vi.advanceTimersByTimeAsync(1000);

      expect(await runA).toBe('only A');
      await expect(runB).resolves.toEqual(SHARES);
      expect(callsTo(RELAYER_URL)).toBe(1);
      expect(callsTo(OTHER_URL)).toBe(2);
    });

    it('one shared caller signal aborted during both waits: both get the reason, nothing left behind', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200], [OTHER_URL]: [S503, S200] });
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

      const runA = scripted(() => 1000, { signal: controller.signal, timeout: 60_000 })
        .run()
        .catch((e: unknown) => e);
      const runB = scripted(() => 2000, { signal: controller.signal }, undefined, OTHER_URL)
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);
      controller.abort('both');

      expect(await runA).toBe('both');
      expect(await runB).toBe('both');
      expect(addSpy).toHaveBeenCalledTimes(2);
      expect(removeSpy).toHaveBeenCalledTimes(2);
      expect(vi.getTimerCount()).toBe(0);
    });
  });
});
