import type { Mock } from 'vitest';
import type { Logger } from '../../../../types/logger.js';
import type { RelayerHttpOperation } from '../RelayerHttpAttempt.js';
import { describe, it, expect, afterEach, vi } from 'vitest';
import { RelayerHttpRequest } from '../RelayerHttpRequest.js';
import { MAX_TIMEOUT_MS } from '../relayerHttpSignal.js';
import { RelayerHttpFetchError } from '../../../../errors/RelayerHttpFetchError.js';
import {
  KNOWN_RELAYER_HTTP_ERROR_CODES,
  RelayerHttpResponseError,
} from '../../../../errors/RelayerHttpResponseError.js';
import { RelayerHttpResultError } from '../../../../errors/RelayerHttpResultError.js';
import { RelayerHttpTimeoutError } from '../../../../errors/RelayerHttpTimeoutError.js';
import { RelayerHttpUnexpectedResponseError } from '../../../../errors/RelayerHttpUnexpectedResponseError.js';

////////////////////////////////////////////////////////////////////////////////
// npx vitest run --config src/vitest.config.ts src/core/modules/relayer/module/test/RelayerHttpRequest.retry.test.ts
////////////////////////////////////////////////////////////////////////////////

const RELAYER_URL = 'https://relayer-http.example.com/v4/user-decrypt';
const VALID_REQUEST_ID = '9b2c1f4e-3a5d-4e6f-8a7b-1c2d3e4f5a6b';
const SHARES = [{ payload: 'ab'.repeat(32), signature: 'cd'.repeat(65), extraData: '0x00' }];
const MAX_BODY_TEXT_LENGTH = 10 * 1024 * 1024;

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

// A finite script: the n-th decision returns script[n - 1], then undefined.
function scriptOf(script: ReadonlyArray<number | undefined>): Mock<Decide> {
  let index = 0;
  return vi.fn<Decide>(() => script[index++]);
}

type TestLogger = Logger & { debug: Mock; warn: Mock; error: Mock };
function makeLogger(): TestLogger {
  return { debug: vi.fn(), warn: vi.fn(), error: vi.fn() };
}

function scripted(
  decide: Mock<Decide> | Decide,
  options: Options = {},
  extra: { payload?: Record<string, unknown>; logger?: Logger } = {},
): RelayerHttpRequest<'USER_DECRYPT'> {
  const ScriptedRequest = scriptedRequestClass(decide);
  return new ScriptedRequest({
    relayerOperation: 'USER_DECRYPT',
    url: RELAYER_URL,
    payload: extra.payload ?? { a: 1 },
    options,
    logger: extra.logger,
  });
}

function successBody(): string {
  return JSON.stringify({ status: 'succeeded', requestId: VALID_REQUEST_ID, result: { result: SHARES } });
}

function errorBody(message = 'busy', code = 'overloaded'): string {
  return JSON.stringify({ code, message, requestId: VALID_REQUEST_ID });
}

type Step = { status: number; body: string } | { reject: unknown } | 'pending' | 'stalled';

// A fresh Response per call, following a finite list of steps; a call past the end fails loudly.
function fetchSequence(steps: Step[]): Mock {
  let index = 0;
  return vi.fn((_url: unknown, init?: RequestInit) => {
    const step = steps[index++];
    if (step === undefined) {
      return Promise.reject(new Error(`fetch script exhausted (call ${index})`));
    }
    if (step === 'pending') {
      return new Promise<Response>((_resolve, reject) => {
        init?.signal?.addEventListener('abort', () => reject(init.signal?.reason), { once: true });
      });
    }
    if (step === 'stalled') {
      const body = new ReadableStream<Uint8Array>({
        start(controller) {
          init?.signal?.addEventListener('abort', () => controller.error(init.signal?.reason), { once: true });
        },
      });
      return Promise.resolve(new Response(body, { status: 200 }));
    }
    if ('reject' in step) {
      return Promise.reject(step.reject);
    }
    return Promise.resolve(new Response(step.body, { status: step.status }));
  });
}

const S503 = (message = 'busy'): Step => ({ status: 503, body: errorBody(message) });
const S200: Step = { status: 200, body: successBody() };

