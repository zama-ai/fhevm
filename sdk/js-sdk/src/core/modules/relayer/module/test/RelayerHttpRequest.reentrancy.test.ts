import type { Mock } from 'vitest';
import type { Logger } from '../../../../types/logger.js';
import type { RelayerHttpOperation } from '../RelayerHttpAttempt.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from '../RelayerHttpRequest.js';
import { RelayerHttpResponseError } from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.reentrancy.test.ts
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
    // Second run() while the first fetch is pending: one-shot rejection; the first still resolves, 1 fetch.
    //   run() #1 ──► run() #2 rejects ──► fetch answers 200 ──► run() #1 resolves
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

    // Second run() after the first settled (success or failure): one-shot rejection, no new fetch.
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

    // Second run() after a request-build throw (BigInt payload): still rejected, the instance is spent.
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
    // Retry decision calls run() on its own instance: that call is rejected; the outer run retries and succeeds.
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

    // Retry decision starts another retrying request: both finish independently with their own fetch counts.
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

    // Retry decision aborts the caller signal and asks for a retry: caller's reason, no second fetch.
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

    // Retry decision aborts the caller signal and stops: the abort wins; the response error is still logged once.
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

    // Retry decision throws (any value): that value propagates as-is, cleanup runs, 1 fetch, no error logged.
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

    // Retry decision throws on attempt 2: that value propagates after 2 fetches.
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
    // The 'Retrying in' warn hook aborts the caller signal: caller's reason, no second fetch.
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

    // The warn hook throws: swallowed, the retry goes on and succeeds, cleanup runs.
    it('a warn hook that throws: swallowed, the retry succeeds, cleanup runs', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503, S200] });
      const controller = new AbortController();
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');
      const logger = makeLogger();
      logger.warn.mockImplementationOnce(() => {
        throw new Error('warn hook failed');
      });

      const run = scripted(() => 1000, { signal: controller.signal, timeout: 60_000 }, logger)
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(1000);

      expect(await run).toEqual(SHARES);
      expect(global.fetch).toHaveBeenCalledTimes(2);
      expect(vi.getTimerCount()).toBe(0);
      expect(removeSpy).toHaveBeenCalledTimes(1);
    });

    // The error hook throws when the run stops: swallowed, the response error still rejects, no timer left.
    it('an error hook that throws at the stop: swallowed, the response error rejects, cleanup runs', async () => {
      vi.useFakeTimers();
      global.fetch = fetchByUrl({ [RELAYER_URL]: [S503] });
      const logger = makeLogger();
      logger.error.mockImplementationOnce(() => {
        throw new Error('error hook failed');
      });

      const error = await scripted(() => undefined, { timeout: 60_000 }, logger)
        .run()
        .catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpResponseError);
      expect((error as RelayerHttpResponseError).status).toBe(503);
      expect(logger.error).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // The warn hook moves the clock past the deadline: RelayerHttpTimeoutError, no second fetch.
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

    // The warn hook calls run() on the same instance: that call is rejected; the outer run retries and succeeds.
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

  // Caller aborts inside fetch, yet a 503 is still delivered: no retry decision, the abort wins.
  //   fetch starts ──► caller aborts ──► 503 delivered ──► run() rejects 'aborted in fetch'
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
    // Two concurrent requests with different scripts, timeouts and signals: each gets its own outcome.
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

    // Two concurrent requests, one aborted during its wait: only that one rejects, the other succeeds.
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

    // Two requests share one caller signal, aborted during both waits: both reject with its reason, all cleaned up.
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