// AbortSignal.timeout() on the fake clock: same caller-owned TimeoutError reason, no wall-clock wait.
function fakeAbortSignalTimeout(ms: number): AbortSignal {
  const controller = new AbortController();
  setTimeout(() => controller.abort(new DOMException('The operation timed out.', 'TimeoutError')), ms);
  return controller.signal;
}

function stopLine(message: string, status = 503, code = 'overloaded'): string {
  return `[RelayerHttpRequest] relayer-http error on ${RELAYER_URL} (status=${status}, code=${code}, requestId=${VALID_REQUEST_ID}): ${message}`;
}

describe('RelayerHttpRequest retry through run()', () => {
  const originalFetch = global.fetch;

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  describe('default decision (base class, R14)', () => {
    // Base class, a 503: no retry, 1 fetch, one error line, no warn, no timer left.
    it('503 relayer-http body → 1 fetch, RelayerHttpResponseError, 1 error line, no warn, no timer', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const logger = makeLogger();
      const request = new RelayerHttpRequest({
        relayerOperation: 'USER_DECRYPT',
        url: RELAYER_URL,
        payload: {},
        options: { timeout: 60_000 },
        logger,
      });

      const error = await request.run().catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpResponseError);
      expect((error as RelayerHttpResponseError).status).toBe(503);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(logger.error).toHaveBeenCalledTimes(1);
      expect(logger.error).toHaveBeenCalledWith(stopLine('busy'), undefined);
      expect(logger.warn).not.toHaveBeenCalled();
      expect(vi.getTimerCount()).toBe(0);
    });

    // The base class never retries a relayer-http error, whatever the status: 1 fetch.
    it.each([400, 401, 403, 404, 408, 429, 500, 502, 503, 504])(
      'never retries a %d relayer-http error (1 fetch)',
      async (status) => {
        global.fetch = fetchSequence([{ status, body: errorBody() }, S200]);
        const request = new RelayerHttpRequest({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });

        await expect(request.run()).rejects.toBeInstanceOf(RelayerHttpResponseError);
        expect(global.fetch).toHaveBeenCalledTimes(1);
      },
    );
  });

  // R14 contract only; S22's policy table comes with the policy task.
  describe('default _retryDecision (pass-through subclass)', () => {
    class PassThrough extends RelayerHttpRequest<'USER_DECRYPT'> {
      public decide(error: RelayerHttpResponseError, attempt: number): number | undefined {
        return this._retryDecision(error, attempt);
      }
    }
    const STATUSES = [400, 401, 403, 404, 405, 408, 409, 413, 422, 429, 500, 501, 502, 503, 504];
    const CODES: Array<string | undefined> = [
      ...KNOWN_RELAYER_HTTP_ERROR_CODES,
      'unknown',
      'brand_new_code',
      undefined,
    ];

    // The default retry decision says stop for every code, status and attempt.
    it.each(CODES.map((code) => [String(code), code]))(
      'returns undefined for code %s, every status, attempts 1..5',
      (_l, code) => {
        const passThrough = new PassThrough({ relayerOperation: 'USER_DECRYPT', url: RELAYER_URL, payload: {} });
        for (const status of STATUSES) {
          const error = new RelayerHttpResponseError({ url: RELAYER_URL, status, code, requestId: VALID_REQUEST_ID });
          for (let attempt = 1; attempt <= 5; attempt++) {
            expect(passThrough.decide(error, attempt)).toBeUndefined();
          }
        }
      },
    );
  });

  describe('override dispatch', () => {
    // The override receives the request itself, the exact error run() rejects with, and attempt 1.
    it('calls the override through `this` with the exact thrown error and a 1-based attempt', async () => {
      global.fetch = fetchSequence([S503()]);
      const decide = scriptOf([undefined]);
      const request = scripted(decide);

      const error = await request.run().catch((e: unknown) => e);

      expect(decide).toHaveBeenCalledTimes(1);
      const [self, seen, attempt] = decide.mock.calls[0]!;
      expect(self).toBe(request);
      expect(seen).toBe(error);
      expect(seen).toBeInstanceOf(RelayerHttpResponseError);
      expect(seen.code).toBe('overloaded');
      expect(attempt).toBe(1);
    });

    // Two concurrent requests with different scripts: each follows its own, attempts counted separately.
    it('two instances with different scripts follow their own script', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S503(), S503(), S503()]);
      const decideA = scriptOf([10, 10, undefined]);
      const decideB = scriptOf([undefined]);

      const runA = scripted(decideA)
        .run()
        .catch((e: unknown) => e);
      const runB = scripted(decideB)
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(20);
      await Promise.all([runA, runB]);

      expect(decideA.mock.calls.map(([, , a]) => a)).toEqual([1, 2, 3]);
      expect(decideB.mock.calls.map(([, , a]) => a)).toEqual([1]);
      expect(global.fetch).toHaveBeenCalledTimes(4);
    });
  });

  describe('S23 response sequences', () => {
    // A 503 then a 200, with one 1000 ms retry: the result after 2 fetches, one retry warn.
    it('503 then 200: 2 fetches, the result, 1 warn, no error line', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const logger = makeLogger();
      const request = scripted(scriptOf([1000]), {}, { logger });

      const run = request.run();
      await vi.advanceTimersByTimeAsync(1000);

      await expect(run).resolves.toEqual(SHARES);
      expect(global.fetch).toHaveBeenCalledTimes(2);
      expect(logger.warn).toHaveBeenCalledTimes(1);
      expect(logger.warn).toHaveBeenCalledWith(
        `[RelayerHttpRequest] relayer-http error on ${RELAYER_URL} (status=503, code=overloaded, requestId=${VALID_REQUEST_ID}): busy. Retrying in 1000 ms (attempt 1).`,
      );
      expect(logger.error).not.toHaveBeenCalled();
    });

    // Override stops on the first 503: run() rejects after 1 fetch.
    it('stop on the first error: 1 fetch', async () => {
      global.fetch = fetchSequence([S503(), S200]);

      await expect(scripted(scriptOf([undefined])).run()).rejects.toBeInstanceOf(RelayerHttpResponseError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    // Override retries 3 times then stops: rejects with the 4th error, 3 warns, 1 error line.
    it('a script ending after 3 retries: 4 fetches, the last error, one error line', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503('m1'), S503('m2'), S503('m3'), S503('m4'), S200]);
      const logger = makeLogger();
      const request = scripted(scriptOf([10, 10, 10, undefined]), {}, { logger });

      const run = request.run().catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(30);

      const error = await run;
      expect((error as RelayerHttpResponseError).responseMessage).toBe('m4');
      expect(global.fetch).toHaveBeenCalledTimes(4);
      expect(logger.warn).toHaveBeenCalledTimes(3);
      expect(logger.error).toHaveBeenCalledTimes(1);
      expect(logger.error).toHaveBeenCalledWith(stopLine('m4'), undefined);
    });

    // Override always says retry: run() still stops at 5 fetches with the 5th error.
    it('an always-retry override stops at 5 fetches (R15)', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503('m1'), S503('m2'), S503('m3'), S503('m4'), S503('m5'), S200]);
      const decide = vi.fn<Decide>(() => 1);

      const run = scripted(decide)
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(100);

      expect(((await run) as RelayerHttpResponseError).responseMessage).toBe('m5');
      expect(global.fetch).toHaveBeenCalledTimes(5);
      expect(decide).toHaveBeenCalledTimes(4);
    });

    // After a retried 503, a different kind of failure is returned as-is, with no further retry.
    it.each([
      ['non-JSON', { status: 502, body: '<html>bad gateway</html>' }, RelayerHttpFetchError],
      ['a non-relayer JSON error', { status: 500, body: '{"error":"x"}' }, RelayerHttpUnexpectedResponseError],
      ['a schema mismatch', { status: 200, body: '{"status":"succeeded"}' }, RelayerHttpResultError],
    ] as const)('503 then %s: that type unchanged, decide called once', async (_label, step, type) => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), step, S200]);
      const decide = vi.fn<Decide>(() => 10);

      const run = scripted(decide)
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);

      expect(await run).toBeInstanceOf(type);
      expect(global.fetch).toHaveBeenCalledTimes(2);
      expect(decide).toHaveBeenCalledTimes(1);
    });

    // After a retried 503, a network failure with no fetch retries left: RelayerHttpFetchError, no further retry.
    it('503 then a network error (retries exhausted): RelayerHttpFetchError, decide called once', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), { reject: new TypeError('fetch failed') }]);
      const decide = vi.fn<Decide>(() => 10);

      const run = scripted(decide, { fetchRetries: 0 })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);

      expect(await run).toBeInstanceOf(RelayerHttpFetchError);
      expect(decide).toHaveBeenCalledTimes(1);
    });

    // An oversized body on the retried attempt is still rejected as too large.
    it('a retried request still rejects an oversized body', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), { status: 200, body: `"${'a'.repeat(MAX_BODY_TEXT_LENGTH - 1)}"` }]);
      const decide = vi.fn<Decide>(() => 10);

      const run = scripted(decide)
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(10);

      const error = await run;
      expect(error).toBeInstanceOf(RelayerHttpFetchError);
      expect((error as Error).message).toContain('too large');
      expect((error as RelayerHttpFetchError).status).toBe(200);
      expect(decide).toHaveBeenCalledTimes(1);
    });

    // Retry delay longer than the whole timeout: no wait, the 503 error is returned at once.
    it('timeout 500 with a 1000 ms script delay: 1 fetch, no wait, the response error at once', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const logger = makeLogger();

      const error = await scripted(scriptOf([1000]), { timeout: 500 }, { logger })
        .run()
        .catch((e: unknown) => e);

      expect(error).toBeInstanceOf(RelayerHttpResponseError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(logger.warn).not.toHaveBeenCalled();
      expect(logger.error).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Retry delay vs time left: waits only if the delay is strictly below it.
    it.each([
      ['equal to', 1000, false],
      ['one below', 999, true],
    ])('a delay %s the remaining timeout: waits only when below', async (_label, delay, waits) => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);

      const run = scripted(scriptOf([delay]), { timeout: 1000 })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(delay);

      if (waits) {
        expect(await run).toEqual(SHARES);
        expect(global.fetch).toHaveBeenCalledTimes(2);
      } else {
        expect(await run).toBeInstanceOf(RelayerHttpResponseError);
        expect(global.fetch).toHaveBeenCalledTimes(1);
      }
    });

    // Delay equal to a MAX_TIMEOUT_MS timeout: not strictly below, so no wait (the abort is only a backstop).
    it('timeout MAX_TIMEOUT_MS with a MAX_TIMEOUT_MS delay: no wait (not strictly below)', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);

      const controller = new AbortController();

      const run = scripted(scriptOf([MAX_TIMEOUT_MS]), { signal: controller.signal, timeout: MAX_TIMEOUT_MS })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);
      controller.abort('backstop: a wait was accepted');

      expect(await run).toBeInstanceOf(RelayerHttpResponseError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });

    // Override returns an unusable delay (zero, negative, fractional, NaN, infinite, too large, non-number):
    // stop at once, no wait.
    it.each([0, -1, 1.5, Number.NaN, Number.POSITIVE_INFINITY, MAX_TIMEOUT_MS + 1, '10', { valueOf: () => 10 }])(
      'an override returning the invalid delay %s fails closed: 1 fetch, same error type, no timer',
      async (value) => {
        vi.useFakeTimers();
        global.fetch = fetchSequence([S503(), S200]);

        const controller = new AbortController();

        const run = scripted(() => value, { signal: controller.signal })
          .run()
          .catch((e: unknown) => e);
        await vi.advanceTimersByTimeAsync(0);
        expect(vi.getTimerCount()).toBe(0);
        controller.abort('backstop: a wait was accepted');

        expect(await run).toBeInstanceOf(RelayerHttpResponseError);
        expect(global.fetch).toHaveBeenCalledTimes(1);
      },
    );
  });

  describe('decide never sees anything but a RelayerHttpResponseError', () => {
    // Non-response failures never reach the retry decision: their own error type after 1 fetch.
    it.each([
      ['a network error', { reject: new TypeError('fetch failed') } as Step, RelayerHttpFetchError],
      ['a non-JSON body', { status: 503, body: 'oops' } as Step, RelayerHttpFetchError],
      ['a non-relayer JSON error', { status: 503, body: '{"code":"x"}' } as Step, RelayerHttpUnexpectedResponseError],
      ['a schema mismatch', { status: 200, body: '{}' } as Step, RelayerHttpResultError],
    ])('%s: that error type, decide not called, 1 fetch', async (_label, step, type) => {
      global.fetch = fetchSequence([step]);
      const decide = vi.fn<Decide>(() => 10);

      const error = await scripted(decide, { fetchRetries: 0 })
        .run()
        .catch((e: unknown) => e);

      expect(error).toBeInstanceOf(type);
      expect(error).not.toBeInstanceOf(RelayerHttpResponseError);
      expect(decide).not.toHaveBeenCalled();
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    // Our timeout fires during fetch: RelayerHttpTimeoutError, retry decision never asked.
    it('our timeout during fetch: RelayerHttpTimeoutError, decide not called', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence(['pending']);
      const decide = vi.fn<Decide>(() => 10);

      const run = scripted(decide, { timeout: 500 })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(500);

      expect(await run).toBeInstanceOf(RelayerHttpTimeoutError);
      expect(decide).not.toHaveBeenCalled();
    });

    // Caller aborts during fetch with a response-error reason: that exact reason, never treated as retryable.
    it('a caller abort whose reason is a RelayerHttpResponseError, during fetch: that reason, decide not called', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence(['pending']);
      const decide = vi.fn<Decide>(() => 10);
      const controller = new AbortController();
      const reason = new RelayerHttpResponseError({ url: RELAYER_URL, status: 503, code: 'overloaded' });

      const run = scripted(decide, { signal: controller.signal })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      controller.abort(reason);

      expect(await run).toBe(reason);
      expect(decide).not.toHaveBeenCalled();
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    // Caller signal already aborted with a response-error reason: rejects with it, no fetch.
    it('a caller signal already aborted with a RelayerHttpResponseError: that reason, no fetch, decide not called', async () => {
      global.fetch = fetchSequence([S503()]);
      const decide = vi.fn<Decide>(() => 10);
      const reason = new RelayerHttpResponseError({ url: RELAYER_URL, status: 503, code: 'overloaded' });

      const error = await scripted(decide, { signal: AbortSignal.abort(reason) })
        .run()
        .catch((e: unknown) => e);

      expect(error).toBe(reason);
      expect(global.fetch).not.toHaveBeenCalled();
      expect(decide).not.toHaveBeenCalled();
    });
  });

  // S24: abort during the wait between HTTP attempts.
  describe('abort during the wait', () => {
    // Caller aborts while waiting to retry: rejects with the caller's reason, listener removed, no timer left.
    it.each([
      ['a string', 'user left'],
      ['null', null],
      ['a caller DOMException TimeoutError', new DOMException('Caller timed out', 'TimeoutError')],
      ['an Error', new Error('cancelled')],
      ['a RelayerHttpResponseError', new RelayerHttpResponseError({ url: RELAYER_URL, status: 503 })],
    ])('caller abort with %s → that reason, 1 fetch, decide once, cleanup', async (_label, reason) => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const decide = vi.fn<Decide>(() => 1000);
      const controller = new AbortController();
      const addSpy = vi.spyOn(controller.signal, 'addEventListener');
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

      const run = scripted(decide, { signal: controller.signal, timeout: 60_000 })
        .run()
        .then(
          () => 'resolved',
          (e: unknown) => e,
        );
      await vi.advanceTimersByTimeAsync(10);
      controller.abort(reason);

      const error = await run;
      expect(error).toBe(reason);
      expect(error).not.toBeInstanceOf(RelayerHttpTimeoutError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(decide).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
      expect(addSpy).toHaveBeenCalledTimes(1);
      expect(removeSpy).toHaveBeenCalledWith('abort', addSpy.mock.calls[0]![1]);
    });

    // Caller's own AbortSignal.timeout() fires during the retry wait: its TimeoutError, not ours.
    it("the caller's own AbortSignal.timeout() during the wait → its reason, not RelayerHttpTimeoutError", async () => {
      vi.useFakeTimers();
      vi.spyOn(AbortSignal, 'timeout').mockImplementation(fakeAbortSignalTimeout);
      global.fetch = fetchSequence([S503(), S200]);
      const callerSignal = AbortSignal.timeout(20);

      const run = scripted(scriptOf([60_000]), { signal: callerSignal, timeout: 120_000 })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(20);

      const error = await run;
      expect(error).toBe(callerSignal.reason);
      expect((error as DOMException).name).toBe('TimeoutError');
      expect(vi.getTimerCount()).toBe(0);
      expect(error).not.toBeInstanceOf(RelayerHttpTimeoutError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    // Our timeout fires while waiting to retry: RelayerHttpTimeoutError, listener and timers cleaned up.
    //   503 ──► warn (clock +500) ──► wait 600 starts ──► +500: our 1000 ms deadline ──► timeout
    it('our timer during the wait → RelayerHttpTimeoutError, 1 fetch, cleanup', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const logger = makeLogger();
      logger.warn.mockImplementationOnce(() => {
        vi.advanceTimersByTime(500);
      });
      const controller = new AbortController();
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

      const run = scripted(scriptOf([600]), { signal: controller.signal, timeout: 1000 }, { logger })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);
      expect(logger.warn).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(2);
      await vi.advanceTimersByTimeAsync(500);

      const error = await run;
      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(1000);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
      expect(removeSpy).toHaveBeenCalledTimes(1);
      expect(logger.warn).toHaveBeenLastCalledWith(
        `[RelayerHttpRequest] Request to ${RELAYER_URL} timed out while waiting to retry.`,
      );
    });

    // Our deadline passes inside the warn call, before the wait starts: still RelayerHttpTimeoutError.
    it('our timer fired inside the warn hook (pre-aborted sleep) → RelayerHttpTimeoutError', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const logger = makeLogger();
      logger.warn.mockImplementationOnce(() => {
        vi.advanceTimersByTime(1000);
      });

      const run = scripted(scriptOf([600]), { timeout: 1000 }, { logger })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);

      expect(await run).toBeInstanceOf(RelayerHttpTimeoutError);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  // S14 with the real sleep: one synchronous clock advance fires the sleep timer and the abort
  // before the loop resumes, so the next attempt's fetch path sees the abort and never fetches.
  describe('abort between the end of the wait and the next fetch (real sleep)', () => {
    // Wait ends and the caller aborts in the same clock jump: the next attempt sees the abort, no 2nd fetch.
    //   503 ──► wait 100 ──► +150: wait over + caller aborts ──► run() rejects with the caller's reason
    it.each([
      ['a string', 'late'],
      ['null', null],
      ['a caller DOMException TimeoutError', new DOMException('Caller timed out', 'TimeoutError')],
    ])('caller abort with %s → that reason, classified by the next attempt, 1 fetch', async (_label, reason) => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const decide = vi.fn<Decide>(() => 100);
      const logger = makeLogger();
      const controller = new AbortController();
      const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

      const run = scripted(decide, { signal: controller.signal, timeout: 60_000 }, { logger })
        .run()
        .then(
          () => 'resolved',
          (e: unknown) => e,
        );
      await vi.advanceTimersByTimeAsync(0);
      expect(logger.warn).toHaveBeenCalledTimes(1);
      setTimeout(() => controller.abort(reason), 150);

      vi.advanceTimersByTime(150);
      await vi.advanceTimersByTimeAsync(0);

      expect(await run).toBe(reason);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(decide).toHaveBeenCalledTimes(1);
      expect(logger.warn).toHaveBeenLastCalledWith(`[RelayerHttpRequest] Request aborted on ${RELAYER_URL}.`);
      expect(vi.getTimerCount()).toBe(0);
      expect(removeSpy).toHaveBeenCalledTimes(1);
    });

    // The 950 ms wait ends and our 1000 ms deadline passes in the same clock jump: timeout, no 2nd fetch.
    it('our timer → RelayerHttpTimeoutError, classified by the next attempt, 1 fetch', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const decide = vi.fn<Decide>(() => 950);
      const logger = makeLogger();

      const run = scripted(decide, { timeout: 1000 }, { logger })
        .run()
        .catch((e: unknown) => e);
      await vi.advanceTimersByTimeAsync(0);
      expect(logger.warn).toHaveBeenCalledTimes(1);

      vi.advanceTimersByTime(1000);
      await vi.advanceTimersByTimeAsync(0);

      const error = await run;
      expect(error).toBeInstanceOf(RelayerHttpTimeoutError);
      expect((error as RelayerHttpTimeoutError).timeoutMs).toBe(1000);
      expect(global.fetch).toHaveBeenCalledTimes(1);
      expect(decide).toHaveBeenCalledTimes(1);
      expect(logger.warn).toHaveBeenLastCalledWith(`[RelayerHttpRequest] Request to ${RELAYER_URL} timed out.`);
      expect(vi.getTimerCount()).toBe(0);
    });
  });

  // S25
  describe('cleanup after a run that retried', () => {
    // After a run that retried, success or failure: caller listener added once and removed, no timer left.
    it.each([
      ['a retried success', [S503(), S503(), S200], [10, 10], true, 20],
      ['a retried failure', [S503(), S503()], [10, undefined], false, 10],
    ] as const)(
      '%s: caller listener removed once, no timer pending',
      async (_label, steps, script, succeeds, settleMs) => {
        vi.useFakeTimers();
        global.fetch = fetchSequence([...steps]);
        const controller = new AbortController();
        const addSpy = vi.spyOn(controller.signal, 'addEventListener');
        const removeSpy = vi.spyOn(controller.signal, 'removeEventListener');

        const run = scripted(scriptOf([...script]), { signal: controller.signal, timeout: 60_000 })
          .run()
          .then(
            () => true,
            () => false,
          );
        // Advance exactly to settlement: a forgotten timer must still be pending when inspected.
        await vi.advanceTimersByTimeAsync(settleMs);

        expect(await run).toBe(succeeds);
        expect(addSpy).toHaveBeenCalledTimes(1);
        expect(removeSpy).toHaveBeenCalledTimes(1);
        expect(removeSpy).toHaveBeenCalledWith('abort', addSpy.mock.calls[0]![1]);
        expect(vi.getTimerCount()).toBe(0);
      },
    );
  });

  // Caller mutates payload, auth, headers and query during the wait: the retry sends the original request.
  it('payload, auth, headers and query mutated during the wait: identical bodies, headers and urls', async () => {
    vi.useFakeTimers();
    global.fetch = fetchSequence([S503(), S200]);
    const payload: Record<string, unknown> = { a: 1, nested: { b: 2 } };
    const auth = { type: 'BearerToken' as const, token: 'token-1' };
    const headers: Record<string, string> = { 'x-custom': 'v1' };
    const query: Record<string, string> = { q: '1' };

    const run = scripted(scriptOf([1000]), { auth, headers, query }, { payload }).run();
    await vi.advanceTimersByTimeAsync(10);
    payload.a = 2;
    (payload.nested as Record<string, unknown>).b = 3;
    payload.big = 1n;
    auth.token = 'token-2';
    headers['x-custom'] = 'v2';
    headers['x-added'] = 'new';
    query.q = '2';
    await vi.advanceTimersByTimeAsync(1000);

    await expect(run).resolves.toEqual(SHARES);
    const calls = (global.fetch as Mock).mock.calls as Array<[string, RequestInit]>;
    expect(calls).toHaveLength(2);
    expect(calls[1]![0]).toBe(calls[0]![0]);
    expect(calls[1]![1].body).toBe(calls[0]![1].body);
    expect(calls[0]![1].body).toBe(JSON.stringify({ a: 1, nested: { b: 2 } }));
    expect(new Headers(calls[1]![1].headers).get('x-custom')).toBe('v1');
    expect(new Headers(calls[1]![1].headers).get('x-added')).toBeNull();
    expect(calls[1]![1].headers).toEqual(calls[0]![1].headers);
    expect(calls[1]![1].signal).toBe(calls[0]![1].signal);
    expect(new Headers(calls[1]![1].headers).get('authorization')).toBe('Bearer token-1');
  });

  // Three attempts: the payload is serialized (toJSON) only once.
  it('serializes the payload once for all attempts (toJSON called once)', async () => {
    vi.useFakeTimers();
    global.fetch = fetchSequence([S503(), S503(), S200]);
    const toJSON = vi.fn(() => ({ a: 1 }));

    const run = scripted(scriptOf([10, 10]), {}, { payload: { toJSON } }).run();
    await vi.advanceTimersByTimeAsync(20);

    await expect(run).resolves.toEqual(SHARES);
    expect(global.fetch).toHaveBeenCalledTimes(3);
    expect(toJSON).toHaveBeenCalledTimes(1);
  });

  // options.query (may hold secrets) never appears in the retry warn or the final error line.
  it('logs the query-less url in the retry warn and the stop error line when options.query is set', async () => {
    vi.useFakeTimers();
    global.fetch = fetchSequence([S503('m1'), S503('m2')]);
    const logger = makeLogger();

    const run = scripted(scriptOf([10, undefined]), { query: { token: 'secret' } }, { logger })
      .run()
      .catch((e: unknown) => e);
    await vi.advanceTimersByTimeAsync(10);

    expect(await run).toBeInstanceOf(RelayerHttpResponseError);
    expect((global.fetch as Mock).mock.calls[0]![0]).toBe(`${RELAYER_URL}?token=secret`);
    expect(logger.warn).toHaveBeenCalledWith(
      `[RelayerHttpRequest] relayer-http error on ${RELAYER_URL} (status=503, code=overloaded, requestId=${VALID_REQUEST_ID}): m1. Retrying in 10 ms (attempt 1).`,
    );
    expect(logger.error).toHaveBeenCalledWith(stopLine('m2'), undefined);
    for (const [line] of [...logger.warn.mock.calls, ...logger.error.mock.calls] as Array<[string]>) {
      expect(line).not.toContain('?');
      expect(line).not.toContain('secret');
    }
  });

  // PUBLIC_DECRYPT retries too: a 503 then a 200 returns the public result.
  it('retries a PUBLIC_DECRYPT request: 503 then 200 returns the public result', async () => {
    vi.useFakeTimers();
    const publicResult = { decryptedValue: '00'.repeat(31) + '2a', signatures: ['ab'.repeat(65)], extraData: '0x00' };
    global.fetch = fetchSequence([
      S503(),
      { status: 200, body: JSON.stringify({ status: 'succeeded', requestId: VALID_REQUEST_ID, result: publicResult }) },
    ]);
    const decide = vi.fn<Decide>(() => 10);
    const ScriptedRequest = scriptedRequestClass(decide);
    const request = new ScriptedRequest({ relayerOperation: 'PUBLIC_DECRYPT', url: RELAYER_URL, payload: {} });

    const run = request.run();
    await vi.advanceTimersByTimeAsync(10);

    await expect(run).resolves.toEqual(publicResult);
    expect(global.fetch).toHaveBeenCalledTimes(2);
    expect(decide.mock.calls.map(([, , attempt]) => attempt)).toEqual([1]);
  });

  // Network retries restart inside each attempt: network fail, 503, network fail, 200 → success after 4 fetches.
  it('runs the network retries inside each HTTP attempt (counter restarts per attempt)', async () => {
    vi.useFakeTimers();
    const network = (): Step => ({ reject: new TypeError('fetch failed') });
    global.fetch = fetchSequence([network(), S503(), network(), S200]);

    const run = scripted(scriptOf([1000]), { fetchRetries: 1, fetchRetryDelayInMilliseconds: 100 }).run();
    await vi.advanceTimersByTimeAsync(100 + 1000 + 100);

    await expect(run).resolves.toEqual(SHARES);
    expect(global.fetch).toHaveBeenCalledTimes(4);
  });

  describe('S8 one-shot run()', () => {
    const MESSAGE = 'RelayerHttpRequest.run() was already called on this instance. Create a new instance per request.';

    // run() called again after it settled: rejects at once, no new fetch.
    it('rejects a second run() after the first settled, without fetching', async () => {
      global.fetch = fetchSequence([S200]);
      const request = scripted(scriptOf([]));

      await request.run();
      await expect(request.run()).rejects.toThrow(MESSAGE);
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    // run() called again during the retry wait: rejects, while the first run still succeeds.
    it('rejects a second run() during the wait; the first one still retries and succeeds', async () => {
      vi.useFakeTimers();
      global.fetch = fetchSequence([S503(), S200]);
      const request = scripted(scriptOf([1000]));

      const first = request.run();
      await vi.advanceTimersByTimeAsync(10);
      await expect(request.run()).rejects.toThrow(MESSAGE);
      await vi.advanceTimersByTimeAsync(1000);

      await expect(first).resolves.toEqual(SHARES);
      expect(global.fetch).toHaveBeenCalledTimes(2);
    });
  });
});
